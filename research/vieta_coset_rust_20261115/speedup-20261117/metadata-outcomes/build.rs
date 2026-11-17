use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=../metadata/src/main.rs");
    let mut s = fs::read_to_string("../metadata/src/main.rs")
        .unwrap()
        .lines()
        .map(|l| {
            if l.starts_with("//!") {
                l.replacen("//!", "//", 1)
            } else {
                l.into()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let anchor = "if status==\"succeeded\"{\"completed\"}else{\"not_completed\"}";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(
        anchor,
        "if status==\"succeeded\" || status==\"failed\"{\"completed\"}else{\"not_completed\"}",
    );
    let anchor = "    let mut files = vec![];";
    assert_eq!(s.matches(anchor).count(), 1);
    s = s.replace(anchor, &format!("    let mut terminal = 0usize;\n{anchor}"));
    let anchor = "            runs.push(json!";
    assert_eq!(s.matches(anchor).count(), 1);
    s=s.replace(anchor,&format!("            if status==\"succeeded\" || status==\"failed\" {{ terminal += 1; }}\n{anchor}"));
    s = s.replace(
        "\"completed\":success,\"not_completed\":failed",
        "\"completed\":terminal,\"not_completed\":actual-terminal",
    );
    s = s.replace("{}-EXPORT", "{}-EXPORT-OUTCOME-REPAIR");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("exporter.rs"),
        s,
    )
    .unwrap();
}
