//! ISO 8601 duration parsing

/// Parse ISO 8601 duration string (PT30M, PT1H15M) to minutes
pub fn parse_iso8601_duration(duration: &str) -> i64 {
    // Handle empty or invalid strings
    if !duration.starts_with("PT") && !duration.starts_with("P") {
        return 0;
    }

    let mut minutes: i64 = 0;
    let mut current_num = String::new();

    // Skip the P prefix
    let chars: Vec<char> = duration.chars().collect();
    let mut i = 1; // Skip 'P'

    // Skip 'T' if present (for time component)
    if i < chars.len() && chars[i] == 'T' {
        i += 1;
    }

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            let num: i64 = current_num.parse().unwrap_or(0);
            current_num.clear();

            match c {
                'H' => minutes += num * 60,
                'M' => minutes += num,
                'S' => {} // Ignore seconds
                _ => {}
            }
        }
        i += 1;
    }

    minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minutes_only() {
        assert_eq!(parse_iso8601_duration("PT30M"), 30);
        assert_eq!(parse_iso8601_duration("PT5M"), 5);
        assert_eq!(parse_iso8601_duration("PT120M"), 120);
    }

    #[test]
    fn test_parse_hours_only() {
        assert_eq!(parse_iso8601_duration("PT1H"), 60);
        assert_eq!(parse_iso8601_duration("PT2H"), 120);
    }

    #[test]
    fn test_parse_hours_and_minutes() {
        assert_eq!(parse_iso8601_duration("PT1H30M"), 90);
        assert_eq!(parse_iso8601_duration("PT2H15M"), 135);
        assert_eq!(parse_iso8601_duration("PT1H45M"), 105);
    }

    #[test]
    fn test_parse_with_seconds() {
        // Seconds are ignored
        assert_eq!(parse_iso8601_duration("PT30M30S"), 30);
        assert_eq!(parse_iso8601_duration("PT1H30M45S"), 90);
    }

    #[test]
    fn test_invalid_duration() {
        assert_eq!(parse_iso8601_duration(""), 0);
        assert_eq!(parse_iso8601_duration("invalid"), 0);
        assert_eq!(parse_iso8601_duration("30"), 0);
    }
}
