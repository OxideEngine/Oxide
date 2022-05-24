use eframe::egui;

pub struct ViewPort {
    texture: Option<egui::TextureHandle>,
}

impl Default for ViewPort {
    fn default() -> Self {
        Self { texture: None }
    }
}

impl ViewPort {
    pub fn ui(&mut self, ui: &mut egui::Ui, desired_size: egui::Vec2) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("viewport", egui::ColorImage::example())
        });

        ui.image(texture, desired_size);
    }
}
