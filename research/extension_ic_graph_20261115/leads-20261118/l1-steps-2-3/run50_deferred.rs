//! L1 step 3, part 1: rerun Run50's deferred-pairs collection locally and export
//! the anchored canonical classes at the kernel checkpoint (D_exact).
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1-steps-2-3.
//! Everything here is [pcc:unverified].
//!
//! The worker body is the "deferred-pairs" arm of Run50's driver
//! (examples/jv_sieve_deferred_pair_compare.rs at 11c9a031, fn worker), trimmed
//! to that one method: the same JV_* environment pinning, a one-thread rayon
//! pool, jv_bitops claim/set_moduli/enable, Phase::Model around the public
//! transfer, and the same DeferredPairDlpOptions construction from Run50's argv
//! (b_count 1018081, max_descent_residuals, max_accepted_descents 32,
//! pair_split_cap 524288, deferred_edge_cap 262144). The scientific modules are
//! the byte-copied 11c9a031 files (crypto_lib::run50) over the Root41-equivalent
//! fcda1e22 library closure.
//!
//! "Without descent": pass MAX_DESCENT_RESIDUALS = 1 to stop after one residual.
//! Passing 8192 (Run50's value) reruns the full arm, which is used here as the
//! reproduction control (3,415 residuals, scalar recovery).
//!
//! D_exact: every graph event of kind "anchor" in collection.sources[].graph.
//! Collection stops at the complete batch where the sparse kernel is certified,
//! so the anchors present then are the anchors at the kernel checkpoint. Each
//! anchor is the canonical key [u0,u1,v0re,v0im,v1re,v1im] with witness w such
//! that sum_j w_j * source_j has large part exactly +D (graph invariant), so its
//! discrete log is fixed by the certified kernel. The export also replays each
//! anchor witness against the source certificates (integer incidence: large
//! part equals +D exactly, every other class cancels).
//!
//! Run: run50_deferred PUBLIC_JSON SEED MAX_DESCENT_RESIDUALS RAW_OUT DEXACT_OUT SUMMARY_OUT
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, EllE, Fq3, PtE6, E2, E6},
};
use crypto_lib::run50::extension_deferred_pair_dlp::{
    run_extension_deferred_pair_dlp_on, DeferredPairDlpOptions,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{BufWriter, Write},
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

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        return Err("usage: run50_deferred PUBLIC_JSON SEED MAX_DESCENT_RESIDUALS RAW_OUT DEXACT_OUT SUMMARY_OUT".into());
    }
    let public: Value = serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let max_descent_residuals: u64 = args[3].parse().map_err(|_| "max descent residuals")?;
    // As Run50's driver: clear JV_* overrides, pin the incidence/shared recipe.
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
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let (alpha, g, q) = (e6(&public["alpha"])?, pt(&public["g"])?, pt(&public["q"])?);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    let (report, scalar, verified, elapsed, phase_rows) = pool.install(|| -> Result<_, String> {
        work::claim();
        work::set_moduli(p, l);
        work::enable(true);
        let before = work::snapshot();
        let started = Instant::now();
        let model = work::enter(Phase::Model);
        let spec = cover::spec_from_curve(p, alpha, l, g, q, 0).ok_or("public transfer failed")?;
        let mut options = DeferredPairDlpOptions::default();
        options.collection.collection.b_count = Some(1018081);
        options.collection.collection.max_descent_residuals = max_descent_residuals;
        options.collection.collection.max_accepted_descents = 32;
        options.collection.collection.max_split_records = 524288;
        options.max_deferred_edges = 262144;
        let (report, scalar) = run_extension_deferred_pair_dlp_on(&spec, seed, options)?;
        drop(model);
        let verified = {
            let _v = work::enter(Phase::Verification);
            let f = Fq3::new(p);
            let ec = EllE::new(&f, &alpha);
            scalar.is_some_and(|d| d > 0 && d < l && ec.mul(&g, d) == q)
        };
        let elapsed = started.elapsed().as_nanos();
        let tally = work::snapshot().since(&before);
        let widths = work::widths(p, l);
        let phase_rows: Vec<_> = Phase::IC
            .iter()
            .map(|&phase| {
                json!({"phase":phase.name(),"diagnostic_priced_bitops":work::price(tally.phase(phase),&widths).expect("known widths").to_string()})
            })
            .collect();
        work::enable(false);
        Ok((report, scalar, verified, elapsed, phase_rows))
    })?;
    {
        let mut w = create(&args[4])?;
        serde_json::to_writer(&mut w, &json!({"method":"deferred-pairs","seed":seed,"actual_scalar":scalar,"verified":verified,"work":report}))
            .map_err(|e| e.to_string())?;
        w.flush().map_err(|e| e.to_string())?;
    }
    // ---- D_exact export with an independent integer-incidence replay ----
    let coll = &report["collection"];
    let sources = coll["sources"].as_array().ok_or("sources")?;
    // remainders per source: canonical key -> signed coefficient
    let rem_of = |id: usize| -> Result<Vec<([u64; 6], i64)>, String> {
        let s = sources.get(id).ok_or("witness source range")?;
        s["certificate"]["remainders"]
            .as_array()
            .ok_or("remainders")?
            .iter()
            .map(|r| {
                let k = r["key"].as_array().ok_or("key")?;
                let mut key = [0u64; 6];
                for (i, x) in k.iter().enumerate() {
                    key[i] = x.as_u64().ok_or("key")?;
                }
                Ok((key, r["coefficient"].as_i64().ok_or("coef")?))
            })
            .collect()
    };
    let mut anchors: BTreeMap<[u64; 6], (usize, Vec<(usize, i64)>)> = BTreeMap::new();
    let (mut replay_ok, mut replay_bad) = (0u64, 0u64);
    for (sid, s) in sources.iter().enumerate() {
        for ev in s["graph"].as_array().ok_or("graph")? {
            if ev["kind"] != "anchor" {
                continue;
            }
            let k = ev["class"].as_array().ok_or("class")?;
            let mut key = [0u64; 6];
            for (i, x) in k.iter().enumerate() {
                key[i] = x.as_u64().ok_or("class")?;
            }
            let witness: Vec<(usize, i64)> = ev["witness"]
                .as_array()
                .ok_or("witness")?
                .iter()
                .map(|t| Ok((t[0].as_u64().ok_or("w")? as usize, t[1].as_i64().ok_or("w")?)))
                .collect::<Result<_, String>>()?;
            // integer incidence: sum_j w_j * large(source_j) == +key
            let mut large: BTreeMap<[u64; 6], i64> = BTreeMap::new();
            for &(id, c) in &witness {
                for (rk, rc) in rem_of(id)? {
                    *large.entry(rk).or_default() += c * rc;
                }
            }
            large.retain(|_, v| *v != 0);
            if large.len() == 1 && large.get(&key) == Some(&1) && witness.iter().all(|&(id, _)| id <= sid) {
                replay_ok += 1;
            } else {
                replay_bad += 1;
            }
            if anchors.insert(key, (sid, witness)).is_some() {
                return Err("class anchored twice".into());
            }
        }
    }
    {
        let mut w = create(&args[5])?;
        for (key, (sid, witness)) in &anchors {
            writeln!(w, "{}", json!({"class":key,"activation_source":sid,"witness":witness})).map_err(|e| e.to_string())?;
        }
        w.flush().map_err(|e| e.to_string())?;
    }
    let summary = json!({
        "schema":"l1-steps-2-3.run50-rerun/v1","pcc":"unverified","seed":seed,"max_descent_residuals":max_descent_residuals,
        "actual_scalar":scalar,"verified":verified,"inner_wall_ns_practicality_only":elapsed.to_string(),
        "counts":coll["counts"],"graph_stats":coll["graph_stats"],"stop":coll["stop"],"rank_checks":coll["rank_checks"],
        "checkpoints_len":coll["checkpoints"].as_array().map(|a|a.len()),
        "last_checkpoint":coll["checkpoints"].as_array().and_then(|a|a.last()).cloned(),
        "projections":coll["projections"].as_array().map(|a|a.len()),
        "descent_residuals":report["descent_residuals"],"accepted_descents":report["descents"].as_array().map(|a|a.len()),
        "collection_rank_ready":report["collection_rank_ready"],
        "screen_calls":coll["discriminant_screen"]["calls"],"screen_refusals":coll["discriminant_screen"]["refusals"].as_array().map(|a|a.len()),
        "h":coll["h"],"base_len":coll["base"].as_array().map(|a|a.len()),
        "d_exact_classes":anchors.len(),"anchor_witness_replay_ok":replay_ok,"anchor_witness_replay_bad":replay_bad,
        "phases":phase_rows,
    });
    let mut w = create(&args[6])?;
    writeln!(w, "{}", serde_json::to_string_pretty(&summary).unwrap()).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string(&json!({"scalar":scalar,"verified":verified,"d_exact":anchors.len(),"replay_bad":replay_bad,"counts":coll["counts"],"graph_stats":coll["graph_stats"]})).unwrap());
    Ok(())
}
