use clap::Parser;
use colored::control::set_override;

mod ascii;
mod config;
mod display;
mod info;

use ascii::AsciiArt;
use config::Config;
use display::{DisplayConfig, list_logos, print_info, print_json, print_logo_only};
use info::SystemInfo;

/// Hyperfetch - A comprehensive system information tool
#[derive(Parser, Debug)]
#[command(name = "hyperfetch")]
#[command(author = "cycl0o0")]
#[command(version = "1.0.0")]
#[command(about = "A comprehensive system information tool - neofetch alternative with extended features")]
#[command(long_about = None)]
struct Args {
    /// Use custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Use specific distro's ASCII art
    #[arg(short, long, value_name = "DISTRO")]
    ascii: Option<String>,

    /// Disable ASCII art display
    #[arg(long)]
    no_ascii: bool,

    /// Disable colored output
    #[arg(long)]
    no_colors: bool,

    /// Use small ASCII art
    #[arg(short, long)]
    small: bool,

    /// Print only the ASCII logo
    #[arg(short, long)]
    logo_only: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Fetch and display public IP with geolocation
    #[cfg(feature = "network")]
    #[arg(long)]
    public_ip: bool,

    /// List available ASCII logos
    #[arg(long)]
    list_logos: bool,
}

fn main() {
    let args = Args::parse();

    // Handle --list-logos
    if args.list_logos {
        list_logos();
        return;
    }

    // Load configuration
    let config = if let Some(ref path) = args.config {
        match Config::load_from(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                Config::default()
            }
        }
    } else {
        Config::load()
    };

    // Handle colors
    let use_colors = !args.no_colors && config.display.show_colors;
    set_override(use_colors);

    // Determine ASCII settings
    let show_ascii = !args.no_ascii && config.display.show_ascii;
    let small_ascii = args.small || config.display.small_ascii;

    // Determine distro for ASCII art
    let ascii_distro = args
        .ascii
        .as_deref()
        .or(config.display.ascii_distro.as_deref());

    // Handle --logo-only
    if args.logo_only {
        let detected_distro = std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|l| l.starts_with("ID="))
                    .map(|l| {
                        l.trim_start_matches("ID=")
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string()
                    })
            });

        #[cfg(target_os = "macos")]
        let detected_distro = detected_distro.or_else(|| Some("macos".to_string()));

        let distro = ascii_distro.or(detected_distro.as_deref());

        let ascii = AsciiArt::for_distro(distro, small_ascii);
        let display_config = DisplayConfig {
            use_colors,
            ..Default::default()
        };
        print_logo_only(&ascii, &display_config);
        return;
    }

    // Determine if we should fetch public IP
    #[cfg(feature = "network")]
    let fetch_public_ip = args.public_ip || config.info.public_ip;
    #[cfg(not(feature = "network"))]
    let fetch_public_ip = false;

    // Gather system information
    let info = SystemInfo::gather(fetch_public_ip);

    // Handle --json output
    if args.json {
        print_json(&info);
        return;
    }

    // Get ASCII art
    let distro = ascii_distro
        .map(|s| s.to_string())
        .or_else(|| info.os_id.clone());
    let ascii = AsciiArt::for_distro(distro.as_deref(), small_ascii);

    // Build display config
    let primary_color = if config.colors.primary.to_lowercase() == "auto"
        || config.colors.primary.to_lowercase() == "distro"
    {
        ascii.colors.first().copied().unwrap_or(config.primary_color())
    } else {
        config.primary_color()
    };

    let display_config = DisplayConfig {
        show_ascii,
        use_colors,
        primary_color,
    };

    // Print everything
    print_info(&info, &ascii, &display_config);
}
