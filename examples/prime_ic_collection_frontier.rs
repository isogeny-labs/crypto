//! Prime-field index-calculus **collection-method frontier**: an isolated
//! A/B of the two relation-collection oracles in `cryptanalysis::prime_fast`,
//! at a fixed factor base and a fixed decomposition set, across factor-base
//! size `B`.
//!
//! The two oracles decide the *same* decomposable set — a target `R` is
//! productive iff `R = ±F_i ± F_j` for factor-base points `F_i, F_j` — but by
//! different work:
//!
//! - **sweep** ([`collect_relations`] with `pairs = None`): subtract every
//!   factor-base element from `R` under one batched inversion and look each
//!   `x(R ∓ F_i)` up — `O(B)` field operations per target, no pair table.
//! - **table** ([`collect_relations_table`]): precompute every `x(F_j ± F_k)`
//!   once (`B²` entries) and look `x(R)` up — `O(1)` per target, plus the
//!   one-time `O(B²)` build.
//!
//! Because both take the same `(g, q, seed)` and the same walk rule
//! (`R ← R + G` until a productive target, then fresh `(a, b)`), they walk the
//! **identical target stream** and retire on the **same** productive targets,
//! so the number of targets is equal by construction (reported, and checked).
//! The only variable is the per-target detection cost, isolating the
//! collection method from target yield, decomposition arity (both `m = 2`),
//! and the shared linear-algebra solve.
//!
//! Every recovered log is checked against the planted scalar; the sweep and
//! the table must agree.  A matched single-thread `rho_parallel` wall is
//! printed per size for context (rho remains the reference solver; this is an
//! IC-oracle study, not a vs-rho claim).
//!
//! ```bash
//! cargo run --release --example prime_ic_collection_frontier -- \
//!     [--bits 24,28] [--fb 60,120,240,480,960] [--seed 20261117] \
//!     [--max-targets 80000000] [--extra 16] [--json out.json]
//! ```
//!
//! Public synthetic known-answer only.

use std::time::Instant;

use crypto_lib::cryptanalysis::prime_fast::{
    collect_relations, collect_relations_table, find_a3_curve, ic_multi_solve_ex, ic_solve,
    ic_solve_fast, ic_solve_table, rho_parallel, solve_relations, FastFactorBase, PairTable,
    RhoConfig,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

struct Args {
    bits: Vec<u32>,
    fb: Vec<usize>,
    seed: u64,
    max_targets: u64,
    extra: usize,
    json: Option<String>,
    rho: bool,
    /// If > 0, also run the three full single-target solvers end to end at this B.
    e2e_b: usize,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut a = Args {
        bits: vec![24, 28],
        fb: vec![60, 120, 240, 480, 960],
        seed: 20_261_117,
        max_targets: 80_000_000,
        extra: 16,
        json: None,
        rho: true,
        e2e_b: 0,
    };
    let mut i = 1;
    while i < argv.len() {
        let next = |i: &mut usize| -> String {
            *i += 1;
            argv.get(*i).cloned().unwrap_or_default()
        };
        let parse_list_u32 = |s: String| -> Vec<u32> {
            s.split(',').filter_map(|t| t.trim().parse().ok()).collect()
        };
        let parse_list_usize = |s: String| -> Vec<usize> {
            s.split(',').filter_map(|t| t.trim().parse().ok()).collect()
        };
        match argv[i].as_str() {
            "--bits" => a.bits = parse_list_u32(next(&mut i)),
            "--fb" => a.fb = parse_list_usize(next(&mut i)),
            "--seed" => a.seed = next(&mut i).parse().unwrap_or(a.seed),
            "--max-targets" => a.max_targets = next(&mut i).parse().unwrap_or(a.max_targets),
            "--extra" => a.extra = next(&mut i).parse().unwrap_or(a.extra),
            "--json" => a.json = Some(next(&mut i)),
            "--no-rho" => a.rho = false,
            "--e2e-b" => a.e2e_b = next(&mut i).parse().unwrap_or(0),
            other => eprintln!("ignoring unknown argument {other}"),
        }
        i += 1;
    }
    a
}

fn secs<T>(f: impl FnOnce() -> T) -> (T, f64) {
    let t = Instant::now();
    let out = f();
    (out, t.elapsed().as_secs_f64())
}

fn main() {
    let args = parse_args();
    let mut out: Vec<serde_json::Value> = Vec::new();
    let host = json!({
        "threads": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        "arch": std::env::consts::ARCH,
        "os": std::env::consts::OS,
    });
    println!("# Prime-field IC collection-method frontier (sweep vs table)\n");
    println!(
        "Host: {} {}, {} hardware threads. seed={} extra={} max_targets={}\n",
        host["os"], host["arch"], host["threads"], args.seed, args.extra, args.max_targets
    );

    for &bits in &args.bits {
        let (t, gen_s) = secs(|| find_a3_curve(bits, 3));
        let Some((fc, g)) = t else {
            eprintln!("no a=-3 curve at {bits} bits");
            continue;
        };
        let mut rng = StdRng::seed_from_u64(args.seed ^ bits as u64);
        let k = rng.gen_range(1..fc.n);
        let q = fc.mul(Some(g), k).unwrap();
        println!(
            "## {bits}-bit curve {}  (p={}, n={}, planted k, generated in {:.2}s)\n",
            fc.name, fc.f.p, fc.n, gen_s
        );

        // Matched single-thread rho, for context.
        let mut rho_wall = f64::NAN;
        if args.rho {
            let mut cfg = RhoConfig::for_bits(bits, true);
            cfg.threads = 1;
            cfg.seed = 3;
            if let Some(r) = rho_parallel(&fc, g, q, &cfg) {
                assert_eq!(r.log, k, "rho wrong answer at {bits} bits");
                rho_wall = r.wall_secs;
            }
            println!("Matched rho (1 thread): {rho_wall:.4}s\n");
        }

        println!("| B | needed | targets | sweep collect s | table build s | table collect s | collect speed-up | total (incl build) speed-up | LA s | sweep=table log |");
        println!("|---:|---:|---:|---:|---:|---:|---:|---:|---:|:--:|");

        for &b in &args.fb {
            let fb = FastFactorBase::build(&fc, b);
            if fb.len() < b {
                eprintln!("  B={b}: only {} points liftable, skipping", fb.len());
                continue;
            }
            let m = fb.len();
            let needed = m + args.extra;

            // ── sweep (no pair table) ───────────────────────────────────
            let ((sweep_rels, sweep_stats), sweep_collect_s) = secs(|| {
                collect_relations(&fc, g, q, &fb, None, needed, args.max_targets, args.seed)
            });
            if sweep_rels.len() < needed {
                println!(
                    "| {b} | {needed} | {} | incomplete (sweep got {} < {needed}) | | | | | | |",
                    sweep_stats.targets,
                    sweep_rels.len()
                );
                continue;
            }
            let (sweep_log, sweep_la_s) = secs(|| solve_relations(&fc, m, &sweep_rels));
            let sweep_log = sweep_log.expect("sweep linear algebra determines log");
            assert_eq!(sweep_log, k, "sweep IC wrong answer at {bits} bits B={b}");

            // ── table (pair-table build + O(1)/target lookup) ───────────
            let (pairs, table_build_s) = secs(|| PairTable::build(&fc, &fb));
            let ((table_rels, table_stats), table_collect_s) = secs(|| {
                collect_relations_table(&fc, g, q, &fb, &pairs, needed, args.max_targets, args.seed)
            });
            let (table_log, table_la_s) = secs(|| solve_relations(&fc, m, &table_rels));
            let table_log = table_log.expect("table linear algebra determines log");
            assert_eq!(table_log, k, "table IC wrong answer at {bits} bits B={b}");

            let collect_speedup = sweep_collect_s / table_collect_s.max(1e-12);
            let sweep_total = sweep_collect_s + sweep_la_s;
            let table_total = table_build_s + table_collect_s + table_la_s;
            let total_speedup = sweep_total / table_total.max(1e-12);
            let agree = sweep_log == table_log;

            println!(
                "| {b} | {needed} | {} | {sweep_collect_s:.4} | {table_build_s:.4} | {table_collect_s:.4} | {collect_speedup:.1}× | {total_speedup:.2}× | {sweep_la_s:.4}/{table_la_s:.4} | {} |",
                sweep_stats.targets,
                if agree { "yes" } else { "NO" }
            );

            out.push(json!({
                "bits": bits, "curve": fc.name, "p": fc.f.p, "n": fc.n,
                "B": b, "needed": needed,
                "sweep_targets": sweep_stats.targets, "table_targets": table_stats.targets,
                "sweep_collect_s": sweep_collect_s, "sweep_la_s": sweep_la_s,
                "table_build_s": table_build_s, "table_collect_s": table_collect_s, "table_la_s": table_la_s,
                "pair_table_entries": pairs.len(),
                "collect_speedup": collect_speedup, "total_speedup": total_speedup,
                "sweep_rows": [sweep_stats.rows_one, sweep_stats.rows_two, sweep_stats.rows_three],
                "table_rows": [table_stats.rows_one, table_stats.rows_two, table_stats.rows_three],
                "rho_wall_s_1thread": rho_wall,
            }));
        }
        println!();

        // ── end-to-end: three full single-target solvers at one B ───────
        if args.e2e_b > 0 {
            let b = args.e2e_b;
            let mw = args.max_targets;
            println!("### End-to-end single-target solve at B={b} (total wall s)\n");
            println!("| solver | total s | relations/targets | vs sweep | vs rho (1 thr) |");
            println!("|:--|--:|--:|--:|--:|");
            // sweep (default ic_solve, summands=2)
            let ((x_sw, rep_sw), sw_s) = {
                let (r, s) = secs(|| ic_solve(&fc, g, q, b, args.extra, 2, mw, args.seed));
                (r.expect("ic_solve"), s)
            };
            assert_eq!(x_sw, k, "ic_solve wrong log");
            // table single-walk (ic_solve_table)
            let ((x_tb, rep_tb), tb_s) = {
                let (r, s) = secs(|| ic_solve_table(&fc, g, q, b, args.extra, mw, args.seed));
                (r.expect("ic_solve_table"), s)
            };
            assert_eq!(x_tb, k, "ic_solve_table wrong log");
            // batched lock-step table (ic_multi_solve_ex, table+batched), single target
            let ((logs_ba, rep_ba), ba_s) = {
                let (r, s) = secs(|| {
                    ic_multi_solve_ex(&fc, g, &[q], b, args.extra, mw, args.seed, true, true)
                });
                (r.expect("ic_multi_solve_ex"), s)
            };
            assert_eq!(
                logs_ba.first().copied().flatten(),
                Some(k),
                "batched wrong log"
            );
            let rows = |t: f64| format!("{t:.4}");
            println!(
                "| sweep `ic_solve` | {} | {}/{} | 1.00× | {:.3}× |",
                rows(sw_s),
                rep_sw.relations,
                rep_sw.targets,
                rho_wall / sw_s
            );
            println!(
                "| table `ic_solve_table` | {} | {}/{} | {:.2}× | {:.3}× |",
                rows(tb_s),
                rep_tb.relations,
                rep_tb.targets,
                sw_s / tb_s,
                rho_wall / tb_s
            );
            println!(
                "| batched `ic_multi_solve_ex` | {} | {}/{} | {:.2}× | {:.3}× |",
                rows(ba_s),
                rep_ba.determined_logs,
                rep_ba.targets_walked.first().copied().unwrap_or(0),
                sw_s / ba_s,
                rho_wall / ba_s
            );
            // ic_solve_fast: auto-tuned B = round(p^(1/3)), batched table path
            let ((x_fa, rep_fa), fa_s) = {
                let (r, s) = secs(|| ic_solve_fast(&fc, g, q, None, args.extra, mw, args.seed));
                (r.expect("ic_solve_fast"), s)
            };
            assert_eq!(x_fa, k, "ic_solve_fast wrong log");
            println!(
                "| `ic_solve_fast` (auto B={}) | {:.4} | {}/{} | {:.2}× | {:.3}× |",
                rep_fa.factor_base_size,
                fa_s,
                rep_fa.determined_logs,
                rep_fa.target_points_walked,
                sw_s / fa_s,
                rho_wall / fa_s
            );
            out.push(json!({
                "section": "e2e_fast", "bits": bits, "sweep_B": b, "fast_B": rep_fa.factor_base_size,
                "sweep_total_s": sw_s, "fast_total_s": fa_s, "fast_vs_sweep": sw_s / fa_s,
            }));
            println!();
            out.push(json!({
                "section": "e2e", "bits": bits, "B": b,
                "sweep_total_s": sw_s, "table_total_s": tb_s, "batched_total_s": ba_s,
                "rho_wall_s_1thread": rho_wall,
                "table_vs_sweep": sw_s / tb_s, "batched_vs_sweep": sw_s / ba_s,
            }));
        }
    }

    if let Some(path) = &args.json {
        let doc = json!({"host": host, "seed": args.seed, "results": out});
        std::fs::write(path, serde_json::to_string_pretty(&doc).unwrap()).expect("write json");
        println!("JSON written to {path}");
    }
}
