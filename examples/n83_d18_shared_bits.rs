//! Exact construction counts for the retained D18 representative set.
//! No solver search or timing comparison is performed.
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    frobenius_orbit_domain_size_with_encoding, ChainedS3Encoding, FrobeniusOrbitRootEncoding,
};
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityStatement, Verdict,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

const CSV_SHA: &str = "534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4";
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn representatives(bytes: &[u8]) -> Result<Vec<BigUint>> {
    if format!("{:x}", Sha256::digest(bytes)) != CSV_SHA {
        return Err("selected representative CSV differs".into());
    }
    let mut lines = std::str::from_utf8(bytes)?.lines();
    if lines.next() != Some("orbit_key_decimal,source_x_decimal,source_x_multiplicity") {
        return Err("CSV header differs".into());
    }
    let mut reps = Vec::new();
    let mut keys = BTreeSet::new();
    let mut sources = BTreeSet::new();
    for line in lines {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 3 {
            return Err("CSV width differs".into());
        }
        let key: u128 = fields[0].parse()?;
        let source: u32 = fields[1].parse()?;
        let multiplicity: u32 = fields[2].parse()?;
        if key >= 1 << 83
            || source >= 1 << 18
            || multiplicity == 0
            || !keys.insert(key)
            || !sources.insert(source)
        {
            return Err("CSV value or uniqueness contract differs".into());
        }
        reps.push(BigUint::from(source));
    }
    if reps.len() != 16_627 {
        return Err("D18 class count differs".into());
    }
    Ok(reps)
}

fn run(input: &Path, output: &Path) -> Result<()> {
    if output.exists() {
        return Err("output already exists".into());
    }
    let reps = representatives(&fs::read(input)?)?;
    let mut roots_digest = blake3::Hasher::new();
    roots_digest.update(b"n83.root-representatives/v1");
    for rep in &reps {
        let x: u128 = rep.to_string().parse()?;
        roots_digest.update(&x.to_le_bytes());
    }
    let curve = KoblitzCurve::known_n83_k0().ok_or("N83 fixture absent")?;
    if curve.label() != CURVE
        || curve.cofactor != BigUint::from(4u8)
        || curve.subgroup_order.to_string() != "2417851639230796216685689"
    {
        return Err("curve identity differs".into());
    }
    let shared: Vec<_> = (0..83u64)
        .filter_map(|bit| {
            let value = reps[0].bit(bit);
            reps.iter()
                .all(|r| r.bit(bit) == value)
                .then_some(json!({"bit":bit,"value":value}))
        })
        .collect();
    // Public corpus point zero. This fixes construction, but launches no query.
    let x = BigUint::parse_bytes(b"1834913121178197242856238", 10).ok_or("public x")?;
    let target_x = F2mElement::from_biguint(&x, 83);
    let m = Expr::var("m");
    let k = Expr::var("k");
    let v = Expr::var("v");
    let c = Expr::var("c");
    let classes = Expr::plus(k.clone(), Expr::Const(1));
    let statement = IdentityStatement {
        name: "Shared-root implication clause count, K=k+1".into(),
        source: "research/n83_d18_shared_bits_20261020/lean/SharedRootBits.lean#N83SharedRootBits.clauseCount".into(),
        variables: ["m", "k", "v", "c"].into_iter().map(str::to_owned).collect(),
        degree_bound: 3,
        lhs: Expr::product(vec![m.clone(), classes.clone(), Expr::plus(v.clone(), c.clone())]),
        rhs: Expr::plus(
            Expr::times(m.clone(), Expr::plus(Expr::times(classes, v), c.clone())),
            Expr::product(vec![m, c, k]),
        ),
    };
    let certificate = issue(statement, 2026102001, 32,
        "Native issuer and extra-seed replay in the same process; separate checked Lean theorem over Nat.",
        None).map_err(|e| format!("identity issue: {e:?}"))?;
    let verdict = check_with_extra(&certificate, Some((2026102002, 32)))
        .map_err(|e| format!("identity replay: {e:?}"))?;
    if !matches!(verdict, Verdict::Accept { .. }) {
        return Err("clause identity rejected".into());
    }
    let mut rows = Vec::new();
    for m in 2..=4 {
        let mut prior = None;
        for (name, encoding) in [
            (
                "per_representative",
                FrobeniusOrbitRootEncoding::PerRepresentative,
            ),
            ("shared_bits", FrobeniusOrbitRootEncoding::SharedBits),
        ] {
            let predicted = frobenius_orbit_domain_size_with_encoding(83, m, &reps, encoding)
                .ok_or("domain preflight rejected")?;
            let mut model = ChainedS3Encoding::build(
                m,
                &curve.curve.irreducible,
                &curve.curve.b,
                Some(&target_x),
                0,
                300_000,
            )
            .map_err(|e| format!("chain preflight: {e:?}"))?;
            let before = (
                model.solver.n_vars(),
                model.solver.n_clauses(),
                model.solver.n_xors(),
            );
            let actual = model
                .constrain_frobenius_orbit_summands_with_encoding(
                    &reps, 300_000, 8_000_000, encoding,
                )
                .map_err(|e| format!("domain installation: {e:?}"))?;
            let after = (
                model.solver.n_vars(),
                model.solver.n_clauses(),
                model.solver.n_xors(),
            );
            if actual != predicted
                || u64::from(after.0 - before.0) != predicted.variables
                || after.1 - before.1 != predicted.clauses
                || after.2 - before.2 != predicted.xor_rows
            {
                return Err("actual construction differs from preflight".into());
            }
            let wires = model.orbit_choice_wires().ok_or("choice wires absent")?;
            if let Some((base_counts, base_wires)) = prior.as_ref() {
                let (variables, clauses, xors): (u32, usize, usize) = *base_counts;
                if variables != after.0
                    || xors != after.2
                    || base_wires != wires
                    || clauses.checked_sub(after.1) != Some(m * shared.len() * (reps.len() - 1))
                {
                    return Err("baseline/candidate structural equivalence differs".into());
                }
            } else {
                prior = Some((after, wires.to_vec()));
            }
            rows.push(json!({
                "m":m,"root_encoding":name,
                "domain_variables":predicted.variables,"domain_clauses":predicted.clauses,
                "domain_xor_rows":predicted.xor_rows,"total_variables":after.0,
                "total_clauses":after.1,"total_xor_rows":after.2,
                "actual_equals_preflight":true
            }));
        }
    }
    let receipt = json!({
        "schema":"n83.d18-shared-root-construction/v1","status":"PASS",
        "curve_id":CURVE,"representatives_sha256":CSV_SHA,"columns":reps.len(),
        "root_representatives_blake3":roots_digest.finalize().to_hex().to_string(),
        "object_blake3":"e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
        "public_corpus_sha256":"2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5",
        "target_x":x.to_string(),"shared_bits":shared,
        "source_module_sha256":format!("{:x}",Sha256::digest(include_bytes!("../src/cryptanalysis/binary_semaev_chain_sat.rs"))),
        "source_example_sha256":format!("{:x}",Sha256::digest(include_bytes!("n83_d18_shared_bits.rs"))),
        "rows":rows,"search_performed":false,"timing_comparison":null,
        "count_identity":certificate,"count_identity_extra_points":32,
        "candidate_id":null,"selected_best_total_runtime":null
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &receipt)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 8 && args[1] == "control-pair" {
        let paths: Vec<_> = args[3..7].iter().map(Path::new).collect();
        let bytes: Vec<_> = paths.iter().map(fs::read).collect::<std::io::Result<_>>()?;
        let values: Vec<Value> = bytes
            .iter()
            .map(|b| serde_json::from_slice(b))
            .collect::<std::result::Result<_, _>>()?;
        check_control_pair(&args[2], &values, &bytes)?;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[7])?;
        serde_json::to_writer_pretty(
            &mut output,
            &json!({
                "schema":"n83.shared-root-control-pair/v1","status":"PASS",
                "comparison":args[2],"arm":values[0]["arm"],
                "input_blake3":bytes.iter().map(|b| blake3::hash(b).to_hex().to_string()).collect::<Vec<_>>(),
                "matched_assignment_and_relation":true,"natural_query_performed":false,
                "candidate_id":null,"selected_best_total_runtime":null
            }),
        )?;
        output.write_all(b"\n")?;
        output.sync_all()?;
        return Ok(());
    }
    if args.len() != 3 {
        return Err("usage: n83_d18_shared_bits SELECTED_CSV NEW_RECEIPT_JSON | control-pair aa|ab A_JSON B_JSON A_REPLAY B_REPLAY NEW_RECEIPT_JSON".into());
    }
    run(Path::new(&args[1]), Path::new(&args[2]))
}

fn check_control_pair(kind: &str, values: &[Value], bytes: &[Vec<u8>]) -> Result<()> {
    if !matches!(kind, "aa" | "ab") || values.len() != 4 || bytes.len() != 4 {
        return Err("control pair arity or kind differs".into());
    }
    for i in 0..2 {
        let w = &values[i];
        let r = &values[i + 2];
        let encoding = if kind == "ab" && i == 1 {
            "shared-bits"
        } else {
            "per-representative"
        };
        if w["schema"] != "n83.small-m4-worker/v1"
            || w["experiment_policy"] != "d18-shared-root-m4/v1"
            || w["root_encoding"] != encoding
            || w["xor_decided_cache"] != true
            || w["status"] != "PASS_CONSTRUCTED_PENDING_REPLAY"
            || w["mode"] != "positive"
            || w["m"] != 4
            || w["target_index"] != 0
            || w["curve_id"] != CURVE
            || !matches!(w["arm"].as_str(), Some("U" | "D"))
            || w["columns"] != 16_627
            || w["points"] != 2_760_082
            || w["group_verified"] != true
            || w["rank"] != 1
            || w["conflict_budget"] != 10_000
            || w["counters"]["phase"] != "Complete"
            || w.get("candidate_id") != Some(&Value::Null)
            || w.get("selected_best_total_runtime") != Some(&Value::Null)
        {
            return Err("constructed worker policy differs".into());
        }
        for key in [
            "sat_model_blake3",
            "sparse_row_blake3",
            "root_representatives_blake3",
            "source_encoding_sha256",
            "binary_sha256",
        ] {
            if !w[key]
                .as_str()
                .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
            {
                return Err(format!("worker digest absent: {key}").into());
            }
        }
        let common = w["root_shared_bit_count"]
            .as_u64()
            .filter(|n| *n < 83)
            .ok_or("shared bit count absent")?;
        let removed = if encoding == "shared-bits" {
            4 * 16_626 * common
        } else {
            0
        };
        if w["sat_variables"] != 201_598
            || w["sat_xor_rows"] != 3_901
            || w["sat_clauses"] != 5_915_238 - removed
        {
            return Err("control construction dimensions differ".into());
        }
        if r["schema"] != "n83.small-m4-general-replay/v1"
            || r["status"] != "PASS"
            || r["worker_blake3"] != blake3::hash(&bytes[i]).to_hex().as_str()
            || r["rank_lower_bound"] != 1
            || r["rows_checked"] != 4
            || r["changed_target_rejected"] != true
            || r["changed_point_rejected"] != true
            || r["changed_coefficient_rejected"] != true
        {
            return Err("control replay contract differs".into());
        }
        for key in [
            "arm",
            "curve_id",
            "mode",
            "object_blake3",
            "public_corpus_sha256",
            "point_indices",
        ] {
            if r.get(key).is_none() || r[key] != w[key] {
                return Err(format!("control replay differs: {key}").into());
            }
        }
    }
    for key in [
        "arm",
        "source_commit",
        "binary_sha256",
        "source_encoding_sha256",
        "object_blake3",
        "point_set_blake3",
        "labeled_rows_blake3",
        "public_corpus_sha256",
        "root_representatives_blake3",
        "root_shared_bit_count",
        "target",
        "sat_variables",
        "sat_xor_rows",
        "sat_model_blake3",
        "point_indices",
        "sparse_row",
        "sparse_row_blake3",
    ] {
        if values[0].get(key).is_none_or(Value::is_null) || values[0][key] != values[1][key] {
            return Err(format!("control pair differs: {key}").into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_input_is_pinned_and_changed_bytes_are_rejected() {
        let bytes = include_bytes!("../research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv");
        assert_eq!(representatives(bytes).unwrap().len(), 16_627);
        let mut changed = bytes.to_vec();
        changed[100] ^= 1;
        assert!(representatives(&changed).is_err());
    }

    #[test]
    fn control_gate_rejects_changed_assignment_and_broken_replay_binding() {
        let mut a = json!({"schema":"n83.small-m4-worker/v1",
            "experiment_policy":"d18-shared-root-m4/v1","root_encoding":"per-representative",
            "xor_decided_cache":true,"status":"PASS_CONSTRUCTED_PENDING_REPLAY","mode":"positive",
            "m":4,"target_index":0,"curve_id":CURVE,"arm":"D","columns":16627,"points":2760082,
            "group_verified":true,"rank":1,"conflict_budget":10000,"counters":{"phase":"Complete"},
            "candidate_id":null,"selected_best_total_runtime":null,"root_shared_bit_count":65,
            "sat_variables":201598,"sat_xor_rows":3901,"sat_clauses":5915238,
            "point_indices":[0,166,332,498],"target":["1","2"],"sparse_row":[[0,"1"]],
            "source_commit":"2".repeat(40)});
        for key in [
            "sat_model_blake3",
            "sparse_row_blake3",
            "root_representatives_blake3",
            "source_encoding_sha256",
            "binary_sha256",
            "object_blake3",
            "point_set_blake3",
            "labeled_rows_blake3",
            "public_corpus_sha256",
        ] {
            a[key] = json!("1".repeat(64));
        }
        let mut b = a.clone();
        b["root_encoding"] = json!("shared-bits");
        b["sat_clauses"] = json!(5915238 - 4 * 16626 * 65);
        let make_replay = |w: &Value, bytes: &[u8]| {
            let mut r = json!({"schema":"n83.small-m4-general-replay/v1","status":"PASS",
                "worker_blake3":blake3::hash(bytes).to_hex().to_string(),"rank_lower_bound":1,
                "rows_checked":4,"changed_target_rejected":true,"changed_point_rejected":true,
                "changed_coefficient_rejected":true});
            for key in [
                "arm",
                "curve_id",
                "mode",
                "object_blake3",
                "public_corpus_sha256",
                "point_indices",
            ] {
                r[key] = w[key].clone();
            }
            r
        };
        let pack = |a: Value, b: Value| {
            let ab = serde_json::to_vec(&a).unwrap();
            let bb = serde_json::to_vec(&b).unwrap();
            let ar = make_replay(&a, &ab);
            let br = make_replay(&b, &bb);
            let arb = serde_json::to_vec(&ar).unwrap();
            let brb = serde_json::to_vec(&br).unwrap();
            (vec![a, b, ar, br], vec![ab, bb, arb, brb])
        };
        let (values, bytes) = pack(a.clone(), b.clone());
        assert!(check_control_pair("ab", &values, &bytes).is_ok());
        assert!(check_control_pair("aa", &values, &bytes).is_err());
        for (key, value) in [
            ("sat_model_blake3", json!("0".repeat(64))),
            ("rank", json!(0)),
            ("status", json!("UNKNOWN_budget")),
            ("root_encoding", json!("per-representative")),
            ("sat_clauses", json!(5915238)),
        ] {
            let mut wrong = b.clone();
            wrong[key] = value;
            let (values, bytes) = pack(a.clone(), wrong);
            assert!(check_control_pair("ab", &values, &bytes).is_err(), "{key}");
        }
        let (mut values, bytes) = pack(a.clone(), b);
        values[3]["worker_blake3"] = json!("0".repeat(64));
        assert!(check_control_pair("ab", &values, &bytes).is_err());
        let (values, bytes) = pack(a.clone(), a);
        assert!(check_control_pair("aa", &values, &bytes).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn command_card_orders_all_attempts_and_stops_after_a_failed_control() {
        use std::os::unix::fs::PermissionsExt;
        use std::process::Command;
        for failure in ["none", "control", "library"] {
            let dir = std::env::temp_dir().join(format!(
                "n83-shared-card-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir_all(dir.join("bin")).unwrap();
            fs::create_dir(dir.join("build")).unwrap();
            for name in [
                "lib-exit.txt",
                "tests-exit.txt",
                "build-exit.txt",
                "proof-exit.txt",
            ] {
                fs::write(dir.join("build").join(name), b"0\n").unwrap();
            }
            if failure == "library" {
                fs::write(dir.join("build/lib-exit.txt"), b"1\n").unwrap();
            }
            fs::write(dir.join("source-commit.txt"), "1".repeat(40)).unwrap();
            fs::write(dir.join("fixture.txt"), b"pin").unwrap();
            fs::write(
                dir.join("inputs.sha256"),
                format!("{:x}  fixture.txt\n", Sha256::digest(b"pin")),
            )
            .unwrap();
            fs::write(dir.join("construction.json"), serde_json::to_vec(&json!({
                "schema":"n83.d18-shared-root-construction/v1","status":"PASS","columns":16627,
                "rows":vec![Value::Null;6],"search_performed":false,"shared_bits":vec![Value::Null;65],
                "root_representatives_blake3":"fixture"
            })).unwrap()).unwrap();
            fs::write(
                dir.join("run-card.sh"),
                include_bytes!("../research/n83_d18_shared_bits_20261020/run-card.sh"),
            )
            .unwrap();
            fs::write(dir.join("run-docker.sh"), r#"#!/bin/sh
set -eu
mode=$1 encoding=$2 arm=$3 target=$4 output=$5
printf '%s %s %s %s\n' "$mode" "$encoding" "$arm" "$target" >> "$N83_CARD_TEST_LOG"
mkdir "$output"
case "$mode" in
positive)
    test "$N83_FAIL_CONTROL" = 0 || exit 17
    printf '{"status":"PASS_CONSTRUCTED_PENDING_REPLAY","root_shared_bit_count":65,"root_representatives_blake3":"fixture"}\n' > "$output/worker.json" ;;
replay)
    printf '{"status":"PASS"}\n' > "$output/worker.json" ;;
natural)
    printf '{"status":"UNKNOWN_search_cap","counters":{"completed_conflicts":10000}}\n' > "$output/worker.json"
    printf '{"schema":"n83.small-m4-progress/v1","identity":{"arm":"D","mode":"natural","root_encoding":"%s","target_index":%s,"experiment_policy":"d18-shared-root-m4/v1","xor_decided_cache":true,"conflict_budget":10000},"counters":{"completed_conflicts":10000}}\n' "$encoding" "$target" > "$output/worker.json.progress.jsonl" ;;
*) exit 2 ;;
esac
"#).unwrap();
            fs::write(
                dir.join("gate"),
                r#"#!/bin/sh
set -eu
test "$#" = 7 && test "$1" = control-pair
case "$2" in aa|ab) ;; *) exit 2 ;; esac
test -s "$3" && test -s "$4" && test -s "$5" && test -s "$6"
printf '{"status":"PASS","matched_assignment_and_relation":true}\n' > "$7"
"#,
            )
            .unwrap();
            fs::write(dir.join("bin/sudo"), "#!/bin/sh\n[ \"$1\" = docker ]\n").unwrap();
            fs::write(
                dir.join("bin/lscpu"),
                "#!/bin/sh\nprintf 'orchestration fixture\\n'\n",
            )
            .unwrap();
            for path in ["gate", "bin/sudo", "bin/lscpu"] {
                fs::set_permissions(dir.join(path), fs::Permissions::from_mode(0o700)).unwrap();
            }
            let log = dir.join("calls.txt");
            let output = Command::new("sh")
                .arg(dir.join("run-card.sh"))
                .arg(&dir)
                .env(
                    "PATH",
                    format!(
                        "{}:{}",
                        dir.join("bin").display(),
                        std::env::var("PATH").unwrap()
                    ),
                )
                .env("N83_CARD_TEST_LOG", &log)
                .env(
                    "N83_FAIL_CONTROL",
                    if failure == "control" { "1" } else { "0" },
                )
                .output()
                .unwrap();
            let calls = fs::read_to_string(&log).unwrap_or_default();
            if failure == "library" {
                assert!(!output.status.success());
                assert!(calls.is_empty());
                assert!(!dir.join("runs").exists());
            } else if failure == "control" {
                assert_eq!(output.status.code(), Some(17));
                assert!(!calls.contains("natural"));
                assert!(fs::read_to_string(dir.join("card-status.txt"))
                    .unwrap()
                    .contains("FAILED_OR_INTERRUPTED"));
            } else {
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                assert_eq!(
                    calls.lines().filter(|l| l.starts_with("positive ")).count(),
                    22
                );
                assert_eq!(
                    calls.lines().filter(|l| l.starts_with("replay ")).count(),
                    22
                );
                let natural: Vec<_> = calls
                    .lines()
                    .filter(|l| l.starts_with("natural "))
                    .collect();
                let mut expected = vec![
                    "natural per-representative D 0".to_owned(),
                    "natural shared-bits D 0".to_owned(),
                    "natural shared-bits D 0".to_owned(),
                    "natural per-representative D 0".to_owned(),
                ];
                for target in 1..8 {
                    let order = if target % 2 == 1 {
                        ["shared-bits", "per-representative"]
                    } else {
                        ["per-representative", "shared-bits"]
                    };
                    for enc in order {
                        expected.push(format!("natural {enc} D {target}"));
                    }
                }
                assert_eq!(natural, expected);
                assert_eq!(
                    fs::read_to_string(dir.join("card-status.txt")).unwrap(),
                    "COMPLETE_BOUNDED\n"
                );
            }
            fs::remove_dir_all(dir).unwrap();
        }
    }
}
