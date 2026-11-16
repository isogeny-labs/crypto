use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Deserialize)]
struct Rho {
    s: f64,
    correct: bool,
}

#[derive(Deserialize)]
struct Raw {
    p: u64,
    seed: u64,
    l: f64,
    c_add_e: f64,
    residuals: u64,
    relations: u64,
    setup_muls: u64,
    base_muls: u64,
    stream_muls: u64,
    weil_muls: u64,
    f4_muls: u64,
    lin_muls: u64,
    post_muls: u64,
    verify_muls: u64,
    la_muls: u64,
    total_muls: u64,
    s: f64,
    solved: bool,
    correct: bool,
    rho: Vec<Rho>,
    mismatches: u64,
}

#[derive(Deserialize)]
struct Summary {
    class: String,
    dlp: Vec<Row>,
}

#[derive(Deserialize)]
struct Row {
    p: u64,
    seed: u64,
    residuals: u64,
    relations: u64,
    solved: [bool; 2],
    correct: [bool; 2],
    rho_runs: u64,
    s_before: f64,
    s_after: f64,
    rho_s_mean: f64,
    s_ratio: f64,
    s_over_rho_before: f64,
    s_over_rho_after: f64,
}

fn read_json<T: for<'a> Deserialize<'a>>(path: &Path) -> T {
    serde_json::from_str(&fs::read_to_string(path).expect("frozen input is missing"))
        .expect("frozen input is invalid JSON")
}

fn phase_total(raw: &Raw) -> u64 {
    [
        raw.setup_muls,
        raw.base_muls,
        raw.stream_muls,
        raw.weil_muls,
        raw.f4_muls,
        raw.lin_muls,
        raw.post_muls,
        raw.verify_muls,
        raw.la_muls,
    ]
    .into_iter()
    .try_fold(0_u64, u64::checked_add)
    .expect("phase counter sum overflowed")
}

fn close(actual: f64, expected: f64) -> bool {
    actual.is_finite()
        && expected.is_finite()
        && actual > 0.0
        && expected > 0.0
        && (actual - expected).abs() <= 1e-12 * expected.abs().max(actual.abs())
}

fn audit(root: &Path) {
    let summary: Summary = read_json(&root.join("summary.json"));
    let raw_dir = root.join("reports/run-20261018T1548Z/raw");
    let before: Vec<Raw> = read_json(&raw_dir.join("dlp_main.json"));
    let after: Vec<Raw> = read_json(&raw_dir.join("dlp_new.json"));
    let expected = [(53, 1), (53, 2), (101, 1), (101, 2)];
    assert_eq!(summary.class, "engineering");
    assert_eq!(summary.dlp.len(), expected.len());
    assert_eq!(before.len(), expected.len());
    assert_eq!(after.len(), expected.len());

    let index = |rows: Vec<Raw>| -> BTreeMap<(u64, u64), Raw> {
        let map: BTreeMap<_, _> = rows.into_iter().map(|r| ((r.p, r.seed), r)).collect();
        assert_eq!(map.len(), expected.len(), "duplicate p/seed row");
        assert_eq!(map.keys().copied().collect::<Vec<_>>(), expected);
        map
    };
    let before = index(before);
    let after = index(after);
    let mut seen = Vec::new();
    for row in &summary.dlp {
        let key = (row.p, row.seed);
        let a = &before[&key];
        let b = &after[&key];
        seen.push(key);
        assert_eq!((row.residuals, row.relations), (a.residuals, a.relations));
        assert_eq!((row.residuals, row.relations), (b.residuals, b.relations));
        assert!(row.relations > 0);
        assert_eq!(row.solved, [a.solved, b.solved]);
        assert_eq!(row.correct, [a.correct, b.correct]);
        assert_eq!(row.solved, [true; 2]);
        assert_eq!(row.correct, [true; 2]);
        assert_eq!((a.mismatches, b.mismatches), (0, 0));
        assert_eq!(a.total_muls, phase_total(a));
        assert_eq!(b.total_muls, phase_total(b));
        assert_eq!(a.l, b.l);
        assert_eq!(a.c_add_e, b.c_add_e);
        assert!(a.l > 0.0 && a.c_add_e > 0.0);
        assert_eq!(a.rho.len(), 4);
        assert_eq!(b.rho.len(), 4);
        assert_eq!(row.rho_runs, 4);
        for (left, right) in a.rho.iter().zip(&b.rho) {
            assert!(left.correct && right.correct);
            assert!(close(left.s, right.s));
        }
        let s_before = a.total_muls as f64 / (a.l.sqrt() * a.c_add_e);
        let s_after = b.total_muls as f64 / (b.l.sqrt() * b.c_add_e);
        let rho_mean = a.rho.iter().map(|r| r.s).sum::<f64>() / 4.0;
        assert!(s_before > s_after);
        assert!(close(a.s, s_before) && close(b.s, s_after));
        assert!(close(row.s_before, s_before) && close(row.s_after, s_after));
        assert!(close(row.rho_s_mean, rho_mean));
        assert!(close(row.s_ratio, s_before / s_after));
        assert!(close(row.s_over_rho_before, s_before / rho_mean));
        assert!(close(row.s_over_rho_after, s_after / rho_mean));
    }
    seen.sort_unstable();
    assert_eq!(seen, expected, "summary rows are missing or duplicated");
    println!(
        "native F4 frozen-accounting audit passed: {} paired DLP rows",
        seen.len()
    );
}

fn main() {
    audit(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("study root"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_phase_counter_or_summary_ratio_is_rejected() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("study root");
        for corrupt_summary in [false, true] {
            let root = std::env::temp_dir().join(format!(
                "f4-frozen-audit-{}-{}",
                std::process::id(),
                corrupt_summary
            ));
            let raw = root.join("reports/run-20261018T1548Z/raw");
            fs::create_dir_all(&raw).expect("temporary raw directory");
            for name in ["dlp_main.json", "dlp_new.json"] {
                fs::copy(
                    source.join("reports/run-20261018T1548Z/raw").join(name),
                    raw.join(name),
                )
                .expect("copy frozen raw input");
            }
            fs::copy(source.join("summary.json"), root.join("summary.json"))
                .expect("copy frozen summary");
            let path = if corrupt_summary {
                root.join("summary.json")
            } else {
                raw.join("dlp_new.json")
            };
            let mut document: serde_json::Value = read_json(&path);
            let field = if corrupt_summary {
                "s_ratio"
            } else {
                "f4_muls"
            };
            if corrupt_summary {
                let old = document["dlp"][0][field].as_f64().expect("ratio");
                document["dlp"][0][field] = serde_json::Value::from(old + 1.0);
            } else {
                let old = document[0][field].as_u64().expect("counter");
                document[0][field] = serde_json::Value::from(old + 1);
            }
            fs::write(
                &path,
                serde_json::to_vec(&document).expect("encode mutation"),
            )
            .expect("write mutation");
            assert!(std::panic::catch_unwind(|| audit(&root)).is_err());
            fs::remove_dir_all(&root).expect("remove temporary input");
        }
    }
}
