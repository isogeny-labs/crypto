//! Native Rust experiment driver and statistical aggregator for extension-field
//! graph-anchored multi-pattern index calculus:
//!
//! - `run50 <out_dir>`: Full factorial ablation on the Run50 benchmark (`p = 1009, seed = 389`)
//!   comparing `{sparse-table, sparse-table-fast, deferred-pairs, deferred-pairs-fast}`
//!   across `{original, traced, anchored, traced-anchored}` plus the frustrated odd-cycle
//!   half-anchor arm (`deferred-pairs-fast-cycle:traced-anchored`) and matched negation-map rho.
//! - `multiprime <out_dir>`: Fresh-target scaling study across `p in {307, 383, 503, 661, 1009}`
//!   (`3` fresh unexposed seeds `[101, 211, 307]` per prime), recording exact anchor density
//!   `f(p)`, finite-`B`-space 1-LP vs 2-LP phase transition, per-phase + top-pair `bitop/v1`
//!   log-log slopes, and symmetric comparisons against both schoolbook and Karatsuba rho.
//! - `overflow-control <out_file>`: Deterministic control suite for comparing standard release
//!   vs `-C overflow-checks=on` execution (`CRITIC_ROUND2` Finding F8).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::jv_anchored_descent::{count_quadratic_universe, DescentMode};
use crypto_lib::cryptanalysis::jv_cover::{generate_spec, Ctx};
use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::{
    benchmark_graph_engines_on_stream, collect_partial_relation_stream, run_anchored_ic_dlp_on,
    run_matched_rho_reference, AnchoredPipelineOptions, AnchoredSolveBundle, CollectorArm,
    MatchedRhoReference,
};
use serde::Serialize;

const MAX_ARTIFACT_BYTES: usize = 63 * 1024;

fn write_bounded_json<T: Serialize>(path: &Path, value: &T) -> Result<usize, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let pretty = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    let payload = if pretty.len() <= MAX_ARTIFACT_BYTES {
        pretty
    } else {
        serde_json::to_string(value).map_err(|e| e.to_string())?
    };
    if payload.len() >= 64 * 1024 {
        return Err(format!(
            "artifact {} exceeds 64 KiB store policy ({} bytes)",
            path.display(),
            payload.len()
        ));
    }
    fs::write(path, &payload).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(payload.len())
}

fn phase_bitops(bundle: &AnchoredSolveBundle, phase: &str) -> u128 {
    bundle
        .accounting
        .phases
        .iter()
        .find(|p| p.phase == phase)
        .map_or(0, |p| p.bitops)
}

#[derive(Clone, Debug, Serialize)]
struct CompactArmRow {
    arm_label: String,
    p: u64,
    l: u64,
    seed: u64,
    verified: bool,
    matches_planted_truth: bool,
    recovered_scalar: Option<u64>,
    b_count_scanned: u64,
    sources_admitted: usize,
    unit_anchors: usize,
    half_anchors: usize,
    total_anchors: usize,
    live_deferred_edges: usize,
    universe_classes: u64,
    anchor_coverage_f: f64,
    predicted_multiplier_m: f64,
    residuals_tried: u64,
    accepted_rows: usize,
    accepted_p1_6: u64,
    accepted_p1_4_2: u64,
    accepted_p1_2_2_2: u64,
    accepted_p2_3: u64,
    accepted_half_scale: u64,
    trace_recorded: u64,
    trace_replayed: u64,
    trace_fallbacks: u64,
    relations_bitops: u128,
    linear_algebra_bitops: u128,
    descent_bitops: u128,
    observed_bitops: u128,
    w0_bitops: u128,
    schoolbook_w0_1_bitops: u128,
    schoolbook_w1_bitops: u128,
    karatsuba_w0_1_bitops: u128,
    karatsuba_w1_bitops: u128,
    wall_ms_indicative: f64,
    phase_wall_ms_indicative: BTreeMap<String, f64>,
    foreign_hook_calls: u64,
    bundle_file: String,
    bundle_bytes: usize,
}

fn summarize_bundle(
    bundle: &AnchoredSolveBundle,
    truth_d: u64,
    bundle_file: String,
    bundle_bytes: usize,
) -> CompactArmRow {
    let h = &bundle.descent.pattern_histogram;
    CompactArmRow {
        arm_label: bundle.arm_label.clone(),
        p: bundle.p,
        l: bundle.l,
        seed: bundle.seed,
        verified: bundle.verified,
        matches_planted_truth: bundle.recovered_scalar == Some(truth_d),
        recovered_scalar: bundle.recovered_scalar,
        b_count_scanned: bundle.collection.b_count_scanned,
        sources_admitted: bundle.collection.sources_admitted,
        unit_anchors: bundle.collection.unit_anchors,
        half_anchors: bundle.collection.half_anchors,
        total_anchors: bundle.collection.total_anchors,
        live_deferred_edges: bundle.collection.live_deferred_edges,
        universe_classes: bundle.universe_census.total_unsigned_classes,
        anchor_coverage_f: bundle.descent.anchor_coverage_f,
        predicted_multiplier_m: bundle.descent.predicted_multiplier_m,
        residuals_tried: bundle.descent.residuals_tried,
        accepted_rows: bundle.descent.accepted_rows,
        accepted_p1_6: h.accepted_p1_6,
        accepted_p1_4_2: h.accepted_p1_4_2,
        accepted_p1_2_2_2: h.accepted_p1_2_2_2,
        accepted_p2_3: h.accepted_p2_3,
        accepted_half_scale: h.accepted_half_scale,
        trace_recorded: bundle.descent.trace_recorded,
        trace_replayed: bundle.descent.trace_replayed,
        trace_fallbacks: bundle.descent.trace_fallbacks,
        relations_bitops: phase_bitops(bundle, "relations"),
        linear_algebra_bitops: phase_bitops(bundle, "linear_algebra"),
        descent_bitops: phase_bitops(bundle, "descent"),
        observed_bitops: bundle.accounting.observed_bitops,
        w0_bitops: bundle.accounting.schoolbook_band.w0_bitops,
        schoolbook_w0_1_bitops: bundle.accounting.schoolbook_band.w0_1_bitops,
        schoolbook_w1_bitops: bundle.accounting.schoolbook_band.w1_bitops,
        karatsuba_w0_1_bitops: bundle.accounting.karatsuba_band.w0_1_bitops,
        karatsuba_w1_bitops: bundle.accounting.karatsuba_band.w1_bitops,
        wall_ms_indicative: bundle.wall_ms_indicative,
        phase_wall_ms_indicative: bundle.phase_wall_ms_indicative.clone(),
        foreign_hook_calls: bundle.accounting.foreign_hook_calls,
        bundle_file,
        bundle_bytes,
    }
}

fn ols_log_slope(points: &[(f64, f64)]) -> f64 {
    if points.len() < 2 {
        return f64::NAN;
    }
    let n = points.len() as f64;
    let xs: Vec<f64> = points.iter().map(|(x, _)| x.ln()).collect();
    let ys: Vec<f64> = points.iter().map(|(_, y)| y.ln()).collect();
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut num = 0.0;
    let mut den = 0.0;
    for (x, y) in xs.iter().zip(&ys) {
        num += (x - mx) * (y - my);
        den += (x - mx) * (x - mx);
    }
    if den == 0.0 {
        f64::NAN
    } else {
        num / den
    }
}

fn run_panel_a_run50(out_dir: &Path) -> Result<(), String> {
    let p = 1009u64;
    let seed = 389u64;
    let truth_spec = generate_spec(p, seed);
    let truth_d = truth_spec.d;
    let mut public_spec = truth_spec.clone();
    public_spec.d = 0;

    let arms: Vec<(CollectorArm, DescentMode, bool, &'static str)> = vec![
        (
            CollectorArm::SparseTable,
            DescentMode::Original,
            false,
            "bundle_run50_sparse_original.json",
        ),
        (
            CollectorArm::SparseTable,
            DescentMode::Traced,
            false,
            "bundle_run50_sparse_traced.json",
        ),
        (
            CollectorArm::SparseTable,
            DescentMode::Anchored,
            false,
            "bundle_run50_sparse_anchored.json",
        ),
        (
            CollectorArm::SparseTable,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_sparse_traced_anchored.json",
        ),
        (
            CollectorArm::SparseTableFast,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_sparse_fast_traced_anchored.json",
        ),
        (
            CollectorArm::DeferredPairs,
            DescentMode::Original,
            false,
            "bundle_run50_deferred_original.json",
        ),
        (
            CollectorArm::DeferredPairs,
            DescentMode::Traced,
            false,
            "bundle_run50_deferred_traced.json",
        ),
        (
            CollectorArm::DeferredPairs,
            DescentMode::Anchored,
            false,
            "bundle_run50_deferred_anchored.json",
        ),
        (
            CollectorArm::DeferredPairs,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_deferred_traced_anchored.json",
        ),
        (
            CollectorArm::DeferredPairsFast,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_deferred_fast_traced_anchored.json",
        ),
        (
            CollectorArm::DeferredPairsFast,
            DescentMode::TracedAnchored,
            true,
            "bundle_run50_deferred_fast_cycle_traced_anchored.json",
        ),
        (
            CollectorArm::SignedMatroid2lp,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_matroid_2lp_traced_anchored.json",
        ),
        (
            CollectorArm::SignedMatroid3lp,
            DescentMode::TracedAnchored,
            false,
            "bundle_run50_matroid_3lp_traced_anchored.json",
        ),
    ];

    let bundles_dir = out_dir.join("bundles");
    let mut rows = Vec::with_capacity(arms.len());
    for (collector, descent, harvest_odd_cycles, filename) in arms {
        eprintln!(
            "[panel-a] running p={p} seed={seed} collector={} descent={} cycle={harvest_odd_cycles}",
            collector.label(),
            descent.label()
        );
        let opts = AnchoredPipelineOptions {
            collector,
            descent,
            harvest_odd_cycles,
            ..Default::default()
        };
        let bundle = run_anchored_ic_dlp_on(&public_spec, seed, opts)?;
        if !bundle.verified || bundle.recovered_scalar != Some(truth_d) {
            return Err(format!(
                "Run50 arm {} failed truth check: got {:?}, want {truth_d}",
                bundle.arm_label, bundle.recovered_scalar
            ));
        }
        let bpath = bundles_dir.join(filename);
        let bytes = write_bounded_json(&bpath, &bundle)?;
        rows.push(summarize_bundle(
            &bundle,
            truth_d,
            format!("bundles/{filename}"),
            bytes,
        ));
    }

    eprintln!("[panel-a] running matched rho reference (analytical + 1 cold b16 walk) on p={p} seed={seed}");
    let rho_ref = run_matched_rho_reference(&truth_spec, Some(truth_d), seed, 1, true);

    let find = |label: &str| -> &CompactArmRow {
        rows.iter()
            .find(|r| r.arm_label == label)
            .expect("arm present")
    };
    let def_orig = find("deferred-pairs:original");
    let def_tr = find("deferred-pairs:traced");
    let def_anc = find("deferred-pairs:anchored");
    let def_ta = find("deferred-pairs:traced-anchored");
    let def_fast_ta = find("deferred-pairs-fast:traced-anchored");
    let def_cyc_ta = find("deferred-pairs-fast-cycle:traced-anchored");
    let mat2_ta = find("signed-matroid-2lp:traced-anchored");
    let mat3_ta = find("signed-matroid-3lp:traced-anchored");
    let sp_orig = find("sparse-table:original");
    let sp_fast_ta = find("sparse-table-fast:traced-anchored");

    eprintln!(
        "[panel-a] running 7-engine partial-relation graph head-to-head on p={p} seed={seed} b_count={}",
        mat2_ta.b_count_scanned
    );
    let (stream_base_len, stream_sources) =
        collect_partial_relation_stream(&public_spec, seed, mat2_ta.b_count_scanned, 3)?;
    let graph_cmp =
        benchmark_graph_engines_on_stream(p, truth_spec.l, seed, stream_base_len, &stream_sources)?;
    write_bounded_json(&out_dir.join("graph_engine_head_to_head.json"), &graph_cmp)?;

    let summary = serde_json::json!({
        "schema": "cover.anchored-multi-pattern.panel-a/v1",
        "instance": {
            "p": p,
            "l": truth_spec.l,
            "seed": seed,
            "universe_unsigned_classes": def_orig.universe_classes,
            "verified_scalar": truth_d,
        },
        "factorial_highlights": {
            "deferred_descent_speedup_traced_vs_orig": def_orig.descent_bitops as f64 / def_tr.descent_bitops as f64,
            "deferred_descent_speedup_anchored_vs_orig": def_orig.descent_bitops as f64 / def_anc.descent_bitops as f64,
            "deferred_descent_speedup_traced_anchored_vs_orig": def_orig.descent_bitops as f64 / def_ta.descent_bitops as f64,
            "deferred_multiplicative_expectation": (def_orig.descent_bitops as f64 / def_tr.descent_bitops as f64)
                * (def_orig.descent_bitops as f64 / def_anc.descent_bitops as f64),
            "deferred_relations_speedup_fast_vs_orig": def_orig.relations_bitops as f64 / def_fast_ta.relations_bitops as f64,
            "matroid_2lp_relations_speedup_vs_orig": def_orig.relations_bitops as f64 / mat2_ta.relations_bitops as f64,
            "matroid_2lp_relations_speedup_vs_fast_bfs": def_fast_ta.relations_bitops as f64 / mat2_ta.relations_bitops as f64,
            "deferred_total_w0_speedup_fast_ta_vs_orig": def_orig.w0_bitops as f64 / def_fast_ta.w0_bitops as f64,
            "matroid_2lp_total_w0_speedup_vs_orig": def_orig.w0_bitops as f64 / mat2_ta.w0_bitops as f64,
            "matroid_2lp_total_w1_karatsuba_speedup_vs_orig": def_orig.karatsuba_w1_bitops as f64 / mat2_ta.karatsuba_w1_bitops as f64,
            "matroid_3lp_total_w0_speedup_vs_orig": def_orig.w0_bitops as f64 / mat3_ta.w0_bitops as f64,
            "sparse_descent_speedup_traced_anchored_vs_orig": sp_orig.descent_bitops as f64 / find("sparse-table:traced-anchored").descent_bitops as f64,
            "sparse_total_w0_speedup_fast_ta_vs_orig": sp_orig.w0_bitops as f64 / sp_fast_ta.w0_bitops as f64,
            "cycle_half_anchors_harvested": def_cyc_ta.half_anchors,
            "matroid_2lp_unit_anchors": mat2_ta.unit_anchors,
            "matroid_2lp_half_anchors": mat2_ta.half_anchors,
            "matroid_2lp_total_anchors": mat2_ta.total_anchors,
        },
        "arms": rows,
        "graph_engine_head_to_head": graph_cmp,
        "matched_rho_reference": rho_ref,
    });
    write_bounded_json(&out_dir.join("panel_a_run50_summary.json"), &summary)?;
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
struct PrimeAggregate {
    p: u64,
    mean_l: f64,
    universe_classes_mean: f64,
    sparse_fast_ta_solved: usize,
    deferred_orig_solved: usize,
    deferred_fast_ta_solved: usize,
    matroid_2lp_ta_solved: usize,
    deferred_anchor_f_mean: f64,
    deferred_p_times_f_mean: f64,
    matroid_anchor_f_mean: f64,
    matroid_p_times_f_mean: f64,
    deferred_predicted_m_mean: f64,
    matroid_predicted_m_mean: f64,
    deferred_orig_descent_bitops_mean: f64,
    deferred_fast_ta_descent_bitops_mean: f64,
    matroid_2lp_ta_descent_bitops_mean: f64,
    deferred_descent_speedup_mean: f64,
    matroid_descent_speedup_mean: f64,
    deferred_orig_relations_bitops_mean: f64,
    deferred_fast_ta_relations_bitops_mean: f64,
    matroid_2lp_ta_relations_bitops_mean: f64,
    matroid_relations_speedup_vs_orig_mean: f64,
    deferred_orig_total_w0_mean: f64,
    deferred_fast_ta_total_w0_mean: f64,
    deferred_fast_ta_total_w1_schoolbook_mean: f64,
    deferred_fast_ta_total_w1_karatsuba_mean: f64,
    matroid_2lp_ta_total_w0_mean: f64,
    matroid_2lp_ta_total_w1_schoolbook_mean: f64,
    matroid_2lp_ta_total_w1_karatsuba_mean: f64,
    deferred_total_w0_speedup_mean: f64,
    matroid_total_w0_speedup_vs_orig_mean: f64,
    rho_analytical_b16_schoolbook_mean: f64,
    rho_analytical_b16_karatsuba_mean: f64,
    rho_empirical_b16_median_schoolbook_mean: f64,
    rho_empirical_b16_median_karatsuba_mean: f64,
    ic_over_rho_b16_schoolbook_w0: f64,
    ic_over_rho_b16_karatsuba_w0: f64,
    ic_over_rho_b16_karatsuba_w1: f64,
    matroid_ic_over_rho_b16_schoolbook_w0: f64,
    matroid_ic_over_rho_b16_karatsuba_w0: f64,
    matroid_ic_over_rho_b16_karatsuba_w1: f64,
}

fn run_panel_b_multiprime(out_dir: &Path) -> Result<(), String> {
    let primes = [307u64, 383, 503, 661, 1009];
    let seeds = [101u64, 211, 307];
    let bundles_dir = out_dir.join("bundles");

    let mut all_rows = Vec::<CompactArmRow>::new();
    let mut rho_refs = Vec::<(u64, u64, MatchedRhoReference)>::new();
    let mut per_prime = Vec::<PrimeAggregate>::new();

    for &p in &primes {
        let mut ls = Vec::new();
        let mut univs = Vec::new();
        let mut sp_solved = 0usize;
        let mut do_solved = 0usize;
        let mut df_solved = 0usize;
        let mut mt_solved = 0usize;
        let mut fs = Vec::new();
        let mut mfs = Vec::new();
        let mut ms = Vec::new();
        let mut mms = Vec::new();
        let mut do_desc = Vec::new();
        let mut df_desc = Vec::new();
        let mut mt_desc = Vec::new();
        let mut do_rel = Vec::new();
        let mut df_rel = Vec::new();
        let mut mt_rel = Vec::new();
        let mut do_tot = Vec::new();
        let mut df_tot_w0 = Vec::new();
        let mut df_tot_w1_sb = Vec::new();
        let mut df_tot_w1_ka = Vec::new();
        let mut mt_tot_w0 = Vec::new();
        let mut mt_tot_w1_sb = Vec::new();
        let mut mt_tot_w1_ka = Vec::new();
        let mut rho_ana_sb = Vec::new();
        let mut rho_ana_ka = Vec::new();
        let mut rho_emp_sb = Vec::new();
        let mut rho_emp_ka = Vec::new();

        for &seed in &seeds {
            eprintln!("[panel-b] generating instance p={p} seed={seed}");
            let truth_spec = generate_spec(p, seed);
            let truth_d = truth_spec.d;
            let mut public_spec = truth_spec.clone();
            public_spec.d = 0;
            ls.push(truth_spec.l as f64);

            // 1. 1-LP best arm (`sparse-table-fast:traced-anchored`).
            let b_sp = run_anchored_ic_dlp_on(
                &public_spec,
                seed,
                AnchoredPipelineOptions {
                    collector: CollectorArm::SparseTableFast,
                    descent: DescentMode::TracedAnchored,
                    ..Default::default()
                },
            )?;
            let fn_sp = format!("bundle_mp_p{p}_s{seed}_sparse_fast_ta.json");
            let sz_sp = write_bounded_json(&bundles_dir.join(&fn_sp), &b_sp)?;
            if b_sp.verified {
                if b_sp.recovered_scalar != Some(truth_d) {
                    return Err(format!("wrong scalar on sparse p={p} seed={seed}"));
                }
                sp_solved += 1;
            }
            all_rows.push(summarize_bundle(
                &b_sp,
                truth_d,
                format!("bundles/{fn_sp}"),
                sz_sp,
            ));

            // 2. 2-LP baseline (`deferred-pairs:original`).
            let b_do = run_anchored_ic_dlp_on(
                &public_spec,
                seed,
                AnchoredPipelineOptions {
                    collector: CollectorArm::DeferredPairs,
                    descent: DescentMode::Original,
                    ..Default::default()
                },
            )?;
            let fn_do = format!("bundle_mp_p{p}_s{seed}_deferred_orig.json");
            let sz_do = write_bounded_json(&bundles_dir.join(&fn_do), &b_do)?;
            if b_do.verified {
                if b_do.recovered_scalar != Some(truth_d) {
                    return Err(format!("wrong scalar on deferred_orig p={p} seed={seed}"));
                }
                do_solved += 1;
            }
            let row_do = summarize_bundle(&b_do, truth_d, format!("bundles/{fn_do}"), sz_do);

            // 3. 2-LP legacy-graph candidate (`deferred-pairs-fast:traced-anchored`).
            let b_df = run_anchored_ic_dlp_on(
                &public_spec,
                seed,
                AnchoredPipelineOptions {
                    collector: CollectorArm::DeferredPairsFast,
                    descent: DescentMode::TracedAnchored,
                    ..Default::default()
                },
            )?;
            let fn_df = format!("bundle_mp_p{p}_s{seed}_deferred_fast_ta.json");
            let sz_df = write_bounded_json(&bundles_dir.join(&fn_df), &b_df)?;
            if b_df.verified {
                if b_df.recovered_scalar != Some(truth_d) {
                    return Err(format!(
                        "wrong scalar on deferred_fast_ta p={p} seed={seed}"
                    ));
                }
                df_solved += 1;
            }
            let row_df = summarize_bundle(&b_df, truth_d, format!("bundles/{fn_df}"), sz_df);

            // 4. Flagship Signed-Matroid 2-LP + Hit-Bucket + Frobenius Resolvent (`signed-matroid-2lp:traced-anchored`).
            let b_mt = run_anchored_ic_dlp_on(
                &public_spec,
                seed,
                AnchoredPipelineOptions {
                    collector: CollectorArm::SignedMatroid2lp,
                    descent: DescentMode::TracedAnchored,
                    ..Default::default()
                },
            )?;
            let fn_mt = format!("bundle_mp_p{p}_s{seed}_matroid_2lp_ta.json");
            let sz_mt = write_bounded_json(&bundles_dir.join(&fn_mt), &b_mt)?;
            if b_mt.verified {
                if b_mt.recovered_scalar != Some(truth_d) {
                    return Err(format!("wrong scalar on matroid_2lp_ta p={p} seed={seed}"));
                }
                mt_solved += 1;
            }
            let row_mt = summarize_bundle(&b_mt, truth_d, format!("bundles/{fn_mt}"), sz_mt);

            univs.push(row_mt.universe_classes as f64);
            fs.push(row_df.anchor_coverage_f);
            mfs.push(row_mt.anchor_coverage_f);
            ms.push(row_df.predicted_multiplier_m);
            mms.push(row_mt.predicted_multiplier_m);
            do_desc.push(row_do.descent_bitops as f64);
            df_desc.push(row_df.descent_bitops as f64);
            mt_desc.push(row_mt.descent_bitops as f64);
            do_rel.push(row_do.relations_bitops as f64);
            df_rel.push(row_df.relations_bitops as f64);
            mt_rel.push(row_mt.relations_bitops as f64);
            do_tot.push(row_do.w0_bitops as f64);
            df_tot_w0.push(row_df.w0_bitops as f64);
            df_tot_w1_sb.push(row_df.schoolbook_w1_bitops as f64);
            df_tot_w1_ka.push(row_df.karatsuba_w1_bitops as f64);
            mt_tot_w0.push(row_mt.w0_bitops as f64);
            mt_tot_w1_sb.push(row_mt.schoolbook_w1_bitops as f64);
            mt_tot_w1_ka.push(row_mt.karatsuba_w1_bitops as f64);

            all_rows.push(row_do);
            all_rows.push(row_df);
            all_rows.push(row_mt);

            // 5. Matched rho reference (5 empirical walks at p <= 661, 1 empirical walk at p = 1009).
            let walks_n = if p <= 661 { 5 } else { 1 };
            let rho = run_matched_rho_reference(&truth_spec, Some(truth_d), seed, walks_n, true);
            if rho.empirical_walks.iter().any(|w| !w.verified) {
                return Err(format!(
                    "empirical rho walk failed verification at p={p} seed={seed}"
                ));
            }
            rho_ana_sb.push(rho.analytical_b16_schoolbook_bitops as f64);
            rho_ana_ka.push(rho.analytical_b16_karatsuba_bitops as f64);
            rho_emp_sb.push(rho.empirical_b16_median_schoolbook_bitops as f64);
            rho_emp_ka.push(rho.empirical_b16_median_karatsuba_bitops as f64);
            rho_refs.push((p, seed, rho));
        }

        let mean = |v: &[f64]| v.iter().sum::<f64>() / (v.len() as f64);
        let m_f = mean(&fs);
        let m_mf = mean(&mfs);
        let m_do_d = mean(&do_desc);
        let m_df_d = mean(&df_desc);
        let m_mt_d = mean(&mt_desc);
        let m_do_r = mean(&do_rel);
        let m_mt_r = mean(&mt_rel);
        let m_do_tot = mean(&do_tot);
        let m_df_w0 = mean(&df_tot_w0);
        let m_df_w1_ka = mean(&df_tot_w1_ka);
        let m_mt_w0 = mean(&mt_tot_w0);
        let m_mt_w1_ka = mean(&mt_tot_w1_ka);
        let m_rho_sb = mean(&rho_ana_sb);
        let m_rho_ka = mean(&rho_ana_ka);

        per_prime.push(PrimeAggregate {
            p,
            mean_l: mean(&ls),
            universe_classes_mean: mean(&univs),
            sparse_fast_ta_solved: sp_solved,
            deferred_orig_solved: do_solved,
            deferred_fast_ta_solved: df_solved,
            matroid_2lp_ta_solved: mt_solved,
            deferred_anchor_f_mean: m_f,
            deferred_p_times_f_mean: (p as f64) * m_f,
            matroid_anchor_f_mean: m_mf,
            matroid_p_times_f_mean: (p as f64) * m_mf,
            deferred_predicted_m_mean: mean(&ms),
            matroid_predicted_m_mean: mean(&mms),
            deferred_orig_descent_bitops_mean: m_do_d,
            deferred_fast_ta_descent_bitops_mean: m_df_d,
            matroid_2lp_ta_descent_bitops_mean: m_mt_d,
            deferred_descent_speedup_mean: m_do_d / m_df_d,
            matroid_descent_speedup_mean: m_do_d / m_mt_d,
            deferred_orig_relations_bitops_mean: m_do_r,
            deferred_fast_ta_relations_bitops_mean: mean(&df_rel),
            matroid_2lp_ta_relations_bitops_mean: m_mt_r,
            matroid_relations_speedup_vs_orig_mean: m_do_r / m_mt_r,
            deferred_orig_total_w0_mean: m_do_tot,
            deferred_fast_ta_total_w0_mean: m_df_w0,
            deferred_fast_ta_total_w1_schoolbook_mean: mean(&df_tot_w1_sb),
            deferred_fast_ta_total_w1_karatsuba_mean: m_df_w1_ka,
            matroid_2lp_ta_total_w0_mean: m_mt_w0,
            matroid_2lp_ta_total_w1_schoolbook_mean: mean(&mt_tot_w1_sb),
            matroid_2lp_ta_total_w1_karatsuba_mean: m_mt_w1_ka,
            deferred_total_w0_speedup_mean: m_do_tot / m_df_w0,
            matroid_total_w0_speedup_vs_orig_mean: m_do_tot / m_mt_w0,
            rho_analytical_b16_schoolbook_mean: m_rho_sb,
            rho_analytical_b16_karatsuba_mean: m_rho_ka,
            rho_empirical_b16_median_schoolbook_mean: mean(&rho_emp_sb),
            rho_empirical_b16_median_karatsuba_mean: mean(&rho_emp_ka),
            ic_over_rho_b16_schoolbook_w0: m_df_w0 / m_rho_sb,
            ic_over_rho_b16_karatsuba_w0: m_df_w0 / m_rho_ka,
            ic_over_rho_b16_karatsuba_w1: m_df_w1_ka / m_rho_ka,
            matroid_ic_over_rho_b16_schoolbook_w0: m_mt_w0 / m_rho_sb,
            matroid_ic_over_rho_b16_karatsuba_w0: m_mt_w0 / m_rho_ka,
            matroid_ic_over_rho_b16_karatsuba_w1: m_mt_w1_ka / m_rho_ka,
        });
    }

    // Per-phase & top-pair slopes (`log(cost) / log(p)`).
    let pts_all = |f: fn(&PrimeAggregate) -> f64| -> Vec<(f64, f64)> {
        per_prime.iter().map(|r| (r.p as f64, f(r))).collect()
    };
    let pts_top2 = |f: fn(&PrimeAggregate) -> f64| -> Vec<(f64, f64)> {
        per_prime[per_prime.len() - 2..]
            .iter()
            .map(|r| (r.p as f64, f(r)))
            .collect()
    };

    let slopes = serde_json::json!({
        "anchor_f_vs_p_slope_all5": ols_log_slope(&pts_all(|r| r.deferred_anchor_f_mean)),
        "anchor_f_vs_p_slope_top2": ols_log_slope(&pts_top2(|r| r.deferred_anchor_f_mean)),
        "matroid_anchor_f_vs_p_slope_all5": ols_log_slope(&pts_all(|r| r.matroid_anchor_f_mean)),
        "matroid_anchor_f_vs_p_slope_top2": ols_log_slope(&pts_top2(|r| r.matroid_anchor_f_mean)),
        "deferred_orig_total_w0_slope_all5": ols_log_slope(&pts_all(|r| r.deferred_orig_total_w0_mean)),
        "deferred_orig_total_w0_slope_top2": ols_log_slope(&pts_top2(|r| r.deferred_orig_total_w0_mean)),
        "deferred_fast_ta_total_w0_slope_all5": ols_log_slope(&pts_all(|r| r.deferred_fast_ta_total_w0_mean)),
        "deferred_fast_ta_total_w0_slope_top2": ols_log_slope(&pts_top2(|r| r.deferred_fast_ta_total_w0_mean)),
        "matroid_2lp_ta_total_w0_slope_all5": ols_log_slope(&pts_all(|r| r.matroid_2lp_ta_total_w0_mean)),
        "matroid_2lp_ta_total_w0_slope_top2": ols_log_slope(&pts_top2(|r| r.matroid_2lp_ta_total_w0_mean)),
        "matroid_2lp_ta_relations_slope_all5": ols_log_slope(&pts_all(|r| r.matroid_2lp_ta_relations_bitops_mean)),
        "matroid_2lp_ta_relations_slope_top2": ols_log_slope(&pts_top2(|r| r.matroid_2lp_ta_relations_bitops_mean)),
        "matroid_2lp_ta_descent_slope_all5": ols_log_slope(&pts_all(|r| r.matroid_2lp_ta_descent_bitops_mean)),
        "matroid_2lp_ta_descent_slope_top2": ols_log_slope(&pts_top2(|r| r.matroid_2lp_ta_descent_bitops_mean)),
        "rho_b16_karatsuba_slope_all5": ols_log_slope(&pts_all(|r| r.rho_analytical_b16_karatsuba_mean)),
        "rho_b16_karatsuba_slope_top2": ols_log_slope(&pts_top2(|r| r.rho_analytical_b16_karatsuba_mean)),
    });

    write_bounded_json(
        &out_dir.join("panel_b_multiprime_summary.json"),
        &serde_json::json!({
            "schema": "cover.anchored-multi-pattern.panel-b/v1",
            "primes": primes,
            "seeds": seeds,
            "per_prime_aggregates": per_prime,
            "scaling_slopes_log_cost_per_log_p": slopes,
        }),
    )?;
    write_bounded_json(&out_dir.join("panel_b_multiprime_runs.json"), &all_rows)?;
    write_bounded_json(&out_dir.join("panel_b_rho_references.json"), &rho_refs)?;
    Ok(())
}

fn run_overflow_control(out_file: &Path) -> Result<(), String> {
    let mut records = Vec::new();
    for &(p, seed, collector) in &[
        (307u64, 101u64, CollectorArm::SignedMatroid2lp),
        (383u64, 211u64, CollectorArm::DeferredPairsFast),
        (503u64, 307u64, CollectorArm::SparseTableFast),
    ] {
        let truth_spec = generate_spec(p, seed);
        let truth_d = truth_spec.d;
        let mut public_spec = truth_spec;
        public_spec.d = 0;
        let bundle = run_anchored_ic_dlp_on(
            &public_spec,
            seed,
            AnchoredPipelineOptions {
                collector,
                descent: DescentMode::TracedAnchored,
                harvest_odd_cycles: collector.is_two_lp(),
                ..Default::default()
            },
        )?;
        if bundle.recovered_scalar != Some(truth_d) {
            return Err(format!(
                "overflow control solve mismatch at p={p} seed={seed}"
            ));
        }
        records.push(serde_json::json!({
            "p": p,
            "seed": seed,
            "arm": bundle.arm_label,
            "recovered_scalar": bundle.recovered_scalar,
            "sources_sha256": bundle.collection.sources_sha256,
            "projections_sha256": bundle.collection.projections_sha256,
            "residuals_tried": bundle.descent.residuals_tried,
            "observed_bitops": bundle.accounting.observed_bitops,
            "w0_bitops": bundle.accounting.schoolbook_band.w0_bitops,
            "foreign_hook_calls": bundle.accounting.foreign_hook_calls,
        }));
    }
    let overflow_checks_active = std::panic::catch_unwind(|| {
        let a = std::hint::black_box(u64::MAX);
        let b = std::hint::black_box(1u64);
        std::hint::black_box(a + b);
    })
    .is_err();
    write_bounded_json(
        out_file,
        &serde_json::json!({
            "schema": "cover.anchored-multi-pattern.overflow-control/v1",
            "overflow_checks_compiled": overflow_checks_active,
            "records": records,
        }),
    )?;
    Ok(())
}

fn render_study_figures(
    panel_a_path: &Path,
    panel_b_path: &Path,
    out_dir: &Path,
) -> Result<(), String> {
    fs::create_dir_all(out_dir).map_err(|e| format!("mkdir {}: {e}", out_dir.display()))?;
    let pa: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(panel_a_path).map_err(|e| format!("read panel A: {e}"))?,
    )
    .map_err(|e| format!("parse panel A: {e}"))?;
    let pb: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(panel_b_path).map_err(|e| format!("read panel B: {e}"))?,
    )
    .map_err(|e| format!("parse panel B: {e}"))?;

    let u128_of = |v: &serde_json::Value| -> f64 {
        v.as_u64()
            .map(|x| x as f64)
            .or_else(|| v.as_f64())
            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            .unwrap_or(0.0)
    };

    // Figure 1: Run50 factorial ablation (stacked phase bitops + M(f) binomial law).
    let arms = pa["arms"].as_array().ok_or("missing arms")?;
    let mut max_tb = 1.0_f64;
    for a in arms {
        let tot = u128_of(&a["w0_bitops"]) / 1e12;
        if tot > max_tb {
            max_tb = tot;
        }
    }
    let ceil_tb = (max_tb * 1.18).ceil().max(1.0);

    let mut svg1 = String::with_capacity(16000);
    svg1.push_str(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 920 445" width="920" height="445" font-family="DejaVu Sans, sans-serif">
<rect width="920" height="445" fill="#ffffff"/>
<text x="18" y="24" font-size="13" font-weight="bold" fill="#111827">(a) Run50 (p = 1009, seed = 389) Cold End-to-End Bit-Operations by Phase (w = 0, x10^12)</text>
<text x="580" y="24" font-size="13" font-weight="bold" fill="#111827">(b) Multi-Pattern Yield Law M(f) vs Anchor Coverage f</text>
"##,
    );

    let x0 = 215.0_f64;
    let pw = 290.0_f64;
    let y0 = 44.0_f64;
    let row_h = 26.2_f64;

    for tick in 0..=5 {
        let val = ceil_tb * (tick as f64) / 5.0;
        let gx = x0 + pw * (tick as f64) / 5.0;
        svg1.push_str(&format!(
            r##"<line x1="{gx:.1}" y1="{y0:.1}" x2="{gx:.1}" y2="{:.1}" stroke="#e5e7eb" stroke-width="1"/>
<text x="{gx:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="middle">{val:.1}</text>
"##,
            y0 + (arms.len() as f64) * row_h,
            y0 + (arms.len() as f64) * row_h + 15.0
        ));
    }

    for (idx, a) in arms.iter().enumerate() {
        let label = a["arm_label"].as_str().unwrap_or("?");
        let short = label
            .replace("signed-matroid-2lp", "Matroid-2LP")
            .replace("signed-matroid-3lp", "Matroid-3LP")
            .replace("deferred-pairs", "2LP-BFS")
            .replace("sparse-table", "1LP")
            .replace("traced-anchored", "trace+anc");
        let is_flagship = label.starts_with("signed-matroid-2lp");
        let rel = u128_of(&a["relations_bitops"]) / 1e12;
        let la = u128_of(&a["linear_algebra_bitops"]) / 1e12;
        let desc = u128_of(&a["descent_bitops"]) / 1e12;
        let tot = u128_of(&a["w0_bitops"]) / 1e12;
        let other = (tot - rel - la - desc).max(0.0);

        let ry = y0 + (idx as f64) * row_h + 4.0;
        let bh = 17.2_f64;
        let w_rel = pw * (rel / ceil_tb);
        let w_la = pw * (la / ceil_tb);
        let w_desc = pw * (desc / ceil_tb);
        let w_oth = pw * (other / ceil_tb);
        let weight_attr = if is_flagship {
            r#" font-weight="bold""#
        } else {
            ""
        };

        svg1.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" font-size="9.2" fill="#1f2937" text-anchor="end"{weight_attr}>{short}</text>
<rect x="{x0:.1}" y="{ry:.1}" width="{w_rel:.1}" height="{bh:.1}" fill="#2563eb"/>
<rect x="{:.1}" y="{ry:.1}" width="{w_la:.1}" height="{bh:.1}" fill="#10b981"/>
<rect x="{:.1}" y="{ry:.1}" width="{w_desc:.1}" height="{bh:.1}" fill="#f59e0b"/>
<rect x="{:.1}" y="{ry:.1}" width="{w_oth:.1}" height="{bh:.1}" fill="#6b7280"/>
<text x="{:.1}" y="{:.1}" font-size="9" fill="#111827"{weight_attr}>{tot:.2}T</text>
"##,
            x0 - 7.0,
            ry + 12.0,
            x0 + w_rel,
            x0 + w_rel + w_la,
            x0 + w_rel + w_la + w_desc,
            x0 + w_rel + w_la + w_desc + w_oth + 5.0,
            ry + 12.0
        ));
    }

    let ly = 425.0_f64;
    for (lx, col, txt) in [
        (25.0, "#2563eb", "Phase 1: Sieve + Graph Relations"),
        (215.0, "#10b981", "Phase 2: Sparse LA"),
        (335.0, "#f59e0b", "Phase 3: F4 + Descent"),
    ] {
        svg1.push_str(&format!(
            r##"<rect x="{lx:.1}" y="{:.1}" width="10" height="10" fill="{col}"/>
<text x="{:.1}" y="{ly:.1}" font-size="9.5" fill="#374151">{txt}</text>
"##,
            ly - 9.0,
            lx + 14.0
        ));
    }

    // Right subpanel: M(f) = 1 + 15f + 45f^2 + 15f^3 curve for f in [0, 0.20] + Run50 observed points.
    let rx0 = 615.0_f64;
    let rpw = 275.0_f64;
    let ry0 = 52.0_f64;
    let rph = 310.0_f64;
    let f_max = 0.20_f64;
    let m_max = 6.5_f64;

    svg1.push_str(&format!(
        r##"<rect x="{rx0:.1}" y="{ry0:.1}" width="{rpw:.1}" height="{rph:.1}" fill="none" stroke="#9ca3af" stroke-width="1"/>
"##
    ));

    for k in 0..=4 {
        let fv = f_max * (k as f64) / 4.0;
        let px = rx0 + rpw * (fv / f_max);
        let mv = 1.0 + (m_max - 1.0) * (k as f64) / 4.0;
        let py = ry0 + rph * (1.0 - (mv - 1.0) / (m_max - 1.0));
        svg1.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{ry0:.1}" x2="{px:.1}" y2="{:.1}" stroke="#f3f4f6" stroke-width="1"/>
<text x="{px:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="middle">{:.0}%</text>
<line x1="{rx0:.1}" y1="{py:.1}" x2="{:.1}" y2="{py:.1}" stroke="#f3f4f6" stroke-width="1"/>
<text x="{:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="end">{mv:.1}x</text>
"##,
            ry0 + rph,
            ry0 + rph + 15.0,
            fv * 100.0,
            rx0 + rpw,
            rx0 - 6.0,
            py + 3.5
        ));
    }

    let mut path_d = String::new();
    for step in 0..=80 {
        let fv = f_max * (step as f64) / 80.0;
        let mv = 1.0 + 15.0 * fv + 45.0 * fv * fv + 15.0 * fv * fv * fv;
        let px = rx0 + rpw * (fv / f_max);
        let py = ry0 + rph * (1.0 - ((mv.min(m_max) - 1.0) / (m_max - 1.0)));
        if step == 0 {
            path_d.push_str(&format!("M {px:.1} {py:.1}"));
        } else {
            path_d.push_str(&format!(" L {px:.1} {py:.1}"));
        }
    }
    svg1.push_str(&format!(
        r##"<path d="{path_d}" fill="none" stroke="#111827" stroke-width="2" stroke-dasharray="5,3"/>
"##
    ));

    let find_arm = |lbl: &str| arms.iter().find(|a| a["arm_label"].as_str() == Some(lbl));
    if let (Some(sp_o), Some(sp_a)) = (
        find_arm("sparse-table:original"),
        find_arm("sparse-table:traced-anchored"),
    ) {
        let fv = sp_a["anchor_coverage_f"].as_f64().unwrap_or(0.0);
        let emp_m = u128_of(&sp_o["residuals_tried"]) / u128_of(&sp_a["residuals_tried"]).max(1.0);
        let px = rx0 + rpw * (fv / f_max).min(1.0);
        let py = ry0 + rph * (1.0 - ((emp_m.clamp(1.0, m_max) - 1.0) / (m_max - 1.0)));
        svg1.push_str(&format!(
            r##"<circle cx="{px:.1}" cy="{py:.1}" r="5.2" fill="#2563eb"/>
<text x="{:.1}" y="{:.1}" font-size="9.2" fill="#1d4ed8">1LP ({:.2}%, {emp_m:.2}x)</text>
"##,
            px + 8.0,
            py + 4.0,
            fv * 100.0
        ));
    }
    if let (Some(df_o), Some(df_a)) = (
        find_arm("deferred-pairs:original"),
        find_arm("deferred-pairs-fast:traced-anchored"),
    ) {
        let fv = df_a["anchor_coverage_f"].as_f64().unwrap_or(0.0);
        let emp_m = u128_of(&df_o["residuals_tried"]) / u128_of(&df_a["residuals_tried"]).max(1.0);
        let px = rx0 + rpw * (fv / f_max).min(1.0);
        let py = ry0 + rph * (1.0 - ((emp_m.clamp(1.0, m_max) - 1.0) / (m_max - 1.0)));
        svg1.push_str(&format!(
            r##"<circle cx="{px:.1}" cy="{py:.1}" r="5.5" fill="#f59e0b"/>
<text x="{:.1}" y="{:.1}" font-size="9.2" fill="#b45309">2LP-BFS ({:.1}%, {emp_m:.2}x)</text>
"##,
            (px - 125.0).max(rx0 + 6.0),
            py - 9.0,
            fv * 100.0
        ));
    }
    if let (Some(df_o), Some(mt_a)) = (
        find_arm("deferred-pairs:original"),
        find_arm("signed-matroid-2lp:traced-anchored"),
    ) {
        let fv = mt_a["anchor_coverage_f"].as_f64().unwrap_or(0.0);
        let emp_m = u128_of(&df_o["residuals_tried"]) / u128_of(&mt_a["residuals_tried"]).max(1.0);
        let px = rx0 + rpw * (fv / f_max).min(1.0);
        let py = ry0 + rph * (1.0 - ((emp_m.clamp(1.0, m_max) - 1.0) / (m_max - 1.0)));
        svg1.push_str(&format!(
            r##"<circle cx="{px:.1}" cy="{py:.1}" r="6.2" fill="#dc2626"/>
<text x="{:.1}" y="{:.1}" font-size="9.3" font-weight="bold" fill="#b91c1c">Matroid-2LP ({:.1}%, {emp_m:.2}x)</text>
"##,
            (px + 8.0).min(rx0 + rpw - 135.0),
            py + 14.0,
            fv * 100.0
        ));
    }
    svg1.push_str(&format!(
        r##"<text x="{:.1}" y="398" font-size="10" fill="#374151" text-anchor="middle">Exact Quadratic Universe Coverage f = |V_anc| / |Q_2(p)|</text>
<text x="625" y="72" font-size="9.5" fill="#111827">Dashed: M_pred(f) = 1 + 15f + 45f^2 + 15f^3</text>
</svg>
"##,
        rx0 + rpw / 2.0
    ));
    fs::write(out_dir.join("run50_ablation.svg"), svg1.as_bytes())
        .map_err(|e| format!("write svg1: {e}"))?;

    // Figure 2: Multi-prime log-log scaling (IC vs Karatsuba/Schoolbook Pollard rho + f(p) scaling).
    let aggs = pb["per_prime_aggregates"]
        .as_array()
        .ok_or("missing per_prime_aggregates")?;
    let mut svg2 = String::with_capacity(14000);
    svg2.push_str(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 920 405" width="920" height="405" font-family="DejaVu Sans, sans-serif">
<rect width="920" height="405" fill="#ffffff"/>
<text x="18" y="24" font-size="13" font-weight="bold" fill="#111827">(a) Multi-Prime Cold Scaling: Signed-Matroid IC vs Executed Pollard Rho</text>
<text x="530" y="24" font-size="13" font-weight="bold" fill="#111827">(b) Matroid Anchor Coverage f(p) and Invariant p * f(p)</text>
"##,
    );

    let lx0 = 68.0_f64;
    let lpw = 410.0_f64;
    let ly0 = 48.0_f64;
    let lph = 278.0_f64;
    let logp_min = (290.0_f64).log10();
    let logp_max = (1060.0_f64).log10();
    let logy_min = 11.2_f64;
    let logy_max = 14.3_f64;

    svg2.push_str(&format!(
        r##"<rect x="{lx0:.1}" y="{ly0:.1}" width="{lpw:.1}" height="{lph:.1}" fill="none" stroke="#9ca3af" stroke-width="1"/>
"##
    ));

    for &pv in &[307.0_f64, 383.0, 503.0, 661.0, 1009.0] {
        let px = lx0 + lpw * ((pv.log10() - logp_min) / (logp_max - logp_min));
        svg2.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{ly0:.1}" x2="{px:.1}" y2="{:.1}" stroke="#f3f4f6" stroke-width="1"/>
<text x="{px:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="middle">{pv:.0}</text>
"##,
            ly0 + lph,
            ly0 + lph + 15.0
        ));
    }
    for exp in [11.5_f64, 12.0, 12.5, 13.0, 13.5, 14.0] {
        let py = ly0 + lph * (1.0 - (exp - logy_min) / (logy_max - logy_min));
        svg2.push_str(&format!(
            r##"<line x1="{lx0:.1}" y1="{py:.1}" x2="{:.1}" y2="{py:.1}" stroke="#f3f4f6" stroke-width="1"/>
<text x="{:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="end">10^{exp:.1}</text>
"##,
            lx0 + lpw,
            lx0 - 6.0,
            py + 3.5
        ));
    }

    let plot_series = |svg: &mut String, key: &str, color: &str, dash: &str| {
        let mut d = String::new();
        for (i, r) in aggs.iter().enumerate() {
            let pv = r["p"].as_f64().unwrap_or(307.0);
            let yv = r[key].as_f64().unwrap_or(1.0).max(1.0).log10();
            let px = lx0 + lpw * ((pv.log10() - logp_min) / (logp_max - logp_min));
            let py = ly0
                + lph * (1.0 - (yv.clamp(logy_min, logy_max) - logy_min) / (logy_max - logy_min));
            if i == 0 {
                d.push_str(&format!("M {px:.1} {py:.1}"));
            } else {
                d.push_str(&format!(" L {px:.1} {py:.1}"));
            }
            svg.push_str(&format!(
                r##"<circle cx="{px:.1}" cy="{py:.1}" r="3.8" fill="{color}"/>
"##
            ));
        }
        let dash_attr = if dash.is_empty() {
            String::new()
        } else {
            format!(r#" stroke-dasharray="{dash}""#)
        };
        svg.push_str(&format!(
            r##"<path d="{d}" fill="none" stroke="{color}" stroke-width="2"{dash_attr}/>
"##
        ));
    };

    plot_series(
        &mut svg2,
        "rho_analytical_b16_schoolbook_mean",
        "#6b7280",
        "4,3",
    );
    plot_series(
        &mut svg2,
        "rho_analytical_b16_karatsuba_mean",
        "#111827",
        "",
    );
    plot_series(&mut svg2, "deferred_orig_total_w0_mean", "#f59e0b", "5,3");
    plot_series(
        &mut svg2,
        "deferred_fast_ta_total_w0_mean",
        "#2563eb",
        "2,2",
    );
    plot_series(&mut svg2, "matroid_2lp_ta_total_w0_mean", "#dc2626", "");

    svg2.push_str(&format!(
        r##"<text x="{:.1}" y="356" font-size="10" fill="#374151" text-anchor="middle">Base Prime p (log scale; E(F_{{p^6}}) subgroup l ~ p^4)</text>
<rect x="24" y="369" width="10" height="3" fill="#111827"/><text x="38" y="374" font-size="9" fill="#111827">Rho B=16 (Karatsuba)</text>
<rect x="160" y="369" width="10" height="3" fill="#6b7280"/><text x="174" y="374" font-size="9" fill="#4b5563">Rho B=16 (Schoolbook)</text>
<rect x="305" y="369" width="10" height="3" fill="#f59e0b"/><text x="319" y="374" font-size="9" fill="#b45309">2LP-BFS Orig (w=0)</text>
<rect x="24" y="385" width="10" height="3" fill="#dc2626"/><text x="38" y="390" font-size="9" font-weight="bold" fill="#b91c1c">Signed-Matroid-2LP + Trace+Anc (w=0)</text>
<rect x="255" y="385" width="10" height="3" fill="#2563eb"/><text x="269" y="390" font-size="9" fill="#1d4ed8">2LP-BFS Fast+Trace+Anc (w=0)</text>
"##,
        lx0 + lpw / 2.0
    ));

    let rx0 = 560.0_f64;
    let rpw = 325.0_f64;
    let ry0 = 48.0_f64;
    let rph = 278.0_f64;
    let mut f_top = 0.28_f64;
    for r in aggs {
        let fv = r["matroid_anchor_f_mean"].as_f64().unwrap_or(0.0);
        if fv * 1.18 > f_top {
            f_top = fv * 1.18;
        }
    }

    svg2.push_str(&format!(
        r##"<rect x="{rx0:.1}" y="{ry0:.1}" width="{rpw:.1}" height="{rph:.1}" fill="none" stroke="#9ca3af" stroke-width="1"/>
"##
    ));

    let mut d_f = String::new();
    for (i, r) in aggs.iter().enumerate() {
        let pv = r["p"].as_f64().unwrap_or(307.0);
        let fv = r["matroid_anchor_f_mean"].as_f64().unwrap_or(0.0);
        let pf = r["matroid_p_times_f_mean"].as_f64().unwrap_or(0.0);
        let sp_ok = r["sparse_fast_ta_solved"].as_u64().unwrap_or(0);
        let mt_ok = r["matroid_2lp_ta_solved"].as_u64().unwrap_or(0);
        let px = rx0 + rpw * ((pv - 270.0) / (1050.0 - 270.0));
        let py = ry0 + rph * (1.0 - (fv / f_top).min(1.0));
        if i == 0 {
            d_f.push_str(&format!("M {px:.1} {py:.1}"));
        } else {
            d_f.push_str(&format!(" L {px:.1} {py:.1}"));
        }
        svg2.push_str(&format!(
            r##"<line x1="{px:.1}" y1="{ry0:.1}" x2="{px:.1}" y2="{:.1}" stroke="#f3f4f6" stroke-width="1"/>
<text x="{px:.1}" y="{:.1}" font-size="9.5" fill="#4b5563" text-anchor="middle">{pv:.0}</text>
<circle cx="{px:.1}" cy="{py:.1}" r="4.8" fill="#059669"/>
<text x="{px:.1}" y="{:.1}" font-size="8.6" fill="#065f46" text-anchor="middle">{:.1}% (pf={pf:.0})</text>
<text x="{px:.1}" y="{:.1}" font-size="8.2" fill="#374151" text-anchor="middle">1LP:{sp_ok}/3 M2:{mt_ok}/3</text>
"##,
            ry0 + rph,
            ry0 + rph + 15.0,
            py - 8.0,
            fv * 100.0,
            ry0 + rph - 8.0
        ));
    }
    svg2.push_str(&format!(
        r##"<path d="{d_f}" fill="none" stroke="#059669" stroke-width="2.2"/>
<text x="{:.1}" y="356" font-size="10" fill="#374151" text-anchor="middle">Base Prime p (badges: 1-LP vs Matroid-2LP targets solved out of 3)</text>
</svg>
"##,
        rx0 + rpw / 2.0
    ));
    fs::write(out_dir.join("multiprime_scaling.svg"), svg2.as_bytes())
        .map_err(|e| format!("write svg2: {e}"))?;

    // Figure 3: 7-Way Partial-Relation Graph Engine Head-to-Head on Run50 Stream.
    if let Some(engs) = pa["graph_engine_head_to_head"]["engines"].as_array() {
        let mut max_g_bops = 1.0_f64;
        let mut max_rank = 500.0_f64;
        for e in engs {
            let b = u128_of(&e["graph_bitops"]) / 1e8;
            if b > max_g_bops {
                max_g_bops = b;
            }
            let r = e["certified_fb_rank"].as_f64().unwrap_or(0.0);
            if r > max_rank {
                max_rank = r;
            }
        }
        let g_ceil = (max_g_bops * 1.22).ceil().max(1.0);
        let r_ceil = (max_rank * 1.12).ceil().max(1.0);

        let mut svg3 = String::with_capacity(12000);
        svg3.push_str(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 920 310" width="920" height="310" font-family="DejaVu Sans, sans-serif">
<rect width="920" height="310" fill="#ffffff"/>
<text x="18" y="24" font-size="13" font-weight="bold" fill="#111827">(a) Graph + Projection Engine Cost on Identical Run50 Stream (bitop/v1, x10^8)</text>
<text x="530" y="24" font-size="13" font-weight="bold" fill="#111827">(b) Certified Factor-Base Rank and Z-Exact Descent Anchors</text>
"##,
        );

        let gx0 = 195.0_f64;
        let gpw = 285.0_f64;
        let gy0 = 46.0_f64;
        let gh = 30.0_f64;
        let rx0 = 545.0_f64;
        let rpw = 255.0_f64;

        for (i, e) in engs.iter().enumerate() {
            let name = e["engine"].as_str().unwrap_or("?");
            let exact_z = e["exact_z_cofactor"].as_bool().unwrap_or(false);
            let bops_e8 = u128_of(&e["graph_bitops"]) / 1e8;
            let rnk = e["certified_fb_rank"].as_f64().unwrap_or(0.0);
            let u_anc = e["exported_unit_anchors"].as_u64().unwrap_or(0);
            let h_anc = e["exported_half_anchors"].as_u64().unwrap_or(0);
            let ry = gy0 + (i as f64) * gh + 4.0;
            let bw = gpw * (bops_e8 / g_ceil).min(1.0);
            let rw = rpw * (rnk / r_ceil).min(1.0);
            let col = if name.contains("signed-matroid") {
                "#dc2626"
            } else if exact_z {
                "#2563eb"
            } else {
                "#6b7280"
            };
            let wt = if name.contains("signed-matroid") {
                r#" font-weight="bold""#
            } else {
                ""
            };
            svg3.push_str(&format!(
                r##"<text x="{:.1}" y="{:.1}" font-size="9.5" fill="#1f2937" text-anchor="end"{wt}>{name}</text>
<rect x="{gx0:.1}" y="{ry:.1}" width="{bw:.1}" height="18" fill="{col}"/>
<text x="{:.1}" y="{:.1}" font-size="9" fill="#111827"{wt}>{bops_e8:.2}e8</text>
<rect x="{rx0:.1}" y="{ry:.1}" width="{rw:.1}" height="18" fill="{col}" opacity="0.85"/>
<text x="{:.1}" y="{:.1}" font-size="8.8" fill="#111827"{wt}>rk={rnk:.0} | anc={u_anc}+{h_anc}h</text>
"##,
                gx0 - 7.0,
                ry + 12.8,
                gx0 + bw + 5.0,
                ry + 12.8,
                rx0 + rw + 5.0,
                ry + 12.8
            ));
        }
        svg3.push_str(
            r##"<rect x="30" y="282" width="10" height="10" fill="#dc2626"/><text x="45" y="291" font-size="9.2" fill="#374151">Online Z-Exact Signed-Matroid DSU / 3-LP Hypergraph (exports unit + half anchors)</text>
<rect x="495" y="282" width="10" height="10" fill="#2563eb"/><text x="510" y="291" font-size="9.2" fill="#374151">Legacy Z-Exact (1-LP / 2-LP BFS)</text>
<rect x="695" y="282" width="10" height="10" fill="#6b7280"/><text x="710" y="291" font-size="9.2" fill="#374151">F_l-Inverting Filters (0 Z-anchors)</text>
</svg>
"##,
        );
        fs::write(out_dir.join("graph_engine_comparison.svg"), svg3.as_bytes())
            .map_err(|e| format!("write svg3: {e}"))?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "usage: {} <census|run50|multiprime|overflow-control|figures> <arg...>",
            args[0]
        );
        std::process::exit(2);
    }
    let res = match args[1].as_str() {
        "census" => {
            let p: u64 = args[2].parse().expect("p");
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(389);
            let spec = generate_spec(p, seed);
            let ctx = Ctx::new(&spec);
            let c = count_quadratic_universe(&ctx);
            println!("{}", serde_json::to_string_pretty(&c).unwrap());
            Ok(())
        }
        "run50" => run_panel_a_run50(&PathBuf::from(&args[2])),
        "multiprime" => run_panel_b_multiprime(&PathBuf::from(&args[2])),
        "overflow-control" => run_overflow_control(&PathBuf::from(&args[2])),
        "figures" => {
            if args.len() < 5 {
                Err("usage: figures <panel_a_summary.json> <panel_b_summary.json> <out_dir>".into())
            } else {
                render_study_figures(
                    &PathBuf::from(&args[2]),
                    &PathBuf::from(&args[3]),
                    &PathBuf::from(&args[4]),
                )
            }
        }
        other => Err(format!("unknown subcommand {other}")),
    };
    if let Err(e) = res {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}
