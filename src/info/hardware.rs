use crate::info::{DiskInfo, GpuInfo, SystemInfo};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use sysinfo::{Disks, System};

pub fn gather(info: &mut SystemInfo) {
    gather_cpu(info);
    gather_memory(info);
    gather_swap(info);
    gather_gpu(info);
    gather_disks(info);
    gather_motherboard(info);
    gather_bios(info);
    gather_cpu_temp(info);
    gather_cpu_governor(info);
}

fn gather_cpu(info: &mut SystemInfo) {
    // Get CPU model from /proc/cpuinfo
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let mut model_name = None;
        let mut cores: HashMap<String, bool> = HashMap::new();
        let mut threads = 0u32;
        let mut max_freq: Option<f64> = None;
        let mut cache_size = None;

        for line in content.lines() {
            if line.starts_with("model name") {
                if model_name.is_none() {
                    model_name = line.split(':').nth(1).map(|s| s.trim().to_string());
                }
                threads += 1;
            } else if line.starts_with("core id") {
                if let Some(id) = line.split(':').nth(1) {
                    cores.insert(id.trim().to_string(), true);
                }
            } else if line.starts_with("cpu MHz") {
                if let Some(freq_str) = line.split(':').nth(1) {
                    if let Ok(freq) = freq_str.trim().parse::<f64>() {
                        max_freq = Some(max_freq.map_or(freq, |m: f64| m.max(freq)));
                    }
                }
            } else if line.starts_with("cache size") {
                if cache_size.is_none() {
                    cache_size = line.split(':').nth(1).map(|s| s.trim().to_string());
                }
            }
        }

        // Clean up model name
        if let Some(ref mut name) = model_name {
            // Remove excessive whitespace and common suffixes
            *name = name
                .replace("(R)", "")
                .replace("(TM)", "")
                .replace("CPU", "")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
        }

        info.cpu = model_name;
        info.cpu_cores = if cores.is_empty() {
            Some(threads)
        } else {
            Some(cores.len() as u32)
        };
        info.cpu_threads = Some(threads);

        if let Some(freq) = max_freq {
            if freq >= 1000.0 {
                info.cpu_freq = Some(format!("{:.2} GHz", freq / 1000.0));
            } else {
                info.cpu_freq = Some(format!("{:.0} MHz", freq));
            }
        }

        if let Some(cache) = cache_size {
            info.cpu_cache = Some(cache);
        }
    }

    // macOS fallback
    #[cfg(target_os = "macos")]
    if info.cpu.is_none() {
        if let Ok(output) = Command::new("sysctl").args(["-n", "machdep.cpu.brand_string"]).output() {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s.is_empty() {
                    info.cpu = Some(s);
                }
            }
        }

        if info.cpu.is_none() {
            if let Ok(output) = Command::new("sysctl").args(["-n", "hw.model"]).output() {
                if output.status.success() {
                    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !s.is_empty() {
                        info.cpu = Some(s);
                    }
                }
            }
        }

        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.physicalcpu"]).output() {
            if output.status.success() {
                if let Ok(v) = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>() {
                    info.cpu_cores = Some(v);
                }
            }
        }

        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.logicalcpu"]).output() {
            if output.status.success() {
                if let Ok(v) = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>() {
                    info.cpu_threads = Some(v);
                }
            }
        }

        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.cpufrequency"]).output() {
            if output.status.success() {
                if let Ok(v) = String::from_utf8_lossy(&output.stdout).trim().parse::<u64>() {
                    let ghz = v as f64 / 1_000_000_000.0;
                    if ghz > 0.0 {
                        info.cpu_freq = Some(format!("{:.2} GHz", ghz));
                    }
                }
            }
        }
    }

    // Get architecture
    if let Ok(output) = Command::new("uname").arg("-m").output() {
        if output.status.success() {
            info.cpu_arch = Some(trim_lossy(&output.stdout));
        }
    }

    // Try to get more detailed cache info
    gather_cpu_cache(info);
}

fn gather_cpu_cache(info: &mut SystemInfo) {
    let cache_path = Path::new("/sys/devices/system/cpu/cpu0/cache");
    if cache_path.exists() {
        let mut l1d = None;
        let mut l1i = None;
        let mut l2 = None;
        let mut l3 = None;

        for i in 0..10 {
            let index_path = cache_path.join(format!("index{}", i));
            if !index_path.exists() {
                break;
            }

            let level = read_trimmed(index_path.join("level").as_path())
                .and_then(|s| s.parse::<u32>().ok());
            let cache_type = read_trimmed(index_path.join("type").as_path());
            let size = read_trimmed(index_path.join("size").as_path());

            if let (Some(lvl), Some(size)) = (level, size) {
                match (lvl, cache_type.as_deref()) {
                    (1, Some("Data")) => l1d = Some(size),
                    (1, Some("Instruction")) => l1i = Some(size),
                    (2, _) => l2 = Some(size),
                    (3, _) => l3 = Some(size),
                    _ => {}
                }
            }
        }

        let mut cache_parts = Vec::new();
        if let (Some(d), Some(i)) = (l1d, l1i) {
            cache_parts.push(format!("L1: {}+{}", d, i));
        }
        if let Some(c) = l2 {
            cache_parts.push(format!("L2: {}", c));
        }
        if let Some(c) = l3 {
            cache_parts.push(format!("L3: {}", c));
        }

        if !cache_parts.is_empty() {
            info.cpu_cache = Some(cache_parts.join(", "));
        }
    }
}

fn gather_memory(info: &mut SystemInfo) {
    let sys = System::new_all();
    let total = sys.total_memory();
    let used = sys.used_memory();

    info.memory_total = Some(total);
    info.memory_used = Some(used);

    let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;

    info.memory = Some(format!(
        "{:.2} GiB / {:.2} GiB ({:.0}%)",
        used_gb,
        total_gb,
        (used as f64 / total as f64) * 100.0
    ));
}

fn gather_swap(info: &mut SystemInfo) {
    let sys = System::new_all();
    let total = sys.total_swap();
    let used = sys.used_swap();

    if total > 0 {
        info.swap_total = Some(total);
        info.swap_used = Some(used);

        let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;

        info.swap = Some(format!(
            "{:.2} GiB / {:.2} GiB ({:.0}%)",
            used_gb,
            total_gb,
            if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        ));
    }
}

fn read_trimmed(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

fn gather_gpu(info: &mut SystemInfo) {
    // Try lspci first
    if let Ok(output) = Command::new("lspci").arg("-mm").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                // Look for VGA or 3D controllers
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                    // Parse the lspci -mm format
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() >= 6 {
                        let vendor = parts.get(3).unwrap_or(&"").trim();
                        let device = parts.get(5).unwrap_or(&"").trim();

                        if !device.is_empty() {
                            let name = if vendor.is_empty() {
                                device.to_string()
                            } else {
                                format!("{} {}", vendor, device)
                            };

                            let mut gpu = GpuInfo {
                                name,
                                driver: None,
                                vram: None,
                                temp: None,
                            };

                            // Try to get driver info
                            if let Some(driver) = get_gpu_driver() {
                                gpu.driver = Some(driver);
                            }

                            // Try to get VRAM
                            if let Some(vram) = get_gpu_vram() {
                                gpu.vram = Some(vram);
                            }

                            info.gpu.push(gpu);
                        }
                    }
                }
            }
        }
    }

    // Fallback: check /sys/class/drm
    if info.gpu.is_empty() {
        for entry in fs::read_dir("/sys/class/drm").into_iter().flatten() {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("card") && !name.contains('-') {
                    let device_path = entry.path().join("device");
                    if let Ok(vendor) = fs::read_to_string(device_path.join("vendor")) {
                        let vendor_id = vendor.trim();
                        let vendor_name = match vendor_id {
                            "0x10de" => "NVIDIA",
                            "0x1002" => "AMD",
                            "0x8086" => "Intel",
                            _ => "Unknown",
                        };

                        let gpu = GpuInfo {
                            name: format!("{} Graphics", vendor_name),
                            driver: get_gpu_driver(),
                            vram: get_gpu_vram(),
                            temp: None,
                        };
                        info.gpu.push(gpu);
                    }
                }
            }
        }
    }

    // Get GPU temperatures
    gather_gpu_temps(info);
}

fn get_gpu_driver() -> Option<String> {
    // Check for NVIDIA
    if Path::new("/proc/driver/nvidia/version").exists() {
        if let Ok(content) = fs::read_to_string("/proc/driver/nvidia/version") {
            if let Some(line) = content.lines().next() {
                if let Some(version) = line.split_whitespace().nth(7) {
                    return Some(format!("NVIDIA {}", version));
                }
            }
        }
    }

    // Check for AMD
    for entry in fs::read_dir("/sys/class/drm").into_iter().flatten() {
        if let Ok(entry) = entry {
            let driver_path = entry.path().join("device/driver");
            if let Ok(target) = fs::read_link(&driver_path) {
                let driver = target.file_name()?.to_string_lossy().to_string();
                if driver == "amdgpu" || driver == "radeon" {
                    return Some(driver);
                } else if driver == "i915" || driver == "xe" {
                    return Some(format!("Intel {}", driver));
                }
            }
        }
    }

    None
}

fn get_gpu_vram() -> Option<String> {
    // Try NVIDIA
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            let mem = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(mb) = mem.parse::<u64>() {
                return Some(format!("{} MiB", mb));
            }
        }
    }

    // Try AMD
    for entry in fs::read_dir("/sys/class/drm").into_iter().flatten() {
        if let Ok(entry) = entry {
            let vram_path = entry.path().join("device/mem_info_vram_total");
            if let Ok(content) = fs::read_to_string(&vram_path) {
                if let Ok(bytes) = content.trim().parse::<u64>() {
                    let mb = bytes / 1024 / 1024;
                    return Some(format!("{} MiB", mb));
                }
            }
        }
    }

    None
}

fn gather_gpu_temps(info: &mut SystemInfo) {
    // NVIDIA temperature
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=temperature.gpu", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            let temp = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !temp.is_empty() {
                if let Some(gpu) = info.gpu.first_mut() {
                    gpu.temp = Some(format!("{}°C", temp));
                }
            }
        }
    }

    // AMD temperature from hwmon
    for entry in fs::read_dir("/sys/class/hwmon").into_iter().flatten() {
        if let Ok(entry) = entry {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                if name.trim() == "amdgpu" {
                    let temp_path = entry.path().join("temp1_input");
                    if let Ok(temp) = fs::read_to_string(&temp_path) {
                        if let Ok(millidegrees) = temp.trim().parse::<i32>() {
                            let celsius = millidegrees / 1000;
                            for gpu in &mut info.gpu {
                                if gpu.name.contains("AMD") && gpu.temp.is_none() {
                                    gpu.temp = Some(format!("{}°C", celsius));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn gather_disks(info: &mut SystemInfo) {
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy().to_string();

        // Skip pseudo filesystems
        let fs_type = disk.file_system().to_string_lossy().to_string();
        if matches!(
            fs_type.as_str(),
            "tmpfs"
                | "devtmpfs"
                | "squashfs"
                | "overlay"
                | "proc"
                | "sysfs"
                | "devpts"
                | "cgroup"
                | "cgroup2"
                | "autofs"
                | "mqueue"
                | "hugetlbfs"
                | "debugfs"
                | "tracefs"
                | "securityfs"
                | "pstore"
                | "configfs"
                | "fusectl"
                | "binfmt_misc"
                | "ramfs"
                | "efivarfs"
        ) {
            continue;
        }

        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);

        // Skip tiny filesystems
        if total < 100 * 1024 * 1024 {
            continue;
        }

        let percent = if total > 0 {
            ((used as f64 / total as f64) * 100.0) as u8
        } else {
            0
        };

        let disk_info = DiskInfo {
            mount,
            filesystem: fs_type,
            size: format_bytes(total),
            used: format_bytes(used),
            available: format_bytes(available),
            percent,
            disk_type: get_disk_type(disk.name().to_string_lossy().as_ref()),
        };

        info.disks.push(disk_info);
    }
}

fn get_disk_type(device: &str) -> Option<String> {
    // Extract base device name
    let base = device
        .trim_start_matches("/dev/")
        .trim_end_matches(|c: char| c.is_numeric());

    // Check rotational status
    let rotational_path = format!("/sys/block/{}/queue/rotational", base);
    if let Ok(content) = fs::read_to_string(&rotational_path) {
        let is_rotational = content.trim() == "1";
        if !is_rotational {
            // Check if NVMe
            if base.starts_with("nvme") {
                return Some("NVMe SSD".to_string());
            }
            return Some("SSD".to_string());
        } else {
            return Some("HDD".to_string());
        }
    }

    None
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TiB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GiB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MiB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KiB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn gather_motherboard(info: &mut SystemInfo) {
    let vendor = fs::read_to_string("/sys/class/dmi/id/board_vendor")
        .ok()
        .map(|s| s.trim().to_string());
    let name = fs::read_to_string("/sys/class/dmi/id/board_name")
        .ok()
        .map(|s| s.trim().to_string());

    match (vendor, name) {
        (Some(v), Some(n)) => info.motherboard = Some(format!("{} {}", v, n)),
        (None, Some(n)) => info.motherboard = Some(n),
        (Some(v), None) => info.motherboard = Some(v),
        _ => {}
    }
}

fn gather_bios(info: &mut SystemInfo) {
    let vendor = fs::read_to_string("/sys/class/dmi/id/bios_vendor")
        .ok()
        .map(|s| s.trim().to_string());
    let version = fs::read_to_string("/sys/class/dmi/id/bios_version")
        .ok()
        .map(|s| s.trim().to_string());
    let date = fs::read_to_string("/sys/class/dmi/id/bios_date")
        .ok()
        .map(|s| s.trim().to_string());

    let mut parts = Vec::new();
    if let Some(v) = vendor {
        parts.push(v);
    }
    if let Some(ver) = version {
        parts.push(ver);
    }
    if let Some(d) = date {
        parts.push(format!("({})", d));
    }

    if !parts.is_empty() {
        info.bios = Some(parts.join(" "));
    }
}

fn gather_cpu_temp(info: &mut SystemInfo) {
    // Try coretemp first (Intel)
    for entry in fs::read_dir("/sys/class/hwmon").into_iter().flatten() {
        if let Ok(entry) = entry {
            let name_path = entry.path().join("name");
            if let Ok(name) = fs::read_to_string(&name_path) {
                let name = name.trim();
                if name == "coretemp" || name == "k10temp" || name == "zenpower" {
                    // Find the package temp or first core temp
                    let temp_path = entry.path().join("temp1_input");
                    if let Ok(temp) = fs::read_to_string(&temp_path) {
                        if let Ok(millidegrees) = temp.trim().parse::<i32>() {
                            let celsius = millidegrees / 1000;
                            info.cpu_temp = Some(format!("{}°C", celsius));
                            return;
                        }
                    }
                }
            }
        }
    }

    // Try thermal zones as fallback
    for i in 0..10 {
        let type_path = format!("/sys/class/thermal/thermal_zone{}/type", i);
        let temp_path = format!("/sys/class/thermal/thermal_zone{}/temp", i);

        if let (Ok(zone_type), Ok(temp)) =
            (fs::read_to_string(&type_path), fs::read_to_string(&temp_path))
        {
            let zone_type = zone_type.trim().to_lowercase();
            if zone_type.contains("cpu")
                || zone_type.contains("core")
                || zone_type.contains("package")
                || zone_type.contains("x86_pkg_temp")
            {
                if let Ok(millidegrees) = temp.trim().parse::<i32>() {
                    let celsius = millidegrees / 1000;
                    info.cpu_temp = Some(format!("{}°C", celsius));
                    return;
                }
            }
        }
    }
}

fn gather_cpu_governor(info: &mut SystemInfo) {
    let governor_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
    if let Ok(governor) = fs::read_to_string(governor_path) {
        info.cpu_governor = Some(governor.trim().to_string());
    }
}
