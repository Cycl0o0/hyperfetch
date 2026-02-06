use crate::info::SystemInfo;
use std::fs;
use std::path::Path;
use std::process::Command;
use sysinfo::System;

pub fn gather(info: &mut SystemInfo) {
    gather_os(info);
    gather_kernel(info);
    gather_hostname(info);
    gather_uptime(info);
    gather_load_average(info);
    gather_processes(info);
    gather_users(info);
    gather_machine_type(info);
    gather_init_system(info);
    gather_boot_time(info);
}

fn gather_os(info: &mut SystemInfo) {
    // Try /etc/os-release first
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        let (name, version, id) = parse_os_release(&content);

        if let Some(n) = name {
            info.os = Some(n);
        } else if let Some(v) = version {
            info.os = Some(format!("Linux {}", v));
        }

        info.os_id = id;
    }

    // Fallback to lsb_release
    if info.os.is_none() {
        if let Ok(output) = Command::new("lsb_release").arg("-ds").output() {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .trim_matches('"')
                    .to_string();
                if !s.is_empty() {
                    info.os = Some(s);
                }
            }
        }
    }

    // macOS fallback
    #[cfg(target_os = "macos")]
    if info.os.is_none() {
        let name = command_output_trim("sw_vers", &["-productName"]);
        let version = command_output_trim("sw_vers", &["-productVersion"]);

        if name.is_some() || version.is_some() {
            let os_name = match (name, version) {
                (Some(n), Some(v)) => format!("{} {}", n, v),
                (Some(n), None) => n,
                (None, Some(v)) => format!("macOS {}", v),
                (None, None) => "macOS".to_string(),
            };
            info.os = Some(os_name);
            info.os_id = Some("macos".to_string());
        }
    }

    // Final fallback
    if info.os.is_none() {
        info.os = Some("Linux".to_string());
    }
}

fn gather_kernel(info: &mut SystemInfo) {
    if let Ok(content) = fs::read_to_string("/proc/version") {
        if let Some(version) = content.split_whitespace().nth(2) {
            info.kernel = Some(version.to_string());
        }
    }

    if info.kernel.is_none() {
        if let Ok(output) = Command::new("uname").arg("-r").output() {
            if output.status.success() {
                info.kernel = Some(trim_lossy(&output.stdout));
            }
        }
    }
}

fn gather_hostname(info: &mut SystemInfo) {
    if let Ok(name) = hostname::get() {
        info.hostname = Some(name.to_string_lossy().to_string());
    }

    // Try to get FQDN
    if let Ok(content) = fs::read_to_string("/etc/hostname") {
        let h = content.trim().to_string();
        if !h.is_empty() && info.hostname.is_none() {
            info.hostname = Some(h);
        }
    }
}

fn gather_uptime(info: &mut SystemInfo) {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(seconds_str) = content.split_whitespace().next() {
            if let Ok(seconds) = seconds_str.parse::<f64>() {
                let secs = seconds as u64;
                info.uptime_seconds = Some(secs);
                info.uptime = Some(format_uptime(secs));
            }
        }
    }

    if info.uptime.is_none() {
        #[cfg(target_os = "macos")]
        if let Some(boot_time) = macos_boot_time_seconds() {
            if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                let secs = now.as_secs().saturating_sub(boot_time as u64);
                info.uptime_seconds = Some(secs);
                info.uptime = Some(format_uptime(secs));
            }
        }
    }
}

fn gather_load_average(info: &mut SystemInfo) {
    if let Ok(content) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            info.load_average = Some(format!("{}, {}, {}", parts[0], parts[1], parts[2]));
        }
    }

    if info.load_average.is_none() {
        #[cfg(target_os = "macos")]
        if let Ok(output) = Command::new("sysctl").args(["-n", "vm.loadavg"]).output() {
            if output.status.success() {
                let s = trim_lossy(&output.stdout);
                let cleaned = s.trim_matches('{').trim_matches('}').trim();
                let parts: Vec<&str> = cleaned.split_whitespace().collect();
                if parts.len() >= 3 {
                    info.load_average = Some(format!("{}, {}, {}", parts[0], parts[1], parts[2]));
                }
            }
        }
    }
}

fn gather_processes(info: &mut SystemInfo) {
    let sys = System::new_all();
    info.processes = Some(sys.processes().len() as u32);
}

fn gather_users(info: &mut SystemInfo) {
    // Count unique logged-in users from /var/run/utmp or use `who`
    if let Ok(output) = Command::new("who").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let users: Vec<&str> = content.lines().filter_map(|l| l.split_whitespace().next()).collect();
            let unique: std::collections::HashSet<_> = users.into_iter().collect();
            if !unique.is_empty() {
                let user_list: Vec<_> = unique.into_iter().collect();
                info.logged_users = Some(format!("{} ({})", user_list.len(), user_list.join(", ")));
            }
        }
    }
}

fn gather_machine_type(info: &mut SystemInfo) {
    // Check for VM/Container first
    if Path::new("/.dockerenv").exists() {
        info.machine_type = Some("Container (Docker)".to_string());
        return;
    }

    if let Ok(content) = fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") || content.contains("lxc") || content.contains("kubepods") {
            info.machine_type = Some("Container".to_string());
            return;
        }
    }

    // Check DMI for VM
    if let Ok(content) = fs::read_to_string("/sys/class/dmi/id/product_name") {
        let product = content.trim().to_lowercase();
        if product.contains("virtualbox") {
            info.machine_type = Some("Virtual Machine (VirtualBox)".to_string());
            return;
        } else if product.contains("vmware") {
            info.machine_type = Some("Virtual Machine (VMware)".to_string());
            return;
        } else if product.contains("kvm") || product.contains("qemu") {
            info.machine_type = Some("Virtual Machine (KVM/QEMU)".to_string());
            return;
        } else if product.contains("hyper-v") {
            info.machine_type = Some("Virtual Machine (Hyper-V)".to_string());
            return;
        }
    }

    // Check chassis type for physical machines
    if let Ok(content) = fs::read_to_string("/sys/class/dmi/id/chassis_type") {
        let chassis: u32 = content.trim().parse().unwrap_or(0);
        info.machine_type = Some(
            match chassis {
                3 | 4 | 5 | 6 | 7 | 15 | 16 => "Desktop",
                8 | 9 | 10 | 11 | 14 | 31 => "Laptop",
                17 | 23 | 28 | 29 => "Server",
                30 => "Tablet",
                _ => "Unknown",
            }
            .to_string(),
        );
    }
}

fn gather_init_system(info: &mut SystemInfo) {
    if let Ok(target) = fs::read_link("/sbin/init") {
        let target_str = target.to_string_lossy().to_lowercase();
        if target_str.contains("systemd") {
            info.init_system = Some("systemd".to_string());
            return;
        } else if target_str.contains("openrc") {
            info.init_system = Some("OpenRC".to_string());
            return;
        } else if target_str.contains("runit") {
            info.init_system = Some("runit".to_string());
            return;
        } else if target_str.contains("s6") {
            info.init_system = Some("s6".to_string());
            return;
        }
    }

    // Check by process
    if Path::new("/run/systemd/system").exists() {
        info.init_system = Some("systemd".to_string());
    } else if Path::new("/run/openrc").exists() {
        info.init_system = Some("OpenRC".to_string());
    } else if Path::new("/run/runit").exists() || Path::new("/etc/runit").exists() {
        info.init_system = Some("runit".to_string());
    } else if Path::new("/run/s6").exists() {
        info.init_system = Some("s6".to_string());
    } else if Path::new("/etc/init.d").exists() {
        info.init_system = Some("SysVinit".to_string());
    }
}

fn gather_boot_time(info: &mut SystemInfo) {
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        for line in content.lines() {
            if line.starts_with("btime ") {
                if let Some(ts) = line.split_whitespace().nth(1) {
                    if let Ok(timestamp) = ts.parse::<i64>() {
                        use std::time::{Duration, UNIX_EPOCH};
                        if let Some(dt) = UNIX_EPOCH.checked_add(Duration::from_secs(timestamp as u64)) {
                            // Format as local time
                            let secs = dt.duration_since(UNIX_EPOCH).unwrap().as_secs();
                            let (year, month, day, hour, min) = timestamp_to_datetime(secs as i64);
                            info.boot_time = Some(format!(
                                "{:04}-{:02}-{:02} {:02}:{:02}",
                                year, month, day, hour, min
                            ));
                        }
                    }
                }
                break;
            }
        }
    }

    if info.boot_time.is_none() {
        #[cfg(target_os = "macos")]
        if let Some(boot_time) = macos_boot_time_seconds() {
            let (year, month, day, hour, min) = timestamp_to_datetime(boot_time);
            info.boot_time = Some(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                year, month, day, hour, min
            ));
        }
    }
}

fn timestamp_to_datetime(timestamp: i64) -> (i32, u32, u32, u32, u32) {
    // Simple UTC conversion (good enough for display)
    let secs_per_day = 86400i64;
    let days = timestamp / secs_per_day;
    let day_secs = timestamp % secs_per_day;

    let hour = (day_secs / 3600) as u32;
    let min = ((day_secs % 3600) / 60) as u32;

    // Days since 1970-01-01
    let mut year = 1970i32;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u32;
    for days_in_month in days_in_months {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days as u32 + 1;

    (year, month, day, hour, min)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if mins > 0 || parts.is_empty() {
        parts.push(format!("{} min{}", mins, if mins == 1 { "" } else { "s" }));
    }

    parts.join(", ")
}

fn parse_os_release(content: &str) -> (Option<String>, Option<String>, Option<String>) {
    let mut name = None;
    let mut version = None;
    let mut id = None;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            name = Some(trim_quotes(value).to_string());
        } else if let Some(value) = line.strip_prefix("VERSION_ID=") {
            version = Some(trim_quotes(value).to_string());
        } else if let Some(value) = line.strip_prefix("ID=") {
            id = Some(trim_quotes(value).to_string());
        }
    }

    (name, version, id)
}

fn trim_quotes(value: &str) -> &str {
    value.trim_matches('"').trim_matches('\'')
}

fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

fn command_output_trim(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value = trim_lossy(&output.stdout);
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

#[cfg(target_os = "macos")]
fn macos_boot_time_seconds() -> Option<i64> {
    let output = Command::new("sysctl").args(["-n", "kern.boottime"]).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&output.stdout);
    parse_macos_boot_time(&s)
}

#[cfg(target_os = "macos")]
fn parse_macos_boot_time(s: &str) -> Option<i64> {
    let mut digits = String::new();
    let mut in_sec = false;

    for token in s.split(|c: char| c.is_whitespace() || c == ',' || c == '{' || c == '}') {
        if token == "sec" {
            in_sec = true;
            digits.clear();
            continue;
        }
        if in_sec {
            if token == "=" {
                continue;
            }
            if token.chars().all(|c| c.is_ascii_digit()) {
                digits = token.to_string();
                break;
            }
        }
    }

    if digits.is_empty() {
        let mut current = String::new();
        for ch in s.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if current.len() >= 9 {
                digits = current.clone();
                break;
            } else {
                current.clear();
            }
        }
        if digits.is_empty() && current.len() >= 9 {
            digits = current;
        }
    }

    if digits.is_empty() {
        None
    } else {
        digits.parse::<i64>().ok()
    }
}
