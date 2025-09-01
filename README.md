# Tower Defense Game 🏗️⚔️

An educational tower defense game built in Rust using the Bevy game engine. This project serves as a learning resource for Rust game development patterns, testing strategies, and CI/CD workflows.

[![CI](https://github.com/danjdewhurst/tower-defense-rust/workflows/CI/badge.svg)](https://github.com/danjdewhurst/tower-defense-rust/actions/workflows/ci.yml)
[![Security Audit](https://github.com/danjdewhurst/tower-defense-rust/workflows/Security%20Audit/badge.svg)](https://github.com/danjdewhurst/tower-defense-rust/actions/workflows/security.yml)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Bevy](https://img.shields.io/badge/bevy-0.15-green.svg)
![Tests](https://img.shields.io/badge/tests-25%20passing-brightgreen.svg)

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

## 📚 Educational Purpose

This repository is designed as an **educational resource** for learning Rust game development. It demonstrates:

- **ECS (Entity Component System)** patterns with Bevy
- **Modular code architecture** with clear separation of concerns
- **Comprehensive unit testing** with 25+ test cases covering game logic
- **CI/CD pipelines** with automated testing, linting, and security audits
- **Self-contained design** with no external asset dependencies
- **Cross-platform compatibility** using standard Rust libraries

### 🎯 Learning Objectives

- Understanding game loops and system scheduling
- Implementing collision detection and physics
- Managing game state and resources
- Creating visual effects and sound systems
- Writing testable, maintainable game code
- Setting up professional development workflows

## 🚫 Contribution Policy

**This repository does not accept external contributions.** It is maintained as an educational reference and playground for exploring Rust game development concepts. Feel free to:

- ⭐ Star the repository if you find it useful
- 🐛 Open issues to report bugs or suggest improvements
- 🍴 Fork the repository to create your own version
- 📝 Use the code as a reference for your own projects

## 🧪 Testing

The project includes comprehensive unit tests covering:

```bash
# Run all tests
cargo test

# Run with coverage information
cargo test --verbose

# Run specific test modules
cargo test components
cargo test systems::enemy
```

**Test Coverage:**
- ✅ Component creation and validation (6 tests)
- ✅ Game state management (6 tests)  
- ✅ Enemy pathfinding and movement (8 tests)
- ✅ Sound system functionality (5 tests)
- ✅ Resource management and validation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- Inspired by classic tower defense games