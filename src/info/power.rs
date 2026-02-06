use crate::info::{BatteryInfo, SystemInfo};
use std::fs;
use std::path::Path;

pub fn gather(info: &mut SystemInfo) {
    gather_battery(info);
    gather_brightness(info);
}

fn gather_battery(info: &mut SystemInfo) {
    #[cfg(target_os = "macos")]
    if gather_macos_battery(info) {
        return;
    }

    let power_supply_path = Path::new("/sys/class/power_supply");

    if !power_supply_path.exists() {
        return;
    }

    let mut total_energy_now: i64 = 0;
    let mut total_energy_full: i64 = 0;
    let mut total_power_now: i64 = 0;
    let mut has_energy = false;
    let mut has_power = false;
    let mut percents: Vec<u8> = Vec::new();
    let mut statuses: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(power_supply_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Look for batteries (BAT0, BAT1, etc.)
            if !name.starts_with("BAT") && !name.contains("battery") {
                continue;
            }

            // Check if this is actually a battery
            let type_path = path.join("type");
            if let Ok(device_type) = fs::read_to_string(&type_path) {
                if device_type.trim() != "Battery" {
                    continue;
                }
            }

            // Get status (Charging, Discharging, Full, Not charging)
            let status = read_trimmed(path.join("status").as_path())
                .unwrap_or_else(|| "Unknown".to_string());
            statuses.push(status.clone());

            // Get capacity (percentage)
            if let Some(capacity) = fs::read_to_string(path.join("capacity"))
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
            {
                percents.push(capacity);
            }

            // Aggregate energy for multi-battery systems
            let energy_now = read_i64(path.join("energy_now").as_path())
                .or_else(|| read_i64(path.join("charge_now").as_path()));

            let energy_full = read_i64(path.join("energy_full").as_path())
                .or_else(|| read_i64(path.join("charge_full").as_path()));

            if let (Some(now), Some(full)) = (energy_now, energy_full) {
                total_energy_now += now;
                total_energy_full += full;
                has_energy = true;
            }

            let power_now = read_i64(path.join("power_now").as_path())
                .or_else(|| read_i64(path.join("current_now").as_path()));

            if let Some(power) = power_now {
                total_power_now += power;
                has_power = true;
            }
        }
    }

    if !has_energy && percents.is_empty() {
        return;
    }

    let mut percent = if has_energy && total_energy_full > 0 {
        ((total_energy_now as f64 / total_energy_full as f64) * 100.0).round() as u8
    } else {
        let sum: u32 = percents.iter().map(|v| *v as u32).sum();
        (sum / percents.len() as u32) as u8
    };
    if percent > 100 {
        percent = 100;
    }

    let status = resolve_status(&statuses);
    let time_remaining = if has_energy && has_power && total_power_now > 0 {
        calculate_time_remaining_total(total_energy_now, total_energy_full, total_power_now, &status)
    } else {
        None
    };

    info.battery = Some(BatteryInfo {
        percent,
        status: format_battery_status(&status),
        time_remaining,
    });
}

fn calculate_time_remaining_total(
    energy_now: i64,
    energy_full: i64,
    power_now: i64,
    status: &str,
) -> Option<String> {
    if power_now <= 0 {
        return None;
    }

    let hours = match status.to_lowercase().as_str() {
        "charging" => (energy_full.saturating_sub(energy_now)) as f64 / power_now as f64,
        "discharging" => energy_now as f64 / power_now as f64,
        _ => return None,
    };

    if hours > 0.0 && hours < 100.0 {
        let total_minutes = (hours * 60.0) as u32;
        let h = total_minutes / 60;
        let m = total_minutes % 60;
        return Some(format!("{}:{:02}", h, m));
    }

    None
}

fn resolve_status(statuses: &[String]) -> String {
    let has_charging = statuses.iter().any(|s| s.eq_ignore_ascii_case("charging"));
    let has_discharging = statuses.iter().any(|s| s.eq_ignore_ascii_case("discharging"));
    let has_full = statuses.iter().any(|s| s.eq_ignore_ascii_case("full"));

    if has_charging {
        "Charging".to_string()
    } else if has_discharging {
        "Discharging".to_string()
    } else if has_full {
        "Full".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "macos")]
fn gather_macos_battery(info: &mut SystemInfo) -> bool {
    use std::process::Command;

    let output = match Command::new("pmset").args(["-g", "batt"]).output() {
        Ok(out) if out.status.success() => out,
        _ => return false,
    };

    let content = String::from_utf8_lossy(&output.stdout);
    for line in content.lines() {
        if !line.contains('%') {
            continue;
        }

        let percent = line
            .split('%')
            .next()
            .and_then(|part| part.split_whitespace().last())
            .and_then(|p| p.parse::<u8>().ok());

        let status = if line.to_lowercase().contains("charging") {
            "Charging".to_string()
        } else if line.to_lowercase().contains("discharging") {
            "Discharging".to_string()
        } else if line.to_lowercase().contains("charged") {
            "Full".to_string()
        } else {
            "Unknown".to_string()
        };

        let time_remaining = line.split(';').nth(2).and_then(|s| {
            let trimmed = s.trim();
            let time = trimmed.split_whitespace().next().unwrap_or("");
            if time.contains(':') {
                Some(time.to_string())
            } else {
                None
            }
        });

        if let Some(percent) = percent {
            info.battery = Some(BatteryInfo {
                percent,
                status,
                time_remaining,
            });
            return true;
        }
    }

    false
}

fn read_trimmed(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_i64(path: &Path) -> Option<i64> {
    read_trimmed(path).and_then(|s| s.parse::<i64>().ok())
}

fn format_battery_status(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "charging" => "Charging".to_string(),
        "discharging" => "Discharging".to_string(),
        "full" => "Full".to_string(),
        "not charging" => "Not Charging".to_string(),
        _ => status.to_string(),
    }
}

fn gather_brightness(info: &mut SystemInfo) {
    let backlight_path = Path::new("/sys/class/backlight");

    if !backlight_path.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(backlight_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            let brightness = fs::read_to_string(path.join("brightness"))
                .ok()
                .and_then(|s| s.trim().parse::<u32>().ok());

            let max_brightness = fs::read_to_string(path.join("max_brightness"))
                .ok()
                .and_then(|s| s.trim().parse::<u32>().ok());

            if let (Some(current), Some(max)) = (brightness, max_brightness) {
                if max > 0 {
                    let percent = (current as f64 / max as f64 * 100.0) as u32;
                    info.brightness = Some(format!("{}%", percent));
                    break;
                }
            }
        }
    }
}
