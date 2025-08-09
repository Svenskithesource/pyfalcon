#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use eframe::egui;
use egui::DroppedFile;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0]) // wider for split layout
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "PyFalcon - PYC and Marshal File Processor",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::<MyApp>::default())),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

struct MyApp {
    pyc_file: Option<egui::DroppedFile>,
    marshal_file: Option<egui::DroppedFile>,
    selected_python_version: String,
    python_versions: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pyc_file: None,
            marshal_file: None,
            selected_python_version: "3.11".to_string(),
            python_versions: vec![
                "2.7".to_string(),
                "3.6".to_string(),
                "3.7".to_string(),
                "3.8".to_string(),
                "3.9".to_string(),
                "3.10".to_string(),
                "3.11".to_string(),
                "3.12".to_string(),
                "3.13".to_string(),
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("pyc_panel")
            .resizable(false)
            .exact_width(ctx.available_rect().width() / 2.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("PYC File");
                    ui.separator();

                    ui.centered_and_justified(|ui| {
                        ui.label("Drop .pyc file here");
                    });
                });
            });

        egui::SidePanel::right("marshal_panel")
            .resizable(false)
            .exact_width(ctx.available_rect().width())
            .show(ctx, |ui| {
                // Right side - Marshal file drop zone with Python version selector
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.heading("Marshal File");
                        ui.separator();

                        // Python version dropdown
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Python Version:");
                                egui::ComboBox::from_label("")
                                    .selected_text(&self.selected_python_version)
                                    .show_ui(ui, |ui| {
                                        for version in &self.python_versions {
                                            ui.selectable_value(
                                                &mut self.selected_python_version,
                                                version.clone(),
                                                version,
                                            );
                                        }
                                    });
                            });
                        });

                        ui.add_space(10.0);

                        ui.centered_and_justified(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Drop marshal file here");
                                ui.label(format!("(for Python {})", self.selected_python_version));
                            });
                        });
                    },
                );
            });

        // Handle dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    // Determine which side to drop the file based on file extension or position
                    if let Some(path) = &file.path {
                        let extension = path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        match extension.as_str() {
                            "pyc" => {
                                self.pyc_file = Some(file.clone());
                            }
                            _ => {
                                // Assume it's a marshal file if not .pyc
                                self.marshal_file = Some(file.clone());
                            }
                        }
                    } else {
                        // If no path, assume marshal file
                        self.marshal_file = Some(file.clone());
                    }
                }
            }
        });
    }
}
