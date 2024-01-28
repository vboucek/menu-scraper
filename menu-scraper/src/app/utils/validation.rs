use anyhow::anyhow;
use regex::Regex;

/// Trait for forms to check correctness of the input
pub trait Validation {
    fn validate(&self) -> Result<(), anyhow::Error>;

    /// Checks if input string is a valid email
    fn is_valid_email(&self, email: &str) -> Result<(), anyhow::Error> {
        if let Ok(regex) = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
            if regex.is_match(email) {
                return Ok(());
            }
        }
        Err(anyhow!("Email nemá správný formát."))
    }
}
