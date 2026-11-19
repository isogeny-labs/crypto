use std::path::PathBuf;
use std::time::Instant;

use prime_pkm_aux_chase_20261119::{
    p0_1c_bench, p1_3a_spec_ladder, p2_5_p256_aux, panel_a_scaling, FullChaseResults,
};

fn main() {
    let out_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("RESULTS.json"));

    let t_start = Instant::now();
    println!("=== Workstream 2: Prime PKM Semaev / T2-Torus Acceleration & P-256 Aux Chase ===");

    println!("\n[1/4] Running Lead P0-1c (ParamS5, Prime-Algebraic Rungs 12..20, M17/M31 Dickson Torus)...");
    let lead_p0_1c = p0_1c_bench::run_all_p0_1c();
    for r in &lead_p0_1c.param_s5_audit {
        println!(
            "  ParamS{} ({}, m={}): setup={} mults, spec={} mults vs grid={} ({:.1}x), mismatches={}",
            r.m + 1,
            r.field_label,
            r.m,
            r.setup_fp_mults_once,
            r.per_target_spec_fp_mults,
            r.grid_interp_fp_mults_baseline,
            r.spec_speedup_x,
            r.mismatches
        );
    }
    for r in &lead_p0_1c.algebraic_rungs {
        println!(
            "  Rung k={} ({}, {}): verified={}, ops {:.2}x ({} -> {}), S(0.1) {:.2}x ({:.2} -> {:.2}, rho={:.2}), wall {:.2}x ({:.1}ms -> {:.1}ms)",
            r.bits,
            r.slug,
            r.oracle_label,
            r.verified_dlog,
            r.solver_ops_speedup_x,
            r.before_solver_ops,
            r.after_solver_ops,
            r.priced_s_w01_speedup_x,
            r.before_s_w01,
            r.after_s_w01,
            r.rho_s_w01,
            r.wall_speedup_x,
            r.before_wall_ms,
            r.after_wall_ms
        );
    }
    for r in &lead_p0_1c.dickson_torus_solves {
        println!(
            "  {} (m={}, t={}, mode={}): verified={}, ops/call {:.2}x ({:.0} -> {:.0}), S(0.1) {:.2}x ({:.2e} -> {:.2e}, rho={:.2}), wall {:.2}x ({:.1}ms -> {:.1}ms)",
            r.curve_label,
            r.m,
            r.t,
            r.mode,
            r.verified_dlog,
            r.per_call_ops_speedup_x,
            r.before_ops_per_call,
            r.after_ops_per_call,
            r.priced_s_w01_speedup_x,
            r.before_s_w01,
            r.after_s_w01,
            r.rho_s_w01,
            r.wall_speedup_x,
            r.before_wall_ms,
            r.after_wall_ms
        );
    }

    println!("\n[2/4] Running Lead P1-3a (KM3-SPEC-LADDER + GrowthRationalPrimeCutoff for m=8, w=1,2)...");
    let lead_p1_3a = p1_3a_spec_ladder::run_all_p1_3a();
    for r in &lead_p1_3a.m8_exponent_table {
        println!(
            "  {} | {} | w={:.0}: alpha={:.5}, gamma={:.3}, E_all={:.4}, E_one={:.4}, kill_all={}, kill_one={}",
            r.tower_family,
            r.degree_slope_source,
            r.matrix_exponent_w,
            r.alpha_per_var,
            r.gamma_per_call_b_exp,
            r.exp_all_output_floor,
            r.exp_one_output_floor,
            r.killed_under_all_output_floor,
            r.killed_under_one_output_floor
        );
    }

    println!("\n[3/4] Running Lead P2-5 (Exact 256-bit P-256 Aux w^2=x^3-3x & Scaled p=3 mod 4 Towers)...");
    let lead_p2_5 = p2_5_p256_aux::run_all_p2_5();
    println!(
        "  P-256 depth-8 exact certificate check: fb_id={} (signed_points={})",
        lead_p2_5.p256_depth8_fb_id_verified, lead_p2_5.p256_depth8_signed_points
    );
    for r in &lead_p2_5.p256_presentations {
        println!(
            "  P-256 d={} {} ({}): vars={}, max_deg={}, cols={}, muladds={} -> replay={} ({:.2}x)",
            r.depth,
            r.fb_id,
            r.arm,
            r.n_vars,
            r.max_step_degree,
            r.max_cols,
            r.full_f4_muladds,
            r.replay_f4_muladds,
            r.replay_speedup_x
        );
    }
    for r in &lead_p2_5.scaled_j1728_aux_towers {
        println!(
            "  Scaled {} (d={}, m={}): D={}, cols={}, ops/call {:.1} -> {:.1} ({:.2}x), wall {:.1}ms -> {:.1}ms ({:.2}x)",
            r.curve_label,
            r.depth,
            r.m,
            r.solving_degree_max,
            r.max_cols,
            r.scratch_ops_per_call,
            r.trace_ops_per_call,
            r.ops_speedup_x,
            r.scratch_wall_ms,
            r.trace_wall_ms,
            r.wall_speedup_x
        );
    }

    println!("\n[4/4] Running CONTINUATION_PROTOCOL.md Panel A Empirical Scaling (<= 4 GiB RAM)...");
    let panel_a = panel_a_scaling::run_panel_a_scaling();
    for f in &panel_a.fits {
        println!(
            "  Panel A {} (m={}, t={:?}): gamma_pos={:.3} [{:.3}, {:.3}], gamma_stream {:.3} -> {:.3} | {}",
            f.family,
            f.m,
            f.rungs_t,
            f.gamma_hat_positive_calls,
            f.gamma_lcb95_positive_calls,
            f.gamma_ucb95_positive_calls,
            f.gamma_hat_natural_stream_before,
            f.gamma_hat_natural_stream_after,
            f.protocol_verdict
        );
    }

    let results = FullChaseResults {
        schema: "prime_pkm_aux_chase_20261119/v1".into(),
        date: "2026-11-19".into(),
        branch: "cursor/prime-pkm-aux-tower-3858".into(),
        standing: "all empirical measurements and projections are [pcc:unverified]".into(),
        kb_status: "KB unavailable (scripts/research-kb exit 127)".into(),
        lead_p0_1c,
        lead_p1_3a,
        lead_p2_5,
        panel_a,
    };

    let json_bytes = serde_json::to_string_pretty(&results).expect("serialize RESULTS.json");
    std::fs::write(&out_path, format!("{json_bytes}\n")).expect("write RESULTS.json");
    println!(
        "\nCompleted all suites in {:.2}s. Wrote {}",
        t_start.elapsed().as_secs_f64(),
        out_path.display()
    );
}
