mod original {
    include!("../fiber/build.rs");
    pub fn generate() {
        main();
    }
}
fn main() {
    println!("cargo:rerun-if-changed=../fiber/build.rs");
    original::generate();
    let path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("worker.rs");
    let mut s = std::fs::read_to_string(&path).unwrap();
    s = s.replace(
        "arm == \"lookup-fast\"",
        "arm == \"lookup-fast\" || arm == \"lookup-fiber\"",
    );
    s = s.replace(
        "if arm.starts_with(\"ntt\") || arm.starts_with(\"fiber\") {",
        "if arm.starts_with(\"ntt\") || arm.starts_with(\"fiber\") || arm == \"lookup-fiber\" {",
    );
    s = s.replace(
        "if arm.starts_with(\"fiber\") {",
        "if arm.starts_with(\"fiber\") || arm == \"lookup-fiber\" {",
    );
    let anchor = "        \"lookup-fast\",";
    assert_eq!(s.matches(anchor).count(), 2);
    s = s.replace(anchor, &format!("{anchor}\n        \"lookup-fiber\","));
    std::fs::write(path, s).unwrap();
}
