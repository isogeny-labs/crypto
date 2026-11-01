//! Prospective evaluator-e5 driver for LEAD-15.
//!
//! Target-size execution is licensed only by the frozen protocol in
//! `research/lead15_diem_quartic_20261101/PROTOCOL.md`. Development checks use the
//! library's historical q=5/q=7 tests; do not use this driver before freeze.

use crypto_lib::cryptanalysis::diem_quartic::{
    gen_prospective_instance, is_prime_u64, prospective_factor_base_size,
    prospective_full_harvest, replay_common_cover_model, CommonCoverWitness, IcCounters, Inst, Kf,
    ProspectiveBuildStats, ProspectiveCostReceipt, ProspectiveHarvestOutcome,
};
use crypto_lib::hash::sha256::sha256;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const BASE_SEED: u64 = 20_261_023;
const CURVES_PER_SIZE: u32 = 6;
const MAX_CANDIDATES: u64 = 200_000;
const RETAIN_FACTOR: usize = 8;
const ALLOWED_Q: [u64; 4] = [257, 2_053, 32_771, 4_194_301];
// Canonical decimal freeze of 9.967e10*(2^15/2^22)^1.408. The analyzer never
// reevaluates a platform `powf`; K4 uses the exact 0.6 multiple of this rational.
const MATCHED_HYPER_NUM: u128 = 10_755_059_589_844_453;
const MATCHED_HYPER_DEN: u128 = 100_000_000;
const K4_THRESHOLD_NUM: u128 = 32_265_178_769_533_359;
const K4_THRESHOLD_DEN: u128 = 500_000_000;

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn add_fraction(a_num: u128, a_den: u128, b_num: u128, b_den: u128) -> (u128, u128) {
    let g = gcd_u128(a_den, b_den);
    let left = a_den / g;
    let right = b_den / g;
    let num = a_num
        .checked_mul(right)
        .and_then(|v| b_num.checked_mul(left).and_then(|w| v.checked_add(w)))
        .expect("row-weight numerator overflow");
    let den = left.checked_mul(b_den).expect("row-weight denominator overflow");
    let reduce = gcd_u128(num, den);
    (num / reduce, den / reduce)
}

fn fraction_f64(num: u128, den: u128) -> f64 {
    num as f64 / den as f64
}

fn exact_cross_products(
    left_num: u128,
    left_den: u128,
    right_num: u128,
    right_den: u128,
) -> (BigUint, BigUint) {
    assert!(left_den != 0 && right_den != 0, "fraction denominator must be nonzero");
    (
        BigUint::from(left_num) * BigUint::from(right_den),
        BigUint::from(right_num) * BigUint::from(left_den),
    )
}

#[derive(Clone, Debug)]
struct RunIdentity {
    commit: String,
    protocol_sha256: String,
    binary_sha256: String,
    image_digest: String,
}

#[derive(Clone, Debug)]
struct FrozenSource {
    identity: RunIdentity,
    freeze_commit: String,
    protocol_file_sha256: String,
    analysis_source_sha256: String,
    worker_source_sha256: String,
    collector_source_sha256: String,
    build_info_sha256: String,
    cargo_lock_sha256: String,
    image_recipe_sha256: String,
    k4_reference_sha256: String,
    manifest_sha256: String,
    image_uri: String,
    jobs: HashMap<u64, FrozenJob>,
}

#[derive(Clone, Debug)]
struct FrozenJob {
    job_name: String,
    argv_template: Vec<String>,
    request_artifact_s3_uri: String,
    results_prefix: String,
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(sha256(bytes))
}

fn sha256_base64(bytes: &[u8]) -> String {
    BASE64_STANDARD.encode(sha256(bytes))
}

const TIMEOUT_TERM_EVENT: &[u8] = b"timeout: sending signal TERM to command 'bash'\n";
const TIMEOUT_KILL_EVENT: &[u8] = b"timeout: sending signal KILL to command 'bash'\n";

fn timeout_events_prove_deadline(bytes: &[u8], exit_code: i64) -> bool {
    match exit_code {
        124 => bytes == TIMEOUT_TERM_EVENT,
        137 => {
            bytes.len() == TIMEOUT_TERM_EVENT.len() + TIMEOUT_KILL_EVENT.len()
                && bytes.starts_with(TIMEOUT_TERM_EVENT)
                && bytes.ends_with(TIMEOUT_KILL_EVENT)
        }
        _ => false,
    }
}

fn timeout_events_are_canonical(bytes: &[u8]) -> bool {
    bytes.is_empty()
        || bytes == TIMEOUT_TERM_EVENT
        || (bytes.len() == TIMEOUT_TERM_EVENT.len() + TIMEOUT_KILL_EVENT.len()
            && bytes.starts_with(TIMEOUT_TERM_EVENT)
            && bytes.ends_with(TIMEOUT_KILL_EVENT))
}

fn required_lower_hex_env(name: &str, len: usize) -> String {
    let value = std::env::var(name).unwrap_or_else(|_| panic!("missing required {name}"));
    assert!(
        value.len() == len
            && value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "{name} must be {len} lowercase hexadecimal characters"
    );
    value
}

fn run_identity() -> RunIdentity {
    let image_digest = std::env::var("LEAD15_IMAGE_DIGEST")
        .expect("missing required LEAD15_IMAGE_DIGEST");
    assert!(
        image_digest
            .strip_prefix("sha256:")
            .is_some_and(|hex| is_lower_hex(hex, 64)),
        "LEAD15_IMAGE_DIGEST must be sha256:<64 lowercase hexadecimal characters>"
    );
    RunIdentity {
        commit: required_lower_hex_env("LEAD15_COMMIT", 40),
        protocol_sha256: required_lower_hex_env("LEAD15_PROTOCOL_SHA256", 64),
        binary_sha256: required_lower_hex_env("LEAD15_BINARY_SHA256", 64),
        image_digest,
    }
}

fn optional_env(name: &str) -> Value {
    std::env::var(name)
        .ok()
        .filter(|v| !v.is_empty())
        .map(Value::String)
        .unwrap_or(Value::Null)
}

fn runtime_provenance() -> Value {
    let cpu_model = fs::read_to_string("/proc/cpuinfo").ok().and_then(|text| {
        text.lines().find_map(|line| {
            let (key, value) = line.split_once(':')?;
            (key.trim() == "model name" || key.trim() == "Hardware")
                .then(|| value.trim().to_string())
        })
    });
    json!({
        "isolation": "none",
        "cpu_model": cpu_model,
        "available_parallelism": std::thread::available_parallelism().ok().map(|n| n.get()),
        "kubernetes_node": optional_env("LEAD15_NODE_NAME"),
        "kubernetes_job": optional_env("LEAD15_JOB_NAME"),
        "kubernetes_pod": std::env::var("LEAD15_POD_NAME")
            .ok()
            .or_else(|| std::env::var("HOSTNAME").ok()),
        "kubernetes_pod_uid": optional_env("LEAD15_POD_UID"),
    })
}

fn seed_for(q: u64, idx: u32, retry: u32) -> u64 {
    BASE_SEED
        ^ q.rotate_left(17)
        ^ (idx as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ (retry as u64).wrapping_mul(0x94D0_49BB_1331_11EB)
}

fn counters(c: &IcCounters) -> Value {
    json!({
        "lines": c.lines,
        "split": c.split,
        "rel_full": c.rel_full,
        "rel_fp": c.rel_slp,
        "rel_pp": c.rel_dlp,
        "duplicates": c.dups,
        "verified": c.verified,
        "verify_failed": c.verify_failed,
        "hash_lookups": c.hash_lookups,
        "hash_inserts": c.hash_inserts,
        "rng_draws": c.rng_draws,
        "uf_ops": c.uf_ops,
        "root_calls": c.root_calls,
        "normalizations": c.norm_calls,
        "fq_mul_collect": c.fq_mul_collect,
        "fq_inv_collect": c.fq_inv_collect,
        "fq_sqrt_collect": c.fq_sqrt_collect,
        "fq_mul_fb": c.fq_mul_fb,
        "fq_inv_fb": c.fq_inv_fb,
        "fq_sqrt_fb": c.fq_sqrt_fb,
        "fq_mul_verify": c.fq_mul_verify,
        "fq_inv_verify": c.fq_inv_verify,
        "fq_sqrt_verify": c.fq_sqrt_verify,
        "lp_sge_ops": c.elim_ops,
        "lp_sge_entries": c.elim_entries,
        "rank_sge_ops": c.rank_ops,
        "rank_inversions": c.rank_inversions,
        "rank_entries": c.rank_entries,
        "sort_items": c.sort_items,
    })
}

fn cost(r: &ProspectiveCostReceipt) -> Value {
    json!({
        "fq_arithmetic": r.fq_arithmetic,
        "lookup_cost": r.lookup_cost,
        "mod_ell_cost": r.mod_ell_cost,
        "native_counter_count": r.native_counter_count,
        "native_band_cost": r.native_band_cost,
        "attack_total": r.attack_total,
        "verification_cost_excluded": r.verification_cost_excluded,
        "inversion_price": r.inversion_price,
        "sqrt_price": r.sqrt_price,
        "mod_ell_price": r.mod_ell_price,
        "native_weight_gae": r.native_weight_gae,
        "gae_price": r.gae_price,
    })
}

fn add_build_stats(total: &mut ProspectiveBuildStats, one: &ProspectiveBuildStats) {
    total.candidates = total.candidates.saturating_add(one.candidates);
    total.no_common_cover = total.no_common_cover.saturating_add(one.no_common_cover);
    total.core_rejects = total.core_rejects.saturating_add(one.core_rejects);
    total.order_rejects = total.order_rejects.saturating_add(one.order_rejects);
    total.fq_muls = total.fq_muls.saturating_add(one.fq_muls);
    total.fq_invs = total.fq_invs.saturating_add(one.fq_invs);
    total.fq_sqrts = total.fq_sqrts.saturating_add(one.fq_sqrts);
    total.hash_lookups = total.hash_lookups.saturating_add(one.hash_lookups);
    total.hash_inserts = total.hash_inserts.saturating_add(one.hash_inserts);
    total.rng_draws = total.rng_draws.saturating_add(one.rng_draws);
    total.sort_items = total.sort_items.saturating_add(one.sort_items);
    total.primality_modular_ops = total
        .primality_modular_ops
        .saturating_add(one.primality_modular_ops);
}

fn build_setup_receipt(q: u64, retry: u32, stats: &ProspectiveBuildStats) -> Value {
    let inversion_price = 64 - q.leading_zeros() as u64;
    let sqrt_price = (1.5 * (q as f64).log2()).ceil() as u64;
    let fq_arithmetic = stats.fq_muls as f64
        + inversion_price as f64 * stats.fq_invs as f64
        + sqrt_price as f64 * stats.fq_sqrts as f64;
    let native_counter_count = stats
        .hash_lookups
        .saturating_add(stats.hash_inserts)
        .saturating_add(stats.rng_draws)
        .saturating_add(stats.sort_items)
        .saturating_add(stats.primality_modular_ops);
    let lookup_cost = 4.0 * stats.hash_lookups as f64;
    let gae_price = 163.0 + 2.0 * inversion_price as f64;
    let cost_receipts: Vec<Value> = [0.0, 0.1, 1.0]
        .into_iter()
        .map(|native_weight_gae| {
            let native_band_cost = native_weight_gae * gae_price * native_counter_count as f64;
            json!({
                "native_weight_gae": native_weight_gae,
                "gae_price": gae_price,
                "native_band_cost": native_band_cost,
                "emitted_total": fq_arithmetic + lookup_cost + native_band_cost,
            })
        })
        .collect();
    json!({
        "streams_attempted": retry as u64 + 1,
        "candidates_total": stats.candidates,
        "rejects_total": {
            "no_common_cover": stats.no_common_cover,
            "quartic_core": stats.core_rejects,
            "order_or_cofactor": stats.order_rejects,
            "duplicate_quartic_streams": retry,
        },
        "fq_mul": stats.fq_muls,
        "fq_inv": stats.fq_invs,
        "fq_sqrt": stats.fq_sqrts,
        "hash_lookups": stats.hash_lookups,
        "hash_inserts": stats.hash_inserts,
        "rng_draws": stats.rng_draws,
        "sort_items": stats.sort_items,
        "primality_modular_ops": stats.primality_modular_ops,
        "fq_arithmetic": fq_arithmetic,
        "lookup_cost": lookup_cost,
        "native_counter_count": native_counter_count,
        "inversion_price": inversion_price,
        "sqrt_price": sqrt_price,
        "cost_receipts": cost_receipts,
    })
}

fn instance_record(
    inst: &Inst,
    w: &CommonCoverWitness,
    idx: u32,
    retry: u32,
    seed: u64,
    accepted_stream_stats: &ProspectiveBuildStats,
    cumulative_stats: &ProspectiveBuildStats,
    identity: &RunIdentity,
) -> Value {
    json!({
        "schema": "lead15.diem-quartic/v1",
        "record": "instance",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "q": inst.q,
        "curve_index": idx,
        "stream_retry": retry,
        "seed": seed,
        "field_modulus": {"x3_a": inst.kf.a, "constant_b": inst.kf.b},
        "ub": inst.ub,
        "uc": inst.uc,
        "quartic": inst.quartic,
        "order_decimal": inst.order.to_string(),
        "cofactor": inst.h,
        "ell": inst.ell,
        "trace": inst.trace.to_string(),
        "candidate_count": accepted_stream_stats.candidates,
        "rejects": {
            "no_common_cover": accepted_stream_stats.no_common_cover,
            "quartic_core": accepted_stream_stats.core_rejects,
            "order_or_cofactor": accepted_stream_stats.order_rejects,
        },
        "builder_setup": build_setup_receipt(inst.q, retry, cumulative_stats),
        "m2_certificate": {
            "z": w.z,
            "target_ordered": w.target_ordered,
        },
        "m3_certificate": {
            "a": w.a,
            "b": w.b,
            "r": w.r,
            "r_q": w.rq,
            "source_ordered": w.source_ordered,
        },
        "common_branch_certificate": {
            "cross_ratio": w.cross_ratio,
            "mobius": w.mobius,
            "verified_all_four_points": true,
        },
        "commit": identity.commit.as_str(),
        "protocol_sha256": identity.protocol_sha256.as_str(),
        "binary_sha256": identity.binary_sha256.as_str(),
        "image_digest": identity.image_digest.as_str(),
    })
}

fn harvest_record(
    r: &ProspectiveHarvestOutcome,
    idx: u32,
    seed: u64,
    identity: &RunIdentity,
) -> Value {
    let mut costs = vec![];
    for m_ell in [1.0, 9.0] {
        for native in [0.0, 0.1, 1.0] {
            costs.push(cost(&r.complete_cost(m_ell, native)));
        }
    }
    let rho_same_instance = (std::f64::consts::PI * r.ell as f64 / 2.0).sqrt() * 74.5;
    let projected_2p15 = 7_300_000.0 + 530_000.0 * r.mean_row_weight_after_lp_sge;
    let matched_hyper_2p15 = fraction_f64(MATCHED_HYPER_NUM, MATCHED_HYPER_DEN);
    json!({
        "schema": "lead15.diem-quartic/v1",
        "record": "harvest",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "q": r.q,
        "curve_index": idx,
        "seed": seed,
        "ell": r.ell,
        "n_factor_base": r.nfb,
        "pairs": r.pairs,
        "yield": {
            "raw_full": r.raw_full,
            "raw_fp": r.raw_fp,
            "raw_pp": r.raw_pp,
            "unique_full": r.full,
            "unique_fp": r.fp,
            "unique_pp": r.pp,
            "raw_fp_per_pair": r.raw_fp as f64 / r.pairs as f64,
            "raw_pp_per_pair": r.raw_pp as f64 / r.pairs as f64,
            "unique_fp_per_pair": r.fp as f64 / r.pairs as f64,
            "unique_pp_per_pair": r.pp as f64 / r.pairs as f64,
        },
        "large_prime_graph": {
            "vertices": r.graph_vertices,
            "edges": r.graph_edges,
            "giant_fraction": r.giant_fraction,
            "cycle_count": r.cycle_count,
            "usable_cycles_including_full": r.usable_cycles,
        },
        "selection_sge": {
            "retained_tree_edges": r.retained_tree_edges,
            "retained_extra_edges": r.retained_extra_edges,
            "materialized_tree_edges": r.materialized_tree_edges,
            "materialized_full_rows": r.materialized_full_rows,
            "materialized_rows": r.materialized_rows,
            "materialized_dependency_dimension": r.materialized_dependency_dimension,
            "rows_after_lp_sge": r.rows_after_lp_sge,
            "total_row_weight_after_lp_sge": r.total_row_weight_after_lp_sge,
            "mean_row_weight_after_lp_sge": r.mean_row_weight_after_lp_sge,
            "max_row_weight_after_lp_sge": r.max_row_weight_after_lp_sge,
            "mean_pivot_weight_after_rank_sge": r.mean_pivot_weight_after_rank_sge,
        },
        "linear_algebra": {
            "rank": r.rank,
            "nullity": r.nullity,
        },
        "cost_receipts": costs,
        "references": {
            "rho_same_instance_relation_only": rho_same_instance,
            "rho_formula": "sqrt(pi*ell/2)*74.5 FqM; no native-band counters available, so lower-bound reference only",
            "matched_hyperelliptic_2p15": matched_hyper_2p15,
            "matched_hyper_formula": "9.967e10*(2^15/2^22)^1.408; q-matched round-4 aggregate extrapolation, not rerun on this exact E",
            "quartic_projected_2p15": projected_2p15,
            "quartic_projection_formula": "7.3e6+0.53e6*w, w=measured mean row weight after LP SGE",
            "k4_threshold": fraction_f64(K4_THRESHOLD_NUM, K4_THRESHOLD_DEN),
        },
        "counters": counters(&r.counters),
        "commit": identity.commit.as_str(),
        "protocol_sha256": identity.protocol_sha256.as_str(),
        "binary_sha256": identity.binary_sha256.as_str(),
        "image_digest": identity.image_digest.as_str(),
    })
}

fn append(writer: &mut BufWriter<std::fs::File>, value: &Value) {
    serde_json::to_writer(&mut *writer, value).expect("serialize JSON record");
    writer.write_all(b"\n").expect("append newline");
    writer.flush().expect("flush record");
    writer.get_ref().sync_data().expect("sync record");
}

fn run_size(q: u64, output: &Path) {
    assert!(ALLOWED_Q.contains(&q), "q is not a frozen size");
    let identity = run_identity();
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)
        .unwrap_or_else(|e| panic!("create {}: {e}", output.display()));
    let mut writer = BufWriter::new(file);
    append(
        &mut writer,
        &json!({
            "schema": "lead15.diem-quartic/v1",
            "record": "run_start",
            "protocol_id": "LEAD15-DT-E5-20261101",
            "q": q,
            "curves": CURVES_PER_SIZE,
            "base_seed": BASE_SEED,
            "max_candidates": MAX_CANDIDATES,
            "retain_factor": RETAIN_FACTOR,
            "runtime_provenance": runtime_provenance(),
            "commit": identity.commit.as_str(),
            "protocol_sha256": identity.protocol_sha256.as_str(),
            "binary_sha256": identity.binary_sha256.as_str(),
            "image_digest": identity.image_digest.as_str(),
        }),
    );

    let mut instances = vec![];
    let mut quartics: HashSet<[u64; 15]> = HashSet::new();
    for idx in 0..CURVES_PER_SIZE {
        let mut retry = 0u32;
        let mut cumulative_stats = ProspectiveBuildStats::default();
        loop {
            let seed = seed_for(q, idx, retry);
            let (inst, witness, stats) = gen_prospective_instance(q, idx, seed, MAX_CANDIDATES)
                .unwrap_or_else(|e| panic!("q={q} idx={idx} retry={retry}: {e}"));
            add_build_stats(&mut cumulative_stats, &stats);
            cumulative_stats.hash_lookups = cumulative_stats.hash_lookups.saturating_add(1);
            if !quartics.insert(inst.quartic) {
                retry = retry.checked_add(1).expect("duplicate retry overflow");
                continue;
            }
            cumulative_stats.hash_inserts = cumulative_stats.hash_inserts.saturating_add(1);
            append(
                &mut writer,
                &instance_record(
                    &inst,
                    &witness,
                    idx,
                    retry,
                    seed,
                    &stats,
                    &cumulative_stats,
                    &identity,
                ),
            );
            instances.push((idx, seed, inst));
            break;
        }
    }
    for (idx, seed, inst) in instances {
        let harvest = prospective_full_harvest(&inst, seed ^ 0x4841_5256_4553_5435, RETAIN_FACTOR, None)
            .unwrap_or_else(|e| panic!("q={q} idx={idx} harvest: {e}"));
        append(&mut writer, &harvest_record(&harvest, idx, seed, &identity));
    }
    append(
        &mut writer,
        &json!({
            "schema": "lead15.diem-quartic/v1",
            "record": "run_end",
            "protocol_id": "LEAD15-DT-E5-20261101",
            "q": q,
            "instances": CURVES_PER_SIZE,
            "harvests": CURVES_PER_SIZE,
            "complete": true,
            "commit": identity.commit.as_str(),
            "protocol_sha256": identity.protocol_sha256.as_str(),
            "binary_sha256": identity.binary_sha256.as_str(),
            "image_digest": identity.image_digest.as_str(),
        }),
    );
}

fn write_new_json(path: &Path, value: &Value) {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    writer
        .write_all(&pretty_json_bytes(value))
        .expect("serialize analysis JSON");
    writer.flush().expect("flush analysis");
    writer.get_ref().sync_data().expect("sync analysis");
}

fn pretty_json_bytes(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("serialize pretty JSON");
    bytes.push(b'\n');
    bytes
}

fn absolute_binding_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("current directory for artifact binding")
            .join(path)
    }
}

fn file_binding_from_bytes(path: &Path, kind: &str, bytes: &[u8]) -> Value {
    let bound_path = absolute_binding_path(path);
    json!({
        "path": bound_path.display().to_string(),
        "sha256": sha256_hex(bytes),
        "bytes": bytes.len(),
        "kind": kind,
    })
}

fn file_binding(path: &Path, kind: &str) -> Value {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let canonical = fs::canonicalize(path)
        .unwrap_or_else(|e| panic!("canonicalize {}: {e}", path.display()));
    file_binding_from_bytes(&canonical, kind, &bytes)
}

fn append_lineage_file_bindings(files: &mut Vec<Value>, completion: &CompletionAudit) {
    let Some(receipt) = completion.receipt.as_ref() else {
        return;
    };
    let Some(lineage) = receipt["lineage_artifacts"].as_object() else {
        return;
    };
    let base = Path::new(&completion.path)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    for descriptor in lineage.values() {
        let Some(name) = descriptor["path"].as_str() else {
            continue;
        };
        if name.contains('/') || name.contains('\\') {
            continue;
        }
        let path = base.join(name);
        if fs::symlink_metadata(&path)
            .is_ok_and(|metadata| metadata.file_type().is_file() && !metadata.file_type().is_symlink())
        {
            files.push(file_binding(
                &path,
                if name == "records.jsonl" { "raw" } else { "receipt" },
            ));
        }
    }
}

fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn valid_frozen_job_name(q: u64, value: &str) -> bool {
    let prefix = format!("lead15-dt-e5-q{q}-");
    value.strip_prefix(&prefix).is_some_and(|suffix| {
        (8..=12).contains(&suffix.len())
            && suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn frozen_source_manifest(path: &Path) -> FrozenSource {
    let manifest_bytes = fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let manifest_sha256 = sha256_hex(&manifest_bytes);
    let manifest: Value = serde_json::from_slice(&manifest_bytes)
    .unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
    assert_exact_object_keys(
        &manifest,
        "source_manifest",
        &[
            "schema", "status", "protocol_id", "instrument_commit", "freeze_commit",
            "protocol_sha256", "protocol_file_sha256", "binary_sha256", "build_info_sha256",
            "build_info_schema_sha256",
            "analysis_source_sha256", "library_source_sha256", "worker_source_sha256",
            "collector_source_sha256", "k4_reference_sha256", "result_schema_sha256",
            "source_manifest_schema_sha256", "completion_receipt_schema_sha256",
            "request_schema_sha256", "binary_path_in_image", "worker_path_in_image",
            "build_info_path_in_image", "image_uri", "image_digest", "launch_tool", "isolation",
            "threads_per_cell", "scientific_environment", "scientific_deadline_seconds",
            "outer_job_deadline_seconds", "termination_grace_seconds", "retries",
            "requested_resources", "jobs", "durable_output_prefix",
            "pre_manifest_package_receipt",
            "frozen_at",
        ],
    );
    assert_eq!(manifest["schema"], "lead15.source-manifest/v1");
    assert_eq!(manifest["status"], "frozen");
    assert_eq!(manifest["protocol_id"], "LEAD15-DT-E5-20261101");
    let commit = manifest["instrument_commit"]
        .as_str()
        .expect("source manifest instrument_commit")
        .to_string();
    let protocol_sha256 = manifest["protocol_sha256"]
        .as_str()
        .expect("source manifest protocol_sha256")
        .to_string();
    let binary_sha256 = manifest["binary_sha256"]
        .as_str()
        .expect("source manifest binary_sha256")
        .to_string();
    let image_digest = manifest["image_digest"]
        .as_str()
        .expect("source manifest image_digest")
        .to_string();
    let freeze_commit = manifest["freeze_commit"]
        .as_str()
        .expect("source manifest freeze_commit")
        .to_string();
    let protocol_file_sha256 = manifest["protocol_file_sha256"]
        .as_str()
        .expect("source manifest protocol_file_sha256")
        .to_string();
    let analysis_source_sha256 = manifest["analysis_source_sha256"]
        .as_str()
        .expect("source manifest analysis_source_sha256")
        .to_string();
    let worker_source_sha256 = manifest["worker_source_sha256"]
        .as_str()
        .expect("source manifest worker_source_sha256")
        .to_string();
    let collector_source_sha256 = manifest["collector_source_sha256"]
        .as_str()
        .expect("source manifest collector_source_sha256")
        .to_string();
    let build_info_sha256 = manifest["build_info_sha256"]
        .as_str()
        .expect("source manifest build_info_sha256")
        .to_string();
    let library_source_sha256 = manifest["library_source_sha256"]
        .as_str()
        .expect("source manifest library_source_sha256");
    assert!(is_lower_hex(library_source_sha256, 64));
    let library_path = path
        .parent()
        .expect("source manifest parent")
        .join("../../src/cryptanalysis/diem_quartic.rs");
    assert_eq!(
        sha256_hex(
            &fs::read(&library_path)
                .unwrap_or_else(|e| panic!("read {}: {e}", library_path.display()))
        ),
        library_source_sha256
    );
    let k4_reference_sha256 = manifest["k4_reference_sha256"]
        .as_str()
        .expect("source manifest k4_reference_sha256")
        .to_string();
    for (field, file) in [
        ("result_schema_sha256", "result.schema.json"),
        ("source_manifest_schema_sha256", "source-manifest.schema.json"),
        (
            "completion_receipt_schema_sha256",
            "completion-receipt.schema.json",
        ),
        ("request_schema_sha256", "isolab-request.schema.json"),
        ("build_info_schema_sha256", "build-info.schema.json"),
    ] {
        let expected = manifest[field]
            .as_str()
            .unwrap_or_else(|| panic!("source manifest {field}"));
        assert!(is_lower_hex(expected, 64));
        let schema_path = path.parent().expect("source manifest parent").join(file);
        let schema_bytes = fs::read(&schema_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", schema_path.display()));
        assert_eq!(sha256_hex(&schema_bytes), expected, "{field}");
        let schema: Value = serde_json::from_slice(&schema_bytes)
            .unwrap_or_else(|e| panic!("parse {}: {e}", schema_path.display()));
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema",
            "{file}: schema dialect"
        );
    }
    let durable_output_prefix = manifest["durable_output_prefix"]
        .as_str()
        .expect("source manifest durable_output_prefix")
        .to_string();
    let cargo_lock_sha256 = sha256_hex(
        &fs::read(path.parent().unwrap().join("../../Cargo.lock"))
            .expect("read Cargo.lock for build identity"),
    );
    let image_recipe_sha256 = sha256_hex(
        &fs::read(path.parent().unwrap().join("lead15-image.Dockerfile"))
            .expect("read LEAD-15 image recipe"),
    );
    let image_uri = manifest["image_uri"]
        .as_str()
        .expect("source manifest image_uri")
        .to_string();
    assert!(is_lower_hex(&commit, 40));
    assert!(is_lower_hex(&freeze_commit, 40));
    assert!(is_lower_hex(&protocol_sha256, 64));
    assert!(is_lower_hex(&protocol_file_sha256, 64));
    assert!(is_lower_hex(&binary_sha256, 64));
    assert!(is_lower_hex(&analysis_source_sha256, 64));
    assert!(is_lower_hex(&worker_source_sha256, 64));
    assert!(is_lower_hex(&collector_source_sha256, 64));
    assert!(is_lower_hex(&build_info_sha256, 64));
    assert!(is_lower_hex(&k4_reference_sha256, 64));
    for (expected, file) in [
        (&worker_source_sha256, "lead15-worker.sh"),
        (&collector_source_sha256, "lead15-collect.sh"),
    ] {
        let source_path = path.parent().expect("source manifest parent").join(file);
        assert_eq!(
            sha256_hex(
                &fs::read(&source_path)
                    .unwrap_or_else(|e| panic!("read {}: {e}", source_path.display()))
            ),
            *expected,
            "{file} source hash"
        );
    }
    let k4_path = path
        .parent()
        .expect("source manifest parent")
        .join("k4-reference.json");
    let k4_bytes =
        fs::read(&k4_path).unwrap_or_else(|e| panic!("read {}: {e}", k4_path.display()));
    assert_eq!(sha256_hex(&k4_bytes), k4_reference_sha256);
    let k4: Value = serde_json::from_slice(&k4_bytes)
        .unwrap_or_else(|e| panic!("parse {}: {e}", k4_path.display()));
    assert_eq!(k4["schema"], "lead15.k4-reference/v1");
    assert_eq!(
        k4["canonical_numerator"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap(),
        MATCHED_HYPER_NUM
    );
    assert_eq!(
        k4["canonical_denominator"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap(),
        MATCHED_HYPER_DEN
    );
    assert_eq!(
        k4["threshold_numerator"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap(),
        K4_THRESHOLD_NUM
    );
    assert_eq!(
        k4["threshold_denominator"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap(),
        K4_THRESHOLD_DEN
    );
    assert!(
        image_digest
            .strip_prefix("sha256:")
            .is_some_and(|hex| is_lower_hex(hex, 64))
    );
    assert!(image_uri
        .strip_suffix(&format!("@{image_digest}"))
        .is_some_and(|repository| {
            !repository.is_empty()
                && !repository.contains('@')
                && !repository.chars().any(char::is_whitespace)
        }));
    assert_eq!(manifest["launch_tool"], "isolab_k8s_run");
    assert_eq!(manifest["isolation"], "none");
    assert_eq!(
        manifest["binary_path_in_image"],
        "/opt/lead15/lead15_diem_quartic"
    );
    assert_eq!(manifest["worker_path_in_image"], "/opt/lead15/lead15-worker");
    assert_eq!(
        manifest["build_info_path_in_image"],
        "/opt/lead15/build-info.json"
    );
    assert_eq!(manifest["threads_per_cell"].as_u64(), Some(1));
    assert_eq!(
        manifest["scientific_environment"],
        json!({"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1"})
    );
    assert_eq!(manifest["scientific_deadline_seconds"].as_u64(), Some(3600));
    assert_eq!(manifest["outer_job_deadline_seconds"].as_u64(), Some(7200));
    assert!(
        manifest["outer_job_deadline_seconds"]
            .as_u64()
            .is_some_and(|seconds| seconds >= 6600)
    );
    assert_eq!(manifest["termination_grace_seconds"].as_u64(), Some(60));
    assert_eq!(manifest["retries"].as_u64(), Some(0));
    assert_eq!(
        manifest["requested_resources"],
        json!({"cpu":4,"memory_gib":16,"spot":true})
    );
    const DURABLE_ROOT: &str = "s3://isolab-results-104186811093-us-east-2/runs/";
    assert_eq!(durable_output_prefix, DURABLE_ROOT);
    let pre_manifest_receipt = &manifest["pre_manifest_package_receipt"];
    assert_exact_object_keys(
        pre_manifest_receipt,
        "source_manifest.pre_manifest_package_receipt",
        &["scope", "s3_uri", "sha256", "bytes", "version_id", "readback_verified"],
    );
    assert_eq!(
        pre_manifest_receipt["scope"],
        "pre_manifest_package_excluding_source_manifest_and_requests"
    );
    assert_eq!(
        pre_manifest_receipt["s3_uri"],
        "s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/pre-manifest-package.tar"
    );
    assert!(pre_manifest_receipt["sha256"]
        .as_str()
        .is_some_and(|v| is_lower_hex(v, 64)));
    assert!(pre_manifest_receipt["bytes"]
        .as_u64()
        .is_some_and(|v| v > 0));
    assert!(pre_manifest_receipt["version_id"]
        .as_str()
        .is_some_and(|value| !value.is_empty()));
    assert_eq!(
        pre_manifest_receipt["readback_verified"].as_bool(),
        Some(true)
    );
    assert!(manifest["frozen_at"]
        .as_str()
        .is_some_and(|v| !v.is_empty()));
    let jobs = manifest["jobs"].as_array().expect("source manifest jobs");
    assert_eq!(jobs.len(), ALLOWED_Q.len());
    let mut frozen_jobs = HashMap::new();
    for (job, &q) in jobs.iter().zip(ALLOWED_Q.iter()) {
        assert_exact_object_keys(
            job,
            "source_manifest.jobs[]",
            &[
                "q",
                "job_name",
                "argv_template",
                "request_artifact_s3_uri",
                "results_prefix",
            ],
        );
        assert_eq!(job["q"].as_u64(), Some(q));
        let job_name = job["job_name"]
            .as_str()
            .expect("source manifest job_name")
            .to_string();
        assert!(valid_frozen_job_name(q, &job_name));
        let argv = job["argv_template"]
            .as_array()
            .expect("source manifest exact argv template");
        assert_eq!(argv.len(), 8);
        assert!(argv.iter().all(Value::is_string));
        let argv_template: Vec<String> = argv
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(argv_template[0], "/opt/lead15/lead15-worker");
        assert_eq!(argv_template[1], q.to_string());
        assert_eq!(argv_template[2], "${SOURCE_MANIFEST_SHA256}");
        assert_eq!(argv_template[3], commit);
        assert_eq!(argv_template[4], protocol_sha256);
        assert_eq!(argv_template[5], binary_sha256);
        assert_eq!(argv_template[6], image_digest);
        assert_eq!(argv_template[7], "3600");
        let request_artifact_s3_uri = job["request_artifact_s3_uri"]
            .as_str()
            .expect("source manifest request artifact URI")
            .to_string();
        assert_eq!(
            request_artifact_s3_uri,
            format!(
                "s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/requests/q-{q}.json"
            )
        );
        let results_prefix = job["results_prefix"]
            .as_str()
            .expect("source manifest results prefix")
            .to_string();
        assert_eq!(results_prefix, format!("{DURABLE_ROOT}{job_name}/"));
        assert!(frozen_jobs
            .insert(
                q,
                FrozenJob {
                    job_name,
                    argv_template,
                    request_artifact_s3_uri,
                    results_prefix,
                },
            )
            .is_none());
    }
    FrozenSource {
        identity: RunIdentity {
            commit,
            protocol_sha256,
            binary_sha256,
            image_digest,
        },
        freeze_commit,
        protocol_file_sha256,
        analysis_source_sha256,
        worker_source_sha256,
        collector_source_sha256,
        build_info_sha256,
        cargo_lock_sha256,
        image_recipe_sha256,
        k4_reference_sha256,
        manifest_sha256,
        image_uri,
        jobs: frozen_jobs,
    }
}

fn assert_record_identity(v: &Value, expected: &RunIdentity, path: &str) {
    assert_eq!(v["commit"].as_str(), Some(expected.commit.as_str()), "{path}: commit");
    assert_eq!(
        v["protocol_sha256"].as_str(),
        Some(expected.protocol_sha256.as_str()),
        "{path}: protocol hash"
    );
    assert_eq!(
        v["binary_sha256"].as_str(),
        Some(expected.binary_sha256.as_str()),
        "{path}: binary hash"
    );
    assert_eq!(
        v["image_digest"].as_str(),
        Some(expected.image_digest.as_str()),
        "{path}: image digest"
    );
}

fn assert_close(label: &str, actual: f64, expected: f64) {
    assert!(actual.is_finite() && expected.is_finite(), "{label}: non-finite value");
    let tolerance = 1e-12 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: {actual} != {expected}"
    );
}

fn validate_frozen_protocol(path: &Path, source: &FrozenSource) -> Value {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert_eq!(sha256_hex(&bytes), source.protocol_file_sha256);
    let protocol: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
    assert_eq!(protocol["schema"], "experiment.protocol/v1");
    assert_eq!(protocol["protocol_id"], "LEAD15-DT-E5-20261101");
    assert_eq!(protocol["freeze"]["status"], "frozen");
    assert_eq!(
        protocol["freeze"]["protocol_sha256"].as_str(),
        Some(source.identity.protocol_sha256.as_str())
    );
    assert_eq!(
        protocol["freeze"]["commit"].as_str(),
        Some(source.freeze_commit.as_str())
    );
    assert_eq!(protocol["freeze"]["filed_before_affected_data"], true);
    assert_eq!(protocol["measurement"]["isolation_required"], "none");
    assert_eq!(
        protocol["links"]["k4_reference"],
        "research/lead15_diem_quartic_20261101/k4-reference.json"
    );
    assert_eq!(protocol["execution"]["timeout_seconds"].as_u64(), Some(7200));
    assert_eq!(
        protocol["extensions"]["lead15_launch_contract"]["scientific_child_deadline_seconds"]
            .as_u64(),
        Some(3600)
    );
    assert_eq!(
        protocol["extensions"]["lead15_launch_contract"]["outer_job_deadline_seconds"]
            .as_u64(),
        Some(7200)
    );
    assert_eq!(
        protocol["extensions"]["lead15_launch_contract"]["termination_grace_seconds"]
            .as_u64(),
        Some(60)
    );
    assert_eq!(protocol["execution"]["checkpoint"]["policy"], "none");
    assert_eq!(protocol["execution"]["retries"]["max_attempts"].as_u64(), Some(1));
    assert_eq!(
        protocol["execution"]["placement"]["requested_cpus"].as_u64(),
        Some(4)
    );
    assert_eq!(
        protocol["execution"]["placement"]["requested_memory_gib"].as_u64(),
        Some(16)
    );
    assert_eq!(
        protocol["execution"]["placement"]["requested_spot"],
        true
    );
    let runtime = &protocol["sample_size"]["preflight_runtime"];
    assert_eq!(runtime["per_sample_seconds"].as_f64(), Some(150.0));
    assert_eq!(runtime["estimated_total_seconds"].as_f64(), Some(3600.0));
    assert!(runtime["measured_on"]
        .as_str()
        .is_some_and(|v| v.starts_with("UNVALIDATED evaluator-e5 desk estimate")));
    assert!(protocol["decision_rule"]["threshold"]
        .as_str()
        .is_some_and(|v| v.contains("32265178769533359/500000000")));
    let arms = protocol["arms"].as_array().expect("protocol arms");
    let candidate = arms
        .iter()
        .find(|v| v["id"] == "dt-common-cover")
        .expect("candidate arm");
    assert_eq!(
        candidate["instrument"]["commit"].as_str(),
        Some(source.identity.commit.as_str())
    );
    assert_eq!(
        candidate["instrument"]["binary_sha256"].as_str(),
        Some(source.identity.binary_sha256.as_str())
    );
    assert_eq!(
        candidate["instrument"]["image_digest"].as_str(),
        Some(source.identity.image_digest.as_str())
    );
    assert_eq!(
        candidate["instrument"]["build"],
        json!([["cargo", "build", "--locked", "--release", "--example", "lead15_diem_quartic"]]),
        "protocol build command must match the locked build-info contract"
    );
    let script = &protocol["analysis_plan"]["script"];
    assert_eq!(
        script["sha256"].as_str(),
        Some(source.analysis_source_sha256.as_str())
    );
    let script_path = Path::new(script["path"].as_str().expect("analysis script path"));
    let resolved = if script_path.is_absolute() {
        script_path.to_path_buf()
    } else {
        path.parent().expect("protocol parent").join(script_path)
    };
    assert_eq!(
        sha256_hex(&fs::read(&resolved).unwrap_or_else(|e| panic!("read {}: {e}", resolved.display()))),
        source.analysis_source_sha256
    );
    protocol
}

#[derive(Clone, Debug, Default)]
struct PrefixRead {
    bytes: Option<Vec<u8>>,
    records: Vec<Value>,
    trailing_fragment: Vec<u8>,
    problems: Vec<String>,
}

fn read_jsonl_prefix(path: &str) -> PrefixRead {
    let mut out = PrefixRead::default();
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            out.problems.push(format!("missing_or_unreadable_payload:{e}"));
            return out;
        }
    };
    let prefix_end = bytes
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|p| p + 1)
        .unwrap_or(0);
    out.trailing_fragment = bytes[prefix_end..].to_vec();
    for (line_no, line) in bytes[..prefix_end].split(|&b| b == b'\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        match serde_json::from_slice::<Value>(line) {
            Ok(v) => out.records.push(v),
            Err(e) => out
                .problems
                .push(format!("line_{}_invalid_json:{e}", line_no + 1)),
        }
    }
    out.bytes = Some(bytes);
    out
}

#[derive(Clone, Debug)]
struct CompletionAudit {
    path: String,
    receipt: Option<Value>,
    classification: String,
    problems: Vec<String>,
}

fn nonempty_string(v: &Value) -> bool {
    v.as_str().is_some_and(|s| !s.is_empty())
}

fn valid_etag(v: &Value) -> bool {
    v.as_str().is_some_and(|etag| etag.chars().count() > 1)
}

fn exact_bool(v: &Value, expected: bool) -> bool {
    v.as_bool() == Some(expected)
}

fn ascii_u32(bytes: &[u8]) -> Option<u32> {
    bytes.iter().try_fold(0u32, |value, byte| {
        if byte.is_ascii_digit() {
            Some(value * 10 + u32::from(*byte - b'0'))
        } else {
            None
        }
    })
}

fn valid_rfc3339_timestamp(v: &Value) -> bool {
    let Some(text) = v.as_str() else {
        return false;
    };
    let bytes = text.as_bytes();
    if bytes.len() != 20
        || bytes.get(4) != Some(&b'-')
        || bytes.get(7) != Some(&b'-')
        || bytes.get(10) != Some(&b'T')
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
        || bytes.get(19) != Some(&b'Z')
    {
        return false;
    }
    let Some(year) = ascii_u32(&bytes[0..4]) else {
        return false;
    };
    let Some(month) = ascii_u32(&bytes[5..7]) else {
        return false;
    };
    let Some(day) = ascii_u32(&bytes[8..10]) else {
        return false;
    };
    let Some(hour) = ascii_u32(&bytes[11..13]) else {
        return false;
    };
    let Some(minute) = ascii_u32(&bytes[14..16]) else {
        return false;
    };
    let Some(second) = ascii_u32(&bytes[17..19]) else {
        return false;
    };
    if year < 1970 {
        return false;
    }
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    if !(1..=days_in_month).contains(&day) || hour > 23 || minute > 59 || second > 59 {
        return false;
    }
    true
}

fn sha256sums_entry_is_unique(
    checksums: &[u8],
    name: &str,
    expected_hash: &str,
) -> bool {
    if !sha256sums_bytes_are_ascii(checksums) {
        return false;
    }
    let Ok(text) = std::str::from_utf8(checksums) else {
        return false;
    };
    let expected_path = format!("./{name}");
    let mut matches = 0usize;
    for line in text.split('\n') {
        let mut fields = line
            .split(|character| matches!(character, ' ' | '\t'))
            .filter(|field| !field.is_empty());
        let hash = fields.next();
        let path = fields.next();
        if path == Some(expected_path.as_str()) {
            if hash != Some(expected_hash) || fields.next().is_some() {
                return false;
            }
            matches += 1;
        }
    }
    matches == 1
}

fn sha256sums_bytes_are_ascii(checksums: &[u8]) -> bool {
    checksums.is_ascii()
}

fn exact_keys(value: &Value, expected: &[&str]) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    let mut actual: Vec<&str> = object.keys().map(String::as_str).collect();
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    actual == expected
}

fn worker_exit_envelope_valid(exit: &Value) -> bool {
    if !exact_keys(
        exit,
        &[
            "schema",
            "terminal_class",
            "deadline_reached",
            "timeout_fired",
            "exit_code",
            "child_elapsed_seconds",
            "payload",
            "started_at",
            "finished_at",
            "results_uri",
            "job_name",
            "pod_uid",
            "node_name",
        ],
    ) || !exact_keys(
        &exit["payload"],
        &["path", "sha256", "bytes", "final_newline"],
    ) {
        return false;
    }
    let Some(terminal) = exit["terminal_class"].as_str() else {
        return false;
    };
    let Some(deadline_reached) = exit["deadline_reached"].as_bool() else {
        return false;
    };
    let Some(timeout_fired) = exit["timeout_fired"].as_bool() else {
        return false;
    };
    let Some(exit_code) = exit["exit_code"].as_u64().filter(|code| *code <= 255) else {
        return false;
    };
    let Some(elapsed) = exit["child_elapsed_seconds"].as_u64() else {
        return false;
    };
    let payload_valid = exit["payload"]["path"] == "records.jsonl"
        && exit["payload"]["sha256"]
            .as_str()
            .is_some_and(|value| is_lower_hex(value, 64))
        && exit["payload"]["bytes"].as_u64().is_some()
        && exit["payload"]["final_newline"].as_bool().is_some();
    let envelope_valid = exit["schema"] == "lead15.worker-exit/v1"
        && valid_rfc3339_timestamp(&exit["started_at"])
        && valid_rfc3339_timestamp(&exit["finished_at"])
        && nonempty_string(&exit["results_uri"])
        && nonempty_string(&exit["job_name"])
        && nonempty_string(&exit["pod_uid"])
        && nonempty_string(&exit["node_name"])
        && payload_valid;
    if !envelope_valid {
        return false;
    }
    match terminal {
        "completed" => !deadline_reached && !timeout_fired && exit_code == 0,
        "deadline" => {
            deadline_reached
                && timeout_fired
                && matches!(exit_code, 124 | 137)
                && elapsed >= 3599
        }
        "scientific_failure" => {
            !deadline_reached
                && !timeout_fired
                && exit_code != 0
                && !matches!(exit_code, 130 | 137 | 143)
        }
        // An externally delivered signal is intentionally classified as
        // infrastructure even if the child races to another exit status.  If
        // timeout did fire, however, its code and elapsed time remain bound.
        "infrastructure_failure" => {
            !deadline_reached
                && (!timeout_fired
                    || (matches!(exit_code, 124 | 137) && elapsed >= 3599))
        }
        _ => false,
    }
}

fn worker_upload_envelope_valid(upload: &Value) -> bool {
    if !exact_keys(
        upload,
        &[
            "schema",
            "uploaded",
            "upload_exit",
            "terminal_class",
            "child_exit_code",
            "results_uri",
            "payload_sha256",
            "payload_bytes",
        ],
    ) {
        return false;
    }
    let Some(uploaded) = upload["uploaded"].as_bool() else {
        return false;
    };
    let Some(upload_exit) = upload["upload_exit"].as_u64().filter(|code| *code <= 255) else {
        return false;
    };
    upload["schema"] == "lead15.worker-upload/v1"
        && uploaded == (upload_exit == 0)
        && matches!(
            upload["terminal_class"].as_str(),
            Some("completed" | "deadline" | "scientific_failure" | "infrastructure_failure")
        )
        && upload["child_exit_code"]
            .as_u64()
            .is_some_and(|code| code <= 255)
        && nonempty_string(&upload["results_uri"])
        && upload["payload_sha256"]
            .as_str()
            .is_some_and(|value| is_lower_hex(value, 64))
        && upload["payload_bytes"].as_u64().is_some()
}

fn worker_start_exit_times_coherent(start: &Value, exit: &Value) -> bool {
    nonempty_string(&start["started_at"])
        && start.get("started_at") == exit.get("started_at")
}

fn runtime_provenance_envelope_valid(
    runtime: &Value,
    expected_job: &str,
    expected_pod_name: &str,
    expected_pod_uid: &str,
    expected_node: &Value,
) -> bool {
    exact_keys(
        runtime,
        &[
            "schema",
            "cpu_model",
            "logical_cpus",
            "memory_bytes",
            "kernel",
            "job_name",
            "pod_name",
            "pod_uid",
            "node_name",
        ],
    ) && runtime["schema"] == "lead15.runtime-provenance/v1"
        && runtime["job_name"].as_str() == Some(expected_job)
        && runtime["pod_name"].as_str() == Some(expected_pod_name)
        && runtime["pod_uid"].as_str() == Some(expected_pod_uid)
        && runtime.get("node_name") == Some(expected_node)
        && nonempty_string(&runtime["cpu_model"])
        && runtime["logical_cpus"]
            .as_u64()
            .is_some_and(|value| value > 0)
        && runtime["memory_bytes"]
            .as_u64()
            .is_some_and(|value| value > 0)
        && nonempty_string(&runtime["kernel"])
}

fn read_bound_lineage_file(
    receipt_path: &Path,
    descriptor: &Value,
    expected_name: &str,
    required: bool,
    problems: &mut Vec<String>,
) -> Option<Vec<u8>> {
    let base = receipt_path.parent().unwrap_or_else(|| Path::new("."));
    let artifact_path = base.join(expected_name);
    if descriptor.is_null() {
        if required {
            problems.push(format!("missing_lineage_artifact:{expected_name}"));
        }
        match fs::symlink_metadata(&artifact_path) {
            Ok(_) => problems.push(format!(
                "unexpected_unbound_lineage_artifact:{expected_name}"
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => problems.push(format!(
                "lineage_artifact_presence_unreadable:{expected_name}:{error}"
            )),
        }
        return None;
    }
    if !exact_keys(descriptor, &["path", "sha256", "bytes"])
        || descriptor["path"].as_str() != Some(expected_name)
        || !descriptor["sha256"]
            .as_str()
            .is_some_and(|value| is_lower_hex(value, 64))
        || descriptor["bytes"].as_u64().is_none()
    {
        problems.push(format!("invalid_lineage_descriptor:{expected_name}"));
        return None;
    }
    let metadata = match fs::symlink_metadata(&artifact_path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            metadata
        }
        Ok(_) => {
            problems.push(format!("lineage_artifact_not_regular:{expected_name}"));
            return None;
        }
        Err(error) => {
            problems.push(format!("lineage_artifact_unreadable:{expected_name}:{error}"));
            return None;
        }
    };
    let bytes = match fs::read(&artifact_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            problems.push(format!("lineage_artifact_unreadable:{expected_name}:{error}"));
            return None;
        }
    };
    if metadata.len() != bytes.len() as u64
        || descriptor["bytes"].as_u64() != Some(bytes.len() as u64)
        || descriptor["sha256"].as_str() != Some(sha256_hex(&bytes).as_str())
    {
        problems.push(format!("lineage_artifact_hash_or_size_mismatch:{expected_name}"));
        return None;
    }
    Some(bytes)
}

fn parse_lineage_json(
    bytes: Option<&Vec<u8>>,
    expected_name: &str,
    problems: &mut Vec<String>,
) -> Option<Value> {
    let bytes = bytes?;
    match serde_json::from_slice(bytes) {
        Ok(value) => Some(value),
        Err(error) => {
            problems.push(format!("lineage_artifact_invalid_json:{expected_name}:{error}"));
            None
        }
    }
}

fn normalized_kubernetes_env(value: &Value) -> Option<Value> {
    let mut env = value.as_array()?.clone();
    for entry in &mut env {
        let Some(field_ref) = entry
            .get_mut("valueFrom")
            .and_then(|value| value.get_mut("fieldRef"))
            .and_then(Value::as_object_mut)
        else {
            continue;
        };
        if let Some(api_version) = field_ref.remove("apiVersion") {
            if api_version != "v1" {
                return None;
            }
        }
    }
    Some(Value::Array(env))
}

fn keys_within(value: &Value, allowed: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.keys().all(|key| allowed.contains(&key.as_str())))
}

fn empty_array_or_absent(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .is_none_or(|field| field.is_null() || field.as_array().is_some_and(Vec::is_empty))
}

fn empty_object_or_absent(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .is_none_or(|field| field.is_null() || field.as_object().is_some_and(|object| object.is_empty()))
}

fn false_or_absent(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .is_none_or(|field| field.is_null() || field.as_bool() == Some(false))
}

fn string_default_or_absent(value: &Value, key: &str, default: &str) -> bool {
    value
        .get(key)
        .is_none_or(|field| field.is_null() || field.as_str() == Some(default))
}

fn expected_kubernetes_resources() -> Value {
    json!({
        "requests":{"cpu":"4","memory":"16Gi","ephemeral-storage":"10Gi"},
        "limits":{"cpu":"4","memory":"16Gi","ephemeral-storage":"10Gi"},
    })
}

fn expected_kubernetes_volume_mounts() -> Value {
    json!([
        {"name":"work","mountPath":"/work"},
        {"name":"tmp","mountPath":"/tmp"},
    ])
}

fn expected_kubernetes_irsa_env() -> [Value; 2] {
    [
        json!({
            "name":"AWS_ROLE_ARN",
            "value":"arn:aws:iam::104186811093:role/isolab-job",
        }),
        json!({
            "name":"AWS_WEB_IDENTITY_TOKEN_FILE",
            "value":"/var/run/secrets/eks.amazonaws.com/serviceaccount/token",
        }),
    ]
}

fn kubernetes_irsa_env_is_exactly_present(value: &Value) -> bool {
    let Some(actual) = value.as_array() else {
        return false;
    };
    let expected = expected_kubernetes_irsa_env();
    actual.len() >= 2
        && expected
            .iter()
            .all(|entry| actual.iter().filter(|actual| *actual == entry).count() == 1)
        && actual
            .iter()
            .filter(|entry| {
                matches!(
                    entry.get("name").and_then(Value::as_str),
                    Some("AWS_ROLE_ARN" | "AWS_WEB_IDENTITY_TOKEN_FILE")
                )
            })
            .count()
            == 2
}

fn kubernetes_irsa_volume_mounts_are_exact(value: &Value) -> bool {
    let Some(actual) = value.as_array() else {
        return false;
    };
    let base: Vec<Value> = actual
        .iter()
        .filter(|mount| mount["name"] != "aws-iam-token")
        .cloned()
        .collect();
    let token_mounts: Vec<&Value> = actual
        .iter()
        .filter(|mount| mount["name"] == "aws-iam-token")
        .collect();
    let admitted = [
        json!({
            "name":"aws-iam-token",
            "mountPath":"/var/run/secrets/eks.amazonaws.com/serviceaccount",
            "readOnly":true,
        }),
        json!({
            "name":"aws-iam-token",
            "mountPath":"/var/run/secrets/eks.amazonaws.com/serviceaccount",
            "readOnly":true,
            "recursiveReadOnly":"Disabled",
        }),
    ];
    actual.len() == 3
        && Value::Array(base) == expected_kubernetes_volume_mounts()
        && token_mounts.len() == 1
        && admitted.iter().any(|expected| token_mounts[0] == expected)
}

fn expected_kubernetes_volumes() -> Value {
    json!([
        {"name":"work","emptyDir":{"sizeLimit":"8Gi"}},
        {"name":"tmp","emptyDir":{"sizeLimit":"1Gi"}},
    ])
}

fn expected_kubernetes_irsa_volume() -> Value {
    json!({
        "name":"aws-iam-token",
        "projected":{
            "defaultMode":420,
            "sources":[{"serviceAccountToken":{
                "audience":"sts.amazonaws.com",
                "expirationSeconds":86400,
                "path":"token",
            }}],
        },
    })
}

fn kubernetes_irsa_volumes_are_exact(value: &Value) -> bool {
    let Some(actual) = value.as_array() else {
        return false;
    };
    let base: Vec<Value> = actual
        .iter()
        .filter(|volume| volume["name"] != "aws-iam-token")
        .cloned()
        .collect();
    let token_volumes: Vec<&Value> = actual
        .iter()
        .filter(|volume| volume["name"] == "aws-iam-token")
        .collect();
    actual.len() == 3
        && Value::Array(base) == expected_kubernetes_volumes()
        && token_volumes.len() == 1
        && token_volumes[0] == &expected_kubernetes_irsa_volume()
}

fn expected_kubernetes_container_security_context() -> Value {
    json!({
        "allowPrivilegeEscalation":false,
        "readOnlyRootFilesystem":true,
        "capabilities":{"drop":["ALL"]},
    })
}

fn expected_kubernetes_pod_security_context() -> Value {
    json!({
        "runAsNonRoot":true,
        "runAsUser":1000,
        "runAsGroup":1000,
        "fsGroup":1000,
        "seccompProfile":{"type":"RuntimeDefault"},
    })
}

fn expected_kubernetes_affinity() -> Value {
    json!({
        "podAntiAffinity":{
            "requiredDuringSchedulingIgnoredDuringExecution":[{
                "labelSelector":{"matchLabels":{"isolab.dev/job":"true"}},
                "topologyKey":"kubernetes.io/hostname",
            }],
        },
    })
}

fn launcher_toleration() -> Value {
    json!({
        "key":"isolab.dev/workload",
        "value":"cpu",
        "operator":"Equal",
        "effect":"NoSchedule",
    })
}

fn kubernetes_default_tolerations() -> [Value; 2] {
    [
        json!({"effect":"NoExecute","key":"node.kubernetes.io/not-ready",
            "operator":"Exists","tolerationSeconds":300}),
        json!({"effect":"NoExecute","key":"node.kubernetes.io/unreachable",
            "operator":"Exists","tolerationSeconds":300}),
    ]
}

fn launcher_tolerations_with_optional_defaults(value: &Value) -> bool {
    let Some(actual) = value.as_array() else {
        return false;
    };
    let launcher = launcher_toleration();
    let defaults = kubernetes_default_tolerations();
    let launcher_count = actual.iter().filter(|item| *item == &launcher).count();
    let default_counts =
        defaults.map(|expected| actual.iter().filter(|item| *item == &expected).count());
    launcher_count == 1
        && (default_counts == [0, 0] || default_counts == [1, 1])
        && actual.len() == 1 + default_counts[0] + default_counts[1]
}

fn kubernetes_env_field_refs_are_v1(value: &Value) -> bool {
    value.as_array().is_some_and(|entries| {
        entries.iter().all(|entry| {
            entry
                .pointer("/valueFrom/fieldRef/apiVersion")
                .is_none_or(|version| version.as_str() == Some("v1"))
        })
    })
}

fn launcher_job_labels(job_name: &str, job_uid: &str) -> Value {
    json!({
        "isolab.dev/job":"true",
        "batch.kubernetes.io/controller-uid":job_uid,
        "batch.kubernetes.io/job-name":job_name,
        "controller-uid":job_uid,
        "job-name":job_name,
    })
}

fn launcher_annotations() -> Value {
    json!({"karpenter.sh/do-not-disrupt":"true"})
}

fn pod_labels_match_launcher(value: &Value, job_name: &str, job_uid: &str) -> bool {
    if value == &launcher_job_labels(job_name, job_uid) {
        return true;
    }
    let Some(labels) = value.as_object() else {
        return false;
    };
    if labels.len() != 7
        || labels.get("topology.kubernetes.io/region").and_then(Value::as_str)
            != Some("us-east-2")
        || !labels
            .get("topology.kubernetes.io/zone")
            .and_then(Value::as_str)
            .is_some_and(|zone| {
                zone.len() == "us-east-2a".len()
                    && zone.starts_with("us-east-2")
                    && zone.as_bytes().last().is_some_and(u8::is_ascii_lowercase)
            })
    {
        return false;
    }
    let mut without_topology = labels.clone();
    without_topology.remove("topology.kubernetes.io/region");
    without_topology.remove("topology.kubernetes.io/zone");
    Value::Object(without_topology) == launcher_job_labels(job_name, job_uid)
}

fn safe_kubernetes_container_surface(container: &Value, allow_irsa_admission: bool) -> bool {
    let env_ok = kubernetes_env_field_refs_are_v1(&container["env"])
        && (!allow_irsa_admission
            || kubernetes_irsa_env_is_exactly_present(&container["env"]));
    let mounts_ok = if allow_irsa_admission {
        kubernetes_irsa_volume_mounts_are_exact(&container["volumeMounts"])
    } else {
        container["volumeMounts"] == expected_kubernetes_volume_mounts()
    };
    keys_within(
        container,
        &[
            "name",
            "image",
            "command",
            "args",
            "workingDir",
            "env",
            "envFrom",
            "resources",
            "volumeMounts",
            "volumeDevices",
            "lifecycle",
            "stdin",
            "stdinOnce",
            "tty",
            "imagePullPolicy",
            "terminationMessagePath",
            "terminationMessagePolicy",
            "securityContext",
        ],
    ) && env_ok
        && container["resources"] == expected_kubernetes_resources()
        && mounts_ok
        && container["securityContext"] == expected_kubernetes_container_security_context()
        && empty_array_or_absent(container, "args")
        && empty_array_or_absent(container, "envFrom")
        && empty_array_or_absent(container, "volumeDevices")
        && empty_object_or_absent(container, "lifecycle")
        && false_or_absent(container, "stdin")
        && false_or_absent(container, "stdinOnce")
        && false_or_absent(container, "tty")
        && string_default_or_absent(container, "imagePullPolicy", "IfNotPresent")
        && string_default_or_absent(
            container,
            "terminationMessagePath",
            "/dev/termination-log",
        )
        && string_default_or_absent(container, "terminationMessagePolicy", "File")
}

fn safe_kubernetes_pod_spec_surface(
    spec: &Value,
    allow_node_name: bool,
    allow_irsa_admission: bool,
) -> bool {
    let node_name_ok = match spec.get("nodeName") {
        None | Some(Value::Null) => true,
        Some(value) => {
            allow_node_name && value.as_str().is_some_and(|name| !name.is_empty())
        }
    };
    let containers_ok = spec["containers"].as_array().is_some_and(|containers| {
        containers.len() == 1
            && containers
                .iter()
                .all(|container| safe_kubernetes_container_surface(container, allow_irsa_admission))
    });
    let volumes_ok = if allow_irsa_admission {
        kubernetes_irsa_volumes_are_exact(&spec["volumes"])
    } else {
        spec["volumes"] == expected_kubernetes_volumes()
    };
    keys_within(
        spec,
        &[
            "restartPolicy",
            "serviceAccountName",
            "serviceAccount",
            "automountServiceAccountToken",
            "terminationGracePeriodSeconds",
            "nodeSelector",
            "containers",
            "initContainers",
            "ephemeralContainers",
            "volumes",
            "securityContext",
            "dnsPolicy",
            "schedulerName",
            "enableServiceLinks",
            "preemptionPolicy",
            "nodeName",
            "tolerations",
            "priority",
            "priorityClassName",
            "imagePullSecrets",
            "affinity",
        ],
    ) && containers_ok
        && empty_array_or_absent(spec, "initContainers")
        && empty_array_or_absent(spec, "ephemeralContainers")
        && volumes_ok
        && spec["securityContext"] == expected_kubernetes_pod_security_context()
        && spec["affinity"] == expected_kubernetes_affinity()
        && string_default_or_absent(spec, "serviceAccount", "runner")
        && string_default_or_absent(spec, "dnsPolicy", "ClusterFirst")
        && string_default_or_absent(spec, "schedulerName", "default-scheduler")
        && spec
            .get("enableServiceLinks")
            .is_none_or(|value| value.is_null() || value.as_bool() == Some(true))
        && string_default_or_absent(spec, "preemptionPolicy", "PreemptLowerPriority")
        && spec
            .get("priority")
            .is_none_or(|value| value.is_null() || value.as_i64() == Some(0))
        && string_default_or_absent(spec, "priorityClassName", "")
        && empty_array_or_absent(spec, "imagePullSecrets")
        && launcher_tolerations_with_optional_defaults(&spec["tolerations"])
        && node_name_ok
}

fn normalized_kubernetes_container(
    container: &Value,
    allow_irsa_admission: bool,
) -> Option<Value> {
    if !safe_kubernetes_container_surface(container, allow_irsa_admission) {
        return None;
    }
    let mut object = container.as_object()?.clone();
    if let Some(env) = object.get("env") {
        let mut normalized = normalized_kubernetes_env(env)?.as_array()?.clone();
        if allow_irsa_admission {
            normalized.retain(|entry| {
                !matches!(
                    entry.get("name").and_then(Value::as_str),
                    Some("AWS_ROLE_ARN" | "AWS_WEB_IDENTITY_TOKEN_FILE")
                )
            });
        }
        object.insert("env".to_string(), Value::Array(normalized));
    }
    if allow_irsa_admission {
        let mut mounts = object.get("volumeMounts")?.as_array()?.clone();
        mounts.retain(|mount| mount["name"] != "aws-iam-token");
        object.insert("volumeMounts".to_string(), Value::Array(mounts));
    }
    for key in [
        "args",
        "envFrom",
        "volumeDevices",
        "lifecycle",
        "stdin",
        "stdinOnce",
        "tty",
        "imagePullPolicy",
        "terminationMessagePath",
        "terminationMessagePolicy",
    ] {
        object.remove(key);
    }
    Some(Value::Object(object))
}

fn normalized_kubernetes_pod_spec(
    spec: &Value,
    allow_node_name: bool,
    allow_irsa_admission: bool,
) -> Option<Value> {
    if !safe_kubernetes_pod_spec_surface(spec, allow_node_name, allow_irsa_admission) {
        return None;
    }
    let mut object = spec.as_object()?.clone();
    let containers = object.get("containers")?.as_array()?;
    let normalized_containers: Option<Vec<Value>> = containers
        .iter()
        .map(|container| normalized_kubernetes_container(container, allow_irsa_admission))
        .collect();
    object.insert(
        "containers".to_string(),
        Value::Array(normalized_containers?),
    );
    if allow_irsa_admission {
        let mut volumes = object.get("volumes")?.as_array()?.clone();
        volumes.retain(|volume| volume["name"] != "aws-iam-token");
        object.insert("volumes".to_string(), Value::Array(volumes));
    }
    object.insert("tolerations".to_string(), json!([launcher_toleration()]));
    for key in [
        "nodeName",
        "initContainers",
        "ephemeralContainers",
        "serviceAccount",
        "dnsPolicy",
        "schedulerName",
        "enableServiceLinks",
        "preemptionPolicy",
        "priority",
        "priorityClassName",
        "imagePullSecrets",
    ] {
        object.remove(key);
    }
    Some(Value::Object(object))
}

fn safe_kubernetes_job_surface(job: &Value) -> bool {
    let Some(metadata) = job.get("metadata") else {
        return false;
    };
    let Some(job_name) = metadata.get("name").and_then(Value::as_str) else {
        return false;
    };
    let Some(job_uid) = metadata.get("uid").and_then(Value::as_str) else {
        return false;
    };
    let Some(spec) = job.get("spec") else {
        return false;
    };
    let Some(template) = spec.get("template") else {
        return false;
    };
    let template_metadata_ok = template.get("metadata").is_some_and(|metadata| {
        keys_within(
            metadata,
            &["creationTimestamp", "labels", "annotations", "name", "namespace"],
        ) && metadata["labels"] == launcher_job_labels(job_name, job_uid)
            && metadata["annotations"] == launcher_annotations()
            && metadata
                .get("creationTimestamp")
                .is_none_or(|value| value.is_null())
            && metadata
                .get("name")
                .is_none_or(|value| value.is_null())
            && metadata
                .get("namespace")
                .is_none_or(|value| value.is_null())
    });
    let selector_ok = spec["selector"]
        == json!({"matchLabels":{"batch.kubernetes.io/controller-uid":job_uid}});
    keys_within(job, &["apiVersion", "kind", "metadata", "spec", "status"])
        && keys_within(
            metadata,
            &[
                "name",
                "namespace",
                "uid",
                "resourceVersion",
                "generation",
                "creationTimestamp",
                "labels",
                "annotations",
                "managedFields",
            ],
        )
        && metadata["labels"] == json!({"isolab.dev/job":"true"})
        && metadata["generation"].as_u64() == Some(1)
        && empty_object_or_absent(metadata, "annotations")
        && keys_within(
            spec,
            &[
                "backoffLimit",
                "activeDeadlineSeconds",
                "template",
                "parallelism",
                "completions",
                "completionMode",
                "manualSelector",
                "selector",
                "suspend",
                "ttlSecondsAfterFinished",
                "podReplacementPolicy",
            ],
        )
        && spec
            .get("parallelism")
            .is_none_or(|value| value.is_null() || value.as_u64() == Some(1))
        && spec
            .get("completions")
            .is_none_or(|value| value.is_null() || value.as_u64() == Some(1))
        && string_default_or_absent(spec, "completionMode", "NonIndexed")
        && false_or_absent(spec, "manualSelector")
        && false_or_absent(spec, "suspend")
        && selector_ok
        && spec.get("ttlSecondsAfterFinished").and_then(Value::as_u64) == Some(86400)
        && string_default_or_absent(spec, "podReplacementPolicy", "TerminatingOrFailed")
        && keys_within(template, &["metadata", "spec"])
        && template_metadata_ok
        && template
            .get("spec")
            .is_some_and(|spec| safe_kubernetes_pod_spec_surface(spec, false, false))
}

fn safe_kubernetes_pod_surface(pod: &Value, expected_job: &str, expected_job_uid: &str) -> bool {
    let Some(metadata) = pod.get("metadata") else {
        return false;
    };
    let finalizers_ok = metadata.get("finalizers").is_none_or(|value| {
        value.is_null()
            || value.as_array().is_some_and(|items| {
                items.is_empty()
                    || (items.len() == 1
                        && items[0] == "batch.kubernetes.io/job-tracking")
            })
    });
    let expected_generate_name = format!("{expected_job}-");
    let generate_name_ok = metadata.get("generateName").is_none_or(|value| {
        value.is_null() || value.as_str() == Some(expected_generate_name.as_str())
    });
    let scheduled = pod["spec"]["nodeName"]
        .as_str()
        .is_some_and(|name| !name.is_empty());
    let observed_generation_ok = match pod.get("status") {
        None => !scheduled,
        Some(status) => {
            let Some(status) = status.as_object() else {
                return false;
            };
            match status.get("observedGeneration") {
                Some(value) => value.as_u64() == Some(1),
                None => !scheduled,
            }
        }
    };
    keys_within(pod, &["apiVersion", "kind", "metadata", "spec", "status"])
        && pod["apiVersion"] == "v1"
        && pod["kind"] == "Pod"
        && keys_within(
            metadata,
            &[
                "name",
                "generateName",
                "namespace",
                "uid",
                "resourceVersion",
                "generation",
                "creationTimestamp",
                "labels",
                "annotations",
                "ownerReferences",
                "finalizers",
                "managedFields",
            ],
        )
        && nonempty_string(&metadata["name"])
        && nonempty_string(&metadata["uid"])
        && metadata["namespace"] == "isolab"
        && metadata["generation"].as_u64() == Some(1)
        && observed_generation_ok
        && pod_labels_match_launcher(&metadata["labels"], expected_job, expected_job_uid)
        && metadata["annotations"] == launcher_annotations()
        && metadata["ownerReferences"]
            == json!([{
                "apiVersion":"batch/v1",
                "kind":"Job",
                "name":expected_job,
                "uid":expected_job_uid,
                "controller":true,
                "blockOwnerDeletion":true,
            }])
        && finalizers_ok
        && generate_name_ok
        && pod
            .get("spec")
            .is_some_and(|spec| safe_kubernetes_pod_spec_surface(spec, true, true))
}

fn terminal_status_pods(status: &Value) -> Option<Vec<Value>> {
    status.get("pods")?.as_array().cloned()
}

fn terminal_status_pod_shape(pod: &Value) -> bool {
    let Some(object) = pod.as_object() else {
        return false;
    };
    nonempty_string(&pod["name"])
        && nonempty_string(&pod["uid"])
        && match object.get("node") {
            None => true,
            Some(node) if node.is_null() => true,
            Some(node) => node.as_str().is_some_and(|name| !name.is_empty()),
        }
}

fn completion_receipt_pod_shape(pod: &Value) -> bool {
    exact_keys(pod, &["name", "uid", "node_name"])
        && nonempty_string(&pod["name"])
        && nonempty_string(&pod["uid"])
        && (pod["node_name"].is_null()
            || pod["node_name"]
                .as_str()
                .is_some_and(|name| !name.is_empty()))
}

fn completion_receipt_job_shape(job: &Value, expected_name: &str) -> bool {
    exact_keys(job, &["name", "uid"])
        && job["name"].as_str() == Some(expected_name)
        && nonempty_string(&job["uid"])
}

fn incomplete_worker_summary_valid(
    receipt: &Value,
    attempts: u64,
    raw_pod_exit_code: Option<i64>,
) -> bool {
    let exit_code_ok = match attempts {
        0 => receipt["exit_code"].is_null(),
        1 => match receipt["pod"].get("node_name") {
            Some(Value::Null) => receipt["exit_code"].is_null(),
            Some(Value::String(node)) if !node.is_empty() => raw_pod_exit_code
                .is_some_and(|code| receipt["exit_code"].as_i64() == Some(code)),
            _ => false,
        },
        _ => false,
    };
    receipt["deadline_reached"].as_bool() == Some(false)
        && receipt["started_at"].is_null()
        && receipt["finished_at"].is_null()
        && exit_code_ok
}

fn audit_completion_receipt(
    path: &str,
    q: u64,
    source: &FrozenSource,
    payload: &PrefixRead,
) -> CompletionAudit {
    let mut problems = vec![];
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {}
        Ok(_) => {
            return CompletionAudit {
                path: path.to_string(),
                receipt: None,
                classification: "unknown_incomplete".to_string(),
                problems: vec!["completion_receipt_not_regular_file".to_string()],
            };
        }
        Err(_) => {}
    }
    let receipt_bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return CompletionAudit {
                path: path.to_string(),
                receipt: None,
                classification: "unknown_incomplete".to_string(),
                problems: vec![format!("missing_or_unreadable_completion_receipt:{e}")],
            };
        }
    };
    let receipt: Value = match serde_json::from_slice(&receipt_bytes) {
        Ok(v) => v,
        Err(e) => {
            return CompletionAudit {
                path: path.to_string(),
                receipt: None,
                classification: "unknown_incomplete".to_string(),
                problems: vec![format!("invalid_completion_receipt_json:{e}")],
            };
        }
    };
    let expected_receipt_keys = [
        "schema", "protocol_id", "q", "source_manifest_sha256", "collector_sha256",
        "request_sha256", "request_artifact", "image_uri", "image_digest", "argv",
        "requested_resources", "scientific_environment", "scientific_deadline_seconds",
        "outer_job_deadline_seconds", "termination_grace_seconds", "retries", "attempts",
        "pod_restarts", "job", "pod", "terminal_class", "deadline_reached", "exit_code",
        "payload", "resource_provenance", "lineage_artifacts", "started_at", "finished_at",
        "collected_at",
    ];
    let exact_receipt_shape = receipt.as_object().is_some_and(|object| {
        let actual: HashSet<&str> = object.keys().map(String::as_str).collect();
        let expected: HashSet<&str> = expected_receipt_keys.iter().copied().collect();
        actual == expected
    });
    if !exact_receipt_shape {
        problems.push("completion_receipt_schema_shape_mismatch".to_string());
    }
    let frozen_job = source.jobs.get(&q).expect("frozen job");
    let mut expected_argv = frozen_job.argv_template.clone();
    expected_argv[2] = source.manifest_sha256.clone();
    if receipt["schema"] != "lead15.completion-receipt/v1"
        || receipt["protocol_id"] != "LEAD15-DT-E5-20261101"
        || receipt["q"].as_u64() != Some(q)
        || receipt["source_manifest_sha256"].as_str() != Some(source.manifest_sha256.as_str())
        || receipt["collector_sha256"].as_str() != Some(source.collector_source_sha256.as_str())
        || receipt["image_uri"].as_str() != Some(source.image_uri.as_str())
        || receipt["image_digest"].as_str() != Some(source.identity.image_digest.as_str())
        || receipt["argv"] != json!(expected_argv)
    {
        problems.push("completion_identity_or_argv_mismatch".to_string());
    }
    if receipt["requested_resources"] != json!({"cpu":4,"memory_gib":16,"spot":true})
        || receipt["scientific_environment"]
            != json!({"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1"})
        || receipt["scientific_deadline_seconds"].as_u64() != Some(3600)
        || receipt["outer_job_deadline_seconds"].as_u64() != Some(7200)
        || receipt["termination_grace_seconds"].as_u64() != Some(60)
        || receipt["retries"].as_u64() != Some(0)
    {
        problems.push("completion_execution_contract_mismatch".to_string());
    }
    if !completion_receipt_job_shape(&receipt["job"], frozen_job.job_name.as_str())
        || !valid_rfc3339_timestamp(&receipt["collected_at"])
    {
        problems.push("completion_job_or_collection_identity_invalid".to_string());
    }

    let lineage = &receipt["lineage_artifacts"];
    let expected_lineage_keys = [
        "request", "request_head", "submission", "job", "pod", "status", "results",
        "object_heads", "payload", "worker_start", "worker_exit", "worker_upload",
        "runtime_provenance", "build_info", "binary_sha256", "stdout", "stderr",
        "sha256sums", "timeout_events",
    ];
    let lineage_shape = lineage.as_object().is_some_and(|object| {
        object.len() == expected_lineage_keys.len()
            && expected_lineage_keys
                .iter()
                .all(|key| object.contains_key(*key))
    });
    if !lineage_shape {
        problems.push("lineage_artifacts_schema_shape_mismatch".to_string());
    }
    let mut artifacts: HashMap<String, Vec<u8>> = HashMap::new();
    for (key, name, required) in [
        ("request", "request.json", true),
        ("request_head", "request-head.json", true),
        ("submission", "submission.json", true),
        ("job", "job.json", true),
        ("pod", "pod.json", false),
        ("status", "status.json", true),
        ("results", "results.json", true),
        ("object_heads", "object-heads.json", true),
        ("payload", "records.jsonl", false),
        ("worker_start", "worker-start.json", false),
        ("worker_exit", "worker-exit.json", false),
        ("worker_upload", "worker-upload.json", false),
        ("runtime_provenance", "runtime-provenance.json", false),
        ("build_info", "build-info.json", false),
        ("binary_sha256", "binary.sha256", false),
        ("stdout", "stdout.log", false),
        ("stderr", "stderr.log", false),
        ("sha256sums", "SHA256SUMS", false),
        ("timeout_events", "timeout-events.log", false),
    ] {
        if let Some(bytes) = read_bound_lineage_file(
            Path::new(path),
            &lineage[key],
            name,
            required,
            &mut problems,
        ) {
            artifacts.insert(name.to_string(), bytes);
        }
    }
    let mut parsed: HashMap<&str, Value> = HashMap::new();
    for name in [
        "request.json", "request-head.json", "submission.json", "job.json", "pod.json",
        "status.json", "results.json", "object-heads.json", "worker-start.json",
        "worker-exit.json", "worker-upload.json", "runtime-provenance.json", "build-info.json",
    ] {
        if let Some(value) = parse_lineage_json(artifacts.get(name), name, &mut problems) {
            parsed.insert(name, value);
        }
    }

    if let Some(request) = parsed.get("request.json") {
        let request_shape = request.as_object().is_some_and(|object| {
            object.len() == 4
                && ["schema", "tool", "source_manifest_sha256", "arguments"]
                    .iter()
                    .all(|key| object.contains_key(*key))
                && request["arguments"].as_object().is_some_and(|arguments| {
                    arguments.len() == 7
                        && ["job_name", "command", "image", "cpu", "memory", "seconds", "retries"]
                            .iter()
                            .all(|key| arguments.contains_key(*key))
                })
        });
        if !request_shape
            || request["schema"] != "lead15.isolab-request/v1"
            || request["tool"] != "isolab_k8s_run"
            || request["source_manifest_sha256"].as_str()
                != Some(source.manifest_sha256.as_str())
            || request["arguments"]
                != json!({
                    "job_name":frozen_job.job_name.as_str(),
                    "command":expected_argv,
                    "image":source.image_uri.as_str(),
                    "cpu":4,
                    "memory":"16Gi",
                    "seconds":7200,
                    "retries":0,
                })
        {
            problems.push("request_bytes_violate_frozen_contract".to_string());
        }
        let request_bytes = &artifacts["request.json"];
        let request_sha = sha256_hex(request_bytes);
        let request_checksum = sha256_base64(request_bytes);
        let request_artifact_shape = receipt["request_artifact"]
            .as_object()
            .is_some_and(|object| {
                object.len() == 8
                    && [
                        "path",
                        "sha256",
                        "checksum_sha256",
                        "bytes",
                        "s3_uri",
                        "etag",
                        "version_id",
                        "readback_verified",
                    ]
                    .iter()
                    .all(|key| object.contains_key(*key))
            });
        if !request_artifact_shape
            || receipt["request_sha256"].as_str() != Some(request_sha.as_str())
            || receipt["request_artifact"]["path"] != "request.json"
            || receipt["request_artifact"]["sha256"].as_str() != Some(request_sha.as_str())
            || receipt["request_artifact"]["checksum_sha256"].as_str()
                != Some(request_checksum.as_str())
            || receipt["request_artifact"]["bytes"].as_u64()
                != Some(request_bytes.len() as u64)
            || receipt["request_artifact"]["s3_uri"].as_str()
                != Some(frozen_job.request_artifact_s3_uri.as_str())
            || receipt["request_artifact"]["readback_verified"].as_bool() != Some(true)
        {
            problems.push("request_artifact_byte_binding_invalid".to_string());
        }
        if let Some(head) = parsed.get("request-head.json") {
            if head["ContentLength"].as_u64() != Some(request_bytes.len() as u64)
                || head["ETag"] != receipt["request_artifact"]["etag"]
                || head["VersionId"] != receipt["request_artifact"]["version_id"]
                || head["ChecksumSHA256"].as_str() != Some(request_checksum.as_str())
                || head["ChecksumSHA256"] != receipt["request_artifact"]["checksum_sha256"]
                || !valid_etag(&head["ETag"])
                || !nonempty_string(&head["VersionId"])
            {
                problems.push("request_head_version_binding_invalid".to_string());
            }
        }
    }

    let job_uid = receipt["job"]["uid"].as_str().unwrap_or("");
    if let Some(submission) = parsed.get("submission.json") {
        if submission["job"]["job_name"].as_str() != Some(frozen_job.job_name.as_str())
            || submission["job"]["uid"].as_str() != Some(job_uid)
            || submission["results_uri"].as_str() != Some(frozen_job.results_prefix.as_str())
        {
            problems.push("submission_bytes_identity_mismatch".to_string());
        }
    }
    if let Some(job) = parsed.get("job.json") {
        let containers = job["spec"]["template"]["spec"]["containers"].as_array();
        let container = containers.and_then(|values| (values.len() == 1).then(|| &values[0]));
        let results_env = format!("{}$(POD_UID)/", frozen_job.results_prefix);
        let expected_env = json!([
            {"name":"HOME","value":"/work"},
            {"name":"AWS_REGION","value":"us-east-2"},
            {"name":"AWS_DEFAULT_REGION","value":"us-east-2"},
            {"name":"POD_UID","valueFrom":{"fieldRef":{"fieldPath":"metadata.uid"}}},
            {"name":"NODE_NAME","valueFrom":{"fieldRef":{"fieldPath":"spec.nodeName"}}},
            {"name":"ISOLAB_JOB_ID","value":frozen_job.job_name.as_str()},
            {"name":"ISOLAB_OUTPUT_DIR","value":"/work/results"},
            {"name":"ISOLAB_RESULTS_URI","value":results_env},
            {"name":"RAYON_NUM_THREADS","value":"4"},
            {"name":"OMP_NUM_THREADS","value":"4"}
        ]);
        let valid_container = container.is_some_and(|container| {
            container["name"] == "task"
                && container["workingDir"] == "/work"
                && container["image"].as_str() == Some(source.image_uri.as_str())
                && container["command"] == json!(expected_argv)
                && normalized_kubernetes_env(&container["env"]).as_ref() == Some(&expected_env)
                && container["resources"]["requests"]["cpu"] == "4"
                && container["resources"]["limits"]["cpu"] == "4"
                && container["resources"]["requests"]["memory"] == "16Gi"
                && container["resources"]["limits"]["memory"] == "16Gi"
                && container["resources"]["requests"]["ephemeral-storage"] == "10Gi"
                && container["resources"]["limits"]["ephemeral-storage"] == "10Gi"
        });
        if !safe_kubernetes_job_surface(job)
            || job["apiVersion"] != "batch/v1"
            || job["kind"] != "Job"
            || job["metadata"]["name"].as_str() != Some(frozen_job.job_name.as_str())
            || job["metadata"]["uid"].as_str() != Some(job_uid)
            || job["metadata"]["namespace"] != "isolab"
            || job["metadata"]["labels"]["isolab.dev/job"] != "true"
            || job["spec"]["backoffLimit"].as_u64() != Some(0)
            || job["spec"]["activeDeadlineSeconds"].as_u64() != Some(7200)
            || job["spec"]["template"]["spec"]["restartPolicy"] != "Never"
            || job["spec"]["template"]["spec"]["serviceAccountName"] != "runner"
            || job["spec"]["template"]["spec"]["automountServiceAccountToken"] != false
            || job["spec"]["template"]["spec"]["terminationGracePeriodSeconds"].as_u64()
                != Some(60)
            || job["spec"]["template"]["spec"]["nodeSelector"]
                != json!({"isolab.dev/pool":"cpu-spot"})
            || !valid_container
        {
            problems.push("job_bytes_violate_frozen_contract".to_string());
        }
    }

    let mut status_state = "unknown";
    let mut status_pods: Vec<Value> = vec![];
    if let Some(status) = parsed.get("status.json") {
        status_state = status["job"]["state"].as_str().unwrap_or("unknown");
        let parsed_status_pods = terminal_status_pods(status);
        let status_pods_is_array = parsed_status_pods.is_some();
        status_pods = parsed_status_pods.unwrap_or_default();
        if status["job"]["job_name"].as_str() != Some(frozen_job.job_name.as_str())
            || status["job"]["uid"].as_str() != Some(job_uid)
            || !matches!(status_state, "succeeded" | "failed")
            || !status_pods_is_array
            || status_pods.len() > 1
            || !status_pods.iter().all(terminal_status_pod_shape)
            || receipt["attempts"].as_u64() != Some(status_pods.len() as u64)
        {
            problems.push("status_bytes_terminal_or_attempt_mismatch".to_string());
        }
    }
    let attempts = receipt["attempts"].as_u64().unwrap_or(u64::MAX);
    let mut pod_exit_code = None;
    let mut pod_termination_reason = None;
    if attempts == 1 {
        let receipt_pod = &receipt["pod"];
        let receipt_pod_shape_ok = completion_receipt_pod_shape(receipt_pod);
        let status_pod = status_pods.first();
        let pod = parsed.get("pod.json");
        pod_exit_code = pod
            .and_then(|pod| pod["status"]["containerStatuses"][0]["state"]["terminated"]["exitCode"].as_i64());
        pod_termination_reason = pod
            .and_then(|pod| pod["status"]["containerStatuses"][0]["state"]["terminated"]["reason"].as_str())
            .map(str::to_string);
        let owner_ok = pod.is_some_and(|pod| {
            pod["metadata"]["ownerReferences"].as_array().is_some_and(|owners| {
                owners.iter().any(|owner| {
                    owner["uid"].as_str() == Some(job_uid)
                        && owner["controller"].as_bool() == Some(true)
                })
            })
        });
        let desired_spec = parsed
            .get("job.json")
            .map(|job| &job["spec"]["template"]["spec"]);
        let pod_container_ok = match (pod, desired_spec) {
            (Some(pod), Some(desired)) => {
                let actual = &pod["spec"];
                safe_kubernetes_pod_surface(
                    pod,
                    frozen_job.job_name.as_str(),
                    job_uid,
                )
                    && normalized_kubernetes_pod_spec(actual, true, true)
                        == normalized_kubernetes_pod_spec(desired, false, false)
                    && actual["containers"].as_array().is_some_and(|containers| {
                        containers.len() == 1
                            && containers[0]["image"].as_str()
                                == Some(source.image_uri.as_str())
                            && containers[0]["command"] == json!(expected_argv)
                    })
            }
            _ => false,
        };
        let runtime_ok = pod.is_some_and(|pod| {
            match receipt_pod.get("node_name") {
                Some(Value::Null) => pod["spec"]
                    .get("nodeName")
                    .is_none_or(Value::is_null),
                Some(Value::String(node)) if !node.is_empty() => {
                    pod["spec"]["nodeName"].as_str() == Some(node.as_str())
                        && pod["status"]["containerStatuses"].as_array().is_some_and(|statuses| {
                            statuses.len() == 1
                                && statuses[0]["name"] == "task"
                                && nonempty_string(&statuses[0]["image"])
                                && statuses[0]["restartCount"].as_u64() == Some(0)
                                && statuses[0]["state"]["terminated"]["exitCode"]
                                    .as_i64()
                                    .is_some_and(|code| (0..=255).contains(&code))
                                && statuses[0]["state"]["terminated"]["reason"]
                                    .as_str()
                                    .is_some_and(|reason| !reason.is_empty())
                                && statuses[0]["imageID"]
                                    .as_str()
                                    .is_some_and(|id| id.ends_with(&format!("@{}", source.identity.image_digest)))
                        })
                }
                _ => false,
            }
        });
        let restart_summary_ok = if receipt_pod["node_name"].is_null() {
            receipt["pod_restarts"].is_null()
        } else {
            receipt["pod_restarts"].as_u64() == Some(0)
        };
        if !receipt_pod_shape_ok
            || status_pod.is_none()
            || pod.is_none()
            || status_pod.unwrap()["name"] != receipt_pod["name"]
            || status_pod.unwrap()["uid"] != receipt_pod["uid"]
            || status_pod.unwrap()["node"] != receipt_pod["node_name"]
            || pod.unwrap()["metadata"]["name"] != receipt_pod["name"]
            || pod.unwrap()["metadata"]["uid"] != receipt_pod["uid"]
            || !owner_ok
            || !pod_container_ok
            || !runtime_ok
            || !restart_summary_ok
        {
            problems.push("pod_bytes_identity_or_runtime_mismatch".to_string());
        }
    } else if attempts == 0 {
        if !receipt["pod"].is_null()
            || !lineage["pod"].is_null()
            || !receipt["pod_restarts"].is_null()
        {
            problems.push("zero_attempt_cell_has_pod_evidence".to_string());
        }
    } else {
        problems.push("attempt_count_outside_preregistered_range".to_string());
    }

    let pod_uid = receipt["pod"]["uid"].as_str().unwrap_or("");
    let results = parsed.get("results.json");
    let object_heads = parsed.get("object-heads.json");
    if results.is_none()
        || results.unwrap()["results_uri"].as_str() != Some(frozen_job.results_prefix.as_str())
        || !results.unwrap()["next_token"].is_null()
        || results.unwrap()["objects"].as_array().is_none()
        || object_heads.is_none()
        || object_heads.unwrap()["schema"] != "lead15.object-heads/v1"
        || object_heads.unwrap()["results_prefix"].as_str()
            != Some(frozen_job.results_prefix.as_str())
        || object_heads.unwrap()["objects"].as_object().is_none()
    {
        problems.push("result_listing_or_object_heads_invalid".to_string());
    } else {
        let objects = results.unwrap()["objects"].as_array().unwrap();
        let mut keys = HashSet::new();
        let remote_names = [
            "records.jsonl", "worker-start.json", "worker-exit.json", "worker-upload.json",
            "runtime-provenance.json", "build-info.json", "binary.sha256", "stdout.log",
            "stderr.log", "SHA256SUMS", "timeout-events.log",
        ];
        let head_names: HashSet<&str> = object_heads
            .unwrap()["objects"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        if pod_uid.is_empty() {
            if !objects.is_empty() || !head_names.is_empty() {
                problems.push("zero_pod_has_remote_worker_objects".to_string());
            }
        } else {
            let result_key_prefix = format!("runs/{}/{pod_uid}/", frozen_job.job_name);
            let mut listed_names = HashSet::new();
            let valid_listing = objects.iter().all(|object| {
                object["Key"].as_str().is_some_and(|key| {
                    let Some(name) = key.strip_prefix(&result_key_prefix) else {
                        return false;
                    };
                    !name.contains('/')
                        && remote_names.contains(&name)
                        && keys.insert(key)
                        && listed_names.insert(name)
                })
            });
            let local_names: HashSet<&str> = remote_names
                .iter()
                .copied()
                .filter(|name| artifacts.contains_key(*name))
                .collect();
            if !valid_listing || listed_names != head_names || head_names != local_names {
                problems.push("result_listing_head_or_local_set_mismatch".to_string());
            }
        }
        for name in remote_names {
            let Some(bytes) = artifacts.get(name) else { continue };
            if pod_uid.is_empty() {
                problems.push(format!("remote_artifact_without_pod:{name}"));
                continue;
            }
            let head = &object_heads.unwrap()["objects"][name];
            let key = format!("runs/{}/{pod_uid}/{name}", frozen_job.job_name);
            let head_etag = head["ETag"].as_str();
            let expected_checksum = sha256_base64(bytes);
            let listed = objects.iter().filter(|object| {
                object["Key"].as_str() == Some(key.as_str())
                    && object["Size"].as_u64() == Some(bytes.len() as u64)
                    && object["ETag"].as_str() == head_etag
                    && object["ChecksumAlgorithm"]
                        .as_array()
                        .is_some_and(|algorithms| algorithms.iter().any(|value| value == "SHA256"))
            }).count();
            if head["ContentLength"].as_u64() != Some(bytes.len() as u64)
                || !valid_etag(&head["ETag"])
                || !nonempty_string(&head["VersionId"])
                || head["ChecksumSHA256"].as_str() != Some(expected_checksum.as_str())
                || listed != 1
            {
                problems.push(format!("remote_object_version_or_listing_mismatch:{name}"));
            }
            if name == "records.jsonl"
                && (receipt["payload"]["etag"] != head["ETag"]
                    || receipt["payload"]["version_id"] != head["VersionId"]
                    || receipt["payload"]["checksum_sha256"] != head["ChecksumSHA256"])
            {
                problems.push("payload_object_version_summary_mismatch".to_string());
            }
        }
    }

    let worker_start = parsed.get("worker-start.json");
    let worker_exit = parsed.get("worker-exit.json");
    let worker_upload = parsed.get("worker-upload.json");
    let runtime = parsed.get("runtime-provenance.json");
    let build_info = parsed.get("build-info.json");
    let binary_sha256 = artifacts.get("binary.sha256");
    let stdout = artifacts.get("stdout.log");
    let stderr = artifacts.get("stderr.log");
    let sums = artifacts.get("SHA256SUMS");
    let timeout_events = artifacts.get("timeout-events.log");
    let timeout_events_empty = timeout_events.is_some_and(Vec::is_empty);
    let mut worker_complete = worker_start.is_some()
        && worker_exit.is_some()
        && worker_upload.is_some()
        && runtime.is_some()
        && build_info.is_some()
        && binary_sha256.is_some()
        && stdout.is_some()
        && stderr.is_some()
        && sums.is_some()
        && timeout_events.is_some();
    if timeout_events.is_some_and(|bytes| !timeout_events_are_canonical(bytes)) {
        problems.push("timeout_event_bytes_noncanonical".to_string());
    }
    if let Some(start) = worker_start {
        if !exact_keys(
            start,
            &[
                "schema",
                "q",
                "source_manifest_sha256",
                "instrument_commit",
                "protocol_sha256",
                "binary_sha256",
                "image_digest",
                "worker_sha256",
                "scientific_deadline_seconds",
                "job_name",
                "pod_uid",
                "node_name",
                "scientific_environment",
                "started_at",
            ],
        ) || start["schema"] != "lead15.worker-start/v1"
            || start["q"].as_u64() != Some(q)
            || start["source_manifest_sha256"].as_str() != Some(source.manifest_sha256.as_str())
            || start["instrument_commit"].as_str() != Some(source.identity.commit.as_str())
            || start["protocol_sha256"].as_str() != Some(source.identity.protocol_sha256.as_str())
            || start["binary_sha256"].as_str() != Some(source.identity.binary_sha256.as_str())
            || start["image_digest"].as_str() != Some(source.identity.image_digest.as_str())
            || start["worker_sha256"].as_str() != Some(source.worker_source_sha256.as_str())
            || start["scientific_deadline_seconds"].as_u64() != Some(3600)
            || start["scientific_environment"]
                != json!({"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1"})
            || start["job_name"].as_str() != Some(frozen_job.job_name.as_str())
            || start["pod_uid"].as_str() != Some(pod_uid)
            || start["node_name"] != receipt["pod"]["node_name"]
            || !valid_rfc3339_timestamp(&start["started_at"])
        {
            problems.push("worker_start_bytes_identity_mismatch".to_string());
        }
    }
    if let Some(build) = build_info {
        let build_shape = build.as_object().is_some_and(|object| {
            object.len() == 12
                && [
                    "schema", "source_revision", "cargo_lock_sha256", "rustc_verbose",
                    "target_triple", "base_image_uri", "image_recipe_sha256", "build_command",
                    "lead15_binary_sha256", "lead15_worker_sha256", "binary_path", "worker_path",
                ]
                .iter()
                .all(|key| object.contains_key(*key))
        });
        if !build_shape
            || sha256_hex(&artifacts["build-info.json"]) != source.build_info_sha256
            || build["schema"] != "lead15.build-info/v1"
            || build["source_revision"].as_str() != Some(source.identity.commit.as_str())
            || build["lead15_binary_sha256"].as_str()
                != Some(source.identity.binary_sha256.as_str())
            || build["lead15_worker_sha256"].as_str()
                != Some(source.worker_source_sha256.as_str())
            || build["cargo_lock_sha256"].as_str() != Some(source.cargo_lock_sha256.as_str())
            || build["image_recipe_sha256"].as_str()
                != Some(source.image_recipe_sha256.as_str())
            || !build["base_image_uri"].as_str().is_some_and(|image| {
                image
                    .rsplit_once("@sha256:")
                    .is_some_and(|(_, digest)| is_lower_hex(digest, 64))
            })
            || !nonempty_string(&build["rustc_verbose"])
            || build["target_triple"] != "x86_64-unknown-linux-gnu"
            || build["build_command"]
                != json!(["cargo","build","--locked","--release","--example","lead15_diem_quartic"])
            || build["binary_path"] != "/opt/lead15/lead15_diem_quartic"
            || build["worker_path"] != "/opt/lead15/lead15-worker"
        {
            problems.push("build_info_bytes_identity_mismatch".to_string());
        }
    }
    if let Some(binary_hash_bytes) = binary_sha256 {
        let expected = format!(
            "{}  /opt/lead15/lead15_diem_quartic\n",
            source.identity.binary_sha256
        );
        if binary_hash_bytes.as_slice() != expected.as_bytes() {
            problems.push("binary_sha256_bytes_identity_mismatch".to_string());
        }
    }
    if let Some(runtime) = runtime {
        let expected_resource = json!({
            "cpu_model":runtime["cpu_model"],
            "logical_cpus":runtime["logical_cpus"],
            "memory_bytes":runtime["memory_bytes"],
            "kernel":runtime["kernel"],
            "node_name":runtime["node_name"],
        });
        if !runtime_provenance_envelope_valid(
            runtime,
            frozen_job.job_name.as_str(),
            receipt["pod"]["name"].as_str().unwrap_or(""),
            pod_uid,
            &receipt["pod"]["node_name"],
        )
            || receipt["resource_provenance"] != expected_resource
        {
            problems.push("runtime_provenance_bytes_mismatch".to_string());
        }
    } else if !receipt["resource_provenance"].is_null() {
        problems.push("resource_provenance_without_runtime_bytes".to_string());
    }
    if let Some(exit) = worker_exit {
        let exit_code = exit["exit_code"].as_i64();
        let timeout_proven = timeout_events
            .zip(exit_code)
            .is_some_and(|(bytes, code)| timeout_events_prove_deadline(bytes, code));
        if !worker_exit_envelope_valid(exit)
            || exit["job_name"].as_str() != Some(frozen_job.job_name.as_str())
            || exit["pod_uid"].as_str() != Some(pod_uid)
            || exit["node_name"] != receipt["pod"]["node_name"]
            || exit["results_uri"].as_str()
                != Some(format!("{}{pod_uid}/", frozen_job.results_prefix).as_str())
        {
            problems.push("worker_exit_bytes_identity_mismatch".to_string());
        }
        if let Some(bytes) = timeout_events {
            if exit["timeout_fired"].as_bool() != Some(timeout_proven)
                || (!timeout_proven && !bytes.is_empty())
            {
                problems.push("worker_timeout_event_bytes_mismatch".to_string());
            }
        }
        if receipt["deadline_reached"] != exit["deadline_reached"]
            || receipt["exit_code"] != exit["exit_code"]
            || receipt["started_at"] != exit["started_at"]
            || receipt["finished_at"] != exit["finished_at"]
        {
            problems.push("worker_exit_receipt_summary_mismatch".to_string());
        }
        if let Some(payload_bytes) = artifacts.get("records.jsonl") {
            if exit["payload"]["sha256"].as_str() != Some(sha256_hex(payload_bytes).as_str())
                || exit["payload"]["bytes"].as_u64() != Some(payload_bytes.len() as u64)
                || exit["payload"]["final_newline"].as_bool()
                    != Some(payload_bytes.ends_with(b"\n"))
            {
                problems.push("worker_exit_payload_bytes_mismatch".to_string());
            }
        } else if exit["payload"]["bytes"].as_u64() != Some(0)
            || exit["payload"]["sha256"]
                != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            || exit["payload"]["final_newline"] != false
        {
            worker_complete = false;
        }
    } else if !incomplete_worker_summary_valid(&receipt, attempts, pod_exit_code) {
        problems.push("incomplete_worker_receipt_summary_invalid".to_string());
    }
    if let (Some(start), Some(exit)) = (worker_start, worker_exit) {
        if !worker_start_exit_times_coherent(start, exit) {
            problems.push("worker_start_exit_timestamp_mismatch".to_string());
        }
    }
    if let Some(upload) = worker_upload {
        if !worker_upload_envelope_valid(upload)
            || upload["results_uri"].as_str()
                != Some(format!("{}{pod_uid}/", frozen_job.results_prefix).as_str())
        {
            problems.push("worker_upload_bytes_identity_mismatch".to_string());
        }
        if let Some(payload_bytes) = artifacts.get("records.jsonl") {
            if upload["payload_sha256"].as_str() != Some(sha256_hex(payload_bytes).as_str())
                || upload["payload_bytes"].as_u64() != Some(payload_bytes.len() as u64)
            {
                problems.push("worker_upload_payload_bytes_mismatch".to_string());
            }
        } else if upload["payload_bytes"].as_u64() != Some(0)
            || upload["payload_sha256"]
                != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        {
            worker_complete = false;
        }
        if let Some(exit) = worker_exit {
            if upload["terminal_class"] != exit["terminal_class"]
                || upload["child_exit_code"] != exit["exit_code"]
            {
                problems.push("worker_upload_exit_summary_mismatch".to_string());
            }
        }
    }
    if let Some(checksums) = sums {
        if !sha256sums_bytes_are_ascii(checksums) {
            problems.push("sha256sums_non_ascii".to_string());
        }
        for name in [
            "records.jsonl", "worker-start.json", "worker-exit.json",
            "runtime-provenance.json", "build-info.json", "binary.sha256", "stdout.log",
            "stderr.log", "timeout-events.log",
        ] {
            let Some(bytes) = artifacts.get(name) else { continue };
            if !sha256sums_entry_is_unique(checksums, name, &sha256_hex(bytes)) {
                problems.push(format!("sha256sums_mismatch:{name}"));
            }
        }
    }

    let payload_artifact = artifacts.get("records.jsonl");
    let bulk_uploaded = worker_upload
        .and_then(|upload| upload["uploaded"].as_bool())
        .unwrap_or(false);
    let payload_shape = receipt["payload"].as_object().is_some_and(|object| {
        object.len() == 10
            && [
                "present",
                "s3_uri",
                "sha256",
                "checksum_sha256",
                "bytes",
                "etag",
                "version_id",
                "uploaded",
                "readback_verified",
                "final_newline",
            ]
            .iter()
            .all(|key| object.contains_key(*key))
    });
    if !payload_shape {
        problems.push("payload_receipt_schema_shape_mismatch".to_string());
    }
    if exact_bool(&receipt["payload"]["present"], true) {
        if payload_artifact.is_none()
            || receipt["payload"]["sha256"].as_str()
                != payload_artifact.map(|bytes| sha256_hex(bytes)).as_deref()
            || receipt["payload"]["checksum_sha256"].as_str()
                != payload_artifact.map(|bytes| sha256_base64(bytes)).as_deref()
            || receipt["payload"]["bytes"].as_u64()
                != payload_artifact.map(|bytes| bytes.len() as u64)
            || receipt["payload"]["s3_uri"].as_str()
                != Some(format!("{}{pod_uid}/records.jsonl", frozen_job.results_prefix).as_str())
            || receipt["payload"]["readback_verified"].as_bool() != Some(true)
            || receipt["payload"]["uploaded"].as_bool() != Some(bulk_uploaded)
            || receipt["payload"]["final_newline"].as_bool()
                != payload_artifact.map(|bytes| bytes.ends_with(b"\n"))
        {
            problems.push("payload_receipt_local_byte_binding_invalid".to_string());
        }
        if payload.bytes.as_ref() != payload_artifact {
            problems.push("analysis_payload_differs_from_collected_version".to_string());
        }
    } else if !exact_bool(&receipt["payload"]["present"], false)
        || payload_artifact.is_some()
        || receipt["payload"]["bytes"].as_u64() != Some(0)
        || !receipt["payload"]["s3_uri"].is_null()
        || !receipt["payload"]["sha256"].is_null()
        || !receipt["payload"]["checksum_sha256"].is_null()
        || !receipt["payload"]["etag"].is_null()
        || !receipt["payload"]["version_id"].is_null()
        || receipt["payload"]["uploaded"].as_bool() != Some(false)
        || receipt["payload"]["readback_verified"].as_bool() != Some(false)
        || receipt["payload"]["final_newline"].as_bool() != Some(false)
    {
        problems.push("absent_payload_summary_invalid".to_string());
    }

    let computed_terminal = if worker_complete {
        let worker_terminal = worker_exit.unwrap()["terminal_class"].as_str().unwrap_or("unknown");
        let worker_deadline = worker_exit.unwrap()["deadline_reached"].as_bool().unwrap_or(false);
        let worker_timeout_fired = worker_exit.unwrap()["timeout_fired"].as_bool().unwrap_or(false);
        let worker_code = worker_exit.unwrap()["exit_code"].as_i64();
        let timeout_proven = timeout_events
            .zip(worker_code)
            .is_some_and(|(bytes, code)| timeout_events_prove_deadline(bytes, code));
        let pod_exit_matches = pod_exit_code == worker_code;
        let bulk_uploaded = worker_upload.unwrap()["uploaded"].as_bool() == Some(true);
        if worker_terminal == "completed"
            && status_state == "succeeded"
            && worker_code == Some(0)
            && !worker_timeout_fired
            && timeout_events_empty
            && pod_exit_matches
            && pod_termination_reason.as_deref() == Some("Completed")
            && bulk_uploaded
            && receipt["payload"]["present"].as_bool() == Some(true)
            && receipt["payload"]["readback_verified"].as_bool() == Some(true)
            && receipt["payload"]["final_newline"].as_bool() == Some(true)
        {
            "completed"
        } else if worker_terminal == "deadline"
            && status_state == "failed"
            && worker_deadline
            && worker_timeout_fired
            && timeout_proven
            && matches!(worker_code, Some(124) | Some(137))
            && pod_exit_matches
            && pod_termination_reason.as_deref() == Some("Error")
            && bulk_uploaded
        {
            "deadline"
        } else if worker_terminal == "scientific_failure"
            && status_state == "failed"
            && !worker_deadline
            && !worker_timeout_fired
            && timeout_events_empty
            && worker_code.is_some_and(|code| code != 0)
            && pod_exit_matches
            && pod_termination_reason.as_deref() == Some("Error")
            && bulk_uploaded
        {
            "scientific_failure"
        } else {
            "infrastructure_failure"
        }
    } else {
        "infrastructure_failure"
    };
    if receipt["terminal_class"].as_str() != Some(computed_terminal) {
        problems.push("terminal_class_not_recomputed_from_lineage".to_string());
    }
    let terminal = computed_terminal;
    let classification = if !problems.is_empty() {
        "unknown_incomplete"
    } else {
        match terminal {
            "completed"
                if receipt["exit_code"].as_i64() == Some(0)
                    && receipt["deadline_reached"].as_bool() == Some(false)
                    && receipt["payload"]["uploaded"].as_bool() == Some(true)
                    && receipt["payload"]["final_newline"].as_bool() == Some(true) =>
            {
                "receipt_completed"
            }
            "deadline"
                if status_state == "failed"
                    && receipt["deadline_reached"].as_bool() == Some(true)
                    && matches!(receipt["exit_code"].as_i64(), Some(124) | Some(137)) =>
            {
                "censored_deadline"
            }
            "scientific_failure" => "scientific_invalid",
            "infrastructure_failure" => "infrastructure_incomplete",
            _ => "unknown_incomplete",
        }
    };
    CompletionAudit {
        path: path.to_string(),
        receipt: Some(receipt),
        classification: classification.to_string(),
        problems,
    }
}

fn export_measurements(counted: Value, counters: Value, max_rss_bytes: Value) -> Value {
    json!({
        "counted": counted,
        "wall_seconds": null,
        "wall_valid": false,
        "cpu_user_seconds": null,
        "cpu_sys_seconds": null,
        "max_rss_bytes": max_rss_bytes,
        "counters": counters,
    })
}

fn candidate_export_data(instance: &Value, harvest: &Value) -> (Value, Value) {
    let mut counted = serde_json::Map::new();
    let pairs = harvest["pairs"].as_u64().unwrap();
    let pp = harvest["yield"]["unique_pp"].as_u64().unwrap();
    let total_weight = harvest["selection_sge"]["total_row_weight_after_lp_sge"]
        .as_u64()
        .unwrap();
    let rows = harvest["selection_sge"]["rows_after_lp_sge"]
        .as_u64()
        .unwrap();
    counted.insert(
        "unique_pp_per_pair".to_string(),
        Value::String(format!("{pp}/{pairs}")),
    );
    counted.insert(
        "usable_cycles".to_string(),
        harvest["large_prime_graph"]["usable_cycles_including_full"].clone(),
    );
    counted.insert(
        "nullity".to_string(),
        harvest["linear_algebra"]["nullity"].clone(),
    );
    counted.insert(
        "post_lp_sge_weight".to_string(),
        Value::String(format!("{total_weight}/{}", if rows == 0 { 1 } else { rows })),
    );
    let builder = &instance["builder_setup"];
    for setup in builder["cost_receipts"].as_array().unwrap() {
        let nw = (10.0 * setup["native_weight_gae"].as_f64().unwrap()).round() as u64;
        counted.insert(
            format!("builder_setup_cost_native_tenths{nw}"),
            setup["emitted_total"].clone(),
        );
    }
    for cost in harvest["cost_receipts"].as_array().unwrap() {
        let ml = cost["mod_ell_price"].as_f64().unwrap() as u64;
        let native_weight = cost["native_weight_gae"].as_f64().unwrap();
        let nw = (10.0 * native_weight).round() as u64;
        counted.insert(
            format!("attack_complete_cost_mell{ml}_native_tenths{nw}"),
            cost["attack_total"].clone(),
        );
        let builder_total = builder["cost_receipts"]
            .as_array()
            .unwrap()
            .iter()
            .find(|setup| setup["native_weight_gae"].as_f64() == Some(native_weight))
            .and_then(|setup| setup["emitted_total"].as_f64())
            .expect("matching builder native-band receipt");
        counted.insert(
            format!("combined_builder_attack_cost_mell{ml}_native_tenths{nw}"),
            Value::from(builder_total + cost["attack_total"].as_f64().unwrap()),
        );
    }
    let mut counters = harvest["counters"].as_object().unwrap().clone();
    for field in [
        "streams_attempted",
        "candidates_total",
        "fq_mul",
        "fq_inv",
        "fq_sqrt",
        "hash_lookups",
        "hash_inserts",
        "rng_draws",
        "sort_items",
        "primality_modular_ops",
        "native_counter_count",
    ] {
        counters.insert(format!("builder_{field}"), builder[field].clone());
    }
    (Value::Object(counted), Value::Object(counters))
}

fn canonical_flow(runs: &[Value], planned: usize) -> Value {
    let mut excluded: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for run in runs {
        if run["exclusion"]["excluded"].as_bool() == Some(true) {
            excluded
                .entry(
                    run["exclusion"]["rule"]
                        .as_str()
                        .unwrap_or("unspecified")
                        .to_string(),
                )
                .or_default()
                .push(run["run_id"].as_str().unwrap().to_string());
        }
    }
    json!({
        "planned": planned,
        "attempted": runs
            .iter()
            .filter(|run| run["attempts"].as_u64().unwrap_or(0) >= 1)
            .count(),
        "completed": runs
            .iter()
            .filter(|run| run["outcome_class"] == "completed")
            .count(),
        "not_completed": runs
            .iter()
            .filter(|run| run["outcome_class"] == "not_completed")
            .count(),
        "interrupted": runs.iter().filter(|run| run["status"] == "interrupted").count(),
        "resumed": runs
            .iter()
            .filter(|run| run["resumed"].as_u64().unwrap_or(0) > 0)
            .count(),
        "excluded": excluded
            .into_iter()
            .map(|(rule, run_ids)| json!({"rule":rule,"run_ids":run_ids}))
            .collect::<Vec<_>>(),
    })
}

fn complete_export(
    protocol_path: &Path,
    manifest_path: &Path,
    source: &FrozenSource,
    protocol: &Value,
    analysis_path: &Path,
    gates_path: &Path,
    analysis: &Value,
    completions: &[CompletionAudit],
    inputs: &[String],
    instances: &[Value],
    harvests: &[Value],
) -> Value {
    let mut runs = vec![];
    for harvest in harvests {
        let q = harvest["q"].as_u64().unwrap();
        let idx = harvest["curve_index"].as_u64().unwrap();
        let qpos = ALLOWED_Q.iter().position(|&candidate| candidate == q).unwrap();
        let instance = instances
            .iter()
            .find(|instance| {
                instance["q"].as_u64() == Some(q)
                    && instance["curve_index"].as_u64() == Some(idx)
            })
            .expect("paired instance for canonical export");
        let receipt = completions[qpos].receipt.as_ref().unwrap();
        let raw_binding = file_binding(Path::new(&inputs[qpos]), "raw");
        let instance_id = format!("q{q}-curve{idx}");
        let worker = format!(
            "node={};job={};job_uid={};pod={};pod_uid={}",
            receipt["pod"]["node_name"].as_str().unwrap_or("unscheduled"),
            receipt["job"]["name"].as_str().unwrap(),
            receipt["job"]["uid"].as_str().unwrap(),
            receipt["pod"]["name"].as_str().unwrap(),
            receipt["pod"]["uid"].as_str().unwrap(),
        );
        let backend_ref = format!(
            "request_sha256={};job_uid={}",
            receipt["request_sha256"].as_str().unwrap(),
            receipt["job"]["uid"].as_str().unwrap()
        );
        let (counted, candidate_counters) = candidate_export_data(instance, harvest);
        let max_rss = receipt["resource_provenance"]["max_rss_bytes"].clone();
        let common = json!({
            "instance_id": instance_id,
            "seed": harvest["seed"],
            "replicate": 0,
            "warmup": false,
            "status": "succeeded",
            "outcome_class": "completed",
            "attempts": receipt["attempts"],
            "resumed": 0,
            "checkpoint": null,
            "isolation": {
                "tier": "none",
                "contended": null,
                "blockers": ["isolation none by design; lifecycle clocks are provenance only"],
            },
            "exclusion": {"excluded": false, "rule": null},
            "artifacts": [raw_binding],
            "started_at": receipt["started_at"],
            "finished_at": receipt["finished_at"],
            "worker": worker,
            "backend_ref": backend_ref,
        });
        let mut candidate = common.as_object().unwrap().clone();
        candidate.insert(
            "run_id".to_string(),
            Value::String(format!("lead15-dt-q{q}-c{idx}")),
        );
        candidate.insert("arm".to_string(), Value::String("dt-common-cover".to_string()));
        candidate.insert(
            "measurements".to_string(),
            export_measurements(
                counted,
                candidate_counters,
                max_rss.clone(),
            ),
        );
        candidate.insert(
            "verification".to_string(),
            json!({"status":"verified","verifier":"strict LEAD-15 identity, arithmetic, graph, rank, cost and gate-input analyzer"}),
        );
        runs.push(Value::Object(candidate));

    }
    assert_eq!(runs.len(), 24);
    let flow = canonical_flow(&runs, 24);
    let created = completions
        .iter()
        .filter_map(|c| c.receipt.as_ref()?.get("collected_at")?.as_str())
        .max()
        .unwrap_or("post-run-analysis")
        .to_string();
    let script_rel = protocol["analysis_plan"]["script"]["path"].as_str().unwrap();
    let script_path = protocol_path.parent().unwrap().join(script_rel);
    let analysis_bytes = pretty_json_bytes(analysis);
    let gates_bytes = pretty_json_bytes(&analysis["gates"]);
    let mut files = vec![
        file_binding(protocol_path, "protocol"),
        file_binding(manifest_path, "receipt"),
        file_binding(
            &manifest_path.parent().unwrap().join("k4-reference.json"),
            "analysis",
        ),
    ];
    for path in completions.iter().map(|c| c.path.as_str()) {
        files.push(file_binding(Path::new(path), "receipt"));
    }
    for completion in completions {
        append_lineage_file_bindings(&mut files, completion);
    }
    for path in inputs {
        files.push(file_binding(Path::new(path), "raw"));
    }
    let backend_receipts: Vec<Value> = completions
        .iter()
        .map(|c| c.receipt.as_ref().unwrap().clone())
        .collect();
    json!({
        "schema": "experiment.export/v1",
        "export_id": "lead15-dt-e5-20261101-complete",
        "created": created,
        "protocol": {
            "path": absolute_binding_path(protocol_path).display().to_string(),
            "sha256": source.identity.protocol_sha256.as_str(),
            "protocol_id": "LEAD15-DT-E5-20261101",
            "version": protocol["version"],
        },
        "producer": {"runtime":"native-rust","model":null,"session":null,"role":"analysis"},
        "source": {"repo":"crypto","commit":source.identity.commit.as_str(),"dirty":false},
        "backend": "isolab-k8s",
        "host": {
            "path": null,
            "sha256": null,
            "class_id": null,
            "summary": {
                "cpu_model": null,
                "logical_cpus": null,
                "memory_bytes": null,
                "kernel": null,
                "hypervisor": null,
                "isolation_tier_max": "none",
                "instance_type": null,
                "spot": null,
            },
        },
        "arms": [
            {
                "id":"dt-common-cover",
                "role":"candidate",
                "binary_sha256":source.identity.binary_sha256.as_str(),
                "commit":source.identity.commit.as_str(),
                "image_digest":source.identity.image_digest.as_str(),
            },
        ],
        "runs": runs,
        "flow": flow,
        "files": files,
        "analysis": {
            "script": file_binding(&script_path,"analysis"),
            "blinded": false,
            "blinding_key_sha256": null,
            "outputs": [
                file_binding_from_bytes(analysis_path,"summary",&analysis_bytes),
                file_binding_from_bytes(gates_path,"summary",&gates_bytes)
            ],
            "unmasked_at": null,
        },
        "notes": [
            {"kind":"observation","text":"All lifecycle clock fields are non-scientific provenance and wall_valid is false on every row.","by":"lead15 analyzer"},
            {"kind":"observation","text":"The 24 matched formula references are analysis-only derived values, not execution rows; no reference binary or exact-E hyperelliptic measurement was executed.","by":"lead15 analyzer"}
        ],
        "reviews_dir":"reviews",
        "backend_record":{"completion_receipts":backend_receipts},
    })
}

fn assert_exact_object_keys(value: &Value, label: &str, expected: &[&str]) {
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("{label}: expected object"));
    let mut actual: Vec<&str> = object.keys().map(String::as_str).collect();
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected, "{label}: exact schema keys");
}

fn assert_array_len(value: &Value, label: &str, len: usize) {
    assert_eq!(
        value
            .as_array()
            .unwrap_or_else(|| panic!("{label}: expected array"))
            .len(),
        len,
        "{label}: array length"
    );
}

fn assert_result_record_schema(record: &Value, line: usize) {
    let label = format!("line_{line}");
    match record["record"]
        .as_str()
        .unwrap_or_else(|| panic!("{label}: record type"))
    {
        "run_start" => {
            assert_exact_object_keys(
                record,
                &label,
                &[
                    "schema",
                    "record",
                    "protocol_id",
                    "q",
                    "curves",
                    "base_seed",
                    "max_candidates",
                    "retain_factor",
                    "runtime_provenance",
                    "commit",
                    "protocol_sha256",
                    "binary_sha256",
                    "image_digest",
                ],
            );
            assert_exact_object_keys(
                &record["runtime_provenance"],
                &format!("{label}.runtime_provenance"),
                &[
                    "isolation",
                    "cpu_model",
                    "available_parallelism",
                    "kubernetes_node",
                    "kubernetes_job",
                    "kubernetes_pod",
                    "kubernetes_pod_uid",
                ],
            );
        }
        "instance" => {
            assert_exact_object_keys(
                record,
                &label,
                &[
                    "schema",
                    "record",
                    "protocol_id",
                    "q",
                    "curve_index",
                    "stream_retry",
                    "seed",
                    "field_modulus",
                    "ub",
                    "uc",
                    "quartic",
                    "order_decimal",
                    "cofactor",
                    "ell",
                    "trace",
                    "candidate_count",
                    "rejects",
                    "builder_setup",
                    "m2_certificate",
                    "m3_certificate",
                    "common_branch_certificate",
                    "commit",
                    "protocol_sha256",
                    "binary_sha256",
                    "image_digest",
                ],
            );
            assert_exact_object_keys(
                &record["field_modulus"],
                &format!("{label}.field_modulus"),
                &["x3_a", "constant_b"],
            );
            assert_array_len(&record["ub"], &format!("{label}.ub"), 3);
            assert_array_len(&record["uc"], &format!("{label}.uc"), 3);
            assert_array_len(&record["quartic"], &format!("{label}.quartic"), 15);
            assert_exact_object_keys(
                &record["rejects"],
                &format!("{label}.rejects"),
                &["no_common_cover", "quartic_core", "order_or_cofactor"],
            );
            let builder = &record["builder_setup"];
            assert_exact_object_keys(
                builder,
                &format!("{label}.builder_setup"),
                &[
                    "streams_attempted",
                    "candidates_total",
                    "rejects_total",
                    "fq_mul",
                    "fq_inv",
                    "fq_sqrt",
                    "hash_lookups",
                    "hash_inserts",
                    "rng_draws",
                    "sort_items",
                    "primality_modular_ops",
                    "fq_arithmetic",
                    "lookup_cost",
                    "native_counter_count",
                    "inversion_price",
                    "sqrt_price",
                    "cost_receipts",
                ],
            );
            assert_exact_object_keys(
                &builder["rejects_total"],
                &format!("{label}.builder_setup.rejects_total"),
                &[
                    "no_common_cover",
                    "quartic_core",
                    "order_or_cofactor",
                    "duplicate_quartic_streams",
                ],
            );
            let costs = builder["cost_receipts"]
                .as_array()
                .unwrap_or_else(|| panic!("{label}.builder_setup.cost_receipts: array"));
            assert_eq!(costs.len(), 3, "{label}: builder cost bands");
            for (cost_index, cost) in costs.iter().enumerate() {
                assert_exact_object_keys(
                    cost,
                    &format!("{label}.builder_setup.cost_receipts[{cost_index}]"),
                    &[
                        "native_weight_gae",
                        "gae_price",
                        "native_band_cost",
                        "emitted_total",
                    ],
                );
            }
            assert_exact_object_keys(
                &record["m2_certificate"],
                &format!("{label}.m2_certificate"),
                &["z", "target_ordered"],
            );
            assert_exact_object_keys(
                &record["m3_certificate"],
                &format!("{label}.m3_certificate"),
                &["a", "b", "r", "r_q", "source_ordered"],
            );
            assert_exact_object_keys(
                &record["common_branch_certificate"],
                &format!("{label}.common_branch_certificate"),
                &["cross_ratio", "mobius", "verified_all_four_points"],
            );
        }
        "harvest" => {
            assert_exact_object_keys(
                record,
                &label,
                &[
                    "schema",
                    "record",
                    "protocol_id",
                    "q",
                    "curve_index",
                    "seed",
                    "ell",
                    "n_factor_base",
                    "pairs",
                    "yield",
                    "large_prime_graph",
                    "selection_sge",
                    "linear_algebra",
                    "cost_receipts",
                    "references",
                    "counters",
                    "commit",
                    "protocol_sha256",
                    "binary_sha256",
                    "image_digest",
                ],
            );
            assert_exact_object_keys(
                &record["yield"],
                &format!("{label}.yield"),
                &[
                    "raw_full",
                    "raw_fp",
                    "raw_pp",
                    "unique_full",
                    "unique_fp",
                    "unique_pp",
                    "raw_fp_per_pair",
                    "raw_pp_per_pair",
                    "unique_fp_per_pair",
                    "unique_pp_per_pair",
                ],
            );
            assert_exact_object_keys(
                &record["large_prime_graph"],
                &format!("{label}.large_prime_graph"),
                &[
                    "vertices",
                    "edges",
                    "giant_fraction",
                    "cycle_count",
                    "usable_cycles_including_full",
                ],
            );
            assert_exact_object_keys(
                &record["selection_sge"],
                &format!("{label}.selection_sge"),
                &[
                    "retained_tree_edges",
                    "retained_extra_edges",
                    "materialized_tree_edges",
                    "materialized_full_rows",
                    "materialized_rows",
                    "materialized_dependency_dimension",
                    "rows_after_lp_sge",
                    "total_row_weight_after_lp_sge",
                    "mean_row_weight_after_lp_sge",
                    "max_row_weight_after_lp_sge",
                    "mean_pivot_weight_after_rank_sge",
                ],
            );
            assert_exact_object_keys(
                &record["linear_algebra"],
                &format!("{label}.linear_algebra"),
                &["rank", "nullity"],
            );
            let costs = record["cost_receipts"]
                .as_array()
                .unwrap_or_else(|| panic!("{label}.cost_receipts: array"));
            assert_eq!(costs.len(), 6, "{label}: attack cost bands");
            for (cost_index, cost) in costs.iter().enumerate() {
                assert_exact_object_keys(
                    cost,
                    &format!("{label}.cost_receipts[{cost_index}]"),
                    &[
                        "fq_arithmetic",
                        "lookup_cost",
                        "mod_ell_cost",
                        "native_counter_count",
                        "native_band_cost",
                        "attack_total",
                        "verification_cost_excluded",
                        "inversion_price",
                        "sqrt_price",
                        "mod_ell_price",
                        "native_weight_gae",
                        "gae_price",
                    ],
                );
            }
            assert_exact_object_keys(
                &record["references"],
                &format!("{label}.references"),
                &[
                    "rho_same_instance_relation_only",
                    "rho_formula",
                    "matched_hyperelliptic_2p15",
                    "matched_hyper_formula",
                    "quartic_projected_2p15",
                    "quartic_projection_formula",
                    "k4_threshold",
                ],
            );
            assert_exact_object_keys(
                &record["counters"],
                &format!("{label}.counters"),
                &[
                    "lines",
                    "split",
                    "rel_full",
                    "rel_fp",
                    "rel_pp",
                    "duplicates",
                    "verified",
                    "verify_failed",
                    "hash_lookups",
                    "hash_inserts",
                    "rng_draws",
                    "uf_ops",
                    "root_calls",
                    "normalizations",
                    "fq_mul_collect",
                    "fq_inv_collect",
                    "fq_sqrt_collect",
                    "fq_mul_fb",
                    "fq_inv_fb",
                    "fq_sqrt_fb",
                    "fq_mul_verify",
                    "fq_inv_verify",
                    "fq_sqrt_verify",
                    "lp_sge_ops",
                    "lp_sge_entries",
                    "rank_sge_ops",
                    "rank_inversions",
                    "rank_entries",
                    "sort_items",
                ],
            );
        }
        "run_end" => assert_exact_object_keys(
            record,
            &label,
            &[
                "schema",
                "record",
                "protocol_id",
                "q",
                "instances",
                "harvests",
                "complete",
                "commit",
                "protocol_sha256",
                "binary_sha256",
                "image_digest",
            ],
        ),
        other => panic!("{label}: unknown record type {other:?}"),
    }
}

fn parse_k(value: &Value, q: u64, label: &str) -> [u64; 3] {
    let values = value
        .as_array()
        .unwrap_or_else(|| panic!("{label}: expected K array"));
    assert_eq!(values.len(), 3, "{label}: K array length");
    let mut out = [0u64; 3];
    for (index, value) in values.iter().enumerate() {
        out[index] = value
            .as_u64()
            .unwrap_or_else(|| panic!("{label}[{index}]: expected u64"));
        assert!(out[index] < q, "{label}[{index}]: coefficient outside F_q");
    }
    out
}

fn validate_common_cover_record(record: &Value, q: u64, kf: &Kf) {
    assert_eq!(
        record["field_modulus"]["x3_a"].as_u64(),
        Some(kf.a),
        "field modulus x3 coefficient"
    );
    assert_eq!(
        record["field_modulus"]["constant_b"].as_u64(),
        Some(kf.b),
        "field modulus constant"
    );
    let a = record["m3_certificate"]["a"]
        .as_u64()
        .expect("m3 certificate a");
    let b = record["m3_certificate"]["b"]
        .as_u64()
        .expect("m3 certificate b");
    assert!(a < q && b < q && a != b, "m3 base-field branch points");
    let r = parse_k(&record["m3_certificate"]["r"], q, "m3_certificate.r");
    let (witness, ub, uc, quartic) = replay_common_cover_model(kf, a, b, r)
        .unwrap_or_else(|e| panic!("common-cover replay: {e}"));
    assert_eq!(record["m3_certificate"]["r_q"], json!(witness.rq));
    assert_eq!(
        record["m3_certificate"]["source_ordered"],
        json!(witness.source_ordered)
    );
    assert_eq!(record["m2_certificate"]["z"], json!(witness.z));
    assert_eq!(
        record["m2_certificate"]["target_ordered"],
        json!(witness.target_ordered)
    );
    assert_eq!(
        record["common_branch_certificate"]["cross_ratio"],
        json!(witness.cross_ratio)
    );
    assert_eq!(
        record["common_branch_certificate"]["mobius"],
        json!(witness.mobius)
    );
    assert_eq!(
        record["common_branch_certificate"]["verified_all_four_points"],
        true
    );
    assert_eq!(record["ub"], json!(ub), "replayed common-cover ub");
    assert_eq!(record["uc"], json!(uc), "replayed common-cover uc");
    assert_eq!(record["quartic"], json!(quartic), "replayed quartic");
}

fn assert_harvest_count_partition(
    pairs: u64,
    split: u64,
    verified: u64,
    duplicates: u64,
    raw: [u64; 3],
    unique: [u64; 3],
) {
    let raw_total: u128 = raw.iter().map(|&value| value as u128).sum();
    let unique_total: u128 = unique.iter().map(|&value| value as u128).sum();
    assert!(split <= pairs, "at most one split section is classified per pair");
    assert_eq!(raw_total, split as u128, "raw classes partition split sections");
    assert!(verified <= split, "verified sections cannot exceed split sections");
    assert_eq!(
        unique_total,
        verified as u128,
        "unique classes partition verified sections"
    );
    assert_eq!(
        unique_total + duplicates as u128,
        raw_total,
        "unique plus duplicate sections must reproduce raw classified sections"
    );
    for (class_index, (&unique_count, &raw_count)) in unique.iter().zip(raw.iter()).enumerate() {
        assert!(
            unique_count <= raw_count,
            "unique class {class_index} cannot exceed its raw class"
        );
    }
}

fn assert_run_start_runtime_binding(start: &Value, completion_receipt: &Value) {
    let runtime = &start["runtime_provenance"];
    assert_eq!(runtime["isolation"], "none");
    assert!(runtime["cpu_model"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(runtime["available_parallelism"]
        .as_u64()
        .is_some_and(|value| value >= 1));
    for (runtime_field, receipt_value) in [
        ("kubernetes_pod", &completion_receipt["pod"]["name"]),
        ("kubernetes_pod_uid", &completion_receipt["pod"]["uid"]),
        ("kubernetes_node", &completion_receipt["pod"]["node_name"]),
        ("kubernetes_job", &completion_receipt["job"]["name"]),
    ] {
        assert_eq!(&runtime[runtime_field], receipt_value, "{runtime_field} binding");
        assert!(runtime[runtime_field]
            .as_str()
            .is_some_and(|value| !value.is_empty()));
    }
    assert_eq!(
        runtime["cpu_model"], completion_receipt["resource_provenance"]["cpu_model"],
        "runtime CPU model binding"
    );
}

fn assert_row_weight_certificate(
    n_factor_base: u64,
    rows: u64,
    total_weight: u64,
    max_weight: u64,
    emitted_mean: f64,
) {
    if rows == 0 {
        assert_eq!(total_weight, 0, "zero LP-SGE rows have zero total weight");
        assert_eq!(max_weight, 0, "zero LP-SGE rows have zero maximum weight");
        assert_close("mean row weight", emitted_mean, 0.0);
        return;
    }
    assert!(rows > 0, "LP-SGE row count must be positive");
    assert!(max_weight > 0, "every retained LP-SGE row is nonempty");
    assert!(max_weight <= n_factor_base, "row weight cannot exceed the column count");
    assert!(total_weight >= rows, "every retained LP-SGE row contributes at least one entry");
    assert!(total_weight >= max_weight, "total row weight must cover the maximum row");
    assert!(total_weight <= rows.checked_mul(n_factor_base).expect("row/column bound overflow"));
    assert!(total_weight <= rows.checked_mul(max_weight).expect("row/max bound overflow"));
    assert_close(
        "mean row weight",
        emitted_mean,
        total_weight as f64 / rows as f64,
    );
}

fn validate_complete_cell_panicking(
    prefix: &PrefixRead,
    q: u64,
    identity: &RunIdentity,
    completion_receipt: &Value,
) {
    assert!(prefix.problems.is_empty(), "JSONL parse problems: {:?}", prefix.problems);
    assert!(prefix.trailing_fragment.is_empty(), "trailing JSONL fragment");
    assert_eq!(prefix.records.len(), 14, "exact result record count");
    for (i, v) in prefix.records.iter().enumerate() {
        assert_result_record_schema(v, i + 1);
        assert_eq!(v["schema"], "lead15.diem-quartic/v1");
        assert_eq!(v["protocol_id"], "LEAD15-DT-E5-20261101");
        assert_eq!(v["q"].as_u64(), Some(q));
        assert_record_identity(v, identity, "authenticated result record");
    }
    assert_eq!(prefix.records[0]["record"], "run_start");
    assert!(prefix.records[1..=CURVES_PER_SIZE as usize]
        .iter()
        .all(|record| record["record"] == "instance"));
    assert!(prefix.records[(CURVES_PER_SIZE as usize + 1)..13]
        .iter()
        .all(|record| record["record"] == "harvest"));
    assert_eq!(prefix.records[13]["record"], "run_end");
    let start = &prefix.records[0];
    assert_eq!(start["curves"].as_u64(), Some(CURVES_PER_SIZE as u64));
    assert_eq!(start["base_seed"].as_u64(), Some(BASE_SEED));
    assert_eq!(start["max_candidates"].as_u64(), Some(MAX_CANDIDATES));
    assert_eq!(start["retain_factor"].as_u64(), Some(RETAIN_FACTOR as u64));
    assert_run_start_runtime_binding(start, completion_receipt);
    let end = &prefix.records[13];
    assert_eq!(end["instances"].as_u64(), Some(CURVES_PER_SIZE as u64));
    assert_eq!(end["harvests"].as_u64(), Some(CURVES_PER_SIZE as u64));
    assert_eq!(end["complete"].as_bool(), Some(true));

    let kf = Kf::new(q);
    let mut indices = HashSet::new();
    let mut quartics = HashSet::new();
    let mut instance_identity = HashMap::new();
    for record in &prefix.records[1..=CURVES_PER_SIZE as usize] {
        let idx = record["curve_index"].as_u64().expect("instance curve_index");
        assert!(idx < CURVES_PER_SIZE as u64 && indices.insert(idx));
        let retry = record["stream_retry"].as_u64().expect("stream_retry");
        assert!(retry <= u32::MAX as u64);
        let seed = record["seed"].as_u64().expect("instance seed");
        assert_eq!(seed, seed_for(q, idx as u32, retry as u32));
        assert!(quartics.insert(record["quartic"].to_string()));
        for coefficient in record["quartic"].as_array().expect("quartic array") {
            assert!(coefficient.as_u64().is_some_and(|value| value < q));
        }
        validate_common_cover_record(record, q, &kf);
        let ell = record["ell"].as_u64().expect("instance ell");
        assert_eq!(record["cofactor"].as_u64(), Some(4));
        assert!(is_prime_u64(ell));
        assert!((ell as u128).pow(2) > (q as u128).pow(5));
        let order = record["order_decimal"]
            .as_str()
            .expect("order_decimal string")
            .parse::<u128>()
            .expect("order_decimal u128");
        assert_eq!(order, 4 * ell as u128);
        let trace = record["trace"]
            .as_str()
            .expect("trace string")
            .parse::<i128>()
            .expect("trace i128");
        assert_eq!(trace, (q as i128).pow(3) + 1 - order as i128);
        let candidate_count = record["candidate_count"]
            .as_u64()
            .expect("candidate_count");
        assert!((1..=MAX_CANDIDATES).contains(&candidate_count));
        let rejects = &record["rejects"];
        assert_eq!(
            rejects["no_common_cover"].as_u64().unwrap()
                + rejects["quartic_core"].as_u64().unwrap()
                + rejects["order_or_cofactor"].as_u64().unwrap()
                + 1,
            candidate_count
        );
        let builder = &record["builder_setup"];
        assert_eq!(builder["streams_attempted"].as_u64(), Some(retry + 1));
        assert_eq!(
            builder["rejects_total"]["duplicate_quartic_streams"].as_u64(),
            Some(retry)
        );
        assert_eq!(
            builder["rejects_total"]["no_common_cover"].as_u64().unwrap()
                + builder["rejects_total"]["quartic_core"].as_u64().unwrap()
                + builder["rejects_total"]["order_or_cofactor"].as_u64().unwrap()
                + builder["streams_attempted"].as_u64().unwrap(),
            builder["candidates_total"].as_u64().unwrap()
        );
        let inversion_price = 64 - q.leading_zeros() as u64;
        let sqrt_price = (1.5 * (q as f64).log2()).ceil() as u64;
        let gae_price = 163.0 + 2.0 * inversion_price as f64;
        assert_eq!(builder["inversion_price"].as_u64(), Some(inversion_price));
        assert_eq!(builder["sqrt_price"].as_u64(), Some(sqrt_price));
        let expected_fq = builder["fq_mul"].as_u64().unwrap() as f64
            + inversion_price as f64 * builder["fq_inv"].as_u64().unwrap() as f64
            + sqrt_price as f64 * builder["fq_sqrt"].as_u64().unwrap() as f64;
        let expected_native = builder["hash_lookups"].as_u64().unwrap()
            + builder["hash_inserts"].as_u64().unwrap()
            + builder["rng_draws"].as_u64().unwrap()
            + builder["sort_items"].as_u64().unwrap()
            + builder["primality_modular_ops"].as_u64().unwrap();
        assert_close("builder fq arithmetic", builder["fq_arithmetic"].as_f64().unwrap(), expected_fq);
        assert_close(
            "builder lookup cost",
            builder["lookup_cost"].as_f64().unwrap(),
            4.0 * builder["hash_lookups"].as_u64().unwrap() as f64,
        );
        assert_eq!(builder["native_counter_count"].as_u64(), Some(expected_native));
        let mut bands = HashSet::new();
        for cost in builder["cost_receipts"].as_array().unwrap() {
            let weight = cost["native_weight_gae"].as_f64().unwrap();
            assert!(weight == 0.0 || weight == 0.1 || weight == 1.0);
            assert!(bands.insert(format!("{weight:.1}")));
            assert_close("builder GAE price", cost["gae_price"].as_f64().unwrap(), gae_price);
            assert_close(
                "builder native cost",
                cost["native_band_cost"].as_f64().unwrap(),
                weight * gae_price * expected_native as f64,
            );
            assert_close(
                "builder total",
                cost["emitted_total"].as_f64().unwrap(),
                expected_fq
                    + 4.0 * builder["hash_lookups"].as_u64().unwrap() as f64
                    + weight * gae_price * expected_native as f64,
            );
        }
        assert!(instance_identity.insert(idx, (seed, ell)).is_none());
    }
    assert_eq!(indices.len(), CURVES_PER_SIZE as usize);

    let mut harvest_indices = HashSet::new();
    for record in &prefix.records[(CURVES_PER_SIZE as usize + 1)..13] {
        let idx = record["curve_index"].as_u64().expect("harvest curve_index");
        assert!(idx < CURVES_PER_SIZE as u64 && harvest_indices.insert(idx));
        let (seed, ell) = *instance_identity.get(&idx).expect("paired instance");
        assert_eq!(record["seed"].as_u64(), Some(seed));
        assert_eq!(record["ell"].as_u64(), Some(ell));
        let n = record["n_factor_base"].as_u64().expect("factor base size");
        assert_eq!(n as usize, prospective_factor_base_size(q));
        let pairs = record["pairs"].as_u64().expect("pair count");
        assert_eq!(pairs, n * (n - 1) / 2);
        let counters = &record["counters"];
        assert_eq!(counters["lines"].as_u64(), Some(pairs));
        assert_eq!(counters["verify_failed"].as_u64(), Some(0));
        let yield_ = &record["yield"];
        let raw_full = yield_["raw_full"].as_u64().unwrap();
        let raw_fp = yield_["raw_fp"].as_u64().unwrap();
        let raw_pp = yield_["raw_pp"].as_u64().unwrap();
        let full = yield_["unique_full"].as_u64().unwrap();
        let fp = yield_["unique_fp"].as_u64().unwrap();
        let pp = yield_["unique_pp"].as_u64().unwrap();
        assert_harvest_count_partition(
            pairs,
            counters["split"].as_u64().unwrap(),
            counters["verified"].as_u64().unwrap(),
            counters["duplicates"].as_u64().unwrap(),
            [raw_full, raw_fp, raw_pp],
            [full, fp, pp],
        );
        assert_eq!(counters["rel_full"].as_u64(), Some(full));
        assert_eq!(counters["rel_fp"].as_u64(), Some(fp));
        assert_eq!(counters["rel_pp"].as_u64(), Some(pp));
        for (name, numerator) in [
            ("raw_fp_per_pair", raw_fp),
            ("raw_pp_per_pair", raw_pp),
            ("unique_fp_per_pair", fp),
            ("unique_pp_per_pair", pp),
        ] {
            assert_close(name, yield_[name].as_f64().unwrap(), numerator as f64 / pairs as f64);
        }
        let graph = &record["large_prime_graph"];
        assert_eq!(graph["edges"].as_u64(), Some(fp + pp));
        let cycle_count = graph["cycle_count"].as_i64().expect("cycle_count");
        assert!(cycle_count >= 0);
        assert!(cycle_count <= (fp + pp) as i64);
        assert_eq!(
            graph["usable_cycles_including_full"].as_i64(),
            Some(full as i64 + cycle_count)
        );
        let selection = &record["selection_sge"];
        let materialized_dependency_dimension = selection["materialized_dependency_dimension"]
            .as_u64()
            .expect("materialized_dependency_dimension");
        assert!(
            materialized_dependency_dimension <= cycle_count as u64,
            "retained certificate cannot exceed full-graph dependency dimension"
        );
        assert_eq!(
            selection["materialized_rows"].as_u64(),
            Some(
                selection["materialized_tree_edges"].as_u64().unwrap()
                    + selection["retained_extra_edges"].as_u64().unwrap()
                    + selection["materialized_full_rows"].as_u64().unwrap()
            )
        );
        assert!(
            selection["materialized_tree_edges"].as_u64().unwrap()
                <= selection["retained_tree_edges"].as_u64().unwrap()
        );
        assert!(
            selection["retained_tree_edges"].as_u64().unwrap()
                <= graph["vertices"].as_u64().unwrap()
        );
        assert!(
            selection["materialized_full_rows"].as_u64().unwrap()
                + selection["retained_extra_edges"].as_u64().unwrap()
                <= RETAIN_FACTOR as u64 * n + 1024
        );
        assert!(
            selection["materialized_tree_edges"].as_u64().unwrap()
                <= selection["retained_tree_edges"].as_u64().unwrap()
        );
        assert!(selection["materialized_full_rows"].as_u64().unwrap() <= full);
        let rows = selection["rows_after_lp_sge"].as_u64().unwrap();
        assert_eq!(
            rows,
            selection["materialized_full_rows"].as_u64().unwrap()
                + materialized_dependency_dimension,
            "exact LP elimination must reproduce the retained certificate dimension"
        );
        let total_weight = selection["total_row_weight_after_lp_sge"].as_u64().unwrap();
        assert_row_weight_certificate(
            n,
            rows,
            total_weight,
            selection["max_row_weight_after_lp_sge"].as_u64().unwrap(),
            selection["mean_row_weight_after_lp_sge"].as_f64().unwrap(),
        );
        let linear = &record["linear_algebra"];
        let rank = linear["rank"].as_u64().unwrap();
        assert!(rank <= rows && rank <= n);
        assert_eq!(rank + linear["nullity"].as_u64().unwrap(), n);
        let inversion_price = 64 - q.leading_zeros() as u64;
        let sqrt_price = (1.5 * (q as f64).log2()).ceil() as u64;
        let gae_price = 163.0 + 2.0 * inversion_price as f64;
        let expected_fq = (counters["fq_mul_collect"].as_u64().unwrap()
            + counters["fq_mul_fb"].as_u64().unwrap()) as f64
            + inversion_price as f64
                * (counters["fq_inv_collect"].as_u64().unwrap()
                    + counters["fq_inv_fb"].as_u64().unwrap()) as f64
            + sqrt_price as f64
                * (counters["fq_sqrt_collect"].as_u64().unwrap()
                    + counters["fq_sqrt_fb"].as_u64().unwrap()) as f64;
        let expected_verify = counters["fq_mul_verify"].as_u64().unwrap() as f64
            + inversion_price as f64 * counters["fq_inv_verify"].as_u64().unwrap() as f64
            + sqrt_price as f64 * counters["fq_sqrt_verify"].as_u64().unwrap() as f64;
        let expected_native = counters["hash_lookups"].as_u64().unwrap()
            + counters["hash_inserts"].as_u64().unwrap()
            + counters["rng_draws"].as_u64().unwrap()
            + counters["uf_ops"].as_u64().unwrap()
            + counters["root_calls"].as_u64().unwrap()
            + counters["normalizations"].as_u64().unwrap()
            + counters["lp_sge_entries"].as_u64().unwrap()
            + counters["rank_entries"].as_u64().unwrap()
            + counters["sort_items"].as_u64().unwrap();
        let mut bands = HashSet::new();
        for cost in record["cost_receipts"].as_array().unwrap() {
            let mod_price = cost["mod_ell_price"].as_f64().unwrap();
            let native_weight = cost["native_weight_gae"].as_f64().unwrap();
            assert!(mod_price == 1.0 || mod_price == 9.0);
            assert!(native_weight == 0.0 || native_weight == 0.1 || native_weight == 1.0);
            assert!(bands.insert(format!("{mod_price:.1}/{native_weight:.1}")));
            assert_eq!(cost["inversion_price"].as_u64(), Some(inversion_price));
            assert_eq!(cost["sqrt_price"].as_u64(), Some(sqrt_price));
            assert_eq!(cost["native_counter_count"].as_u64(), Some(expected_native));
            assert_close("attack fq", cost["fq_arithmetic"].as_f64().unwrap(), expected_fq);
            assert_close("attack verification", cost["verification_cost_excluded"].as_f64().unwrap(), expected_verify);
            assert_close("attack GAE price", cost["gae_price"].as_f64().unwrap(), gae_price);
            let lookup = 4.0 * counters["hash_lookups"].as_u64().unwrap() as f64;
            let modular = mod_price
                * (counters["lp_sge_ops"].as_u64().unwrap()
                    + counters["rank_sge_ops"].as_u64().unwrap()) as f64;
            let native = native_weight * gae_price * expected_native as f64;
            assert_close("attack lookup", cost["lookup_cost"].as_f64().unwrap(), lookup);
            assert_close("attack mod ell", cost["mod_ell_cost"].as_f64().unwrap(), modular);
            assert_close("attack native", cost["native_band_cost"].as_f64().unwrap(), native);
            assert_close("attack total", cost["attack_total"].as_f64().unwrap(), expected_fq + lookup + modular + native);
        }
        let references = &record["references"];
        assert_eq!(
            references["rho_formula"],
            "sqrt(pi*ell/2)*74.5 FqM; no native-band counters available, so lower-bound reference only"
        );
        assert_eq!(
            references["matched_hyper_formula"],
            "9.967e10*(2^15/2^22)^1.408; q-matched round-4 aggregate extrapolation, not rerun on this exact E"
        );
        assert_eq!(
            references["quartic_projection_formula"],
            "7.3e6+0.53e6*w, w=measured mean row weight after LP SGE"
        );
        assert_close(
            "rho reference",
            references["rho_same_instance_relation_only"].as_f64().unwrap(),
            (std::f64::consts::PI * ell as f64 / 2.0).sqrt() * 74.5,
        );
        assert_close(
            "matched hyper reference",
            references["matched_hyperelliptic_2p15"].as_f64().unwrap(),
            fraction_f64(MATCHED_HYPER_NUM, MATCHED_HYPER_DEN),
        );
        assert_close(
            "quartic projection",
            references["quartic_projected_2p15"].as_f64().unwrap(),
            7_300_000.0
                + 530_000.0 * selection["mean_row_weight_after_lp_sge"].as_f64().unwrap(),
        );
        assert_close(
            "K4 threshold",
            references["k4_threshold"].as_f64().unwrap(),
            fraction_f64(K4_THRESHOLD_NUM, K4_THRESHOLD_DEN),
        );
    }
    assert_eq!(harvest_indices.len(), CURVES_PER_SIZE as usize);
}

fn deep_complete_cell_problems(
    prefix: &PrefixRead,
    q: u64,
    identity: &RunIdentity,
    completion_receipt: Option<&Value>,
) -> Vec<String> {
    let Some(completion_receipt) = completion_receipt else {
        return vec!["missing_authenticated_completion_receipt".to_string()];
    };
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        validate_complete_cell_panicking(prefix, q, identity, completion_receipt)
    })) {
        Ok(()) => vec![],
        Err(payload) => vec![panic_message(payload.as_ref()).replacen(
            "strict_complete_validation_failed:",
            "deep_cell_validation_failed:",
            1,
        )],
    }
}

fn authenticated_partial_attempt_indices(
    prefix: &PrefixRead,
    q: u64,
    identity: &RunIdentity,
    completion_receipt: &Value,
) -> Result<Vec<u64>, String> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(prefix.problems.is_empty(), "JSONL parse problems: {:?}", prefix.problems);
        assert!(prefix.records.len() <= 14, "too many result records");
        if prefix.records.is_empty() {
            return vec![];
        }

        let mut instance_identity: [Option<(u64, u64)>; CURVES_PER_SIZE as usize] =
            [None; CURVES_PER_SIZE as usize];
        let mut attempted = vec![];
        for (position, record) in prefix.records.iter().enumerate() {
            assert_result_record_schema(record, position + 1);
            assert_eq!(record["schema"], "lead15.diem-quartic/v1");
            assert_eq!(record["protocol_id"], "LEAD15-DT-E5-20261101");
            assert_eq!(record["q"].as_u64(), Some(q));
            assert_record_identity(record, identity, "authenticated partial result record");
            match position {
                0 => {
                    assert_eq!(record["record"], "run_start");
                    assert_eq!(record["curves"].as_u64(), Some(CURVES_PER_SIZE as u64));
                    assert_eq!(record["base_seed"].as_u64(), Some(BASE_SEED));
                    assert_eq!(record["max_candidates"].as_u64(), Some(MAX_CANDIDATES));
                    assert_eq!(record["retain_factor"].as_u64(), Some(RETAIN_FACTOR as u64));
                    assert_run_start_runtime_binding(record, completion_receipt);
                }
                1..=6 => {
                    let expected_index = (position - 1) as u64;
                    assert_eq!(record["record"], "instance");
                    assert_eq!(record["curve_index"].as_u64(), Some(expected_index));
                    let retry = record["stream_retry"].as_u64().expect("stream_retry");
                    assert!(retry <= u32::MAX as u64);
                    let seed = record["seed"].as_u64().expect("instance seed");
                    assert_eq!(seed, seed_for(q, expected_index as u32, retry as u32));
                    let ell = record["ell"].as_u64().expect("instance ell");
                    instance_identity[expected_index as usize] = Some((seed, ell));
                    attempted.push(expected_index);
                }
                7..=12 => {
                    let expected_index = (position - 7) as u64;
                    assert_eq!(record["record"], "harvest");
                    assert_eq!(record["curve_index"].as_u64(), Some(expected_index));
                    let (seed, ell) = instance_identity[expected_index as usize]
                        .expect("harvest requires its preceding instance record");
                    assert_eq!(record["seed"].as_u64(), Some(seed));
                    assert_eq!(record["ell"].as_u64(), Some(ell));
                }
                13 => {
                    assert_eq!(record["record"], "run_end");
                    assert_eq!(record["instances"].as_u64(), Some(CURVES_PER_SIZE as u64));
                    assert_eq!(record["harvests"].as_u64(), Some(CURVES_PER_SIZE as u64));
                    assert_eq!(record["complete"].as_bool(), Some(true));
                }
                _ => unreachable!(),
            }
        }
        attempted
    })) {
        Ok(attempted) => Ok(attempted),
        Err(payload) => Err(panic_message(payload.as_ref()).replacen(
            "strict_complete_validation_failed:",
            "partial_record_authentication_failed:",
            1,
        )),
    }
}

fn incomplete_export(
    protocol_path: &Path,
    manifest_path: &Path,
    source: &FrozenSource,
    protocol: &Value,
    analysis_path: &Path,
    gates_path: &Path,
    analysis: &Value,
    completions: &[CompletionAudit],
    prefixes: &[PrefixRead],
    inputs: &[String],
    classifications: &[String],
) -> Value {
    let mut runs = vec![];
    for (qpos, &q) in ALLOWED_Q.iter().enumerate() {
        let class = classifications[qpos].as_str();
        // A planned row is emitted only after a locally lineage-validated collector
        // receipt proves that the single frozen attempt actually started. This assumes
        // trusted provider-session acquisition; it is not independent provider
        // attestation. Missing identities or receipts that fail lineage validation
        // remain visible in flow.planned but cannot become fictional attempts.
        let Some(receipt) = completions[qpos].receipt.as_ref() else {
            continue;
        };
        if !completions[qpos].problems.is_empty() || receipt["attempts"].as_u64() != Some(1) {
            continue;
        }
        let Ok(authenticated_indices) =
            authenticated_partial_attempt_indices(
                &prefixes[qpos],
                q,
                &source.identity,
                receipt,
            )
        else {
            continue;
        };
        let raw_artifacts = if Path::new(&inputs[qpos]).is_file() {
            vec![file_binding(Path::new(&inputs[qpos]), "raw")]
        } else {
            vec![]
        };
        let (status, outcome, verification, exclusion_rule) = match class {
            "complete_valid" => ("succeeded", "completed", "verified", None),
            "censored_deadline" => (
                "timeout",
                "not_completed",
                "not_run",
                Some("frozen 3600-second terminal censoring rule"),
            ),
            "scientific_invalid" => (
                "failed",
                "completed",
                "refuted",
                Some("scientific output or verification invalid"),
            ),
            "infrastructure_incomplete" => (
                "infra_error",
                "not_completed",
                "not_run",
                Some("infrastructure incomplete; no replacement"),
            ),
            _ => (
                "interrupted",
                "not_completed",
                "not_run",
                Some("unknown incomplete; no replacement"),
            ),
        };
        let max_rss = receipt["resource_provenance"]["max_rss_bytes"].clone();
        let started_at = receipt["started_at"].clone();
        let finished_at = receipt["finished_at"].clone();
        let worker = Value::String(format!(
            "node={};job={};job_uid={};pod={};pod_uid={}",
            receipt["pod"]["node_name"].as_str().unwrap_or("unscheduled"),
            receipt["job"]["name"].as_str().unwrap(),
            receipt["job"]["uid"].as_str().unwrap(),
            receipt["pod"]["name"].as_str().unwrap(),
            receipt["pod"]["uid"].as_str().unwrap()
        ));
        let backend_ref = Value::String(format!(
            "request_sha256={};job_uid={}",
            receipt["request_sha256"].as_str().unwrap(),
            receipt["job"]["uid"].as_str().unwrap()
        ));
        for idx in authenticated_indices {
            let instance = prefixes[qpos]
                .records
                .iter()
                .find(|v| v["record"] == "instance" && v["curve_index"].as_u64() == Some(idx));
            let harvest = prefixes[qpos]
                .records
                .iter()
                .find(|v| v["record"] == "harvest" && v["curve_index"].as_u64() == Some(idx));
            // The authenticated cell receipt proves one batch attempt, but a
            // per-instance canonical run exists only when durable result bytes
            // identify that curve. Never fan one Job attempt out into six
            // fictional row attempts after an early failure.
            if instance.is_none() && harvest.is_none() {
                continue;
            }
            let seed = instance
                .or(harvest)
                .and_then(|v| v["seed"].as_u64())
                .map(Value::from)
                .unwrap_or(Value::Null);
            let common = json!({
                "instance_id":format!("q{q}-curve{idx}"),
                "seed":seed,
                "replicate":0,
                "warmup":false,
                "status":status,
                "outcome_class":outcome,
                "attempts":receipt["attempts"],
                "resumed":0,
                "checkpoint":null,
                "isolation":{"tier":"none","contended":null,"blockers":["isolation none by design; lifecycle clocks are provenance only"]},
                "exclusion":{"excluded":class!="complete_valid","rule":exclusion_rule},
                "artifacts":raw_artifacts,
                "started_at":started_at,
                "finished_at":finished_at,
                "worker":worker,
                "backend_ref":backend_ref,
            });
            let (candidate_counted, candidate_counters) =
                if class == "complete_valid" {
                    if let (Some(i), Some(h)) = (instance, harvest) {
                        candidate_export_data(i, h)
                    } else {
                        (json!({}), json!({}))
                    }
                } else {
                    (json!({}), json!({}))
                };
            let candidate_id = format!("lead15-dt-q{q}-c{idx}");
            let mut candidate = common.as_object().unwrap().clone();
            candidate.insert("run_id".to_string(), Value::String(candidate_id.clone()));
            candidate.insert("arm".to_string(), Value::String("dt-common-cover".to_string()));
            candidate.insert(
                "measurements".to_string(),
                export_measurements(
                    candidate_counted,
                    candidate_counters,
                    max_rss.clone(),
                ),
            );
            candidate.insert(
                "verification".to_string(),
                json!({"status":verification,"verifier":"terminal LEAD-15 analyzer; no K gate evaluated unless all four cells are complete_valid"}),
            );
            runs.push(Value::Object(candidate));
        }
    }
    assert!(runs.len() <= 24);
    let flow = canonical_flow(&runs, 24);
    let created = completions
        .iter()
        .filter_map(|c| c.receipt.as_ref()?.get("collected_at")?.as_str())
        .max()
        .unwrap_or("post-run-analysis")
        .to_string();
    let script_rel = protocol["analysis_plan"]["script"]["path"].as_str().unwrap();
    let script_path = protocol_path.parent().unwrap().join(script_rel);
    let analysis_bytes = pretty_json_bytes(analysis);
    let gates_bytes = pretty_json_bytes(&analysis["gates"]);
    let mut files = vec![
        file_binding(protocol_path, "protocol"),
        file_binding(manifest_path, "receipt"),
        file_binding(
            &manifest_path.parent().unwrap().join("k4-reference.json"),
            "analysis",
        ),
    ];
    for completion in completions {
        if Path::new(&completion.path).is_file() {
            files.push(file_binding(Path::new(&completion.path), "receipt"));
        }
        append_lineage_file_bindings(&mut files, completion);
    }
    for path in inputs {
        if Path::new(path).is_file() {
            files.push(file_binding(Path::new(path), "raw"));
        }
    }
    let backend_receipts: Vec<Value> = completions
        .iter()
        .map(|c| c.receipt.clone().unwrap_or(Value::Null))
        .collect();
    json!({
        "schema":"experiment.export/v1",
        "export_id":"lead15-dt-e5-20261101-terminal-incomplete",
        "created":created,
        "protocol":{"path":absolute_binding_path(protocol_path).display().to_string(),"sha256":source.identity.protocol_sha256.as_str(),"protocol_id":"LEAD15-DT-E5-20261101","version":protocol["version"]},
        "producer":{"runtime":"native-rust","model":null,"session":null,"role":"analysis"},
        "source":{"repo":"crypto","commit":source.identity.commit.as_str(),"dirty":false},
        "backend":"isolab-k8s",
        "host":{"path":null,"sha256":null,"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":"none","instance_type":null,"spot":null}},
        "arms":[
            {"id":"dt-common-cover","role":"candidate","binary_sha256":source.identity.binary_sha256.as_str(),"commit":source.identity.commit.as_str(),"image_digest":source.identity.image_digest.as_str()}
        ],
        "runs":runs,
        "flow":flow,
        "files":files,
        "analysis":{"script":file_binding(&script_path,"analysis"),"blinded":false,"blinding_key_sha256":null,"outputs":[file_binding_from_bytes(analysis_path,"summary",&analysis_bytes),file_binding_from_bytes(gates_path,"summary",&gates_bytes)],"unmasked_at":null},
        "notes":[
            {"kind":"observation","text":"At least one size cell was not complete_valid; all K gates are not_evaluated and no missing value is imputed.","by":"lead15 analyzer"},
            {"kind":"observation","text":"All lifecycle clock fields are non-scientific provenance and wall_valid is false on every row.","by":"lead15 analyzer"},
            {"kind":"observation","text":"Formula references are analysis-only and never appear as attempted execution rows.","by":"lead15 analyzer"}
        ],
        "reviews_dir":"reviews",
        "backend_record":{"completion_receipts":backend_receipts,"cell_classifications":classifications}
    })
}

fn analyze_complete(
    protocol_path: &Path,
    manifest_path: &Path,
    analysis_path: &Path,
    gates_path: &Path,
    export_path: &Path,
    completion_paths: &[String],
    inputs: &[String],
) {
    assert_eq!(inputs.len(), ALLOWED_Q.len(), "analysis requires exactly four size files");
    assert_eq!(
        completion_paths.len(),
        ALLOWED_Q.len(),
        "analysis requires exactly four completion receipts"
    );
    let source = frozen_source_manifest(manifest_path);
    let protocol = validate_frozen_protocol(protocol_path, &source);
    let prefixes: Vec<PrefixRead> = inputs.iter().map(|path| read_jsonl_prefix(path)).collect();
    let completions: Vec<CompletionAudit> = completion_paths
        .iter()
        .zip(ALLOWED_Q.iter())
        .zip(prefixes.iter())
        .map(|((path, &q), prefix)| audit_completion_receipt(path, q, &source, prefix))
        .collect();
    for ((completion, prefix), &q) in completions
        .iter()
        .zip(prefixes.iter())
        .zip(ALLOWED_Q.iter())
    {
        assert_eq!(
            completion.classification, "receipt_completed",
            "q={q}: completion receipt is not a clean completed cell: {:?}",
            completion.problems
        );
        assert!(prefix.problems.is_empty(), "q={q}: {:?}", prefix.problems);
        assert!(
            prefix.trailing_fragment.is_empty(),
            "q={q}: authenticated payload has a trailing fragment"
        );
        let receipt = completion.receipt.as_ref().unwrap();
        assert_eq!(receipt["payload"]["uploaded"], true);
        assert_eq!(receipt["payload"]["final_newline"], true);
        let problems = deep_complete_cell_problems(
            prefix,
            q,
            &source.identity,
            completion.receipt.as_ref(),
        );
        assert!(problems.is_empty(), "q={q}: {problems:?}");
    }
    let expected_identity = &source.identity;
    let mut instance_records: Vec<Value> = vec![];
    let mut harvests: Vec<Value> = vec![];
    let mut reference_rows: Vec<Value> = vec![];
    let mut size_receipts = vec![];
    for (((path, &expected_q), prefix), completion) in inputs
        .iter()
        .zip(ALLOWED_Q.iter())
        .zip(prefixes.iter())
        .zip(completions.iter())
    {
        let records = prefix.records.clone();
        for v in &records {
            assert_eq!(v["schema"], "lead15.diem-quartic/v1");
            assert_eq!(v["protocol_id"], "LEAD15-DT-E5-20261101");
            assert_eq!(v["q"].as_u64(), Some(expected_q));
            assert_record_identity(v, expected_identity, path);
        }
        assert_eq!(records.len(), 14, "{path}: exact record count");
        assert_eq!(records.first().and_then(|v| v["record"].as_str()), Some("run_start"));
        assert_eq!(records.last().and_then(|v| v["record"].as_str()), Some("run_end"));
        assert_eq!(records.last().unwrap()["complete"], true);
        let runtime_provenance = records[0]["runtime_provenance"].clone();
        assert_eq!(runtime_provenance["isolation"], "none");
        assert!(runtime_provenance["cpu_model"]
            .as_str()
            .is_some_and(|v| !v.is_empty()));
        let completion_receipt = completion.receipt.as_ref().unwrap();
        assert_eq!(
            runtime_provenance["cpu_model"],
            completion_receipt["resource_provenance"]["cpu_model"]
        );
        assert_eq!(
            runtime_provenance["kubernetes_pod"],
            completion_receipt["pod"]["name"]
        );
        assert_eq!(
            runtime_provenance["kubernetes_pod_uid"],
            completion_receipt["pod"]["uid"]
        );
        assert_eq!(
            runtime_provenance["kubernetes_node"],
            completion_receipt["pod"]["node_name"]
        );
        assert_eq!(
            runtime_provenance["kubernetes_job"],
            completion_receipt["job"]["name"]
        );
        for field in [
            "kubernetes_pod",
            "kubernetes_pod_uid",
            "kubernetes_node",
            "kubernetes_job",
        ] {
            assert!(runtime_provenance[field]
                .as_str()
                .is_some_and(|v| !v.is_empty()));
        }
        let instances: Vec<&Value> = records.iter().filter(|v| v["record"] == "instance").collect();
        let hs: Vec<&Value> = records.iter().filter(|v| v["record"] == "harvest").collect();
        assert_eq!(instances.len(), CURVES_PER_SIZE as usize, "{path}: instance count");
        assert_eq!(hs.len(), CURVES_PER_SIZE as usize, "{path}: harvest count");
        let mut indices = HashSet::new();
        let mut quartics = HashSet::new();
        let mut instance_identity: HashMap<u64, (u64, u64)> = HashMap::new();
        for v in &instances {
            assert_eq!(v["cofactor"].as_u64(), Some(4));
            assert_eq!(v["common_branch_certificate"]["verified_all_four_points"], true);
            let idx = v["curve_index"].as_u64().expect("instance curve_index");
            assert!(idx < CURVES_PER_SIZE as u64 && indices.insert(idx));
            assert!(quartics.insert(v["quartic"].to_string()));
            let retry = v["stream_retry"].as_u64().expect("instance stream_retry");
            assert!(retry <= u32::MAX as u64);
            assert_eq!(
                v["seed"].as_u64(),
                Some(seed_for(expected_q, idx as u32, retry as u32))
            );
            let ell = v["ell"].as_u64().expect("instance ell");
            assert!(is_prime_u64(ell));
            assert!((ell as u128) * (ell as u128) > (expected_q as u128).pow(5));
            assert_eq!(
                v["order_decimal"].as_str().unwrap().parse::<u128>().unwrap(),
                4 * ell as u128
            );
            let builder = &v["builder_setup"];
            assert_eq!(
                builder["streams_attempted"].as_u64(),
                Some(v["stream_retry"].as_u64().unwrap() + 1)
            );
            let inversion_price = 64 - expected_q.leading_zeros() as u64;
            let sqrt_price = (1.5 * (expected_q as f64).log2()).ceil() as u64;
            let expected_gae = 163.0 + 2.0 * inversion_price as f64;
            let expected_builder_fq = builder["fq_mul"].as_u64().unwrap() as f64
                + inversion_price as f64 * builder["fq_inv"].as_u64().unwrap() as f64
                + sqrt_price as f64 * builder["fq_sqrt"].as_u64().unwrap() as f64;
            let expected_builder_native = builder["hash_lookups"].as_u64().unwrap()
                + builder["hash_inserts"].as_u64().unwrap()
                + builder["rng_draws"].as_u64().unwrap()
                + builder["sort_items"].as_u64().unwrap()
                + builder["primality_modular_ops"].as_u64().unwrap();
            assert_close(
                "builder fq arithmetic",
                builder["fq_arithmetic"].as_f64().unwrap(),
                expected_builder_fq,
            );
            assert_close(
                "builder lookup cost",
                builder["lookup_cost"].as_f64().unwrap(),
                4.0 * builder["hash_lookups"].as_u64().unwrap() as f64,
            );
            assert_eq!(
                builder["native_counter_count"].as_u64(),
                Some(expected_builder_native)
            );
            let builder_receipts = builder["cost_receipts"].as_array().unwrap();
            assert_eq!(builder_receipts.len(), 3);
            let mut builder_bands = HashSet::new();
            for receipt in builder_receipts {
                let nw = receipt["native_weight_gae"].as_f64().unwrap();
                assert!(nw == 0.0 || nw == 0.1 || nw == 1.0);
                assert!(builder_bands.insert(format!("{nw:.1}")));
                assert_close("builder gae price", receipt["gae_price"].as_f64().unwrap(), expected_gae);
                assert_close(
                    "builder native band",
                    receipt["native_band_cost"].as_f64().unwrap(),
                    nw * expected_gae * expected_builder_native as f64,
                );
                let sum = builder["fq_arithmetic"].as_f64().unwrap()
                    + builder["lookup_cost"].as_f64().unwrap()
                    + receipt["native_band_cost"].as_f64().unwrap();
                assert_close("builder total", receipt["emitted_total"].as_f64().unwrap(), sum);
            }
            assert!(instance_identity
                .insert(idx, (v["seed"].as_u64().unwrap(), ell))
                .is_none());
            instance_records.push((*v).clone());
        }
        let mut unique_pp_total = 0u64;
        let mut unique_fp_total = 0u64;
        let mut pair_opportunities_total = 0u64;
        let mut giant_sum = 0.0;
        let (mut weight_sum_num, mut weight_sum_den) = (0u128, 1u128);
        let mut harvest_indices = HashSet::new();
        for v in hs {
            let idx = v["curve_index"].as_u64().unwrap();
            assert!(idx < CURVES_PER_SIZE as u64 && harvest_indices.insert(idx));
            let &(instance_seed, instance_ell) = instance_identity.get(&idx).expect("harvest instance");
            assert_eq!(v["seed"].as_u64(), Some(instance_seed));
            assert_eq!(v["ell"].as_u64(), Some(instance_ell));
            let n = v["n_factor_base"].as_u64().unwrap();
            assert_eq!(n as usize, prospective_factor_base_size(expected_q));
            let pairs = v["pairs"].as_u64().unwrap();
            assert_eq!(pairs, n * (n - 1) / 2);
            let c = &v["counters"];
            let inversion_price = 64 - expected_q.leading_zeros() as u64;
            let sqrt_price = (1.5 * (expected_q as f64).log2()).ceil() as u64;
            let expected_gae = 163.0 + 2.0 * inversion_price as f64;
            let expected_fq = (c["fq_mul_collect"].as_u64().unwrap()
                + c["fq_mul_fb"].as_u64().unwrap()) as f64
                + inversion_price as f64
                    * (c["fq_inv_collect"].as_u64().unwrap()
                        + c["fq_inv_fb"].as_u64().unwrap()) as f64
                + sqrt_price as f64
                    * (c["fq_sqrt_collect"].as_u64().unwrap()
                        + c["fq_sqrt_fb"].as_u64().unwrap()) as f64;
            let expected_native = c["hash_lookups"].as_u64().unwrap()
                + c["hash_inserts"].as_u64().unwrap()
                + c["rng_draws"].as_u64().unwrap()
                + c["uf_ops"].as_u64().unwrap()
                + c["root_calls"].as_u64().unwrap()
                + c["normalizations"].as_u64().unwrap()
                + c["lp_sge_entries"].as_u64().unwrap()
                + c["rank_entries"].as_u64().unwrap()
                + c["sort_items"].as_u64().unwrap();
            let costs = v["cost_receipts"].as_array().unwrap();
            assert_eq!(costs.len(), 6);
            let mut cost_bands = HashSet::new();
            for receipt in costs {
                let ml = receipt["mod_ell_price"].as_f64().unwrap();
                let nw = receipt["native_weight_gae"].as_f64().unwrap();
                assert!(ml == 1.0 || ml == 9.0);
                assert!(nw == 0.0 || nw == 0.1 || nw == 1.0);
                assert!(cost_bands.insert(format!("{ml:.1}/{nw:.1}")));
                assert_close("attack fq arithmetic", receipt["fq_arithmetic"].as_f64().unwrap(), expected_fq);
                assert_close("attack gae price", receipt["gae_price"].as_f64().unwrap(), expected_gae);
                assert_eq!(receipt["native_counter_count"].as_u64(), Some(expected_native));
                assert_close(
                    "attack lookup cost",
                    receipt["lookup_cost"].as_f64().unwrap(),
                    4.0 * c["hash_lookups"].as_u64().unwrap() as f64,
                );
                let expected_mod = ml
                    * (c["lp_sge_ops"].as_u64().unwrap()
                        + c["rank_sge_ops"].as_u64().unwrap()) as f64;
                assert_close("mod-ell cost", receipt["mod_ell_cost"].as_f64().unwrap(), expected_mod);
                assert_close(
                    "attack native band",
                    receipt["native_band_cost"].as_f64().unwrap(),
                    nw * expected_gae * expected_native as f64,
                );
                let sum = receipt["fq_arithmetic"].as_f64().unwrap()
                    + receipt["lookup_cost"].as_f64().unwrap()
                    + receipt["mod_ell_cost"].as_f64().unwrap()
                    + receipt["native_band_cost"].as_f64().unwrap();
                assert_close("attack total", receipt["attack_total"].as_f64().unwrap(), sum);
            }
            assert_eq!(c["lines"].as_u64(), Some(pairs));
            assert_eq!(c["verify_failed"].as_u64(), Some(0));
            let raw = [
                v["yield"]["raw_full"].as_u64().unwrap(),
                v["yield"]["raw_fp"].as_u64().unwrap(),
                v["yield"]["raw_pp"].as_u64().unwrap(),
            ];
            let full = v["yield"]["unique_full"].as_u64().unwrap();
            let fp = v["yield"]["unique_fp"].as_u64().unwrap();
            let pp = v["yield"]["unique_pp"].as_u64().unwrap();
            assert_harvest_count_partition(
                pairs,
                c["split"].as_u64().unwrap(),
                c["verified"].as_u64().unwrap(),
                c["duplicates"].as_u64().unwrap(),
                raw,
                [full, fp, pp],
            );
            for (name, numerator) in [
                ("raw_fp_per_pair", v["yield"]["raw_fp"].as_u64().unwrap()),
                ("raw_pp_per_pair", v["yield"]["raw_pp"].as_u64().unwrap()),
                ("unique_fp_per_pair", fp),
                ("unique_pp_per_pair", pp),
            ] {
                assert_close(
                    name,
                    v["yield"][name].as_f64().unwrap(),
                    numerator as f64 / pairs as f64,
                );
            }
            assert_eq!(v["large_prime_graph"]["edges"].as_u64().unwrap(), fp + pp);
            assert_eq!(
                v["large_prime_graph"]["usable_cycles_including_full"].as_i64().unwrap(),
                full as i64 + v["large_prime_graph"]["cycle_count"].as_i64().unwrap()
            );
            assert_eq!(
                v["linear_algebra"]["rank"].as_u64().unwrap()
                    + v["linear_algebra"]["nullity"].as_u64().unwrap(),
                n
            );
            let rows = v["selection_sge"]["rows_after_lp_sge"].as_u64().unwrap();
            let total_weight = v["selection_sge"]["total_row_weight_after_lp_sge"]
                .as_u64()
                .unwrap();
            assert_row_weight_certificate(
                n,
                rows,
                total_weight,
                v["selection_sge"]["max_row_weight_after_lp_sge"]
                    .as_u64()
                    .unwrap(),
                v["selection_sge"]["mean_row_weight_after_lp_sge"]
                    .as_f64()
                    .unwrap(),
            );
            unique_pp_total = unique_pp_total.checked_add(pp).unwrap();
            unique_fp_total = unique_fp_total.checked_add(fp).unwrap();
            pair_opportunities_total = pair_opportunities_total.checked_add(pairs).unwrap();
            giant_sum += v["large_prime_graph"]["giant_fraction"].as_f64().unwrap();
            let (curve_weight_num, curve_weight_den) = if rows == 0 {
                (0, 1)
            } else {
                (total_weight as u128, rows as u128)
            };
            (weight_sum_num, weight_sum_den) = add_fraction(
                weight_sum_num,
                weight_sum_den,
                curve_weight_num,
                curve_weight_den,
            );
            reference_rows.push(json!({
                "arm": "matched-formula-references",
                "instance_id": format!("q{expected_q}-curve{idx}"),
                "q": expected_q,
                "curve_index": idx,
                "seed": instance_seed,
                "ell": instance_ell,
                "rho_relation_only_lower_bound":
                    (std::f64::consts::PI * instance_ell as f64 / 2.0).sqrt() * 74.5,
                "rho_complete_cost_rankable": false,
                "hyperelliptic_q_matched_2p15":
                    fraction_f64(MATCHED_HYPER_NUM, MATCHED_HYPER_DEN),
                "hyperelliptic_exact_e_measurement": false,
                "reference_binary_executed": false,
            }));
            harvests.push((*v).clone());
        }
        let mean_weight_num = weight_sum_num;
        let mean_weight_den = weight_sum_den
            .checked_mul(CURVES_PER_SIZE as u128)
            .expect("mean row-weight denominator overflow");
        let mean_reduce = gcd_u128(mean_weight_num, mean_weight_den);
        let mean_weight_num = mean_weight_num / mean_reduce;
        let mean_weight_den = mean_weight_den / mean_reduce;
        size_receipts.push(json!({
            "q": expected_q,
            "curves": CURVES_PER_SIZE,
            "unique_pp_total": unique_pp_total,
            "unique_fp_total": unique_fp_total,
            "pair_opportunities_total": pair_opportunities_total,
            "mean_unique_pp_per_pair": unique_pp_total as f64 / pair_opportunities_total as f64,
            "mean_unique_fp_per_pair": unique_fp_total as f64 / pair_opportunities_total as f64,
            "mean_giant_fraction": giant_sum / CURVES_PER_SIZE as f64,
            "mean_row_weight_numerator": mean_weight_num.to_string(),
            "mean_row_weight_denominator": mean_weight_den.to_string(),
            "mean_row_weight_after_lp_sge": fraction_f64(mean_weight_num, mean_weight_den),
            "runtime_provenance": runtime_provenance,
            "completion_provenance": {
                "request_sha256": completion_receipt["request_sha256"],
                "job": completion_receipt["job"],
                "pod": completion_receipt["pod"],
                "resource_provenance": completion_receipt["resource_provenance"],
                "payload": completion_receipt["payload"],
                "attempts": completion_receipt["attempts"],
                "pod_restarts": completion_receipt["pod_restarts"],
                "classification": "complete_valid",
            },
        }));
    }

    let k1_failures: Vec<Value> = size_receipts
        .iter()
        .filter(|v| {
            5u128 * (v["unique_pp_total"].as_u64().unwrap() as u128)
                < v["pair_opportunities_total"].as_u64().unwrap() as u128
        })
        .cloned()
        .collect();
    let k2_failures: Vec<Value> = harvests
        .iter()
        .filter(|v| {
            v["large_prime_graph"]["usable_cycles_including_full"]
                .as_i64()
                .unwrap()
                < v["n_factor_base"].as_i64().unwrap() + 2
        })
        .map(|v| json!({
            "q": v["q"],
            "curve_index": v["curve_index"],
            "usable_cycles": v["large_prime_graph"]["usable_cycles_including_full"],
            "required": v["n_factor_base"].as_u64().unwrap() + 2,
        }))
        .collect();
    let k3_failures: Vec<Value> = harvests
        .iter()
        .filter(|v| v["linear_algebra"]["nullity"].as_u64() != Some(1))
        .map(|v| json!({
            "q": v["q"],
            "curve_index": v["curve_index"],
            "rank": v["linear_algebra"]["rank"],
            "nullity": v["linear_algebra"]["nullity"],
        }))
        .collect();
    let q15 = size_receipts
        .iter()
        .find(|v| v["q"].as_u64() == Some(32_771))
        .expect("q=32771 size receipt");
    let mean_w_num = q15["mean_row_weight_numerator"]
        .as_str()
        .unwrap()
        .parse::<u128>()
        .unwrap();
    let mean_w_den = q15["mean_row_weight_denominator"]
        .as_str()
        .unwrap()
        .parse::<u128>()
        .unwrap();
    let mean_w = fraction_f64(mean_w_num, mean_w_den);
    assert_eq!(
        K4_THRESHOLD_NUM,
        MATCHED_HYPER_NUM.checked_mul(3).unwrap(),
        "K4 threshold numerator must be exact 3/5 of frozen reference"
    );
    assert_eq!(
        K4_THRESHOLD_DEN,
        MATCHED_HYPER_DEN.checked_mul(5).unwrap(),
        "K4 threshold denominator must be exact 3/5 of frozen reference"
    );
    let projected_num_unreduced = 7_300_000u128
        .checked_mul(mean_w_den)
        .and_then(|v| 530_000u128.checked_mul(mean_w_num).and_then(|w| v.checked_add(w)))
        .expect("K4 projection overflow");
    let projected_reduce = gcd_u128(projected_num_unreduced, mean_w_den);
    let projected_num = projected_num_unreduced / projected_reduce;
    let projected_den = mean_w_den / projected_reduce;
    let projected = fraction_f64(projected_num, projected_den);
    let matched_hyper = fraction_f64(MATCHED_HYPER_NUM, MATCHED_HYPER_DEN);
    let threshold = fraction_f64(K4_THRESHOLD_NUM, K4_THRESHOLD_DEN);
    let (k4_left_cross_product, k4_right_cross_product) = exact_cross_products(
        projected_num,
        projected_den,
        K4_THRESHOLD_NUM,
        K4_THRESHOLD_DEN,
    );
    let k4_fires = k4_left_cross_product > k4_right_cross_product;
    assert_eq!(harvests.len(), ALLOWED_Q.len() * CURVES_PER_SIZE as usize);
    assert_eq!(instance_records.len(), harvests.len());
    assert_eq!(reference_rows.len(), harvests.len());
    let all_survive = k1_failures.is_empty()
        && k2_failures.is_empty()
        && k3_failures.is_empty()
        && !k4_fires;
    let verdict = if all_survive {
        "advance"
    } else {
        "new_row_or_reject"
    };
    let gates = json!({
        "schema": "lead15.k-gates/v1",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "K1": {"survives": k1_failures.is_empty(), "exact_rule": "5*sum(unique_pp) >= sum(pairs) at every size", "threshold": 0.20, "failures": k1_failures},
        "K2": {"survives": k2_failures.is_empty(), "rule": "usable_cycles >= N+2 on every curve", "failures": k2_failures},
        "K3": {"survives": k3_failures.is_empty(), "rule": "nullity == 1 on every curve", "failures": k3_failures},
        "K4": {
            "survives": !k4_fires,
            "q": 32771,
            "mean_w": mean_w,
            "mean_w_numerator": mean_w_num.to_string(),
            "mean_w_denominator": mean_w_den.to_string(),
            "projected_numerator": projected_num.to_string(),
            "projected_denominator": projected_den.to_string(),
            "projected_2p15": projected,
            "matched_hyperelliptic_numerator": MATCHED_HYPER_NUM.to_string(),
            "matched_hyperelliptic_denominator": MATCHED_HYPER_DEN.to_string(),
            "matched_hyperelliptic_2p15": matched_hyper,
            "threshold_numerator": K4_THRESHOLD_NUM.to_string(),
            "threshold_denominator": K4_THRESHOLD_DEN.to_string(),
            "threshold_0p6": threshold,
            "exact_comparison": "projected_numerator*threshold_denominator <= threshold_numerator*projected_denominator",
            "left_cross_product": k4_left_cross_product.to_str_radix(10),
            "right_cross_product": k4_right_cross_product.to_str_radix(10),
        },
        "verdict": verdict,
    });
    let analysis = json!({
        "schema": "lead15.analysis/v1",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "protocol_file": protocol_path.display().to_string(),
        "source_manifest": manifest_path.display().to_string(),
        "instrument_commit": expected_identity.commit.as_str(),
        "protocol_sha256": expected_identity.protocol_sha256.as_str(),
        "binary_sha256": expected_identity.binary_sha256.as_str(),
        "image_digest": expected_identity.image_digest.as_str(),
        "k4_reference_sha256": source.k4_reference_sha256.as_str(),
        "cell_classifications": ALLOWED_Q.iter().zip(completions.iter()).map(|(&q,c)| json!({
            "q":q,
            "classification":"complete_valid",
            "completion_receipt":c.path,
        })).collect::<Vec<_>>(),
        "valid_harvest_records": harvests.len(),
        "derived_reference_rows": reference_rows,
        "size_receipts": size_receipts,
        "gates": gates,
        "reference_status": {
            "rho": "lower_bound_relation_only_native_counters_missing",
            "hyperelliptic": "round4_incumbent_extrapolation",
        },
    });
    let export = complete_export(
        protocol_path,
        manifest_path,
        &source,
        &protocol,
        analysis_path,
        gates_path,
        &analysis,
        &completions,
        inputs,
        &instance_records,
        &harvests,
    );
    write_new_json(analysis_path, &analysis);
    write_new_json(gates_path, &analysis["gates"]);
    write_new_json(export_path, &export);
}

fn analyze_incomplete(
    protocol_path: &Path,
    manifest_path: &Path,
    analysis_path: &Path,
    gates_path: &Path,
    export_path: &Path,
    completion_paths: &[String],
    inputs: &[String],
    forced_complete_validation_failure: Option<&str>,
) {
    assert_eq!(inputs.len(), ALLOWED_Q.len(), "analysis requires exactly four size files");
    assert_eq!(
        completion_paths.len(),
        ALLOWED_Q.len(),
        "analysis requires exactly four completion receipts"
    );
    let source = frozen_source_manifest(manifest_path);
    let protocol = validate_frozen_protocol(protocol_path, &source);
    let prefixes: Vec<PrefixRead> = inputs.iter().map(|path| read_jsonl_prefix(path)).collect();
    let completions: Vec<CompletionAudit> = completion_paths
        .iter()
        .zip(ALLOWED_Q.iter())
        .zip(prefixes.iter())
        .map(|((path, &q), prefix)| audit_completion_receipt(path, q, &source, prefix))
        .collect();
    let deep_cell_problems: Vec<Vec<String>> = completions
        .iter()
        .zip(prefixes.iter())
        .zip(ALLOWED_Q.iter())
        .map(|((completion, prefix), &q)| {
            if completion.classification == "receipt_completed" && completion.problems.is_empty() {
                deep_complete_cell_problems(
                    prefix,
                    q,
                    &source.identity,
                    completion.receipt.as_ref(),
                )
            } else {
                vec![]
            }
        })
        .collect();
    let all_deep_complete = completions
        .iter()
        .zip(deep_cell_problems.iter())
        .all(|(completion, problems)| {
            completion.classification == "receipt_completed"
                && completion.problems.is_empty()
                && problems.is_empty()
        });
    let mut classifications = vec![];
    let mut reasons = vec![];
    let mut input_receipts = vec![];
    for (qpos, ((((path, &q), prefix), completion_path), completion)) in inputs
        .iter()
        .zip(ALLOWED_Q.iter())
        .zip(prefixes.iter())
        .zip(completion_paths.iter())
        .zip(completions.iter())
        .enumerate()
    {
        let mut problems = completion.problems.clone();
        problems.extend(prefix.problems.clone());
        if completion
            .receipt
            .as_ref()
            .is_some_and(|receipt| receipt["attempts"].as_u64() == Some(1))
        {
            if let Err(reason) = authenticated_partial_attempt_indices(
                prefix,
                q,
                &source.identity,
                completion.receipt.as_ref().unwrap(),
            ) {
                problems.push(reason);
            }
        }
        if !prefix.trailing_fragment.is_empty() {
            problems.push("trailing_fragment_preserved_not_parsed".to_string());
        }
        let class = if completion.classification == "receipt_completed" {
            problems.extend(deep_cell_problems[qpos].iter().cloned());
            if all_deep_complete {
                if let Some(reason) = forced_complete_validation_failure {
                    problems.push(reason.to_string());
                }
            }
            if problems.is_empty() {
                "complete_valid".to_string()
            } else {
                "scientific_invalid".to_string()
            }
        } else {
            completion.classification.clone()
        };
        if class != "complete_valid" {
            problems.push(format!("terminal_class:{class}"));
        }
        for problem in &problems {
            reasons.push(json!({
                "q": q,
                "payload_path": path,
                "completion_receipt": completion_path,
                "classification": class,
                "reason": problem,
            }));
        }
        input_receipts.push(json!({
            "q": q,
            "payload_path": path,
            "completion_receipt": completion_path,
            "classification": class,
            "parsed_prefix_records": prefix.records.len(),
            "payload_bytes": prefix.bytes.as_ref().map(Vec::len),
            "payload_sha256": prefix.bytes.as_ref().map(|v| sha256_hex(v)),
            "trailing_fragment_bytes": prefix.trailing_fragment.len(),
            "trailing_fragment_sha256": (!prefix.trailing_fragment.is_empty())
                .then(|| sha256_hex(&prefix.trailing_fragment)),
            "request_sha256": completion.receipt.as_ref().and_then(|v|v["request_sha256"].as_str()),
            "job_uid": completion.receipt.as_ref().and_then(|v|v["job"]["uid"].as_str()),
            "pod_uid": completion.receipt.as_ref().and_then(|v|v["pod"]["uid"].as_str()),
            "pod_restarts": completion.receipt.as_ref().and_then(|v|v["pod_restarts"].as_u64()),
            "problems": problems,
        }));
        classifications.push(class);
    }
    assert!(
        classifications.iter().any(|v| v != "complete_valid"),
        "all four cells are complete_valid; use the strict analyze command"
    );
    let gates = json!({
        "schema": "lead15.k-gates/v1",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "K1": {"status": "not_evaluated"},
        "K2": {"status": "not_evaluated"},
        "K3": {"status": "not_evaluated"},
        "K4": {"status": "not_evaluated"},
        "verdict": "invalid_or_incomplete",
        "reasons": reasons,
    });
    let analysis = json!({
        "schema": "lead15.analysis/v1",
        "protocol_id": "LEAD15-DT-E5-20261101",
        "protocol_file": protocol_path.display().to_string(),
        "source_manifest": manifest_path.display().to_string(),
        "instrument_commit": source.identity.commit.as_str(),
        "protocol_sha256": source.identity.protocol_sha256.as_str(),
        "binary_sha256": source.identity.binary_sha256.as_str(),
        "image_digest": source.identity.image_digest.as_str(),
        "k4_reference_sha256": source.k4_reference_sha256.as_str(),
        "complete_evidence": false,
        "cell_classifications": classifications,
        "input_receipts": input_receipts,
        "gates": gates,
        "note": "Only newline-terminated prefix records were parsed. Immutable completion receipts determine deadline/scientific/infrastructure/unknown terminal class; trailing fragments are preserved by digest and never interpreted.",
    });
    let export = incomplete_export(
        protocol_path,
        manifest_path,
        &source,
        &protocol,
        analysis_path,
        gates_path,
        &analysis,
        &completions,
        &prefixes,
        inputs,
        &classifications,
    );
    write_new_json(analysis_path, &analysis);
    write_new_json(gates_path, &analysis["gates"]);
    write_new_json(export_path, &export);
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    let text = if let Some(value) = payload.downcast_ref::<String>() {
        value.clone()
    } else if let Some(value) = payload.downcast_ref::<&str>() {
        (*value).to_string()
    } else {
        "non-string panic payload".to_string()
    };
    let mut text = text.replace('\n', " ").replace('\r', " ");
    if text.len() > 512 {
        let mut boundary = 512;
        while !text.is_char_boundary(boundary) {
            boundary -= 1;
        }
        text.truncate(boundary);
    }
    format!("strict_complete_validation_failed:{text}")
}

fn analyze(
    protocol_path: &Path,
    manifest_path: &Path,
    analysis_path: &Path,
    gates_path: &Path,
    export_path: &Path,
    completion_paths: &[String],
    inputs: &[String],
) {
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        analyze_complete(
            protocol_path,
            manifest_path,
            analysis_path,
            gates_path,
            export_path,
            completion_paths,
            inputs,
        )
    }));
    if let Err(payload) = outcome {
        let reason = panic_message(payload.as_ref());
        analyze_incomplete(
            protocol_path,
            manifest_path,
            analysis_path,
            gates_path,
            export_path,
            completion_paths,
            inputs,
            Some(&reason),
        );
    }
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  lead15_diem_quartic run-size <q> <new-output.jsonl>\n  lead15_diem_quartic analyze <protocol.json> <source-manifest.json> <new-analysis.json> <new-gates.json> <new-export.json> <receipt-q257.json> <receipt-q2053.json> <receipt-q32771.json> <receipt-q4194301.json> <q257.jsonl> <q2053.jsonl> <q32771.jsonl> <q4194301.jsonl>\n  lead15_diem_quartic analyze-incomplete <protocol.json> <source-manifest.json> <new-analysis.json> <new-gates.json> <new-export.json> <receipt-q257.json> <receipt-q2053.json> <receipt-q32771.json> <receipt-q4194301.json> <q257.jsonl> <q2053.jsonl> <q32771.jsonl> <q4194301.jsonl>"
    );
    std::process::exit(2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run-size") if args.len() == 4 => {
            let q: u64 = args[2].parse().unwrap_or_else(|_| usage());
            run_size(q, Path::new(&args[3]));
        }
        Some("analyze") if args.len() == 15 => {
            analyze(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new(&args[6]),
                &args[7..11],
                &args[11..15],
            );
        }
        Some("analyze-incomplete") if args.len() == 15 => {
            analyze_incomplete(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new(&args[6]),
                &args[7..11],
                &args[11..15],
                None,
            );
        }
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_schema_rejects_unknown_fields() {
        let mut record = json!({
            "schema":"lead15.diem-quartic/v1",
            "record":"run_end",
            "protocol_id":"LEAD15-DT-E5-20261101",
            "q":257,
            "instances":6,
            "harvests":6,
            "complete":true,
            "commit":"0".repeat(40),
            "protocol_sha256":"0".repeat(64),
            "binary_sha256":"0".repeat(64),
            "image_digest":format!("sha256:{}", "0".repeat(64)),
        });
        assert_result_record_schema(&record, 1);
        record["unpreregistered"] = Value::Bool(true);
        assert!(std::panic::catch_unwind(|| assert_result_record_schema(&record, 1)).is_err());
    }

    #[test]
    fn deep_validator_contains_malformed_cell_failure() {
        let identity = RunIdentity {
            commit: "0".repeat(40),
            protocol_sha256: "1".repeat(64),
            binary_sha256: "2".repeat(64),
            image_digest: format!("sha256:{}", "3".repeat(64)),
        };
        let receipt = json!({});
        let problems = deep_complete_cell_problems(
            &PrefixRead::default(),
            257,
            &identity,
            Some(&receipt),
        );
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("deep_cell_validation_failed:"));
        assert!(valid_etag(&json!("\"etag\"")));
        assert!(!valid_etag(&json!("x")));
        assert!(!valid_etag(&json!("é")));
        assert!(exact_bool(&json!(false), false));
        for invalid_bool in [Value::Null, json!("false"), json!(0), json!({})] {
            assert!(!exact_bool(&invalid_bool, false));
        }
        assert!(valid_rfc3339_timestamp(&json!("2026-11-02T00:00:00Z")));
        for invalid_timestamp in [
            json!("1969-12-31T23:59:59Z"),
            json!("0000-01-01T00:00:00Z"),
            json!("2026-02-29T00:00:00Z"),
            json!("2026-11-02T24:00:00Z"),
            json!("2026-11-02t00:00:00.123+05:30"),
            json!("2026-11-02T00:00:00"),
            json!("not-a-date-time"),
        ] {
            assert!(!valid_rfc3339_timestamp(&invalid_timestamp));
        }
        let valid_job_identity = json!({"name":"job","uid":"job-uid"});
        assert!(completion_receipt_job_shape(&valid_job_identity, "job"));
        let mut extra_job_identity = valid_job_identity.clone();
        extra_job_identity["extra"] = json!(true);
        assert!(!completion_receipt_job_shape(&extra_job_identity, "job"));
        let checksum = "0".repeat(64);
        let checksum_line = format!("{checksum}  ./worker-exit.json\n");
        assert!(sha256sums_entry_is_unique(
            checksum_line.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let duplicate_checksum = format!("{checksum_line}{checksum_line}");
        assert!(!sha256sums_entry_is_unique(
            duplicate_checksum.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let trailing_field = format!("{checksum}  ./worker-exit.json extra\n");
        assert!(!sha256sums_entry_is_unique(
            trailing_field.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let crlf = format!("{checksum}  ./worker-exit.json\r\n");
        assert!(!sha256sums_entry_is_unique(
            crlf.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let unicode_separator = format!("{checksum}\u{2003}./worker-exit.json\n");
        assert!(!sha256sums_entry_is_unique(
            unicode_separator.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let mut invalid_utf8 = checksum_line.into_bytes();
        invalid_utf8.extend_from_slice(b"unrelated-\xff\n");
        assert!(!sha256sums_entry_is_unique(
            &invalid_utf8,
            "worker-exit.json",
            &checksum,
        ));
        let unicode_unrelated = format!("{checksum}  ./worker-exit.json\né\n");
        assert!(!sha256sums_entry_is_unique(
            unicode_unrelated.as_bytes(),
            "worker-exit.json",
            &checksum,
        ));
        let sums_only_unicode = "é\n".as_bytes();
        assert!(!sha256sums_bytes_are_ascii(sums_only_unicode));

        #[cfg(unix)]
        {
            let unique = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after Unix epoch")
                .as_nanos();
            let test_dir = std::env::temp_dir().join(format!(
                "lead15-null-descriptor-{}-{unique}",
                std::process::id()
            ));
            fs::create_dir(&test_dir).expect("create null-descriptor test directory");
            let receipt_path = test_dir.join("completion-receipt.json");
            let artifact_path = test_dir.join("pod.json");
            let mut descriptor_problems = Vec::new();
            assert!(read_bound_lineage_file(
                &receipt_path,
                &Value::Null,
                "pod.json",
                false,
                &mut descriptor_problems,
            )
            .is_none());
            assert!(descriptor_problems.is_empty());

            fs::write(&artifact_path, b"{}\n").expect("write unbound optional artifact");
            assert!(read_bound_lineage_file(
                &receipt_path,
                &Value::Null,
                "pod.json",
                false,
                &mut descriptor_problems,
            )
            .is_none());
            assert_eq!(
                descriptor_problems,
                vec!["unexpected_unbound_lineage_artifact:pod.json"]
            );
            fs::remove_file(&artifact_path).expect("remove unbound optional artifact");

            descriptor_problems.clear();
            std::os::unix::fs::symlink(test_dir.join("missing-target"), &artifact_path)
                .expect("create dangling optional-artifact symlink");
            assert!(read_bound_lineage_file(
                &receipt_path,
                &Value::Null,
                "pod.json",
                false,
                &mut descriptor_problems,
            )
            .is_none());
            assert_eq!(
                descriptor_problems,
                vec!["unexpected_unbound_lineage_artifact:pod.json"]
            );
            fs::remove_file(&artifact_path).expect("remove dangling optional-artifact symlink");
            fs::remove_dir(&test_dir).expect("remove null-descriptor test directory");
        }

        let mut worker_exit = json!({
            "schema":"lead15.worker-exit/v1",
            "terminal_class":"completed",
            "deadline_reached":false,
            "timeout_fired":false,
            "exit_code":0,
            "child_elapsed_seconds":1,
            "payload":{
                "path":"records.jsonl",
                "sha256":"0".repeat(64),
                "bytes":3,
                "final_newline":true,
            },
            "started_at":"2026-11-01T00:00:00Z",
            "finished_at":"2026-11-01T00:00:01Z",
            "results_uri":"s3://bucket/prefix/",
            "job_name":"job",
            "pod_uid":"pod-uid",
            "node_name":"node",
        });
        assert!(worker_exit_envelope_valid(&worker_exit));
        worker_exit["exit_code"] = json!(1);
        assert!(!worker_exit_envelope_valid(&worker_exit));
        worker_exit["terminal_class"] = json!("deadline");
        worker_exit["deadline_reached"] = json!(true);
        worker_exit["timeout_fired"] = json!(true);
        worker_exit["exit_code"] = json!(124);
        worker_exit["child_elapsed_seconds"] = json!(3598);
        assert!(!worker_exit_envelope_valid(&worker_exit));
        worker_exit["child_elapsed_seconds"] = json!(3599);
        assert!(worker_exit_envelope_valid(&worker_exit));
        worker_exit["started_at"] = json!("not-a-date-time");
        assert!(!worker_exit_envelope_valid(&worker_exit));
        worker_exit["started_at"] = json!("2026-11-01T00:00:00Z");
        worker_exit["finished_at"] = json!("2026-02-29T00:00:00Z");
        assert!(!worker_exit_envelope_valid(&worker_exit));
        worker_exit["finished_at"] = json!("2026-11-01T00:00:01Z");
        worker_exit["untrusted"] = json!(true);
        assert!(!worker_exit_envelope_valid(&worker_exit));

        let mut upload = json!({
            "schema":"lead15.worker-upload/v1",
            "uploaded":true,
            "upload_exit":0,
            "terminal_class":"completed",
            "child_exit_code":0,
            "results_uri":"s3://bucket/prefix/",
            "payload_sha256":"0".repeat(64),
            "payload_bytes":3,
        });
        assert!(worker_upload_envelope_valid(&upload));
        upload["uploaded"] = json!(false);
        assert!(!worker_upload_envelope_valid(&upload));
        upload["uploaded"] = json!(true);
        upload["upload_exit"] = json!(256);
        assert!(!worker_upload_envelope_valid(&upload));

        let mut runtime = json!({
            "schema":"lead15.runtime-provenance/v1",
            "cpu_model":"fixture",
            "logical_cpus":4,
            "memory_bytes":1024,
            "kernel":"fixture",
            "job_name":"job",
            "pod_name":"pod",
            "pod_uid":"pod-uid",
            "node_name":"node",
        });
        assert!(runtime_provenance_envelope_valid(
            &runtime,
            "job",
            "pod",
            "pod-uid",
            &json!("node"),
        ));
        runtime["pod_name"] = json!("other-pod");
        assert!(!runtime_provenance_envelope_valid(
            &runtime,
            "job",
            "pod",
            "pod-uid",
            &json!("node"),
        ));

        let start = json!({"started_at":"2026-11-01T00:00:00Z"});
        let matching_exit = json!({"started_at":"2026-11-01T00:00:00Z"});
        let mixed_exit = json!({"started_at":"2026-11-01T00:00:02Z"});
        assert!(worker_start_exit_times_coherent(&start, &matching_exit));
        assert!(!worker_start_exit_times_coherent(&start, &mixed_exit));

        let container = json!({
            "name":"task",
            "image":"example.invalid/lead15@sha256:00",
            "command":["/opt/lead15/lead15-worker"],
            "workingDir":"/work",
            "env":[],
            "resources":{
                "requests":{"cpu":"4","memory":"16Gi","ephemeral-storage":"10Gi"},
                "limits":{"cpu":"4","memory":"16Gi","ephemeral-storage":"10Gi"},
            },
            "securityContext":{
                "allowPrivilegeEscalation":false,
                "readOnlyRootFilesystem":true,
                "capabilities":{"drop":["ALL"]},
            },
            "volumeMounts":[
                {"name":"work","mountPath":"/work"},
                {"name":"tmp","mountPath":"/tmp"},
            ],
        });
        assert!(safe_kubernetes_container_surface(&container, false));
        for (field, value) in [
            ("args", json!(["--replace-command"])),
            ("envFrom", json!([{"configMapRef":{"name":"mutable"}}])),
            ("volumeMounts", json!([{"name":"overlay","mountPath":"/opt/lead15"}])),
            ("volumeDevices", json!([{"name":"device","devicePath":"/dev/x"}])),
            ("lifecycle", json!({"postStart":{"exec":{"command":["/bin/false"]}}})),
            ("stdin", json!(true)),
            ("stdinOnce", json!(true)),
            ("tty", json!(true)),
        ] {
            let mut mutated = container.clone();
            mutated[field] = value;
            assert!(
                !safe_kubernetes_container_surface(&mutated, false),
                "field={field}"
            );
        }
        let mut device_resource = container.clone();
        device_resource["resources"]["limits"]["example.com/device"] = json!("1");
        assert!(!safe_kubernetes_container_surface(&device_resource, false));
        let mut resource_claim = container.clone();
        resource_claim["resources"]["claims"] = json!([{"name":"mutable"}]);
        assert!(!safe_kubernetes_container_surface(&resource_claim, false));
        let mut bad_field_ref = container.clone();
        bad_field_ref["env"] = json!([{
            "name":"POD_UID",
            "valueFrom":{"fieldRef":{"apiVersion":"v2","fieldPath":"metadata.uid"}},
        }]);
        assert!(!safe_kubernetes_container_surface(&bad_field_ref, false));
        let mut null_field_ref_version = container.clone();
        null_field_ref_version["env"] = json!([{
            "name":"POD_UID",
            "valueFrom":{"fieldRef":{"apiVersion":null,"fieldPath":"metadata.uid"}},
        }]);
        assert!(!safe_kubernetes_container_surface(
            &null_field_ref_version,
            false
        ));
        let desired_spec = json!({
            "restartPolicy":"Never",
            "serviceAccountName":"runner",
            "automountServiceAccountToken":false,
            "terminationGracePeriodSeconds":60,
            "nodeSelector":{"isolab.dev/pool":"cpu-spot"},
            "tolerations":[{
                "key":"isolab.dev/workload","value":"cpu","operator":"Equal","effect":"NoSchedule",
            }],
            "affinity":{"podAntiAffinity":{"requiredDuringSchedulingIgnoredDuringExecution":[{
                "labelSelector":{"matchLabels":{"isolab.dev/job":"true"}},
                "topologyKey":"kubernetes.io/hostname",
            }]}},
            "securityContext":{
                "runAsNonRoot":true,"runAsUser":1000,"runAsGroup":1000,"fsGroup":1000,
                "seccompProfile":{"type":"RuntimeDefault"},
            },
            "containers":[container],
            "volumes":[
                {"name":"work","emptyDir":{"sizeLimit":"8Gi"}},
                {"name":"tmp","emptyDir":{"sizeLimit":"1Gi"}},
            ],
        });
        assert!(safe_kubernetes_pod_spec_surface(&desired_spec, false, false));
        let mut actual_spec = desired_spec.clone();
        actual_spec["nodeName"] = json!("node");
        actual_spec["serviceAccount"] = json!("runner");
        actual_spec["dnsPolicy"] = json!("ClusterFirst");
        actual_spec["schedulerName"] = json!("default-scheduler");
        actual_spec["enableServiceLinks"] = json!(true);
        actual_spec["preemptionPolicy"] = json!("PreemptLowerPriority");
        actual_spec["priority"] = json!(0);
        actual_spec["containers"][0]["imagePullPolicy"] = json!("IfNotPresent");
        actual_spec["containers"][0]["terminationMessagePath"] = json!("/dev/termination-log");
        actual_spec["containers"][0]["terminationMessagePolicy"] = json!("File");
        actual_spec["containers"][0]["env"]
            .as_array_mut()
            .expect("environment")
            .extend(expected_kubernetes_irsa_env());
        actual_spec["containers"][0]["volumeMounts"]
            .as_array_mut()
            .expect("volume mounts")
            .push(json!({
                "name":"aws-iam-token",
                "mountPath":"/var/run/secrets/eks.amazonaws.com/serviceaccount",
                "readOnly":true,
                "recursiveReadOnly":"Disabled",
            }));
        actual_spec["volumes"]
            .as_array_mut()
            .expect("volumes")
            .push(expected_kubernetes_irsa_volume());
        actual_spec["tolerations"]
            .as_array_mut()
            .expect("tolerations")
            .extend(kubernetes_default_tolerations());
        assert!(safe_kubernetes_pod_spec_surface(&actual_spec, true, true));
        for invalid_node_name in [json!(false), json!(0), json!({}), json!("")] {
            let mut invalid_node_spec = actual_spec.clone();
            invalid_node_spec["nodeName"] = invalid_node_name;
            assert!(!safe_kubernetes_pod_spec_surface(
                &invalid_node_spec,
                true,
                true
            ));
        }
        assert_eq!(
            normalized_kubernetes_pod_spec(&actual_spec, true, true),
            normalized_kubernetes_pod_spec(&desired_spec, false, false),
        );
        let mut bad_irsa_role = actual_spec.clone();
        bad_irsa_role["containers"][0]["env"][0]["value"] = json!("other");
        assert!(!safe_kubernetes_pod_spec_surface(
            &bad_irsa_role,
            true,
            true
        ));
        let mut bad_irsa_mount = actual_spec.clone();
        bad_irsa_mount["containers"][0]["volumeMounts"][2]["mountPath"] =
            json!("/opt/lead15");
        assert!(!safe_kubernetes_pod_spec_surface(
            &bad_irsa_mount,
            true,
            true
        ));
        let mut bad_irsa_volume = actual_spec.clone();
        bad_irsa_volume["volumes"][2]["projected"]["sources"][0]["serviceAccountToken"]
            ["audience"] = json!("other");
        assert!(!safe_kubernetes_pod_spec_surface(
            &bad_irsa_volume,
            true,
            true
        ));
        for (field, value) in [
            ("volumes", json!([{"name":"overlay","emptyDir":{}}])),
            ("initContainers", json!([actual_spec["containers"][0].clone()])),
            ("ephemeralContainers", json!([actual_spec["containers"][0].clone()])),
        ] {
            let mut mutated = actual_spec.clone();
            mutated[field] = value;
            assert!(
                !safe_kubernetes_pod_spec_surface(&mutated, true, true),
                "field={field}"
            );
        }
        let mut sidecar = actual_spec.clone();
        sidecar["containers"] = json!([
            actual_spec["containers"][0].clone(),
            actual_spec["containers"][0].clone()
        ]);
        assert!(!safe_kubernetes_pod_spec_surface(&sidecar, true, true));
        let mut changed_workdir = actual_spec.clone();
        changed_workdir["containers"][0]["workingDir"] = json!("/tmp");
        assert_ne!(
            normalized_kubernetes_pod_spec(&changed_workdir, true, true),
            normalized_kubernetes_pod_spec(&desired_spec, false, false),
        );
        let job = json!({
            "apiVersion":"batch/v1",
            "kind":"Job",
            "metadata":{
                "name":"job","namespace":"isolab","uid":"job-uid","generation":1,
                "labels":{"isolab.dev/job":"true"},
            },
            "spec":{
                "backoffLimit":0,"activeDeadlineSeconds":7200,"ttlSecondsAfterFinished":86400,
                "selector":{"matchLabels":{"batch.kubernetes.io/controller-uid":"job-uid"}},
                "template":{
                    "metadata":{
                        "creationTimestamp":null,
                        "labels":{
                            "isolab.dev/job":"true",
                            "batch.kubernetes.io/controller-uid":"job-uid",
                            "batch.kubernetes.io/job-name":"job",
                            "controller-uid":"job-uid",
                            "job-name":"job",
                        },
                        "annotations":{"karpenter.sh/do-not-disrupt":"true"},
                    },
                    "spec":desired_spec,
                },
            },
        });
        assert!(safe_kubernetes_job_surface(&job));
        let mut wrong_job_generation = job.clone();
        wrong_job_generation["metadata"]["generation"] = json!(2);
        assert!(!safe_kubernetes_job_surface(&wrong_job_generation));
        wrong_job_generation["metadata"]["generation"] = json!("1");
        assert!(!safe_kubernetes_job_surface(&wrong_job_generation));
        let mut named_template = job.clone();
        named_template["spec"]["template"]["metadata"]["name"] = json!("mutable");
        assert!(!safe_kubernetes_job_surface(&named_template));
        let mut namespaced_template = job.clone();
        namespaced_template["spec"]["template"]["metadata"]["namespace"] =
            json!("isolab");
        assert!(!safe_kubernetes_job_surface(&namespaced_template));
        let mut timestamped_template = job.clone();
        timestamped_template["spec"]["template"]["metadata"]["creationTimestamp"] =
            json!("2026-11-01T00:00:00Z");
        assert!(!safe_kubernetes_job_surface(&timestamped_template));
        let mut wrong_selector = job.clone();
        wrong_selector["spec"]["selector"]["matchLabels"]
            ["batch.kubernetes.io/controller-uid"] = json!("other");
        assert!(!safe_kubernetes_job_surface(&wrong_selector));
        let pod = json!({
            "apiVersion":"v1","kind":"Pod",
            "metadata":{
                "name":"pod","namespace":"isolab","uid":"pod-uid","generation":1,
                "labels":job["spec"]["template"]["metadata"]["labels"].clone(),
                "annotations":{"karpenter.sh/do-not-disrupt":"true"},
                "ownerReferences":[{
                    "apiVersion":"batch/v1","kind":"Job","name":"job","uid":"job-uid",
                    "controller":true,"blockOwnerDeletion":true,
                }],
            },
            "spec":actual_spec,"status":{"observedGeneration":1},
        });
        assert!(safe_kubernetes_pod_surface(&pod, "job", "job-uid"));
        let mut wrong_generation = pod.clone();
        wrong_generation["metadata"]["generation"] = json!(2);
        assert!(!safe_kubernetes_pod_surface(
            &wrong_generation,
            "job",
            "job-uid"
        ));
        for field in ["name", "uid"] {
            let mut invalid_identity = pod.clone();
            invalid_identity["metadata"][field] = json!("");
            assert!(!safe_kubernetes_pod_surface(
                &invalid_identity,
                "job",
                "job-uid"
            ));
        }
        let mut wrong_observed_generation = pod.clone();
        wrong_observed_generation["status"]["observedGeneration"] = json!(2);
        assert!(!safe_kubernetes_pod_surface(
            &wrong_observed_generation,
            "job",
            "job-uid"
        ));
        wrong_observed_generation["status"]["observedGeneration"] = json!("1");
        assert!(!safe_kubernetes_pod_surface(
            &wrong_observed_generation,
            "job",
            "job-uid"
        ));
        wrong_observed_generation["status"]
            .as_object_mut()
            .expect("Pod status object")
            .remove("observedGeneration");
        assert!(!safe_kubernetes_pod_surface(
            &wrong_observed_generation,
            "job",
            "job-uid"
        ));
        wrong_generation["metadata"]["generation"] = json!("1");
        assert!(!safe_kubernetes_pod_surface(
            &wrong_generation,
            "job",
            "job-uid"
        ));
        let mut topology_pod = pod.clone();
        topology_pod["metadata"]["labels"]["topology.kubernetes.io/region"] = json!("us-east-2");
        topology_pod["metadata"]["labels"]["topology.kubernetes.io/zone"] = json!("us-east-2a");
        assert!(safe_kubernetes_pod_surface(&topology_pod, "job", "job-uid"));
        let mut injected_label = topology_pod.clone();
        injected_label["metadata"]["labels"]["admission.example/inject"] = json!("true");
        assert!(!safe_kubernetes_pod_surface(&injected_label, "job", "job-uid"));
        let mut missing_type_meta = pod.clone();
        missing_type_meta.as_object_mut().expect("pod object").remove("kind");
        assert!(!safe_kubernetes_pod_surface(&missing_type_meta, "job", "job-uid"));
        assert!(terminal_status_pods(&json!({"pods":[]})).is_some());
        assert!(terminal_status_pods(&json!({"pods":{}})).is_none());
        assert!(terminal_status_pod_shape(
            &json!({"name":"pod","uid":"pod-uid","node":"node"})
        ));
        assert!(terminal_status_pod_shape(
            &json!({"name":"pod","uid":"pod-uid","node":null})
        ));
        assert!(!terminal_status_pod_shape(
            &json!({"name":"pod","uid":"pod-uid","node":false})
        ));
        assert!(!terminal_status_pod_shape(
            &json!({"name":"","uid":"pod-uid","node":null})
        ));
        for invalid_node in [json!(false), json!(0), json!({}), json!("")] {
            assert!(!terminal_status_pod_shape(
                &json!({"name":"pod","uid":"pod-uid","node":invalid_node})
            ));
        }
        let valid_receipt_pod = json!({"name":"pod","uid":"pod-uid","node_name":null});
        assert!(completion_receipt_pod_shape(&valid_receipt_pod));
        for invalid_node in [json!(false), json!(0), json!({}), json!("")] {
            let mut invalid_receipt_pod = valid_receipt_pod.clone();
            invalid_receipt_pod["node_name"] = invalid_node;
            assert!(!completion_receipt_pod_shape(&invalid_receipt_pod));
        }
        let mut incomplete = json!({
            "pod":{"name":"pod","uid":"pod-uid","node_name":"node"},
            "deadline_reached":false,"exit_code":0,"started_at":null,"finished_at":null,
        });
        assert!(incomplete_worker_summary_valid(&incomplete, 1, Some(0)));
        for (field, invalid) in [
            ("deadline_reached", json!(true)),
            ("exit_code", json!(1)),
            ("started_at", json!("2026-11-02T00:00:00Z")),
            ("finished_at", json!("2026-11-02T00:00:00Z")),
        ] {
            let saved = incomplete[field].clone();
            incomplete[field] = invalid;
            assert!(!incomplete_worker_summary_valid(&incomplete, 1, Some(0)));
            incomplete[field] = saved;
        }
        incomplete["pod"]["node_name"] = Value::Null;
        incomplete["exit_code"] = Value::Null;
        assert!(incomplete_worker_summary_valid(&incomplete, 1, None));
        incomplete["exit_code"] = json!(0);
        assert!(!incomplete_worker_summary_valid(&incomplete, 1, None));
        incomplete["pod"] = Value::Null;
        incomplete["exit_code"] = Value::Null;
        assert!(incomplete_worker_summary_valid(&incomplete, 0, None));
    }

    #[test]
    fn common_cover_replay_rejects_tampered_certificate() {
        let kf = Kf::new(5);
        let (witness, ub, uc, quartic) =
            replay_common_cover_model(&kf, 0, 1, [0, 1, 0]).expect("q=5 replay fixture");
        let mut record = json!({
            "field_modulus":{"x3_a":kf.a,"constant_b":kf.b},
            "ub":ub,
            "uc":uc,
            "quartic":quartic,
            "m2_certificate":{"z":witness.z,"target_ordered":witness.target_ordered},
            "m3_certificate":{
                "a":witness.a,
                "b":witness.b,
                "r":witness.r,
                "r_q":witness.rq,
                "source_ordered":witness.source_ordered,
            },
            "common_branch_certificate":{
                "cross_ratio":witness.cross_ratio,
                "mobius":witness.mobius,
                "verified_all_four_points":true,
            },
        });
        validate_common_cover_record(&record, 5, &kf);
        record["common_branch_certificate"]["verified_all_four_points"] = Value::Bool(false);
        assert!(std::panic::catch_unwind(|| validate_common_cover_record(&record, 5, &kf)).is_err());
    }

    #[test]
    fn canonical_flow_keeps_unattempted_plans_out_of_runs() {
        let flow = canonical_flow(&[], 24);
        assert_eq!(flow["planned"].as_u64(), Some(24));
        assert_eq!(flow["attempted"].as_u64(), Some(0));
        assert_eq!(flow["completed"].as_u64(), Some(0));
    }

    #[test]
    fn k4_bigint_cross_product_handles_admissible_denominators() {
        let mut sum_num = 0u128;
        let mut sum_den = 1u128;
        for rows in [5119u128, 5107, 5099, 5087, 5077, 5059] {
            (sum_num, sum_den) = add_fraction(sum_num, sum_den, rows + 1, rows);
        }
        let mean_den = sum_den.checked_mul(6).unwrap();
        let reduce = gcd_u128(sum_num, mean_den);
        let mean_num = sum_num / reduce;
        let mean_den = mean_den / reduce;
        let projected_num = 7_300_000u128
            .checked_mul(mean_den)
            .and_then(|value| {
                530_000u128
                    .checked_mul(mean_num)
                    .and_then(|weight| value.checked_add(weight))
            })
            .unwrap();
        let projected_reduce = gcd_u128(projected_num, mean_den);
        let projected_num = projected_num / projected_reduce;
        let projected_den = mean_den / projected_reduce;
        let (left, right) = exact_cross_products(
            projected_num,
            projected_den,
            K4_THRESHOLD_NUM,
            K4_THRESHOLD_DEN,
        );
        assert!(right > BigUint::from(u128::MAX));
        assert_eq!(
            right.to_str_radix(10),
            "1685873213336503201030743465372485466819"
        );
        assert_eq!(left > right, fraction_f64(projected_num, projected_den) > fraction_f64(K4_THRESHOLD_NUM, K4_THRESHOLD_DEN));
    }

    #[test]
    fn panic_message_truncates_utf8_on_a_character_boundary() {
        let payload = format!("{}é", "x".repeat(511));
        let message = panic_message(&payload);
        assert!(message.starts_with("strict_complete_validation_failed:"));
        assert!(message.is_char_boundary(message.len()));
        assert!(!message.ends_with('�'));
    }

    #[test]
    fn harvest_count_partition_rejects_balanced_impossibilities() {
        assert!(std::panic::catch_unwind(|| {
            assert_harvest_count_partition(1, 2, 2, 0, [1, 1, 0], [1, 1, 0]);
        })
        .is_err());
        assert!(std::panic::catch_unwind(|| {
            assert_harvest_count_partition(2, 2, 2, 0, [1, 1, 0], [2, 0, 0]);
        })
        .is_err());
        assert_row_weight_certificate(5, 2, 6, 4, 3.0);
        assert_row_weight_certificate(5, 0, 0, 0, 0.0);
        assert!(std::panic::catch_unwind(|| {
            assert_row_weight_certificate(5, 2, 6, 6, 3.0);
        })
        .is_err());
        assert!(std::panic::catch_unwind(|| {
            assert_row_weight_certificate(5, 2, 9, 4, 4.5);
        })
        .is_err());
    }

    #[test]
    fn partial_attempt_authentication_rejects_wrong_identity() {
        let expected = RunIdentity {
            commit: "0".repeat(40),
            protocol_sha256: "1".repeat(64),
            binary_sha256: "2".repeat(64),
            image_digest: format!("sha256:{}", "3".repeat(64)),
        };
        let mut prefix = PrefixRead {
            records: vec![json!({
                "schema":"lead15.diem-quartic/v1",
                "record":"run_start",
                "protocol_id":"LEAD15-DT-E5-20261101",
                "q":257,
                "curves":6,
                "base_seed":BASE_SEED,
                "max_candidates":MAX_CANDIDATES,
                "retain_factor":RETAIN_FACTOR,
                "runtime_provenance":{
                    "isolation":"none",
                    "cpu_model":"fixture",
                    "available_parallelism":1,
                    "kubernetes_node":"node",
                    "kubernetes_job":"job",
                    "kubernetes_pod":"pod",
                    "kubernetes_pod_uid":"pod-uid"
                },
                "commit":"f".repeat(40),
                "protocol_sha256":expected.protocol_sha256.as_str(),
                "binary_sha256":expected.binary_sha256.as_str(),
                "image_digest":expected.image_digest.as_str(),
            })],
            ..PrefixRead::default()
        };
        let receipt = json!({
            "job":{"name":"job","uid":"job-uid"},
            "pod":{"name":"pod","uid":"pod-uid","node_name":"node"},
            "resource_provenance":{"cpu_model":"fixture"},
        });
        assert!(authenticated_partial_attempt_indices(&prefix, 257, &expected, &receipt).is_err());
        prefix.records[0]["commit"] = Value::String(expected.commit.clone());
        let mut wrong_receipt = receipt.clone();
        wrong_receipt["pod"]["uid"] = Value::String("different-pod-uid".to_string());
        assert!(
            authenticated_partial_attempt_indices(&prefix, 257, &expected, &wrong_receipt)
                .is_err()
        );
        prefix.records[0]["runtime_provenance"]["kubernetes_job_uid"] =
            Value::String("invented-job-uid".to_string());
        assert!(authenticated_partial_attempt_indices(&prefix, 257, &expected, &receipt).is_err());
        prefix.records[0]["runtime_provenance"]
            .as_object_mut()
            .unwrap()
            .remove("kubernetes_job_uid");
        assert_eq!(
            authenticated_partial_attempt_indices(&prefix, 257, &expected, &receipt).unwrap(),
            Vec::<u64>::new()
        );
    }
}
