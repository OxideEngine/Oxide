use crate::components::rect::*;
use legion::*;

pub fn set_rect_size(world: &mut World, entity: &Entity, width: f32, height: f32) {
    assert!(width > 0.0f32 && height > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Rect>() {
            Ok(comp) => {
                comp.width = width;
                comp.height = height;
            }
            Err(_) => panic!("Rect component does not exist"),
        }
    }
}
