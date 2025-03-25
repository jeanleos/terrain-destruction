// -----------------------------------------------------------------------------
// File: build.rs
// Description: Build script for the Terrain Destruction simulation game.
// Author(s): DIARRA Amara & SERRANO Jean-Léo
// License: CC BY-NC 4.0
// Created: March 25, 2025
// Last modified: March 25, 2025
// Version: 1.0
// -----------------------------------------------------------------------------

use std::env;
use std::fs;
use std::path::Path;

// Copy the resources directory to the target directory during the build process.
// This is necessary to ensure that the resources are available when running the game.
fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let target_dir = Path::new(&out_dir).ancestors().nth(3).expect("Invalid OUT_DIR structure");

    let source = Path::new("resources");
    let destination = target_dir.join("resources");

    if source.exists() {
        if let Err(e) = copy_dir_all(source, &destination) {
            eprintln!("Failed to copy resources: {:?}", e);
        }
    } else {
        eprintln!("Resources directory not found!");
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}