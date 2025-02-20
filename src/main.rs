use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;

fn main() {
    clear_terminal();

    let username = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let hostname = get_hostname();
    let kernel = get_kernel_info();
    let os_name = get_os_name();
    let uptime = get_uptime();
    let os_age = get_os_age();
    let memory = get_memory_info();

    let vibrant_orange = "\x1b[38;2;255;184;108m";
    let lively_green = "\x1b[38;2;166;227;161m";
    let sky_blue = "\x1b[38;2;137;220;235m";
    let vibrant_purple = "\x1b[38;2;203;166;247m";
    let lively_pink = "\x1b[38;2;245;194;231m";
    let reset = "\x1b[0m";

    println!("\n    {}{}{}  ", vibrant_orange, os_name, reset);
    println!("  =======================================\n");
    println!("  {}  User        :{} {}@{}", lively_green, reset, username, hostname);
    println!("  {}  Kernel      :{} {}", sky_blue, reset, kernel);
    println!("  {}  Uptime      :{} {}", vibrant_purple, reset, uptime);
    println!("  {}  OS Age      :{} {}", lively_pink, reset, os_age);
    println!("  {}溜 Memory      :{} {}", lively_green, reset, memory);
    println!("\n  =======================================\n");
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
}

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "Unknown".to_string())
        .trim()
        .to_string()
}

fn get_kernel_info() -> String {
    Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .and_then(|out| {
            String::from_utf8(out.stdout)
                .ok()
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_os_name() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .replace("PRETTY_NAME=", "")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Unknown".to_string()
}

fn get_uptime() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/uptime") {
        if let Some(first_line) = contents.lines().next() {
            if let Ok(secs) = first_line.split_whitespace().next().unwrap_or("0").parse::<f64>() {
                return format_uptime(secs);
            }
        }
    }
    "Unknown".to_string()
}

fn format_uptime(seconds: f64) -> String {
    let days = (seconds / 86400.0).floor() as u64;
    let hours = ((seconds % 86400.0) / 3600.0).floor() as u64;
    let minutes = ((seconds % 3600.0) / 60.0).floor() as u64;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn get_os_age() -> String {
    let install_log_path = "/var/log/pacman.log";

    let first_line = fs::read_to_string(install_log_path)
        .ok()
        .and_then(|contents| contents.lines().next().map(|s| s.to_string()));

    if let Some(line) = first_line {
        if let (Some(start), Some(end)) = (line.find('['), line.find(']')) {
            let timestamp_str = &line[start + 1..end];
            if let Ok(install_datetime) =
                NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S")
            {
                let install_timestamp = install_datetime.and_utc().timestamp() as u64;
                let current_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let days = (current_timestamp - install_timestamp) / 86400;
                if days >= 365 {
                    return format!("{}y {}d", days / 365, days % 365);
                } else {
                    return format!("{}d", days);
                }
            }
        }
    }
    "Unknown".to_string()
}

fn get_memory_info() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/meminfo") {
        let mut total = 0;
        let mut available = 0;

        for line in contents.lines() {
            if line.starts_with("MemTotal:") {
                total = extract_kb_value(line);
            } else if line.starts_with("MemAvailable:") {
                available = extract_kb_value(line);
            }
            if total > 0 && available > 0 {
                break;
            }
        }

        if total > 0 {
            let used = total.saturating_sub(available);
            return format!("{:.2} GiB / {:.2} GiB", kb_to_gib(used), kb_to_gib(total));
        }
    }
    "Unknown".to_string()
}

fn extract_kb_value(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

fn kb_to_gib(kb: u64) -> f64 {
    kb as f64 / 1_048_576.0
}
