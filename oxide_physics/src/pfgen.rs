use crate::particle::*;
use oxide_math::commons::vector::*;
use oxide_math::commons::vector3::Vector3;
use std::vec::Vec;

extern crate generational_arena;
use generational_arena::Arena;

/*
 * force generator trait
 */
pub trait ParticleForceGenerator {
    /*
     * interface to calculate and update the force
     * to the given particle
     */
    fn update_force(&self, particle: &mut Particle, duration: f32);
}

/*
 * Registry for (Particle, Force Generator) pairs
 * Holds all particles and associated force generators
 */
struct ParticleForceRegistration {
    particle: DefaultParticleHandle,
    fg: DefaultForceGeneratorHandle,
}

pub type DefaultForceGeneratorHandle = generational_arena::Index;

pub struct ParticleForceRegistry {
    registrations: Vec<ParticleForceRegistration>,
}

impl ParticleForceRegistry {
    pub fn add(&self, particle: DefaultParticleHandle, fg: DefaultForceGeneratorHandle) {
        /* NOT implemented */
    }

    pub fn remove(&self, particle: DefaultParticleHandle, fg: DefaultForceGeneratorHandle) {
        /* NOT implemented */
    }

    pub fn clear(&self) {
        /* NOT implemented */
    }

    pub fn update_forces(&self, duration: f32) {
        // for i in self.registrations.iter_mut() {
        //     i.fg.update_force(i.particle, duration);
        // }
    }
}

/*
 * Gravity Force Generator
 */
struct ParticleGravity {
    gravity: Vector3,
}

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&self, particle: &mut Particle, duration: f32) {
        if particle.has_finite_mass() {
            particle.add_force(&self.gravity.scale(particle.get_mass()));
        }
    }
}

/*
 * Drag Force Generator
 */
struct ParticleDrag {
    k1: f32,
    k2: f32,
}

impl ParticleForceGenerator for ParticleDrag {
    fn update_force(&self, particle: &mut Particle, duration: f32) {
        let force: &Vector3 = &particle.velocity;

        // Calculate the total drag coefficient
        let drag_coeff: f32 =
            self.k1 * force.get_length() + self.k2 * force.get_length() * force.get_length();

        // Calculate the final force and apply it
        let final_force = force.normalize().scale(-drag_coeff);
        particle.add_force(&final_force);
    }
}
