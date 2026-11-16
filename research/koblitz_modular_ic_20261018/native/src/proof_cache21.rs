//! Verification infrastructure only: byte-identical compiled-object cache locality.
use crate::sha256_impl;
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};
fn selected(p: &Path) -> bool {
    let s = p.file_name().unwrap().to_str().unwrap();
    [".olean", ".olean.private", ".olean.server", ".ir", ".so"]
        .iter()
        .any(|x| s.ends_with(x))
}
fn gather(root: &Path, dir: &Path, index: usize, out: &mut Vec<(usize, PathBuf, PathBuf, u64)>) {
    for e in fs::read_dir(dir).unwrap() {
        let entry = e.unwrap();
        let kind = entry.file_type().unwrap();
        let p = entry.path();
        if kind.is_dir() || (kind.is_symlink() && p.is_dir()) {
            gather(root, &p, index, out)
        } else if selected(&p) {
            out.push((index, p.clone(), p.strip_prefix(root).unwrap().into(), 0));
        }
    }
}
pub fn run(list: &Path, dest: &Path, receipt: &Path) {
    assert!(
        !dest.exists(),
        "new cache directory required;reconcile any previous cache first"
    );
    assert!(!receipt.exists());
    let roots = fs::read_to_string(list)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    assert!(roots.len() >= 2);
    let mut jobs = vec![];
    let mut absent = vec![];
    for (i, p) in roots.iter().enumerate() {
        if p.is_dir() {
            gather(p, p, i, &mut jobs)
        } else {
            absent.push(i);
        }
    }
    jobs.sort_by(|a, b| (&a.0, &a.2).cmp(&(&b.0, &b.2)));
    println!(
        "cache discovery:{} selected files;{} originally absent search roots",
        jobs.len(),
        absent.len()
    );
    let sizes = Mutex::new(vec![]);
    let meta_next = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| loop {
                let n = meta_next.fetch_add(1, Ordering::Relaxed);
                if n >= jobs.len() {
                    break;
                }
                let size = fs::metadata(&jobs[n].1).unwrap().len();
                sizes.lock().unwrap().push((n, size));
            });
        }
    });
    for (n, size) in sizes.into_inner().unwrap() {
        jobs[n].3 = size;
    }
    let bytes = jobs.iter().map(|j| j.3).sum::<u64>();
    println!(
        "cache preflight:{} roots,{} files,{bytes} bytes;8GiB limit",
        roots.len(),
        jobs.len()
    );
    assert!(
        bytes <= 8 * 1024 * 1024 * 1024,
        "verification cache envelope"
    );
    fs::create_dir(dest).unwrap();
    for i in 0..roots.len() {
        fs::create_dir(dest.join(format!("root-{i:02}"))).unwrap();
    }
    let next = AtomicUsize::new(0);
    let records = Mutex::new(vec![]);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| loop {
                let n = next.fetch_add(1, Ordering::Relaxed);
                if n >= jobs.len() {
                    break;
                }
                let (i, src, rel, size) = &jobs[n];
                let data = fs::read(src).unwrap();
                assert_eq!(data.len() as u64, *size);
                let hash = hex::encode(sha256_impl::sha256(&data));
                let target = dest.join(format!("root-{i:02}")).join(rel);
                fs::create_dir_all(target.parent().unwrap()).unwrap();
                fs::write(&target, &data).unwrap();
                let copied = fs::read(&target).unwrap();
                assert_eq!(copied.len(), data.len());
                assert_eq!(hex::encode(sha256_impl::sha256(&copied)), hash);
                records.lock().unwrap().push((
                    n,
                    json!({"root":i,"relative":rel,"bytes":size,"sha256":hash}),
                ));
            });
        }
    });
    let mut records = records.into_inner().unwrap();
    records.sort_by_key(|v| v.0);
    assert_eq!(records.len(), jobs.len());
    let mapped = (0..roots.len())
        .map(|i| dest.join(format!("root-{i:02}")))
        .collect::<Vec<_>>();
    let lean_path = mapped
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect::<Vec<_>>()
        .join(":");
    fs::write(dest.join("lean-path.txt"), format!("{lean_path}\n")).unwrap();
    let value = json!({"schema":"koblitz.round21-proof-cache/v1","source_roots":roots,"originally_absent_roots":absent,"local_roots":mapped,"files":records.len(),"bytes":bytes,"all_source_destination_hashes_equal":true,"members":records.into_iter().map(|v|v.1).collect::<Vec<_>>()});
    fs::write(receipt, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    println!("PASS:{} cached files;all SHA256 and byte lengths match;source/dependency/theorem bytes unchanged",jobs.len());
}

pub fn signatures(base: &Path, dest: &Path, receipt: &Path) {
    assert!(!receipt.exists());
    let old: serde_json::Value = serde_json::from_slice(&fs::read(base).unwrap()).unwrap();
    assert_eq!(old["all_source_destination_hashes_equal"], true);
    let members = old["members"].as_array().unwrap();
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| loop {
                let n = next.fetch_add(1, Ordering::Relaxed);
                if n >= members.len() {
                    break;
                }
                let m = &members[n];
                let root = m["root"].as_u64().unwrap();
                let target = dest
                    .join(format!("root-{root:02}"))
                    .join(m["relative"].as_str().unwrap());
                let data = fs::read(target).unwrap();
                assert_eq!(data.len() as u64, m["bytes"].as_u64().unwrap());
                assert_eq!(
                    hex::encode(sha256_impl::sha256(&data)),
                    m["sha256"].as_str().unwrap()
                );
            });
        }
    });
    let ir = members
        .iter()
        .filter(|m| m["relative"].as_str().unwrap().ends_with(".ir"))
        .collect::<Vec<_>>();
    let next = AtomicUsize::new(0);
    let records = Mutex::new(vec![]);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| loop {
                let n = next.fetch_add(1, Ordering::Relaxed);
                if n >= ir.len() {
                    break;
                }
                let m = ir[n];
                let root = m["root"].as_u64().unwrap();
                let rel = PathBuf::from(format!("{}.sig", m["relative"].as_str().unwrap()));
                let source =
                    Path::new(old["source_roots"][root as usize].as_str().unwrap()).join(&rel);
                if source.is_file() {
                    let data = fs::read(&source).unwrap();
                    records.lock().unwrap().push((root, rel, data));
                }
            });
        }
    });
    let mut additions = records.into_inner().unwrap();
    additions.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    let bytes = additions.iter().map(|v| v.2.len() as u64).sum::<u64>();
    let total = old["bytes"].as_u64().unwrap() + bytes;
    println!(
        "IR signatures:{} files,{bytes} bytes;combined{total};8GiB limit",
        additions.len()
    );
    assert!(total <= 8 * 1024 * 1024 * 1024, "combined cache envelope");
    let mut result = vec![];
    for (root, rel, data) in additions {
        let target = dest.join(format!("root-{root:02}")).join(&rel);
        assert!(!target.exists());
        fs::write(&target, &data).unwrap();
        let copied = fs::read(target).unwrap();
        assert_eq!(copied, data);
        let hash = hex::encode(sha256_impl::sha256(&data));
        result.push(json!({"root":root,"relative":rel,"bytes":data.len(),"sha256":hash}));
    }
    let value = json!({"schema":"koblitz.round21-proof-cache-signatures/v1","base_manifest_sha256":hex::encode(sha256_impl::sha256(&fs::read(base).unwrap())),"original_members_rechecked":members.len(),"files":result.len(),"bytes":bytes,"combined_bytes":total,"all_source_destination_hashes_equal":true,"members":result});
    fs::write(receipt, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    println!(
        "PASS:original cache members rechecked;all required IR signature copies byte-identical"
    );
}
