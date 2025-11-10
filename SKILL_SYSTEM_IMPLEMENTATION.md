# Skill-System Implementation TODO

## ✅ Bereits erledigt:

1. **shared/src/lib.rs**
   - ✅ Specialization enum (8 Spezialisierungen)
   - ✅ SkillId enum (40 Skills)
   - ✅ SkillInfo struct + SkillEffect enum
   - ✅ CharacterClass::specializations()
   - ✅ Specialization::from_class_and_index()
   - ✅ Specialization::name(), description(), skills()
   - ✅ SkillId::info()
   - ✅ CharacterData.specialization
   - ✅ CharacterSummary.specialization
   - ✅ ClientMessage::ChooseSpecialization
   - ✅ ServerMessage::SpecializationChosen/Failed
   - ✅ ServerMessage::CharacterSelected.specialization

2. **server/migrations/003_add_specialization.sql**
   - ✅ ALTER TABLE characters ADD specialization
   - ✅ CREATE INDEX

## 📋 Noch zu implementieren:

### 1. Server-Seite

**server/src/db/characters.rs:**
- [ ] Character struct: Add `specialization: Option<String>`
- [ ] `create_character()`: Handle specialization (should be None initially)
- [ ] `to_character_data()`: Convert DB string to Specialization enum
- [ ] `update_specialization()`: New function to set specialization (Level >= 5 check)
- [ ] `get_user_characters()`: Include specialization in CharacterSummary

**server/src/main.rs:**
- [ ] `handle_client_message()`: Add `ClientMessage::ChooseSpecialization` handler
- [ ] `handle_choose_specialization()`: New function
  - Validate token
  - Check character level >= 5
  - Check specialization not already chosen
  - Check specialization matches class
  - Update DB
  - Send SpecializationChosen response
- [ ] `SelectCharacter`: Return specialization in CharacterSelected message
- [ ] `LevelUp`: Check if level == 5 -> Send notification to choose spec

**server/src/auth/handlers.rs:**
- [ ] `handle_login()`: Include specialization in CharacterSummary

### 2. Client-Seite

**client/src/ui/specialization_choice.rs:** (NEU erstellen)
- [ ] SpecializationChoicePlugin
- [ ] GameState::ChooseSpecialization (neuer State)
- [ ] UI zum Wählen der Spezialisierung (bei Level 5)
- [ ] Zeigt beide Optionen mit Beschreibung
- [ ] Schickt ClientMessage::ChooseSpecialization
- [ ] Transition zu InGame nach Wahl

**client/src/ui/game_ui.rs:**
- [ ] Skill-Bar UI (Slots 1-5)
- [ ] Zeigt Skills der gewählten Spezialisierung
- [ ] Grau = Noch nicht freigeschaltet (Level zu niedrig)
- [ ] Grün = Bereit zu nutzen
- [ ] Cooldown-Anzeige
- [ ] Hotkeys 1-5 zum Aktivieren

**client/src/networking.rs:**
- [ ] Event für SpecializationChosen/Failed
- [ ] Handler für beide Events

**client/src/main.rs:**
- [ ] GameState::ChooseSpecialization hinzufügen
- [ ] SpecializationChoicePlugin registrieren

**client/src/auth_state.rs:**
- [ ] AuthState.specialization: Option<Specialization>
- [ ] Getter/Setter

### 3. Skill-System (später)

**shared/src/lib.rs:**
- [ ] Alle 40 SkillInfo implementieren (aktuell nur ~10)
- [ ] Skill-Aktivierungs-Messages
- [ ] ClientMessage::UseSkill { skill_id, target, position }
- [ ] ServerMessage::SkillUsed, SkillFailed

**server/src/skills.rs:** (NEU)
- [ ] Skill-Execution-System
- [ ] Cooldown-Tracking
- [ ] Mana-Verbrauch
- [ ] Damage-Calculations
- [ ] Effect-Application (Stun, Buff, etc.)

**client/src/skills.rs:** (NEU)
- [ ] Skill-Cooldown-Tracking
- [ ] Visual Effects pro Skill
- [ ] Animation-Triggers
- [ ] Hotkey-Bindings

## 🎯 Nächste Schritte (Priorität)

### Phase 1: Basic Specialization Choice (JETZT)
1. DB Migration ausführen
2. Server: Specialization speichern/laden
3. Server: ChooseSpecialization Handler
4. Client: SpecializationChoicePlugin erstellen
5. Client: UI für Auswahl (Level 5)

### Phase 2: UI Integration
6. Skill-Bar in game_ui.rs
7. Skills anzeigen (grau/grün)
8. Cooldown-Anzeige

### Phase 3: Skill Execution (später)
9. Alle 40 SkillInfo implementieren
10. Skill-Activation Messages
11. Server-side Skill-System
12. Visual Effects

