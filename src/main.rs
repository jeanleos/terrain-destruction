use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use libnoise::prelude::*;

static STEP: f64 = 0.1;
const X_EXTENT: f32 = 900.;


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

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let shapes = [
        meshes.add(Rectangle::new(50.0, 100.0))
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
        ));
    }
}

fn main() {
    let mut app = App::new();
        /*.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [480., 360.].into(),
                title: "Rust".to_string(),
                ..default()
            }),
            ..default()
        }), 
            #[cfg(not(target_arch = "wasm32"))]
            Wireframe2dPlugin)*/
    app.add_plugins(DefaultPlugins);
    app.add_plugins(
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin
    );
    app.add_systems(Startup, (print_noise, setup));
    app.run();
}
