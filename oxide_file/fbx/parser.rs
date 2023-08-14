use std::any::type_name;
use std::array::TryFromSliceError;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::string::FromUtf8Error;

use crate::{debug_print, util};

use super::token::Node;

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
pub enum ParseError {
    Io(std::io::Error),
    FromUtf8(FromUtf8Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;
fn to_parse_result<T, E>(value: Result<T, E>) -> ParseResult<T>
where
    ParseError: From<E>,
{
    match value {
        Ok(v) => Ok(v),
        Err(e) => Err(ParseError::from(e)),
    }
}

#[derive(Debug)]
pub struct Parser {
    reader: BufReader<File>,
}

impl Parser {
    pub fn read_node(&mut self) -> ParseResult<Node> {
        let mut end_offset_bytes: [u8; 4] = [0; 4];
        self.reader.read_exact(&mut end_offset_bytes)?;

        let end_offset = u32::from_le_bytes(end_offset_bytes) as usize;

        let mut num_properties_bytes: [u8; 4] = [0; 4];
        self.reader.read_exact(&mut num_properties_bytes)?;
        let num_properties = u32::from_le_bytes(num_properties_bytes);

        let mut property_list_len_bytes: [u8; 4] = [0; 4];
        self.reader.read_exact(&mut property_list_len_bytes)?;
        let property_list_len = u32::from_le_bytes(property_list_len_bytes);

        let mut name_len_bytes: [u8; 1] = [0; 1];
        self.reader.read_exact(&mut name_len_bytes)?;
        let name_len = name_len_bytes[0];

        let mut name_bytes: Vec<u8> = vec![0; name_len as usize];
        self.reader.read_exact(&mut name_bytes)?;
        let name = std::string::String::from_utf8(name_bytes)?;

        debug_print!(end_offset);
        debug_print!(num_properties);
        debug_print!(property_list_len);
        debug_print!(name);

        Ok(Node {
            end_offset,
            num_properties,
            property_list_len,
            name_len,
            name: name.to_string(),
            properties: todo!(),
            nested_list: todo!(),
        })
    }

    pub fn new<P: AsRef<Path>>(target_filename: P) -> ParseResult<Self> {
        let file = File::open(target_filename)?;
        let mut reader = BufReader::new(file);

        let mut file_magic: [u8; 21] = [0; 21];
        reader.read_exact(&mut file_magic)?;

        if FILE_MAGIC != file_magic {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "unmatched FILE_MAGIC").into());
        }

        let mut unknown_bytes: [u8; 2] = [0; 2];
        reader.read_exact(&mut unknown_bytes)?;

        if UNKNOWN_BYTES != unknown_bytes {
            return Err(
                io::Error::new(io::ErrorKind::InvalidData, "unmatched UNKNOWN_BYTES").into(),
            );
        }

        let mut version: [u8; 4] = [0; 4];
        // NOTE: Just consume because we do not care about what version of FBX it is currently.
        reader.read_exact(&mut version)?;

        Ok(Parser { reader })
    }
}
