fn main() {
    slint_build::compile("ui/mainwindow.slint").unwrap();
    embed_resource::compile("src/icon.rc", &[] as &[&str]);
}