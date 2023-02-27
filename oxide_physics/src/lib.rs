pub mod particle;
pub mod pfgen;
pub mod collide_broad_phase;

#[cfg(test)]
mod tests {
    #[test]
    fn aabb_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
