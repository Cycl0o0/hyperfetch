use crate::ascii::AsciiArt;
use crate::info::SystemInfo;
use colored::{Color, Colorize};

pub struct DisplayConfig {
    pub show_ascii: bool,
    pub use_colors: bool,
    pub primary_color: Color,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_ascii: true,
            use_colors: true,
            primary_color: Color::Cyan,
        }
    }
}

pub fn print_info(info: &SystemInfo, ascii: &AsciiArt, config: &DisplayConfig) {
    let mut info_lines = Vec::new();

    // System
    push_opt(&mut info_lines, "OS", info.os.as_deref(), config);
    push_opt(&mut info_lines, "Kernel", info.kernel.as_deref(), config);
    push_opt(&mut info_lines, "Host", info.hostname.as_deref(), config);
    push_opt(&mut info_lines, "Uptime", info.uptime.as_deref(), config);
    push_opt(&mut info_lines, "Machine", info.machine_type.as_deref(), config);
    push_opt(&mut info_lines, "Init", info.init_system.as_deref(), config);
    push_opt(&mut info_lines, "Packages", info.packages.as_deref(), config);

    if let Some(shell) = info.shell.as_ref() {
        let shell_str = info
            .shell_version
            .as_ref()
            .map(|ver| format!("{shell} {ver}"))
            .unwrap_or_else(|| shell.clone());
        info_lines.push(format_line("Shell", &shell_str, config));
    }

    push_opt(&mut info_lines, "Display", info.display_server.as_deref(), config);
    push_opt(&mut info_lines, "Resolution", info.resolution.as_deref(), config);
    push_opt(&mut info_lines, "DE", info.de.as_deref(), config);
    push_opt(&mut info_lines, "WM", info.wm.as_deref(), config);
    push_opt(&mut info_lines, "Theme", info.theme.as_deref(), config);
    push_opt(&mut info_lines, "Icons", info.icons.as_deref(), config);
    push_opt(&mut info_lines, "Cursor", info.cursor.as_deref(), config);

    if let Some(terminal) = info.terminal.as_ref() {
        let term_str = info
            .terminal_font
            .as_ref()
            .map(|font| format!("{terminal} ({font})"))
            .unwrap_or_else(|| terminal.clone());
        info_lines.push(format_line("Terminal", &term_str, config));
    }

    // Hardware
    push_sep(&mut info_lines);
    if let Some(cpu) = info.cpu.as_ref() {
        let threads = info
            .cpu_threads
            .map(|t| t.to_string())
            .unwrap_or_else(|| "?".to_string());
        let freq = info.cpu_freq.as_deref().unwrap_or("?");
        let cpu_str = format!("{cpu} ({threads}) @ {freq}");
        info_lines.push(format_line("CPU", &cpu_str, config));
    }
    push_opt(&mut info_lines, "Arch", info.cpu_arch.as_deref(), config);
    push_opt(&mut info_lines, "Cache", info.cpu_cache.as_deref(), config);
    push_opt(&mut info_lines, "CPU Temp", info.cpu_temp.as_deref(), config);
    push_opt(&mut info_lines, "Governor", info.cpu_governor.as_deref(), config);

    for gpu in &info.gpu {
        let mut gpu_str = gpu.name.clone();
        if let Some(ref driver) = gpu.driver {
            gpu_str.push_str(&format!(" [{}]", driver));
        }
        if let Some(ref vram) = gpu.vram {
            gpu_str.push_str(&format!(" ({})", vram));
        }
        if let Some(ref temp) = gpu.temp {
            gpu_str.push_str(&format!(" @ {}", temp));
        }
        info_lines.push(format_line("GPU", &gpu_str, config));
    }

    push_opt(&mut info_lines, "Memory", info.memory.as_deref(), config);
    push_opt(&mut info_lines, "Swap", info.swap.as_deref(), config);
    push_opt(&mut info_lines, "Load", info.load_average.as_deref(), config);
    if let Some(procs) = info.processes {
        info_lines.push(format_line("Processes", &procs.to_string(), config));
    }

    // Disks
    for disk in &info.disks {
        let disk_str = format!(
            "{} / {} ({}%) [{}{}]",
            disk.used,
            disk.size,
            disk.percent,
            disk.filesystem,
            disk.disk_type
                .as_ref()
                .map(|t| format!(", {}", t))
                .unwrap_or_default()
        );
        info_lines.push(format_line(&format!("Disk ({})", disk.mount), &disk_str, config));
    }

    // Board info
    push_opt(&mut info_lines, "Board", info.motherboard.as_deref(), config);
    push_opt(&mut info_lines, "BIOS", info.bios.as_deref(), config);

    // Network
    push_sep(&mut info_lines);
    for iface in &info.interfaces {
        let mut parts = Vec::new();
        if let Some(ref ip) = iface.ipv4 {
            parts.push(ip.clone());
        }
        if let Some(ref ip) = iface.ipv6 {
            parts.push(ip.clone());
        }
        if let Some(ref speed) = iface.speed {
            parts.push(speed.clone());
        }
        if let Some(ref state) = iface.state {
            if state != "up" {
                parts.push(format!("[{}]", state));
            }
        }
        if !parts.is_empty() {
            info_lines.push(format_line(&format!("Net ({})", iface.name), &parts.join(", "), config));
        }
    }

    #[cfg(feature = "network")]
    if let Some(public_ip) = info.public_ip.as_ref() {
        let mut ip_str = public_ip.ip.clone();
        let mut location_parts = Vec::new();
        if let Some(city) = public_ip.city.as_ref() {
            location_parts.push(city.clone());
        }
        if let Some(region) = public_ip.region.as_ref() {
            location_parts.push(region.clone());
        }
        if let Some(country) = public_ip.country.as_ref() {
            location_parts.push(country.clone());
        }
        if let Some(zip) = public_ip.zip.as_ref() {
            location_parts.push(zip.clone());
        }
        if !location_parts.is_empty() {
            ip_str.push_str(&format!(" ({})", location_parts.join(", ")));
        }
        if let Some(isp) = public_ip.isp.as_ref() {
            ip_str.push_str(&format!(" [{}]", isp));
        }
        info_lines.push(format_line("Public IP", &ip_str, config));
    }

    // Power
    if let Some(battery) = info.battery.as_ref() {
        let mut bat_str = format!("{}% ({})", battery.percent, battery.status);
        if let Some(time) = battery.time_remaining.as_ref() {
            bat_str.push_str(&format!(" ~{}", time));
        }
        info_lines.push(format_line("Battery", &bat_str, config));
    }
    push_opt(&mut info_lines, "Brightness", info.brightness.as_deref(), config);

    // Audio
    push_opt(&mut info_lines, "Audio", info.audio_device.as_deref(), config);
    push_opt(&mut info_lines, "Volume", info.volume.as_deref(), config);

    // Misc
    push_sep(&mut info_lines);
    push_opt(&mut info_lines, "Locale", info.locale.as_deref(), config);
    push_opt(&mut info_lines, "Timezone", info.timezone.as_deref(), config);
    push_opt(&mut info_lines, "Boot Time", info.boot_time.as_deref(), config);
    push_opt(&mut info_lines, "Users", info.logged_users.as_deref(), config);
    push_opt(&mut info_lines, "Virt", info.virtualization.as_deref(), config);
    push_opt(&mut info_lines, "Container", info.container.as_deref(), config);
    push_opt(&mut info_lines, "Security", info.security.as_deref(), config);
    push_opt(&mut info_lines, "SSH", info.ssh_connection.as_deref(), config);
    push_opt(&mut info_lines, "Bluetooth", info.bluetooth.as_deref(), config);

    // Add color bar
    info_lines.push(String::new());
    info_lines.push(color_bar());

    // Print combined output
    if config.show_ascii {
        print_with_ascii(ascii, &info_lines, config);
    } else {
        for line in info_lines {
            println!("{}", line);
        }
    }
}

fn format_line(label: &str, value: &str, config: &DisplayConfig) -> String {
    if config.use_colors {
        format!(
            "{}{} {}",
            label.color(config.primary_color).bold(),
            ":".color(config.primary_color),
            value
        )
    } else {
        format!("{}: {}", label, value)
    }
}

fn push_opt(lines: &mut Vec<String>, label: &str, value: Option<&str>, config: &DisplayConfig) {
    if let Some(value) = value {
        lines.push(format_line(label, value, config));
    }
}

fn push_sep(lines: &mut Vec<String>) {
    lines.push(String::new());
}

fn print_with_ascii(ascii: &AsciiArt, info_lines: &[String], config: &DisplayConfig) {
    let ascii_width = ascii.width + 2; // Add padding
    let primary_color = ascii.colors.first().copied().unwrap_or(Color::White);

    let max_lines = ascii.lines.len().max(info_lines.len());

    for i in 0..max_lines {
        // Print ASCII art line (or padding)
        let ascii_line = if i < ascii.lines.len() {
            let rendered = ascii.render_line(i, config.use_colors, primary_color);
            let visible_len = ascii.line_visible_width(i);
            let padding = if ascii_width > visible_len {
                ascii_width - visible_len
            } else {
                0
            };
            format!("{}{}", rendered, " ".repeat(padding))
        } else {
            " ".repeat(ascii_width)
        };

        // Print info line
        let info_line = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        println!("{}{}", ascii_line, info_line);
    }
}

fn color_bar() -> String {
    let colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let mut bar = String::new();
    for color in &colors {
        bar.push_str(&"   ".on_color(*color).to_string());
    }
    bar.push_str("\n");
    for color in &colors {
        bar.push_str(&"   ".on_color(*color).to_string());
    }

    bar
}

pub fn print_logo_only(ascii: &AsciiArt, config: &DisplayConfig) {
    let primary_color = ascii.colors.first().copied().unwrap_or(Color::White);

    for i in 0..ascii.lines.len() {
        let rendered = ascii.render_line(i, config.use_colors, primary_color);
        println!("{}", rendered);
    }
}

pub fn print_json(info: &SystemInfo) {
    match serde_json::to_string_pretty(info) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

pub fn list_logos() {
    println!("Available ASCII logos:");
    for logo in crate::ascii::AsciiArt::list_available() {
        println!("  - {}", logo);
    }
}
