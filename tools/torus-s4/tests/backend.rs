//! Optional native-backend correctness control, never a performance sample.
use std::{env, fs, path::Path, process::Command};
use torus_s4::{sha256, Case, System};
#[test]
#[ignore = "requires native Singular, GNU timeout/prlimit and a fresh evidence directory"]
fn singular_actual_field_m1_controls() {
    let engine = env::var("TORUS_SINGULAR").expect("TORUS_SINGULAR");
    let output = env::var("TORUS_BACKEND_LOG_DIR").expect("TORUS_BACKEND_LOG_DIR");
    let output = Path::new(&output);
    fs::create_dir(output).unwrap();
    let mut inputs = vec![];
    for name in ["M1_planted", "M1_nonmember"] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!("{name}.json"));
        let bytes = fs::read(path).unwrap();
        let case: Case = serde_json::from_slice(&bytes).unwrap();
        let system = System::compile(&case).unwrap();
        let script = system.singular();
        let path = output.join(format!("{name}.sing"));
        fs::write(&path, &script).unwrap();
        inputs.push((name, path, sha256(script.as_bytes())));
    }
    fs::write(output.join("backend-pre-call.json"),serde_json::to_vec_pretty(&serde_json::json!({"purpose":"M1 functional correctness only","engine_sha256":sha256(&fs::read(&engine).unwrap()),"inputs":inputs.iter().map(|(n,_,h)|serde_json::json!({"case":n,"script_sha256":h})).collect::<Vec<_>>(),"per_process_cpu_seconds":4,"wall_seconds":5,"address_space_bytes":805306368u64})).unwrap()).unwrap();
    for (name, path, _) in inputs {
        let result = Command::new("prlimit")
            .args([
                "--as=805306368",
                "--cpu=4:4",
                "--",
                "timeout",
                "--signal=KILL",
                "5",
            ])
            .arg(&engine)
            .arg("-q")
            .arg(path)
            .output()
            .unwrap();
        fs::write(output.join(format!("{name}.stdout")), &result.stdout).unwrap();
        fs::write(output.join(format!("{name}.stderr")), &result.stderr).unwrap();
        assert!(
            result.status.success(),
            "backend control failed: {name}: {}",
            result.status
        );
        let text = String::from_utf8(result.stdout).unwrap();
        assert!(text.contains("TORUS_NATIVE_DONE"));
        assert!(!text.contains("   ?"), "Singular error: {text}");
        assert!(
            text.contains(if name == "M1_planted" {
                "TORUS_NONUNIT_BASIS"
            } else {
                "TORUS_UNIT_IDEAL"
            }),
            "wrong M1 control classification: {text}"
        );
    }
}
