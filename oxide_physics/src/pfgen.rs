
use std::vec::Vec;
use crate::particle::*;
use oxide_math::commons::vector::*;
use oxide_math::commons::vector3::Vector3;

type Real = f32;

/*
 * force generator trait
 */
pub trait ParticleForceGenerator {
    /*
     * interface to calculate and update the force
     * to the given particle
     */
    fn update_force(&self, particle: &mut Particle, duration: Real);
}


/*
 * Registry for (Particle, Force Generator) pairs
 */
struct ParticleForceRegistration<'a> {
    particle: &'a mut Particle,
    fg: &'a dyn ParticleForceGenerator,
}

struct ParticleForceRegistry<'a> {
    registrations: Vec<ParticleForceRegistration<'a>>,
}

impl ParticleForceRegistry<'_> {
    pub fn add(&self, particle: &Particle, fg: &dyn ParticleForceGenerator) {
        /* TBD */
    }
    
    pub fn remove(&self, particle: &Particle, fg: &dyn ParticleForceGenerator) {
        /* TBD */
    }

    pub fn clear(&self) {
        /* TBD */
    }

    pub fn update_forces(&self, duration: Real) {
        /*
         * TBD
        let reg_iter = self.registrations.iter();

        for i in reg_iter {
            i.fg.update_force(i.particle, duration);
        }
        */
    }
}


/*
 * Gravity Force Generator
 */
struct ParticleGravity {
    gravity: Vector3,
}

impl ParticleGravity {
    pub fn particle_gravity(&self, gravity: &Vector3) {
        /* TBD */
    }
}

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&self, particle: &mut Particle, duration: Real) {
        if !particle.has_finite_mass() {
            return ;
        }
        particle.add_force(&self.gravity.scale(particle.get_mass()));
    }
}

/*
 * Drag Force Generator
 */
struct ParticleDrag {
    k1: Real,
    k2: Real,
}

impl ParticleDrag {
    pub fn particle_drag(&self, k1: Real, k2:Real) {
        /* TBD */
    }
}

impl ParticleForceGenerator for ParticleDrag {
    fn update_force(&self, particle: &mut Particle, duration: Real) {
        /* TBD */
    }
}
