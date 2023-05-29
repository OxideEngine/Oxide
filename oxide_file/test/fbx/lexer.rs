mod lexer {
    mod new {
        use crate::fbx::lexer;
        use std::{io, path::Path};

        #[test]
        fn it_should_accept_fbx() {
            let lexer = lexer::Lexer::new(Path::new("testdata/fbx/Cube.fbx"));

            match lexer {
                Ok(_) => {}
                Err(_) => assert!(false, "it should accept fbx file"),
            };
        }

        #[test]
        fn it_should_reject_malformed_file_magic() {
            let lexer = lexer::Lexer::new(Path::new("testdata/fbx/MalformedFileMagic.fbx"));

            match lexer {
                Ok(_) => assert!(false, "it should reject malformed fbx file"),
                Err(e) => assert_eq!(io::ErrorKind::InvalidData, e.kind()),
            };
        }

        #[test]
        fn it_should_reject_malformed_unknown_bytes() {
            let lexer = lexer::Lexer::new(Path::new("testdata/fbx/MalformedUnknownByte.fbx"));

            match lexer {
                Ok(_) => assert!(false, "it should reject malformed unknown byte fbx file"),
                Err(e) => assert_eq!(io::ErrorKind::InvalidData, e.kind()),
            }
        }

        #[test]
        fn it_should_reject_without_version_number() {
            let lexer = lexer::Lexer::new(Path::new("testdata/fbx/WithoutVersionNumber.fbx"));

            match lexer {
                Ok(_) => assert!(false, "it should reject fbx file without version number"),
                // Caveat: Read a file without a version field and check for EOF errors to confirm
                // that we are consuming the version field.
                Err(e) => assert_eq!(io::ErrorKind::UnexpectedEof, e.kind()),
            }
        }
    }
}
