#[cfg(test)]
mod test {
    use crate::components::position::*;
    use crate::components::cube::*;
    use crate::helpers::size_helpers::*;
    use legion::*;

    #[test]
    #[should_panic]
    fn no_cube_component() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, 3.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    fn reset_width_and_height() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32,
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, 3.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32,
        },));
        set_cube_size(&mut world, &rect_entity, -2.0f32, 3.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, -2.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32,
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, -3.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, -3.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_length() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32,
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, 3.0f32, -4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, -4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_width_height_and_length() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32
        },));
        set_cube_size(&mut world, &rect_entity, -2.0f32, -3.0f32, -4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, -2.0f32);
            assert_approx_eq!(cube.height, -3.0f32);
            assert_approx_eq!(cube.length, -4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_width() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32
        },));
        set_cube_size(&mut world, &rect_entity, 0.0f32, 3.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 0.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_height() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, 0.0f32, 4.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, 0.0f32);
            assert_approx_eq!(cube.length, 4.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_length() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32
        },));
        set_cube_size(&mut world, &rect_entity, 2.0f32, 3.0f32, 0.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 2.0f32);
            assert_approx_eq!(cube.height, 3.0f32);
            assert_approx_eq!(cube.length, 0.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_width_height_and_length() {
        let mut world = World::default();
        let rect_entity: Entity = world.push((Cube {
            width: 1.0f32,
            height: 1.0f32,
            length: 1.0f32
        },));
        set_cube_size(&mut world, &rect_entity, 0.0f32, 0.0f32, 0.0f32);

        if let Some(entry) = world.entry(rect_entity) {
            let cube: Cube = *entry.get_component::<Cube>().unwrap();
            assert_approx_eq!(cube.width, 0.0f32);
            assert_approx_eq!(cube.height, 0.0f32);
            assert_approx_eq!(cube.length, 0.0f32);
        }
    }
}
