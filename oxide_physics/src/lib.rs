pub mod particle;
pub mod pfgen;

pub mod shape;

pub mod collide_broad_phase;
pub mod aabb;
mod aabb_ball;
mod aabb_cuboid;

#[cfg(test)]
mod tests {
    #[test]
    fn aabb_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
