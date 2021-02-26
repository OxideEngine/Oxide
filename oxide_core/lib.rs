pub mod vector;
pub mod vector2;

#[cfg(test)]
mod test {
    #[test]
    fn test_add() {
        use super::*;
        assert_eq!(add(3, 4), 7);
    }
}
