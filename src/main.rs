use std::process::Command;
use std::fs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let output = Command::new("uname")
        .arg("-r")
        .output();
    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

fn get_os_name() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(contents) => {
            for line in contents.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line
                        .replace("PRETTY_NAME=", "")
                        .trim_matches('"')
                        .to_string();
                }
            }
            "Unknown".to_string()
        }
        Err(_) => "Unknown".to_string(),
    }
}

fn get_uptime() -> String {
    match fs::read_to_string("/proc/uptime") {
        Ok(contents) => {
            if let Some(first_line) = contents.lines().next() {
                let secs: f64 = first_line.split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0.0);
                format_uptime(secs)
            } else {
                "Unknown".to_string()
            }
        }
        Err(_) => "Unknown".to_string(),
    }
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

    let install_date_cmd = Command::new("sed")
        .args(&["-n", "1p", install_log_path])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .unwrap_or_default();

    let install_date = install_date_cmd
        .split_whitespace()
        .nth(0)
        .map(|date| date.trim_matches('[').trim_matches(']'))
        .unwrap_or_default();

    let install_date_seconds = Command::new("date")
        .args(&["-d", install_date, "+%s"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|secs| secs.trim().parse::<u64>().ok())
        .unwrap_or(0);

    let current_date = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days = (current_date - install_date_seconds) / 86400;

    if days >= 365 {
        format!("{}y {}d", days / 365, days % 365)
    } else {
        format!("{}d", days)
    }
}

fn get_memory_info() -> String {
    match fs::read_to_string("/proc/meminfo") {
        Ok(contents) => {
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
                let used = total - available;
                format!("{:.2} GiB / {:.2} GiB", kb_to_gib(used), kb_to_gib(total))
            } else {
                "Unknown".to_string()
            }
        }
        Err(_) => "Unknown".to_string(),
    }
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
