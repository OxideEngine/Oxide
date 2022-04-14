#[cfg(test)]
mod test {
    use crate::components::position::*;
    use crate::components::sphere::*;
    use crate::helpers::size_helpers::*;
    use legion::*;

    #[test]
    #[should_panic]
    fn no_sphere_component() {
        let mut world = World::default();
        let sphere_entity: Entity = world.push((Position {
            x: 1.0f32,
            y: 1.0f32,
        },));
        set_sphere_size(&mut world, &sphere_entity, 2.0f32);

        if let Some(entry) = world.entry(sphere_entity) {
            let sphere: Sphere = *entry.get_component::<Sphere>().unwrap();
            assert_approx_eq!(sphere.radius, 2.0f32);
        }
    }

    #[test]
    fn reset_radius() {
        let mut world = World::default();
        let sphere_entity: Entity = world.push((Sphere { radius: 1.0f32, },));
        set_sphere_size(&mut world, &sphere_entity, 2.0f32);

        if let Some(entry) = world.entry(sphere_entity) {
            let sphere: Sphere = *entry.get_component::<Sphere>().unwrap();
            assert_approx_eq!(sphere.radius, 2.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn negative_radius() {
        let mut world = World::default();
        let sphere_entity: Entity = world.push((Sphere { radius: 1.0f32, },));
        set_sphere_size(&mut world, &sphere_entity, -2.0f32);

        if let Some(entry) = world.entry(sphere_entity) {
            let sphere: Sphere = *entry.get_component::<Sphere>().unwrap();
            assert_approx_eq!(sphere.radius, -2.0f32);
        }
    }

    #[test]
    #[should_panic]
    fn zero_radius() {
        let mut world = World::default();
        let sphere_entity: Entity = world.push((Sphere { radius: 1.0f32, },));
        set_sphere_size(&mut world, &sphere_entity, 0.0f32);

        if let Some(entry) = world.entry(sphere_entity) {
            let sphere: Sphere = *entry.get_component::<Sphere>().unwrap();
            assert_approx_eq!(sphere.radius, 0.0f32);
        }
    }
}
