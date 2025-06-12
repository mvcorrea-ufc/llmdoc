// llmdoc/src/utils.rs

use chrono::{DateTime, Utc};

/// Formats a DateTime<Utc> into a standard string format.
pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Parses a string into a DateTime<Utc>.
pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
}

/// Generates a new UUID v4 string.
pub fn new_uuid_v4_str() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn utils_init_message() {
    tracing::debug!("Utils module initialized (placeholder).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_parse_datetime() {
        let now = Utc::now();
        let formatted = format_datetime(now);
        let parsed = parse_datetime(&formatted).expect("Failed to parse formatted datetime");
        
        // Compare timestamps to avoid issues with nanosecond precision differences
        // if the original `now` had nanoseconds beyond seconds.
        assert_eq!(now.timestamp(), parsed.timestamp());
    }

    #[test]
    fn test_new_uuid_v4_str() {
        let uuid_str = new_uuid_v4_str();
        assert!(uuid::Uuid::parse_str(&uuid_str).is_ok(), "Generated string should be a valid UUID");
    }
}