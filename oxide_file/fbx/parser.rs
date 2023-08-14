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

const UNKNOWN_BYTES: [u8; 2] = [0x1A, 0x00];

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn new<P: AsRef<Path>>(target_filename: P) -> io::Result<Parser> {
        let file = File::open(target_filename)?;
        let mut reader = BufReader::new(file);

        let mut file_magic = [0; 21];
        reader.read_exact(&mut file_magic)?;

        if FILE_MAGIC != file_magic {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unmatched FILE_MAGIC",
            ));
        }

        let mut unknown_bytes = [0; 2];
        reader.read_exact(&mut unknown_bytes)?;

        if UNKNOWN_BYTES != unknown_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unmatched UNKNOWN_BYTES",
            ));
        }

        let mut version = [0; 4];
        // NOTE: Just consume because we do not care about what version of FBX it is currently.
        reader.read_exact(&mut version)?;

        Ok(Parser {})
    }
}
