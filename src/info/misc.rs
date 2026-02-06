use crate::info::SystemInfo;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn gather(info: &mut SystemInfo) {
    gather_locale(info);
    gather_timezone(info);
    gather_virtualization(info);
    gather_container(info);
    gather_security(info);
    gather_ssh(info);
    gather_bluetooth(info);
}

fn gather_locale(info: &mut SystemInfo) {
    // Try LANG environment variable
    if let Ok(lang) = env::var("LANG") {
        info.locale = Some(lang);
        return;
    }
    if let Ok(lang) = env::var("LC_ALL") {
        info.locale = Some(lang);
        return;
    }

    // Try locale command
    if let Ok(output) = Command::new("locale").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.starts_with("LANG=") {
                    info.locale = Some(line.trim_start_matches("LANG=").to_string());
                    return;
                }
            }
        }
    }
}

fn gather_timezone(info: &mut SystemInfo) {
    // Try /etc/timezone
    if let Ok(tz) = fs::read_to_string("/etc/timezone") {
        info.timezone = Some(tz.trim().to_string());
        return;
    }

    // Try TZ environment variable
    if let Ok(tz) = env::var("TZ") {
        info.timezone = Some(tz);
        return;
    }

    // Try /etc/localtime symlink
    if let Ok(target) = fs::read_link("/etc/localtime") {
        let path_str = target.to_string_lossy();
        if let Some(tz) = path_str.strip_prefix("/usr/share/zoneinfo/") {
            info.timezone = Some(tz.to_string());
            return;
        }
    }

    // Try timedatectl
    if let Ok(output) = Command::new("timedatectl").arg("status").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if let Some(tz) = line.split(':').nth(1) {
                    if line.contains("Time zone:") {
                        let tz = tz.trim().split_whitespace().next().unwrap_or(tz.trim());
                        info.timezone = Some(tz.to_string());
                        return;
                    }
                }
            }
        }
    }
}

fn gather_virtualization(info: &mut SystemInfo) {
    // Check systemd-detect-virt first
    if let Ok(output) = Command::new("systemd-detect-virt").output() {
        if output.status.success() {
            let virt = trim_lossy(&output.stdout);
            if virt != "none" && !virt.is_empty() {
                info.virtualization = Some(format_virt_name(&virt));
                return;
            }
        }
    }

    // Check DMI for VM indicators
    if let Ok(product) = fs::read_to_string("/sys/class/dmi/id/product_name") {
        let product = product.trim().to_lowercase();
        if product.contains("virtualbox") {
            info.virtualization = Some("VirtualBox".to_string());
            return;
        }
        if product.contains("vmware") {
            info.virtualization = Some("VMware".to_string());
            return;
        }
        if product.contains("kvm") || product.contains("qemu") {
            info.virtualization = Some("KVM/QEMU".to_string());
            return;
        }
        if product.contains("hyper-v") {
            info.virtualization = Some("Hyper-V".to_string());
            return;
        }
        if product.contains("xen") {
            info.virtualization = Some("Xen".to_string());
            return;
        }
    }

    // Check /proc/cpuinfo for hypervisor
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("hypervisor") {
            // We detected a hypervisor but don't know which
            info.virtualization = Some("Virtual Machine".to_string());
            return;
        }
    }

    // Check for WSL
    if let Ok(version) = fs::read_to_string("/proc/version") {
        if version.to_lowercase().contains("microsoft") {
            info.virtualization = Some("WSL".to_string());
            return;
        }
    }
}

fn format_virt_name(name: &str) -> String {
    match name {
        "kvm" => "KVM".to_string(),
        "qemu" => "QEMU".to_string(),
        "vmware" => "VMware".to_string(),
        "oracle" => "VirtualBox".to_string(),
        "xen" => "Xen".to_string(),
        "microsoft" => "Hyper-V".to_string(),
        "docker" => "Docker".to_string(),
        "podman" => "Podman".to_string(),
        "lxc" => "LXC".to_string(),
        "lxc-libvirt" => "LXC (libvirt)".to_string(),
        "systemd-nspawn" => "systemd-nspawn".to_string(),
        "openvz" => "OpenVZ".to_string(),
        "wsl" => "WSL".to_string(),
        _ => name.to_string(),
    }
}

fn gather_container(info: &mut SystemInfo) {
    // Check for Docker
    if Path::new("/.dockerenv").exists() {
        info.container = Some("Docker".to_string());
        return;
    }

    // Check cgroup for container indicators
    if let Ok(cgroup) = fs::read_to_string("/proc/1/cgroup") {
        if cgroup.contains("docker") {
            info.container = Some("Docker".to_string());
            return;
        }
        if cgroup.contains("lxc") {
            info.container = Some("LXC".to_string());
            return;
        }
        if cgroup.contains("kubepods") {
            info.container = Some("Kubernetes".to_string());
            return;
        }
    }

    // Check for Podman
    if let Ok(content) = fs::read_to_string("/run/.containerenv") {
        if content.contains("podman") || Path::new("/run/.containerenv").exists() {
            info.container = Some("Podman".to_string());
            return;
        }
    }

    // Check for systemd-nspawn
    if let Ok(content) = fs::read_to_string("/proc/1/environ") {
        if content.contains("container=systemd-nspawn") {
            info.container = Some("systemd-nspawn".to_string());
            return;
        }
    }
}

fn gather_security(info: &mut SystemInfo) {
    let mut security_modules = Vec::new();

    // Check SELinux
    if Path::new("/sys/fs/selinux").exists() {
        if let Ok(enforce) = fs::read_to_string("/sys/fs/selinux/enforce") {
            let mode = if enforce.trim() == "1" {
                "SELinux (Enforcing)"
            } else {
                "SELinux (Permissive)"
            };
            security_modules.push(mode.to_string());
        } else {
            security_modules.push("SELinux".to_string());
        }
    }

    // Check AppArmor
    if Path::new("/sys/kernel/security/apparmor").exists() {
        if let Ok(profiles) = fs::read_to_string("/sys/kernel/security/apparmor/profiles") {
            let count = profiles.lines().count();
            security_modules.push(format!("AppArmor ({} profiles)", count));
        } else {
            security_modules.push("AppArmor".to_string());
        }
    }

    // Check TOMOYO
    if Path::new("/sys/kernel/security/tomoyo").exists() {
        security_modules.push("TOMOYO".to_string());
    }

    // Check Smack
    if Path::new("/sys/fs/smackfs").exists() {
        security_modules.push("Smack".to_string());
    }

    // Check for seccomp (very common)
    // We skip this as it's too common and not that interesting

    if !security_modules.is_empty() {
        info.security = Some(security_modules.join(", "));
    }
}

fn gather_ssh(info: &mut SystemInfo) {
    // Check SSH_CONNECTION environment variable
    if let Ok(connection) = env::var("SSH_CONNECTION") {
        let parts: Vec<&str> = connection.split_whitespace().collect();
        if parts.len() >= 2 {
            let client_ip = parts[0];
            let client_port = parts[1];
            info.ssh_connection = Some(format!("{}:{}", client_ip, client_port));
        }
    } else if let Ok(client) = env::var("SSH_CLIENT") {
        let parts: Vec<&str> = client.split_whitespace().collect();
        if parts.len() >= 2 {
            info.ssh_connection = Some(format!("{}:{}", parts[0], parts[1]));
        }
    }
}

fn gather_bluetooth(info: &mut SystemInfo) {
    // Check if bluetooth is available and get status
    if let Ok(output) = Command::new("bluetoothctl").args(["show"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);

            let mut powered = false;
            let mut name = None;

            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("Powered:") {
                    powered = line.contains("yes");
                } else if let Some(value) = line.strip_prefix("Name:") {
                    name = Some(value.trim().to_string());
                }
            }

            if let Some(n) = name {
                if powered {
                    info.bluetooth = Some(format!("{} (On)", n));
                } else {
                    info.bluetooth = Some(format!("{} (Off)", n));
                }
            } else if powered {
                info.bluetooth = Some("On".to_string());
            } else {
                info.bluetooth = Some("Off".to_string());
            }
            return;
        }
    }

    // Check /sys for bluetooth
    if Path::new("/sys/class/bluetooth").exists() {
        if let Ok(entries) = fs::read_dir("/sys/class/bluetooth") {
            let count = entries.count();
            if count > 0 {
                info.bluetooth = Some(format!("{} adapter(s)", count));
            }
        }
    }
}

fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}
