# Skill-System Design - Spezialisierungen

## Übersicht

Ab Level 5 kann jeder Charakter eine Spezialisierung wählen.
Jede Spezialisierung bietet 5 einzigartige Skills.

---

## 🗡️ KRIEGER - Der unbezwingbare Kämpfer

### Spezialisierung 1: "Leibwächter" (PvM - Tank)
**Fokus:** Monster tanken, Gruppe schützen

1. **Schildwall** (Level 5)
   - Cooldown: 15s | Mana: 20
   - Reduziert eingehenden Schaden um 50% für 5 Sekunden
   - Visuell: Blauer Schild-Effekt um den Spieler

2. **Provokation** (Level 10)
   - Cooldown: 10s | Mana: 25
   - Zwingt alle Monster im Umkreis (10m) den Spieler anzugreifen
   - Visuell: Roter Kreis-Puls vom Spieler

3. **Erderschütterung** (Level 15)
   - Cooldown: 20s | Mana: 40
   - Schlägt auf den Boden, betäubt Monster im Umkreis (5m) für 2s
   - Schaden: 150% Angriffskraft
   - Visuell: Braune Schockwelle am Boden

4. **Eiserne Haut** (Level 25)
   - Cooldown: 30s | Mana: 50
   - Passive: +10% max HP | Aktiv: Immun gegen Crowd Control für 3s
   - Visuell: Metallischer Glanz auf Rüstung

5. **Letzte Bastion** (Level 40)
   - Cooldown: 60s | Mana: 80
   - Bei tödlichem Schaden: Überlebt mit 1 HP, +100% Verteidigung für 5s
   - Visuell: Goldener Wiederauferstehungs-Effekt

### Spezialisierung 2: "Gladiator" (PvP - Damage)
**Fokus:** Spieler bekämpfen, Burst-Schaden

1. **Wirbelsturm** (Level 5)
   - Cooldown: 12s | Mana: 30
   - Rotiert mit Schwert, trifft alle Feinde im Umkreis (3m)
   - Schaden: 120% Angriffskraft
   - Visuell: Rotierende Schwert-Schlieren

2. **Kriegsschrei** (Level 10)
   - Cooldown: 20s | Mana: 25
   - Reduziert Verteidigung aller Feinde im Umkreis (8m) um 30% für 6s
   - Visuell: Roter Schrei-Effekt mit Schallwellen

3. **Hinrichtung** (Level 15)
   - Cooldown: 15s | Mana: 45
   - Mächtiger Einzelschlag, +100% Schaden gegen Feinde unter 30% HP
   - Schaden: 200% Angriffskraft
   - Visuell: Schwert glüht rot, Abwärts-Schlag

4. **Raserei** (Level 25)
   - Cooldown: 25s | Mana: 40
   - +50% Angriffsgeschwindigkeit für 8 Sekunden
   - -20% Verteidigung während aktiv
   - Visuell: Roter Nebel um Spieler

5. **Tödlicher Stoß** (Level 40)
   - Cooldown: 30s | Mana: 70
   - Stürmt zum Ziel (bis 15m), betäubt für 1.5s, Schaden: 250%
   - Visuell: Blitz-Sprint mit roter Spur

---

## 🥷 NINJA - Der lautlose Schatten

### Spezialisierung 1: "Bogenschütze" (Fernkampf)
**Fokus:** Distanz-DPS, Kiting

1. **Präzisionsschuss** (Level 5)
   - Cooldown: 8s | Mana: 20
   - Zielgerichteter Pfeil, 150% Schaden, ignoriert 20% Verteidigung
   - Reichweite: 25m
   - Visuell: Gelb leuchtender Pfeil

2. **Pfeilhagel** (Level 10)
   - Cooldown: 15s | Mana: 40
   - Schießt 5 Pfeile in die Luft, fallen auf Zielgebiet (5m Radius)
   - Schaden: 80% pro Pfeil
   - Visuell: Pfeile regnen vom Himmel

3. **Giftpfeil** (Level 15)
   - Cooldown: 12s | Mana: 30
   - Pfeil vergiftet Ziel, 50 Schaden/s für 8 Sekunden
   - Visuell: Grüner Pfeil, grüner DoT-Effekt am Ziel

4. **Rückwärtssprung** (Level 25)
   - Cooldown: 10s | Mana: 25
   - Springt 8m rückwärts, hinterlässt verlangsamende Falle
   - Falle: Verlangsamt Feinde um 50% für 3s
   - Visuell: Rückwärts-Flip, blaue Falle am Boden

5. **Durchschlag** (Level 40)
   - Cooldown: 20s | Mana: 60
   - Pfeil durchdringt alle Feinde in Linie (30m)
   - Schaden: 200% zum ersten Ziel, -20% pro weiterem
   - Visuell: Leuchtender Pfeil mit Schweif

### Spezialisierung 2: "Attentäter" (Nahkampf)
**Fokus:** Burst-Schaden, Kritische Treffer

1. **Schattenschritt** (Level 5)
   - Cooldown: 8s | Mana: 25
   - Teleportiert hinter das Ziel (max 10m)
   - Nächster Angriff: +50% kritische Chance
   - Visuell: Schwarzer Rauch-Effekt

2. **Dolchwirbel** (Level 10)
   - Cooldown: 10s | Mana: 35
   - 6 schnelle Dolchstiche in 2 Sekunden
   - Schaden: 60% pro Treffer
   - Visuell: Schnelle Stich-Animationen

3. **Tödliche Gifte** (Level 15)
   - Cooldown: 20s | Mana: 30
   - Beschichtet Waffen mit Gift
   - +30% Schaden für 10 Sekunden
   - 20% Chance zu vergiften (30 DoT/s für 5s)
   - Visuell: Grün leuchtende Dolche

4. **Unsichtbarkeit** (Level 25)
   - Cooldown: 30s | Mana: 50
   - Wird unsichtbar für 5 Sekunden
   - Nächster Angriff aus Unsichtbarkeit: 300% Schaden
   - Visuell: Verblasst komplett (Outline für Verbündete)

5. **Gnadenstoß** (Level 40)
   - Cooldown: 25s | Mana: 70
   - Führt 3 mächtige Stiche aus
   - Schaden: 150% pro Stoß
   - Letzter Stoß: +200% gegen Ziele unter 40% HP
   - Visuell: Schwarze Dolche mit roten Blitzen

---

## 🔥 SURA - Der Magische Krieger

### Spezialisierung 1: "Dämonen-Jäger" (PvM)
**Fokus:** Monster-Damage, Lebensraub

1. **Flammenschlag** (Level 5)
   - Cooldown: 10s | Mana: 25
   - Schwert-Schlag mit Feuer-Effekt
   - Schaden: 140% + verbrennt für 30 DoT/s (4s)
   - Visuell: Orangene Flammen um Schwert

2. **Seelenraub** (Level 10)
   - Cooldown: 15s | Mana: 40
   - Entzieht Lebensenergie vom Ziel
   - Schaden: 100%, heilt für 50% des Schadens
   - Visuell: Violette Energie fließt vom Ziel zum Spieler

3. **Zauberklinge** (Level 15)
   - Cooldown: 12s | Mana: 35
   - Beschwört magische Klinge, die automatisch angreift (8s)
   - Schaden: 50% Angriffskraft alle 2s
   - Visuell: Schwebende blaue Klinge kreist um Spieler

4. **Dunkler Schutz** (Level 25)
   - Cooldown: 20s | Mana: 45
   - Absorbiert die nächsten 200 Schadenspunkte
   - Bei Ablauf: Reflektiert 50% des absorbierten Schadens
   - Visuell: Violette Schutzblase

5. **Dämonische Verwandlung** (Level 40)
   - Cooldown: 60s | Mana: 80
   - Verwandelt sich für 15s in Dämonen-Form
   - +40% Schaden, +20% Lebensraub, +30% Bewegungsgeschwindigkeit
   - Visuell: Rote Haut, glühende Augen, Hörner

### Spezialisierung 2: "Blutkrieger" (PvP)
**Fokus:** Spieler bekämpfen, Healing Reduction

1. **Blutgier** (Level 5)
   - Cooldown: 12s | Mana: 30
   - Schlägt zu, verursacht Blutung
   - Schaden: 130% + 40 Blutschaden/s für 6s
   - Reduziert Heilung am Ziel um 50%
   - Visuell: Rotes Schwert, Blutstropfen

2. **Seelenketten** (Level 10)
   - Cooldown: 18s | Mana: 40
   - Fesselt Feind mit magischen Ketten (3s Immobilisierung)
   - Schaden: 100%, zieht Gegner 5m näher
   - Visuell: Schwarze magische Ketten

3. **Vampirschlag** (Level 15)
   - Cooldown: 15s | Mana: 35
   - Mächtiger Schlag, heilt für 80% des verursachten Schadens
   - Schaden: 160%
   - Visuell: Schwert wird rot, Blut-Absorption

4. **Furchtaura** (Level 25)
   - Cooldown: 25s | Mana: 50
   - Alle Feinde im Umkreis (10m) erleiden -30% Angriff für 8s
   - Schaden: 80% beim Aktivieren
   - Visuell: Dunkle rote Aura pulsiert

5. **Seelenernte** (Level 40)
   - Cooldown: 30s | Mana: 70
   - Bei jedem Angriff (10s): Stapelt "Seelenenergie" (max 5)
   - Bei 5 Stacks: Nächster Angriff = 400% Schaden + AOE Explosion
   - Visuell: Violette Orbs kreisen, explodierende Seelen

---

## ⚡ SCHAMANE - Der Naturbeherrscher

### Spezialisierung 1: "Lebenshüter" (Support/Healer)
**Fokus:** Gruppe heilen, Buffs

1. **Heilende Welle** (Level 5)
   - Cooldown: 8s | Mana: 30
   - Heilt Verbündeten für 150 HP
   - Springt zu 2 weiteren Verbündeten (80% Heilung)
   - Visuell: Grüne Wasser-Welle springt zwischen Spielern

2. **Naturschild** (Level 10)
   - Cooldown: 15s | Mana: 40
   - Gibt Verbündetem Schild (200 HP) für 8s
   - Schild regeneriert 20 HP/s
   - Visuell: Grüne Blätter kreisen um Spieler

3. **Erneuerung** (Level 15)
   - Cooldown: 20s | Mana: 50
   - Heilt alle Verbündeten im Umkreis (15m) über Zeit
   - 40 HP/s für 8 Sekunden
   - Visuell: Grünes Licht regnet vom Himmel

4. **Segnung der Natur** (Level 25)
   - Cooldown: 30s | Mana: 60
   - Buff für alle Verbündeten (15m Radius)
   - +20% Angriff, +15% Verteidigung für 12s
   - Visuell: Goldene Blätter umkreisen Verbündete

5. **Wiedergeburt** (Level 40)
   - Cooldown: 120s | Mana: 100
   - Belebt gestorbenen Verbündeten mit 50% HP/Mana wieder
   - Alternativ: Verhindert nächsten Tod in 60s (Buff)
   - Visuell: Goldenes Licht, Phönix-Effekt

### Spezialisierung 2: "Sturmrufer" (PvP/Damage)
**Fokus:** Elemental-Schaden, Crowd Control

1. **Blitzschlag** (Level 5)
   - Cooldown: 6s | Mana: 25
   - Ruft Blitz auf Feind herab
   - Schaden: 150%, 20% Chance zu betäuben (1s)
   - Visuell: Blau-weißer Blitz vom Himmel

2. **Kettenblitz** (Level 10)
   - Cooldown: 12s | Mana: 40
   - Blitz springt zwischen bis zu 5 Feinden
   - Schaden: 120% zum ersten, -15% pro Sprung
   - Visuell: Verzweigte Blitze zwischen Feinden

3. **Tornado** (Level 15)
   - Cooldown: 18s | Mana: 50
   - Beschwört Tornado (10s)
   - Zieht Feinde an (8m Radius), 30 Schaden/s
   - Visuell: Wirbelnder Wind mit Blättern

4. **Erdspieße** (Level 25)
   - Cooldown: 15s | Mana: 45
   - Lässt Erdspieße aus Boden schießen (Linie 15m)
   - Schaden: 180%, wirft Feinde in die Luft (2s)
   - Visuell: Braune Felsspitzen brechen hervor

5. **Zorn der Elemente** (Level 40)
   - Cooldown: 45s | Mana: 80
   - Kanalisiert für 5s: Meteore fallen auf Gebiet (12m Radius)
   - 8 Meteore, je 150 Schaden
   - Visuell: Feurige Meteore, Explosionen, Rauch

