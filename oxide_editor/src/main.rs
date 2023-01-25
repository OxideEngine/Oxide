use crate::widgets::content_drawer::ContentDrawer;
use eframe::egui;

mod widgets;

#[cfg(test)]
mod test;

fn main() {
    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "Oxide Engine",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

#[derive(Default)]
pub struct App {
    content_drawer: ContentDrawer,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(self.content_drawer.clone());
        });
    }
}
