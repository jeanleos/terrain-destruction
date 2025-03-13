use bevy::{prelude::*, window::WindowResized};
use libnoise::prelude::*;

static STEP: f64 = 0.1;

fn print_noise() {
    let perlin = Source::simplex(42);
    let empty_string = "";
    let mut res_string: String;
    for x in 0..15 {
        res_string = empty_string.to_string();
        for y in 0..15 {
            let val = perlin.sample([x as f64 * STEP, y as f64 * STEP]);
            res_string.push_str(&format!("|{:^7.3}", val).to_string());
        }
        println!("{}|", res_string);
    }
}

fn set_window(keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    resolution: Res<ResolutionSettings>,) {
    window.resolution.set(200.0, 350.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (set_window, print_noise))
        .run();
}
