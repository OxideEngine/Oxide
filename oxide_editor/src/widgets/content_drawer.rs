use std::fs;

use eframe::{
    egui::{Sense, Widget},
    epaint::Stroke,
};

pub struct ContentDrawer {
    pub location: String,
}

impl ContentDrawer {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
        }
    }

    pub fn list_location(self: &Self) -> Vec<String> {
        let paths = fs::read_dir(&self.location).unwrap();
        paths.map(|path| path.unwrap().file_name().to_str().unwrap().to_string()).collect()
    }
}

impl Widget for ContentDrawer {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let width = ui.available_width();
        let height = 100.0;
        let (outer_rect, resp) = ui.allocate_at_least(
            eframe::emath::Vec2 {
                x: width,
                y: height,
            },
            Sense::click(),
        );

        let visuals = ui.style().visuals.clone();
        ui.painter()
            .rect(outer_rect, 0.0, visuals.extreme_bg_color, Stroke::none());

        resp
    }
}
