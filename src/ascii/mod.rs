pub mod logos;

use std::fs;
use std::path::{Path, PathBuf};

use colored::{Color, Colorize};

pub struct AsciiArt {
    pub lines: Vec<&'static str>,
    pub colors: Vec<Color>,
    pub width: usize,
}

impl AsciiArt {
    pub fn for_distro(distro_id: Option<&str>, small: bool) -> Self {
        let id = distro_id.unwrap_or("linux").to_lowercase();

        if let Some(from_file) = load_from_ascii_dir(&id, small) {
            return from_file;
        }

        if small {
            return logos::get_small_logo(&id);
        }

        logos::get_logo(&id)
    }

    pub fn list_available() -> Vec<&'static str> {
        if let Some(list) = list_from_ascii_dir() {
            return list;
        }

        logos::AVAILABLE_LOGOS.to_vec()
    }

    pub fn render_line(&self, index: usize, use_colors: bool, fallback: Color) -> String {
        if index >= self.lines.len() {
            return String::new();
        }

        let line = self.lines[index];
        render_with_palette(line, &self.colors, use_colors, fallback)
    }

    pub fn line_visible_width(&self, index: usize) -> usize {
        if index >= self.lines.len() {
            return 0;
        }

        strip_color_tokens(self.lines[index])
            .chars()
            .count()
    }
}

fn ascii_dir() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("HYPERFETCH_ASCII_DIR") {
        let dir = PathBuf::from(path);
        if dir.is_dir() {
            return Some(dir);
        }
    }

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ascii");
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

fn load_from_ascii_dir(id: &str, small: bool) -> Option<AsciiArt> {
    let dir = ascii_dir()?;
    let mut candidates = Vec::new();

    if small {
        candidates.push(format!("{id}_small.txt"));
        candidates.push(format!("{id}.txt"));
    } else {
        candidates.push(format!("{id}.txt"));
    }

    for candidate in candidates {
        let path = dir.join(candidate);
        if !path.exists() {
            continue;
        }

        let content = fs::read_to_string(path).ok()?;
        let mut lines = Vec::new();
        let mut width = 0usize;

        for raw in content.lines() {
            let raw_line = raw.trim_end_matches('\r').to_string();
            let cleaned = strip_color_tokens(&raw_line);
            width = width.max(cleaned.chars().count());
            let leaked: &'static str = Box::leak(raw_line.into_boxed_str());
            lines.push(leaked);
        }

        if !lines.is_empty() {
            return Some(AsciiArt {
                lines,
                colors: palette_for_distro(id),
                width,
            });
        }
    }

    None
}

fn list_from_ascii_dir() -> Option<Vec<&'static str>> {
    let dir = ascii_dir()?;
    let mut names = Vec::new();

    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("txt") {
            continue;
        }

        let file_name = path.file_stem()?.to_string_lossy().to_string();
        if file_name.ends_with("_small") {
            continue;
        }

        let leaked: &'static str = Box::leak(file_name.into_boxed_str());
        names.push(leaked);
    }

    if names.is_empty() {
        return None;
    }

    names.sort_unstable();
    Some(names)
}

pub(crate) fn strip_color_tokens(line: &str) -> String {
    let mut output = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next) = chars.peek() {
                if next.is_ascii_digit() || *next == 'R' || *next == 'r' {
                    chars.next();
                    continue;
                }
            }
        }

        output.push(ch);
    }

    output
}

pub fn palette_for_distro(id: &str) -> Vec<Color> {
    match id {
        "gentoo" => vec![
            Color::Magenta,
            Color::White,
            Color::Magenta,
            Color::White,
            Color::Magenta,
            Color::White,
        ],
        "macos" | "macosx" | "osx" | "darwin" => vec![
            Color::Yellow,
            Color::White,
            Color::Yellow,
            Color::White,
            Color::Yellow,
            Color::White,
        ],
        "arch" | "archlinux" => vec![
            Color::Cyan,
            Color::Blue,
            Color::Cyan,
            Color::Blue,
            Color::Cyan,
            Color::Blue,
        ],
        "debian" => vec![Color::Red, Color::White, Color::Red, Color::White, Color::Red, Color::White],
        "ubuntu" => vec![Color::Red, Color::White, Color::Red, Color::White, Color::Red, Color::White],
        "fedora" => vec![Color::Blue, Color::White, Color::Blue, Color::White, Color::Blue, Color::White],
        "nixos" => vec![Color::Cyan, Color::Blue, Color::Cyan, Color::Blue, Color::Cyan, Color::Blue],
        "alpine" => vec![Color::Blue, Color::White, Color::Blue, Color::White, Color::Blue, Color::White],
        "manjaro" => vec![Color::Green, Color::White, Color::Green, Color::White, Color::Green, Color::White],
        "endeavouros" => vec![Color::Magenta, Color::Cyan, Color::Magenta, Color::Cyan, Color::Magenta, Color::Cyan],
        "pop" | "pop_os" | "pop!_os" => vec![Color::Cyan, Color::White, Color::Cyan, Color::White, Color::Cyan, Color::White],
        "mint" | "linuxmint" => vec![Color::Green, Color::White, Color::Green, Color::White, Color::Green, Color::White],
        "elementary" | "elementaryos" => vec![Color::White, Color::Cyan, Color::White, Color::Cyan, Color::White, Color::Cyan],
        "zorin" | "zorinos" => vec![Color::Blue, Color::White, Color::Blue, Color::White, Color::Blue, Color::White],
        "kali" => vec![Color::Blue, Color::White, Color::Blue, Color::White, Color::Blue, Color::White],
        "parrot" | "parrotos" => vec![Color::Green, Color::Cyan, Color::Green, Color::Cyan, Color::Green, Color::Cyan],
        "slackware" => vec![Color::Blue, Color::White, Color::Blue, Color::White, Color::Blue, Color::White],
        "void" | "voidlinux" => vec![Color::Green, Color::White, Color::Green, Color::White, Color::Green, Color::White],
        _ => vec![Color::Cyan, Color::White, Color::Cyan, Color::White, Color::Cyan, Color::White],
    }
}

fn render_with_palette(line: &str, palette: &[Color], use_colors: bool, fallback: Color) -> String {
    if !use_colors {
        return strip_color_tokens(line);
    }

    let mut out = String::new();
    let mut current = palette.first().copied().unwrap_or(fallback);
    let mut buffer = String::new();
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next) = chars.peek().copied() {
                if next.is_ascii_digit() {
                    let idx = next.to_digit(10).unwrap_or(1) as usize;
                    chars.next();
                    if !buffer.is_empty() {
                        out.push_str(&buffer.color(current).to_string());
                        buffer.clear();
                    }
                    if idx == 0 {
                        current = fallback;
                    } else {
                        current = palette.get(idx - 1).copied().unwrap_or(fallback);
                    }
                    continue;
                }
                if next == 'R' || next == 'r' {
                    chars.next();
                    if !buffer.is_empty() {
                        out.push_str(&buffer.color(current).to_string());
                        buffer.clear();
                    }
                    current = fallback;
                    continue;
                }
            }
        }

        buffer.push(ch);
    }

    if !buffer.is_empty() {
        out.push_str(&buffer.color(current).to_string());
    }

    out
}
