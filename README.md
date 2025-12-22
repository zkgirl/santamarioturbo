# 🎅 Santa Mario

A festive Christmas-themed platformer built with **Rust** and the **Turbo Genesis SDK**. Help Santa deliver gifts across 10 challenging levels culminating in an epic boss battle against Krampus!

## 🎥 Gameplay Preview

[![Watch Gameplay Video](https://img.youtube.com/vi/Whxqm5CN9p0/0.jpg)](https://youtu.be/Whxqm5CN9p0)


---

## 🎮 Controls

| Action | Keyboard | Gamepad |
|--------|----------|---------|
| Move Left | ← Arrow / A | D-Pad Left |
| Move Right | → Arrow / D | D-Pad Right |
| Jump | Space | A Button |
| Open Settings | ESC | Start |
| Select/Confirm | Space / Enter | A Button |
| Navigate Menu | ↑↓ Arrows | D-Pad |

### Advanced Mechanics
- **Short Hop**: Release jump early for shorter jumps
- **Coyote Time**: Jump briefly after leaving a platform edge
- **Jump Buffer**: Press jump slightly before landing for instant jumps
- **Enemy Stomp**: Land on enemy heads to defeat them (bounce automatically)

---

## 🌟 Features

### Gameplay
- **10 Unique Levels** with progressive difficulty
- **3 Enemy Types**: Gingerbread, Grinch, and Krampus
- **Boss Fight**: Epic final battle against Giant Krampus with 3 phases
- **5 Collectible Gifts** per level
- **Multi-floor Platforms**: 2nd floor elevated sections in later levels
- **Scoring System** with S/A/B/C/D ranks

### Technical
- **60 FPS** smooth gameplay
- **320×180** pixel resolution (retro aesthetic)
- **Keyboard + Gamepad** support
- **Settings Menu**: Toggle music, SFX, Arcade mode, Developer mode
- **Automatic Checkpoints**: Respawn on death with remaining lives

---

## 🗺️ Levels Overview

| Level | Name | Length | Difficulty | Features |
|-------|------|--------|------------|----------|
| 1 | Christmas Streets | ~2100px | ⭐ | Tutorial |
| 2 | Gingerbread Alley | ~2300px | ⭐ | Gingerbread enemies |
| 3 | Long Jump District | ~2250px | ⭐⭐ | Wider gaps |
| 4 | Highrise Gifts | ~2500px | ⭐⭐ | Elevated sections |
| 5 | Ice Bridge City | ~2600px | ⭐⭐⭐ | Ice physics |
| 6 | Grinch Rooftops | ~2400px | ⭐⭐⭐ | 2nd floor platforms |
| 7 | Rooftop Maze | ~2700px | ⭐⭐⭐⭐ | Complex 2-floor layout |
| 8 | Grinch Gauntlet | ~2800px | ⭐⭐⭐⭐ | Many Grinches |
| 9 | Krampus Pursuit | ~3100px | ⭐⭐⭐⭐⭐ | Hardest platforming |
| 10 | North Pole Showdown | Boss | ⭐⭐⭐⭐⭐ | Krampus Boss Fight |

---

## 👾 Enemies

### Gingerbread Man 🍪
- Basic patrol enemy
- Walks back and forth on platforms
- Defeat by stomping

### Grinch 💚
- Ranged attacker
- Throws snowballs at Santa when in range
- Faster movement speed

### Krampus 😈
- Mini-boss enemy in levels
- More aggressive behavior
- Higher speed and attack range

### Giant Krampus (Boss) 👹
- 150 HP across 3 phases
- **Phase 1** (100%→67% HP): Slow walking, basic attacks
- **Phase 2** (67%→33% HP): Faster movement, snowball attacks
- **Phase 3** (33%→0% HP): Very fast, unpredictable patterns
- Defeat by stomping on his head (10 damage per stomp)

---

## 🏆 Scoring System

| Action | Points |
|--------|--------|
| Collect Gift | 100 |
| Defeat Enemy | 200 |
| Complete Level | 500 |
| All 5 Gifts Bonus | 1,000 |
| No Deaths Bonus | 500 |
| Time Bonus | 10 pts/sec under par |

### Ranks
- **S Rank**: Perfect run (all gifts, no deaths, under par time)
- **A Rank**: Excellent performance
- **B Rank**: Good performance
- **C Rank**: Average performance
- **D Rank**: Completed level

---

## ⚙️ Settings

Access via **ESC** during gameplay:

| Setting | Description |
|---------|-------------|
| Music | Toggle background music on/off |
| SFX | Toggle sound effects on/off |
| Arcade Mode | Classic arcade-style gameplay |
| Developer Mode | Debug features for testing |

---

## 🛠️ Technical Details

### Built With
- **Language**: Rust
- **Game Engine**: [Turbo Genesis SDK](https://turbo.computer) v5.2.0
- **Target**: WebAssembly (WASM)
- **Resolution**: 320×180 pixels
- **Frame Rate**: 60 FPS

### Project Structure
```
santamario/
├── src/
│   ├── lib.rs          # Main game loop
│   ├── player.rs       # Player input and physics
│   ├── enemies.rs      # Enemy AI and behavior
│   ├── boss.rs         # Krampus boss fight logic
│   ├── levels.rs       # All 10 level definitions
│   ├── constants.rs    # Game constants and physics
│   ├── settings.rs     # Settings menu
│   └── level_select.rs # Level selection screen
├── sprites/            # 67 sprite files (PNG)
│   ├── santa*.png      # Santa animations
│   ├── grinch*.png     # Grinch animations
│   ├── boss/           # Boss sprites
│   └── levelpage/      # UI backgrounds
├── audio/              # 14 sound files
│   ├── sleighride.ogg  # Background music
│   ├── jump.mp3        # Jump sound
│   └── *.mp3           # Various SFX
├── www/                # Web deployment files
├── Cargo.toml          # Rust dependencies
└── turbo.toml          # Turbo config
```

### Physics Constants
| Constant | Value | Description |
|----------|-------|-------------|
| Gravity | 0.22 | Downward acceleration |
| Jump Velocity | -4.8 | Initial jump speed |
| Walk Speed | 1.8 | Horizontal movement |
| Max Fall Speed | 5.5 | Terminal velocity |
| Coyote Time | 7 frames | Grace period after leaving platform |
| Jump Buffer | 7 frames | Pre-landing jump input window |

---

## 🚀 Running the Game

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Turbo CLI](https://turbo.computer)

### Development
```bash
# Install Turbo CLI
cargo install turbo-genesis-cli

# Run in development mode
turbo run -w
```

### Build for Production
```bash
turbo build
```

The compiled game will be in `www/` directory, ready for web deployment.

---

## 🎄 Credits

- **Development**: Built with Turbo Genesis SDK
- **Art**: Custom pixel art sprites (32×32)
- **Audio**: Original sound effects and music

---

## 📜 License

This project is for educational and entertainment purposes.

---

**Happy Holidays! 🎁🎄❄️**
