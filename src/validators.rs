use chrono::{Datelike, NaiveDate};

// Validate the YYYY-MM date format using chrono
pub fn validate_month_date_format(s: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
        Ok(date) if date.day() == 1 => Ok(s.to_string()),
        _ => Err(String::from("Date must be in the format YYYY-MM")),
    }
}

// Validate the YYYY-MM-DD date format using chrono
pub fn validate_date_format(s: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(_) => Ok(s.to_string()),
        _ => Err(String::from("Date must be in the format YYYY-MM-DD")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_month_date_format() {
        assert_eq!(
            validate_month_date_format("2023-01"),
            Ok("2023-01".to_string())
        );
        assert_eq!(
            validate_month_date_format("2023"),
            Err("Date must be in the format YYYY-MM".to_string())
        );
    }

    #[test]
    fn test_validate_date_format() {
        assert_eq!(
            validate_date_format("2023-01-05"),
            Ok("2023-01-05".to_string())
        );
        assert_eq!(
            validate_date_format("2023-01"),
            Err("Date must be in the format YYYY-MM-DD".to_string())
        );
    }
}
