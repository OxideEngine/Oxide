use eframe::{egui, epi};

use crate::components;

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

            egui::Window::new("Content Drawer")
                .anchor(eframe::emath::Align2::LEFT_BOTTOM, [0.0, 0.0])
                .collapsible(true)
                .vscroll(true)
                .hscroll(true)
                .default_width(ui.available_width())
                .show(ctx, |ui| {
                    ui.with_layout(
                        egui::Layout::left_to_right()
                            .with_main_wrap(true)
                            .with_main_justify(true)
                            .with_cross_justify(true),
                        |ui| {
                            for i in 0..50 {
                                ui.label(format!("Lorem Ipsum {}", i));
                            }
                        },
                    )
                })
        });
    }

    fn name(&self) -> &str {
        "Oxide Engine"
    }
}
