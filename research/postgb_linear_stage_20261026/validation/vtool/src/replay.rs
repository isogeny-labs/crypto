//! Nonce-driven sample of the replay, and the comparison of the validator's
//! replays (run by round2/run.sh with the pinned binaries) with the recorded
//! R3 rows; writes the replay certificate.

use crate::util::*;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const K: [(&str, usize); 5] = [("jv-nagao", 3), ("gaudry-s4", 3), ("gaudry-d4", 3), ("gaudry-s5", 1), ("dense", 4)];
const TIMING: [&str; 3] = ["f4_ms", "lin_ms", "wall_ms"];

fn round2() -> PathBuf {
    PathBuf::from(REPO).join(STUDY).join("validation/round2")
}

fn nonce() -> String {
    std::fs::read_to_string(round2().join("nonce.txt")).unwrap().trim().to_string()
}

/// `vtool sample --snapshot C --out FILE`
pub fn sample(args: &[String]) {
    let snap = full_commit(&need(args, "--snapshot"));
    let n = nonce();
    assert_eq!(n.len(), 64);
    let doc = git_json(&snap, &format!("{STUDY}/runs/R3-C.json"));
    let mut picked = Vec::new();
    for (fam, k) in K {
        let mut m: Vec<(String, Value)> = doc["rows"].as_array().unwrap().iter().enumerate()
            .filter(|(_, r)| r["family"] == json!(fam))
            .map(|(si, r)| (sha_hex(format!("{n}:R3-C:{fam}:{}", r["label"].as_str().unwrap()).as_bytes()),
                            json!({"family": fam, "label": r["label"], "p": r["p"], "delta": r["delta"], "si": si})))
            .collect();
        m.sort_by(|a, b| a.0.cmp(&b.0));
        picked.extend(m.into_iter().take(k).map(|x| x.1));
    }
    let mut dlp = Vec::new();
    for (card, lin) in [("R3-D1", "sparse"), ("R3-D2", "fast")] {
        let mut ks: Vec<(String, u64, u64)> = [(53u64, 1u64), (53, 2), (101, 1), (101, 2)].iter()
            .map(|(p, s)| (sha_hex(format!("{n}:{card}:{p}:{s}").as_bytes()), *p, *s)).collect();
        ks.sort();
        dlp.push(json!({"card": card, "JV_LIN": lin, "p": ks[0].1, "seed": ks[0].2}));
    }
    let out = json!({
        "nonce": n,
        "procedure": "R3-C systems: within each family, ordered by sha256(nonce:R3-C:family:label), the first k taken. R3-D: per card, the (p, seed) with the smallest sha256(nonce:card:p:seed).",
        "k_per_family": K.iter().map(|(f, k)| (f.to_string(), json!(k))).collect::<Map<_, _>>(),
        "reached_by": "the full R3-C card command (one process, every family), so every system keeps its recorded index si and its counts are comparable exactly",
        "sampled_systems": picked,
        "dlp_instances": dlp,
    });
    write_json(&PathBuf::from(need(args, "--out")), &out);
    println!("{}", serde_json::to_string_pretty(&out["sampled_systems"]).unwrap());
    println!("{}", serde_json::to_string(&out["dlp_instances"]).unwrap());
}

fn strip_ms(v: &Value) -> Value {
    match v {
        Value::Object(o) => Value::Object(o.iter().filter(|(k, _)| !k.ends_with("_ms") && k.as_str() != "ms").map(|(k, x)| (k.clone(), strip_ms(x))).collect()),
        Value::Array(a) => Value::Array(a.iter().map(strip_ms).collect()),
        _ => v.clone(),
    }
}

/// `vtool check-replay --snapshot C --out FILE`
pub fn check(args: &[String]) {
    let snap = full_commit(&need(args, "--snapshot"));
    let sample = read_json(&round2().join("sample.json"));
    let journal: Vec<Value> = std::fs::read_to_string(round2().join("journal.jsonl")).unwrap().lines()
        .filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).unwrap()).collect();
    let mut led = Ledger::default();
    let mut runs = Vec::new();
    let sampled: BTreeSet<(String, String)> = sample["sampled_systems"].as_array().unwrap().iter()
        .map(|s| (s["family"].as_str().unwrap().to_string(), s["label"].as_str().unwrap().to_string())).collect();
    let mut sampled_seen = BTreeSet::new();
    for e in &journal {
        let name = e["name"].as_str().unwrap();
        let outs = e["outputs"].as_object().unwrap();
        let hashes_ok = outs.iter().all(|(p, h)| file_sha(&PathBuf::from(REPO).join(p)).as_deref() == h.as_str());
        let mut run = json!({"name": name, "run_by": "validator", "argv": e["argv"], "env": e["env"], "cwd": e["cwd"],
                             "binary_sha256": e["binary_sha256"], "exit_code": e["exit_code"],
                             "start_utc": e["start_utc"], "end_utc": e["end_utc"], "outputs": e["outputs"]});
        let mut ok = e["exit_code"] == json!(0) && hashes_ok;
        let json_path = outs.keys().find(|p| p.ends_with(".json")).cloned().unwrap();
        let got = read_json(&PathBuf::from(REPO).join(&json_path));
        if name.starts_with("r3c") {
            let rec = git_json(&snap, &format!("{STUDY}/runs/R3-C.json"));
            let meta_ok = ["schema", "simd", "fast_threads", "families", "reps", "per", "arms", "shapes", "primes", "skipped"].iter().all(|k| got[*k] == rec[*k]);
            let (gr, rr) = (got["rows"].as_array().unwrap(), rec["rows"].as_array().unwrap());
            let mut bad = Vec::new();
            let (mut muls_eq, mut sampled_ok) = (0usize, 0usize);
            for (si, (g, r)) in gr.iter().zip(rr).enumerate() {
                let k = (r["family"].as_str().unwrap().to_string(), r["label"].as_str().unwrap().to_string());
                let same = strip_ms(g) == strip_ms(r);
                let m_eq = g["arms"].as_array().unwrap().iter().zip(r["arms"].as_array().unwrap()).all(|(a, b)| a["muls"] == b["muls"]);
                if m_eq {
                    muls_eq += 1;
                }
                if !same {
                    bad.push(json!({"si": si, "system": [k.0, k.1]}));
                }
                if sampled.contains(&k) {
                    sampled_seen.insert(k.clone());
                    if same {
                        sampled_ok += 1;
                    }
                }
            }
            ok &= meta_ok && gr.len() == rr.len() && bad.is_empty();
            led.check("R3-C replay: every row equals the recorded row in every non-timing field (sols, muls, give-up, fallback, fast-arm counters)", ok, json!(bad));
            run["compared_with"] = json!(format!("{STUDY}/runs/R3-C.json"));
            run["matched"] = json!(format!("{}/{} systems equal in every non-timing field (n, p, delta, agree, planted, per arm: sols, muls, gave_up, fallback, and the fast arm's krylov/bm/nf/roots/readout muls, delta, dense_cols, functionals_tried, nnz, nf_rows, roots); muls equal on {}/{}; the {} nonce-sampled systems: {} equal",
                                     gr.len() - bad.len(), rr.len(), muls_eq, rr.len(), sampled.len(), sampled_ok));
            run["sampled_systems_equal"] = json!(sampled_ok);
            run["mismatching_systems"] = json!(bad);
        } else {
            let card = if name.contains("sparse") { "R3-D1" } else { "R3-D2" };
            let rec = git_json(&snap, &format!("{STUDY}/runs/{card}.json"));
            let mut diffs = Map::new();
            let mut compared = 0;
            for g in got.as_array().unwrap() {
                let r = rec.as_array().unwrap().iter().find(|r| r["p"] == g["p"] && r["seed"] == g["seed"]).unwrap();
                let fields: BTreeSet<String> = g.as_object().unwrap().keys().chain(r.as_object().unwrap().keys()).cloned().collect();
                for f in fields.iter().filter(|f| !TIMING.contains(&f.as_str())) {
                    compared += 1;
                    if g.get(f) != r.get(f) {
                        diffs.insert(f.clone(), json!([g.get(f), r.get(f)]));
                    }
                }
                run["required"] = json!({
                    "solved": [g["solved"], r["solved"]], "correct": [g["correct"], r["correct"]],
                    "lin_muls": [g["lin_muls"], r["lin_muls"]], "relations": [g["relations"], r["relations"]],
                    "s": [g["s"], r["s"]], "rho_neg_equal": g["rho_neg"] == r["rho_neg"], "rho_equal": g["rho"] == r["rho"],
                });
                run["timing_not_compared"] = json!(TIMING.iter().map(|t| (t.to_string(), json!([g[*t], r[*t]]))).collect::<Map<_, _>>());
                run["key"] = json!([g["p"], g["seed"]]);
            }
            let dump_path = outs.keys().find(|p| p.ends_with(".instances.jsonl")).cloned();
            let dump_same = dump_path.as_ref().map(|p| {
                let mine = std::fs::read_to_string(PathBuf::from(REPO).join(p)).unwrap();
                let theirs = String::from_utf8(git_show(&snap, &format!("{STUDY}/runs/{card}.instances.jsonl"))).unwrap();
                mine.lines().all(|l| theirs.lines().any(|t| t == l)) && mine.lines().count() == 1
            }).unwrap_or(false);
            ok &= diffs.is_empty() && dump_same && got.as_array().unwrap().len() == 1;
            led.check(&format!("{name}: every non-timing field equals {card} ({compared} compared), and the instance line is byte-identical"), ok,
                      Value::Object(diffs.clone()));
            run["compared_with"] = json!(format!("{STUDY}/runs/{card}.json and {card}.instances.jsonl"));
            run["matched"] = json!(format!("{} of {compared} non-timing fields differ (incl. lin_muls, relations, S, the 4 rho and 4 rho_neg runs); instance dump line byte-identical: {dump_same}", diffs.len()));
        }
        run["ok"] = json!(ok);
        runs.push(run);
    }
    let missing: Vec<_> = sampled.difference(&sampled_seen).collect();
    led.check("every nonce-sampled R3-C system was replayed", missing.is_empty(), json!(missing));
    let want_dlp: BTreeSet<String> = sample["dlp_instances"].as_array().unwrap().iter()
        .map(|d| format!("dlp_{}_p{}_s{}", d["JV_LIN"].as_str().unwrap(), d["p"], d["seed"])).collect();
    let have: BTreeSet<String> = journal.iter().map(|e| e["name"].as_str().unwrap().to_string()).collect();
    led.check("one nonce-chosen R3-D instance per arm was replayed", want_dlp.is_subset(&have), json!(want_dlp));
    // scalars
    let sv_path = round2().join("scalar-verification.json");
    let sv = read_json(&sv_path);
    let r3_ver = sv["instances"].as_array().unwrap().iter().filter(|i| (i["set"] == json!("R3-D1") || i["set"] == json!("R3-D2")) && i["verified"] == json!(true)).count();
    let own_ok = !sv["own_replay_dumps"].as_array().unwrap().is_empty() && sv["own_replay_dumps"].as_array().unwrap().iter().all(|o| o["identical"] == json!(true));
    let scalars_ok = sv["all_verified"] == json!(true) && r3_ver == 8 && own_ok;
    led.check("scalars: all 8 R3-D instances verified by the validator's own arithmetic, the validator's replay dumps identical", scalars_ok,
              json!({"r3_verified": r3_ver, "own_dumps_identical": own_ok}));
    let vtool = PathBuf::from(REPO).join(STUDY).join("validation/vtool/src");
    let mut src = BTreeMap::new();
    for f in ["main.rs", "util.rs", "scalars.rs", "recompute.rs", "replay.rs"] {
        src.insert(format!("validation/vtool/src/{f}"), json!(file_sha(&vtool.join(f))));
    }
    let status = if led.problems.is_empty() { "verified" } else { "failed" };
    let cert = json!({
        "schema": "postgb-la-replay-certificate/v2",
        "status": status,
        "independent": true,
        "independence": "replayed by the validator with the pinned R3 binaries (copies whose sha256 match the cards' binds); nonce drawn by the validator after the snapshot commit; comparison and scalar arithmetic in the validator's own Rust (validation/vtool), sharing no code with the repository",
        "scalars_verified": scalars_ok,
        "scalar_check_used": "validation/vtool verify-scalars: own F_p, F_{p^2}, F_{p^6} and curve arithmetic. For each of the 8 R3-D instances (and the 8 round-1 dumps, and the validator's own replay dumps): w a non-residue and s a non-cube (Euler and exhaustive root search), curve non-singular, G and Q on it, l prime (Miller-Rabin and trial division), G != O, l G = O, dd G = Q, dd = d; self-tests and negative controls. Ties: each dumped (p, seed, l) is its R3-D row's; R3 dumps byte-identical to the round-1 dumps and to the validator's replay dumps.",
        "scalar_verification": {"path": "validation/round2/scalar-verification.json", "sha256": file_sha(&sv_path),
                                "instances_verified": sv["instances_verified"], "instances_total": sv["instances_total"]},
        "nonce": nonce(),
        "nonce_drawn_utc": std::fs::read_to_string(round2().join("nonce.drawn_at")).unwrap().trim(),
        "snapshot_commit": snap,
        "sampling": sample,
        "pins": {"postgb_la_bench": "06351d89429a1d50d5026e9bfe99c7299e25e3d15b0024b1c0b7cda4d686444b",
                 "jv_cover": "a49184b0c25ae27538f083999459ca9c4cc88abf4f9c99cc06b007253b5feb85"},
        "tool": {"crate": "validation/vtool", "sources_sha256": src},
        "previous_round": "round 1 (snapshot e9bf8737a3) replayed R2 with Python tooling (validation/replay/, historical; superseded by this certificate)",
        "checks": led.checks,
        "problems": led.problems,
        "runs": runs,
    });
    write_json(&PathBuf::from(need(args, "--out")), &cert);
    println!("check-replay: {status}; scalars_verified {scalars_ok}; {} runs; {} problems", cert["runs"].as_array().unwrap().len(), cert["problems"].as_array().unwrap().len());
    for p in cert["problems"].as_array().unwrap() {
        let s = p.to_string();
        println!("PROBLEM {}", &s[..s.len().min(600)]);
    }
}
