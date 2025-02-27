# My RTS Game

A real-time strategy game built with Rust and the Bevy game engine.

## For Players

### Game Overview

In this game, players manage resources, build structures, and command units to defeat opponents. Gather resources, expand your base, and outmaneuver your enemies to achieve victory.

### Features

- Worker units that gather resources and build structures
- Multiple resource types (Wood, Stone, Gold)
- Resource management and economy
- Unit selection and command system
- Inventory management for resource carriers

### Controls

- **Left-click**: Select units
- **Left-click and drag**: Create selection box for multiple units
- **Right-click**: Move selected units
- **Right-click on resource**: Command selected workers to gather resource

### System Requirements

- Operating system: Windows, macOS, or Linux
- GPU with Vulkan support
- 4GB RAM recommended
- 500MB disk space

### Installation & Running

1. Download the latest release package
2. Extract the files to your preferred location
3. Run the executable file (my-rts-game.exe on Windows)

## For Developers

### Current Implementation

#### Core Systems

- Entity-component system architecture using Bevy ECS
- Game state management
- Time and frame-based updates

#### Units and Movement

- Worker units with animation states
- Point-and-click movement system
- Unit selection with click and drag selection box
- Visual selection indicators (selection rings)
- Destination markers when issuing move commands

#### Resource System

- Multiple resource types with custom properties
- Resource nodes with visual representation
- Player resource tracking
- Unit inventory system for carrying gathered resources

#### User Interface

- Unit information panel displaying selected unit data
- Resource counters showing available resources
- Inventory display for selected units
- Custom UI components and styling

### Code Structure

The project is organized into several key modules:

- `components/`: Component definitions for the ECS
  - `unit.rs`: Unit-related components
  - `resource.rs`: Resource-related components
  - `inventory.rs`: Inventory system components
  - `ui.rs`: User interface components

- `resources/`: Game resources (Bevy ECS Resources)
  - Resource definitions
  - Player resources tracking
  - Game state management

- `systems/`: Game systems that operate on components
  - `movement.rs`: Unit movement handling
  - `selection.rs`: Unit selection logic
  - `gathering.rs`: Resource gathering mechanics
  - `animation.rs`: Animation state management
  - `ui.rs`: UI update and management

- `main.rs`: Application setup and initialization

### Development Setup

#### Prerequisites

- Rust and Cargo
- Compatible IDE (VS Code recommended with rust-analyzer extension)
- Git

#### Getting Started

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).
2. Clone the repository:

   ```shell
   git clone <repository-url>
   cd my-rts-game
   ```

3. Build the project:

   ```shell
   cargo build
   ```

4. Run the game:

   ```shell
   cargo run
   ```

## Contribution Guidelines

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your branch and create a pull request.
