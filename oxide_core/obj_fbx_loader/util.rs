use std::fmt::Write;

pub fn join(a: &[&str]) -> String {
    a.iter().fold(String::new(), |mut s, &n| {
        write!(s, "{}", n).ok();
        s
    })
}
