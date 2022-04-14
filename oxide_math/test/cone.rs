#[cfg(test)]
mod test {
    use crate::components::cone::*;
    use crate::components::position::*;
    use crate::helpers::size_helpers::*;
    use legion::*;

    #[test]
    #[should_panic]
    fn no_cone_component() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 2.0f32);
            assert_approx_eq!(cone.height, 3.0f32);
        }
    }

    #[test]
    fn reset_radius_and_height() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 2.0f32);
            assert_approx_eq!(cone.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, -2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, -2.0f32);
            assert_approx_eq!(cone.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 2.0f32, -3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 2.0f32);
            assert_approx_eq!(cone.height, -3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius_and_height() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, -2.0f32, -3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, -2.0f32);
            assert_approx_eq!(cone.height, -3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 0.0f32, 3.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 0.0f32);
            assert_approx_eq!(cone.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_height() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 2.0f32, 0.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 2.0f32);
            assert_approx_eq!(cone.height, 0.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius_and_height() {
        let mut world = World::default();
        let cone_entity: Entity = world.push((Cone {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cone_size(&mut world, &cone_entity, 0.0f32, 0.0f32);

        if let Some(entry) = world.entry(cone_entity) {
            let cone: Cone = *entry.get_component::<Cone>().unwrap();
            assert_approx_eq!(cone.radius, 0.0f32);
            assert_approx_eq!(cone.height, 0.0f32);
        }
    }
}
