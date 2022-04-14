#[cfg(test)]
mod test {
    use crate::components::cylinder::*;
    use crate::components::position::*;
    use crate::helpers::size_helpers::*;
    use legion::*;

    #[test]
    #[should_panic]
    fn no_cylinder_component() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 2.0f32);
            assert_approx_eq!(cylinder.height, 3.0f32);
        }
    }

    #[test]
    fn reset_radius_and_height() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 2.0f32);
            assert_approx_eq!(cylinder.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, -2.0f32, 3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, -2.0f32);
            assert_approx_eq!(cylinder.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 2.0f32, -3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 2.0f32);
            assert_approx_eq!(cylinder.height, -3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius_and_height() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, -2.0f32, -3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, -2.0f32);
            assert_approx_eq!(cylinder.height, -3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 0.0f32, 3.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 0.0f32);
            assert_approx_eq!(cylinder.height, 3.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_height() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 2.0f32, 0.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 2.0f32);
            assert_approx_eq!(cylinder.height, 0.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius_and_height() {
        let mut world = World::default();
        let cylinder_entity: Entity = world.push((Cylinder {
            radius: 1.0f32,
            height: 1.0f32,
        },));
        set_cylinder_size(&mut world, &cylinder_entity, 0.0f32, 0.0f32);

        if let Some(entry) = world.entry(cylinder_entity) {
            let cylinder: Cylinder = *entry.get_component::<Cylinder>().unwrap();
            assert_approx_eq!(cylinder.radius, 0.0f32);
            assert_approx_eq!(cylinder.height, 0.0f32);
        }
    }
}
