macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !(($x - $y).abs() < $d) {
            panic!();
        }
    };
}
