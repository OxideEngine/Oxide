use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use eframe::egui::*;

struct ContentDrawerState {
    pub location: RefCell<PathBuf>,
}

#[derive(Clone)]
pub struct ContentDrawer {
    state: Rc<RefCell<ContentDrawerState>>,
}

impl ContentDrawer {
    pub fn new(location: PathBuf) -> Self {
        Self {
            state: Rc::new(RefCell::new(ContentDrawerState {
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
            let state = self.state.borrow();
            let mut location = state.location.borrow_mut();
            ui.label(&location.to_str().unwrap().to_string());

            if ui.add(Button::new("<")).clicked() {
                if let Some(parent) = location.parent() {
                    *location = parent.to_path_buf();
                }
            }

            let paths = fs::read_dir(location.to_str().unwrap()).unwrap();
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
                            *location = path.borrow().to_path_buf().into();
                        }
                    }
                }
            });
        })
        .response
    }
}
