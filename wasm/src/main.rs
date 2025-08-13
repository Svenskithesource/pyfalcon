#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0]) // wider for split layout
            .with_drag_and_drop(true),
        ..Default::default()
    };

    // ansi colors don't work in egui
    core::disable_colors();

    eframe::run_native(
        "PyFalcon - PYC and Marshal File Processor",
        options,
        Box::new(|_cc| Ok(Box::<PyFalcon>::default())),
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

        // ansi colors don't work in egui
        core::disable_colors();

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_| Ok(Box::<PyFalcon>::default())),
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

#[derive(Default)]
struct PyFalcon {
    pyc_file: Option<Vec<u8>>,
    disassembled_text: Option<String>,
}

impl eframe::App for PyFalcon {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.pyc_file {
            None => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("pyfalcon");
                        ui.separator();

                        ui.centered_and_justified(|ui| {
                            ui.label("Drop .pyc file here");
                        });
                    });
                });

                // Handle dropped files
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        for file in &i.raw.dropped_files {
                            match &file.bytes {
                                None => {
                                    continue; // Can't read file
                                }
                                Some(contents) => {
                                    self.pyc_file = Some(contents.to_vec());
                                    self.disassembled_text = None;
                                    break; // Pick first file
                                }
                            }
                        }
                    }
                });
            }
            Some(data) => {
                let data = data.clone();
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("pyfalcon");
                        if ui.button("Close file").clicked() {
                            self.pyc_file = None;
                        }
                    });

                    ui.separator();

                    let code_object = pyc_editor::load_pyc(std::io::Cursor::new(data.to_vec()))
                        .map(|pyc| match pyc {
                            pyc_editor::PycFile::V310(pyc_file) => {
                                pyc_editor::CodeObject::V310(pyc_file.code_object)
                            }
                            pyc_editor::PycFile::V311(pyc_file) => {
                                pyc_editor::CodeObject::V311(pyc_file.code_object)
                            }
                        });

                    match code_object {
                        Ok(code_object) => {
                            let mut text = match &self.disassembled_text {
                                None => {
                                    let text = core::disassemble_code(&code_object, true);
                                    self.disassembled_text = Some(text.clone());
                                    text
                                }
                                Some(text) => text.to_string(),
                            };

                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut text)
                                            .font(egui::TextStyle::Monospace)
                                            .code_editor()
                                            .desired_rows(20)
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                        }
                        Err(e) => {
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("Failed to load file: {e}"),
                            );
                        }
                    }
                });
            }
        }
    }
}
