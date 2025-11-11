# 🚀 PHASE 2: MESH BATCHING - PERFORMANCE OPTIMIZATION

## Problem
- CPU bei 100%, GPU bei 1-2% Auslastung
- CPU-Bottleneck durch zu viele Draw Calls
- ~1000-1500 Entities nur für die Stadt
- Jedes Fenster, jeder Balken = separater Draw Call

## Lösung: Material-basiertes Mesh Batching

### Implementiert:
1. **MeshBuilder System** (`client/src/building/mesh_combiner.rs`)
   - Kombiniert hunderte kleine Meshes zu wenigen großen
   - add_quad(), add_cuboid(), add_cylinder() Funktionen
   - Reduziert Draw Calls drastisch

2. **Optimized Building Spawner** (`client/src/building/optimized_spawner.rs`)
   - `spawn_building_optimized()` - neue Funktion
   - Kombiniert alle Meshes eines Gebäudes nach Material:
     - 1 Mesh für ALLE Wände (gleiches Material)
     - 1 Mesh für ALLE Fenster (gleiches Material)
     - 1 Mesh für ALLE Fachwerk-Balken (gleiches Material)
     - 1 Mesh für Dach
     - 1 Mesh für Dekorationen

3. **Alle 17 Gebäude konvertiert** zu optimierter Version

## Erwartete Verbesserungen:

### Entity Count Reduktion:
| Vorher | Nachher | Reduktion |
|--------|---------|-----------|
| ~100 Entities/Gebäude | ~6 Entities/Gebäude | **-94%** |
| ~1700 Entities gesamt | ~100 Entities gesamt | **-94%** |

### Draw Calls:
| Vorher | Nachher | Reduktion |
|--------|---------|-----------|
| ~1700 Draw Calls | ~100 Draw Calls | **-94%** |

### CPU-Last:
- **Erwartete Reduktion: 60-80%**
- GPU-Auslastung sollte steigen (mehr Polygone pro Frame)
- FPS sollte sich verdoppeln oder verdreifachen!

## Technische Details:

### Vorher (Pro Gebäude):
```
Ground Floor:   1 Entity
Mid Floor 1:    1 Entity + Trim
Mid Floor 2:    1 Entity + Trim
Top Floor:      1 Entity + Trim
Roof:           1 Entity
Windows:        20-40 Entities
Fachwerk:       10-30 Entities
Decorations:    10-20 Entities
Door:           1 Entity
---
TOTAL:          ~60-120 Entities
```

### Nachher (Pro Gebäude):
```
Walls Combined:    1 Entity (alle Stockwerke)
Trim Combined:     1 Entity (alle Trims)
Windows Combined:  1 Entity (alle Fenster)
Fachwerk Combined: 1 Entity (alle Balken)
Roof:              1 Entity
Decorations:       1 Entity (Tür, Chimney)
---
TOTAL:             ~6 Entities
```

## GPU vs CPU Rendering:

### CPU-Bottleneck (Vorher):
```
CPU: for entity in 1700 {
    prepare_draw_call(entity.mesh, entity.material, entity.transform)
    send_to_gpu()
}
GPU: *wartet auf CPU*
```

### GPU-Optimiert (Nachher):
```
CPU: for entity in 100 {
    prepare_draw_call(entity.combined_mesh, entity.material, entity.transform)
    send_to_gpu()  // Mesh hat jetzt 100x mehr Polygone!
}
GPU: *rendert endlich viel!*
```

## Weitere Optimierungen (Phase 3 - optional):

1. **GPU Instancing**
   - Identische Objekte (z.B. 100 gleiche Fenster) = 1 Draw Call
   - Transforms auf GPU übertragen

2. **Level-of-Detail (LOD)**
   - Fern-Gebäude (>50m): Vereinfacht
   - Sehr fern (>100m): Billboard oder unsichtbar

3. **Occlusion Culling**
   - Gebäude hinter anderen werden nicht gerendert

## Files Changed:
- `client/src/building/mesh_combiner.rs` (neu)
- `client/src/building/optimized_spawner.rs` (neu)
- `client/src/building/mod.rs`
- `client/src/building/city.rs`
- `client/src/physics.rs` (Rapier Debug default OFF)
- `client/src/ui/game_ui.rs` (FPS Counter)
- `client/src/main.rs` (Diagnostics Plugin)

## Testing:
1. Starte Client: `./run_client.sh`
2. Schaue FPS Counter (oben links)
3. Drücke F1 um Rapier Wireframes zu sehen (jetzt standardmäßig aus!)
4. Vergleiche FPS mit vorheriger Version

**Erwartete FPS-Steigerung: 2-3x höher!**
