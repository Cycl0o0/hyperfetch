use crate::info::{NetworkInterface, PublicIpInfo, SystemInfo};
use std::fs;
use std::process::Command;

pub fn gather(info: &mut SystemInfo, fetch_public_ip: bool) {
    gather_interfaces(info);

    if fetch_public_ip {
        gather_public_ip(info);
    }
}

fn gather_interfaces(info: &mut SystemInfo) {
    let net_path = "/sys/class/net";

    if let Ok(entries) = fs::read_dir(net_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip loopback
            if name == "lo" {
                continue;
            }

            let iface_path = entry.path();

            // Get interface state
            let state = read_trimmed(iface_path.join("operstate").as_path());

            // Skip interfaces that are down (optional - you might want to show them)
            // if state.as_deref() == Some("down") {
            //     continue;
            // }

            // Get MAC address
            let mac = read_trimmed(iface_path.join("address").as_path())
                .filter(|s| s != "00:00:00:00:00:00");

            // Get speed (if available and up)
            let speed = if state.as_deref() == Some("up") {
                read_trimmed(iface_path.join("speed").as_path())
                    .and_then(|s| s.parse::<i32>().ok())
                    .filter(|&s| s > 0)
                    .map(|s| format!("{} Mbps", s))
            } else {
                None
            };

            let mut interface = NetworkInterface {
                name: name.clone(),
                ipv4: None,
                ipv6: None,
                mac,
                speed,
                state,
            };

            // Get IP addresses using ip command
            if let Ok(output) = Command::new("ip").args(["addr", "show", &name]).output() {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    for line in content.lines() {
                        let line = line.trim();
                        if let Some(ip) = parse_ip_line(line, "inet ") {
                            interface.ipv4 = Some(ip);
                        } else if let Some(ip) = parse_ip_line(line, "inet6 ") {
                            if !ip.starts_with("fe80") {
                                interface.ipv6 = Some(ip);
                            }
                        }
                    }
                }
            }

            // Only add interfaces that have at least one IP or are interesting
            if interface.ipv4.is_some()
                || interface.ipv6.is_some()
                || interface.state.as_deref() == Some("up")
            {
                info.interfaces.push(interface);
            }
        }
    }
}

#[cfg(feature = "network")]
fn gather_public_ip(info: &mut SystemInfo) {
    // Use ip-api.com for IP + geolocation (free, no API key needed)
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    // Try ip-api.com first (includes geolocation)
    if let Ok(response) = client.get("http://ip-api.com/json/?fields=query,country,regionName,city,zip,isp").send() {
        if let Ok(text) = response.text() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                let ip_info = PublicIpInfo {
                    ip: json["query"].as_str().unwrap_or("").to_string(),
                    country: json["country"].as_str().map(|s| s.to_string()),
                    region: json["regionName"].as_str().map(|s| s.to_string()),
                    city: json["city"].as_str().map(|s| s.to_string()),
                    zip: json["zip"].as_str().map(|s| s.to_string()),
                    isp: json["isp"].as_str().map(|s| s.to_string()),
                };

                if !ip_info.ip.is_empty() {
                    info.public_ip = Some(ip_info);
                    return;
                }
            }
        }
    }

    // Fallback to simple IP services
    let ip_services = [
        "https://api.ipify.org",
        "https://icanhazip.com",
        "https://ifconfig.me/ip",
    ];

    for service in ip_services {
        if let Ok(response) = client.get(service).send() {
            if let Ok(ip) = response.text() {
                let ip = ip.trim().to_string();
                if !ip.is_empty() && ip.len() < 50 {
                    info.public_ip = Some(PublicIpInfo {
                        ip,
                        country: None,
                        region: None,
                        city: None,
                        zip: None,
                        isp: None,
                    });
                    return;
                }
            }
        }
    }
}

#[cfg(not(feature = "network"))]
fn gather_public_ip(_info: &mut SystemInfo) {
    // Network feature not enabled
}

fn read_trimmed(path: &std::path::Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn parse_ip_line(line: &str, prefix: &str) -> Option<String> {
    if !line.starts_with(prefix) {
        return None;
    }
    let ip = line.split_whitespace().nth(1)?;
    let ip_only = ip.split('/').next().unwrap_or(ip);
    Some(ip_only.to_string())
}
