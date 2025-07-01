#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use File_Explorer::app::FileExplorerApp;
use eframe::egui;
use display_info::DisplayInfo;
use std::sync::Arc;

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let display_info = DisplayInfo::all().unwrap(); // getting information about the display

    let icon = include_bytes!("../assets/logo.png");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                (display_info[0].width / 2) as f32,
                (display_info[0].height / 2) as f32,
            ])
            .with_icon(
                // display our icon
                Arc::new(eframe::icon_data::from_png_bytes(&icon[..]).unwrap()),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "File Explorer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<FileExplorerApp>::default())
        }),
    )
}
