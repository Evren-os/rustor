use std::env;
use std::fs;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;

enum LogType {
    Pacman,
    Dpkg,
    Installer,
}

// CACHING CONSTANTS & TYPES
const CACHE_EXPIRY_SECS: u64 = 3600; // 1 hour TTL

struct CacheData {
    timestamp: u64,
    hostname: String,
    kernel:   String,
    os_name:  String,
    os_age:   String,
}

fn get_cache_file_path() -> Option<PathBuf> {
    let cache_base = if let Ok(x) = env::var("XDG_CACHE_HOME") {
        PathBuf::from(x)
    } else if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".cache")
    } else {
        return None;
    };
    let dir = cache_base.join("rustor");
    if fs::create_dir_all(&dir).is_err() {
        return None;
    }
    Some(dir.join("cache.txt"))
}

fn load_cache() -> Option<CacheData> {
    let path = get_cache_file_path()?;
    let s    = fs::read_to_string(&path).ok()?;
    let mut lines     = s.lines();
    let timestamp     = lines.next()?.parse::<u64>().ok()?;
    let now           = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    if now > timestamp + CACHE_EXPIRY_SECS {
        return None;
    }
    let hostname = lines.next()?.strip_prefix("hostname=").map(str::to_string)?;
    let kernel   = lines.next()?.strip_prefix("kernel=").map(str::to_string)?;
    let os_name  = lines.next()?.strip_prefix("os_name=").map(str::to_string)?;
    let os_age   = lines.next()?.strip_prefix("os_age=").map(str::to_string)?;
    Some(CacheData { timestamp, hostname, kernel, os_name, os_age })
}

fn save_cache(data: &CacheData) {
    if let Some(path) = get_cache_file_path() {
        if let Ok(mut file) = fs::File::create(path) {
            let _ = writeln!(file, "{}", data.timestamp);
            let _ = writeln!(file, "hostname={}", data.hostname);
            let _ = writeln!(file, "kernel={}", data.kernel);
            let _ = writeln!(file, "os_name={}", data.os_name);
            let _ = writeln!(file, "os_age={}", data.os_age);
        }
    }
}

fn main() {
    clear_terminal();

    // Try to pull in cached values
    let cache = load_cache();

    let username = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let hostname = if let Some(ref c) = cache { c.hostname.clone() } else { get_hostname() };
    let kernel   = if let Some(ref c) = cache { c.kernel.clone()   } else { get_kernel_info() };
    let os_name  = if let Some(ref c) = cache { c.os_name.clone()  } else { get_os_name() };
    let uptime   = get_uptime();
    let os_age   = if let Some(ref c) = cache { c.os_age.clone()   } else { get_os_age() };
    let memory   = get_memory_info();

    // If there was no valid cache, save it for next run
    if cache.is_none() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let new_cache = CacheData {
            timestamp: now,
            hostname:  hostname.clone(),
            kernel:    kernel.clone(),
            os_name:   os_name.clone(),
            os_age:    os_age.clone(),
        };
        save_cache(&new_cache);
    }

    let vibrant_orange = "\x1b[38;2;255;184;108m";
    let lively_green   = "\x1b[38;2;166;227;161m";
    let sky_blue        = "\x1b[38;2;137;220;235m";
    let vibrant_purple  = "\x1b[38;2;203;166;247m";
    let lively_pink     = "\x1b[38;2;245;194;231m";
    let reset           = "\x1b[0m";

    println!("\n     {}{}{}  ", vibrant_orange, os_name, reset);
    println!("   =======================================\n");
    println!("  {}   User        :{} {}@{}", lively_green, reset, username, hostname);
    println!("  {}   Kernel      :{} {}", sky_blue, reset, kernel);
    println!("  {}   Uptime      :{} {}", vibrant_purple, reset, uptime);
    println!("  {}󱦟   OS Age      :{} {}", lively_pink, reset, os_age);
    println!("  {}   Memory      :{} {}", lively_green, reset, memory);
    println!("\n   =======================================\n");
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
    let days    = (seconds / 86400.0).floor() as u64;
    let hours   = ((seconds % 86400.0) / 3600.0).floor() as u64;
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
    let log_candidates = [
        ("/var/log/pacman.log", LogType::Pacman),
        ("/var/log/dpkg.log",   LogType::Dpkg),
        ("/var/log/installer/install.log", LogType::Installer),
    ];
    for (path, log_type) in log_candidates.iter() {
        if Path::new(path).exists() {
            if let Ok(file) = fs::File::open(path) {
                let mut reader = std::io::BufReader::new(file);
                let mut line   = String::new();
                if reader.read_line(&mut line).is_ok() && !line.trim().is_empty() {
                    let maybe_dt = match log_type {
                        LogType::Pacman => {
                            if let (Some(s), Some(e)) = (line.find('['), line.find(']')) {
                                let ts = &line[s + 1..e];
                                NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S").ok()
                            } else {
                                None
                            }
                        }
                        LogType::Dpkg => {
                            if line.len() >= 19 {
                                let ts = &line[0..19];
                                NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S").ok()
                            } else {
                                None
                            }
                        }
                        LogType::Installer => {
                            let parts: Vec<&str> = line.split_whitespace().take(2).collect();
                            if parts.len() >= 2 {
                                NaiveDateTime::parse_from_str(&parts.join(" "), "%Y-%m-%d %H:%M:%S").ok()
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(dt) = maybe_dt {
                        return format_os_age_from_timestamp(dt);
                    }
                }
            }
        }
    }

    // Fallback to filesystem creation time
    if let Ok(meta) = fs::metadata("/") {
        if let Ok(created) = meta.created() {
            if let Ok(dur) = created.duration_since(UNIX_EPOCH) {
                return format_os_age_from_unix(dur.as_secs());
            }
        }
    }
    "Unknown".to_string()
}

fn format_os_age_from_timestamp(install_datetime: NaiveDateTime) -> String {
    let ts = install_datetime.timestamp() as u64;
    format_os_age_from_unix(ts)
}

fn format_os_age_from_unix(install_timestamp: u64) -> String {
    let current = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days    = (current - install_timestamp) / 86400;
    if days >= 365 {
        format!("{}y {}d", days / 365, days % 365)
    } else {
        format!("{}d", days)
    }
}

fn get_memory_info() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/meminfo") {
        let mut total     = 0;
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
