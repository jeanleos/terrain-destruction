// -----------------------------------------------------------------------------
// File: mainstate.rs
// Description: Main game state and event handler for the Terrain Destruction game.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 15, 2025
// Last modified: March 26, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{
    Image, Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, Drawable, InstanceArray
};
use ggez::audio::{Source, SoundSource};
use noise::{NoiseFn, Perlin, Fbm};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::cell;
use std::f32::consts::TAU;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use rand::Rng;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use clap::Parser;

use rayon::prelude::*;
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};


use crate::{read_cell_size, read_delta, read_noisetype, read_screen_height, read_screen_width, read_seed, read_terrain_height, read_terrain_width};
use crate::cell::Cell;
use crate::effect::{Effect, EffectType};
use crate::materials::Material;
use crate::quadtree;
use crate::noisetypes::NoiseType;
use crate::noisegenerator::NoiseGenerator;

/// The `MainState` struct represents the main game state for the Terrain Destruction game.
/// It manages the terrain, effects, UI, audio, and game logic.
///
/// # Fields
/// - `terrain`: A 2D vector representing the terrain grid, where each cell is of type `Cell`.
/// - `effects`: A vector of active effects in the game.
/// - `seed`: The seed used for terrain generation.
/// - `noise_generator`: An instance of `NoiseGenerator` for generating terrain noise.
/// - `input_seed`: A string representing the user-provided seed for terrain generation.
/// - `is_focused_input`: A boolean indicating whether the input field is focused.
/// - `screen_width`: The width of the game screen.
/// - `screen_height`: The height of the game screen.
/// - `selected_effect`: The currently selected effect type (e.g., Bubbles, Lightning).
/// - `_stream`: The audio output stream for sound playback.
/// - `stream_handle`: A handle to the audio output stream for managing audio sinks.
/// - `sinks`: A vector of audio sinks for concurrent sound playback.
/// - `terrain_quadtree`: A quadtree for efficient spatial queries on the terrain.
/// - `quadtree_dirty`: A flag indicating whether the quadtree needs to be updated.
/// - `intro_timer`: A timer for displaying the introduction screen.
/// - `show_intro`: A boolean indicating whether the introduction screen is active.
/// - `lightning_mesh`: A mesh representing the lightning effect.
/// - `bubble_mesh`: A mesh representing the bubble effect.
/// - `more_bubble_mesh`: A mesh representing the more bubbles effect.
/// - `instances`: An instance array for rendering terrain and effects efficiently.
///
/// # Methods
/// - `new() -> GameResult<MainState>`
///   Creates a new instance of `MainState` and initializes the game state.
/// - `generate_terrain(&mut self)`
///   Generates the terrain using Perlin noise and rebuilds the quadtree.
/// - `damage_terrain_at(&mut self, x: usize, y: usize, amount: f32, ignore_durability: bool)`
///   Damages the terrain at the specified coordinates, optionally ignoring durability.
/// - `spawn_effect(&mut self, x: f32, y: f32)`
///   Spawns a new effect at the specified position.
/// - `update_effects(&mut self, ctx: &mut Context, dt: f32) -> GameResult`
///   Updates the active effects, processes damage requests, and handles sound playback.
/// - `play_sound(&mut self, sound_path: &str, volume: f32)`
///   Plays a sound from the specified file path at the given volume.
/// - `update_quadtree_if_needed(&mut self)`
///   Rebuilds the quadtree if it is marked as dirty.
///
/// # Event Handling
/// Implements `EventHandler<GameError>` for handling game events:
/// - `update(&mut self, ctx: &mut Context) -> GameResult`
///   Updates the game state, including effects and audio sinks.
/// - `draw(&mut self, ctx: &mut Context) -> GameResult`
///   Renders the game state, including terrain, effects, and UI.
/// - `mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult`
///   Handles mouse input for spawning effects or interacting with UI buttons.
pub struct MainState {
    // Terrain
    terrain: Vec<Vec<Cell>>,

    // Effects
    effects: Vec<Effect>,

    // Terrain generation
    seed: i64,
    noise_generator: NoiseGenerator,

    // UI
    input_seed: String,
    is_focused_input: bool,

    // Dimensions
    screen_width: f32,
    screen_height: f32,

    // Selected effect
    selected_effect: EffectType, // Track the currently selected effect

    // Audio-related fields
    _stream: OutputStream,
    stream_handle: Arc<OutputStreamHandle>,
    sinks: Vec<Arc<Sink>>, // Store sinks for concurrent playback

    // New: quadtree for terrain cells
    terrain_quadtree: quadtree::QuadTree,
    
    // Flag to track if the quadtree needs updating
    quadtree_dirty: bool,

    // Timer for the introduction screen
    intro_timer: f32, 
    
    // Whether the introduction screen is active
    show_intro: bool, 

    // Meshes for terrain and effects
    lightning_mesh: Mesh,
    bubble_mesh: Mesh,
    more_bubble_mesh: Mesh,

    // Instance arrays for grass and rock
    instances: InstanceArray,
}


/// # Methods
///
/// ## `new`
/// Initializes a new instance of `MainState`.
/// - Sets up the audio output stream.
/// - Initializes the terrain and quadtree.
/// - Configures various game settings like screen dimensions, selected effects, and intro timer.
/// - Generates the initial terrain using Perlin noise.
///
/// ## `generate_terrain`
/// Generates the terrain using Perlin noise and populates the quadtree with terrain cells.
/// - Assigns materials and durability to each cell based on Perlin noise values.
/// - Rebuilds the quadtree to reflect the updated terrain.
///
/// ## `damage_terrain_at`
/// Damages the terrain at a specific cell.
/// - Parameters:
///   - `x`: The x-coordinate of the cell.
///   - `y`: The y-coordinate of the cell.
///   - `amount`: The amount of damage to apply.
///   - `ignore_durability`: If `true`, the cell is destroyed regardless of its durability.
/// - Marks the quadtree as dirty if a cell is modified.
/// - Returns the material of the cell after damage is applied.
///
/// ## `spawn_effect`
/// Spawns a new effect at a specified position.
/// - Parameters:
///   - `x`: The x-coordinate of the effect's position.
///   - `y`: The y-coordinate of the effect's position.
/// - Adds the effect to the list of active effects.
///
/// ## `update_effects`
/// Updates the state of all active effects.
/// - Parameters:
///   - `ctx`: The game context.
///   - `dt`: The delta time since the last update.
/// - Handles effect movement, collision detection, and interactions with the terrain.
/// - Spawns sub-effects for certain effect types (e.g., bubbles).
/// - Plays sounds for specific events (e.g., bouncing off edges).
/// - Applies damage to terrain cells affected by effects.
/// - Removes expired effects and ensures the quadtree is up-to-date.
///
/// ## `play_sound`
/// Plays a sound effect.
/// - Parameters:
///   - `sound_path`: The file path to the sound resource.
///   - `volume`: The volume level (0.0 = mute, 1.0 = full volume).
/// - Creates a new audio sink for the sound and stores it to keep it alive.
///
/// ## `update_quadtree_if_needed`
/// Rebuilds the quadtree if it is marked as dirty.
/// - Iterates through the terrain and inserts non-air cells into the quadtree.
/// - Resets the dirty flag after rebuilding.
impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio output stream");
        
        // Initialize the quadtree covering the entire terrain area
        let qt_boundary = Rect::new(0.0, 0.0, read_terrain_width() as f32 * read_cell_size(), read_terrain_height() as f32 * read_cell_size());
        
        let mut s = MainState {
            terrain: vec![vec![
                Cell { material: Material::Air, durability: 0.0 }; read_terrain_height()
            ]; read_terrain_width()],
            effects: vec![],
            seed: read_seed(),
            noise_generator: NoiseGenerator::new(read_noisetype(), read_seed() as u32),
            input_seed: String::new(),
            is_focused_input: false,
            screen_width: read_screen_width(),
            screen_height: read_screen_height(),
            selected_effect: EffectType::Bubbles,
            _stream,
            stream_handle: Arc::new(stream_handle),
            sinks: Vec::new(),
            terrain_quadtree: quadtree::QuadTree::new(qt_boundary, 4),
            quadtree_dirty: false, // Initialize the flag
            intro_timer: 3.0, // Show the intro for 3 seconds
            show_intro: true, // Start with the introduction screen
            lightning_mesh: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(-30.0 / 2.0, -10.0 / 2.0, 30.0, 10.0),
                Color::from_rgb(255, 255, 0),
            )?,
            bubble_mesh: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                ggez::mint::Point2 { x: 0.0, y: 0.0 },
                read_cell_size(),
                0.5,
                Color::RED,
            )?,
            more_bubble_mesh: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                ggez::mint::Point2 { x: 0.0, y: 0.0 },
                read_cell_size(),
                0.5,
                Color::from_rgb(0, 0, 0),
            )?,
            instances: InstanceArray::new(ctx,Image::from_color(ctx, read_cell_size() as u32, read_cell_size() as u32, Some(Color::from_rgb(255, 255, 255)))),
        };
        s.generate_terrain();
        Ok(s)
    }

    // Generate the terrain using Perlin noise
    fn generate_terrain(&mut self) {

        // Clear the terrain and instance arrays
        self.instances.clear();

        // Generate the terrain using Perlin noise
        let actual_seed = if self.seed == -1 {
            rand::rng().random_range(0..100_000)
        } else {
            self.seed as u32
        };

        // Update the seed
        self.noise_generator.generate(actual_seed);
    
        // Generate the terrain based on Perlin noise
        let scale = 0.05;
        let terrain_width = read_terrain_width();
        let terrain_height = read_terrain_height();
    
        // Iterate over each cell in the terrain
        for x in 0..terrain_width {
            for y in 0..terrain_height {
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;
                let val = self.noise_generator.get(nx, ny);

                // Assign material and durability based on the noise value
                let (mat, dura) = if val < -0.2 {
                    (Material::Air, 0.0)
                } else if val < 0.2 {
                    (Material::Grass, 1.0)
                } else {
                    (Material::Rock, 8.0)
                };

                // Update the cell in the terrain
                self.terrain[x][y] = Cell { material: mat, durability: dura };

                let dest = ggez::mint::Point2 {
                    x: x as f32 * read_cell_size(),
                    y: y as f32 * read_cell_size(),
                };
                let dp = DrawParam::default().dest(dest);
                match mat {
                    Material::Grass => {
                        self.instances.push(dp.color(Color::from_rgb(111, 171, 51)));
                    }
                    Material::Rock => {
                        self.instances.push(dp.color(Color::from_rgb(123, 108, 113)));
                    }
                    _ => {
                        self.instances.push(dp.color(Color::from_rgb(255, 255, 255)));
                    }
                }
            }
        }
    
        // Rebuild the quadtree for the terrain.
        let boundary = Rect::new(0.0, 0.0, terrain_width as f32 * read_cell_size(), terrain_height as f32 * read_cell_size());

        // Increase capacity
        self.terrain_quadtree = quadtree::QuadTree::new(boundary, 8); 

        // Insert cells into the quadtree
        for tx in 0..terrain_width {
            for ty in 0..terrain_height {
                // Compute center of the cell.
                let cx = tx as f32 * read_cell_size() + read_cell_size() / 2.0;
                let cy = ty as f32 * read_cell_size() + read_cell_size() / 2.0;

                // Insert the cell into the quadtree
                self.terrain_quadtree.insert(quadtree::QuadTreeItem { x: cx, y: cy, tx, ty });
            }
        }
    }

    // Damage the terrain at the specified position
    fn damage_terrain_at(&mut self, x: usize, y: usize, amount: f32, ignore_durability: bool) -> Material {
        // Check if the position is within the terrain bounds
        
        let should_play_sound = self.terrain[x][y].durability - amount <= 0.0;

        // Play sound if the cell is not air and the durability will be 0
        if self.terrain[x][y].material != Material::Air && (should_play_sound || ignore_durability) && rand::rng().random_bool(0.10) {
            if self.terrain[x][y].material == Material::Grass {
                self.play_sound("resources/sounds/grass.ogg", 0.2);
            } else if self.terrain[x][y].material == Material::Rock {
                self.play_sound("resources/sounds/stone.ogg", 0.2);
            }
        }

        let cell = &mut self.terrain[x][y];
        let old_type = cell.material;
        
        if x < read_terrain_width() && y < read_terrain_height() {

            let white_dp = DrawParam::default().dest(ggez::mint::Point2 {
                x: x as f32 * read_cell_size(),
                y: y as f32 * read_cell_size(),
            }).color(Color::WHITE);

            // Apply damage if the cell is not air
            if cell.material != Material::Air {
                if ignore_durability {
                    self.instances.update((x * read_terrain_height() + y) as u32, white_dp);

                    cell.material = Material::Air;
                    cell.durability = 0.0;
                } else {
                    cell.durability -= amount;
                    if cell.durability <= 0.0 {

                        self.instances.update((x * read_terrain_height() + y) as u32, white_dp);

                        cell.material = Material::Air;
                        cell.durability = 0.0;
                    }
                }
                // Mark the quadtree as dirty
                self.quadtree_dirty = true; 
            }
            cell.material
        } else {
            Material::Air
        }
    }

    // Spawn a new effect at the specified position
    fn spawn_effect(&mut self, x: f32, y: f32) {

        // Add a new effect to the list
        self.effects.push(Effect {
            effect_type: self.selected_effect,
            position: (x, y),
            direction: rand::rng().random_range(0.0..TAU),
            started_at: Instant::now(),
            spawned: false,
        });
    }

    // Update the effects
    fn update_effects(&mut self, ctx: &mut Context, dt: f32) -> GameResult {

        // Ensure the quadtree is up-to-date
        self.update_quadtree_if_needed(); 

        // Counter to track the number of bubbles
        let mut bubble_count = 0; 
        let now = Instant::now();

        // Get the screen 
        let width = read_screen_width();
        let height = read_screen_height();

        // Collect damage requests, new effects, and sounds to play with Mutexes semaphores
        let damage_requests = Mutex::new(Vec::new());
        let new_effects = Mutex::new(Vec::new());
        let sounds_to_play = Mutex::new(Vec::new());
        let bubble_count = AtomicUsize::new(0);
        let now = Instant::now();

        // Update each effect
        self.effects.par_iter_mut().for_each(|eff| {
            let elapsed = now.duration_since(eff.started_at).as_secs_f32();
            let speed = match eff.effect_type {
                EffectType::Lightning => 200.0,
                _ => 50.0,
            };
        
            // Update the effect's position
            let dx = speed * dt * eff.direction.cos();
            let dy = speed * dt * eff.direction.sin();
            eff.position.0 += dx;
            eff.position.1 += dy;
            
            // Get screen dimensions
            let width = read_screen_width();
            let height = read_screen_height();
            
            // Check if the effect is outside the borders
            let mut bounced = (eff.position.0 <= 0.0
                || eff.position.0 >= width
                || eff.position.1 <= 0.0
                || eff.position.1 >= height);
            
            // Bounce the effect off the edges
            eff.bounce(width, height);
            
            if bounced {
                let boing_sounds = ["resources/sounds/boing/boing.ogg", "resources/sounds/boing/boing_casseur.ogg"];
                let index = rand::rng().random_range(0..boing_sounds.len());
                sounds_to_play.lock().unwrap().push(boing_sounds[index]);
            }
            
            // Process cells near the effect
            let radius = read_cell_size();
            let query_rect = ggez::graphics::Rect::new(
                eff.position.0 - radius,
                eff.position.1 - radius,
                radius * 2.0,
                radius * 2.0,
            );
            let candidates = self.terrain_quadtree.query(query_rect);
            for candidate in candidates {
                let cell_center_x = candidate.x;
                let cell_center_y = candidate.y;
                let distance = (eff.position.0 - cell_center_x).abs() + (eff.position.1 - cell_center_y).abs();
                if distance <= radius {
                    let dmg = match eff.effect_type {
                        EffectType::Bubbles => 5.0,
                        EffectType::MoreBubbles => 5.0,
                        EffectType::Lightning => 10.0,
                    };
                    let ignore_durability = matches!(eff.effect_type, EffectType::Lightning);

                    if self.terrain[candidate.tx][candidate.ty].material == Material::Air {
                        continue;
                    }

                    damage_requests.lock().unwrap().push((candidate.tx, candidate.ty, dmg, ignore_durability));

                    let cell = &self.terrain[candidate.tx][candidate.ty];
                    if cell.material != Material::Air {
                        let remaining = if ignore_durability { 0.0 } else { cell.durability - dmg };
                        if remaining > 0.0 {
                            eff.direction += std::f32::consts::PI 
                                + rand::rng().random_range(-std::f32::consts::PI..std::f32::consts::PI);
                            bounced = true;
                            break; // Bounce off the first intact cell
                        }
                    }
                }
            }
            
            // Spawn sub-effects if not already spawned
            if !eff.spawned {
                match eff.effect_type {
                    EffectType::Bubbles => {
                        if rand::rng().random_bool(0.2) && bubble_count.load(Ordering::SeqCst) < 10 {
                            let offset = rand::rng().random_range(-0.3..0.3);
                            let d1 = eff.direction + offset;
                            let d2 = eff.direction - offset;
                            {
                                let mut ne = new_effects.lock().unwrap();
                                ne.push(Effect {
                                    effect_type: EffectType::Bubbles,
                                    position: eff.position,
                                    direction: d1,
                                    started_at: Instant::now(),
                                    spawned: true,
                                });
                                ne.push(Effect {
                                    effect_type: EffectType::Bubbles,
                                    position: eff.position,
                                    direction: d2,
                                    started_at: Instant::now(),
                                    spawned: true,
                                });
                            }
                            bubble_count.fetch_add(2, Ordering::SeqCst);
                            eff.spawned = true;
                        }
                    }
                    EffectType::MoreBubbles => {
                        if rand::rng().random_bool(0.5)
                            && bubble_count.load(Ordering::SeqCst) < 10
                            && !eff.spawned
                        {
                            for _ in 0..10 {
                                if bubble_count.load(Ordering::SeqCst) >= 10 {
                                    break;
                                }
                                let offset = rand::rng().random_range(-0.5..0.5);
                                new_effects.lock().unwrap().push(Effect {
                                    effect_type: EffectType::MoreBubbles,
                                    position: eff.position,
                                    direction: eff.direction + offset,
                                    started_at: Instant::now(),
                                    spawned: true,
                                });
                                bubble_count.fetch_add(1, Ordering::SeqCst);
                            }
                            eff.spawned = true;
                        }
                    }
                    _ => {}
                }
            }
        });

        // Add new effects to the list
        self.effects.extend(new_effects.lock().unwrap().drain(..));
        
        // Process all collected damage requests after the loop
        for (tx, ty, dmg, ignore_durability) in damage_requests.lock().unwrap().drain(..) {
            self.damage_terrain_at(tx, ty, dmg, ignore_durability);
        }

        // Remove expired effects
        self.effects.retain(|eff| now.duration_since(eff.started_at) < Duration::from_secs_f32(3.0));

        // Play all collected sounds after the loop
        for sound_path in sounds_to_play.lock().unwrap().drain(..) {
            self.play_sound(sound_path, 0.2);
        }
    
        Ok(())
    }

    // Play a sound effect
    fn play_sound(&mut self, sound_path: &str, volume: f32) {

        // Open the sound file
        let file = File::open(sound_path).expect("Failed to open sound file.");

        // Decode the audio file
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");

        // Create a new sink for this sound
        let sink = Sink::try_new(&self.stream_handle).expect("Failed to create audio sink");

        // Set the volume
        sink.set_volume(volume); 

        // Play the sound
        sink.append(source);

        // Store the sink to keep it alive
        self.sinks.push(Arc::new(sink));
    }

    // Update the quadtree if needed
    fn update_quadtree_if_needed(&mut self) {

        // Check if the quadtree is dirty
        if self.quadtree_dirty {

            // Read the terrain dimensions and cell size
            let terrain_width = read_terrain_width();
            let terrain_height = read_terrain_height();
            let cell_size = read_cell_size();
            
            // Rebuild the quadtree
            let boundary = Rect::new(0.0, 0.0, terrain_width as f32 * cell_size, terrain_height as f32 * cell_size);

            // Adjust capacity
            self.terrain_quadtree = quadtree::QuadTree::new(boundary, 8);
            
            // Insert non-air cells into the quadtree
            for tx in 0..terrain_width {
                for ty in 0..terrain_height {

                    // Current cell
                    let cell = &self.terrain[tx][ty];

                    // Insert non-air cells into the quadtree
                    if cell.material != Material::Air {

                        // Compute the center of cell
                        let cx = tx as f32 * cell_size + cell_size / 2.0;
                        let cy = ty as f32 * cell_size + cell_size / 2.0;

                        // Insert the cell into the quadtree
                        self.terrain_quadtree.insert(quadtree::QuadTreeItem { x: cx, y: cy, tx, ty });
                    }
                }
            }

            // Reset the dirty flag
            self.quadtree_dirty = false;
        }
    }
}

// Implement EventHandler<ggez::GameError> properly for ggez
impl EventHandler<GameError> for MainState {

    // Implement the required event handler methods
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        // Skip updates during the introduction screen
        if self.show_intro {

            // Decrease the timer
            let dt = ctx.time.delta().as_secs_f32();
            self.intro_timer -= dt;

            // End the introduction when the timer reaches zero
            if (self.intro_timer <= 0.0) {
                self.show_intro = false;
            }
            
            // Skip other updates during the intro
            return Ok(()); 
        }

        // Rest of the update logic
        self.sinks.retain(|sink| !sink.empty());

        let dt = read_delta(); // Fixed delta time

        // Update the game state
        self.update_effects(ctx, dt)
    }

    // Implement the required event handler methods
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        self.instances.draw(&mut canvas, DrawParam::default());

        if self.show_intro {
            let mut canvas = Canvas::from_frame(ctx, Color::WHITE); // White background
        
            // Create the text with a larger font size
            let text = Text::new(TextFragment {
                text: "Destroy the terrain!".to_string(),
                scale: Some(ggez::graphics::PxScale::from(30.0)),
                ..Default::default()
            });

            // Add smaller text near the bottom border
            let footer_text = Text::new(TextFragment {
                text: "DIARRA Amara & SERRANO Jean-Léo. 2025 ESIEE Paris".to_string(),
                scale: Some(ggez::graphics::PxScale::from(16.0)),
                ..Default::default()
            });

            let footer_dims = footer_text.dimensions(ctx).unwrap_or_default();
            let footer_x = (self.screen_width - footer_dims.w) / 2.0;
            let footer_y = self.screen_height - footer_dims.h - 10.0;

            canvas.draw(
                &footer_text,
                DrawParam::default()
                    .dest(ggez::mint::Point2 { x: footer_x, y: footer_y })
                    .color(Color::BLACK),
            );
        
            let text_dims = text.dimensions(ctx).unwrap_or_default();
            let text_x = (self.screen_width - text_dims.w) / 2.0;
            let text_y = (self.screen_height - text_dims.h) / 2.0;
        
            canvas.draw(
                &text,
                DrawParam::default()
                    .dest(ggez::mint::Point2 { x: text_x, y: text_y })
                    .color(Color::BLACK),
            );
        
            canvas.finish(ctx)?;
            return Ok(());
        }

        // Use precomputed effect meshes for lightning, bubbles and more bubbles.
        // Draw effects using the precomputed meshes.
        for eff in &self.effects {
            match eff.effect_type {
                EffectType::Lightning => {
                    canvas.draw(
                        &self.lightning_mesh,
                        DrawParam::default()
                            .dest(ggez::mint::Point2 { x: eff.position.0, y: eff.position.1 })
                            .rotation(eff.direction),
                    );
                }
                EffectType::Bubbles => {
                    canvas.draw(
                        &self.bubble_mesh,
                        DrawParam::default().dest(ggez::mint::Point2 { x: eff.position.0, y: eff.position.1 }),
                    );
                }
                EffectType::MoreBubbles => {
                    canvas.draw(
                        &self.more_bubble_mesh,
                        DrawParam::default().dest(ggez::mint::Point2 { x: eff.position.0, y: eff.position.1 }),
                    );
                }
            }
        }

        // Draw buttons
        let button_width = 100.0;
        let button_height = 40.0;
        let button_spacing = 10.0;
        let button_y = 10.0;

        // Buttons array now includes Reset
        let buttons = [
            ("Reset", None), // Reset button
            ("Bubbles", Some(EffectType::Bubbles)),
            ("MoreBubbles", Some(EffectType::MoreBubbles)),
            ("Lightning", Some(EffectType::Lightning)),
        ];

        for (i, (label, effect_type)) in buttons.iter().enumerate() {
            let button_x = 10.0 + i as f32 * (button_width + button_spacing);
            let button_rect = Rect::new(button_x, button_y, button_width, button_height);

            // Change color if the button is selected
            let button_color = if let Some(effect) = effect_type {
                if *effect == self.selected_effect {
                    Color::from_rgb(150, 150, 255) // Highlighted color
                } else {
                    Color::from_rgb(100, 100, 200) // Default color
                }
            } else {
                Color::from_rgb(100, 100, 200) // Default color for Reset
            };

            let button_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), button_rect, button_color)?;
            canvas.draw(&button_mesh, DrawParam::default());

            let btn_label = Text::new(*label);
            let label_dims = btn_label.dimensions(ctx).unwrap_or_default();
            let label_x = button_x + (button_width - label_dims.w) / 2.0;
            let label_y = button_y + (button_height - label_dims.h) / 2.0;
            canvas.draw(
                &btn_label,
                DrawParam::default().dest(ggez::mint::Point2 { x: label_x, y: label_y }),
            );
        }

        canvas.finish(ctx)
    }

    // Handle mouse input
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        // Handle mouse button down events
        if button == MouseButton::Left {
            let button_width = 100.0;
            let button_height = 40.0;
            let button_spacing = 10.0;
            let button_y = 10.0;

            // Buttons array
            let buttons = [
                (None, 0, "resources/sounds/btnclick.ogg"), // Reset button
                (Some(EffectType::Bubbles), 1, "resources/sounds/btnclick.ogg"),
                (Some(EffectType::MoreBubbles), 2, "resources/sounds/btnclick.ogg"),
                (Some(EffectType::Lightning), 3, "resources/sounds/btnclick.ogg"),
            ];

            // Check if the user clicked on a button
            for (effect_type, i, sound_path) in buttons.iter() {

                // Calculate the button rectangle
                let button_x = 10.0 + *i as f32 * (button_width + button_spacing);
                let button_rect = Rect::new(button_x, button_y, button_width, button_height);

                // Check if the click is inside the button rectangle
                if button_rect.contains(ggez::mint::Point2 { x, y }) {
                    // Play the button sound
                    self.play_sound(sound_path, 0.2);

                    // Handle the button action
                    if let Some(effect) = effect_type {
                        // Update the selected effect
                        self.selected_effect = *effect; 
                    } else {
                        // Reset button logic
                        self.generate_terrain();
                        
                        // Clear all effects
                        self.effects.clear(); 
                    }
                    return Ok(());
                }
            }

            // If user clicked on the main canvas
            if y > button_y + button_height {
                self.spawn_effect(x, y);
            }
        }
        Ok(())
    }
}