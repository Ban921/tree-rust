use std::time::SystemTime;
use chrono::{DateTime, Local};

/// Format file size in human-readable format
pub fn format_size(size: u64, si: bool) -> String {
    let units = if si {
        ["B", "kB", "MB", "GB", "TB", "PB"]
    } else {
        ["B", "K", "M", "G", "T", "P"]
    };
    let base: f64 = if si { 1000.0 } else { 1024.0 };

    if size < base as u64 {
        return format!("{:>4}", size);
    }

    let mut size_f = size as f64;
    let mut unit_idx = 0;

    while size_f >= base && unit_idx < units.len() - 1 {
        size_f /= base;
        unit_idx += 1;
    }

    if size_f >= 10.0 {
        format!("{:>3.0}{}", size_f, units[unit_idx])
    } else {
        format!("{:>3.1}{}", size_f, units[unit_idx])
    }
}

/// Format timestamp for display
pub fn format_time(time: SystemTime, format: Option<&str>) -> String {
    let datetime: DateTime<Local> = time.into();
    let fmt = format.unwrap_or("%b %d %H:%M");
    datetime.format(fmt).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0, false).trim(), "0");
        assert_eq!(format_size(512, false).trim(), "512");
    }

    #[test]
    fn test_format_size_kilobytes() {
        assert_eq!(format_size(1024, false).trim(), "1.0K");
        assert_eq!(format_size(2048, false).trim(), "2.0K");
    }

    #[test]
    fn test_format_size_si() {
        assert_eq!(format_size(1000, true).trim(), "1.0kB");
        assert_eq!(format_size(1500, true).trim(), "1.5kB");
    }
}
