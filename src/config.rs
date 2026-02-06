use colored::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayOptions,

    #[serde(default)]
    pub colors: ColorOptions,

    #[serde(default)]
    pub info: InfoOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayOptions {
    #[serde(default = "default_true")]
    pub show_ascii: bool,

    #[serde(default = "default_true")]
    pub show_colors: bool,

    #[serde(default)]
    pub small_ascii: bool,

    #[serde(default)]
    pub ascii_distro: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorOptions {
    #[serde(default = "default_cyan")]
    pub primary: String,

    #[serde(default = "default_white")]
    pub secondary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoOptions {
    #[serde(default = "default_true")]
    pub os: bool,
    #[serde(default = "default_true")]
    pub kernel: bool,
    #[serde(default = "default_true")]
    pub hostname: bool,
    #[serde(default = "default_true")]
    pub uptime: bool,
    #[serde(default = "default_true")]
    pub packages: bool,
    #[serde(default = "default_true")]
    pub shell: bool,
    #[serde(default = "default_true")]
    pub resolution: bool,
    #[serde(default = "default_true")]
    pub de: bool,
    #[serde(default = "default_true")]
    pub wm: bool,
    #[serde(default = "default_true")]
    pub theme: bool,
    #[serde(default = "default_true")]
    pub icons: bool,
    #[serde(default = "default_true")]
    pub terminal: bool,
    #[serde(default = "default_true")]
    pub cpu: bool,
    #[serde(default = "default_true")]
    pub gpu: bool,
    #[serde(default = "default_true")]
    pub memory: bool,
    #[serde(default = "default_true")]
    pub disk: bool,
    #[serde(default = "default_true")]
    pub network: bool,
    #[serde(default = "default_true")]
    pub battery: bool,
    #[cfg(feature = "network")]
    #[serde(default)]
    pub public_ip: bool,
}

fn default_true() -> bool {
    true
}

fn default_cyan() -> String {
    "auto".to_string()
}

fn default_white() -> String {
    "white".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayOptions::default(),
            colors: ColorOptions::default(),
            info: InfoOptions::default(),
        }
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_ascii: true,
            show_colors: true,
            small_ascii: false,
            ascii_distro: None,
        }
    }
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            primary: "auto".to_string(),
            secondary: "white".to_string(),
        }
    }
}

impl Default for InfoOptions {
    fn default() -> Self {
        Self {
            os: true,
            kernel: true,
            hostname: true,
            uptime: true,
            packages: true,
            shell: true,
            resolution: true,
            de: true,
            wm: true,
            theme: true,
            icons: true,
            terminal: true,
            cpu: true,
            gpu: true,
            memory: true,
            disk: true,
            network: true,
            battery: true,
            #[cfg(feature = "network")]
            public_ip: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn load_from(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("hyperfetch").join("config.toml"))
    }

    pub fn primary_color(&self) -> Color {
        parse_color(&self.colors.primary)
    }
}

fn parse_color(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" | "purple" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "bright_black" | "brightblack" => Color::BrightBlack,
        "bright_red" | "brightred" => Color::BrightRed,
        "bright_green" | "brightgreen" => Color::BrightGreen,
        "bright_yellow" | "brightyellow" => Color::BrightYellow,
        "bright_blue" | "brightblue" => Color::BrightBlue,
        "bright_magenta" | "brightmagenta" => Color::BrightMagenta,
        "bright_cyan" | "brightcyan" => Color::BrightCyan,
        "bright_white" | "brightwhite" => Color::BrightWhite,
        _ => Color::Cyan,
    }
}
