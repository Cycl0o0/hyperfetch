use crate::info::{PackageCount, SystemInfo};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn gather(info: &mut SystemInfo) {
    let mut counts = Vec::new();

    // Native package managers
    if let Some(count) = count_pacman() {
        counts.push(PackageCount {
            manager: "pacman".to_string(),
            count,
        });
    }

    if let Some(count) = count_apt() {
        counts.push(PackageCount {
            manager: "apt".to_string(),
            count,
        });
    }

    if let Some(count) = count_dnf() {
        counts.push(PackageCount {
            manager: "dnf".to_string(),
            count,
        });
    }

    if let Some(count) = count_rpm() {
        counts.push(PackageCount {
            manager: "rpm".to_string(),
            count,
        });
    }

    if let Some(count) = count_emerge() {
        counts.push(PackageCount {
            manager: "emerge".to_string(),
            count,
        });
    }

    if let Some(count) = count_xbps() {
        counts.push(PackageCount {
            manager: "xbps".to_string(),
            count,
        });
    }

    if let Some(count) = count_apk() {
        counts.push(PackageCount {
            manager: "apk".to_string(),
            count,
        });
    }

    if let Some(count) = count_zypper() {
        counts.push(PackageCount {
            manager: "zypper".to_string(),
            count,
        });
    }

    if let Some(count) = count_nix() {
        counts.push(PackageCount {
            manager: "nix".to_string(),
            count,
        });
    }

    if let Some(count) = count_guix() {
        counts.push(PackageCount {
            manager: "guix".to_string(),
            count,
        });
    }

    // Universal/third-party package managers
    if let Some(count) = count_flatpak() {
        counts.push(PackageCount {
            manager: "flatpak".to_string(),
            count,
        });
    }

    if let Some(count) = count_snap() {
        counts.push(PackageCount {
            manager: "snap".to_string(),
            count,
        });
    }

    if let Some(count) = count_brew() {
        counts.push(PackageCount {
            manager: "brew".to_string(),
            count,
        });
    }

    // Language package managers
    if let Some(count) = count_cargo() {
        counts.push(PackageCount {
            manager: "cargo".to_string(),
            count,
        });
    }

    if let Some(count) = count_pip() {
        counts.push(PackageCount {
            manager: "pip".to_string(),
            count,
        });
    }

    if let Some(count) = count_npm() {
        counts.push(PackageCount {
            manager: "npm".to_string(),
            count,
        });
    }

    if let Some(count) = count_gem() {
        counts.push(PackageCount {
            manager: "gem".to_string(),
            count,
        });
    }

    if let Some(count) = count_go() {
        counts.push(PackageCount {
            manager: "go".to_string(),
            count,
        });
    }

    // Calculate total and format string
    let total: u32 = counts.iter().map(|c| c.count).sum();

    if !counts.is_empty() {
        let details: Vec<String> = counts
            .iter()
            .map(|c| format!("{} ({})", c.count, c.manager))
            .collect();

        info.packages = Some(format!("{} ({})", total, details.join(", ")));
        info.package_counts = counts;
    }
}

fn count_pacman() -> Option<u32> {
    let db_path = Path::new("/var/lib/pacman/local");
    if db_path.exists() {
        fs::read_dir(db_path)
            .ok()
            .map(|entries| entries.filter_map(|e| e.ok()).count() as u32 - 1) // -1 for ALPM_DB_VERSION
    } else {
        None
    }
}

fn count_apt() -> Option<u32> {
    let dpkg_path = Path::new("/var/lib/dpkg/status");
    if dpkg_path.exists() {
        if let Ok(content) = fs::read_to_string(dpkg_path) {
            let count = content
                .split("\n\n")
                .filter(|pkg| {
                    pkg.contains("Status: install ok installed")
                        || pkg.contains("Status: hold ok installed")
                })
                .count();
            return Some(count as u32);
        }
    }
    None
}

fn count_dnf() -> Option<u32> {
    // DNF uses RPM database, so we use rpm command
    if !Path::new("/usr/bin/dnf").exists() {
        return None;
    }
    count_rpm()
}

fn count_rpm() -> Option<u32> {
    if let Ok(output) = Command::new("rpm").args(["-qa", "--last"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            return Some(content.lines().count() as u32);
        }
    }
    None
}

fn count_emerge() -> Option<u32> {
    let db_path = Path::new("/var/db/pkg");
    if db_path.exists() {
        let mut count = 0u32;
        if let Ok(categories) = fs::read_dir(db_path) {
            for cat in categories.flatten() {
                if cat.path().is_dir() {
                    if let Ok(packages) = fs::read_dir(cat.path()) {
                        count += packages.filter_map(|e| e.ok()).count() as u32;
                    }
                }
            }
        }
        if count > 0 {
            return Some(count);
        }
    }
    None
}

fn count_xbps() -> Option<u32> {
    if let Ok(output) = Command::new("xbps-query").arg("-l").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            return Some(content.lines().count() as u32);
        }
    }
    None
}

fn count_apk() -> Option<u32> {
    if let Ok(output) = Command::new("apk").args(["info"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            return Some(content.lines().count() as u32);
        }
    }
    None
}

fn count_zypper() -> Option<u32> {
    if let Ok(output) = Command::new("rpm").args(["-qa"]).output() {
        if output.status.success() && Path::new("/usr/bin/zypper").exists() {
            let content = String::from_utf8_lossy(&output.stdout);
            return Some(content.lines().count() as u32);
        }
    }
    None
}

fn count_nix() -> Option<u32> {
    // Count installed packages in user profile
    if let Some(home) = dirs::home_dir() {
        let nix_profile = home.join(".nix-profile/manifest.nix");
        if nix_profile.exists() {
            if let Ok(output) = Command::new("nix-env").args(["-q"]).output() {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    return Some(content.lines().count() as u32);
                }
            }
        }
    }
    None
}

fn count_guix() -> Option<u32> {
    if let Ok(output) = Command::new("guix").args(["package", "-I"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            return Some(content.lines().count() as u32);
        }
    }
    None
}

fn count_flatpak() -> Option<u32> {
    if let Ok(output) = Command::new("flatpak")
        .args(["list", "--app"])
        .output()
    {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let count = content.lines().count() as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_snap() -> Option<u32> {
    if let Ok(output) = Command::new("snap").arg("list").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            // Subtract 1 for header line
            let count = content.lines().count().saturating_sub(1) as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_brew() -> Option<u32> {
    // Check for Homebrew on Linux (Linuxbrew)
    let brew_path = Path::new("/home/linuxbrew/.linuxbrew/bin/brew");
    let brew_cmd = if brew_path.exists() {
        brew_path.to_string_lossy().to_string()
    } else if Path::new("/usr/local/bin/brew").exists() {
        "/usr/local/bin/brew".to_string()
    } else {
        return None;
    };

    if let Ok(output) = Command::new(&brew_cmd).args(["list", "--formula", "-1"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let count = content.lines().count() as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_cargo() -> Option<u32> {
    if let Some(home) = dirs::home_dir() {
        let cargo_bin = home.join(".cargo/bin");
        if cargo_bin.exists() {
            if let Ok(entries) = fs::read_dir(&cargo_bin) {
                let count = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().is_file()
                            && !e.file_name().to_string_lossy().starts_with("cargo")
                            && !e.file_name().to_string_lossy().starts_with("rust")
                    })
                    .count() as u32;
                if count > 0 {
                    return Some(count);
                }
            }
        }
    }
    None
}

fn count_pip() -> Option<u32> {
    // Count packages in pip --user or virtualenv
    if let Ok(output) = Command::new("pip").args(["list", "--format=freeze"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let count = content.lines().count() as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_npm() -> Option<u32> {
    if let Ok(output) = Command::new("npm").args(["list", "-g", "--depth=0"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            // Subtract 1 for the first line which shows the path
            let count = content.lines().count().saturating_sub(1) as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_gem() -> Option<u32> {
    if let Ok(output) = Command::new("gem").args(["list", "--no-details"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let count = content.lines().count() as u32;
            if count > 0 {
                return Some(count);
            }
        }
    }
    None
}

fn count_go() -> Option<u32> {
    if let Some(home) = dirs::home_dir() {
        let go_bin = home.join("go/bin");
        if go_bin.exists() {
            if let Ok(entries) = fs::read_dir(&go_bin) {
                let count = entries.filter_map(|e| e.ok()).count() as u32;
                if count > 0 {
                    return Some(count);
                }
            }
        }
    }
    None
}
