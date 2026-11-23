use std::{env, fs, path::PathBuf};
fn text(path: &str) -> String {
    println!("cargo:rerun-if-changed={path}");
    fs::read_to_string(path)
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
        .join("\n")
}
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut lib = text("../transform/src/lib.rs");
    // Rc cache remains tuple field2; independent root flag is tuple field3.
    let anchor = "    std::rc::Rc<std::cell::RefCell<ntt::Tables<F::Elem>>>,";
    assert_eq!(lib.matches(anchor).count(), 1);
    lib = lib.replace(anchor, &format!("{anchor}\n    bool,"));
    let anchor = "Self(Arc::new(field), false, Default::default())";
    assert_eq!(lib.matches(anchor).count(), 1);
    lib = lib.replace(
        anchor,
        "Self(Arc::new(field), false, Default::default(), false)",
    );
    let anchor = "    pub fn with_ntt(mut self) -> Self {";
    assert_eq!(lib.matches(anchor).count(), 1);
    lib = lib.replace(
        anchor,
        &format!("    pub fn with_fibers(mut self) -> Self {{ self.3=true; self }}\n{anchor}"),
    );
    let anchor = "    type E = Encoded<F::Elem>;";
    assert_eq!(lib.matches(anchor).count(), 1);
    lib = lib.replace(
        anchor,
        &format!("{anchor}\n    fn root_shortcuts(&self) -> bool {{self.3}}"),
    );
    let path = PathBuf::from("../transform/src/ntt.rs")
        .canonicalize()
        .unwrap();
    lib = lib.replace("mod ntt;", &format!("#[path = {:?}] mod ntt;", path));
    fs::write(out.join("adapter.rs"), lib).unwrap();
    let mut main =
        text("../transform/src/main.rs").replace("vieta_transform_lab", "vieta_fiber_lab");
    let analysis = PathBuf::from("analysis.rs").canonicalize().unwrap();
    main = main.replace(
        "mod analysis;",
        &format!("#[path = {:?}] mod analysis;", analysis),
    );
    main = main.replace(
        "arm.starts_with(\"ntt\")",
        "arm.starts_with(\"ntt\") || arm.starts_with(\"fiber\")",
    );
    main = main.replace(
        "arm.starts_with(\"ntt-half\")",
        "arm.starts_with(\"ntt-half\") || arm.starts_with(\"fiber-half\")",
    );
    main = main.replace(
        "f.with_ntt()",
        "if arm.starts_with(\"fiber\") {f.with_ntt().with_fibers()}else{f.with_ntt()}",
    );
    let anchor = "        \"ntt-half-double\"";
    assert_eq!(main.matches(anchor).count(), 1);
    main = main.replace(
        anchor,
        &format!("{anchor}, \"fiber\", \"fiber-half\", \"fiber-half-aa\", \"fiber-half-double\""),
    );
    // The frozen NTT1 tests are reused; add the new root-enabled worker prefixes.
    main = main.replace(
        "for prefix in [\"ntt\", \"ntt-half\"]",
        "for prefix in [\"ntt\", \"ntt-half\", \"fiber-half\"]",
    );

    main = main.replace(
        "(17 * i + 1)",
        "(17 * (i + c[\"target_offset\"].as_u64().unwrap_or(0) as usize) + 1)",
    );
    main = main.replace(
        "(31 * i + 2)",
        "(31 * (i + c[\"target_offset\"].as_u64().unwrap_or(0) as usize) + 2)",
    );
    let anchor = "        let n = i.saturating_sub(warm);";
    assert_eq!(main.matches(anchor).count(), 1);
    main=main.replace(anchor,&format!("        if let Some(path) = c[\"forbidden_targets\"].as_str() {{\n            let prior: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();\n            assert!(!prior[\"targets\"].as_array().unwrap().contains(&target), \"frozen prior-input collision; retain input and stop, no resampling\");\n        }}\n{anchor}"));
    fs::write(out.join("worker.rs"), main).unwrap();
}
