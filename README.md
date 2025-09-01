# Tower Defense Game 🏗️⚔️

A basic tower defense game built in Rust using the Bevy game engine.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Bevy](https://img.shields.io/badge/bevy-0.15-green.svg)

## Features

- **One Enemy Type**: Red circles that follow a predefined path
- **One Tower Type**: Blue squares that automatically shoot at nearby enemies
- **Tower Placement**: Click anywhere to place towers (costs 20 money)
- **Basic UI**: Shows player health, score, and money
- **Sound Effects**: Console beep sounds for shooting, hits, and enemy deaths (no external files required)
- **Visual Effects**: Hit effects, explosion particles, and visual sound indicators
- **Progressive Difficulty**: Enemies spawn every 2 seconds

## Controls

- **Left Click**: Place a tower (costs 20 money)

## Game Mechanics

- Players start with 20 health and 100 money
- Enemies deal 1 damage to player health when they reach the end of the path
- Each enemy killed gives 10 points and 5 money
- Towers cost 20 money each
- Towers automatically target the closest enemy within range

## How to Run

```bash
cargo run
```

## Requirements

- Rust 1.70+
- Bevy 0.15 dependencies will be automatically downloaded

## Project Structure

- `src/main.rs` - Main game setup and systems registration
- `src/components.rs` - Game entity components (Enemy, Tower, Bullet, etc.)
- `src/systems/` - Game logic systems organized by functionality:
  - `mod.rs` - Module exports and organization
  - `setup.rs` - Game initialization (camera, path, UI setup)
  - `enemy.rs` - Enemy spawning, movement, and pathfinding
  - `tower.rs` - Tower shooting logic and placement input handling
  - `combat.rs` - Bullet movement and collision detection
  - `ui.rs` - UI updates (health, score, money display)
  - `effects.rs` - Visual effects (hit effects, explosion particles)
  - `sound.rs` - Sound system using console beeps and visual indicators
- `src/resources.rs` - Global game state and resources

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- Inspired by classic tower defense games