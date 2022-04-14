#[cfg(test)]
mod test {
    use crate::components::circle::*;
    use crate::components::position::*;
    use crate::helpers::size_helpers::*;
    use legion::*;

    #[test]
    #[should_panic]
    fn no_circle_component() {
        let mut world = World::default();
        let circle_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_circle_size(&mut world, &circle_entity, 2.0f32);

        if let Some(entry) = world.entry(circle_entity) {
            let circle: Circle = *entry.get_component::<Circle>().unwrap();
            assert_approx_eq!(circle.radius, 2.0f32);
        }
    }

    #[test]
    fn reset_radius() {
        let mut world = World::default();
        let circle_entity: Entity = world.push((Circle { radius: 1.0f32 },));
        set_circle_size(&mut world, &circle_entity, 2.0f32);

        if let Some(entry) = world.entry(circle_entity) {
            let circle: Circle = *entry.get_component::<Circle>().unwrap();
            assert_approx_eq!(circle.radius, 2.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius() {
        let mut world = World::default();
        let circle_entity: Entity = world.push((Circle { radius: 1.0f32 },));
        set_circle_size(&mut world, &circle_entity, -2.0f32);

        if let Some(entry) = world.entry(circle_entity) {
            let circle: Circle = *entry.get_component::<Circle>().unwrap();
            assert_approx_eq!(circle.radius, -2.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius() {
        let mut world = World::default();
        let circle_entity: Entity = world.push((Circle { radius: 1.0f32 },));
        set_circle_size(&mut world, &circle_entity, 0.0f32);

        if let Some(entry) = world.entry(circle_entity) {
            let circle: Circle = *entry.get_component::<Circle>().unwrap();
            assert_approx_eq!(circle.radius, 0.0f32);
        }
    }
}
