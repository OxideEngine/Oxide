use app::App;

mod app;
mod widgets;

fn main() {
    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "Oxide Engine",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    );
}
