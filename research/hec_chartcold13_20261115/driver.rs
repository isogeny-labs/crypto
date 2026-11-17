use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn file(p: PathBuf) -> fs::File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap()
}
fn execute(mut c: Command, root: &Path, label: &str) -> std::process::ExitStatus {
    c.current_dir(root)
        .stdout(Stdio::from(file(root.join(format!("{label}.stdout")))))
        .stderr(Stdio::from(file(root.join(format!("{label}.stderr")))));
    let mut child = c.spawn().unwrap();
    let start = Instant::now();
    let s = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break s;
        }
        if start.elapsed().as_secs() >= 600 {
            child.kill().unwrap();
            child.wait().unwrap();
            fs::write(root.join(format!("{label}.status")), "censored_wall600s\n").unwrap();
            panic!("stage_wall_cap label={label}")
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    fs::write(
        root.join(format!("{label}.status")),
        format!("exit={:?}\n", s.code()),
    )
    .unwrap();
    s
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let input = fs::canonicalize(&a[1]).unwrap();
    let root = Path::new(&a[2]);
    assert!(!root.exists());
    assert!(!Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("KERNELS.txt")
        .exists());
    assert!(!Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ALPHA.txt")
        .exists());
    fs::create_dir(root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    let bindir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let manifest = std::env::var("HEC_SOLVER_MANIFEST").expect("HEC_SOLVER_MANIFEST is mandatory");
    assert!(Command::new("shasum")
        .args(["-a", "256", "-c", &manifest])
        .status()
        .unwrap()
        .success());
    let setup = root.join("setup");
    let mut neg = Command::new(bindir.join("fiberalign13"));
    neg.arg("negative-loader").env_remove("HEC_SETUP_DIR");
    let s = execute(neg, &root, "NEGATIVE-LOADER");
    assert!(!s.success());
    let err = fs::read_to_string(root.join("NEGATIVE-LOADER.stderr")).unwrap();
    assert!(err.contains("HEC_SETUP_DIR is mandatory"));
    println!("LOADER_CONTROL missing_setup_rejected=true fallback_absent=true");
    let mut prep = Command::new(bindir.join("coldprep13"));
    prep.arg(&input).arg(&setup);
    assert!(
        execute(prep, &root, "COLDPREP").success(),
        "fresh_cold_setup"
    );
    assert!(setup.join("KERNELS.txt").is_file() && setup.join("ALPHA.txt").is_file());
    for (label, bin, arg) in [
        ("ALIGN", "fiberalign13", Some("generated")),
        ("SOLVE", "collect13", None),
        ("RECOVER", "recover13", None),
        ("DESCENT", "descent13", None),
    ] {
        let mut c = Command::new(bindir.join(bin));
        if let Some(arg) = arg {
            c.arg(arg);
        }
        c.env("HEC_SETUP_DIR", &setup)
            .env("HEC_PUBLIC_INPUT", &input);
        assert!(
            execute(c, &root, label).success(),
            "pipeline_stage_failed {label}"
        );
        println!("STAGE_PASS {label} fresh_setup=true");
    }
    let data = fs::read_to_string(root.join("DESCENT.stdout")).unwrap();
    let result = data
        .lines()
        .find(|l| l.starts_with("RECOVERED k="))
        .expect("source/destination verified answer missing");
    assert!(
        result.contains("source_verified=true") && result.contains("destination_verified=true")
    );
    println!("{result}");
    println!("PASS complete_cold_fiber_ic=true setup_constructed_in_run=true prior_kernel_alpha_loaded=false old_map_scans_called=false known_target_scalar_supplied=false single_correctness_execution=true timing_claim=false");
}
