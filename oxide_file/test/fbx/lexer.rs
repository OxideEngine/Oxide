mod Lexer {
    mod new {
        use crate::fbx::lexer;
        use std::{io, path::Path};

        #[test]
        fn it_should_reject_non_fbx() {
            let lexer = lexer::Lexer::new(Path::new("testdata/Malformed.fbx"));

            match lexer {
                Ok(_) => assert!(false, "it should reject malformed fbx file"),
                Err(e) => assert_eq!(io::ErrorKind::InvalidData, e.kind()),
            };
        }

        #[test]
        fn it_should_accept_fbx() {
            let lexer = lexer::Lexer::new(Path::new("testdata/Cube.fbx"));

            match lexer {
                Ok(_) => {}
                Err(e) => assert!(false, "it should accept fbx file"),
            };
        }
    }
}
