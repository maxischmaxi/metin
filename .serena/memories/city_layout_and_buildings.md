# Stadt-Layout und Gebäude-Dokumentation

## Übersicht

Mittelalterliche Fantasy-Stadt mit zentralem Marktplatz und umliegenden Gebäuden. Die Stadt wurde in `client/src/player.rs` in der `setup_player` Funktion implementiert.

## Marktplatz (Plaza)

- **Größe**: 40x40m (von -20 bis +20 in X und Z Koordinaten)
- **Fläche**: 1.600m²
- **Spieler-Spawn**: (0, 1, 0) - Mitte des Platzes
- **NPC Position**: (5, 1, 5) - "Meister der Künste" am östlichen Platzrand
- **Zweck**: Haupttreffpunkt, Handelszentrum, Events

## Bodenfläche

- **Größe**: 150x150m Plane3d
- **Material**: Grün (0.3, 0.7, 0.3)
- **Typ**: Gras/Erde

## Spieler-Maßstab

- **Spieler-Höhe**: ~1.8m (realistische Körpergröße)
- **Kapsel-Größe**: 0.5 Radius, 1.5 Höhe
- Alle Gebäude sind im Verhältnis zum Spieler skaliert

## Gebäude-Liste (22 Strukturen)

### Nord-Seite (hinter dem Platz)

#### 1. Gasthaus/Taverne
- **Position**: (-8.0, 4.5, 28.0)
- **Größe**: 15m breit × 9m hoch × 12m tief
- **Rotation**: 0.15 Radians (leicht gedreht)
- **Farbe**: Braun/Holz (0.6, 0.4, 0.2)
- **Etagen**: 3 Etagen
- **Zweck**: Haupttreffpunkt, Unterkunft, Taverne
- **Besonderheit**: Größtes Gasthaus, zentrale Lage

#### 2. Waffenschmied
- **Position**: (-28.0, 3.5, 28.0)
- **Größe**: 10m × 7m × 9m
- **Rotation**: -0.2 Radians
- **Farbe**: Dunkelgrau (0.3, 0.3, 0.35)
- **Etagen**: 2 Etagen
- **Zweck**: Waffenherstellung, Handel

#### 3. Wohnhaus Nord 1 (Stadthaus)
- **Position**: (24.0, 5.0, 30.0)
- **Größe**: 7m × 10m × 7m
- **Rotation**: 0.3 Radians
- **Farbe**: Beige (0.8, 0.75, 0.6)
- **Etagen**: 3 Etagen (hoch und schmal)
- **Zweck**: Wohngebäude

#### 4. Wohnhaus Nord 2
- **Position**: (32.0, 3.5, 26.0)
- **Größe**: 8m × 7m × 8m
- **Rotation**: -0.4 Radians
- **Farbe**: Helles Beige (0.75, 0.7, 0.55)
- **Etagen**: 2 Etagen
- **Zweck**: Wohngebäude

#### 5. Wachturm
- **Position**: (35.0, 7.5, 35.0)
- **Größe**: 6m × 15m × 6m
- **Rotation**: Keine (0.0)
- **Farbe**: Grauer Stein (0.5, 0.5, 0.55)
- **Etagen**: ~5 Etagen (Verteidigungsturm)
- **Zweck**: Stadtverteidigung, Aussichtspunkt
- **Besonderheit**: Zweithöchstes Gebäude

### Ost-Seite (rechts vom Platz)

#### 6. Markthalle/Händler
- **Position**: (30.0, 2.5, 10.0)
- **Größe**: 10m × 5m × 18m
- **Rotation**: 0.1 Radians
- **Farbe**: Rot/Ziegel (0.7, 0.3, 0.2)
- **Etagen**: 1 Etage (hohe Decke)
- **Zweck**: Überdachter Markt, Händler-Halle
- **Besonderheit**: Lang und niedrig, Handelszentrum

#### 7. Wohnhaus Ost 1
- **Position**: (31.0, 4.0, -8.0)
- **Größe**: 9m × 8m × 8m
- **Rotation**: -0.25 Radians
- **Farbe**: Beige (0.8, 0.75, 0.6)
- **Etagen**: 2 Etagen
- **Zweck**: Wohngebäude

#### 8. Lagerhaus/Lagerschuppen
- **Position**: (27.0, 2.25, -20.0)
- **Größe**: 7m × 4.5m × 10m
- **Rotation**: 0.5 Radians
- **Farbe**: Dunkles Holz (0.5, 0.35, 0.2)
- **Etagen**: 1 Etage
- **Zweck**: Lagerung, Warehouse

### Süd-Seite (vor dem Platz)

#### 9. Schmiede (Blacksmith)
- **Position**: (2.0, 3.5, -30.0)
- **Größe**: 12m × 7m × 10m
- **Rotation**: -0.1 Radians
- **Farbe**: Dunkelgrau (0.25, 0.25, 0.3)
- **Etagen**: 1-2 Etagen (hohe Decken für Esse)
- **Zweck**: Schmiedewerk, Metallverarbeitung
- **Besonderheit**: Breites, massives Gebäude

#### 10. Alchemisten-Turm
- **Position**: (-20.0, 6.0, -28.0)
- **Größe**: 8m × 12m × 8m
- **Rotation**: 0.35 Radians
- **Farbe**: Lila/Mystisch (0.5, 0.3, 0.6)
- **Etagen**: 3-4 Etagen (Turm)
- **Zweck**: Alchemie, Tränke, Magie
- **Besonderheit**: Hoch und mystisch wirkend

#### 11. Wohnhaus Süd 1
- **Position**: (18.0, 3.5, -31.0)
- **Größe**: 8m × 7m × 9m
- **Rotation**: -0.3 Radians
- **Farbe**: Beige (0.8, 0.75, 0.6)
- **Etagen**: 2 Etagen
- **Zweck**: Wohngebäude

#### 12. Wohnhaus Süd 2 (Cottage)
- **Position**: (30.0, 3.0, -27.0)
- **Größe**: 7m × 6m × 7m
- **Rotation**: 0.2 Radians
- **Farbe**: Helles Beige (0.75, 0.7, 0.55)
- **Etagen**: 1.5 Etagen (Cottage-Stil)
- **Zweck**: Kleineres Wohngebäude

### West-Seite (links vom Platz)

#### 13. Kathedrale/Tempel
- **Position**: (-32.0, 8.0, 8.0)
- **Größe**: 14m × 16m × 20m
- **Rotation**: 0.08 Radians
- **Farbe**: Weiß/Hell (0.9, 0.9, 0.85)
- **Etagen**: Hohe Kathedrale mit Gewölbe
- **Volumen**: 5.600m³
- **Zweck**: Hauptkirche, religiöses Zentrum
- **Besonderheit**: HÖCHSTES GEBÄUDE der Stadt (16m), imposantes Wahrzeichen

#### 14. Bibliothek
- **Position**: (-30.0, 5.0, -16.0)
- **Größe**: 11m × 10m × 11m
- **Rotation**: -0.15 Radians
- **Farbe**: Dunkelbraun (0.4, 0.25, 0.15)
- **Etagen**: 3 Etagen
- **Volumen**: 1.210m³
- **Zweck**: Bücher, Wissen, Forschung

#### 15. Wohnhaus West 1
- **Position**: (-34.0, 4.0, -31.0)
- **Größe**: 8m × 8m × 8m
- **Rotation**: 0.45 Radians
- **Farbe**: Beige (0.8, 0.75, 0.6)
- **Etagen**: 2 Etagen
- **Zweck**: Wohngebäude

#### 16. Werkstatt (Workshop)
- **Position**: (-36.0, 3.0, 24.0)
- **Größe**: 10m × 6m × 8m
- **Rotation**: -0.3 Radians
- **Farbe**: Holz (0.55, 0.4, 0.25)
- **Etagen**: 1 Etage (hohe Decke)
- **Zweck**: Handwerk, Werkstatt

#### 17. Kleine Kapelle
- **Position**: (-38.0, 4.5, -36.0)
- **Größe**: 8m × 9m × 7m
- **Rotation**: 0.25 Radians
- **Farbe**: Heller Stein (0.85, 0.85, 0.8)
- **Etagen**: Hohe Nave
- **Zweck**: Kleine Kirche, Gebetshaus

### Platz-Dekorationen

#### 18. Zentral-Brunnen
- **Position**: (-6.0, 1.5, 6.0)
- **Typ**: Cylinder
- **Größe**: Radius 2.5m, Höhe 3m
- **Farbe**: Stein (0.6, 0.6, 0.65)
- **Zweck**: Wasserversorgung, Treffpunkt
- **Collider**: Cylinder (radius: 2.5, height: 3.0)

#### 19. Statue/Monument
- **Position**: (8.0, 2.5, -8.0)
- **Typ**: Cuboid
- **Größe**: 2m × 5m × 2m
- **Farbe**: Grauer Stein (0.5, 0.5, 0.5)
- **Zweck**: Denkmal, Dekoration
- **Collider**: Cylinder (radius: 1.0, height: 5.0)

#### 20. Marktstand 1
- **Position**: (22.0, 1.5, 4.0)
- **Größe**: 4m × 3m × 3.5m
- **Rotation**: 0.6 Radians
- **Farbe**: Holz (0.65, 0.5, 0.3)
- **Zweck**: Marktstand, Verkauf

#### 21. Marktstand 2
- **Position**: (20.0, 1.5, -3.0)
- **Größe**: 3.5m × 3m × 4m
- **Rotation**: -0.4 Radians
- **Farbe**: Holz (0.65, 0.5, 0.3)
- **Zweck**: Marktstand, Verkauf

#### 22. Marktstand 3
- **Position**: (18.5, 1.5, 10.0)
- **Größe**: 4m × 3m × 3m
- **Rotation**: 0.2 Radians
- **Farbe**: Holz (0.65, 0.5, 0.3)
- **Zweck**: Marktstand, Verkauf

## Gebäude-Kategorien

### Nach Größe (Höhe)
1. **Kathedrale**: 16m (höchstes Gebäude)
2. **Wachturm**: 15m
3. **Alchemisten-Turm**: 12m
4. **Bibliothek**: 10m
5. **Stadthaus Nord 1**: 10m
6. **Gasthaus**: 9m
7. **Kapelle**: 9m
8. **Standard-Wohnhäuser**: 7-8m
9. **Handwerks-Gebäude**: 6-7m
10. **Markthalle**: 5m (niedrig aber lang)
11. **Marktstände**: 3m

### Nach Funktion
- **Religiös**: Kathedrale, Kapelle
- **Wohnen**: 6 Wohnhäuser verschiedener Größen
- **Handwerk**: Waffenschmied, Schmiede, Workshop, Alchemist
- **Handel**: Markthalle, 3 Marktstände
- **Bildung**: Bibliothek
- **Unterkunft**: Gasthaus
- **Lagerung**: Lagerhaus
- **Verteidigung**: Wachturm
- **Dekoration**: Brunnen, Statue

## Farb-Palette

### Gebäude-Materialien
- **Braun/Holz**: (0.6, 0.4, 0.2) - Gasthaus, Marktstände
- **Dunkles Holz**: (0.5-0.55, 0.35-0.4, 0.2-0.25) - Lagerhaus, Workshop
- **Beige/Putz**: (0.8, 0.75, 0.6) - Wohnhäuser
- **Helles Beige**: (0.75, 0.7, 0.55) - Wohnhäuser
- **Dunkelgrau**: (0.25-0.3, 0.25-0.3, 0.3-0.35) - Schmieden
- **Grauer Stein**: (0.5, 0.5, 0.55) - Wachturm
- **Rot/Ziegel**: (0.7, 0.3, 0.2) - Markthalle
- **Lila/Mystisch**: (0.5, 0.3, 0.6) - Alchemist
- **Weiß/Hell**: (0.9, 0.9, 0.85) - Kathedrale
- **Heller Stein**: (0.85, 0.85, 0.8) - Kapelle
- **Dunkelbraun**: (0.4, 0.25, 0.15) - Bibliothek
- **Stein**: (0.5-0.6, 0.5-0.6, 0.5-0.65) - Brunnen, Statue

## Collision System

### Alle Gebäude verwenden:
```rust
AutoCollider {
    detail: CollisionDetail::Low,
    collision_type: CollisionType::Static,
    layer: CollisionLayer::World,
    padding: 0.0,
    preferred_shape: Some(PreferredShape::Box),
}
```

### Spezielle Collider:
- **Brunnen**: Cylinder (radius: 2.5, height: 3.0)
- **Statue**: Cylinder (radius: 1.0, height: 5.0)
- Alle haben `CollisionType::Static` und `CollisionLayer::World`

## Stadt-Charakteristik

### Mittelalterliche Authentizität
✓ Kathedrale als höchstes Gebäude (historisch korrekt)
✓ Wachturm für Stadtverteidigung
✓ Zentraler Marktplatz als Handelszentrum
✓ Handwerker-Viertel konzentriert
✓ Wohnviertel mit verschiedenen Hausgrößen
✓ Religiöse Gebäude prominent platziert
✓ Öffentliche Brunnen für Wasserversorgung

### Organisches Layout
- Gebäude sind leicht rotiert (nicht perfekt ausgerichtet)
- Rotation zwischen -0.4 und 0.6 Radians (~-23° bis +34°)
- Unregelmäßige Abstände zwischen Gebäuden
- Verschiedene Gebäudegrößen und -höhen
- Kein perfektes Raster-Muster

### Realistische Proportionen
- Durchschnittliche Etagen-Höhe: ~3m
- Wohnhäuser: 1.5-3 Etagen (6-10m)
- Handwerks-Gebäude: 1-2 Etagen (4.5-7m)
- Öffentliche Gebäude: 2-5 Etagen (9-16m)
- Spieler (1.8m) wirkt realistisch klein

## Technische Details

### Position-Konvention
- Y-Koordinate = Höhe/2 (Mittelpunkt des Gebäudes)
- Alle Gebäude haben `GameWorld` Component für Cleanup
- Spawn-Funktion: `setup_player()` in `client/src/player.rs`
- Cleanup-Funktion: `cleanup_player()` (löscht alle GameWorld Entities)

### Erweiterbarkeit
- Alle Gebäude sind einfache Cuboids/Cylinders
- Können später durch 3D-Modelle ersetzt werden
- Positionen und Größen bleiben gleich
- Collider müssen eventuell angepasst werden

### Stadt-Statistik
- **Gesamt-Strukturen**: 22 (17 Gebäude + 3 Stände + 2 Deko)
- **Marktplatz**: 1.600m² (40x40m)
- **Bodenfläche**: 22.500m² (150x150m)
- **Höchstes Gebäude**: Kathedrale (16m)
- **Größtes Volumen**: Kathedrale (5.600m³)
- **Durchschn. Gebäudehöhe**: ~8m

## Zukünftige Erweiterungen

### Mögliche Ergänzungen
- Stadtmauer und Tore
- Mehr Wohnhäuser in den Randbereichen
- Zusätzliche Handwerks-Gebäude
- Hafen-Bereich (falls Wasser hinzugefügt wird)
- Kaserne für Wachen
- Rathaus/Verwaltung
- Gilden-Häuser
- Straßenlaternen
- Wegweiser
- Bänke und weitere Platz-Dekoration

### Potenzielle Verbesserungen
- Texturen für verschiedene Materialien
- Innenbereiche für Gebäude
- Türen und Fenster-Details
- Dächer mit verschiedenen Formen
- Detaillierte Fassaden
- Beleuchtung (Fenster, Laternen)
- NPCs für verschiedene Gebäude
