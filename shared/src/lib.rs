use serde::{Deserialize, Serialize};
use bevy::prelude::*;

// Re-export bevy for server use
pub use bevy;

// Network configuration
pub const PROTOCOL_ID: u64 = 1000;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";

// Character data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub name: String,
    pub class: CharacterClass,
    pub appearance: CharacterAppearance,
    pub level: i32,
    pub experience: i64,
    pub specialization: Option<Specialization>,  // Unlocked at level 5
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum CharacterClass {
    #[default]
    Krieger,
    Ninja,
    Sura,
    Schamane,
}

impl CharacterClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            CharacterClass::Krieger => "Krieger",
            CharacterClass::Ninja => "Ninja",
            CharacterClass::Sura => "Sura",
            CharacterClass::Schamane => "Schamane",
        }
    }
    
    pub fn specializations(&self) -> (&'static str, &'static str) {
        match self {
            CharacterClass::Krieger => ("Leibwächter", "Gladiator"),
            CharacterClass::Ninja => ("Bogenschütze", "Attentäter"),
            CharacterClass::Sura => ("Dämonen-Jäger", "Blutkrieger"),
            CharacterClass::Schamane => ("Lebenshüter", "Sturmrufer"),
        }
    }
}

// Specialization system (unlocked at level 5)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Specialization {
    // Krieger
    Leibwaechter,
    Gladiator,
    // Ninja
    Bogenschuetze,
    Attentaeter,
    // Sura
    DaemonenJaeger,
    Blutkrieger,
    // Schamane
    Lebenshueter,
    Sturmrufer,
}

impl Specialization {
    pub fn from_class_and_index(class: CharacterClass, index: u8) -> Option<Self> {
        match (class, index) {
            (CharacterClass::Krieger, 0) => Some(Specialization::Leibwaechter),
            (CharacterClass::Krieger, 1) => Some(Specialization::Gladiator),
            (CharacterClass::Ninja, 0) => Some(Specialization::Bogenschuetze),
            (CharacterClass::Ninja, 1) => Some(Specialization::Attentaeter),
            (CharacterClass::Sura, 0) => Some(Specialization::DaemonenJaeger),
            (CharacterClass::Sura, 1) => Some(Specialization::Blutkrieger),
            (CharacterClass::Schamane, 0) => Some(Specialization::Lebenshueter),
            (CharacterClass::Schamane, 1) => Some(Specialization::Sturmrufer),
            _ => None,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Specialization::Leibwaechter => "Leibwächter",
            Specialization::Gladiator => "Gladiator",
            Specialization::Bogenschuetze => "Bogenschütze",
            Specialization::Attentaeter => "Attentäter",
            Specialization::DaemonenJaeger => "Dämonen-Jäger",
            Specialization::Blutkrieger => "Blutkrieger",
            Specialization::Lebenshueter => "Lebenshüter",
            Specialization::Sturmrufer => "Sturmrufer",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Specialization::Leibwaechter => "PvM Tank - Monster tanken, Gruppe schützen",
            Specialization::Gladiator => "PvP Damage - Burst-Schaden gegen Spieler",
            Specialization::Bogenschuetze => "Fernkampf - Distanz-DPS, Kiting",
            Specialization::Attentaeter => "Nahkampf - Burst, kritische Treffer",
            Specialization::DaemonenJaeger => "PvM - Monster-Damage, Lebensraub",
            Specialization::Blutkrieger => "PvP - Healing Reduction, Damage",
            Specialization::Lebenshueter => "Support - Gruppe heilen, Buffs",
            Specialization::Sturmrufer => "PvP Damage - Elemental-Schaden, CC",
        }
    }
    
    pub fn skills(&self) -> Vec<SkillId> {
        match self {
            Specialization::Leibwaechter => vec![
                SkillId::Schildwall, SkillId::Provokation, SkillId::Erderschuetterung,
                SkillId::EiserneHaut, SkillId::LetzteBastion,
            ],
            Specialization::Gladiator => vec![
                SkillId::Wirbelsturm, SkillId::Kriegsschrei, SkillId::Hinrichtung,
                SkillId::Raserei, SkillId::ToedlicherStoss,
            ],
            Specialization::Bogenschuetze => vec![
                SkillId::Praezisionsschuss, SkillId::Pfeilhagel, SkillId::Giftpfeil,
                SkillId::Rueckwaertssprung, SkillId::Durchschlag,
            ],
            Specialization::Attentaeter => vec![
                SkillId::Schattenschritt, SkillId::Dolchwirbel, SkillId::ToedlicheGifte,
                SkillId::Unsichtbarkeit, SkillId::Gnadenstoss,
            ],
            Specialization::DaemonenJaeger => vec![
                SkillId::Flammenschlag, SkillId::Seelenraub, SkillId::Zauberklinge,
                SkillId::DunklerSchutz, SkillId::DaemonischeVerwandlung,
            ],
            Specialization::Blutkrieger => vec![
                SkillId::Blutgier, SkillId::Seelenketten, SkillId::Vampirschlag,
                SkillId::Furchtaura, SkillId::Seelenernte,
            ],
            Specialization::Lebenshueter => vec![
                SkillId::HeilendeWelle, SkillId::Naturschild, SkillId::Erneuerung,
                SkillId::SegnungDerNatur, SkillId::Wiedergeburt,
            ],
            Specialization::Sturmrufer => vec![
                SkillId::Blitzschlag, SkillId::Kettenblitz, SkillId::Tornado,
                SkillId::Erdspiesse, SkillId::ZornDerElemente,
            ],
        }
    }
    
    /// Convert specialization to database string
    pub fn as_str(&self) -> &'static str {
        match self {
            Specialization::Leibwaechter => "Leibwaechter",
            Specialization::Gladiator => "Gladiator",
            Specialization::Bogenschuetze => "Bogenschuetze",
            Specialization::Attentaeter => "Attentaeter",
            Specialization::DaemonenJaeger => "DaemonenJaeger",
            Specialization::Blutkrieger => "Blutkrieger",
            Specialization::Lebenshueter => "Lebenshueter",
            Specialization::Sturmrufer => "Sturmrufer",
        }
    }
    
    /// Parse specialization from database string
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Leibwaechter" => Some(Specialization::Leibwaechter),
            "Gladiator" => Some(Specialization::Gladiator),
            "Bogenschuetze" => Some(Specialization::Bogenschuetze),
            "Attentaeter" => Some(Specialization::Attentaeter),
            "DaemonenJaeger" => Some(Specialization::DaemonenJaeger),
            "Blutkrieger" => Some(Specialization::Blutkrieger),
            "Lebenshueter" => Some(Specialization::Lebenshueter),
            "Sturmrufer" => Some(Specialization::Sturmrufer),
            _ => None,
        }
    }
    
    /// Check if this specialization is valid for the given class
    pub fn is_valid_for_class(&self, class: CharacterClass) -> bool {
        match (class, self) {
            (CharacterClass::Krieger, Specialization::Leibwaechter) => true,
            (CharacterClass::Krieger, Specialization::Gladiator) => true,
            (CharacterClass::Ninja, Specialization::Bogenschuetze) => true,
            (CharacterClass::Ninja, Specialization::Attentaeter) => true,
            (CharacterClass::Sura, Specialization::DaemonenJaeger) => true,
            (CharacterClass::Sura, Specialization::Blutkrieger) => true,
            (CharacterClass::Schamane, Specialization::Lebenshueter) => true,
            (CharacterClass::Schamane, Specialization::Sturmrufer) => true,
            _ => false,
        }
    }
}

// Skill definitions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillId {
    // Leibwächter
    Schildwall, Provokation, Erderschuetterung, EiserneHaut, LetzteBastion,
    // Gladiator
    Wirbelsturm, Kriegsschrei, Hinrichtung, Raserei, ToedlicherStoss,
    // Bogenschütze
    Praezisionsschuss, Pfeilhagel, Giftpfeil, Rueckwaertssprung, Durchschlag,
    // Attentäter
    Schattenschritt, Dolchwirbel, ToedlicheGifte, Unsichtbarkeit, Gnadenstoss,
    // Dämonen-Jäger
    Flammenschlag, Seelenraub, Zauberklinge, DunklerSchutz, DaemonischeVerwandlung,
    // Blutkrieger
    Blutgier, Seelenketten, Vampirschlag, Furchtaura, Seelenernte,
    // Lebenshüter
    HeilendeWelle, Naturschild, Erneuerung, SegnungDerNatur, Wiedergeburt,
    // Sturmrufer
    Blitzschlag, Kettenblitz, Tornado, Erdspiesse, ZornDerElemente,
}

impl SkillId {
    pub fn info(&self) -> SkillInfo {
        match self {
            // Leibwächter Skills
            SkillId::Schildwall => SkillInfo {
                name: "Schildwall",
                description: "Reduziert eingehenden Schaden um 50% für 5 Sekunden",
                cooldown: 15.0,
                mana_cost: 20.0,
                required_level: 5,
                damage_multiplier: 0.0,
                effect: SkillEffect::DamageReduction(0.5, 5.0),
            },
            SkillId::Provokation => SkillInfo {
                name: "Provokation",
                description: "Zwingt alle Monster im Umkreis (10m) dich anzugreifen",
                cooldown: 10.0,
                mana_cost: 25.0,
                required_level: 10,
                damage_multiplier: 0.0,
                effect: SkillEffect::Taunt(10.0),
            },
            SkillId::Erderschuetterung => SkillInfo {
                name: "Erderschütterung",
                description: "Schlägt auf den Boden, betäubt Monster im Umkreis (5m) für 2s",
                cooldown: 20.0,
                mana_cost: 40.0,
                required_level: 15,
                damage_multiplier: 1.5,
                effect: SkillEffect::Stun(2.0, 5.0),
            },
            SkillId::EiserneHaut => SkillInfo {
                name: "Eiserne Haut",
                description: "Immun gegen Crowd Control für 3s",
                cooldown: 30.0,
                mana_cost: 50.0,
                required_level: 25,
                damage_multiplier: 0.0,
                effect: SkillEffect::CrowdControlImmunity(3.0),
            },
            SkillId::LetzteBastion => SkillInfo {
                name: "Letzte Bastion",
                description: "Bei tödlichem Schaden: Überlebt mit 1 HP, +100% Verteidigung für 5s",
                cooldown: 60.0,
                mana_cost: 80.0,
                required_level: 40,
                damage_multiplier: 0.0,
                effect: SkillEffect::Revive(1.0, 5.0),
            },
            
            // Gladiator Skills
            SkillId::Wirbelsturm => SkillInfo {
                name: "Wirbelsturm",
                description: "Rotiert mit Schwert, trifft alle Feinde im Umkreis (3m)",
                cooldown: 12.0,
                mana_cost: 30.0,
                required_level: 5,
                damage_multiplier: 1.2,
                effect: SkillEffect::AreaDamage(3.0),
            },
            SkillId::Kriegsschrei => SkillInfo {
                name: "Kriegsschrei",
                description: "Reduziert Verteidigung aller Feinde im Umkreis (8m) um 30% für 6s",
                cooldown: 20.0,
                mana_cost: 25.0,
                required_level: 10,
                damage_multiplier: 0.0,
                effect: SkillEffect::DefenseReduction(0.3, 6.0, 8.0),
            },
            SkillId::Hinrichtung => SkillInfo {
                name: "Hinrichtung",
                description: "Mächtiger Einzelschlag, +100% Schaden gegen Feinde unter 30% HP",
                cooldown: 15.0,
                mana_cost: 45.0,
                required_level: 15,
                damage_multiplier: 2.0,
                effect: SkillEffect::ExecuteDamage(0.3, 1.0),
            },
            SkillId::Raserei => SkillInfo {
                name: "Raserei",
                description: "+50% Angriffsgeschwindigkeit für 8 Sekunden",
                cooldown: 25.0,
                mana_cost: 40.0,
                required_level: 25,
                damage_multiplier: 0.0,
                effect: SkillEffect::AttackSpeedBuff(0.5, 8.0),
            },
            SkillId::ToedlicherStoss => SkillInfo {
                name: "Tödlicher Stoß",
                description: "Stürmt zum Ziel (bis 15m), betäubt für 1.5s",
                cooldown: 30.0,
                mana_cost: 70.0,
                required_level: 40,
                damage_multiplier: 2.5,
                effect: SkillEffect::DashStun(15.0, 1.5),
            },
            
            // Für jetzt erstmal nur die ersten zwei Specs implementieren
            // TODO: Restliche Skills implementieren
            _ => SkillInfo {
                name: "Unknown",
                description: "Not implemented yet",
                cooldown: 10.0,
                mana_cost: 30.0,
                required_level: 5,
                damage_multiplier: 1.0,
                effect: SkillEffect::None,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub cooldown: f32,      // Seconds
    pub mana_cost: f32,
    pub required_level: i32,
    pub damage_multiplier: f32,
    pub effect: SkillEffect,
}

#[derive(Debug, Clone)]
pub enum SkillEffect {
    None,
    AreaDamage(f32),                          // radius
    DamageReduction(f32, f32),                // amount, duration
    Taunt(f32),                               // radius
    Stun(f32, f32),                           // duration, radius
    CrowdControlImmunity(f32),                // duration
    Revive(f32, f32),                         // health, defense_buff_duration
    DefenseReduction(f32, f32, f32),          // amount, duration, radius
    ExecuteDamage(f32, f32),                  // threshold, bonus_multiplier
    AttackSpeedBuff(f32, f32),                // amount, duration
    DashStun(f32, f32),                       // distance, stun_duration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    pub skin_color: [f32; 3],
    pub hair_color: [f32; 3],
}

impl Default for CharacterAppearance {
    fn default() -> Self {
        Self {
            skin_color: [1.0, 0.8, 0.6],
            hair_color: [0.3, 0.2, 0.1],
        }
    }
}

// Authentication messages
#[derive(Debug, Serialize, Deserialize)]
pub enum AuthMessage {
    Register { username: String, password: String, email: Option<String> },
    Login { username: String, password: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthResponse {
    RegisterSuccess,
    RegisterFailed { reason: String },
    LoginSuccess { token: String, characters: Vec<CharacterSummary> },
    LoginFailed { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSummary {
    pub id: i64,
    pub name: String,
    pub class: CharacterClass,
    pub level: i32,
    pub last_played: Option<String>,
    pub specialization: Option<Specialization>,
}

// Network messages
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    // Authentication
    Auth(AuthMessage),
    
    // Character Management
    CreateCharacter { token: String, character: CharacterData },
    SelectCharacter { token: String, character_id: i64 },
    DeleteCharacter { token: String, character_id: i64 },
    
    // Gameplay
    Join { character: CharacterData },
    Move { direction: Vec3 },
    UpdatePosition { position: Vec3 },  // Absolute position update
    GainExperience { amount: i64 },  // Dev command for testing
    
    // Specialization
    ChooseSpecialization { token: String, specialization: Specialization },
    
    Disconnect,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    // Authentication
    AuthResponse(AuthResponse),
    
    // Character Management
    CharacterCreated { character_id: i64 },
    CharacterCreationFailed { reason: String },
    CharacterSelected { 
        character_id: i64,
        character_name: String,
        character_class: CharacterClass,
        position: Vec3,
        level: i32,
        experience: i64,
        max_health: f32,
        max_mana: f32,
        max_stamina: f32,
        specialization: Option<Specialization>,
    },
    CharacterSelectionFailed { reason: String },
    CharacterDeleted { character_id: i64 },
    CharacterDeletionFailed { reason: String },
    
    // Gameplay
    PlayerJoined { id: u64, character: CharacterData, position: Vec3 },
    PlayerLeft { id: u64 },
    PlayerMoved { id: u64, position: Vec3 },
    WorldState { players: Vec<PlayerState> },
    
    // Leveling System
    ExperienceGained { amount: i64, new_total: i64, xp_needed: i64 },
    LevelUp { 
        new_level: i32, 
        new_max_health: f32, 
        new_max_mana: f32, 
        new_max_stamina: f32,
    },
    
    // Specialization
    SpecializationChosen { specialization: Specialization },
    SpecializationFailed { reason: String },
    
    // Time of Day System
    TimeUpdate { hour: f32 },  // 0.0 - 24.0 (12.0 = noon, 0.0 = midnight)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: u64,
    pub character: CharacterData,
    pub position: Vec3,
}

// Game settings
#[derive(Debug, Clone, Serialize, Deserialize, bevy::prelude::Resource)]
pub struct MMOSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
}

impl Default for MMOSettings {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: (u32, u32),
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            resolution: (1280, 720),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.8,
        }
    }
}

// Leveling System - XP Calculations
// Uses exponential formula: XP = 100 * level^2.8
// This creates a grindy progression similar to major MMORPGs
pub fn calculate_xp_for_level(level: i32) -> i64 {
    if level <= 1 {
        return 100; // Level 1->2 needs 100 XP
    }
    (100.0 * (level as f64).powf(2.8)) as i64
}

// Calculate max stats based on level and class
// Base stats at level 1: HP=100, Mana=100, Stamina=100
// Per level gains depend on class
pub fn calculate_stats_for_level(level: i32, class: &CharacterClass) -> (f32, f32, f32) {
    let base_hp = 100.0;
    let base_mana = 100.0;
    let base_stamina = 100.0;
    
    let levels_gained = (level - 1) as f32;
    
    // Class-specific stat multipliers
    let (hp_per_level, mana_per_level, stamina_per_level) = match class {
        CharacterClass::Krieger => (20.0, 5.0, 12.0),   // Tanky warrior, low mana
        CharacterClass::Ninja => (12.0, 8.0, 15.0),     // Agile assassin, high stamina
        CharacterClass::Sura => (15.0, 12.0, 10.0),     // Balanced magic warrior
        CharacterClass::Schamane => (8.0, 18.0, 8.0),   // Shaman healer, high mana
    };
    
    let max_health = base_hp + (levels_gained * hp_per_level);
    let max_mana = base_mana + (levels_gained * mana_per_level);
    let max_stamina = base_stamina + (levels_gained * stamina_per_level);
    
    (max_health, max_mana, max_stamina)
}
