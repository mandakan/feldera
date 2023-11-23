// Example custom build script.
fn main() {
    // copy assets to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
}
