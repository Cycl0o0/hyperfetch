use crate::info::SystemInfo;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn gather(info: &mut SystemInfo) {
    gather_display_server(info);
    gather_resolution(info);
    gather_de(info);
    gather_wm(info);
    gather_themes(info);
    gather_terminal(info);
    gather_shell(info);
}

fn gather_display_server(info: &mut SystemInfo) {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
        info.display_server = Some(format!("Wayland ({})", session_type));
    } else if env::var("DISPLAY").is_ok() {
        info.display_server = Some("X11".to_string());
    } else {
        info.display_server = Some("TTY".to_string());
    }
}

fn gather_resolution(info: &mut SystemInfo) {
    let mut resolutions = Vec::new();

    // Try xrandr first
    if let Ok(output) = Command::new("xrandr").arg("--current").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);

            for line in content.lines() {
                if line.contains(" connected") {
                    // Look for resolution in format like "1920x1080+0+0"
                    for part in line.split_whitespace() {
                        if part.contains('x') && part.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                            let res = part.split('+').next().unwrap_or(part);
                            push_resolution(&mut resolutions, res);
                        }
                    }
                }
            }

            if !resolutions.is_empty() {
                info.resolution = Some(resolutions.join(", "));
                return;
            }
        }
    }

    // Try wlr-randr for Wayland
    if let Ok(output) = Command::new("wlr-randr").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);

            for line in content.lines() {
                if line.contains("current") {
                    for part in line.split_whitespace() {
                        if part.contains('x') && part.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                            push_resolution(&mut resolutions, part);
                        }
                    }
                }
            }

            if !resolutions.is_empty() {
                info.resolution = Some(resolutions.join(", "));
                return;
            }
        }
    }

    // Try reading from /sys for framebuffer resolution
    if let Ok(content) = fs::read_to_string("/sys/class/graphics/fb0/virtual_size") {
        let res = content.trim().replace(',', "x");
        info.resolution = Some(res);
    }
}

fn push_resolution(resolutions: &mut Vec<String>, res: &str) {
    if !resolutions.contains(&res.to_string()) {
        resolutions.push(res.to_string());
    }
}

fn gather_de(info: &mut SystemInfo) {
    // Check environment variables
    let de = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("XDG_SESSION_DESKTOP"))
        .ok();

    if let Some(de_name) = de {
        let de_name = de_name.to_uppercase();
        let pretty_name = match de_name.as_str() {
            "GNOME" | "GNOME-XORG" | "GNOME-WAYLAND" => "GNOME",
            "KDE" | "PLASMA" | "KDE-PLASMA" => "KDE Plasma",
            "XFCE" | "XFCE4" => "Xfce",
            "MATE" => "MATE",
            "CINNAMON" => "Cinnamon",
            "LXDE" => "LXDE",
            "LXQT" => "LXQt",
            "BUDGIE" | "BUDGIE-DESKTOP" => "Budgie",
            "DEEPIN" => "Deepin",
            "PANTHEON" => "Pantheon",
            "UNITY" => "Unity",
            "ENLIGHTENMENT" => "Enlightenment",
            _ => &de_name,
        };
        info.de = Some(pretty_name.to_string());
    }
}

fn gather_wm(info: &mut SystemInfo) {
    // Check for common WM environment variables
    if let Ok(wm) = env::var("WAYLAND_WM") {
        info.wm = Some(wm);
        return;
    }

    // Try to detect from running processes
    let wm_list = [
        ("sway", "Sway"),
        ("hyprland", "Hyprland"),
        ("i3", "i3"),
        ("bspwm", "bspwm"),
        ("openbox", "Openbox"),
        ("fluxbox", "Fluxbox"),
        ("awesome", "awesome"),
        ("dwm", "dwm"),
        ("xmonad", "XMonad"),
        ("herbstluftwm", "herbstluftwm"),
        ("qtile", "Qtile"),
        ("spectrwm", "spectrwm"),
        ("river", "River"),
        ("wayfire", "Wayfire"),
        ("kwin", "KWin"),
        ("mutter", "Mutter"),
        ("xfwm4", "Xfwm4"),
        ("marco", "Marco"),
        ("muffin", "Muffin"),
        ("compiz", "Compiz"),
        ("metacity", "Metacity"),
        ("enlightenment", "Enlightenment"),
        ("icewm", "IceWM"),
        ("fvwm", "FVWM"),
        ("windowmaker", "Window Maker"),
        ("2bwm", "2bwm"),
    ];

    if let Ok(output) = Command::new("ps").args(["-e", "-o", "comm="]).output() {
        if output.status.success() {
            let procs = String::from_utf8_lossy(&output.stdout).to_lowercase();
            for (proc_name, wm_name) in wm_list {
                if procs.lines().any(|l| l.trim() == proc_name) {
                    info.wm = Some(wm_name.to_string());

                    // Get WM theme
                    gather_wm_theme(info, proc_name);
                    return;
                }
            }
        }
    }

    // Fallback: use wmctrl
    if let Ok(output) = Command::new("wmctrl").arg("-m").output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if let Some(name) = line.strip_prefix("Name:") {
                    info.wm = Some(name.trim().to_string());
                    break;
                }
            }
        }
    }
}

fn gather_wm_theme(info: &mut SystemInfo, wm_name: &str) {
    match wm_name {
        "i3" => {
            // i3 doesn't really have themes
            info.wm_theme = Some("N/A".to_string());
        }
        "openbox" => {
            let config_paths = [
                dirs::config_dir().map(|p| p.join("openbox/rc.xml")),
                Some(Path::new("/etc/xdg/openbox/rc.xml").to_path_buf()),
            ];

            for path in config_paths.into_iter().flatten() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(start) = content.find("<name>") {
                        if let Some(end) = content[start..].find("</name>") {
                            let theme = &content[start + 6..start + end];
                            info.wm_theme = Some(theme.to_string());
                            return;
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn gather_themes(info: &mut SystemInfo) {
    // GTK Theme
    gather_gtk_theme(info);

    // Icon Theme
    gather_icon_theme(info);

    // Cursor Theme
    gather_cursor_theme(info);
}

fn gather_gtk_theme(info: &mut SystemInfo) {
    // Try GTK 3 settings first
    if let Some(theme) = read_settings_value("gtk-theme-name", &[
        dirs::config_dir().map(|p| p.join("gtk-3.0/settings.ini")),
    ]) {
        info.theme = Some(theme);
        return;
    }

    // Try gsettings
    if let Some(theme) = gsettings_get("org.gnome.desktop.interface", "gtk-theme") {
        info.theme = Some(theme);
        return;
    }

    // Try .gtkrc-2.0
    if let Some(theme) = read_settings_value("gtk-theme-name", &[dirs::home_dir().map(|p| p.join(".gtkrc-2.0"))]) {
        info.theme = Some(theme.trim_matches('"').to_string());
    }
}

fn gather_icon_theme(info: &mut SystemInfo) {
    // Try GTK 3 settings
    if let Some(icons) = read_settings_value("gtk-icon-theme-name", &[
        dirs::config_dir().map(|p| p.join("gtk-3.0/settings.ini")),
    ]) {
        info.icons = Some(icons);
        return;
    }

    // Try gsettings
    if let Some(icons) = gsettings_get("org.gnome.desktop.interface", "icon-theme") {
        info.icons = Some(icons);
    }
}

fn gather_cursor_theme(info: &mut SystemInfo) {
    // Check environment variable
    if let Ok(cursor) = env::var("XCURSOR_THEME") {
        info.cursor = Some(cursor);
        return;
    }

    // Try GTK 3 settings
    if let Some(cursor) = read_settings_value("gtk-cursor-theme-name", &[
        dirs::config_dir().map(|p| p.join("gtk-3.0/settings.ini")),
    ]) {
        info.cursor = Some(cursor);
        return;
    }

    // Try gsettings
    if let Some(cursor) = gsettings_get("org.gnome.desktop.interface", "cursor-theme") {
        info.cursor = Some(cursor);
    }

    // Try ~/.icons/default/index.theme
    if let Some(cursor) = read_settings_value("Inherits", &[
        dirs::home_dir().map(|p| p.join(".icons/default/index.theme")),
    ]) {
        info.cursor = Some(cursor);
    }
}

fn gather_terminal(info: &mut SystemInfo) {
    // Check TERM_PROGRAM first
    if let Ok(term) = env::var("TERM_PROGRAM") {
        info.terminal = Some(format_terminal_name(&term));
        gather_terminal_font(info, &term);
        return;
    }

    // Try to detect from parent process
    if let Ok(ppid) = fs::read_to_string("/proc/self/stat") {
        let parts: Vec<&str> = ppid.split_whitespace().collect();
        if let Some(parent_pid) = parts.get(3) {
            if let Ok(parent_comm) = fs::read_to_string(format!("/proc/{}/comm", parent_pid)) {
                let term = parent_comm.trim().to_lowercase();

                // Walk up the process tree to find the terminal
                let terminal = find_terminal_in_parents(parent_pid.parse().unwrap_or(1));
                if let Some(t) = terminal {
                    info.terminal = Some(format_terminal_name(&t));
                    gather_terminal_font(info, &t);
                    return;
                }

                // Check if the parent is the terminal
                if is_terminal(&term) {
                    info.terminal = Some(format_terminal_name(&term));
                    gather_terminal_font(info, &term);
                    return;
                }
            }
        }
    }

    // Check common terminal environment variables
    let terminal_vars = ["TERMINAL", "TERM"];
    for var in terminal_vars {
        if let Ok(term) = env::var(var) {
            if !term.is_empty() && term != "dumb" && term != "linux" {
                info.terminal = Some(format_terminal_name(&term));
                gather_terminal_font(info, &term);
                return;
            }
        }
    }
}

fn find_terminal_in_parents(start_pid: u32) -> Option<String> {
    let mut pid = start_pid;

    for _ in 0..20 {
        // Max depth to prevent infinite loop
        if pid <= 1 {
            break;
        }

        if let Ok(comm) = fs::read_to_string(format!("/proc/{}/comm", pid)) {
            let name = comm.trim().to_lowercase();
            if is_terminal(&name) {
                return Some(name);
            }
        }

        // Get parent PID
        if let Ok(stat) = fs::read_to_string(format!("/proc/{}/stat", pid)) {
            let parts: Vec<&str> = stat.split_whitespace().collect();
            if let Some(ppid_str) = parts.get(3) {
                pid = ppid_str.parse().unwrap_or(0);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    None
}

fn is_terminal(name: &str) -> bool {
    let terminals = [
        "alacritty",
        "kitty",
        "konsole",
        "gnome-terminal",
        "xfce4-terminal",
        "mate-terminal",
        "tilix",
        "terminator",
        "urxvt",
        "rxvt",
        "xterm",
        "st",
        "foot",
        "wezterm",
        "termite",
        "sakura",
        "lxterminal",
        "qterminal",
        "terminology",
        "cool-retro-term",
        "hyper",
        "tabby",
        "contour",
        "warp",
        "rio",
        "ghostty",
    ];

    terminals.iter().any(|t| name.contains(t))
}

fn format_terminal_name(name: &str) -> String {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "alacritty" => "Alacritty",
        "kitty" => "Kitty",
        "konsole" => "Konsole",
        "gnome-terminal-server" | "gnome-terminal" => "GNOME Terminal",
        "xfce4-terminal" => "Xfce4 Terminal",
        "mate-terminal" => "MATE Terminal",
        "tilix" => "Tilix",
        "terminator" => "Terminator",
        "urxvt" | "urxvtd" => "URxvt",
        "rxvt" => "rxvt",
        "xterm" => "XTerm",
        "st" => "st",
        "foot" => "foot",
        "wezterm" | "wezterm-gui" => "WezTerm",
        "termite" => "Termite",
        "sakura" => "Sakura",
        "lxterminal" => "LXTerminal",
        "qterminal" => "QTerminal",
        "terminology" => "Terminology",
        "cool-retro-term" => "Cool Retro Term",
        "hyper" => "Hyper",
        "tabby" => "Tabby",
        "contour" => "Contour",
        "warp" => "Warp",
        "rio" => "Rio",
        "ghostty" => "Ghostty",
        _ => name,
    }
    .to_string()
}

fn gather_terminal_font(info: &mut SystemInfo, terminal: &str) {
    let term_lower = terminal.to_lowercase();

    if let Some(config_dir) = dirs::config_dir() {
        let font = match term_lower.as_str() {
            "alacritty" => {
                let config = config_dir.join("alacritty/alacritty.toml");
                if let Ok(content) = fs::read_to_string(&config) {
                    extract_alacritty_font(&content)
                } else {
                    let config = config_dir.join("alacritty/alacritty.yml");
                    fs::read_to_string(&config)
                        .ok()
                        .and_then(|c| extract_yaml_font(&c, "family"))
                }
            }
            "kitty" => {
                let config = config_dir.join("kitty/kitty.conf");
                fs::read_to_string(&config)
                    .ok()
                    .and_then(|c| extract_conf_value(&c, "font_family"))
            }
            "wezterm" | "wezterm-gui" => {
                let config = config_dir.join("wezterm/wezterm.lua");
                fs::read_to_string(&config)
                    .ok()
                    .and_then(|c| extract_lua_font(&c))
            }
            "foot" => {
                let config = config_dir.join("foot/foot.ini");
                fs::read_to_string(&config)
                    .ok()
                    .and_then(|c| extract_ini_value(&c, "font"))
            }
            _ => None,
        };

        if let Some(f) = font {
            info.terminal_font = Some(f);
        }
    }
}

fn extract_alacritty_font(content: &str) -> Option<String> {
    // TOML format
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("family") && line.contains('=') {
            return line
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn extract_yaml_font(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) && line.contains(':') {
            return line
                .split(':')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }
    None
}

fn extract_conf_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) {
            let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
            if parts.len() >= 2 {
                return Some(parts[1].trim().to_string());
            }
        }
    }
    None
}

fn extract_lua_font(content: &str) -> Option<String> {
    // Look for font_family = "..." pattern
    for line in content.lines() {
        if line.contains("font") && line.contains('=') {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    return Some(line[start + 1..start + 1 + end].to_string());
                }
            }
        }
    }
    None
}

fn extract_ini_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) && line.contains('=') {
            return line.split('=').nth(1).map(|s| s.trim().to_string());
        }
    }
    None
}

fn read_settings_value(key: &str, paths: &[Option<std::path::PathBuf>]) -> Option<String> {
    for path in paths.iter().flatten() {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix(&format!("{key}=")) {
                    let trimmed = value.trim().trim_matches('\'');
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
        }
    }
    None
}

fn gsettings_get(schema: &str, key: &str) -> Option<String> {
    let output = Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\'')
        .to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn gather_shell(info: &mut SystemInfo) {
    // Get shell from environment
    if let Ok(shell_path) = env::var("SHELL") {
        let shell_name = Path::new(&shell_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or(shell_path.clone());

        info.shell = Some(shell_name.clone());

        // Get shell version
        let version_flag = match shell_name.as_str() {
            "bash" => Some("--version"),
            "zsh" => Some("--version"),
            "fish" => Some("--version"),
            "dash" => None,
            "sh" => None,
            "tcsh" | "csh" => Some("--version"),
            "ksh" => Some("--version"),
            "nu" | "nushell" => Some("--version"),
            "pwsh" | "powershell" => Some("--version"),
            "elvish" => Some("--version"),
            "ion" => Some("--version"),
            "xonsh" => Some("--version"),
            _ => Some("--version"),
        };

        if let Some(flag) = version_flag {
            if let Ok(output) = Command::new(&shell_path).arg(flag).output() {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    let version = extract_version(&version_output, &shell_name);
                    if let Some(v) = version {
                        info.shell_version = Some(v);
                    }
                }
            }
        }
    }
}

fn extract_version(output: &str, shell: &str) -> Option<String> {
    let first_line = output.lines().next()?;

    match shell {
        "bash" => {
            // "GNU bash, version 5.1.16(1)-release"
            first_line
                .split("version ")
                .nth(1)
                .map(|s| s.split('(').next().unwrap_or(s).trim().to_string())
        }
        "zsh" => {
            // "zsh 5.8.1 (x86_64-pc-linux-gnu)"
            first_line.split_whitespace().nth(1).map(|s| s.to_string())
        }
        "fish" => {
            // "fish, version 3.3.1"
            first_line
                .split("version ")
                .nth(1)
                .map(|s| s.trim().to_string())
        }
        _ => {
            // Generic: try to find a version number
            for word in first_line.split_whitespace() {
                if word
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_digit())
                {
                    return Some(word.to_string());
                }
            }
            None
        }
    }
}
