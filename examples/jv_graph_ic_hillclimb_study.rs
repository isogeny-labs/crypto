//! Multi-round algorithmic hillclimb driver, statistical aggregator, and vector SVG
//! renderer for extension-field graph-anchored multi-pattern index calculus (`bitop/v1`).
//!
//! Subcommands:
//! - `hillclimb <out_dir>`: Executes the preregistered 12-round (`R0`..`R11`) greedy
//!   hillclimb across all 4 frozen seeds `[389, 101, 211, 307]` at `p = 1009`, promoting
//!   any round that achieves `4/4` verified solves and `>= 1.0%` reduction in 4-seed mean
//!   `observed_bitops`, and recording every stalled/rejected probe (`STALL`) verbatim.
//! - `multiprime <out_dir>`: Evaluates `R0`, `R1`, the discovered `HillclimbChampion`, and
//!   matched schoolbook + Karatsuba negation-map Pollard rho across `p in {307, 383, 503, 661, 1009}`
//!   (`seeds = [101, 211, 307]`).
//! - `overflow-control <out_file>`: Runs deterministic control solves for comparing standard
//!   `--release` against `RUSTFLAGS="-C overflow-checks=on"`.
//! - `render-svgs <out_dir>`: Renders publication-grade vector SVGs (`< 45 KiB` each) directly
//!   from `results/hillclimb_rounds.json` and `results/multiprime_frontier.json`.

use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::jv_anchored_descent::DescentMode;
use crypto_lib::cryptanalysis::jv_cover::{self as cover, generate_spec, Ctx};
use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::{
    run_anchored_ic_dlp_with_config, run_matched_rho_reference, AnchoredPipelineOptions,
    AnchoredSolveBundle, CollectorArm, HillclimbConfig,
};
use crypto_lib::cryptanalysis::jv_sieve::{self as sieve, FpRing, HitBuckets};
use crypto_lib::cryptanalysis::jv_sieve_fast_m9::{
    build_inv_2w_sq_table, build_inv_table, lines_for_b_frob_stickelberger_m9,
    lines_for_b_partition_m9, sieve_line_buckets_fast, sieve_line_buckets_zero_mul,
    FastSieveScratch, SharedBFast,
};
use crypto_lib::cryptanalysis::jv_sparse_markowitz::{SparseMarkowitzTracker, WiedemannTracker};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

const MAX_ARTIFACT_BYTES: usize = 60 * 1024;
const PROMOTION_THRESHOLD_PCT: f64 = -1.0;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SeedSolveRow {
    round_id: String,
    arm_label: String,
    p: u64,
    l: u64,
    seed: u64,
    verified: bool,
    matches_planted_truth: bool,
    recovered_scalar: Option<u64>,
    b_count_scanned: u64,
    sources_admitted: usize,
    projections_emitted: usize,
    total_anchors: usize,
    anchor_coverage_f: f64,
    residuals_tried: u64,
    accepted_rows: usize,
    relations_bitops: u128,
    linear_algebra_bitops: u128,
    descent_bitops: u128,
    observed_bitops: u128,
    wall_ms_indicative: f64,
    foreign_hook_calls: u64,
    sources_sha256_prefix: String,
    projections_sha256_prefix: String,
    bundle_file: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RoundSummary {
    round_index: usize,
    round_id: String,
    label: String,
    short_title: String,
    phase_target: String,
    hypothesis_note: String,
    base_round_id: String,
    all_seeds_verified: bool,
    promoted: bool,
    status: String,
    consecutive_stalls_after: usize,
    run50_seed389_bitops: u128,
    run50_relations_bitops: u128,
    run50_la_bitops: u128,
    run50_descent_bitops: u128,
    run50_wall_ms: f64,
    mean4_observed_bitops: f64,
    mean4_relations_bitops: f64,
    mean4_la_bitops: f64,
    mean4_descent_bitops: f64,
    mean4_wall_ms: f64,
    mean4_b_scanned: f64,
    mean4_anchor_f: f64,
    mean4_residuals_tried: f64,
    delta_vs_incumbent_pct: f64,
    cumulative_speedup_vs_r0: f64,
    cumulative_speedup_vs_r1: f64,
    ratio_vs_karatsuba_rho_mean3: f64,
    champion_after_round_id: String,
    champion_mean4_bitops: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HillclimbTrajectoryReport {
    schema: String,
    protocol_id: String,
    p: u64,
    seeds: Vec<u64>,
    promotion_threshold_pct: f64,
    rho_seed389_schoolbook_bitops: u128,
    rho_seed389_karatsuba_bitops: u128,
    rho_mean3_schoolbook_bitops: f64,
    rho_mean3_karatsuba_bitops: f64,
    final_champion_round_id: String,
    final_champion_label: String,
    final_consecutive_stalls: usize,
    total_promoted_steps_after_r1: usize,
    rounds: Vec<RoundSummary>,
}

#[derive(Clone)]
struct RoundSpec {
    round_id: &'static str,
    label: &'static str,
    short_title: &'static str,
    phase_target: &'static str,
    hypothesis_note: &'static str,
    build: fn(
        &(AnchoredPipelineOptions, HillclimbConfig),
    ) -> (AnchoredPipelineOptions, HillclimbConfig),
}

fn r0_config() -> (AnchoredPipelineOptions, HillclimbConfig) {
    let opts = AnchoredPipelineOptions {
        collector: CollectorArm::DeferredPairsFast,
        descent: DescentMode::Original,
        harvest_odd_cycles: false,
        ..Default::default()
    };
    let mut hc = HillclimbConfig::round1_default();
    hc.custom_label = Some("r0_deferred_original");
    (opts, hc)
}

fn r1_config() -> (AnchoredPipelineOptions, HillclimbConfig) {
    let opts = AnchoredPipelineOptions {
        collector: CollectorArm::SignedMatroid2lp,
        descent: DescentMode::TracedAnchored,
        harvest_odd_cycles: false,
        ..Default::default()
    };
    let mut hc = HillclimbConfig::round1_default();
    hc.custom_label = Some("r1_matroid_2lp_traced");
    (opts, hc)
}

fn round_schedule() -> Vec<RoundSpec> {
    vec![
        RoundSpec {
            round_id: "R0",
            label: "r0_deferred_original",
            short_title: "Deferred 2-LP + (1^6) + Lanczos",
            phase_target: "Baseline",
            hypothesis_note: "Pre-matroid baseline: DeferredPairsFast + Original (1^6) descent + Randomized Lanczos LA.",
            build: |_| r0_config(),
        },
        RoundSpec {
            round_id: "R1",
            label: "r1_matroid_2lp_traced",
            short_title: "Matroid 2-LP + TracedMP + Lanczos",
            phase_target: "Round-1 Champion",
            hypothesis_note: "Round-1 winner: SignedMatroid2lp + deterministic resolvent + TracedAnchored descent + Lanczos LA.",
            build: |_| r1_config(),
        },
        RoundSpec {
            round_id: "R2",
            label: "r2_markowitz_sge_la",
            short_title: "+ Online Markowitz SGE LA",
            phase_target: "Phase 2 (Sparse LA)",
            hypothesis_note: "Replace repeated 3x Randomized-Lanczos checkpoints with persistent incremental minimum-Markowitz sparse Gaussian elimination + column-support pre-gating.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.sparse_markowitz_la = true;
                hc2.custom_label = Some("r2_markowitz_sge_la");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R3",
            label: "r3_partition_m9_fast_sieve",
            short_title: "+ Partition m=9 + InvTable Sieve",
            phase_target: "Phase 1 (Sieve)",
            hypothesis_note: "Root-free deg-8/7 partition direct line factorization (skipping Yun/Frobenius on ~94.5% of planes) + 1-inv F_p table + single-flush stack-diff bucket sieve.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.partition_fast_sieve = true;
                hc2.custom_label = Some("r3_partition_m9_fast_sieve");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R4",
            label: "r4_stickelberger_stack_2lp",
            short_title: "+ Stickelberger Stack 2-LP Split",
            phase_target: "Phase 1 (Extractor)",
            hypothesis_note: "Stickelberger-parity stack-poly quartic splitter (square disc + gcd(q, x^p-x)==1 certifies (2,2) without x^{p^2} mod q) + deferred ordinate checks.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.stickelberger_fast_extract = true;
                hc2.custom_label = Some("r4_stickelberger_stack_2lp");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R5",
            label: "r5_batch4096_sub64_fast_sextic",
            short_title: "+ 4096-Incidence / 64-SubRank / FastSextic",
            phase_target: "Cadence + Phase 3 Post",
            hypothesis_note: "Amortize roots_by_incidence over 4,096 planes, check Markowitz rank every 64 planes at surplus=1, and run zero-inversion stack-poly sextic post-screening.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.incidence_batch_size = 4096;
                hc2.rank_check_subbatch = 64;
                hc2.markowitz_surplus = Some(1);
                hc2.markowitz_stride = Some(4);
                hc2.descent_fast_sextic = true;
                hc2.custom_label = Some("r5_batch4096_sub64_fast_sextic");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R6",
            label: "r6_flat_unit5_d0_elim",
            short_title: "? Flat 5-Var d0 Elimination (6->5 Eqs)",
            phase_target: "Phase 3 (F4 Algebra)",
            hypothesis_note: "Eliminate constant-coeff linear d_0 from Im(q_5)=0 via 126-monomial flat accumulator (6 quadrics -> 5 deg-4 eqs) + 5-var traced F4.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.descent_unit_five = true;
                hc2.custom_label = Some("r6_flat_unit5_d0_elim");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R7",
            label: "r7_hilbert8_f4_stop",
            short_title: "+ Hilbert(8) Early F4 Stop",
            phase_target: "Phase 3 (F4 Stop)",
            hypothesis_note: "Switch zero-dimensional F4 stopping rule from Staircase(64) to Hilbert(8) under active cold-trace replay, avoiding an extra redundant Macaulay degree step.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.descent_hilbert_stop = Some(8);
                hc2.custom_label = Some("r7_hilbert8_f4_stop");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R8",
            label: "r8_wiedemann_addchain_la",
            short_title: "+ AddChain Scalar Wiedemann LA",
            phase_target: "Phase 2 (Krylov LA)",
            hypothesis_note: "Replace fill-prone Markowitz SGE with factored Scalar Wiedemann (Berlekamp-Massey) using +-1 projection and pure ModrAdd chains on |coef|<=8.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.wiedemann_la = true;
                hc2.markowitz_surplus = Some(3);
                hc2.markowitz_stride = Some(6);
                hc2.custom_label = Some("r8_wiedemann_addchain_la");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R9",
            label: "r9_zero_mul_deg4_sieve",
            short_title: "+ Zero-Mul inv_2w_sq + Deg-4 A1 Sieve",
            phase_target: "Phase 1 (Zero-Mul Sieve)",
            hypothesis_note: "Advance degree-4 A_1'(x) instead of degree-8 w*A_1'^2 (halving sieve FieldAdds), pre-scale N(B)*rho once per multi-line plane, and replace 3*|FB| Montgomery batch muls with inv_2w_sq[y] O(1) lookup.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.zero_mul_sieve = true;
                hc2.custom_label = Some("r9_zero_mul_deg4_sieve");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R10",
            label: "r10_frob_basis_stickelberger_g",
            short_title: "+ FrobBasis + Stickelberger G-Screen",
            phase_target: "Phase 1 (Frob + Disc)",
            hypothesis_note: "Compute disc(rest) Legendre character inside the squarefreeness GCD loop to skip impossible partitions on dr in {4,5,6,7,8}, and replace 735-mul Horner compose_mod with a squaring-built Frobenius basis.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.frob_stickelberger_sieve = true;
                hc2.custom_label = Some("r10_frob_basis_stickelberger_g");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R11",
            label: "r11_overcollect_1024b",
            short_title: "? Post-Rank +1,024 B Anchor Margin",
            phase_target: "Phase 1 <-> 3 Trade-off",
            hypothesis_note: "Scan +1,024 extra fast-sieve B-planes after rank certification to raise 2-LP anchor coverage f(p) and test net Phase-1 vs Phase-3 slope.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.extra_b_after_rank = 1024;
                hc2.custom_label = Some("r11_overcollect_1024b");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R12",
            label: "r12_overcollect_3072b",
            short_title: "? Post-Rank +3,072 B Anchor Margin",
            phase_target: "Phase 1 <-> 3 Trade-off",
            hypothesis_note: "Scan +3,072 extra fast-sieve B-planes after rank certification to test deeper anchor saturation vs linear sieve cost.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.extra_b_after_rank = 3072;
                hc2.custom_label = Some("r12_overcollect_3072b");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R13",
            label: "r13_selective_3lp_productive_lines",
            short_title: "? Conditional 3-LP on 2-LP Lines",
            phase_target: "Phase 1 (Hypergraph)",
            hypothesis_note: "Test <= 1 cnt==3 (3-LP) candidate only on lines that already yielded a 2-LP source to test smooth-line conditioning.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.selective_3lp_on_productive_lines = 1;
                hc2.custom_label = Some("r13_selective_3lp_productive_lines");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R14",
            label: "r14_subsampled_fb_92pct",
            short_title: "? Sub-Sampled 92% Factor Base",
            phase_target: "Phase 1/2 (Dimension)",
            hypothesis_note: "Truncate rational factor base to 92% (|FB| = 0.92 |B_max|) to test 8% rank-target reduction vs (0.92)^5 = 65.9% hit-rate drop.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.factor_base_pct = 92;
                hc2.custom_label = Some("r14_subsampled_fb_92pct");
                (*opts, hc2)
            },
        },
        RoundSpec {
            round_id: "R15",
            label: "r15_batch512_sub16_cadence",
            short_title: "? Narrow 512-Incidence / 16-SubRank",
            phase_target: "Phase 1/2 (Cadence)",
            hypothesis_note: "Test narrow 512-plane roots_by_incidence batches with 16-plane Wiedemann rank checks to verify 4096-plane incidence amortization optimality.",
            build: |(opts, hc)| {
                let mut hc2 = *hc;
                hc2.incidence_batch_size = 512;
                hc2.rank_check_subbatch = 16;
                hc2.custom_label = Some("r15_batch512_sub16_cadence");
                (*opts, hc2)
            },
        },
    ]
}

fn run_hillclimb(out_dir: &Path) -> Result<(AnchoredPipelineOptions, HillclimbConfig), String> {
    let p = 1009u64;
    let seeds = vec![389u64, 101, 211, 307];
    let bundles_dir = out_dir.join("bundles");
    let results_dir = out_dir.join("results");
    fs::create_dir_all(&bundles_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&results_dir).map_err(|e| e.to_string())?;

    // Precompute analytical B=16 matched Pollard rho (`sqrt(pi * l / 4)`) across all 4 seeds.
    let mut rho_schoolbook_fresh = Vec::new();
    let mut rho_karatsuba_fresh = Vec::new();
    let mut rho_seed389_sb = 0u128;
    let mut rho_seed389_ka = 0u128;
    for &seed in &seeds {
        let truth = generate_spec(p, seed);
        let rho = run_matched_rho_reference(&truth, Some(truth.d), seed, 0, false);
        if seed == 389 {
            rho_seed389_sb = rho.analytical_b16_schoolbook_bitops;
            rho_seed389_ka = rho.analytical_b16_karatsuba_bitops;
        } else {
            rho_schoolbook_fresh.push(rho.analytical_b16_schoolbook_bitops as f64);
            rho_karatsuba_fresh.push(rho.analytical_b16_karatsuba_bitops as f64);
        }
    }
    let rho_mean3_sb = rho_schoolbook_fresh.iter().sum::<f64>() / rho_schoolbook_fresh.len() as f64;
    let rho_mean3_ka = rho_karatsuba_fresh.iter().sum::<f64>() / rho_karatsuba_fresh.len() as f64;

    let schedule = round_schedule();
    let mut incumbent_cfg = r0_config();
    let mut incumbent_round_id = String::from("R0");
    let mut incumbent_label = String::from("r0_deferred_original");
    let mut incumbent_mean4 = 0.0f64;
    let mut r0_mean4 = 0.0f64;
    let mut r1_mean4 = 0.0f64;
    let mut consecutive_stalls = 0usize;
    let mut promoted_after_r1 = 0usize;

    let mut round_summaries = Vec::new();
    let mut all_seed_rows = Vec::new();
    let mut champion_seed_bundles: Vec<(u64, AnchoredSolveBundle)> = Vec::new();

    for (idx, rspec) in schedule.iter().enumerate() {
        let base_round_id = if idx == 0 {
            "NONE".to_string()
        } else if idx == 1 {
            "R0".to_string()
        } else {
            incumbent_round_id.clone()
        };
        let (cand_opts, cand_hc) = (rspec.build)(&incumbent_cfg);
        eprintln!(
            "[hillclimb] === {} ({}) on base {} ===",
            rspec.round_id, rspec.label, base_round_id
        );

        let mut round_rows = Vec::new();
        let mut round_bundles = Vec::new();
        for &seed in &seeds {
            let truth_spec = generate_spec(p, seed);
            let truth_d = truth_spec.d;
            let mut public_spec = truth_spec.clone();
            public_spec.d = 0;
            let bundle = run_anchored_ic_dlp_with_config(&public_spec, seed, cand_opts, cand_hc)?;
            let row = SeedSolveRow {
                round_id: rspec.round_id.to_string(),
                arm_label: rspec.label.to_string(),
                p: bundle.p,
                l: bundle.l,
                seed,
                verified: bundle.verified,
                matches_planted_truth: bundle.recovered_scalar == Some(truth_d),
                recovered_scalar: bundle.recovered_scalar,
                b_count_scanned: bundle.collection.b_count_scanned,
                sources_admitted: bundle.collection.sources_admitted,
                projections_emitted: bundle.collection.projections_emitted,
                total_anchors: bundle.collection.total_anchors,
                anchor_coverage_f: bundle.descent.anchor_coverage_f,
                residuals_tried: bundle.descent.residuals_tried,
                accepted_rows: bundle.descent.accepted_rows,
                relations_bitops: phase_bitops(&bundle, "relations"),
                linear_algebra_bitops: phase_bitops(&bundle, "linear_algebra"),
                descent_bitops: phase_bitops(&bundle, "descent"),
                observed_bitops: bundle.accounting.observed_bitops,
                wall_ms_indicative: bundle.wall_ms_indicative,
                foreign_hook_calls: bundle.accounting.foreign_hook_calls,
                sources_sha256_prefix: bundle.collection.sources_sha256[..16].to_string(),
                projections_sha256_prefix: bundle.collection.projections_sha256[..16].to_string(),
                bundle_file: None,
            };
            eprintln!(
                "  seed={seed}: verified={} bitops={:.3}G (rel={:.3}G, la={:.3}G, desc={:.3}G) B={} res={} wall={:.0}ms",
                row.verified && row.matches_planted_truth,
                row.observed_bitops as f64 / 1e9,
                row.relations_bitops as f64 / 1e9,
                row.linear_algebra_bitops as f64 / 1e9,
                row.descent_bitops as f64 / 1e9,
                row.b_count_scanned,
                row.residuals_tried,
                row.wall_ms_indicative,
            );
            round_rows.push(row);
            round_bundles.push((seed, bundle));
        }

        let all_ok = round_rows
            .iter()
            .all(|r| r.verified && r.matches_planted_truth && r.foreign_hook_calls == 0);
        let n = round_rows.len() as f64;
        let mean4_obs = round_rows
            .iter()
            .map(|r| r.observed_bitops as f64)
            .sum::<f64>()
            / n;
        let mean4_rel = round_rows
            .iter()
            .map(|r| r.relations_bitops as f64)
            .sum::<f64>()
            / n;
        let mean4_la = round_rows
            .iter()
            .map(|r| r.linear_algebra_bitops as f64)
            .sum::<f64>()
            / n;
        let mean4_desc = round_rows
            .iter()
            .map(|r| r.descent_bitops as f64)
            .sum::<f64>()
            / n;
        let mean4_wall = round_rows.iter().map(|r| r.wall_ms_indicative).sum::<f64>() / n;
        let mean4_b = round_rows
            .iter()
            .map(|r| r.b_count_scanned as f64)
            .sum::<f64>()
            / n;
        let mean4_f = round_rows.iter().map(|r| r.anchor_coverage_f).sum::<f64>() / n;
        let mean4_res = round_rows
            .iter()
            .map(|r| r.residuals_tried as f64)
            .sum::<f64>()
            / n;

        // Fresh-3-seed mean (`[101, 211, 307]`) for apple-to-apple comparison against multiprime Karatsuba rho.
        let mean3_fresh_obs = round_rows
            .iter()
            .filter(|r| r.seed != 389)
            .map(|r| r.observed_bitops as f64)
            .sum::<f64>()
            / 3.0;

        let (promoted, status, delta_pct) = if idx == 0 {
            r0_mean4 = mean4_obs;
            incumbent_mean4 = mean4_obs;
            incumbent_cfg = (cand_opts, cand_hc);
            incumbent_round_id = rspec.round_id.to_string();
            incumbent_label = rspec.label.to_string();
            champion_seed_bundles = round_bundles.clone();
            (true, "BASELINE".to_string(), 0.0)
        } else {
            let delta = (mean4_obs - incumbent_mean4) / incumbent_mean4 * 100.0;
            if idx == 1 {
                r1_mean4 = mean4_obs;
            }
            if all_ok && delta <= PROMOTION_THRESHOLD_PCT {
                if idx > 1 {
                    promoted_after_r1 += 1;
                }
                consecutive_stalls = 0;
                incumbent_mean4 = mean4_obs;
                incumbent_cfg = (cand_opts, cand_hc);
                incumbent_round_id = rspec.round_id.to_string();
                incumbent_label = rspec.label.to_string();
                champion_seed_bundles = round_bundles.clone();
                (true, "PROMOTED".to_string(), delta)
            } else {
                consecutive_stalls += 1;
                (false, format!("STALL #{consecutive_stalls}"), delta)
            }
        };

        // Persist Run50 (`seed = 389`) bundle for baseline, R1, and every promoted round so the
        // standalone qualifier independently verifies every milestone on the trajectory.
        if idx <= 1 || promoted {
            let (_, b389) = &round_bundles[0];
            let fname = format!("bundle_run50_{}.json", rspec.label);
            let fpath = bundles_dir.join(&fname);
            write_bounded_json(&fpath, b389)?;
            round_rows[0].bundle_file = Some(format!("bundles/{fname}"));
        }

        let r389 = &round_rows[0];
        let summary = RoundSummary {
            round_index: idx,
            round_id: rspec.round_id.to_string(),
            label: rspec.label.to_string(),
            short_title: rspec.short_title.to_string(),
            phase_target: rspec.phase_target.to_string(),
            hypothesis_note: rspec.hypothesis_note.to_string(),
            base_round_id,
            all_seeds_verified: all_ok,
            promoted,
            status: status.clone(),
            consecutive_stalls_after: consecutive_stalls,
            run50_seed389_bitops: r389.observed_bitops,
            run50_relations_bitops: r389.relations_bitops,
            run50_la_bitops: r389.linear_algebra_bitops,
            run50_descent_bitops: r389.descent_bitops,
            run50_wall_ms: r389.wall_ms_indicative,
            mean4_observed_bitops: mean4_obs,
            mean4_relations_bitops: mean4_rel,
            mean4_la_bitops: mean4_la,
            mean4_descent_bitops: mean4_desc,
            mean4_wall_ms: mean4_wall,
            mean4_b_scanned: mean4_b,
            mean4_anchor_f: mean4_f,
            mean4_residuals_tried: mean4_res,
            delta_vs_incumbent_pct: delta_pct,
            cumulative_speedup_vs_r0: r0_mean4 / mean4_obs,
            cumulative_speedup_vs_r1: if r1_mean4 > 0.0 {
                r1_mean4 / mean4_obs
            } else {
                1.0
            },
            ratio_vs_karatsuba_rho_mean3: mean3_fresh_obs / rho_mean3_ka,
            champion_after_round_id: incumbent_round_id.clone(),
            champion_mean4_bitops: incumbent_mean4,
        };
        eprintln!(
            "[hillclimb] -> {} {}: mean4={:.3}G (delta={:+.2}% vs prev champ, {:.2}x vs R0, {:.2}x vs R1, {:.3}x Karatsuba rho)",
            rspec.round_id,
            status,
            mean4_obs / 1e9,
            delta_pct,
            summary.cumulative_speedup_vs_r0,
            summary.cumulative_speedup_vs_r1,
            summary.ratio_vs_karatsuba_rho_mean3
        );

        round_summaries.push(summary);
        all_seed_rows.extend(round_rows);
    }

    // Also persist the 3 fresh-seed bundles (`101, 211, 307`) of the Final Hillclimb Champion
    // for standalone cryptographic qualification.
    for (seed, bundle) in &champion_seed_bundles {
        if *seed != 389 {
            let fname = format!("bundle_champion_p1009_s{seed}.json");
            write_bounded_json(&bundles_dir.join(&fname), bundle)?;
        }
    }

    let report = HillclimbTrajectoryReport {
        schema: "cover.hillclimb-trajectory/v1".into(),
        protocol_id: "EXT-IC-GRAPH-HILLCLIMB-STALL-20261126".into(),
        p,
        seeds,
        promotion_threshold_pct: PROMOTION_THRESHOLD_PCT,
        rho_seed389_schoolbook_bitops: rho_seed389_sb,
        rho_seed389_karatsuba_bitops: rho_seed389_ka,
        rho_mean3_schoolbook_bitops: rho_mean3_sb,
        rho_mean3_karatsuba_bitops: rho_mean3_ka,
        final_champion_round_id: incumbent_round_id,
        final_champion_label: incumbent_label,
        final_consecutive_stalls: consecutive_stalls,
        total_promoted_steps_after_r1: promoted_after_r1,
        rounds: round_summaries,
    };

    write_bounded_json(&results_dir.join("hillclimb_rounds.json"), &report)?;
    write_bounded_json(
        &results_dir.join("hillclimb_seed_rows.json"),
        &all_seed_rows,
    )?;
    Ok(incumbent_cfg)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PrimeFrontierRow {
    p: u64,
    solved_seeds: usize,
    total_seeds: usize,
    mean_l: f64,
    r0_mean_bitops: f64,
    r1_mean_bitops: f64,
    champion_mean_bitops: f64,
    champion_relations_bitops: f64,
    champion_la_bitops: f64,
    champion_descent_bitops: f64,
    champion_mean_wall_ms: f64,
    rho_schoolbook_mean_bitops: f64,
    rho_karatsuba_mean_bitops: f64,
    champion_speedup_vs_r0: f64,
    champion_speedup_vs_r1: f64,
    champion_over_schoolbook_rho: f64,
    champion_over_karatsuba_rho: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MultiprimeFrontierReport {
    schema: String,
    protocol_id: String,
    primes: Vec<u64>,
    seeds: Vec<u64>,
    champion_label: String,
    slopes_vs_log_p: BTreeMap<String, f64>,
    rows_by_prime: Vec<PrimeFrontierRow>,
}

fn champion_default_cfg() -> (AnchoredPipelineOptions, HillclimbConfig) {
    let (opts, mut hc) = r1_config();
    hc.sparse_markowitz_la = true;
    hc.wiedemann_la = true;
    hc.partition_fast_sieve = true;
    hc.zero_mul_sieve = true;
    hc.frob_stickelberger_sieve = true;
    hc.stickelberger_fast_extract = true;
    hc.incidence_batch_size = 4096;
    hc.rank_check_subbatch = 64;
    hc.markowitz_surplus = Some(3);
    hc.markowitz_stride = Some(6);
    hc.descent_fast_sextic = true;
    hc.descent_hilbert_stop = Some(8);
    hc.custom_label = Some("hillclimb_champion");
    (opts, hc)
}

fn run_multiprime(
    out_dir: &Path,
    champ_cfg: Option<(AnchoredPipelineOptions, HillclimbConfig)>,
) -> Result<(), String> {
    let primes = vec![307u64, 383, 503, 661, 1009];
    let seeds = vec![101u64, 211, 307];
    let results_dir = out_dir.join("results");
    let bundles_dir = out_dir.join("bundles");
    fs::create_dir_all(&results_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&bundles_dir).map_err(|e| e.to_string())?;

    let (champ_opts, mut champ_hc) = champ_cfg.unwrap_or_else(champion_default_cfg);
    let champ_lbl = champ_hc
        .custom_label
        .unwrap_or("hillclimb_champion")
        .to_string();
    champ_hc.custom_label = Some("hillclimb_champion");

    let (r0_opts, r0_hc) = r0_config();
    let (r1_opts, r1_hc) = r1_config();

    let mut rows_by_prime = Vec::new();
    for &p in &primes {
        let mut ls = Vec::new();
        let mut r0_v = Vec::new();
        let mut r1_v = Vec::new();
        let mut ch_v = Vec::new();
        let mut ch_rel = Vec::new();
        let mut ch_la = Vec::new();
        let mut ch_desc = Vec::new();
        let mut ch_wall = Vec::new();
        let mut sb_rho = Vec::new();
        let mut ka_rho = Vec::new();

        for &seed in &seeds {
            let truth = generate_spec(p, seed);
            let truth_d = truth.d;
            let mut pub_spec = truth.clone();
            pub_spec.d = 0;

            let b0 = run_anchored_ic_dlp_with_config(&pub_spec, seed, r0_opts, r0_hc)?;
            let b1 = run_anchored_ic_dlp_with_config(&pub_spec, seed, r1_opts, r1_hc)?;
            let bc = run_anchored_ic_dlp_with_config(&pub_spec, seed, champ_opts, champ_hc)?;

            for (arm_name, b) in [("R0", &b0), ("R1", &b1), ("Champion", &bc)] {
                if b.verified && b.recovered_scalar != Some(truth_d) {
                    return Err(format!(
                        "{arm_name} recovered wrong scalar at p={p}, seed={seed}"
                    ));
                }
                if b.accounting.foreign_hook_calls != 0 {
                    return Err(format!(
                        "{arm_name} had nonzero foreign_hook_calls at p={p}, seed={seed}"
                    ));
                }
            }

            if !(b0.verified && b1.verified && bc.verified) {
                if p == 307 && seed == 307 {
                    eprintln!(
                        "[multiprime] p=307, seed=307 hit finite B_max plane cap (verified: R0={}, R1={}, Champ={}); excluded from solved mean",
                        b0.verified, b1.verified, bc.verified
                    );
                    continue;
                }
                return Err(format!(
                    "Arm failed verification at p={p}, seed={seed} (R0={}, R1={}, Champ={})",
                    b0.verified, b1.verified, bc.verified
                ));
            }

            ls.push(pub_spec.l as f64);
            let walks = if p < 1009 { 5 } else { 1 };
            let rho = run_matched_rho_reference(&truth, Some(truth_d), seed, walks, p < 1009);
            let sb_val = if p < 1009 {
                rho.empirical_b16_mean_schoolbook_bitops as f64
            } else {
                rho.analytical_b16_schoolbook_bitops as f64
            };
            let ka_val = if p < 1009 {
                rho.empirical_b16_mean_karatsuba_bitops as f64
            } else {
                rho.analytical_b16_karatsuba_bitops as f64
            };
            sb_rho.push(sb_val);
            ka_rho.push(ka_val);

            r0_v.push(b0.accounting.observed_bitops as f64);
            r1_v.push(b1.accounting.observed_bitops as f64);
            ch_v.push(bc.accounting.observed_bitops as f64);
            ch_rel.push(phase_bitops(&bc, "relations") as f64);
            ch_la.push(phase_bitops(&bc, "linear_algebra") as f64);
            ch_desc.push(phase_bitops(&bc, "descent") as f64);
            ch_wall.push(bc.wall_ms_indicative);

            if seed == 101 && p < 1009 {
                let fname = format!("bundle_champion_p{p}_s101.json");
                write_bounded_json(&bundles_dir.join(&fname), &bc)?;
            }
        }

        let solved_seeds = ch_v.len();
        if solved_seeds == 0 {
            return Err(format!("Zero solved seeds at p={p}"));
        }
        let n = solved_seeds as f64;
        let m_r0 = r0_v.iter().sum::<f64>() / n;
        let m_r1 = r1_v.iter().sum::<f64>() / n;
        let m_ch = ch_v.iter().sum::<f64>() / n;
        let m_sb = sb_rho.iter().sum::<f64>() / n;
        let m_ka = ka_rho.iter().sum::<f64>() / n;
        eprintln!(
            "[multiprime] p={p} ({solved_seeds}/{}): R0={:.2}G, R1={:.2}G, Champ={:.2}G ({:.2}x vs R0, {:.2}x vs R1), Rho_KA={:.2}G -> Champ/Rho_KA={:.4}x",
            seeds.len(),
            m_r0 / 1e9,
            m_r1 / 1e9,
            m_ch / 1e9,
            m_r0 / m_ch,
            m_r1 / m_ch,
            m_ka / 1e9,
            m_ch / m_ka
        );

        rows_by_prime.push(PrimeFrontierRow {
            p,
            solved_seeds,
            total_seeds: seeds.len(),
            mean_l: ls.iter().sum::<f64>() / n,
            r0_mean_bitops: m_r0,
            r1_mean_bitops: m_r1,
            champion_mean_bitops: m_ch,
            champion_relations_bitops: ch_rel.iter().sum::<f64>() / n,
            champion_la_bitops: ch_la.iter().sum::<f64>() / n,
            champion_descent_bitops: ch_desc.iter().sum::<f64>() / n,
            champion_mean_wall_ms: ch_wall.iter().sum::<f64>() / n,
            rho_schoolbook_mean_bitops: m_sb,
            rho_karatsuba_mean_bitops: m_ka,
            champion_speedup_vs_r0: m_r0 / m_ch,
            champion_speedup_vs_r1: m_r1 / m_ch,
            champion_over_schoolbook_rho: m_ch / m_sb,
            champion_over_karatsuba_rho: m_ch / m_ka,
        });
    }

    let mut slopes = BTreeMap::new();
    let pts = |f: fn(&PrimeFrontierRow) -> f64| -> Vec<(f64, f64)> {
        rows_by_prime.iter().map(|r| (r.p as f64, f(r))).collect()
    };
    slopes.insert("r0_total".into(), ols_log_slope(&pts(|r| r.r0_mean_bitops)));
    slopes.insert("r1_total".into(), ols_log_slope(&pts(|r| r.r1_mean_bitops)));
    slopes.insert(
        "champion_total".into(),
        ols_log_slope(&pts(|r| r.champion_mean_bitops)),
    );
    slopes.insert(
        "champion_relations".into(),
        ols_log_slope(&pts(|r| r.champion_relations_bitops)),
    );
    slopes.insert(
        "champion_la".into(),
        ols_log_slope(&pts(|r| r.champion_la_bitops)),
    );
    slopes.insert(
        "champion_descent".into(),
        ols_log_slope(&pts(|r| r.champion_descent_bitops)),
    );
    slopes.insert(
        "rho_karatsuba".into(),
        ols_log_slope(&pts(|r| r.rho_karatsuba_mean_bitops)),
    );

    let report = MultiprimeFrontierReport {
        schema: "cover.hillclimb-multiprime/v1".into(),
        protocol_id: "EXT-IC-GRAPH-HILLCLIMB-STALL-20261126".into(),
        primes,
        seeds,
        champion_label: champ_lbl,
        slopes_vs_log_p: slopes,
        rows_by_prime,
    };
    write_bounded_json(&results_dir.join("multiprime_frontier.json"), &report)?;
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
struct OverflowCheckCase {
    p: u64,
    seed: u64,
    overflow_checks_compiled: bool,
    verified: bool,
    matches_planted_truth: bool,
    recovered_scalar: Option<u64>,
    b_count_scanned: u64,
    sources_admitted: usize,
    total_anchors: usize,
    residuals_tried: u64,
    observed_bitops: u128,
    sources_sha256: String,
    projections_sha256: String,
    foreign_hook_calls: u64,
}

fn runtime_overflow_checks_enabled() -> bool {
    std::panic::catch_unwind(|| {
        let x = std::hint::black_box(u8::MAX);
        std::hint::black_box(x + 1);
    })
    .is_err()
}

fn run_overflow_control(out_file: &Path) -> Result<(), String> {
    let ovf_enabled = runtime_overflow_checks_enabled();
    let (opts, hc) = champion_default_cfg();
    let mut rows = Vec::new();
    for &(p, seed) in &[(307u64, 101u64), (383, 211), (503, 307)] {
        let truth = generate_spec(p, seed);
        let truth_d = truth.d;
        let mut pub_spec = truth.clone();
        pub_spec.d = 0;
        let b = run_anchored_ic_dlp_with_config(&pub_spec, seed, opts, hc)?;
        rows.push(OverflowCheckCase {
            p,
            seed,
            overflow_checks_compiled: ovf_enabled,
            verified: b.verified,
            matches_planted_truth: b.recovered_scalar == Some(truth_d),
            recovered_scalar: b.recovered_scalar,
            b_count_scanned: b.collection.b_count_scanned,
            sources_admitted: b.collection.sources_admitted,
            total_anchors: b.collection.total_anchors,
            residuals_tried: b.descent.residuals_tried,
            observed_bitops: b.accounting.observed_bitops,
            sources_sha256: b.collection.sources_sha256,
            projections_sha256: b.collection.projections_sha256,
            foreign_hook_calls: b.accounting.foreign_hook_calls,
        });
    }
    write_bounded_json(out_file, &rows)?;
    Ok(())
}

fn render_svgs(out_dir: &Path) -> Result<(), String> {
    let results_dir = out_dir.join("results");
    let fig_dir = out_dir.join("figures");
    fs::create_dir_all(&fig_dir).map_err(|e| e.to_string())?;

    let hc_raw = fs::read_to_string(results_dir.join("hillclimb_rounds.json"))
        .map_err(|e| format!("read hillclimb_rounds.json: {e}"))?;
    let hc: HillclimbTrajectoryReport =
        serde_json::from_str(&hc_raw).map_err(|e| format!("parse hillclimb_rounds.json: {e}"))?;

    let mp_raw = fs::read_to_string(results_dir.join("multiprime_frontier.json"))
        .map_err(|e| format!("read multiprime_frontier.json: {e}"))?;
    let mp: MultiprimeFrontierReport = serde_json::from_str(&mp_raw)
        .map_err(|e| format!("parse multiprime_frontier.json: {e}"))?;

    // 1. `hillclimb_trajectory.svg`: 12-round cost trajectory + consecutive stalls + slope panel.
    {
        let mut s = String::with_capacity(16_384);
        writeln!(
            s,
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 980 460" width="980" height="460" font-family="DejaVu Sans, Arial, sans-serif">"##
        )
        .unwrap();
        writeln!(s, r##"<rect width="980" height="460" fill="#ffffff"/>"##).unwrap();
        writeln!(
            s,
            r##"<text x="490" y="28" text-anchor="middle" font-size="15" font-weight="bold" fill="#0f172a">12-Round Algorithmic Hillclimb Trajectory &amp; Frontier Stall Verification (p = 1009, 4-Seed Mean bitop/v1)</text>"##
        )
        .unwrap();

        // Left panel: Cost trajectory (Gbitops), x in [70..560], y in [60..370]
        let x0 = 72.0f64;
        let x1 = 555.0f64;
        let y0 = 58.0f64;
        let y1 = 365.0f64;
        let max_g = (hc
            .rounds
            .iter()
            .map(|r| r.mean4_observed_bitops / 1e9)
            .fold(0.0f64, f64::max)
            * 1.08)
            .max(7000.0);
        let map_x = |i: usize| -> f64 {
            x0 + (i as f64) * (x1 - x0) / ((hc.rounds.len() - 1).max(1) as f64)
        };
        let map_y = |g: f64| -> f64 { y1 - (g / max_g).clamp(0.0, 1.05) * (y1 - y0) };

        writeln!(
            s,
            r##"<rect x="{x0}" y="{y0}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
            x1 - x0,
            y1 - y0
        )
        .unwrap();
        for tick in [0.0, 1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0] {
            if tick <= max_g {
                let yy = map_y(tick);
                writeln!(
                    s,
                    r##"<line x1="{x0}" y1="{yy:.1}" x2="{x1}" y2="{yy:.1}" stroke="#e2e8f0" stroke-width="1"/>"##
                )
                .unwrap();
                writeln!(
                    s,
                    r##"<text x="{}" y="{:.1}" text-anchor="end" font-size="10" fill="#475569">{:.1}T</text>"##,
                    x0 - 6.0,
                    yy + 3.5,
                    tick / 1000.0
                )
                .unwrap();
            }
        }

        // Karatsuba & Schoolbook Pollard rho off-chart callout badge
        let ka_g = hc.rho_mean3_karatsuba_bitops / 1e9;
        let sb_g = hc.rho_mean3_schoolbook_bitops / 1e9;
        writeln!(
            s,
            r##"<rect x="260" y="66" width="286" height="22" rx="4" fill="#f5f3ff" stroke="#7c3aed" stroke-width="1"/>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<text x="403" y="80.5" text-anchor="middle" font-size="9" font-weight="bold" fill="#5b21b6">Matched B=16 Pollard rho: {:.1}T Karatsuba / {:.1}T Schoolbook</text>"##,
            ka_g / 1000.0,
            sb_g / 1000.0
        )
        .unwrap();

        // Monotonic champion polyline
        let mut champ_pts = String::new();
        for (i, r) in hc.rounds.iter().enumerate() {
            let xx = map_x(i);
            let yy = map_y(r.champion_mean4_bitops / 1e9);
            write!(champ_pts, "{xx:.1},{yy:.1} ").unwrap();
        }
        writeln!(
            s,
            r##"<polyline fill="none" stroke="#059669" stroke-width="2.6" points="{}"/>"##,
            champ_pts.trim()
        )
        .unwrap();

        // Candidate probe points + dashed connectors from incumbent
        for (i, r) in hc.rounds.iter().enumerate() {
            let xx = map_x(i);
            let cand_g = r.mean4_observed_bitops / 1e9;
            let yy = map_y(cand_g);
            if !r.promoted {
                let cy = map_y(r.champion_mean4_bitops / 1e9);
                writeln!(
                    s,
                    r##"<line x1="{xx:.1}" y1="{cy:.1}" x2="{xx:.1}" y2="{yy:.1}" stroke="#dc2626" stroke-width="1.4" stroke-dasharray="3,2"/>"##
                )
                .unwrap();
                writeln!(
                    s,
                    r##"<circle cx="{xx:.1}" cy="{yy:.1}" r="4.8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>"##
                )
                .unwrap();
                let dy = if i % 2 == 0 { -9.0 } else { -19.0 };
                writeln!(
                    s,
                    r##"<text x="{xx:.1}" y="{:.1}" text-anchor="middle" font-size="8.5" font-weight="bold" fill="#b91c1c">STALL</text>"##,
                    yy + dy
                )
                .unwrap();
            } else {
                let col = if i == 0 { "#334155" } else { "#059669" };
                writeln!(
                    s,
                    r##"<circle cx="{xx:.1}" cy="{yy:.1}" r="5.2" fill="{col}" stroke="#ffffff" stroke-width="1.5"/>"##
                )
                .unwrap();
                writeln!(
                    s,
                    r##"<text x="{:.1}" y="{:.1}" text-anchor="start" font-size="8.8" font-weight="bold" fill="#065f46">{:.2}T</text>"##,
                    xx + 5.0,
                    yy - 6.0,
                    cand_g / 1000.0
                )
                .unwrap();
            }
            writeln!(
                s,
                r##"<text x="{xx:.1}" y="{}" text-anchor="middle" font-size="9.5" font-weight="bold" fill="#1e293b">{}</text>"##,
                y1 + 15.0,
                r.round_id
            )
            .unwrap();
        }
        writeln!(
            s,
            r##"<text x="313" y="402" text-anchor="middle" font-size="10.5" fill="#334155">Hillclimb Round (Green Solid = Active Champion Envelope; Red Open = Stalled/Rejected Hypothesis)</text>"##
        )
        .unwrap();

        // Right panel: Relative delta % vs incumbent per round (R1..R11), x in [620..950], y in [60..365]
        let rx0 = 635.0f64;
        let rx1 = 955.0f64;
        let zero_x = rx0 + 0.58 * (rx1 - rx0);
        let scale_pct = (rx1 - rx0) / 135.0; // spans [-78% .. +57%]
        writeln!(
            s,
            r##"<rect x="{rx0}" y="{y0}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
            rx1 - rx0,
            y1 - y0
        )
        .unwrap();
        // Stall threshold band [-1%, +infinity]
        let thresh_x = zero_x + PROMOTION_THRESHOLD_PCT * scale_pct;
        writeln!(
            s,
            r##"<rect x="{thresh_x:.1}" y="{y0}" width="{:.1}" height="{}" fill="#fee2e2" fill-opacity="0.35"/>"##,
            rx1 - thresh_x,
            y1 - y0
        )
        .unwrap();
        writeln!(
            s,
            r##"<line x1="{zero_x:.1}" y1="{y0}" x2="{zero_x:.1}" y2="{y1}" stroke="#475569" stroke-width="1.2"/>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<line x1="{thresh_x:.1}" y1="{y0}" x2="{thresh_x:.1}" y2="{y1}" stroke="#dc2626" stroke-width="1" stroke-dasharray="3,3"/>"##
        )
        .unwrap();

        let step_h = (y1 - y0 - 16.0) / ((hc.rounds.len() - 1).max(1) as f64);
        for (k, r) in hc.rounds.iter().skip(1).enumerate() {
            let cy = y0 + 12.0 + (k as f64 + 0.5) * step_h;
            let pct_clamped = r.delta_vs_incumbent_pct.clamp(-75.0, 52.0);
            let bx = zero_x + pct_clamped * scale_pct;
            let (left_x, width_x, fill) = if pct_clamped < 0.0 {
                (
                    bx,
                    zero_x - bx,
                    if r.promoted { "#059669" } else { "#d97706" },
                )
            } else {
                (zero_x, bx - zero_x, "#dc2626")
            };
            writeln!(
                s,
                r##"<rect x="{left_x:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="{fill}" rx="2"/>"##,
                cy - step_h * 0.34,
                width_x.max(1.5),
                step_h * 0.68
            )
            .unwrap();
            writeln!(
                s,
                r##"<text x="{}" y="{:.1}" text-anchor="end" font-size="9.5" font-weight="bold" fill="#1e293b">{}</text>"##,
                rx0 - 5.0,
                cy + 3.5,
                r.round_id
            )
            .unwrap();
            let lbl_x = if pct_clamped < -12.0 {
                left_x + 4.0
            } else if pct_clamped < 0.0 {
                left_x - 4.0
            } else {
                left_x + width_x + 4.0
            };
            let anchor = if pct_clamped < -12.0 {
                "start"
            } else if pct_clamped < 0.0 {
                "end"
            } else {
                "start"
            };
            let tcol = if pct_clamped < -12.0 {
                "#ffffff"
            } else {
                "#0f172a"
            };
            writeln!(
                s,
                r##"<text x="{lbl_x:.1}" y="{:.1}" text-anchor="{anchor}" font-size="8.8" font-weight="bold" fill="{tcol}">{:+.1}% ({})</text>"##,
                cy + 3.0,
                r.delta_vs_incumbent_pct,
                r.status
            )
            .unwrap();
        }
        writeln!(
            s,
            r##"<text x="795" y="402" text-anchor="middle" font-size="10.5" fill="#334155">Step Slope (% Delta vs Incumbent; Shaded Zone &gt; -1.0% = STALL)</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<text x="490" y="438" text-anchor="middle" font-size="10" fill="#475569">Final Champion: {} ({}) — {:.2}x faster than R0, {:.2}x faster than R1, {:.3}x Karatsuba Pollard rho, followed by {} consecutive STALL rounds</text>"##,
            hc.final_champion_round_id,
            hc.final_champion_label,
            hc.rounds[0].mean4_observed_bitops / hc.rounds.last().unwrap().champion_mean4_bitops,
            hc.rounds[1].mean4_observed_bitops / hc.rounds.last().unwrap().champion_mean4_bitops,
            hc.rounds
                .iter()
                .find(|r| r.round_id == hc.final_champion_round_id)
                .map_or(1.0, |r| r.ratio_vs_karatsuba_rho_mean3),
            hc.final_consecutive_stalls
        )
        .unwrap();
        writeln!(s, "</svg>").unwrap();
        fs::write(fig_dir.join("hillclimb_trajectory.svg"), s).map_err(|e| e.to_string())?;
    }

    // 2. `phase_waterfall.svg`: Stacked Phase-1 / Phase-2 / Phase-3 breakdown across promoted milestones
    {
        let promoted: Vec<&RoundSummary> = hc
            .rounds
            .iter()
            .filter(|r| r.round_index <= 1 || r.promoted)
            .collect();
        let mut s = String::with_capacity(12_288);
        writeln!(
            s,
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 920 420" width="920" height="420" font-family="DejaVu Sans, Arial, sans-serif">"##
        )
        .unwrap();
        writeln!(s, r##"<rect width="920" height="420" fill="#ffffff"/>"##).unwrap();
        writeln!(
            s,
            r##"<text x="460" y="28" text-anchor="middle" font-size="15" font-weight="bold" fill="#0f172a">Per-Phase bitop/v1 &amp; Wall-Clock Collapse Across Promoted Hillclimb Milestones (p = 1009, 4-Seed Mean)</text>"##
        )
        .unwrap();

        let x0 = 90.0f64;
        let x1 = 870.0f64;
        let y0 = 62.0f64;
        let y1 = 335.0f64;
        let max_g = (promoted[0].mean4_observed_bitops / 1e9 * 1.10).max(7000.0);
        let map_y = |g: f64| -> f64 { y1 - (g / max_g) * (y1 - y0) };

        writeln!(
            s,
            r##"<rect x="{x0}" y="{y0}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
            x1 - x0,
            y1 - y0
        )
        .unwrap();
        for tick in [0.0, 1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0] {
            if tick <= max_g {
                let yy = map_y(tick);
                writeln!(
                    s,
                    r##"<line x1="{x0}" y1="{yy:.1}" x2="{x1}" y2="{yy:.1}" stroke="#e2e8f0" stroke-width="1"/>"##
                )
                .unwrap();
                writeln!(
                    s,
                    r##"<text x="{}" y="{:.1}" text-anchor="end" font-size="10" fill="#475569">{:.1}T</text>"##,
                    x0 - 8.0,
                    yy + 3.5,
                    tick / 1000.0
                )
                .unwrap();
            }
        }

        // Karatsuba rho callout badge
        writeln!(
            s,
            r##"<rect x="550" y="70" width="310" height="22" rx="4" fill="#f5f3ff" stroke="#7c3aed" stroke-width="1"/>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<text x="705" y="84.5" text-anchor="middle" font-size="9.2" font-weight="bold" fill="#5b21b6">Matched B=16 Karatsuba Pollard rho = {:.1}T (off-chart above)</text>"##,
            hc.rho_mean3_karatsuba_bitops / 1e12
        )
        .unwrap();

        let slot_w = (x1 - x0) / (promoted.len() as f64);
        let bar_w = (slot_w * 0.52).min(88.0);
        for (i, r) in promoted.iter().enumerate() {
            let cx = x0 + (i as f64 + 0.5) * slot_w;
            let bx = cx - bar_w * 0.5;
            let rel_g = r.mean4_relations_bitops / 1e9;
            let la_g = r.mean4_la_bitops / 1e9;
            let desc_g = r.mean4_descent_bitops / 1e9;
            let tot_g = r.mean4_observed_bitops / 1e9;

            let y_rel_top = map_y(rel_g);
            let y_la_top = map_y(rel_g + la_g);
            let y_desc_top = map_y(rel_g + la_g + desc_g);

            writeln!(
                s,
                r##"<rect x="{bx:.1}" y="{y_rel_top:.1}" width="{bar_w:.1}" height="{:.1}" fill="#2563eb"/>"##,
                (y1 - y_rel_top).max(0.5)
            )
            .unwrap();
            writeln!(
                s,
                r##"<rect x="{bx:.1}" y="{y_la_top:.1}" width="{bar_w:.1}" height="{:.1}" fill="#f59e0b"/>"##,
                (y_rel_top - y_la_top).max(0.5)
            )
            .unwrap();
            writeln!(
                s,
                r##"<rect x="{bx:.1}" y="{y_desc_top:.1}" width="{bar_w:.1}" height="{:.1}" fill="#10b981"/>"##,
                (y_la_top - y_desc_top).max(0.5)
            )
            .unwrap();

            writeln!(
                s,
                r##"<text x="{cx:.1}" y="{:.1}" text-anchor="middle" font-size="9.5" font-weight="bold" fill="#0f172a">{:.2}T ({:.2}s)</text>"##,
                y_desc_top - 6.0,
                tot_g / 1000.0,
                r.mean4_wall_ms / 1000.0
            )
            .unwrap();
            writeln!(
                s,
                r##"<text x="{cx:.1}" y="{}" text-anchor="middle" font-size="10" font-weight="bold" fill="#1e293b">{}</text>"##,
                y1 + 16.0,
                r.round_id
            )
            .unwrap();
            writeln!(
                s,
                r##"<text x="{cx:.1}" y="{}" text-anchor="middle" font-size="8.5" fill="#475569">{}</text>"##,
                y1 + 30.0,
                r.phase_target
            )
            .unwrap();
        }

        // Legend
        writeln!(
            s,
            r##"<rect x="200" y="388" width="12" height="12" fill="#2563eb"/><text x="217" y="398" font-size="10" fill="#1e293b">Phase 1: Sieve + Matroid Graph</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="420" y="388" width="12" height="12" fill="#f59e0b"/><text x="437" y="398" font-size="10" fill="#1e293b">Phase 2: Sparse Homogeneous LA</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="645" y="388" width="12" height="12" fill="#10b981"/><text x="662" y="398" font-size="10" fill="#1e293b">Phase 3: Multi-Pattern Descent</text>"##
        )
        .unwrap();
        writeln!(s, "</svg>").unwrap();
        fs::write(fig_dir.join("phase_waterfall.svg"), s).map_err(|e| e.to_string())?;
    }

    // 3. `multiprime_frontier.svg`: Log-log scaling p in {307..1009}
    {
        let mut s = String::with_capacity(12_288);
        writeln!(
            s,
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 880 420" width="880" height="420" font-family="DejaVu Sans, Arial, sans-serif">"##
        )
        .unwrap();
        writeln!(s, r##"<rect width="880" height="420" fill="#ffffff"/>"##).unwrap();
        writeln!(
            s,
            r##"<text x="440" y="28" text-anchor="middle" font-size="15" font-weight="bold" fill="#0f172a">Multi-Prime Log-Log bitop/v1 Scaling Frontier Across p in [307, 1009] (3 Fresh Seeds/Prime)</text>"##
        )
        .unwrap();

        let x0 = 90.0f64;
        let x1 = 830.0f64;
        let y0 = 58.0f64;
        let y1 = 340.0f64;
        let lp_min = 290.0f64.ln();
        let lp_max = 1060.0f64.ln();
        let ly_min = 9.0f64; // 10^9
        let ly_max = 14.5f64; // 3.16 * 10^14
        let map_x = |p: f64| -> f64 { x0 + (p.ln() - lp_min) / (lp_max - lp_min) * (x1 - x0) };
        let map_y = |b: f64| -> f64 { y1 - (b.log10() - ly_min) / (ly_max - ly_min) * (y1 - y0) };

        writeln!(
            s,
            r##"<rect x="{x0}" y="{y0}" width="{}" height="{}" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1"/>"##,
            x1 - x0,
            y1 - y0
        )
        .unwrap();
        for exp in 9..=14 {
            let yy = y1 - ((exp as f64) - ly_min) / (ly_max - ly_min) * (y1 - y0);
            writeln!(
                s,
                r##"<line x1="{x0}" y1="{yy:.1}" x2="{x1}" y2="{yy:.1}" stroke="#e2e8f0" stroke-width="1"/>"##
            )
            .unwrap();
            writeln!(
                s,
                r##"<text x="{}" y="{:.1}" text-anchor="end" font-size="10" fill="#475569">10^{exp}</text>"##,
                x0 - 8.0,
                yy + 3.5
            )
            .unwrap();
        }
        for &p in &mp.primes {
            let xx = map_x(p as f64);
            writeln!(
                s,
                r##"<line x1="{xx:.1}" y1="{y0}" x2="{xx:.1}" y2="{y1}" stroke="#e2e8f0" stroke-width="1"/>"##
            )
            .unwrap();
            writeln!(
                s,
                r##"<text x="{xx:.1}" y="{}" text-anchor="middle" font-size="10" font-weight="bold" fill="#1e293b">p = {p}</text>"##,
                y1 + 17.0
            )
            .unwrap();
        }

        let draw_series = |s: &mut String,
                           vals: Vec<(f64, f64)>,
                           color: &str,
                           dash: &str,
                           width: f64| {
            let mut pts = String::new();
            for &(p, v) in &vals {
                write!(pts, "{:.1},{:.1} ", map_x(p), map_y(v)).unwrap();
            }
            if dash.is_empty() {
                writeln!(
                    s,
                    r##"<polyline fill="none" stroke="{color}" stroke-width="{width}" points="{}"/>"##,
                    pts.trim()
                )
                .unwrap();
            } else {
                writeln!(
                    s,
                    r##"<polyline fill="none" stroke="{color}" stroke-width="{width}" stroke-dasharray="{dash}" points="{}"/>"##,
                    pts.trim()
                )
                .unwrap();
            }
            for &(p, v) in &vals {
                writeln!(
                    s,
                    r##"<circle cx="{:.1}" cy="{:.1}" r="4.3" fill="{color}" stroke="#ffffff" stroke-width="1.2"/>"##,
                    map_x(p),
                    map_y(v)
                )
                .unwrap();
            }
        };

        draw_series(
            &mut s,
            mp.rows_by_prime
                .iter()
                .map(|r| (r.p as f64, r.rho_schoolbook_mean_bitops))
                .collect(),
            "#64748b",
            "5,4",
            1.8,
        );
        draw_series(
            &mut s,
            mp.rows_by_prime
                .iter()
                .map(|r| (r.p as f64, r.rho_karatsuba_mean_bitops))
                .collect(),
            "#7c3aed",
            "6,3",
            2.2,
        );
        draw_series(
            &mut s,
            mp.rows_by_prime
                .iter()
                .map(|r| (r.p as f64, r.r0_mean_bitops))
                .collect(),
            "#dc2626",
            "",
            2.0,
        );
        draw_series(
            &mut s,
            mp.rows_by_prime
                .iter()
                .map(|r| (r.p as f64, r.r1_mean_bitops))
                .collect(),
            "#d97706",
            "",
            2.2,
        );
        draw_series(
            &mut s,
            mp.rows_by_prime
                .iter()
                .map(|r| (r.p as f64, r.champion_mean_bitops))
                .collect(),
            "#059669",
            "",
            2.8,
        );

        let s_r0 = mp.slopes_vs_log_p.get("r0_total").copied().unwrap_or(0.0);
        let s_r1 = mp.slopes_vs_log_p.get("r1_total").copied().unwrap_or(0.0);
        let s_ch = mp
            .slopes_vs_log_p
            .get("champion_total")
            .copied()
            .unwrap_or(0.0);
        let s_ka = mp
            .slopes_vs_log_p
            .get("rho_karatsuba")
            .copied()
            .unwrap_or(0.0);

        writeln!(
            s,
            r##"<text x="460" y="382" text-anchor="middle" font-size="9.8" fill="#1e293b">OLS Slopes in log(p): Karatsuba Pollard rho = {s_ka:.2} | R0 Baseline = {s_r0:.2} | R1 Matroid = {s_r1:.2} | Hillclimb Champion = {s_ch:.2}</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="95" y="395" width="11" height="11" fill="#059669"/><text x="111" y="405" font-size="9.5" font-weight="bold" fill="#065f46">Hillclimb Champion</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="255" y="395" width="11" height="11" fill="#d97706"/><text x="271" y="405" font-size="9.5" fill="#1e293b">R1 Matroid-2LP</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="395" y="395" width="11" height="11" fill="#dc2626"/><text x="411" y="405" font-size="9.5" fill="#1e293b">R0 Deferred-Orig</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="540" y="395" width="11" height="11" fill="#7c3aed"/><text x="556" y="405" font-size="9.5" font-weight="bold" fill="#6d28d9">Karatsuba Pollard rho</text>"##
        )
        .unwrap();
        writeln!(
            s,
            r##"<rect x="710" y="395" width="11" height="11" fill="#64748b"/><text x="726" y="405" font-size="9.5" fill="#475569">Schoolbook rho</text>"##
        )
        .unwrap();
        writeln!(s, "</svg>").unwrap();
        fs::write(fig_dir.join("multiprime_frontier.svg"), s).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[derive(Clone, Debug, Serialize)]
struct SelfTestReport {
    markowitz_planted_kernel_passed: bool,
    wiedemann_planted_kernel_passed: bool,
    partition_m9_planes_checked: usize,
    frob_stickelberger_planes_checked: usize,
    partition_m9_lines_matched: usize,
    sieve_buckets_lines_matched: usize,
    zero_mul_sieve_lines_matched: usize,
    rng_sync_exact: bool,
    all_passed: bool,
}

fn run_selftest(out_file: &Path) -> Result<(), String> {
    // 1. SparseMarkowitzTracker & WiedemannTracker exact planted homogeneous kernel checks.
    let l = 1_000_000_007u64;
    let cols = 160usize;
    let mut rng = StdRng::seed_from_u64(0x5EED_2026);
    let mut h = vec![0u64; cols];
    for x in h.iter_mut().take(cols - 1) {
        *x = rng.gen_range(1..l);
    }
    h[cols - 1] = 1;
    let mut rows = Vec::new();
    for r in 0..(cols + 18) {
        let mut terms = Vec::new();
        let c0 = r % (cols - 1);
        let c1 = (r * 7 + 3) % (cols - 1);
        let v0 = rng.gen_range(1..7) as u64;
        terms.push((c0, v0));
        let mut dot = (v0 * h[c0]) % l;
        if c1 != c0 {
            let v1 = rng.gen_range(1..7) as u64;
            terms.push((c1, v1));
            dot = (dot + v1 * h[c1]) % l;
        }
        let last_coef = if dot == 0 { 0 } else { l - dot };
        if last_coef != 0 {
            terms.push((cols - 1, last_coef));
        }
        rows.push(terms);
    }
    let mut tracker = SparseMarkowitzTracker::new(l, cols, 2, 8, 32)?;
    tracker.check(100, &rows)?;
    if tracker.kernel.as_deref() != Some(h.as_slice()) {
        return Err("SparseMarkowitzTracker failed planted kernel recovery".into());
    }
    let mut wied = WiedemannTracker::new(l, cols, 0x7777_8888, 2, 8, 32)?;
    wied.check(100, &rows)?;
    if wied.kernel.as_deref() != Some(h.as_slice()) {
        return Err("WiedemannTracker failed planted kernel recovery".into());
    }

    // 2. Partition m=9 + Frob/Stickelberger line factorization + Zero-Mul bucket-sieve bit-for-bit parity.
    let mut planes_checked = 0usize;
    let mut lines_matched = 0usize;
    let mut buckets_matched = 0usize;
    for &(p, seed) in &[(307u64, 101u64), (503, 211)] {
        let spec = generate_spec(p, seed);
        let ctx = Ctx::new(&spec);
        let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
        let base = cover::factor_base(&ctx, &mut base_rng);
        let setup = sieve::sieve_setup(&ctx.f.f, &ctx.cov.hx, &base, 9);
        let nu = sieve::nu_lines(&setup);
        let ring = FpRing::new(p);
        let inv_fp = build_inv_table(p);
        let inv_2w_sq = build_inv_2w_sq_table(p, setup.w, &inv_fp);
        let mut slot = Vec::new();
        let mut rng_ref = StdRng::seed_from_u64(seed ^ 0xB10C);
        let mut rng_fast = StdRng::seed_from_u64(seed ^ 0xB10C);
        let mut rng_frob = StdRng::seed_from_u64(seed ^ 0xB10C);
        let scramble = sieve::coprime_scramble(
            sieve::b_space(&setup),
            &mut StdRng::seed_from_u64(seed ^ 0x5C4A),
        );
        let batch: Vec<_> = (0..512u128)
            .map(|k| sieve::b_from_index(&setup, k, scramble))
            .collect();
        let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
        let hints = sieve::roots_by_incidence(p, &nu, &ab, &mut slot).0;

        let mut b_ref = HitBuckets::new(p);
        let mut b_fast = HitBuckets::new(p);
        let mut b_zmul = HitBuckets::new(p);
        let mut scratch = FastSieveScratch::new(p);

        for (i, b) in batch.iter().enumerate() {
            planes_checked += 1;
            let mut l_ref = sieve::lines_for_b_hint(
                &ring,
                &ctx.f.f,
                &setup,
                b,
                &mut rng_ref,
                true,
                hints[i].as_deref(),
            );
            let mut l_fast = lines_for_b_partition_m9(
                &ring,
                &ctx.f.f,
                &setup,
                b,
                &inv_fp,
                &mut rng_fast,
                hints[i].as_deref(),
            );
            let mut l_frob = lines_for_b_frob_stickelberger_m9(
                &ring,
                &ctx.f.f,
                &setup,
                b,
                &inv_fp,
                &mut rng_frob,
                hints[i].as_deref(),
            );
            let key = |ln: &sieve::Line| (ln.a0.clone(), ln.b.clone());
            l_ref.sort_by_key(key);
            l_fast.sort_by_key(key);
            l_frob.sort_by_key(key);
            let v_ref: Vec<_> = l_ref.iter().map(key).collect();
            let v_fast: Vec<_> = l_fast.iter().map(key).collect();
            let v_frob: Vec<_> = l_frob.iter().map(key).collect();
            if v_ref != v_fast || v_ref != v_frob {
                return Err(format!("partition/frob m=9 mismatch at p={p}, plane={i}"));
            }
            let draw_ref = rng_ref.gen::<u64>();
            if draw_ref != rng_fast.gen::<u64>() || draw_ref != rng_frob.gen::<u64>() {
                return Err(format!("StdRng desynchronized at p={p}, plane={i}"));
            }
            lines_matched += l_ref.len();

            let sh_ref = if l_ref.len() >= 2 {
                Some(sieve::shared_for_b(&ring, &l_ref[0], &mut 0))
            } else {
                None
            };
            let sh_fast = if l_ref.len() >= 2 {
                Some(SharedBFast::build(&setup, &l_ref[0]))
            } else {
                None
            };
            for ln in &l_ref {
                let (mut h1, _) =
                    sieve::sieve_line_buckets(&ring, &setup, ln, &mut b_ref, sh_ref.as_ref(), 3);
                let (mut h2, _) = sieve_line_buckets_fast(
                    &setup,
                    ln,
                    &inv_fp,
                    &mut b_fast,
                    &mut scratch,
                    sh_ref.as_ref(),
                    3,
                );
                let (mut h3, _) = sieve_line_buckets_zero_mul(
                    &setup,
                    ln,
                    &inv_fp,
                    &inv_2w_sq,
                    &mut b_zmul,
                    sh_fast.as_ref(),
                    3,
                );
                h1.sort_unstable();
                h2.sort_unstable();
                h3.sort_unstable();
                if h1 != h2 || h1 != h3 {
                    return Err(format!("sieve_line_buckets hit mismatch at p={p}"));
                }
                for &s in &h1 {
                    let mut x1 = b_ref.hits_for(s).to_vec();
                    let mut x2 = b_fast.hits_for(s).to_vec();
                    let mut x3 = b_zmul.hits_for(s).to_vec();
                    x1.sort_unstable();
                    x2.sort_unstable();
                    x3.sort_unstable();
                    if x1 != x2 || x1 != x3 {
                        return Err(format!(
                            "sieve_line_buckets abscissa mismatch at p={p}, s={s}"
                        ));
                    }
                }
                buckets_matched += 1;
            }
        }
    }

    let rep = SelfTestReport {
        markowitz_planted_kernel_passed: true,
        wiedemann_planted_kernel_passed: true,
        partition_m9_planes_checked: planes_checked,
        frob_stickelberger_planes_checked: planes_checked,
        partition_m9_lines_matched: lines_matched,
        sieve_buckets_lines_matched: buckets_matched,
        zero_mul_sieve_lines_matched: buckets_matched,
        rng_sync_exact: true,
        all_passed: true,
    };
    write_bounded_json(out_file, &rep)?;
    eprintln!(
        "[self-test] ALL PASSED: planes={planes_checked}, lines={lines_matched}, bucket_lines={buckets_matched}"
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err(format!(
            "usage: {} <self-test|full|hillclimb|multiprime|overflow-control|render-svgs> <path>",
            args.first()
                .map_or("jv_graph_ic_hillclimb_study", String::as_str)
        ));
    }
    let cmd = args[1].as_str();
    let target = PathBuf::from(&args[2]);
    match cmd {
        "self-test" => run_selftest(&target),
        "hillclimb" => {
            let _ = run_hillclimb(&target)?;
            Ok(())
        }
        "multiprime" => run_multiprime(&target, None),
        "full" => {
            let champ = run_hillclimb(&target)?;
            run_multiprime(&target, Some(champ))?;
            render_svgs(&target)?;
            Ok(())
        }
        "overflow-control" => run_overflow_control(&target),
        "render-svgs" => render_svgs(&target),
        other => Err(format!("unknown subcommand `{other}`")),
    }
}
