use crate::components::circle::*;
use crate::components::cone::*;
use crate::components::cube::*;
use crate::components::cylinder::*;
use crate::components::rect::*;
use crate::components::sphere::*;
use legion::*;

pub fn set_circle_size(world: &mut World, entity: &Entity, radius: f32) {
    assert!(radius > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Circle>() {
            Ok(comp) => comp.radius = radius,
            Err(_) => panic!("Circle component does not exist"),
        }
    }
}

pub fn set_cone_size(world: &mut World, entity: &Entity, radius: f32, height: f32) {
    assert!(radius > 0.0f32 && height > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Cone>() {
            Ok(comp) => {
                comp.radius = radius;
                comp.height = height;
            },
            Err(_) => panic!("Cone component does not exist"),
        }
    }
}

pub fn set_cube_size(world: &mut World, entity: &Entity, width: f32, height: f32, length: f32) {
    assert!(width > 0.0f32 && height > 0.0f32 && length > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Cube>() {
            Ok(comp) => {
                comp.width = width;
                comp.height = height;
                comp.length = length;
            }
            Err(_) => panic!("Cube component does not exist"),
        }
    }
}

pub fn set_cylinder_size(world: &mut World, entity: &Entity, radius: f32, height: f32) {
    assert!(radius > 0.0f32 && height > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Cylinder>() {
            Ok(comp) => {
                comp.radius = radius;
                comp.height = height;
            },
            Err(_) => panic!("Cylinder component does not exist"),
        }
    }
}

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

pub fn set_sphere_size(world: &mut World, entity: &Entity, radius: f32) {
    assert!(radius > 0.0f32);

    if let Some(mut entry) = world.entry(*entity) {
        match entry.get_component_mut::<Sphere>() {
            Ok(comp) => comp.radius = radius,
            Err(_) => panic!("Sphere component does not exist"),
        }
    }
}
