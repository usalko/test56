/// Exercise 1: Print name, date of birth, and mobile number
pub fn print_personal_info() -> String {
    format!(
        "Name : Alex Johnson\nDOB : July 14, 1985\nMobile : 999-999-9999"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_personal_info() {
        let result = print_personal_info();
        assert!(result.contains("Alex Johnson"));
        assert!(result.contains("July 14, 1985"));
        assert!(result.contains("999-999-9999"));
    }
}
