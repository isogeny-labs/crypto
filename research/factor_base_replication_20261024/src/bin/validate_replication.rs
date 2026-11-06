//! Independent target-derivation, answer, cohort and aggregate validator.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

const ROOT: &str = "research/factor_base_replication_20261024/results";
const RAW: &str = "research/factor_base_replication_20261024/results/raw";
const ARCHIVE: &str = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
const REPLICATION_ID: &str = "factor-base-graph-replication/20261024/v1";
const PROTOCOL_SHA256: &str = "ace04952556bfe1cb9b1a0c8cb5dec6715040f55dd376d1e3ca3d7b659738997";
const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const ARMS: [(u64, u64); 5] = [(1, 16), (2, 16), (4, 16), (1, 8), (1, 32)];

fn rows(path: &Path) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}
fn archive_rows(bits: u64) -> BTreeMap<String, Value> {
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", ARCHIVE, &member])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "tar: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|row| row["arm"] == "genus_projective")
        .map(|row| {
            (
                row["workload"]["workload_id"].as_str().unwrap().to_owned(),
                row,
            )
        })
        .collect()
}
fn digest(domain: &str, bits: u64, workload: &Value) -> [u8; 32] {
    let mut h = Sha256::new();
    for part in [
        REPLICATION_ID.to_owned(),
        domain.to_owned(),
        bits.to_string(),
        workload["curve"]["icv1"].as_str().unwrap().to_owned(),
        workload["target_index"].as_u64().unwrap().to_string(),
    ] {
        h.update(part.as_bytes());
        h.update([0]);
    }
    h.finalize().into()
}
fn digest_u64(d: &[u8; 32]) -> u64 {
    u64::from_be_bytes(d[..8].try_into().unwrap())
}
fn digest_hex(d: &[u8]) -> String {
    d.iter().map(|b| format!("{b:02x}")).collect()
}
fn workload_digest(bits: u64, workload: &Value, planted: u64, target: Point) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(digest("workload/v1", bits, workload));
    h.update(planted.to_be_bytes());
    h.update(target.x.to_be_bytes());
    h.update(target.y.to_be_bytes());
    h.finalize().into()
}
fn curve(workload: &Value) -> (Elliptic, Point, u64) {
    let spec = &workload["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let e = Elliptic {
        f: Field::new(p, 2, spec["modulus"].as_u64().unwrap()),
        c: [
            spec["b"].as_u64().unwrap(),
            spec["a"].as_u64().unwrap(),
            0,
            1,
        ],
        binary: false,
    };
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    (e, generator, spec["r"].as_u64().unwrap())
}
fn hex_point(row: &Value) -> Point {
    Point::affine(
        u64::from_str_radix(
            row["target"][0].as_str().unwrap().trim_start_matches("0x"),
            16,
        )
        .unwrap(),
        u64::from_str_radix(
            row["target"][1].as_str().unwrap().trim_start_matches("0x"),
            16,
        )
        .unwrap(),
    )
}
fn num(v: &Value, k: &str) -> f64 {
    v[k].as_u64().unwrap() as f64
}
fn native(v: &Value) -> f64 {
    v.as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| v.as_u64().unwrap() as f64)
        .sum()
}
fn cost(row: &Value, s: f64, i: f64, l: f64) -> f64 {
    let c = &row["counts"];
    let u = &row["source_add"];
    (num(c, "m") + s * num(c, "s") + i * num(c, "i"))
        / (num(u, "m") + s * num(u, "s") + i * num(u, "i"))
        + l * (native(&row["native"]) + num(c, "field_adds") + num(c, "bit_ops"))
}
fn arm_name(d: u64, j: u64) -> String {
    format!("fb{d}-j{j}")
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}

fn main() {
    let summary: Value =
        serde_json::from_str(&fs::read_to_string(format!("{ROOT}/summary.json")).unwrap()).unwrap();
    assert_eq!(summary["protocol_sha256"], PROTOCOL_SHA256);
    let summary_map: BTreeMap<(String, String), Value> = summary["summary"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            (
                (
                    x["arm"].as_str().unwrap().to_owned(),
                    x["slice"].as_str().unwrap().to_owned(),
                ),
                x.clone(),
            )
        })
        .collect();
    let mut source_by_bits = BTreeMap::new();
    for bits in BITS {
        let r = archive_rows(bits);
        assert_eq!(r.len(), 180);
        source_by_bits.insert(bits, r);
    }
    let mut all: BTreeMap<String, Vec<(u64, Value)>> = BTreeMap::new();
    let mut canonical = BTreeSet::new();
    let mut candidate_verified = 0u64;
    let mut rho_verified = 0u64;
    let mut algorithm_seed_overlaps = 0u64;
    let mut planted_overlaps = 0u64;
    let mut target_overlaps = 0u64;
    for (d, j) in ARMS {
        let arm = arm_name(d, j);
        let mut ids = BTreeSet::new();
        for bits in BITS {
            let path = format!("{RAW}/{arm}-bits{bits}.jsonl");
            let rs = rows(Path::new(&path));
            assert_eq!(rs.len(), 180);
            for row in rs {
                assert_eq!(row["protocol_sha256"], PROTOCOL_SHA256);
                let source = &source_by_bits[&bits][row["source_workload_id"].as_str().unwrap()];
                let w = &source["workload"];
                assert_eq!(row["source_workload_sha256"], w["workload_sha256"]);
                assert_eq!(row["curve_icv1"], w["curve"]["icv1"]);
                assert_eq!(row["target_index"], w["target_index"]);
                let (e, g, r) = curve(w);
                assert!(e.valid(g));
                let td = digest("target-scalar/v1", bits, w);
                let ad = digest("algorithm-seed/v1", bits, w);
                let planted = 1 + digest_u64(&td) % (r - 1);
                let seed = digest_u64(&ad);
                let target = e.mul(g, planted);
                let wd = workload_digest(bits, w, planted, target);
                let wh = digest_hex(&wd);
                assert_eq!(row["planted"].as_u64().unwrap(), planted);
                assert_eq!(
                    row["algorithm_seed"]
                        .as_str()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    seed
                );
                assert_eq!(row["target_derivation"]["target_digest"], digest_hex(&td));
                assert_eq!(
                    row["target_derivation"]["algorithm_digest"],
                    digest_hex(&ad)
                );
                assert_eq!(row["workload_sha256"], wh);
                assert_eq!(row["workload_id"], format!("W{}", &wh[..12]));
                assert_eq!(hex_point(&row), target);
                assert_eq!(row["recovered"].as_u64().unwrap(), planted);
                assert_eq!(row["rho"]["recovered"].as_u64().unwrap(), planted);
                assert_eq!(e.mul(g, row["recovered"].as_u64().unwrap()), target);
                assert_eq!(e.mul(g, row["rho"]["recovered"].as_u64().unwrap()), target);
                assert_eq!(row["verified"], true);
                assert_eq!(row["rho"]["verified"], true);
                candidate_verified += 1;
                rho_verified += 1;
                if arm == "fb1-j16" {
                    if seed
                        == source["algorithm_seed"]
                            .as_str()
                            .unwrap()
                            .parse::<u64>()
                            .unwrap()
                    {
                        algorithm_seed_overlaps += 1;
                    }
                    if planted == w["planted"].as_u64().unwrap() {
                        planted_overlaps += 1;
                    }
                    let old = Point::affine(
                        u64::from_str_radix(
                            w["target"][0].as_str().unwrap().trim_start_matches("0x"),
                            16,
                        )
                        .unwrap(),
                        u64::from_str_radix(
                            w["target"][1].as_str().unwrap().trim_start_matches("0x"),
                            16,
                        )
                        .unwrap(),
                    );
                    if target == old {
                        target_overlaps += 1;
                    }
                }
                let id = row["workload_id"].as_str().unwrap().to_owned();
                assert!(ids.insert(id));
                all.entry(arm.clone()).or_default().push((bits, row));
            }
        }
        assert_eq!(ids.len(), 1260);
        if canonical.is_empty() {
            canonical = ids
        } else {
            assert_eq!(canonical, ids)
        }
    }
    assert_eq!(candidate_verified, 6300);
    assert_eq!(rho_verified, 6300);
    for (arm, rs) in &all {
        for slice in std::iter::once(None).chain(BITS.map(Some)) {
            let selected: Vec<_> = rs
                .iter()
                .filter(|(b, _)| slice.is_none_or(|x| x == *b))
                .collect();
            let label = slice.map_or_else(|| "all".to_owned(), |x| x.to_string());
            let expected = &summary_map[&(arm.clone(), label)];
            let mut ratios = Vec::new();
            let mut wins = 0;
            let mut losses = 0;
            let mut ties = 0;
            let mut representative = None;
            for s in [0.0, 0.5, 1.0] {
                for i in [1.0, 10.0, 100.0, 1000.0] {
                    for l in [0.0, 0.1, 1.0] {
                        let c: f64 = selected.iter().map(|(_, r)| cost(r, s, i, l)).sum();
                        let rho: f64 = selected.iter().map(|(_, r)| cost(&r["rho"], s, i, l)).sum();
                        let q = c / rho;
                        ratios.push(q);
                        if q < 1.0 {
                            wins += 1
                        } else if q > 1.0 {
                            losses += 1
                        } else {
                            ties += 1
                        };
                        if s == 0.5 && i == 10.0 && l == 0.1 {
                            representative = Some((c, rho, q));
                        }
                    }
                }
            }
            ratios.sort_by(f64::total_cmp);
            close(
                expected["candidate_over_rho_band"][0].as_f64().unwrap(),
                ratios[0],
            );
            close(
                expected["candidate_over_rho_band"][1].as_f64().unwrap(),
                ratios[35],
            );
            assert_eq!(expected["candidate_win_cells_vs_rho"], wins);
            assert_eq!(expected["candidate_loss_cells_vs_rho"], losses);
            assert_eq!(expected["candidate_tie_cells_vs_rho"], ties);
            let (c, r, q) = representative.unwrap();
            close(
                expected["representative"]["candidate_GAE"]
                    .as_f64()
                    .unwrap(),
                c,
            );
            close(expected["representative"]["rho_GAE"].as_f64().unwrap(), r);
            close(
                expected["representative"]["candidate_over_rho"]
                    .as_f64()
                    .unwrap(),
                q,
            );
        }
    }
    fs::write(format!("{ROOT}/validator-report.json"),serde_json::to_string_pretty(&json!({"schema":"factor-base-graph-replication-validator/v1","ok":true,"protocol_sha256":PROTOCOL_SHA256,"raw_records":6300,"unique_targets":canonical.len(),"candidate_verified":candidate_verified,"rho_verified":rho_verified,"derivations_recomputed":6300,"source_checks_recomputed":12600,"summary_slices_recomputed":40,"pricing_cells_per_slice":36,"independence":{"full16_targets_checked":1260,"algorithm_seed_overlaps":algorithm_seed_overlaps,"planted_scalar_overlaps":planted_overlaps,"target_point_overlaps":target_overlaps}})).unwrap()+"\n").unwrap();
}
