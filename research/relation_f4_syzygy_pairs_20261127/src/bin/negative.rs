//! End-to-end corruptions against the frozen replay executable, no F4 calls.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
fn hash_tree(root: &Path) -> BTreeMap<PathBuf, String> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, String>) {
        for e in fs::read_dir(dir).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            if e.file_type().unwrap().is_dir() {
                walk(root, &p, out)
            } else {
                assert!(e.file_type().unwrap().is_file());
                out.insert(
                    p.strip_prefix(root).unwrap().to_path_buf(),
                    format!("{:x}", Sha256::digest(fs::read(p).unwrap())),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir(dst).unwrap();
    for e in fs::read_dir(src).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            copy_tree(&e.path(), &dst.join(e.file_name()))
        } else {
            assert!(e.file_type().unwrap().is_file());
            fs::copy(e.path(), dst.join(e.file_name())).unwrap();
        }
    }
}
fn load(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn update(p: &Path, edit: impl FnOnce(&mut Value)) {
    let mut v = load(p);
    edit(&mut v);
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4, "usage: negative STUDY WORKER SCRATCH");
    let study = Path::new(&args[1]);
    let worker = Path::new(&args[2]).canonicalize().unwrap();
    let scratch = Path::new(&args[3]);
    fs::create_dir(scratch).unwrap();
    let data = study.join("data");
    let before = hash_tree(&data);
    let mut records = vec![];
    let cases = [
        ("gate_boolean", "incomplete/unsupported space decision"),
        ("missing_cap", "incomplete/unsupported space decision"),
        ("claimed_rank", "rank drift"),
        ("snapshot_coefficient", "original-ideal witness fails"),
        ("basis_coefficient", "basis not nonzero monic"),
        (
            "aa_counter",
            "fresh A/A or prior original/full comparison mismatch",
        ),
        (
            "unsupported_observation",
            "fabricated unsupported observation",
        ),
    ];
    for (name, expected) in cases {
        let root = scratch.join(name);
        copy_tree(&data, &root);
        let first = root.join("p2147483647-n3");
        match name {
            "gate_boolean" => update(&first.join("original-a.json"), |v| {
                v["polynomial_space_gate_pass"] = json!(true)
            }),
            "missing_cap" => update(&first.join("full.json"), |v| {
                v["space_checks"].as_array_mut().unwrap().pop();
            }),
            "claimed_rank" => update(&first.join("original-a.json"), |v| {
                v["space_checks"][0]["rank"] = json!(1)
            }),
            "snapshot_coefficient" => {
                let p = first.join("baseline-D3.json.gz");
                let data = fs::read(&p).unwrap();
                let mut plain = vec![];
                flate2::read::GzDecoder::new(data.as_slice())
                    .take(128 * 1024 * 1024 + 1)
                    .read_to_end(&mut plain)
                    .unwrap();
                assert!(plain.len() <= 128 * 1024 * 1024);
                let mut v: Value = serde_json::from_slice(&plain).unwrap();
                let c = &mut v["basis"][0]["polynomial"][0]["coefficient"];
                *c = json!(if c.as_u64().unwrap() == 1 { 2 } else { 1 });
                let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
                gz.write_all(&serde_json::to_vec(&v).unwrap()).unwrap();
                fs::write(p, gz.finish().unwrap()).unwrap();
            }
            "basis_coefficient" => update(&first.join("original-a.json"), |v| {
                v["observation"]["basis"][0][0][1] = json!(2)
            }),
            "aa_counter" => update(&first.join("original-aa.json"), |v| {
                let count = v["observation"]["row_reduction_multiplications_only"]
                    .as_u64()
                    .unwrap();
                v["observation"]["row_reduction_multiplications_only"] = json!(count + 1);
            }),
            "unsupported_observation" => {
                update(&root.join("p9007199254740881-n3/original-a.json"), |v| {
                    v["observation"] = json!({"productive_degree_observation":4})
                });
                // Order changes only to reach this guard first. All six cases,
                // original fields, witnesses and expected totals are retained.
                update(&root.join("ROSTER.json"), |v| {
                    v["cases"].as_array_mut().unwrap().swap(0, 3)
                });
            }
            _ => unreachable!(),
        }
        let output = Command::new(&worker)
            .arg("replay")
            .arg(&root)
            .arg(root.join("forbidden-summary.json"))
            .output()
            .unwrap();
        assert!(output.stdout.len() < 65536 && output.stderr.len() < 65536);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let rejected = output.status.code() == Some(1)
            && stderr.contains(expected)
            && !root.join("forbidden-summary.json").exists();
        let raw = study.join("raw");
        let log = raw.join(format!("negative-{name}.stderr"));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(log)
            .unwrap();
        file.write_all(&output.stderr).unwrap();
        records.push(json!({"canary":name,"exit":output.status.code(),"expected_diagnostic":expected,"rejected":rejected,"forbidden_summary_absent":!root.join("forbidden-summary.json").exists()}));
        assert!(rejected, "{name}: {stderr}");
        fs::remove_dir_all(&root).unwrap();
    }
    assert_eq!(before, hash_tree(&data), "original evidence changed");
    let report = json!({"schema":"syzygy-pair-negative-replay/v1","records":records,"all_original_data_bytes_unchanged":true,"original_files_checked":before.len(),"solver_calls":0,"reference_constructions":0,"original_15x_achieved":false});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(study.join("NEGATIVE.json"))
        .unwrap();
    file.write_all(&serde_json::to_vec_pretty(&report).unwrap())
        .unwrap();
    file.sync_all().unwrap();
    println!(
        "7 end-to-end corruptions rejected with exact diagnostics, original data bytes unchanged"
    );
}
