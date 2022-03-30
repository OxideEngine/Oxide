use legion::*;
use crate::components::rect::*;

pub fn set_rect_size(world: &mut World, entity: &Entity, width: f32, height: f32){
    assert!(width > 0.0f32 && height > 0.0f32);
    if let Some(mut entry) = world.entry(*entity){
        entry.add_component(
            Rect{
                width: width,
                height: height,
            }
        );
    }
}
