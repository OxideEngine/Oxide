use crate::fbx::lexer;
use std::path::Path;

#[test]
fn foobar() {
    let _lexer = lexer::Lexer::new(Path::new("testdata/Cube.fbx"));
}
