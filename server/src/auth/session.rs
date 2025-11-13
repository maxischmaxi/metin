use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct SessionData {
    pub user_id: i64,
    pub username: String,
    pub character_id: Option<i64>,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl SessionData {
    pub fn new(user_id: i64, username: String, token: String, duration_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            username,
            character_id: None,
            token: token.clone(),
            created_at: now,
            expires_at: now + Duration::hours(duration_hours),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn set_character(&mut self, character_id: i64) {
        self.character_id = Some(character_id);
    }
}

pub struct SessionManager {
    sessions: HashMap<String, SessionData>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Add a new session
    pub fn add_session(&mut self, token: String, session: SessionData) {
        self.sessions.insert(token, session);
    }

    /// Check if a user is already logged in
    pub fn is_user_logged_in(&self, user_id: i64) -> bool {
        self.sessions.values().any(|s| s.user_id == user_id && !s.is_expired())
    }

    /// Remove all sessions for a specific user
    pub fn remove_user_sessions(&mut self, user_id: i64) -> usize {
        let before_count = self.sessions.len();
        self.sessions.retain(|_, session| session.user_id != user_id);
        before_count - self.sessions.len()
    }

    /// Get session by token
    pub fn get_session(&self, token: &str) -> Option<&SessionData> {
        self.sessions.get(token)
    }

    /// Get mutable session by token
    pub fn get_session_mut(&mut self, token: &str) -> Option<&mut SessionData> {
        self.sessions.get_mut(token)
    }

    /// Remove session by token
    pub fn remove_session(&mut self, token: &str) -> Option<SessionData> {
        self.sessions.remove(token)
    }

    /// Validate token and return session if valid
    pub fn validate_token(&self, token: &str) -> Option<&SessionData> {
        self.sessions.get(token).filter(|s| !s.is_expired())
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&mut self) -> usize {
        let before_count = self.sessions.len();
        self.sessions.retain(|_, session| !session.is_expired());
        before_count - self.sessions.len()
    }

    /// Get all active sessions
    pub fn active_sessions_count(&self) -> usize {
        self.sessions.values().filter(|s| !s.is_expired()).count()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = SessionData::new(1, "testuser".to_string(), "token123".to_string(), 24);
        
        assert_eq!(session.user_id, 1);
        assert_eq!(session.username, "testuser");
        assert_eq!(session.character_id, None);
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_manager() {
        let mut manager = SessionManager::new();
        let session = SessionData::new(1, "testuser".to_string(), "token123".to_string(), 24);
        
        manager.add_session("token123".to_string(), session);
        
        assert!(manager.get_session("token123").is_some());
        assert!(manager.validate_token("token123").is_some());
        assert_eq!(manager.active_sessions_count(), 1);
    }

    #[test]
    fn test_set_character() {
        let mut session = SessionData::new(1, "testuser".to_string(), "token123".to_string(), 24);
        
        assert_eq!(session.character_id, None);
        session.set_character(5);
        assert_eq!(session.character_id, Some(5));
    }

    #[test]
    fn test_is_user_logged_in() {
        let mut manager = SessionManager::new();
        
        assert!(!manager.is_user_logged_in(1));
        
        let session = SessionData::new(1, "testuser".to_string(), "token123".to_string(), 24);
        manager.add_session("token123".to_string(), session);
        
        assert!(manager.is_user_logged_in(1));
        assert!(!manager.is_user_logged_in(2));
    }

    #[test]
    fn test_remove_user_sessions() {
        let mut manager = SessionManager::new();
        
        // Add multiple sessions for same user
        let session1 = SessionData::new(1, "testuser".to_string(), "token1".to_string(), 24);
        let session2 = SessionData::new(1, "testuser".to_string(), "token2".to_string(), 24);
        let session3 = SessionData::new(2, "otheruser".to_string(), "token3".to_string(), 24);
        
        manager.add_session("token1".to_string(), session1);
        manager.add_session("token2".to_string(), session2);
        manager.add_session("token3".to_string(), session3);
        
        assert_eq!(manager.active_sessions_count(), 3);
        
        // Remove all sessions for user 1
        let removed = manager.remove_user_sessions(1);
        assert_eq!(removed, 2);
        assert_eq!(manager.active_sessions_count(), 1);
        assert!(!manager.is_user_logged_in(1));
        assert!(manager.is_user_logged_in(2));
    }

    #[test]
    fn test_duplicate_login_prevention() {
        let mut manager = SessionManager::new();
        
        // User 1 logs in
        let session1 = SessionData::new(1, "testuser".to_string(), "token1".to_string(), 24);
        manager.add_session("token1".to_string(), session1);
        
        // Check if user is logged in
        assert!(manager.is_user_logged_in(1));
        
        // Simulate duplicate login attempt - should fail in handler
        // This test just validates the is_user_logged_in function works correctly
        assert!(manager.is_user_logged_in(1));
    }
}
