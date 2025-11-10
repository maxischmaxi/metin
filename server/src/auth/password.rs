use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};

/// Hash a password using bcrypt
/// 
/// # Arguments
/// * `password` - The plaintext password to hash
/// 
/// # Returns
/// * `Ok(String)` - The hashed password
/// * `Err(BcryptError)` - If hashing fails
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verify a password against a hash
/// 
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The hash to verify against
/// 
/// # Returns
/// * `Ok(true)` - If password matches
/// * `Ok(false)` - If password doesn't match
/// * `Err(BcryptError)` - If verification fails
pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        
        assert_ne!(password, hash);
        assert!(hash.starts_with("$2"));
    }

    #[test]
    fn test_verify_password_success() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        
        let result = verify_password(password, &hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "test_password_123";
        let wrong_password = "wrong_password";
        let hash = hash_password(password).unwrap();
        
        let result = verify_password(wrong_password, &hash).unwrap();
        assert!(!result);
    }
}
