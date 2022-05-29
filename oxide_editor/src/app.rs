use eframe::{egui, epi};

use crate::{components, viewmodels::content_drawer::ContentDrawerViewModel, views};

pub struct TemplateApp;

impl Default for TemplateApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("viewport");
            let mut viewport = components::viewport::ViewPort::default();
            viewport.ui(ui, egui::vec2(2000.0, 1000.0));

            views::content_drawer::content_drawer(ui, ctx, ContentDrawerViewModel::default());
        });
    }

    fn name(&self) -> &str {
        "Oxide Engine"
    }
}
