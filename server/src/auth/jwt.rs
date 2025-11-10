use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

// Secret key for JWT (in production, use environment variable)
const JWT_SECRET: &str = "your-secret-key-change-this-in-production";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: i64,  // Expiration time (Unix timestamp)
    pub iat: i64,  // Issued at (Unix timestamp)
}

/// Create a JWT token for a user
/// 
/// # Arguments
/// * `user_id` - The user's ID
/// * `username` - The user's username
/// * `duration_hours` - Token validity duration in hours (default: 24)
/// 
/// # Returns
/// * `Ok(String)` - The JWT token
/// * `Err(JwtError)` - If token creation fails
pub fn create_token(user_id: i64, username: &str, duration_hours: i64) -> Result<String, JwtError> {
    let now = Utc::now();
    let expiration = now + Duration::hours(duration_hours);
    
    let claims = Claims {
        user_id,
        username: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

/// Verify and decode a JWT token
/// 
/// # Arguments
/// * `token` - The JWT token to verify
/// 
/// # Returns
/// * `Ok(Claims)` - The token claims if valid
/// * `Err(JwtError)` - If token is invalid or expired
pub fn verify_token(token: &str) -> Result<Claims, JwtError> {
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let token = create_token(1, "testuser", 24).unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_verify_valid_token() {
        let token = create_token(1, "testuser", 24).unwrap();
        let claims = verify_token(&token).unwrap();
        
        assert_eq!(claims.user_id, 1);
        assert_eq!(claims.username, "testuser");
    }

    #[test]
    fn test_verify_invalid_token() {
        let result = verify_token("invalid.token.here");
        assert!(result.is_err());
    }
}
