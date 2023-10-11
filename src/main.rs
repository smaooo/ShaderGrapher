#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use ShaderGrapher::ShaderGrapher;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;

    eframe::run_native(
        "Shader Grapher",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            #[cfg(feature = "persistence")]
            {
                Box::new(ShaderGrapher::new(cc))
            }
            #[cfg(not(feature = "persistence"))]
            Box::<ShaderGrapher>::default()
        }),
    )
    .expect("Failed to run native app");
}
