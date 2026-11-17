//! The three findings of the 2026-11-17 pod runs, and the command line end to end.
//!   1. CPU count: cgroup v2 `cpu.max` and v1 quota are respected, not only the affinity mask.
//!   2. GPU memory roof: a `triad` class exists and the kernel's declared bytes are used.
//!   3. `gpu probe` names missing helper tools and reports ERR_NVGPUCTRPERM as
//!      "counters withheld by the driver", tier T-gpu-timing.
//!
//! The GPU tests run against stub `nvidia-smi`, `nvcc` and `ncu` programs: no GPU is needed.
use optlevel::cmd::{self, MeasureArgs, ScoreArgs};
use optlevel::host::{cgroup_cpu_quota, classify_perf_probe, usable_cpus};
use optlevel::util::{self, load_json, read_json};
use serde_json::{json, Value};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_optlevel");

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("optlevel-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}
fn put(root: &Path, rel: &str, text: &str) {
    let p = root.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, text).unwrap();
}
fn script(dir: &Path, name: &str, body: &str) {
    let p = dir.join(name);
    std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
}
fn run(args: &[&str], path: &Path, cache: &Path) -> Output {
    Command::new(BIN)
        .args(args)
        .env("PATH", path)
        .env("OPTLEVEL_CACHE", cache)
        .env_remove("OPTLEVEL_CPUS")
        .output()
        .unwrap()
}
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

// ---- 1. CPUs the process may really use ------------------------------------------
#[test]
fn cgroup_v2_quota_at_the_namespace_root() {
    // the GPU pod of 2026-11-17: 48 host CPUs in the affinity mask, 6 vCPUs allotted
    let root = tmp("v2root");
    put(&root, "proc/self/cgroup", "0::/\n");
    put(&root, "sys/fs/cgroup/cpu.max", "600000 100000\n");
    let q = cgroup_cpu_quota(&root).unwrap();
    assert_eq!(q.cores, 6.0);
    assert!(
        q.source.starts_with("cgroup v2 cpu.max \"600000 100000\""),
        "{}",
        q.source
    );
    let (n, why) = usable_cpus(48, Some(&q), None);
    assert_eq!(n, 6);
    assert!(
        why.contains("6.00 cores") && why.contains("48 CPUs"),
        "{why}"
    );
}

#[test]
fn cgroup_v2_tightest_ancestor_wins_and_fractions_round_down() {
    let root = tmp("v2nest");
    put(&root, "proc/self/cgroup", "0::/kubepods/pod1/c1\n");
    put(
        &root,
        "sys/fs/cgroup/kubepods/pod1/c1/cpu.max",
        "max 100000\n",
    );
    put(
        &root,
        "sys/fs/cgroup/kubepods/pod1/cpu.max",
        "250000 100000\n",
    );
    put(&root, "sys/fs/cgroup/kubepods/cpu.max", "800000 100000\n");
    let q = cgroup_cpu_quota(&root).unwrap();
    assert_eq!(q.cores, 2.5);
    assert_eq!(usable_cpus(16, Some(&q), None).0, 2);
    // a quota below one core still leaves one usable CPU
    assert_eq!(
        usable_cpus(
            16,
            Some(&optlevel::host::Quota {
                cores: 0.5,
                source: "x".into()
            }),
            None
        )
        .0,
        1
    );
}

#[test]
fn cgroup_v1_quota() {
    let root = tmp("v1");
    put(
        &root,
        "proc/self/cgroup",
        "9:name=systemd:/\n5:cpu,cpuacct:/docker/abc\n0::/\n",
    );
    put(
        &root,
        "sys/fs/cgroup/cpu,cpuacct/docker/abc/cpu.cfs_quota_us",
        "600000\n",
    );
    put(
        &root,
        "sys/fs/cgroup/cpu,cpuacct/docker/abc/cpu.cfs_period_us",
        "100000\n",
    );
    put(&root, "sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us", "-1\n");
    put(
        &root,
        "sys/fs/cgroup/cpu,cpuacct/cpu.cfs_period_us",
        "100000\n",
    );
    let q = cgroup_cpu_quota(&root).unwrap();
    assert_eq!(q.cores, 6.0);
    assert!(
        q.source
            .starts_with("cgroup v1 cpu.cfs_quota_us 600000 / cpu.cfs_period_us 100000"),
        "{}",
        q.source
    );
    assert_eq!(usable_cpus(48, Some(&q), None).0, 6);
}

#[test]
fn cgroup_v1_host_path_not_visible_in_the_container() {
    // /proc/self/cgroup shows the host's path, the mount shows the container's own group at its root
    let root = tmp("v1ns");
    put(&root, "proc/self/cgroup", "3:cpu:/docker/0123\n");
    put(&root, "sys/fs/cgroup/cpu/cpu.cfs_quota_us", "400000\n");
    put(&root, "sys/fs/cgroup/cpu/cpu.cfs_period_us", "100000\n");
    assert_eq!(cgroup_cpu_quota(&root).unwrap().cores, 4.0);
}

#[test]
fn no_quota_means_the_affinity_mask() {
    let root = tmp("none");
    put(&root, "proc/self/cgroup", "1:cpu:/\n0::/\n");
    put(&root, "sys/fs/cgroup/cpu/cpu.cfs_quota_us", "-1\n");
    put(&root, "sys/fs/cgroup/cpu/cpu.cfs_period_us", "100000\n");
    put(&root, "sys/fs/cgroup/cpu.max", "max 100000\n");
    assert_eq!(cgroup_cpu_quota(&root), None);
    assert_eq!(
        usable_cpus(8, None, None),
        (8, "affinity mask (8 CPUs)".to_string())
    );
    // a quota above the mask does not raise it
    let big = optlevel::host::Quota {
        cores: 64.0,
        source: "x".into(),
    };
    assert_eq!(usable_cpus(8, Some(&big), None).0, 8);
    assert_eq!(cgroup_cpu_quota(&tmp("empty")), None);
}

#[test]
fn declared_allotment_only_lowers() {
    assert_eq!(usable_cpus(48, None, Some(6)).0, 6);
    assert!(usable_cpus(48, None, Some(6)).1.contains("OPTLEVEL_CPUS=6"));
    assert_eq!(usable_cpus(4, None, Some(6)).0, 4);
    assert_eq!(usable_cpus(4, None, Some(0)).0, 4);
}

#[test]
fn probe_reports_affinity_quota_and_usable_cpus() {
    let d = tmp("probe");
    let out = Command::new(BIN)
        .arg("probe")
        .env("OPTLEVEL_CPUS", "1")
        .env("OPTLEVEL_CACHE", &d)
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
    let h = &doc["host"];
    assert_eq!(h["usable_cpus"], 1);
    assert_eq!(
        h["affinity_cpus"].as_u64().unwrap() as usize,
        h["allowed_cpus"].as_array().unwrap().len()
    );
    assert!(h["logical_cpus"].as_u64().unwrap() >= h["affinity_cpus"].as_u64().unwrap());
    for k in [
        "cpu_quota_cores",
        "cpu_quota_source",
        "usable_cpus_basis",
        "fingerprint",
    ] {
        assert!(h.get(k).is_some(), "probe lacks host.{k}");
    }
    assert!(doc["helper_tools"]["missing"].is_array());
    assert!(["T-pmu", "T-timing"].contains(&doc["cpu_tier"].as_str().unwrap()));
}

// ---- 2. GPU memory roof ------------------------------------------------------------
fn ops_line(cls: &str, ops: f64, secs: f64, bytes: f64) -> Value {
    json!({"schema": "optlevel.ops/v1", "op_class": cls, "ops": ops, "kernel_seconds": secs, "bytes_declared": bytes, "device": "stub"})
}
fn gpu_run(cls: &str, ops: f64, secs: f64, bytes: f64) -> Value {
    json!({"ops": ops, "ops_per_s": ops / secs, "kernel_seconds": secs, "wall_s": secs + 0.4, "cpu_s": secs, "bytes_declared": bytes, "op_class": cls,
           "ops_source": "optlevel.ops/v1 line", "exit_status": [0], "conditions": {"other_cpu_cores": 0.0}})
}

#[test]
fn gpu_calibration_has_a_triad_class_and_score_uses_declared_bytes() {
    let lines = [
        ops_line("muladd_u32", 4.0e11, 0.5, 0.0),
        ops_line("fma_f64", 6.0e10, 0.5, 0.0),
        ops_line("fma_f64", 5.8e10, 0.5, 0.0),
        ops_line("triad", 4.0e9, 0.5, 24.0 * 4.0e9),
        ops_line("triad", 3.9e9, 0.5, 24.0 * 3.9e9),
    ];
    let host = json!({"fingerprint": "abc", "cpu_model": "stub", "logical_cpus": 4});
    let cal = cmd::gpu_calibration_doc(&lines, host.clone(), json!({}));
    assert_eq!(cal["compute"]["triad"]["alias_of"], "fma_f64");
    assert_eq!(
        cal["compute"]["triad"]["total_ops_per_s"],
        cal["compute"]["fma_f64"]["total_ops_per_s"]
    );
    assert_eq!(
        cal["memory"]["bandwidth_bytes_per_s"]["1"]["max"]
            .as_f64()
            .unwrap(),
        24.0 * 4.0e9 / 0.5
    );
    // a streamed kernel at 95 % of the device bandwidth, measured with no --bytes-per-op and no --op-class
    let a = MeasureArgs {
        label: "stream".into(),
        repeat: 5,
        cmd: vec!["k".into()],
        ..Default::default()
    };
    let runs: Vec<Value> = (0..5)
        .map(|_| gpu_run("triad", 3.8e9, 0.5, 24.0 * 3.8e9))
        .collect();
    let meas =
        cmd::gpu_measurement_doc(&a, &runs, &runs, cmd::parse_smi_samples(""), host).unwrap();
    assert_eq!(meas["op_class"], "triad");
    assert_eq!(meas["declared_bytes_per_op"].as_f64().unwrap(), 24.0);
    assert_eq!(meas["tier"], "T-gpu-timing");
    let rec = cmd::score_documents(&cal, &meas, &ScoreArgs::default()).unwrap();
    assert_eq!(cmd::record_errors(&rec), Vec::<String>::new());
    assert_eq!(rec["roofline"]["single"]["binding"], "memory");
    assert_eq!(rec["intensity"]["source"], "declared");
    assert!((rec["attainment"]["single"]["value"].as_f64().unwrap() - 0.95).abs() < 1e-12);
    assert!(rec["grade"]["limiting_factor"]
        .as_str()
        .unwrap()
        .contains("memory-bandwidth roof"));
    // --bytes-per-op still overrides what the kernel declares
    let a2 = MeasureArgs {
        bytes_per_op: Some(48.0),
        ..a
    };
    assert_eq!(
        cmd::gpu_measurement_doc(&a2, &runs, &runs, cmd::parse_smi_samples(""), json!({})).unwrap()
            ["declared_bytes_per_op"],
        48.0
    );
    // arithmetic streams declare zero traffic: compute-bound, not "traffic unknown"
    let s: Vec<Value> = (0..5)
        .map(|_| gpu_run("fma_f64", 5.0e10, 0.5, 0.0))
        .collect();
    let m = cmd::gpu_measurement_doc(
        &MeasureArgs {
            repeat: 5,
            cmd: vec!["k".into()],
            ..Default::default()
        },
        &s,
        &s,
        cmd::parse_smi_samples(""),
        json!({"fingerprint": "abc", "cpu_model": "stub", "logical_cpus": 4}),
    )
    .unwrap();
    let r = cmd::score_documents(&cal, &m, &ScoreArgs::default()).unwrap();
    assert_eq!(r["roofline"]["single"]["binding"], "compute");
    assert!(!r["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "INTENSITY_UNKNOWN"));
}

#[test]
fn triad_scores_against_a_calibration_written_before_the_class_existed() {
    // the committed pod calibration has no triad class; on 2026-11-17 the triad had to be
    // relabelled fma_f64 with --bytes-per-op 24 by hand.  The class alias gives the same numbers.
    let dir = repo().join("research/hky_pod_runs_20261117/results");
    let cal = load_json(&dir.join("gpu3_gpu_cal.json")).unwrap();
    assert!(cal["compute"].get("triad").is_none());
    let mut meas = load_json(&dir.join("gpu3_triad.measure.json")).unwrap();
    let want = read_json(&dir.join("gpu3_triad.record.json")).unwrap();
    meas["op_class"] = json!("triad");
    let rec = cmd::score_documents(&cal, &meas, &ScoreArgs::default()).unwrap();
    assert_eq!(rec["attainment"], want["attainment"]);
    assert_eq!(rec["roofline"], want["roofline"]);
    assert_eq!(rec["kernel"]["op_class"], "triad");
    assert!(rec["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "CEILING_CLASS_ALIAS_FMA_F64"));
    // an unknown class is still refused, naming the classes there are
    meas["op_class"] = json!("clmul_u64");
    let e = cmd::score_documents(&cal, &meas, &ScoreArgs::default()).unwrap_err();
    assert!(
        e.msg.contains("no ceiling for operation class") && e.msg.contains("fma_f64"),
        "{}",
        e.msg
    );
}

// ---- 3. gpu probe -------------------------------------------------------------------
#[test]
fn nvgpuctrperm_is_counters_withheld_by_the_driver() {
    // the text ncu printed on the pod, as committed in the profile probe
    let probe =
        read_json(&repo().join("research/hky_pod_runs_20261117/results/gpu3_probe.json")).unwrap();
    let log = probe["capabilities"]["ncu_control_kernel"]["log_tail"]
        .as_str()
        .unwrap();
    assert!(log.contains("ERR_NVGPUCTRPERM"));
    let (status, text) = cmd::classify_ncu_output(log, true);
    assert_eq!(status, "withheld");
    assert!(
        text.starts_with("counters withheld by the driver (ERR_NVGPUCTRPERM)"),
        "{text}"
    );
    assert_eq!(
        cmd::classify_ncu_output(
            "==PROF== Connected to process 1\n==PROF== Disconnected from process 1\n",
            true
        )
        .0,
        "available"
    );
    let (status, text) =
        cmd::classify_ncu_output("==ERROR== Failed to prepare kernel for profiling\n", false);
    assert_eq!(status, "failed");
    assert!(text.contains("Failed to prepare"));
}

/// A directory of stub GPU tools.  `ncu_body` is what the stub ncu does.
fn stub_gpu(name: &str, ncu_body: Option<&str>) -> (PathBuf, PathBuf) {
    let d = tmp(name);
    let bin = d.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    script(
        &bin,
        "nvidia-smi",
        r#"case "$*" in
  *-lms*) while :; do echo "97, 41, 300.0, 72, 0x0000000000000000"; /bin/sleep 0.05; done ;;
  *) echo "Stub RTX, 16380 MiB, 595.112.07, 8.9" ;;
esac"#,
    );
    // "compiles" gpu_calib.cu into a script that prints what the real kernel prints
    script(
        &bin,
        "nvcc",
        r#"out=; while [ $# -gt 0 ]; do [ "$1" = -o ] && { out=$2; shift; }; shift; done
/bin/cat > "$out" <<'K'
#!/bin/sh
line() { echo "{\"schema\":\"optlevel.ops/v1\",\"op_class\":\"$1\",\"ops\":$2,\"kernel_seconds\":0.500000000,\"bytes_declared\":$3,\"device\":\"Stub RTX\"}"; }
[ -z "$1" ] || [ "$1" = muladd_u32 ] && line muladd_u32 200000000000 0
[ -z "$1" ] || [ "$1" = fma_f64 ] && line fma_f64 30000000000 0
[ -z "$1" ] || [ "$1" = triad ] && line triad 2000000000 48000000000
exit 0
K
/bin/chmod +x "$out""#,
    );
    if let Some(body) = ncu_body {
        script(&bin, "ncu", body);
    }
    (d, bin)
}

#[test]
fn gpu_probe_reports_missing_helpers_and_withheld_counters() {
    let (d, bin) = stub_gpu("gpuprobe", Some("echo '==PROF== Connected to process 7229'\necho '==ERROR== ERR_NVGPUCTRPERM - The user does not have permission to access NVIDIA GPU Performance Counters on the target device 0.'"));
    let out = run(&["gpu", "probe"], &bin, &d);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(doc["gpus"], json!(["Stub RTX, 16380 MiB, 595.112.07, 8.9"]));
    assert_eq!(doc["helper_tools"]["missing"], json!(["gawk", "jq"]));
    assert_eq!(doc["ncu_counters"]["status"], "withheld");
    assert!(doc["ncu_counters"]["detail"]
        .as_str()
        .unwrap()
        .starts_with("counters withheld by the driver"));
    assert_eq!(doc["gpu_tier"], "T-gpu-timing");
    // said up front, on stderr, not only inside the JSON
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("missing helper tools: gawk, jq"), "{err}");
    assert!(
        err.contains("counters withheld by the driver") && err.contains("T-gpu-timing"),
        "{err}"
    );

    // helpers present and counters readable: the ncu tier
    script(&bin, "gawk", "exit 0");
    script(&bin, "jq", "exit 0");
    script(
        &bin,
        "ncu",
        "echo '==PROF== Connected to process 1'; echo '==PROF== Disconnected from process 1'",
    );
    let doc: Value = serde_json::from_slice(&run(&["gpu", "probe"], &bin, &d).stdout).unwrap();
    assert_eq!(doc["helper_tools"]["missing"], json!([]));
    assert_eq!(
        (
            doc["ncu_counters"]["status"].as_str(),
            doc["gpu_tier"].as_str()
        ),
        (Some("available"), Some("T-gpu-ncu"))
    );
    // no ncu at all
    std::fs::remove_file(bin.join("ncu")).unwrap();
    let doc: Value = serde_json::from_slice(&run(&["gpu", "probe"], &bin, &d).stdout).unwrap();
    assert_eq!(
        (
            doc["ncu_counters"]["status"].as_str(),
            doc["gpu_tier"].as_str()
        ),
        (Some("absent"), Some("T-gpu-timing"))
    );
}

#[test]
fn gpu_path_end_to_end_on_stub_tools() {
    let (d, bin) = stub_gpu("gpue2e", None);
    let p = |n: &str| d.join(n).to_string_lossy().into_owned();
    let out = run(
        &["gpu", "calibrate", "--out", &p("cal.json"), "--repeat", "2"],
        &bin,
        &d,
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let cal = read_json(&d.join("cal.json")).unwrap();
    assert_eq!(cal["device"], "gpu");
    assert_eq!(cal["compute"]["triad"]["alias_of"], "fma_f64");
    assert_eq!(cal["compute"]["fma_f64"]["total_ops_per_s"]["1"]["n"], 2);
    assert_eq!(
        cal["memory"]["bandwidth_bytes_per_s"]["1"]["max"]
            .as_f64()
            .unwrap(),
        9.6e10
    );
    // the kernel the calibration built, one class at a time (the .cu file's optional argument)
    let kernel = std::fs::read_dir(&d)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("gpu_calib-")
                && p.extension().is_none()
        })
        .unwrap();
    let out = run(
        &[
            "gpu",
            "measure",
            "--out",
            &p("triad.measure.json"),
            "--label",
            "gpu-triad",
            "--repeat",
            "5",
            "--",
            &kernel.to_string_lossy(),
            "triad",
        ],
        &bin,
        &d,
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let meas = read_json(&d.join("triad.measure.json")).unwrap();
    assert_eq!(
        (
            meas["op_class"].as_str(),
            meas["declared_bytes_per_op"].as_f64(),
            meas["tier"].as_str()
        ),
        (Some("triad"), Some(24.0), Some("T-gpu-timing"))
    );
    assert_eq!(meas["noise_floor"]["pairs"], 5);
    assert!(meas["gpu"]["nvidia_smi"]["samples"].as_u64().unwrap() >= 1);
    let out = run(
        &[
            "score",
            "--calibration",
            &p("cal.json"),
            "--measurement",
            &p("triad.measure.json"),
            "--out",
            &p("triad.record.json"),
        ],
        &bin,
        &d,
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rec = read_json(&d.join("triad.record.json")).unwrap();
    assert_eq!(rec["roofline"]["single"]["binding"], "memory");
    assert_eq!(rec["attainment"]["single"]["value"].as_f64().unwrap(), 1.0);
    assert_eq!(rec["grade"]["letter"], "A");
    assert!(run(&["validate", &p("triad.record.json")], &bin, &d)
        .status
        .success());
    let out = run(
        &[
            "plot",
            "--calibration",
            &p("cal.json"),
            "--out",
            &p("gpu.svg"),
            &p("triad.record.json"),
        ],
        &bin,
        &d,
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(std::fs::read_to_string(d.join("gpu.svg"))
        .unwrap()
        .contains("gpu-triad 1t: 1.00"));
    // ncu figures lift the tier; a CPU record refuses them
    util::write_json(&d.join("kernels.json"), &json!({"launches": [{"kernel": "triad", "duration_ns": 5, "dram_throughput_pct": 91.0}, {"kernel": "fill", "duration_ns": 1}]})).unwrap();
    let out = run(
        &[
            "score",
            "--calibration",
            &p("cal.json"),
            "--measurement",
            &p("triad.measure.json"),
            "--ncu",
            &p("kernels.json"),
            "--out",
            &p("triad.ncu.record.json"),
        ],
        &bin,
        &d,
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rec = read_json(&d.join("triad.ncu.record.json")).unwrap();
    assert_eq!(
        (
            rec["tier"].as_str(),
            rec["diagnosis"]["gpu"]["ncu"]["kernel"].as_str(),
            rec["diagnosis"]["gpu"]["ncu"]["dram_throughput_pct"].as_f64()
        ),
        (Some("T-gpu-ncu"), Some("triad"), Some(91.0))
    );
    let summary = run(&["summary", &p("triad.record.json")], &bin, &d);
    let line: Value = serde_json::from_slice(&summary.stdout).unwrap();
    assert_eq!(
        (
            line["tier"].as_str(),
            line["grade"].as_str(),
            line["op_class"].as_str()
        ),
        (Some("T-gpu-timing"), Some("A"), Some("triad"))
    );
}

#[test]
fn gpu_path_refuses_without_a_gpu_and_says_why() {
    let d = tmp("nogpu");
    let empty = d.join("bin");
    std::fs::create_dir_all(&empty).unwrap();
    for args in [
        vec!["gpu", "probe"],
        vec!["gpu", "calibrate", "--out", "x.json"],
    ] {
        let out = Command::new(BIN)
            .args(&args)
            .current_dir(&d)
            .env("PATH", &empty)
            .env("OPTLEVEL_CACHE", &d)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(3), "{args:?}");
        assert!(String::from_utf8_lossy(&out.stderr)
            .contains("GPU path unsupported on this host: nvidia-smi is not installed"));
    }
    let doc = read_json(&d.join("x.json")).unwrap();
    assert_eq!(
        (doc["schema"].as_str(), doc["status"].as_str()),
        (Some("optlevel.unsupported/v1"), Some("unsupported"))
    );
    assert_eq!(doc["helper_tools"]["missing"], json!(["gawk", "jq"]));
}

// ---- the CPU command line, end to end, on the tool's own kernels ----------------------
#[test]
fn measure_score_validate_plot_on_a_real_kernel() {
    let d = tmp("cpu");
    let p = |n: &str| d.join(n).to_string_lossy().into_owned();
    let path = std::env::var_os("PATH").unwrap();
    let go = |args: &[&str]| {
        Command::new(BIN)
            .args(args)
            .env("PATH", &path)
            .env("OPTLEVEL_CACHE", &d)
            .output()
            .unwrap()
    };
    let out = go(&[
        "measure",
        "--out",
        &p("m.json"),
        "--label",
        "t",
        "--repeat",
        "2",
        "--timing-only",
        "--",
        BIN,
        "kernel",
        "modmuladd_p31",
        "--isa",
        "scalar",
        "--seconds",
        "0.05",
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let m = read_json(&d.join("m.json")).unwrap();
    assert_eq!(
        (
            m["schema"].as_str(),
            m["tier"].as_str(),
            m["op_class"].as_str()
        ),
        (
            Some("optlevel.measurement/v1"),
            Some("T-timing"),
            Some("modmuladd_p31")
        )
    );
    assert_eq!(
        (
            m["declared_bytes_per_op"].as_f64(),
            m["working_set_bytes"].as_i64(),
            m["noise_floor"]["pairs"].as_i64()
        ),
        (Some(12.0), Some(16384), Some(2))
    );
    assert!(m["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "TOO_SHORT"));
    assert_eq!(m["runs"].as_array().unwrap().len(), 4);
    assert!(m["points"]["1"]["ops_per_s"]["median"].as_f64().unwrap() > 1e7);
    assert_eq!(m["pmu_unavailable_reason"], "disabled by --timing-only");
    // scored against a committed calibration of another machine: flagged, and too short to grade
    let cal = repo().join("research/optlevel_20261117/controls-run2/calibration.json");
    let out = go(&[
        "score",
        "--calibration",
        &cal.to_string_lossy(),
        "--measurement",
        &p("m.json"),
        "--out",
        &p("r.json"),
        "--vector-share",
        "0.0",
        "--registry-id",
        "x.y",
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("This is not a speed claim"));
    let r = read_json(&d.join("r.json")).unwrap();
    assert!(r["grade"]["score"].is_null());
    assert_eq!(
        (
            r["kernel"]["registry_id"].as_str(),
            r["diagnosis"]["vector_share_static"].as_f64(),
            r["intensity"]["in_cache"].as_bool()
        ),
        (Some("x.y"), Some(0.0), Some(true))
    );
    assert_eq!(go(&["validate", &p("r.json")]).status.code(), Some(0));
    assert!(go(&["benchrec", &p("r.json")]).stdout.starts_with(
        b"python3 tools/benchrec.py metric --name optlevel:modmuladd_p31:single:ops_per_s"
    ));
    // a record that breaks a tier rule fails validation with exit 1
    let mut bad = r.clone();
    bad["diagnosis"]["ipc"] = json!(2.0);
    util::write_json(&d.join("bad.json"), &bad).unwrap();
    let out = go(&["validate", &p("bad.json")]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stdout).contains("must not report PMU components: ipc"));
}

#[test]
fn operation_counts_from_regex_constant_and_none() {
    let d = tmp("ops");
    let p = |n: &str| d.join(n).to_string_lossy().into_owned();
    script(&d, "steps", "echo walk_steps=1,000; echo walk_steps=24 >&2");
    let k = p("steps");
    let go = |args: &[&str]| {
        Command::new(BIN)
            .args(args)
            .env("OPTLEVEL_CACHE", &d)
            .output()
            .unwrap()
    };
    assert!(go(&[
        "measure",
        "--out",
        &p("a.json"),
        "--repeat",
        "1",
        "--timing-only",
        "--op-class",
        "xor_u64",
        "--ops-regex",
        r"walk_steps=([\d,]+)",
        "--",
        &k
    ])
    .status
    .success());
    let a = read_json(&d.join("a.json")).unwrap();
    assert_eq!(a["points"]["1"]["ops"]["median"].as_f64().unwrap(), 1024.0);
    assert!(a["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "WALL_INCLUDES_STARTUP"));
    assert!(go(&[
        "measure",
        "--out",
        &p("b.json"),
        "--repeat",
        "1",
        "--timing-only",
        "--op-class",
        "xor_u64",
        "--ops",
        "7",
        "--scale",
        "copies:1,2",
        "--no-pin",
        &k
    ])
    .status
    .success());
    let b = read_json(&d.join("b.json")).unwrap();
    assert_eq!(
        (
            b["points"]["1"]["ops"]["median"].as_f64(),
            b["points"]["2"]["ops"]["median"].as_f64(),
            b["ops_source"].as_str()
        ),
        (Some(7.0), Some(14.0), Some("--ops (constant)"))
    );
    // no count at all: exit 4, and the file says why
    let out = go(&[
        "measure",
        "--out",
        &p("c.json"),
        "--repeat",
        "1",
        "--timing-only",
        "--op-class",
        "xor_u64",
        "--",
        &k,
    ]);
    assert_eq!(out.status.code(), Some(4));
    assert!(read_json(&d.join("c.json")).unwrap()["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "OPS_MISSING"));
    // usage errors
    assert_eq!(
        go(&["measure", "--out", &p("d.json")]).status.code(),
        Some(2)
    );
    assert_eq!(go(&["score", "--out", "x"]).status.code(), Some(2));
    assert_eq!(go(&["nonsense"]).status.code(), Some(2));
    assert_eq!(
        go(&["measure", "--bogus", "1", "--", &k]).status.code(),
        Some(2)
    );
    assert!(go(&["--help"]).status.success());
}

#[test]
fn timeout_kills_a_stuck_command() {
    let d = tmp("timeout");
    script(&d, "hang", "exec /bin/sleep 30");
    let t0 = std::time::Instant::now();
    let out = Command::new(BIN)
        .args([
            "measure",
            "--out",
            &d.join("m.json").to_string_lossy(),
            "--repeat",
            "1",
            "--timing-only",
            "--op-class",
            "xor_u64",
            "--ops",
            "1",
            "--timeout",
            "0.3",
            "--",
            &d.join("hang").to_string_lossy(),
        ])
        .env("OPTLEVEL_CACHE", &d)
        .output()
        .unwrap();
    assert!(t0.elapsed().as_secs_f64() < 10.0);
    assert!(out.status.success());
    let m = read_json(&d.join("m.json")).unwrap();
    assert!(m["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "COMMAND_FAILED"));
    assert_eq!(m["runs"][0]["exit_status"], json!([-9]));
}

#[test]
fn vector_share_of_the_tools_own_kernels() {
    if optlevel::util::which("objdump").is_none() {
        return;
    }
    let share = |isa: &str| -> f64 {
        let out = Command::new(BIN)
            .args([
                "vector-share",
                BIN,
                &optlevel::kernels::symbol(isa, "modmuladd_p31"),
            ])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["vector_share_static"]
            .as_f64()
            .unwrap()
    };
    assert_eq!(share("scalar"), 0.0);
    let widest = optlevel::kernels::available_isas()[0];
    if widest != "scalar" && cfg!(target_arch = "x86_64") {
        assert!(share(widest) > 0.3, "{widest}: {}", share(widest));
    }
    assert_eq!(
        Command::new(BIN)
            .args(["vector-share", BIN, "no_such_symbol"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
    // the parser, on objdump text
    let text = "0000000000001000 <f>:\n    1000:\tvpmuludq %ymm1,%ymm2,%ymm3\n    1004:\tmov    %rax,%rbx\n    1008:\tnop\n    1009:\taddps  %xmm1,%xmm0\n    100c:\tvmovq  %xmm0,%rax\n0000000000002000 <g>:\n    2000:\tret\n";
    assert_eq!(
        cmd::vector_share_of(text, &["f".to_string()]),
        Some((vec!["f".to_string()], 4, 2))
    );
    assert_eq!(cmd::vector_share_of(text, &["h".to_string()]), None);
}

#[test]
fn perf_probe_classification() {
    assert!(
        classify_perf_probe("<not supported>,,cycles:u,0,100.00,,\n", Some(0))
            .unwrap_err()
            .contains("no hardware PMU")
    );
    assert!(classify_perf_probe(
        "Error:\nAccess to performance monitoring and observability operations is limited.\n",
        Some(255)
    )
    .unwrap_err()
    .contains("forbids counting"));
    assert!(classify_perf_probe("", Some(1))
        .unwrap_err()
        .contains("exit 1"));
    assert!(classify_perf_probe("1000,,cycles:u,5,100.00,,\n", Some(0)).is_ok());
}
