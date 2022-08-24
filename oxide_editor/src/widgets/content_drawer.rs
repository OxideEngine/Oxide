use std::fs;

use eframe::egui::Widget;


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
        ui.vertical(|ui| {
            ui.label("Content Drawer");
            ui.horizontal(|ui| {
                self.list_location().iter().for_each(move |path| {
                    ui.label(path);
                })
            });
        }).response
    }
}
