use std::fs::File;
use std::io::{self, Read};
use std::io::{BufReader, Seek};
use std::path::Path;
use std::string::FromUtf8Error;

use crate::debug_print;

use super::token::{self, Node, PrimitiveType, PropertyRecord};

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
    UnknownTypeCode(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
            _ => write!(f, "{}", self),
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
    version: u32,
}

impl Parser {
    pub fn read_property(&mut self) -> ParseResult<PropertyRecord> {
        let mut type_code_raw: [u8; 1] = [0; 1];
        self.reader.read_exact(&mut type_code_raw)?;

        let type_code_char = char::from(type_code_raw[0]);
        debug_print!(type_code_char);

        let property: PropertyRecord = match type_code_char {
            'Y' => {
                todo!()
            }
            'C' => {
                todo!()
            }
            'I' => {
                let mut buf: [u8; 4] = [0; 4];
                self.reader.read_exact(&mut buf)?;

                let data = i32::from_le_bytes(buf);
                debug_print!(data);

                PropertyRecord::Primitive(PrimitiveType::FourByteSignedInteger(
                    token::FourByteSignedInteger { data },
                ))
            }
            'F' => {
                todo!()
            }
            'D' => {
                todo!()
            }
            'L' => {
                todo!()
            }
            'f' => {
                todo!()
            }
            'd' => {
                todo!()
            }
            'l' => {
                todo!()
            }
            'i' => {
                todo!()
            }
            'b' => {
                todo!()
            }
            'S' => {
                todo!()
            }
            'R' => {
                todo!()
            }
            _ => {
                if type_code_raw[0] != 0x0a {
                    let mut buf: [u8; 3] = [0; 3];
                    self.reader.read_exact(&mut buf)?;
                }

                PropertyRecord::Primitive(PrimitiveType::None)
            }
        };

        Ok(property)
    }

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

        debug_print!(property_list_len);
        debug_print!(name);

        let mut properties = vec![];
        for _ in 0..property_list_len {
            properties.push(self.read_property()?);
        }

        let mut nested_list = vec![];
        while (self.reader.stream_position()? as usize) < end_offset - 13 {
            nested_list.push(self.read_node()?);
        }

        if self.version < 7500 {
            let mut buf: [u8; 13] = [0; 13];
            self.reader.read_exact(&mut buf)?;
            debug_print!(buf);
        } else {
            let mut buf: [u8; 25] = [0; 25];
            self.reader.read_exact(&mut buf)?;
            debug_print!(buf);
        }

        debug_print!(nested_list);

        Ok(Node {
            end_offset,
            num_properties,
            property_list_len,
            name_len,
            name: name.to_string(),
            properties,
            nested_list,
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

        let mut version_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);

        Ok(Parser { reader, version })
    }
}
