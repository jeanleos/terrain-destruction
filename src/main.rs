// -----------------------------------------------------------------------------
// File: main.rs
// Description: Entry point for the Terrain Destruction simulation game.
//              Handles initialization, command-line arguments, and game loop.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 13, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

#![allow(unused)]

use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, 
    Drawable,
};
use ggez::audio::{Source, SoundSource};
use noise::{NoiseFn, Perlin};
use std::f32::consts::TAU;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use rand::Rng;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use clap::Parser;

mod quadtree;
use crate::quadtree::{QuadTree, QuadTreeItem};

mod materials;
use crate::materials::Material;

mod cell;
use crate::cell::Cell;

mod effect;
use crate::effect::{Effect, EffectType};

mod mainstate;
use crate::mainstate::MainState;

// Constants
const MIN_WIDTH: u32 = 500;
const MIN_HEIGHT: u32 = 300;
const MIN_DELTA: u32 = 15;
const MIN_SIZE_CELL: f32 = 5.0;
const UI_HEIGHT: f32 = 50.0;

/// A struct representing the command-line arguments for configuring the application.
///
/// # Fields
///
/// * `width` - The width of the window. Must be at least 500. Defaults to 700.
/// * `height` - The height of the window. Must be at least 300. Defaults to 500.
/// * `delta` - The fixed delta time for the simulation in milliseconds. Must be at least 15. Defaults to 15.
/// * `cellsize` - The fixed size of each cell in the simulation. Must be at least 5.0. Defaults to 5.0.
#[derive(Parser)]
#[command(name = "Terrain Destruction")]
#[command(about = "A terrain destruction simulation", long_about = None)]
struct Args {
    /// Width of the window (minimum 500)
    #[arg(long, default_value_t = 700)]
    width: u32,

    /// Height of the window (minimum 300)
    #[arg(long, default_value_t = 500)]
    height: u32,

    /// Fixed delta time for the simulation (minimum 15)
    #[arg(long, default_value_t = 15)]
    delta: u32,

    /// Fixed cell size for the simulation (minimum 5.0)
    #[arg(long, default_value_t = 5.0)]
    cellsize: f32,

    /// Seed for Perlin noise
    #[arg(long, default_value_t = -1)]
    seed: i64,
}

// Constants
lazy_static::lazy_static! {
    static ref SCREEN_WIDTH: RwLock<f32> = RwLock::new(700.0);
    static ref SCREEN_HEIGHT: RwLock<f32> = RwLock::new(500.0);
    static ref FIXED_DELTA: RwLock<f32> = RwLock::new(1.0 / 15.0);
    static ref TERRAIN_WIDTH : RwLock<usize> = RwLock::new(140);
    static ref TERRAIN_HEIGHT : RwLock<usize> = RwLock::new(100);
    static ref CELL_SIZE: RwLock<f32> = RwLock::new(5.0);
    static ref SEED: RwLock<i64> = RwLock::new(-1);
}

// Read constants
pub fn read_screen_width() -> f32 {
    *SCREEN_WIDTH.read().unwrap()
}
pub fn read_screen_height() -> f32 {
    *SCREEN_HEIGHT.read().unwrap()
}
pub fn read_terrain_width() -> usize {
    *TERRAIN_WIDTH.read().unwrap()
}
pub fn read_terrain_height() -> usize {
    *TERRAIN_HEIGHT.read().unwrap()
}
pub fn read_delta() -> f32 {
    *FIXED_DELTA.read().unwrap()
}
pub fn read_cell_size() -> f32 {
    *CELL_SIZE.read().unwrap() as f32
}
pub fn read_seed() -> i64 {
    *SEED.read().unwrap()
}

// Update constants
fn update_constants(width: u32, height: u32, delta: u32, cell_size: f32, seed: i64) {
    let cell_size = cell_size.max(MIN_SIZE_CELL);

    // Adjust width and height to be multiples of cell_size
    let adjusted_width = ((width as f32 / cell_size).floor() * cell_size) as f32;
    let adjusted_height = ((height as f32 / cell_size).floor() * cell_size) as f32;

    let delta = 1.0 / delta.max(MIN_DELTA) as f32;

    *SCREEN_WIDTH.write().unwrap() = adjusted_width;
    *SCREEN_HEIGHT.write().unwrap() = adjusted_height;
    *FIXED_DELTA.write().unwrap() = delta;

    // Update terrain dimensions based on adjusted screen size
    let terrain_width = (adjusted_width / cell_size) as usize;
    let terrain_height = (adjusted_height / cell_size) as usize;
    *TERRAIN_WIDTH.write().unwrap() = terrain_width;
    *TERRAIN_HEIGHT.write().unwrap() = terrain_height;

    *CELL_SIZE.write().unwrap() = cell_size;

    *SEED.write().unwrap() = seed;
}

pub fn main() -> GameResult {

    // Parse command line arguments
    let args = Args::parse();

    // Initialize variables
    let width;
    let height;
    let delta;
    let cell_size;
    let seed = args.seed;

    // Check if any arguments are below the minimum values and display a message
    if args.width < MIN_WIDTH {
        println!("Warning: Width is below the minimum value of {}. Using {} instead.", MIN_WIDTH, MIN_WIDTH);
        width = MIN_WIDTH;
    } else {
        width = args.width.max(MIN_WIDTH);
    }
    if args.height < MIN_HEIGHT {
        println!("Warning: Height is below the minimum value of {}. Using {} instead.", MIN_HEIGHT, MIN_HEIGHT);
        height = MIN_HEIGHT;
    } else {
        height = args.height.max(MIN_HEIGHT);
    }
    if args.delta < MIN_DELTA {
        println!("Warning: Delta time is below the minimum value of {}. Using {} instead.", MIN_DELTA, MIN_DELTA);
        delta = MIN_DELTA;
    } else {
        delta = args.delta.max(MIN_DELTA);
    }
    if args.cellsize < MIN_SIZE_CELL {
        println!("Warning: Cell size is below the minimum value of {}. Using {} instead.", MIN_SIZE_CELL, MIN_SIZE_CELL);
        cell_size = MIN_SIZE_CELL;
    } else {
        cell_size = args.cellsize.max(MIN_SIZE_CELL);
    }

    // Update constants
    update_constants(width, height, delta, cell_size, seed);

    // Create a new context and event loop
    let cb = ContextBuilder::new("Terrain Destruction", "DIARRA&SERRANO")
        .window_setup(ggez::conf::WindowSetup::default().title("Terrain Destruction"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(*SCREEN_WIDTH.read().unwrap(), *SCREEN_HEIGHT.read().unwrap())
                .resizable(false),
        );

    // Build the context and event loop
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;

    // Run the event loop
    event::run(ctx, event_loop, state)
}
