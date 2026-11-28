//! L2 falsification, collection part: Run50's deferred-pairs collection, continued
//! past the rank stop to PERMILLE/1000 x the rank-stop source count, without descent
//! (MAX_DESCENT_RESIDUALS = 1, as l1-steps-2-3's "without descent" run).
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l2-anchor-trajectory.
//! Everything here is [pcc:unverified].
//!
//! The worker body is ../l1-steps-2-3/run50_deferred.rs (itself the trimmed
//! "deferred-pairs" arm of Run50's driver) with the same JV_* pinning, one-thread
//! pool, bitops claim, Phase::Model around the transfer and the same options from
//! Run50's argv. The only science change is in the collector copy made by
//! build/patch_l2.py (continue past the rank stop; per-batch records).
//!
//! Control (before any post-stop number is used): the rank-stop snapshot must equal
//! Run50's recorded pair-deferred outcome (run-study.json) in counts, graph stats,
//! sources, projections, screen calls/refusals, split records, the rank check and
//! the relations / factor-base / rank-check bitops, the last checkpoint must equal
//! l1-steps-2-3's recorded cohort hash, and the anchors activated before the stop
//! must equal l1-steps-2-3's D_exact export key for key. The program exits nonzero
//! if any control fails, before writing the trajectory.
//!
//! D_exact at a checkpoint = graph anchors whose activation source index is below
//! the checkpoint's source count (anchors are never removed). Every anchor witness
//! is replayed for integer incidence over the source certificates.
//!
//! Run: l2_collect PUBLIC_JSON SEED PERMILLE RUN_STUDY_JSON L1_CONTROL_JSON L1_DEXACT_JSONL OUTDIR
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, E2, E6, PtE6},
};
use crypto_lib::run50::l2_deferred_pair_dlp::{
    run_extension_deferred_pair_dlp_on, DeferredPairDlpOptions, L2_EXTEND_PERMILLE, L2_LOG,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::OpenOptions,
    io::{BufRead, BufWriter, Write},
    time::Instant,
};

fn e2(v: &Value) -> Result<E2, String> {
    let a = v.as_array().ok_or("e2")?;
    Ok(E2([a[0].as_u64().ok_or("e2")?, a[1].as_u64().ok_or("e2")?]))
}
fn e6(v: &Value) -> Result<E6, String> {
    let a = v.as_array().ok_or("e6")?;
    Ok(E6([e2(&a[0])?, e2(&a[1])?, e2(&a[2])?]))
}
fn pt(v: &Value) -> Result<PtE6, String> {
    Ok(PtE6 { x: e6(&v["x"])?, y: e6(&v["y"])?, inf: v["inf"].as_bool().ok_or("inf")? })
}
fn create(path: &str) -> Result<BufWriter<std::fs::File>, String> {
    Ok(BufWriter::new(
        OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| format!("{path}: {e}"))?,
    ))
}
fn key6(v: &Value) -> Result<[u64; 6], String> {
    let a = v.as_array().ok_or("key")?;
    let mut k = [0u64; 6];
    for (i, x) in a.iter().enumerate() {
        k[i] = x.as_u64().ok_or("key")?;
    }
    Ok(k)
}
/// numeric JSON values may be numbers or decimal strings (arbitrary_precision)
fn num(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 8 {
        return Err("usage: l2_collect PUBLIC_JSON SEED PERMILLE RUN_STUDY_JSON L1_CONTROL_JSON L1_DEXACT_JSONL OUTDIR".into());
    }
    let read = |p: &str| -> Result<Value, String> {
        serde_json::from_slice(&std::fs::read(p).map_err(|e| format!("{p}: {e}"))?).map_err(|e| e.to_string())
    };
    let public = read(&args[1])?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let permille: u64 = args[3].parse().map_err(|_| "permille")?;
    let study = read(&args[4])?;
    let l1_control = read(&args[5])?;
    let out = &args[7];
    let overrides: Vec<_> = std::env::vars_os()
        .map(|(k, _)| k)
        .filter(|k| k.to_str().is_some_and(|s| s.starts_with("JV_")))
        .collect();
    for key in overrides {
        std::env::remove_var(key);
    }
    std::env::set_var("JV_ADDS_INC", "1");
    std::env::set_var("JV_ADDS_SHARE", "1");
    std::env::set_var("JV_SIEVE_PROGRESS", "1");
    L2_EXTEND_PERMILLE.store(permille, std::sync::atomic::Ordering::SeqCst);
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let (alpha, g, q) = (e6(&public["alpha"])?, pt(&public["g"])?, pt(&public["q"])?);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    let (report, elapsed) = pool.install(|| -> Result<_, String> {
        work::claim();
        work::set_moduli(p, l);
        work::enable(true);
        let started = Instant::now();
        let model = work::enter(Phase::Model);
        let spec = cover::spec_from_curve(p, alpha, l, g, q, 0).ok_or("public transfer failed")?;
        let mut options = DeferredPairDlpOptions::default();
        options.collection.collection.b_count = Some(1018081);
        options.collection.collection.max_descent_residuals = 1;
        options.collection.collection.max_accepted_descents = 32;
        options.collection.collection.max_split_records = 524288;
        options.max_deferred_edges = 262144;
        let (report, _scalar) = run_extension_deferred_pair_dlp_on(&spec, seed, options)?;
        drop(model);
        work::enable(false);
        Ok((report, started.elapsed().as_nanos()))
    })?;
    let log: Vec<Value> = std::mem::take(&mut *L2_LOG.lock().map_err(|_| "log")?);
    let coll = &report["collection"];
    let sources = coll["sources"].as_array().ok_or("sources")?;
    // ---------------- control at the rank stop ----------------
    let rs = log.iter().find(|r| r["kind"] == "rank_stop").ok_or("no rank stop reached")?;
    let s0 = rs["sources"].as_u64().ok_or("s0")? as usize;
    let rec = study["outcomes"]
        .as_array()
        .ok_or("outcomes")?
        .iter()
        .find(|o| o["mode"] == "pair-deferred")
        .ok_or("pair-deferred outcome")?;
    let mut checks: BTreeMap<String, Value> = BTreeMap::new();
    let mut eq = |name: &str, local: Value, recorded: Value| {
        checks.insert(name.into(), json!({"equal":local==recorded,"local":local,"recorded":recorded}));
    };
    eq("counts", rs["counts"].clone(), rec["counts"].clone());
    eq("graph_stats", rs["graph_stats"].clone(), rec["graph_stats"].clone());
    eq("sources", rs["sources"].clone(), rec["sources"].clone());
    eq("projections", rs["projections"].clone(), rec["projections"].clone());
    eq("screen_calls", rs["screen_calls"].clone(), rec["screen_calls"].clone());
    eq("screen_refusals_vs_early_refusals", rs["screen_refusals"].clone(), rec["early_refusals"].clone());
    eq("splits_vs_slow_split_records", rs["splits"].clone(), rec["slow_split_records"].clone());
    eq("stop", rs["stop"].clone(), rec["stop"].clone());
    let rc_local: Vec<Value> = rs["rank_checks"].as_array().ok_or("rank_checks")?.iter()
        .map(|c| json!([num(&c["b_count"]), num(&c["rows"]), num(&c["result"]["rank_certificate"]), num(&c["native"]["observed_incomplete_subtotal"])])).collect();
    let rc_rec: Vec<Value> = rec["rank_checks"].as_array().ok_or("rank_checks")?.iter()
        .map(|c| json!([num(&c["b_count"]), num(&c["rows"]), num(&c["rank_certificate"]), num(&c["observed_incomplete_bitops"])])).collect();
    eq("rank_checks", json!(rc_local), json!(rc_rec));
    let rec_phase = |name: &str| -> Value {
        rec["phases"].as_array().and_then(|a| a.iter().find(|x| x["phase"] == name)).map(|x| json!(num(&x["observed_incomplete_bitops"]))).unwrap_or(Value::Null)
    };
    eq("relations_bitops", rs["phase_bitops"]["relations"].clone(), rec_phase("relations"));
    eq("factor_base_bitops", rs["phase_bitops"]["factor_base"].clone(), rec_phase("factor_base"));
    eq("linear_algebra_bitops_at_stop_vs_rank_check", rs["phase_bitops"]["linear_algebra"].clone(), json!(rc_rec.last().map(|v| v[3].clone())));
    let l1_last = &l1_control["arms"]["local_full_max_descent_residuals_8192"]["last_checkpoint"];
    eq("last_checkpoint_vs_l1", rs["last_checkpoint"].clone(), l1_last.clone());
    // ---------------- D_exact export with integer-incidence replay ----------------
    let rem_of = |id: usize| -> Result<Vec<([u64; 6], i64)>, String> {
        sources.get(id).ok_or("witness source range")?["certificate"]["remainders"].as_array().ok_or("remainders")?
            .iter().map(|r| Ok((key6(&r["key"])?, r["coefficient"].as_i64().ok_or("coef")?))).collect()
    };
    let terms_of = |id: usize| -> Result<Vec<(usize, i64)>, String> {
        sources.get(id).ok_or("witness source range")?["certificate"]["terms"].as_array().ok_or("terms")?
            .iter().map(|t| Ok((t[0].as_u64().ok_or("t")? as usize, t[1].as_i64().ok_or("t")?))).collect()
    };
    let mut anchors: BTreeMap<[u64; 6], (usize, Vec<(usize, i64)>, Vec<(usize, i64)>)> = BTreeMap::new();
    let (mut replay_ok, mut replay_bad) = (0u64, 0u64);
    for (sid, s) in sources.iter().enumerate() {
        for ev in s["graph"].as_array().ok_or("graph")? {
            if ev["kind"] != "anchor" {
                continue;
            }
            let key = key6(&ev["class"])?;
            let witness: Vec<(usize, i64)> = ev["witness"].as_array().ok_or("witness")?.iter()
                .map(|t| Ok((t[0].as_u64().ok_or("w")? as usize, t[1].as_i64().ok_or("w")?)))
                .collect::<Result<_, String>>()?;
            let mut large: BTreeMap<[u64; 6], i64> = BTreeMap::new();
            let mut small: BTreeMap<usize, i64> = BTreeMap::new();
            for &(id, c) in &witness {
                for (rk, rc) in rem_of(id)? {
                    *large.entry(rk).or_default() += c * rc;
                }
                for (i, sgn) in terms_of(id)? {
                    *small.entry(i).or_default() += c * sgn;
                }
            }
            large.retain(|_, v| *v != 0);
            small.retain(|_, v| *v != 0);
            if large.len() == 1 && large.get(&key) == Some(&1) && witness.iter().all(|&(id, _)| id <= sid) {
                replay_ok += 1;
            } else {
                replay_bad += 1;
            }
            if anchors.insert(key, (sid, witness, small.into_iter().collect())).is_some() {
                return Err("class anchored twice".into());
            }
        }
    }
    // D_exact at the rank stop vs l1-steps-2-3's export
    let mut l1_keys = BTreeSet::new();
    let f = std::fs::File::open(&args[6]).map_err(|e| e.to_string())?;
    for line in std::io::BufReader::new(f).lines() {
        let v: Value = serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        l1_keys.insert(key6(&v["class"])?);
    }
    let at_stop: BTreeSet<[u64; 6]> = anchors.iter().filter(|(_, a)| a.0 < s0).map(|(k, _)| *k).collect();
    eq("d_exact_at_rank_stop_vs_l1_export", json!({"len":at_stop.len(),"same_set":at_stop==l1_keys}), json!({"len":l1_keys.len(),"same_set":true}));
    let control_pass = checks.values().all(|c| c["equal"] == true) && replay_bad == 0;
    // activation counts per source, for the trajectory cross-check
    let mut by_source = vec![0u64; sources.len() + 1];
    for (_, (sid, _, _)) in &anchors {
        by_source[*sid + 1] += 1;
    }
    for i in 1..by_source.len() {
        by_source[i] += by_source[i - 1];
    }
    let mut traj = Vec::new();
    let mut cross_bad = 0u64;
    for r in log.iter().filter(|r| r["kind"] == "batch") {
        let s = r["sources"].as_u64().ok_or("s")? as usize;
        let a = r["graph_stats"]["anchors"].as_u64().ok_or("a")?;
        if by_source[s] != a {
            cross_bad += 1;
        }
        traj.push(json!({"b_count":r["b_count"],"sources":s,"d_exact":a,"rank_ready":r["rank_ready"],"projections":r["projections"],
            "vertices":r["graph_stats"]["vertices"],"live_deferred":r["graph_stats"]["live_deferred"],"one_class_anchors":r["graph_stats"]["one_class_anchors"],
            "two_class_anchors":r["graph_stats"]["two_class_anchors"],"vertex_refusals":r["graph_stats"]["vertex_refusals"],"edge_refusals":r["graph_stats"]["edge_refusals"],
            "witness_refusals":r["graph_stats"]["witness_refusals"],"relations_bitops":r["phase_bitops"]["relations"],"factor_base_bitops":r["phase_bitops"]["factor_base"],
            "linear_algebra_bitops":r["phase_bitops"]["linear_algebra"],"descent_bitops":r["phase_bitops"]["descent"]}));
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    {
        let mut w = create(&format!("{out}/raw.json"))?;
        serde_json::to_writer(&mut w, &json!({"method":"deferred-pairs-l2-extended","seed":seed,"permille":permille,"work":report})).map_err(|e| e.to_string())?;
        w.flush().map_err(|e| e.to_string())?;
    }
    {
        let mut w = create(&format!("{out}/dexact.jsonl"))?;
        for (key, (sid, witness, small)) in &anchors {
            writeln!(w, "{}", json!({"class":key,"activation_source":sid,"witness":witness,"small":small})).map_err(|e| e.to_string())?;
        }
        w.flush().map_err(|e| e.to_string())?;
    }
    let summary = json!({
        "schema":"l2-anchor-trajectory.collection/v1","pcc":"unverified","seed":seed,"permille":permille,
        "control":{"pass":control_pass,"checks":checks,"anchor_witness_replay_ok":replay_ok,"anchor_witness_replay_bad":replay_bad},
        "rank_stop":{"b_count":rs["b_count"],"sources":s0,"d_exact":at_stop.len(),"phase_bitops":rs["phase_bitops"]},
        "final":{"stop":coll["stop"],"counts":coll["counts"],"graph_stats":coll["graph_stats"],"sources":sources.len(),"d_exact":anchors.len()},
        "trajectory_cross_check_bad":cross_bad,
        "inner_wall_ns_practicality_only":elapsed.to_string(),
        "trajectory":traj,
    });
    let mut w = create(&format!("{out}/trajectory.json"))?;
    writeln!(w, "{}", serde_json::to_string_pretty(&summary).unwrap()).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    println!("{}", json!({"control_pass":control_pass,"s0":s0,"d_exact_stop":at_stop.len(),"final_sources":sources.len(),"final_d_exact":anchors.len(),"replay_bad":replay_bad,"cross_bad":cross_bad,"stop":coll["stop"]}));
    if !control_pass {
        return Err("CONTROL FAILED: rank-stop snapshot differs from Run50; see trajectory.json control".into());
    }
    Ok(())
}
