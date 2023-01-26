use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use eframe::egui::*;

struct ContentDrawerState {
    pub location: RefCell<PathBuf>,
}

#[derive(Clone)]
pub struct ContentDrawer {
    state: Arc<Mutex<ContentDrawerState>>,
}

impl ContentDrawer {
    pub fn new(location: PathBuf) -> Self {
        Self {
            state: Arc::new(Mutex::new(ContentDrawerState {
                location: RefCell::new(location),
            })),
        }
    }
}

impl Default for ContentDrawer {
    fn default() -> Self {
        Self::new(Path::new(".").to_path_buf())
    }
}

impl Widget for ContentDrawer {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.vertical(|ui| {
            ui.label("Content Drawer");
            let state_arc = self.state.clone();
            if let Ok(state) = state_arc.lock() {
                ui.label(&state.location.borrow().to_str().unwrap().to_string());

                if ui.add(Button::new("<")).clicked() {
                    let mut location = state.location.borrow_mut();
                    if let Some(parent) = location.parent() {
                        let foo: PathBuf = parent.to_path_buf();
                        *location = foo;
                    }
                }
            }

            if let Ok(mut state) = state_arc.clone().lock() {
                let location = &state.location;
                let paths = fs::read_dir(location.borrow().to_str().unwrap()).unwrap();
                ui.horizontal(|ui| {
                    for path in paths.into_iter() {
                        let path_buf = path.unwrap().path();
                        let path: RefCell<&Path> = RefCell::new(path_buf.as_ref());

                        if ui
                            .add(Button::new(
                                path.borrow().file_name().unwrap().to_str().unwrap(),
                            ))
                            .clicked()
                        {
                            if path.borrow().is_dir() {
                                state.location = path.borrow().to_path_buf().into();
                            }
                        }
                    }
                });
            }
        })
        .response
    }
}
