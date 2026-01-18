# 🦀 Crablo

A roguelike dungeon crawler built with Rust and [Macroquad](https://macroquad.rs/).

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Macroquad](https://img.shields.io/badge/Macroquad-0.4-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- **Procedurally Generated Dungeons** - Every floor is unique
- **Multiple Monster Types** - Fast, Normal, Tank, and Boss enemies
- **4 Player Abilities**
  - `SPACE` - Dash through tiles
  - `Q` - Area attack (damages all adjacent enemies)
  - `E` - Heal (restore 25% HP)
  - `R` - Ranged attack (hit enemies from distance)
- **Equipment System** - Find Swords, Shields, and Rings
- **Experience & Leveling** - Gain XP, level up, get stronger
- **Shop System** - Buy upgrades every 3 floors
- **Traps** - Watch out for spikes and poison!
- **Fog of War** - Explore to reveal the map
- **3 Difficulty Levels** - Easy, Normal, Hard
- **Hall of Fame** - SQLite-backed high score persistence
- **Save/Load** - Continue your adventure later

## Screenshots

```
    ╔══════════════════════════════════════╗
    ║           C R A B L O                ║
    ║                                      ║
    ║   [EASY]  [NORMAL]  [HARD]          ║
    ║                                      ║
    ║        Press ENTER to start          ║
    ╚══════════════════════════════════════╝
```

## Controls

| Key                 | Action        |
| ------------------- | ------------- |
| `WASD` / Arrow Keys | Move          |
| `Left Click`        | Move / Attack |
| `Space`             | Dash          |
| `Q`                 | Area Attack   |
| `E`                 | Heal          |
| `R`                 | Ranged Attack |
| `P` / `Escape`      | Pause         |

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/yourusername/crablo.git
cd crablo

# Run the game
cargo run --release
```

## Project Structure

```
src/
├── main.rs              # Game loop and state management
├── lib.rs               # Library entry point
├── core/
│   ├── constants.rs     # Game configuration
│   ├── database.rs      # SQLite persistence
│   ├── game.rs          # Main game state
│   ├── player.rs        # Player entity
│   ├── shop.rs          # Shop logic
│   └── traits.rs        # Damageable, DamageDealer traits
├── systems/
│   ├── audio.rs         # Sound effects
│   ├── game_renderer.rs # High-level rendering
│   ├── pathfinding.rs   # BFS pathfinding
│   └── rendering.rs     # Drawing primitives
└── world/
    ├── entities.rs      # Monsters, items, effects
    └── map.rs           # Procedural map generation
```

## Architecture

The codebase follows clean architecture principles:

- **Trait-based design** - `Damageable` and `DamageDealer` traits for polymorphic behavior
- **Separation of concerns** - Core logic, systems, and world are decoupled
- **Single Responsibility** - Each module has a focused purpose

## Dependencies

| Crate                                           | Purpose                                  |
| ----------------------------------------------- | ---------------------------------------- |
| [macroquad](https://crates.io/crates/macroquad) | Game framework (rendering, input, audio) |
| [rusqlite](https://crates.io/crates/rusqlite)   | SQLite database for persistence          |

## License

MIT License - feel free to use this code for your own projects.

---

_Made with 🦀 and ❤️_
