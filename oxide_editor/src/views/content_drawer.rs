use eframe::egui;

use crate::viewmodels;

pub fn content_drawer(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    vm: viewmodels::content_drawer::ContentDrawerViewModel,
) {
    match vm.list_current() {
        Some(contents) => {
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
                            for content in contents {
                                match content {
                                    Some(path) => {
                                        ui.label(path.to_str().unwrap());
                                    }
                                    None => {}
                                }
                            }
                        },
                    )
                });
        }
        None => {}
    }
}
