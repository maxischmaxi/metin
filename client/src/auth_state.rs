use bevy::prelude::*;
use shared::{CharacterSummary, Specialization, CharacterClass};

#[derive(Resource, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
    pub characters: Vec<CharacterSummary>,
    pub selected_character_id: Option<i64>,
    pub class: Option<CharacterClass>,
    pub specialization: Option<Specialization>,
}

/// Stores the spawn position received from server when character is selected
#[derive(Resource, Default)]
pub struct SpawnPosition(pub Vec3);

impl AuthState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn login(&mut self, token: String, username: String, characters: Vec<CharacterSummary>) {
        self.token = Some(token);
        self.username = Some(username);
        self.characters = characters;
    }

    pub fn logout(&mut self) {
        self.token = None;
        self.username = None;
        self.characters.clear();
        self.selected_character_id = None;
        self.class = None;
        self.specialization = None;
    }

    pub fn select_character(&mut self, character_id: i64) {
        self.selected_character_id = Some(character_id);
    }

    pub fn get_selected_character(&self) -> Option<&CharacterSummary> {
        let character_id = self.selected_character_id?;
        self.characters.iter().find(|c| c.id == character_id)
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
