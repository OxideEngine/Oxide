use crate::widgets::content_drawer::ContentDrawer;


#[test]
fn test_content_drawer_list_location() {
    let content_drawer = ContentDrawer::new("./");
    let result = content_drawer.list_location();
    assert_eq!(result, vec!["Cargo.toml", "src"]);
}
