use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::path::Path;

pub struct Lexer {
    reader: BufReader<File>,
}

impl Lexer {
    pub fn new<P: AsRef<Path>>(target_filename: P) -> io::Result<Lexer> {
        let file = File::open(target_filename)?;
        let mut reader = BufReader::new(file);

        let mut buf = [0; 20];
        reader.read_exact(&mut buf)?;

        println!("{:?}", buf);

        Ok(Lexer { reader })
    }
}
