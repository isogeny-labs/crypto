mod original {
    include!("../fiber/build.rs");
    pub fn generate() {
        main();
    }
}
fn main() {
    println!("cargo:rerun-if-changed=../fiber/build.rs");
    original::generate();
}
