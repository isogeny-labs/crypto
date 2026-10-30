use hashed_cpu_panel_tools::{export, HEADER, MANIFESTS};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    root: PathBuf,
    study: PathBuf,
    out: PathBuf,
    analyzer: PathBuf,
    rows: Vec<Vec<String>>,
}
impl Fixture {
    fn new() -> Self {
        let source = PathBuf::from(std::env::var_os("PANEL_STUDY").expect("PANEL_STUDY required"));
        let analyzer = PathBuf::from(std::env::var_os("ANALYZER").expect("ANALYZER required"));
        let root = std::env::temp_dir().join(format!(
            "native-panel-tests-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let study = root.join(source.file_name().unwrap());
        let out = study.join("results");
        fs::create_dir_all(&out).unwrap();
        let template = root.join("panel2/results");
        fs::create_dir_all(&template).unwrap();
        fs::copy(
            source.parent().unwrap().join("panel2/results/export.json"),
            template.join("export.json"),
        )
        .unwrap();
        fs::copy(source.join("analyze.rs"), study.join("analyze.rs")).unwrap();
        for name in MANIFESTS {
            fs::copy(source.join(name), study.join(name)).unwrap();
            fs::copy(source.join(name), out.join(name)).unwrap();
        }
        let mut rows = Vec::new();
        for arm in ['u', 'v', 'w', 'x', 'y', 'z'] {
            for (case, scalar) in [42, 45678, 54321].iter().enumerate() {
                for round in 0..10 {
                    let mut row = vec![
                        arm.to_string(),
                        case.to_string(),
                        (111 * (case + 1)).to_string(),
                        round.to_string(),
                        (round == 0).to_string(),
                        "verified".into(),
                        scalar.to_string(),
                        "10".into(),
                        "10".into(),
                    ];
                    row.extend(vec!["1.0".into(); 6]);
                    row.extend(vec!["0".into(); 7]);
                    row.extend(["0.001".into(), "L0".into()]);
                    rows.push(row);
                    fs::write(
                        out.join(format!("cert-{arm}-{case}-{round}.txt")),
                        format!("SYNTHETIC TEST ONLY case={case} round={round}\n"),
                    )
                    .unwrap();
                }
            }
        }
        fs::write(
            out.join("cpuinfo.txt"),
            "model name : SYNTHETIC TEST ONLY\n",
        )
        .unwrap();
        fs::write(out.join("kernel.txt"), "SYNTHETIC TEST ONLY\n").unwrap();
        fs::write(out.join("exit-code.txt"), "0\n").unwrap();
        let f = Self {
            root,
            study,
            out,
            analyzer,
            rows,
        };
        f.write_rows(&f.rows);
        f
    }
    fn write_rows(&self, rows: &[Vec<String>]) {
        let mut text = format!("{HEADER}\n");
        for row in rows {
            text.push_str(&row.join(","));
            text.push('\n');
        }
        fs::write(self.out.join("rows.csv"), text).unwrap();
    }
    fn analyze(&self, rows: &[Vec<String>]) -> Output {
        self.write_rows(rows);
        Command::new(&self.analyzer)
            .arg(self.out.join("rows.csv"))
            .arg(&self.out)
            .output()
            .unwrap()
    }
    fn export(&self) -> hashed_cpu_panel_tools::Result<()> {
        export(&self.study, &self.analyzer, "2026-10-30T00:00:00Z")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn complete_six_arm_panel() {
    let f = Fixture::new();
    let output = f.analyze(&f.rows);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout)
        .contains("PASS 180 rows, 162 measured, 18 complete cells"));
    let summary = fs::read_to_string(f.out.join("summary.csv")).unwrap();
    assert_eq!(summary.lines().skip(1).count(), 108);
    let arms: BTreeSet<_> = summary
        .lines()
        .skip(1)
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(arms, BTreeSet::from(["u", "v", "w", "x", "y", "z"]));
}
#[test]
fn missing_or_duplicate_cell() {
    let f = Fixture::new();
    let mut duplicated = f.rows.clone();
    duplicated.push(f.rows[0].clone());
    let no_z: Vec<_> = f.rows.iter().filter(|r| r[0] != "z").cloned().collect();
    for rows in [&f.rows[..179], &duplicated, &no_z] {
        assert!(!f.analyze(rows).status.success());
    }
}
#[test]
fn mismatched_full_certificate() {
    let f = Fixture::new();
    fs::write(
        f.out.join("cert-z-2-9.txt"),
        "DIFFERENT SYNTHETIC CERTIFICATE\n",
    )
    .unwrap();
    assert!(!f.analyze(&f.rows).status.success());
}
#[test]
fn empty_certificates() {
    let f = Fixture::new();
    for arm in ['u', 'v', 'w', 'x', 'y', 'z'] {
        fs::write(f.out.join(format!("cert-{arm}-0-0.txt")), " \n").unwrap();
    }
    assert!(!f.analyze(&f.rows).status.success());
}
#[test]
fn row_order_does_not_change_summary() {
    let f = Fixture::new();
    assert!(f.analyze(&f.rows).status.success());
    let expected = fs::read(f.out.join("summary.csv")).unwrap();
    let mut reversed = f.rows.clone();
    reversed.reverse();
    assert!(f.analyze(&reversed).status.success());
    assert_eq!(fs::read(f.out.join("summary.csv")).unwrap(), expected);
}
#[test]
fn malformed_fields() {
    let f = Fixture::new();
    for (col, value) in [
        (0, "bad"),
        (1, "3"),
        (2, "999"),
        (3, "10"),
        (4, "false"),
        (5, "failed"),
        (6, "999"),
        (23, "L2"),
    ] {
        let mut rows = f.rows.clone();
        rows[0][col] = value.into();
        assert!(!f.analyze(&rows).status.success(), "column {col}: {value}");
    }
    for row in [0, 1] {
        for (col, value) in [
            (9, "NaN"),
            (14, "-1"),
            (14, "inf"),
            (22, "NaN"),
            (7, "-1"),
            (15, "broken"),
            (21, "18446744073709551616"),
        ] {
            let mut rows = f.rows.clone();
            rows[row][col] = value.into();
            assert!(
                !f.analyze(&rows).status.success(),
                "row {row}, column {col}: {value}"
            );
        }
    }
}
#[test]
fn complete_export_and_no_overwrite() {
    let f = Fixture::new();
    f.export().unwrap();
    let original = fs::read(f.out.join("export.json")).unwrap();
    let d: Value = serde_json::from_slice(&original).unwrap();
    let runs = d["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 180);
    assert_eq!(runs.iter().filter(|r| r["warmup"] == true).count(), 18);
    assert_eq!(d["arms"].as_array().unwrap().len(), 6);
    assert_eq!(d["flow"]["planned"], 180);
    let round = f.study.file_name().unwrap().to_str().unwrap();
    let job = format!("hashed-ecdl-cpu-{round}-20261030");
    assert_eq!(d["host"]["class_id"], job);
    assert!(runs.iter().all(|run| run["backend_ref"] == job));
    assert!(f.export().is_err());
    assert_eq!(fs::read(f.out.join("export.json")).unwrap(), original);
}
#[test]
fn unsupported_study_is_not_relabelled() {
    let mut f = Fixture::new();
    let renamed = f.root.join("round999");
    fs::rename(&f.study, &renamed).unwrap();
    f.study = renamed;
    f.out = f.study.join("results");
    assert!(f.export().is_err());
    assert!(!f.out.join("export.json").exists());
}
#[test]
fn wrong_returned_manifest_rejected() {
    let f = Fixture::new();
    fs::write(f.out.join("inverse-materialization.json"), "{}").unwrap();
    assert!(f.export().is_err());
    assert!(!f.out.join("export.json").exists());
}
#[test]
fn incomplete_rows_rejected() {
    let f = Fixture::new();
    f.write_rows(&f.rows[..179]);
    assert!(f.export().is_err());
    assert!(!f.out.join("export.json").exists());
}
#[test]
fn stale_pass_does_not_bypass_certificate_check() {
    let f = Fixture::new();
    fs::write(
        f.out.join("analysis.log"),
        "PASS 180 rows, 162 measured, 18 complete cells, byte-identical full certificates\n",
    )
    .unwrap();
    fs::write(
        f.out.join("cert-z-2-9.txt"),
        "DIFFERENT SYNTHETIC CERTIFICATE\n",
    )
    .unwrap();
    assert!(f.export().is_err());
    assert!(!f.out.join("export.json").exists());
}
#[test]
fn changed_analyzer_source_rejected() {
    let f = Fixture::new();
    fs::write(f.study.join("analyze.rs"), "// wrong source\n").unwrap();
    assert!(f.export().is_err());
    assert!(!f.out.join("export.json").exists());
}

#[test]
#[ignore = "explicitly run against the retained round18 results; writes only a temporary copy"]
fn historical_round18_export_parity() {
    let f = Fixture::new();
    let source = PathBuf::from(std::env::var_os("PANEL_STUDY").unwrap());
    assert_eq!(source.file_name().unwrap(), "round18");
    fs::remove_dir_all(&f.out).unwrap();
    fs::create_dir(&f.out).unwrap();
    for entry in fs::read_dir(source.join("results")).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file()
            && !matches!(
                path.file_name().unwrap().to_str().unwrap(),
                "export.json" | "export-check.log"
            )
        {
            fs::copy(&path, f.out.join(path.file_name().unwrap())).unwrap();
        }
    }
    f.export().unwrap();
    let mut actual: Value =
        serde_json::from_slice(&fs::read(f.out.join("export.json")).unwrap()).unwrap();
    let mut expected: Value =
        serde_json::from_slice(&fs::read(source.join("results/export.json")).unwrap()).unwrap();
    // Producer wording and export creation time intentionally differ. All
    // measurements, per-run fields, flow counts and artifact hashes must match.
    for value in [&mut actual, &mut expected] {
        value.as_object_mut().unwrap().remove("created");
        value.as_object_mut().unwrap().remove("notes");
    }
    assert_eq!(actual, expected);
}
