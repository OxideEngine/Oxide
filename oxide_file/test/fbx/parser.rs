mod parser {
    mod new {
        use crate::fbx::parser;
        use std::{io, path::Path};

        #[test]
        fn it_should_accept_fbx() {
            let parser = parser::Parser::new(Path::new("testdata/fbx/Cube.fbx"));

            match parser {
                Ok(_) => {}
                Err(_) => assert!(false, "it should accept fbx file"),
            };
        }

        #[test]
        fn it_should_reject_malformed_file_magic() {
            let parser = parser::Parser::new(Path::new("testdata/fbx/MalformedFileMagic.fbx"));

            match parser {
                Ok(_) => assert!(false, "it should reject malformed fbx file"),
                Err(parser::ParseError::Io(e)) => assert_eq!(io::ErrorKind::InvalidData, e.kind()),
                Err(_) => assert!(false, "unexpected error"),
            };
        }

        #[test]
        fn it_should_reject_malformed_unknown_bytes() {
            let parser = parser::Parser::new(Path::new("testdata/fbx/MalformedUnknownByte.fbx"));

            match parser {
                Ok(_) => assert!(false, "it should reject malformed unknown byte fbx file"),
                Err(parser::ParseError::Io(e)) => assert_eq!(io::ErrorKind::InvalidData, e.kind()),
                Err(_) => assert!(false, "unexpected error"),
            }
        }

        #[test]
        fn it_should_reject_without_version_number() {
            let parser = parser::Parser::new(Path::new("testdata/fbx/WithoutVersionNumber.fbx"));

            match parser {
                Ok(_) => assert!(false, "it should reject fbx file without version number"),
                // Caveat: Read a file without a version field and check for EOF errors to confirm
                // that we are consuming the version field.
                Err(parser::ParseError::Io(e)) => {
                    assert_eq!(io::ErrorKind::UnexpectedEof, e.kind())
                }
                Err(_) => assert!(false, "unexpected error"),
            }
        }

        #[test]
        fn it_should_read_node() {
            let mut parser = parser::Parser::new(Path::new("testdata/fbx/Cube.fbx")).unwrap();

            parser.read_node().unwrap();
        }
    }
}
