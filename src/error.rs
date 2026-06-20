use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(message: String) -> Self{
        Self{
            message
        }
    }

}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl error::Error for CustomError {}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn new_creates_error_with_message() {
        let err = CustomError::new("test error".to_string());
        assert_eq!(err.to_string(), "Error: test error");
    }

    #[test]
    fn display_formats_error_correctly() {
        let err = CustomError::new("something failed".to_string());
        assert_eq!(format!("{}", err), "Error: something failed");
    }

    #[test]
    fn error_trait_is_implemented() {
        fn takes_error(_: &dyn error::Error) {}
        let err = CustomError::new("implemented".to_string());
        takes_error(&err);
    }
}
