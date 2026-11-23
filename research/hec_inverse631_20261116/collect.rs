use std::{
    fs::{self, OpenOptions},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn file(p: &str) -> fs::File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap()
}
fn main() {
    assert!(Command::new("shasum")
        .args([
            "-a",
            "256",
            "-c",
            &std::env::var("HEC_SOLVER_MANIFEST").expect("HEC_SOLVER_MANIFEST is mandatory")
        ])
        .status()
        .unwrap()
        .success());
    for label in ["positive", "D1", "B"] {
        let input = if label == "positive" {
            "generated-D1/positive.ms".to_string()
        } else {
            format!("generated-{label}/target.ms")
        };
        let output = format!("solve-{label}.res");
        assert!(!std::path::Path::new(&output).exists());
        let mut child = Command::new(std::env::var("HEC_MSOLVE").expect("HEC_MSOLVE is mandatory"))
            .args(["-t", "2", "-v", "1", "-c", "0", "-f", &input, "-o", &output])
            .stdout(Stdio::from(file(&format!("solve-{label}.stdout"))))
            .stderr(Stdio::from(file(&format!("solve-{label}.stderr"))))
            .spawn()
            .unwrap();
        let start = Instant::now();
        let status = loop {
            if let Some(s) = child.try_wait().unwrap() {
                break s;
            }
            if start.elapsed().as_secs() >= 300 {
                child.kill().unwrap();
                child.wait().unwrap();
                fs::write(format!("solve-{label}.status"), "censored_wall300s\n").unwrap();
                panic!("solver_wall_cap")
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        fs::write(
            format!("solve-{label}.status"),
            format!("exit={:?}\n", status.code()),
        )
        .unwrap();
        assert!(status.success(), "solver_failed");
        let control = if label == "positive" {
            "generated-D1/control-point.txt"
        } else {
            "-"
        };
        let s = Command::new(std::env::current_exe().unwrap().with_file_name("verify631"))
            .args([&input, &output, control])
            .stdout(Stdio::from(file(&format!("solve-{label}.verify.stdout"))))
            .stderr(Stdio::from(file(&format!("solve-{label}.verify.stderr"))))
            .status()
            .unwrap();
        assert!(s.success(), "independent_substitution_failed");
        println!("SOLVE_VERIFIED label={label} exact12equations=true");
    }
    println!("PASS fresh_base_inverse_systems_solved=true positive=1 target=2 class_recovery_not_yet_claimed=true timing_claim=false");
}
