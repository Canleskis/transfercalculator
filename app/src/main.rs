use eframe::{run_native, NativeOptions};
use app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = app::Gui::default();
    let win_option = NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1500.0, 1000.0)),
        ..NativeOptions::default()
    };
    run_native(Box::new(app), win_option);
}
