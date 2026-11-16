//! Exercise the actual standalone structural CLI without invoking Cargo at runtime.
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "factor-base-profile-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn call(t: &Temp, command: &str, input: &str, output: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_factor-base-sweep"))
        .current_dir(&t.0)
        .env("PATH", "/usr/bin:/bin")
        .args([command, "--input", input, "--out", output])
        .output()
        .unwrap()
}
fn read(t: &Temp, name: &str) -> Value {
    serde_json::from_slice(&fs::read(t.0.join(name)).unwrap()).unwrap()
}
#[test]
fn profile_works_outside_checkout_without_cargo_and_never_overwrites() {
    let t = Temp::new();
    let request = json!({"schema":"factor_base_profile_request/v1","field_degree":3,
        "modulus_low_terms":[0,1],"basis_hex":["1","2"],"max_power":3});
    fs::write(
        t.0.join("input.json"),
        serde_json::to_vec(&request).unwrap(),
    )
    .unwrap();
    let first = call(&t, "profile", "input.json", "out.json");
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let report = read(&t, "out.json");
    assert_eq!(report["product_dimensions"], json!([2, 3, 3]));
    assert_eq!(report["linearization_excess_m2"], -2);
    assert!(report["certified_degree_of_regularity"].is_null());
    assert!(report["predicted_solving_degree"].is_null());
    assert!(report["total_cost_winner"].is_null());
    assert!(String::from_utf8_lossy(&first.stderr).contains("power=3"));
    let frozen = fs::read(t.0.join("out.json")).unwrap();
    assert!(!call(&t, "profile", "input.json", "out.json")
        .status
        .success());
    assert_eq!(frozen, fs::read(t.0.join("out.json")).unwrap());
    let mut invalid = request;
    invalid["modulus_low_terms"] = json!([0]);
    fs::write(t.0.join("bad.json"), serde_json::to_vec(&invalid).unwrap()).unwrap();
    assert!(!call(&t, "profile", "bad.json", "bad-out.json")
        .status
        .success());
    assert!(!t.0.join("bad-out.json").exists());
}
#[test]
fn construction_retains_derivation_and_rejects_impossible_dimension() {
    let t = Temp::new();
    let request = json!({"schema":"factor_base_construct_request/v1","field_degree":3,
        "modulus_low_terms":[0,1],"dimension":2,"generator_hex":"0x2","seed":42,"max_power":3});
    fs::write(
        t.0.join("input.json"),
        serde_json::to_vec(&request).unwrap(),
    )
    .unwrap();
    let first = call(&t, "construct", "input.json", "one.json");
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(call(&t, "construct", "input.json", "two.json")
        .status
        .success());
    assert_eq!(read(&t, "one.json"), read(&t, "two.json"));
    let a = read(&t, "one.json");
    assert_eq!(a["constraint_rank"], 2);
    assert_eq!(a["kernel_basis_hex"].as_array().unwrap().len(), 1);
    assert_ne!(a["scale_hex"], "0x0");
    assert!(a["selector_hex"].is_string());
    assert!(a["profile"]["total_cost_winner"].is_null());
    let mut bad = request;
    bad["dimension"] = json!(3);
    fs::write(t.0.join("bad.json"), serde_json::to_vec(&bad).unwrap()).unwrap();
    assert!(!call(&t, "construct", "bad.json", "bad-out.json")
        .status
        .success());
    assert!(!t.0.join("bad-out.json").exists());
}
