#![allow(dead_code)]
#![allow(unused_variables)]

use num_traits::pow;
use oxide_math::commons::vector::*;
use oxide_math::commons::vector3::Vector3;

extern crate generational_arena;
use generational_arena::Arena;

pub struct Particle {
    inverse_mass: f32,
    damping: f32,
    position: Vector3,
    pub velocity: Vector3,
    force_accum: Vector3,
    acceleration: Vector3,
}

impl Particle {
    fn integrate(&mut self, duration: f32) {
        // not to integrate things with zero mass
        if self.inverse_mass <= 0.0f32 {
            return;
        }
        assert!(duration > 0.0);

        // update linear position
        self.position.x += self.velocity.scale(duration).x;
        self.position.y += self.velocity.scale(duration).y;
        self.position.z += self.velocity.scale(duration).z;

        // work out the acceleration from the force
        let delta = self.force_accum.scale(self.inverse_mass);
        let resulting_acc = Vector3 {
            x: self.acceleration.x + delta.x,
            y: self.acceleration.y + delta.y,
            z: self.acceleration.z + delta.z,
        };

        // update linear velocity from the acceleration
        self.velocity.x += resulting_acc.scale(duration).x;
        self.velocity.y += resulting_acc.scale(duration).y;
        self.velocity.z += resulting_acc.scale(duration).z;

        // impose drag
        self.velocity = self.velocity.scale(pow(self.damping, duration as usize));

        Particle::clear_accumulator(self);
    }

    fn set_mass(&mut self, mass: f32) {
        assert!(mass != 0.0f32);
        self.inverse_mass = (1.0f32) / mass;
    }

    pub fn get_mass(&self) -> f32 {
        if self.inverse_mass == 0.0f32 {
            f32::MAX
        } else {
            1.0f32 / self.inverse_mass
        }
    }

    fn set_inverse_mass(&mut self, inverse_mass: f32) {
        self.inverse_mass = inverse_mass;
    }

    fn get_inverse_mass(&mut self) -> f32 {
        self.inverse_mass
    }

    pub fn has_finite_mass(&self) -> bool {
        self.inverse_mass >= 0.0f32
    }

    fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
    }

    fn get_damping(&self) -> f32 {
        self.damping
    }

    fn set_position(&mut self, position: &Vector3) {
        self.position = Vector3 {
            x: position.x,
            y: position.y,
            z: position.z,
        };
    }

    fn set_position_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vector3 { x, y, z }
    }

    fn get_position(&self) -> &Vector3 {
        &self.position
    }

    fn set_velocity(&mut self, velocity: &Vector3) {
        self.velocity = Vector3 {
            x: velocity.x,
            y: velocity.y,
            z: velocity.z,
        };
    }

    fn set_velocity_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = Vector3 { x, y, z }
    }

    fn get_velocity_into_vec(&self, velocity: &mut Vector3) {
        velocity.x = self.velocity.x;
        velocity.y = self.velocity.y;
        velocity.z = self.velocity.z;
    }

    fn get_velocity(&mut self) -> &Vector3 {
        &self.velocity
    }

    fn set_acceleration(&mut self, acceleration: &Vector3) {
        self.acceleration = Vector3 {
            x: acceleration.x,
            y: acceleration.y,
            z: acceleration.z,
        }
    }

    fn set_acceleration_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration = Vector3 { x, y, z }
    }

    fn get_acceleration_into_vec(&self, acceleration: &mut Vector3) {
        acceleration.x = self.acceleration.x;
        acceleration.y = self.acceleration.y;
        acceleration.z = self.acceleration.z;
    }

    fn get_acceleration(&self) -> &Vector3 {
        &self.acceleration
    }

    fn clear_accumulator(&mut self) {
        self.force_accum = Vector3 {
            x: 0.0f32,
            y: 0.0f32,
            z: 0.0f32,
        };
    }

    pub fn add_force(&mut self, force: &Vector3) {
        self.force_accum = Vector3 {
            x: self.force_accum.x + force.x,
            y: self.force_accum.y + force.y,
            z: self.force_accum.z + force.z,
        };
    }
}

// The default particle set containing all the particles added to the world
// Uses arena to avoid ABA problem
pub struct DefaultParticleSet {
    particles: Arena<Box<Particle>>,
    removed: Vec<DefaultParticleHandle>,
}

impl DefaultParticleSet {
    // Creates an empty set
    pub fn new() -> Self {
        DefaultParticleSet {
            particles: Arena::new(),
            removed: Vec::new(),
        }
    }

    // Adds a particle to this set
    pub fn insert(&mut self, particle: Particle) -> DefaultParticleHandle {
        self.particles.insert(Box::new(particle))
    }

    // Removes a particle from this set
    pub fn remove(&mut self, particle_handle: DefaultParticleHandle) -> Option<Box<Particle>> {
        let result = self.particles.remove(particle_handle)?;
        self.removed.push(particle_handle);
        Some(result)
    }
}

impl Default for DefaultParticleSet {
    fn default() -> Self {
        Self::new()
    }
}

pub type DefaultParticleHandle = generational_arena::Index;
