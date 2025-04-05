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
6. `--noise`: Use a noise generation (perlin, fbm, simplex)

Example: `cargo run --release -- --width=500 --height=500 --noise perlin`

## Explored features during this project

- Quadtrees: Faster research in a grid using quadtrees.
- Cache: Shape caching for drawing terrain faster.
- Modules: Rust modules making.
- Geometry: Line directions, radian angles, collision detection, basic Euclidian mathematics
- General optimisation: Caching, clever drawing, less calculations, bigger window.
- Sounds: Understanding of the `rodio` library.
- Parallelism: Using `rayon`, we can iterate with parallel threads.

## Our goal

We were planning at the outset to use the `bevy` library which includes an ECS system. As it seemed more complicated than `ggez` which is very close to the `LÖVE` engine, our plan was to move towards a simplier solution that would guarantee satisfying results.

We wanted to create a simulation with terrain reaction to effects. The first goal was to create mainly four effects: bubbles, more bubbles, a lightning strike and an explosion.

The lightning strike is likewise based on complex mathematical notions which would have taken more time to implement, so would have the explosion.

Besides, we would have wanted to explore more ways to optimise the project to create the preceding effects.

Finally, this project can be easily modified, and new effects could be added and new noises could be added as well.

# Logs

## 13/03: Exploration

Testing `bevy` library with the ECS system and exploring Perlin noise generation.
The main difficulty was understanding how `bevy` works and how to implement Perlin noise in Rust. It seemed like `bevy` was very powerful but quite complicated to use.

# 15/03: Starting creating tiles

Switching to `ggez` library as it was easier to use. We wanted to focus on functionalities rather than on optimisation, it would be a later point. We have started by creating tiles with colours, each representing grass, rock or air, and we started added functions to generate terrain. The main difficulty was understanding how `ggez` was working.

# 16/03: User Interface and arguments

Using `clap`, we started adding the possibility to pass arguments through the command line. It would allow the user to change some settings such as the size of the grid. We likewise added a window for the project. The problem was about modifying global constants, we used `lazy_static` to fix some of those issues. We added buttons to the UI without any functionalities at first.

# 17/03: Creating effects

We added three effects: Bubbles, MoreBubbles and Lightning, each with a different system. The user can change the effects with the UI buttons. They were spawning but not moving as we did not implement movement yet.

# 18/03: Direction for effects, calculating collisions

We changed the effects structure to account for a direction. The program would move the effect relative to this direction and would be useful as we want to make the effects bounce off terrain and window edges. We likewise added the ability for effects to destroy terrain based on their lifetime. It seemed very obvious our calculations would require optimisation as more tiles and more effects spawning caused jittering.

# 23/03: Bouncing effects

We added the ability for effects to bounce off edges. We also wanted to make the effects bounce off the terrain, but adding window edges was easier at that time. The difficulty was finding the right calculation for making the bounce realistic. Likewise, the project was starting to slow down as more calculations were added. (a window of 500 by 500)

# 24/03: Caching, quadtrees, noises.

We first added a bouncing sound. This had a major problem as instancing a sound would be destroyed directly after exiting the function. Waiting for the sound to end was also not a solution as it would freeze the entire project. We eventually decided to store the sounds in an array and remove it when ended.

We started optimising the project as it was running slower than expected. Caching was a first solution: instead of creating a rectangle for each tile, we would store the mesh when creating a MainState and then using it for each tile. This helped a bit. We also decided to implement QuadTrees to reduce distance checking with tiles. The main difficulty was now to find ways to optimise the project using our knowledge and writing the code in Rust. Although optimisation was not part of the project, the knowledge put into doing so adds another challenge in this project, namely the ability to code anything in Rust. The project runs fine with a 700x500 window.

# 25/03: Optimisation, InstanceArray

Another layer of optimisation was required to achieve better final results. We changed the calculations to account for InstanceArray which is available in `ggez`. Instead of fetching the cached square and drawing it each time, we would push it into an InstanceArray which would be drawn entirely at the end of the draw loop. This massively improved performance. The main difficulty was understanding how InstanceArray was working and finally implementing it. The project is now running at 1000x500

# 26/03: New noise

We added ability to change the default generation noise through the `--noise <NOISE>` flag. The user can now use Perlin noise `perlin` or Fractal Brownian Motion noise `fbm`.
The main difficulty was changing the code to account for this new noise.

# 27/03: Lint

Using `clippy`, we fixed some linting problems. Only 5 were present, namely unnecessary casting.

# 29/03: Optimisation, terrain bouncing

As a final optimisation, instead of putting multiple rectangle images in an InstanceArray, we draw rectangles that span over multiple tiles to reduce the number of calculations. The first version was only taking into account horizontal lines. The main difficulty was implementing such an algorithm. Then, we wanted to enhance the algorithm to be in 2D and span vertically, this was quite complicated. The project is now running in 1920x1080 without any problems.

We added terrain bouncing as our final functionality (listed as our goal). The main difficulty in this situation was a double borrowing in the for loop and damage function call. We decided the manually damage the cell.

# 05/04: Added Simplex, problems with optimisation, and sounds

We added a new noise for generating terrain. To maintain a future-proof architecture, we moved the generation to a new structure named NoiseGenerator which includes Perlin, Simplex and Fbm.

After testing on low-end devices, the project seems unoptimised with the latest modifications. Indeed, the new collision system requires more calculations for each bounce, therefore we used parallelism for effects with `rayon`. Likewise, we observed that Quadtree used a recursive version of for the `query` function, it is now iterative. Finally, an final observation for general optimisation is that our drawing function was ineffective due to its nature: it was filling each InstanceArray and drawing it afterwards. It is now cached and generated once in the terrain generation function and for each "deletion" of the terrain, we change the color to white.

Eventually, destroying cells has a 10% chance of playing a sound regarding its material.

The final project works very smoothly in a 700x500 window for 4-cores machines, and seems to work very well for 1920x1080 for higher-end devices (8-cores). With further testing, a 1000x1000 window also works but may be slower.

Documentation was also fixed.

## Authors

DIARRA Amara, SERRANO Jean-Léo