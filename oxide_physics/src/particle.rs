#![allow(dead_code)]
#![allow(unused_variables)]

use oxide_math::commons::vector::*;
use oxide_math::commons::vector3::Vector3;
use num_traits::pow;

struct Particle {
    inverse_mass: f32,
    damping: f32,
    position: Vector3,
    velocity: Vector3,
    force_accum: Vector3,
    acceleration: Vector3,
}

impl Particle {
    fn integrate(&mut self, duration: f32) {
        // not to integrate things with zero mass
        if self.inverse_mass <= 0.0f32 {
            return ;
        }
        assert!(duration > 0.0);

        // update linear position
        self.position.x += self.velocity.scale(duration).x;
        self.position.y += self.velocity.scale(duration).y;
        self.position.z += self.velocity.scale(duration).z;

        // work out the acceleration from the force
        let mut resulting_acc = Vector3 {
            x: self.acceleration.x,
            y: self.acceleration.y,
            z: self.acceleration.z,
        };
        resulting_acc = resulting_acc + self.force_accum.scale(self.inverse_mass);

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

    fn get_mass(&mut self) -> f32 {
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

    fn has_finite_mass(&mut self) -> bool {
        self.inverse_mass >= 0.0f32
    }

    fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
    }

    fn get_damping(&mut self) -> f32 {
        self.damping
    }

    fn set_position(&mut self, position: &Vector3) {
        self.position.x = position.x;
        self.position.y = position.y;
        self.position.z = position.z;
    }

    fn set_position_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    fn get_position(&mut self) -> &Vector3 {
       &self.position
    }

    fn set_velocity(&mut self, velocity: &Vector3) {
        self.velocity.x = velocity.x;
        self.velocity.y = velocity.y;
        self.velocity.z = velocity.z;
    }

    fn set_velocity_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
    }

    fn get_velocity_into_vec(&mut self, velocity: &mut Vector3) {
        velocity.x = self.velocity.x;
        velocity.y = self.velocity.y;
        velocity.z = self.velocity.z;
    }

    fn get_velocity(&mut self) -> &Vector3 {
        &self.velocity
    }

    fn set_acceleration(&mut self, acceleration: &Vector3) {
        self.acceleration.x = acceleration.x;
        self.acceleration.y = acceleration.y;
        self.acceleration.z = acceleration.z;
    }

    fn set_acceleration_by_coord(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration.x = x;
        self.acceleration.y = y;
        self.acceleration.z = z;
    }

    fn get_acceleration_into_vec(&mut self, acceleration: &mut Vector3) {
        acceleration.x = self.acceleration.x;
        acceleration.y = self.acceleration.y;
        acceleration.z = self.acceleration.z;
    }

    fn get_acceleration(&mut self) -> &Vector3 {
        &self.acceleration
    }

    fn clear_accumulator(&mut self) {
        self.force_accum.x = 0.0f32;
        self.force_accum.y = 0.0f32;
        self.force_accum.z = 0.0f32;
    }

    fn add_force(&mut self, force: &Vector3) {
        self.force_accum.x += force.x;
        self.force_accum.y += force.y;
        self.force_accum.z += force.z;
    }
}
