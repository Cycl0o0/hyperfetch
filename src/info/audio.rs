use crate::info::SystemInfo;
use std::process::Command;

pub fn gather(info: &mut SystemInfo) {
    gather_audio_device(info);
    gather_volume(info);
}

fn gather_audio_device(info: &mut SystemInfo) {
    // Try PipeWire first
    if let Ok(output) = Command::new("wpctl").args(["status"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            // Look for the default sink
            let mut in_sinks = false;
            for line in content.lines() {
                if line.contains("Sinks:") {
                    in_sinks = true;
                    continue;
                }
                if in_sinks {
                    if line.trim().starts_with('*') {
                        // This is the default sink
                        // Format: " *  123. Device Name [vol: 1.00]"
                        if let Some(name) = line.split('.').nth(1) {
                            let name = name.split('[').next().unwrap_or(name).trim();
                            info.audio_device = Some(format!("{} (PipeWire)", name));
                            return;
                        }
                    }
                    if !line.starts_with(' ') && !line.is_empty() {
                        break;
                    }
                }
            }
        }
    }

    // Try PulseAudio
    if let Ok(output) = Command::new("pactl")
        .args(["get-default-sink"])
        .output()
    {
        if output.status.success() {
            let sink_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !sink_name.is_empty() {
                // Get more info about the sink
                if let Ok(info_output) = Command::new("pactl")
                    .args(["list", "sinks"])
                    .output()
                {
                    if info_output.status.success() {
                        let content = String::from_utf8_lossy(&info_output.stdout);
                        let mut in_default = false;

                        for line in content.lines() {
                            if line.contains(&sink_name) {
                                in_default = true;
                            }
                            if in_default && line.trim().starts_with("Description:") {
                                let desc = line.split(':').nth(1).unwrap_or("").trim();
                                info.audio_device = Some(format!("{} (PulseAudio)", desc));
                                return;
                            }
                        }
                    }
                }

                info.audio_device = Some(format!("{} (PulseAudio)", sink_name));
                return;
            }
        }
    }

    // Try ALSA
    if let Ok(output) = Command::new("aplay").args(["-l"]).output() {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.starts_with("card") {
                    // Format: "card 0: Device [Description], device 0: ..."
                    if let Some(start) = line.find('[') {
                        if let Some(end) = line.find(']') {
                            let name = &line[start + 1..end];
                            info.audio_device = Some(format!("{} (ALSA)", name));
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn gather_volume(info: &mut SystemInfo) {
    // Try PipeWire/WirePlumber first
    if let Ok(output) = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            // Format: "Volume: 0.74" or "Volume: 0.74 [MUTED]"
            if let Some(vol_str) = content.split(':').nth(1) {
                let parts: Vec<&str> = vol_str.split_whitespace().collect();
                if let Some(vol) = parts.first() {
                    if let Ok(vol_float) = vol.parse::<f32>() {
                        let percent = (vol_float * 100.0) as u32;
                        let muted = content.contains("[MUTED]");
                        if muted {
                            info.volume = Some(format!("{}% (Muted)", percent));
                        } else {
                            info.volume = Some(format!("{}%", percent));
                        }
                        return;
                    }
                }
            }
        }
    }

    // Try PulseAudio
    if let Ok(output) = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
    {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            // Format: "Volume: front-left: 48000 /  73% / -8.11 dB, ..."
            for part in content.split('/') {
                let trimmed = part.trim();
                if trimmed.ends_with('%') {
                    let volume = trimmed.trim_end_matches('%').trim();
                    if let Ok(vol) = volume.parse::<u32>() {
                        // Check if muted
                        let muted = Command::new("pactl")
                            .args(["get-sink-mute", "@DEFAULT_SINK@"])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).contains("yes"))
                            .unwrap_or(false);

                        if muted {
                            info.volume = Some(format!("{}% (Muted)", vol));
                        } else {
                            info.volume = Some(format!("{}%", vol));
                        }
                        return;
                    }
                }
            }
        }
    }

    // Try ALSA with amixer
    if let Ok(output) = Command::new("amixer")
        .args(["get", "Master"])
        .output()
    {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains('[') && line.contains('%') {
                    // Format: "  Playback 65536 [100%] [on]"
                    if let Some(start) = line.find('[') {
                        if let Some(end) = line[start..].find('%') {
                            let vol_str = &line[start + 1..start + end];
                            if let Ok(vol) = vol_str.parse::<u32>() {
                                let muted = line.contains("[off]");
                                if muted {
                                    info.volume = Some(format!("{}% (Muted)", vol));
                                } else {
                                    info.volume = Some(format!("{}%", vol));
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
