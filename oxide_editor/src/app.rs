use eframe::egui;

use crate::widgets::content_drawer::ContentDrawer;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(ContentDrawer::new());
        });
    }
}
