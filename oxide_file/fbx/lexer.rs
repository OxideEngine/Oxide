use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::path::Path;

const FILE_MAGIC: [u8; 21] = [
    75,  // `K`
    97,  // `a`
    121, // `y`
    100, // `d`
    97,  // `a`
    114, // `r`
    97,  // `a`
    32,  // ` `
    70,  // `F`
    66,  // `B`
    88,  // `X`
    32,  // ` `
    66,  // `B`
    105, // `i`
    110, // `n`
    97,  // `a`
    114, // `r`
    121, // `y`
    32,  // ` `
    32,  // ` `
    00,
];

#[derive(Debug)]
pub struct Lexer {}

impl Lexer {
    pub fn new<P: AsRef<Path>>(target_filename: P) -> io::Result<Lexer> {
        let file = File::open(target_filename)?;
        let mut reader = BufReader::new(file);

        let mut buf = [0; 21];
        reader.read_exact(&mut buf)?;

        if FILE_MAGIC != buf {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unmatched FILE_MAGIC",
            ));
        }

        Ok(Lexer {})
    }
}
