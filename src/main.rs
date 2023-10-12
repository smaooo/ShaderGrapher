// TODO: Options for loading up shader graphs:
    // Exporting a custom extension file from Blender/Unity/UE
    // Reading files:
        // Unity: is JSON
        // Blender: Parse .blend binary https://crates.io/crates/blend https://github.com/fschutt/mystery-of-the-blend-backup

// TODO: Loading up Blender Node Database:
    // Blender source as submodule https://github.com/blender/blender/tree/main/source/blender/nodes/shader/nodes
    // fake_bpy_module .py files https://github.com/nutti/fake-bpy-module /bpy/types.py
    // Use bpy python module in Rust https://pyo3.rs/v0.19.2/python_from_rust



#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use ShaderGrapher::SGShaderGrapher;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // use eframe::egui::Visuals;

    // eframe::run_native(
    //     "Shader Grapher",
    //     eframe::NativeOptions::default(),
    //     Box::new(|cc| {
    //         cc.egui_ctx.set_visuals(Visuals::dark());
    //         #[cfg(feature = "persistence")]
    //         {
    //             Box::new(ShaderGrapher::new(cc))
    //         }
    //         #[cfg(not(feature = "persistence"))]
    //         Box::<ShaderGrapher>::default()
    //     }),
    // )
    // .expect("Failed to run native app");

}
