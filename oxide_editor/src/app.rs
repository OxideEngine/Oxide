use eframe::egui;

use crate::{components, viewmodels::content_drawer::ContentDrawerViewModel, views};

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("viewport");
            let mut viewport = components::viewport::ViewPort::default();
            viewport.ui(ui, egui::vec2(2000.0, 1000.0));

            views::content_drawer::content_drawer(ui, ctx, ContentDrawerViewModel::default());
        });
    }
}
