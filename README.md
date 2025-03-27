# Terrain Destruction Simulation

A procedural terrain destruction simulation game built with [ggez](https://ggez.rs/) and Rust. This project demonstrates procedural terrain generation and interactive effects.

## Features

- **Procedural Terrain Generation**: Uses Perlin noise to generate dynamic terrain.
- **Destructible Terrain**: Interact with the terrain using various effects.
- **Command-Line Configuration**: Customize the simulation with command-line arguments.
- **Audio Effects**: Includes sound effects for interactions.
- **Efficient Spatial Queries**: Uses a quadtree for efficient effect and terrain interaction.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- [ggez](https://ggez.rs/) library for game development

### Dependencies 

- ggez (0.9.3)
- rand (0.9.0)
- clap (4.5.32)
- rodio (0.20.1)
- lazy_static (1.5.0)
- noise (0.9.0)
- rayon (1.10.0)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/jeanleos/terrain-destruction.git
   cd terrain-destruction

2. `cargo build`

### Execution

`cargo run --release -- --noise <noise>`

One can provide flags to modify some properties in the project.

1. `--help`: Shows all the properties
2. `--width`: Changes the window's width
3. `--height`: Changes the window's height
4. `--cellsize`: Changes the size of cells (in pixels)
5. `--seed`: Changes the current noise's seed for terrain generation
6. `--noise`: Use a noise generation (perlin, fbm)

Example: `cargo run --release -- --width=500 --height=500 --noise perlin`

## Explored features during this project

- Quadtrees: Faster research in a grid using quadtrees.
- Cache: Shape caching for drawing terrain faster.
- Modules: Rust modules making.
- Geometry: Line directions, radian angles, collision detection, basic Euclidian mathematics
- General optimisation.
- Sounds: Understanding of the `rodio` library.

## Our goal

We were planning at the outset to use the `bevy` library which includes an ECS system. As it seemed more complicated than `ggez` which is very close to the `LÖVE` engine, our plan was to move towards a simplier solution that would guarantee satisfying results.

We wanted to create a simulation with terrain reaction to effects. The first goal was to create mainly four effects: bubbles, more bubbles, a lightning strike and an explosion.

The lightning strike is likewise based on complex mathematical notions which would have taken more time to implement, so would have the explosion.

Besides, we would have wanted to explore more ways to optimise the project to create the preceding effects. For instance, exploring parallelism with rayon was in our schedule but requires refactoring every for loop.

Finally, this project can be easily modified, and new effects could be added.

# Logs

- 13/03: Testing `bevy` library with the ECS system and exploring Perlin noise generation.
- 15/03: Switching to `ggez` library, creating tiles with colours.
- 16/03: Command line arguments, creating a window, buttons.
- 17/03: Making effects.
- 18/03: Direction for effects, Calculating collisions.
- 23/03: Bouncing effects.
- 24/03: Caching, quadtrees, noises.
- 25/03: Optimisation + InstanceArray
- 26/03: Added ability to change the default generation noise (Perlin or Fractal Brownian Motion)
- 27/03: Lint.

## Authors

DIARRA Amara, SERRANO Jean-Léo