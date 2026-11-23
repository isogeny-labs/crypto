//! Native custody collection; numerical reduction is in heldout_summary.jq.
use std::{fs, path::Path, process::Command};
const SOURCE: &str = env!("HEC_SOURCE");
const ARMS: [&str; 16] = [
    "n1fr",
    "n1fd",
    "n1hr",
    "n1hd",
    "n64fr",
    "n64fd",
    "n64hr",
    "n64hd",
    "t1f",
    "t1h",
    "t64f",
    "t64h",
    "compact",
    "rational-generic",
    "rational-linear",
    "rational-linear-aa",
];
fn sha(p: &Path) -> String {
    let o = Command::new("/usr/bin/shasum")
        .args(["-a", "256"])
        .arg(p)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .into()
}
fn emit(root: &Path, stage: &str, g: usize, p: u64, f: u64, s: u64, arm: &str, hash: &str) {
    let key = if stage == "select" {
        format!("select-g{g}-p{p}-f{f}")
    } else {
        format!("{stage}-g{g}-p{p}-f{f}-s{s}-{arm}")
    };
    let receipt = root.join(format!("{key}.receipt"));
    if !receipt.exists() {
        println!("{{\"stage\":\"{stage}\",\"g\":{g},\"p\":{p},\"family\":{f},\"seed\":{s},\"arm\":\"{arm}\",\"status\":\"unstarted\",\"observation\":null}}");
        return;
    }
    let text = fs::read_to_string(receipt).unwrap();
    let mut lines = text.lines();
    let header = lines.next().unwrap();
    let args = if stage == "select" {
        vec![
            "--select-worker".to_string(),
            g.to_string(),
            p.to_string(),
            f.to_string(),
        ]
    } else {
        vec![
            "--worker".to_string(),
            root.join(format!("select-g{g}-p{p}-f{f}.out"))
                .to_str()
                .unwrap()
                .to_string(),
            s.to_string(),
            arm.to_string(),
        ]
    };
    assert_eq!(
        header,
        format!("HEC-HELDOUT-v1 {SOURCE} {hash} {args:?}"),
        "receipt custody {key}"
    );
    let fields: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    let status = fields[0];
    let wall = fields[1].parse::<u128>().unwrap();
    let output = fs::read_to_string(root.join(format!("{key}.out"))).unwrap_or_default();
    let observation = if stage == "select" || output.trim().is_empty() {
        "null"
    } else {
        output
            .trim()
            .strip_prefix("CELL ")
            .expect("one CELL object")
    };
    let fixture = if stage == "select" {
        format!("{:?}", output.trim())
    } else {
        "null".into()
    };
    println!("{{\"stage\":\"{stage}\",\"g\":{g},\"p\":{p},\"family\":{f},\"seed\":{s},\"arm\":\"{arm}\",\"status\":\"{status}\",\"child_wall_ns\":{wall},\"fixture\":{fixture},\"observation\":{observation}}}");
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(&args[1]);
    for (g, ps) in [
        (2, &[1009, 16381][..]),
        (3, &[127, 1009][..]),
        (4, &[31, 127][..]),
    ] {
        for &p in ps {
            for f in 0..1 {
                let fixture = root.join(format!("select-g{g}-p{p}-f{f}.out"));
                let hash = if fixture.exists() {
                    sha(&fixture)
                } else {
                    "unavailable".into()
                };
                for s in 5..8 {
                    emit(root, "gate", g, p, f, s, "rational-linear-gate", &hash);
                    for arm in ARMS {
                        emit(root, "measure", g, p, f, s, arm, &hash);
                    }
                }
            }
        }
    }
}
