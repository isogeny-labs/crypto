use std::{
    fs::{self, OpenOptions},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn newfile(p: &str) -> fs::File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap()
}
fn main() {
    let label = std::env::args().nth(1).expect("positive|target");
    assert!(label == "positive" || label == "target" || label == "preflight");
    let manifest = fs::canonicalize("solver-runtime.sha256").unwrap();
    assert!(Command::new("shasum")
        .current_dir("../..")
        .args(["-a", "256", "-c"])
        .arg(manifest)
        .status()
        .unwrap()
        .success());
    if label == "preflight" {
        println!("PREFLIGHT_PASS no_solver_spawned=true");
        return;
    }
    let input = format!("generated/{label}.ms");
    let result = format!("run-{label}.res");
    assert!(!std::path::Path::new(&result).exists());
    let mut child = Command::new("./deps/msolve/msolve")
        .args(["-t", "2", "-v", "1", "-c", "0", "-f", &input, "-o", &result])
        .stdout(Stdio::from(newfile(&format!("run-{label}.stdout"))))
        .stderr(Stdio::from(newfile(&format!("run-{label}.stderr"))))
        .spawn()
        .unwrap();
    let start = Instant::now();
    let mut peak = 0u64;
    let (reason, success) = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break (format!("exit={:?}", s.code()), s.success());
        }
        let rss = Command::new("ps")
            .args(["-o", "rss=", "-p", &child.id().to_string()])
            .output()
            .ok()
            .and_then(|x| String::from_utf8(x.stdout).ok())
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        peak = peak.max(rss);
        if start.elapsed().as_secs() >= 300 || rss > 2097152 {
            child.kill().unwrap();
            child.wait().unwrap();
            break (
                if rss > 2097152 {
                    "censored_rss".into()
                } else {
                    "censored_wall".into()
                },
                false,
            );
        }
        std::thread::sleep(Duration::from_millis(250));
    };
    let status = format!(
        "{reason} peak_rss_kib={peak} wall_ns={}\n",
        start.elapsed().as_nanos()
    );
    fs::write(format!("run-{label}.status"), &status).unwrap();
    println!("SOLVER {label} {status}");
    if !success {
        std::process::exit(2)
    }
    let control = if label == "positive" {
        "generated/control-point.txt"
    } else {
        "-"
    };
    let s = Command::new("./tmp/verify")
        .args([&input, &result, control])
        .stdout(Stdio::from(newfile(&format!("run-{label}.verify.stdout"))))
        .stderr(Stdio::from(newfile(&format!("run-{label}.verify.stderr"))))
        .status()
        .unwrap();
    assert!(s.success(), "independent parametrization check failed");
    println!("VERIFIED {label} exact_polynomial_substitution=true");
}
