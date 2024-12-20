

pub fn timestamp_to_seconds(time: String) -> Result<i64,String> {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 3 {
        return Err("Invalid time format".to_string())
    }

    let hours: i64 = parts[0].parse::<i64>().map_err(|_| "Invalid time format".to_string())?;
    let minutes: i64 = parts[1].parse::<i64>().map_err(|_| "Invalid time format".to_string())?;
    let seconds: i64 = parts[2].parse::<i64>().map_err(|_| "Invalid time format".to_string())?;

    Ok((hours * 3600) + (minutes * 60) + seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_time_format() {
        let time = "10:30:45".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Ok(37845));
    }

    #[test]
    fn test_midnight() {
        let time = "00:00:00".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_invalid_format_missing_seconds() {
        let time = "10:30".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Err("Invalid time format".to_string()));
    }

    #[test]
    fn test_invalid_format_non_numeric() {
        let time = "10:xx:45".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Err("Invalid time format".to_string()));
    }

    #[test]
    fn test_midday() {
        let time = "12:00:00".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Ok(43200));
    }

    #[test]
    fn test_invalid_format_extra_components() {
        let time = "10:30:45::00".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Err("Invalid time format".to_string()));
    }

    #[test]
    fn test_single_digit_components() {
        let time = "9:8:7".to_string();
        let result = timestamp_to_seconds(time);

        assert_eq!(result, Ok(32887));
    }
}
