mod app;
mod components;

fn main() {
    let app = app::TemplateApp::default();
    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}