#[cfg(test)]
mod test {
    use crate::components::position::*;
    use crate::components::rect::*;
    use crate::helpers::rect_size_helpers::*;
    use legion::*;

    #[test]
    fn init_width_and_height() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_rect_size(&mut world, &rect_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let rect: Rect = *entry.get_component::<Rect>().unwrap();
            assert_approx_eq!(rect.width, 2.0f32);
            assert_approx_eq!(rect.height, 3.0f32);
        }
    }

    #[test]
    fn reset_width_and_height() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Rect {
            width: 1.0f32,
            height: 1.0f32,
        },));
        set_rect_size(&mut world, &rect_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let rect: Rect = *entry.get_component::<Rect>().unwrap();
            assert_approx_eq!(rect.width, 2.0f32);
            assert_approx_eq!(rect.height, 3.0f32);
        }
    }
}
