use eframe::{
    egui::{Sense, Widget},
    epaint::Stroke,
};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct ContentDrawer {}

impl ContentDrawer {
    pub fn new() -> Self {
        Self {}
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
