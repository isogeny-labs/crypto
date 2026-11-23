//! `tower_large`: the cover tower carried to larger primes, on the
//! [`hyper64`] word Jacobian (`p < 2^31`, genus <= 3).
//!
//! ```text
//! tower_large [--primes 251,503,...] [--instances N] [--seeds S] [--rho-seeds S]
//!             [--json out.json] [--svg out.svg] [--crosscheck-only] [--no-crosscheck]
//! ```
//!
//! 1. **Gate.**  For the eight primes of `tower_bench` (53 ... 251) the same
//!    tower instances are built with `HCurve` (`tower::build`) and with
//!    `hyper64` and compared: cover, `#J`, pulled-up divisors, divisors after
//!    random operation sequences, `ic::solve` against `hyper_ic::solve`
//!    (every counter), rho on `E` and on the rungs.  Any mismatch aborts
//!    before a larger prime is touched.
//! 2. **Run.**  For each prime, `INSTANCES` towers (`E/F_p` with an order-`r`
//!    instance, `r >= p/4`, a genus-2 and a genus-3 cover, one pulled-up
//!    instance on both).  Counted index calculus (full degree-1 factor base,
//!    the `tower_bench` parameters) on both rungs and counted rho on `E`, each
//!    averaged over algorithm seeds.  Every recovered log is compared with the
//!    planted log and certified by an independent `[k]G == T` (right-to-left
//!    multiplication on the rungs, `Curve::mul` on `E`).
//! 3. **Report.**  Group operations (`adds + doubles`, all phases), linear
//!    algebra mulmods, the other emitted unpriced counters, the ratios at 0,
//!    0.1 and 1 GAE per native operation (AGENTS.md section 5a), trials per
//!    relation, and least-squares log-log slopes in `p`.  The one-time
//!    transfer (tower build and pull-up) is reported in its own columns and is
//!    in no total.
//!
//! Operation counts only; wall seconds in the JSON are bookkeeping, not
//! cost claims.  All figures are `[pcc:unverified]`.

use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::hyper64::{
    build64, crosscheck, ic, H64Curve, H64Div, Tower64,
};
use crypto_lib::cryptanalysis::chained_transfer::measure::EcGroup;
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

#[derive(serde::Serialize, Clone, Default)]
struct IcAgg {
    /// Cantor adds + doubles, all phases (the `IC` column).
    ops: f64,
    la_mulmods: f64,
    la_inversions: f64,
    oracle_fp_muls: f64,
    factor_base_fp_muls: f64,
    factor_base_lookups: f64,
    canonicalisations: f64,
    step_draws: f64,
    trials: f64,
    relations: f64,
    columns: f64,
}

impl IcAgg {
    /// Every emitted unpriced native counter of the arm.
    fn unpriced(&self) -> f64 {
        self.la_mulmods
            + self.la_inversions
            + self.oracle_fp_muls
            + self.factor_base_fp_muls
            + self.factor_base_lookups
            + self.canonicalisations
            + self.step_draws
    }
    fn add(&mut self, o: &IcAgg, w: f64) {
        self.ops += w * o.ops;
        self.la_mulmods += w * o.la_mulmods;
        self.la_inversions += w * o.la_inversions;
        self.oracle_fp_muls += w * o.oracle_fp_muls;
        self.factor_base_fp_muls += w * o.factor_base_fp_muls;
        self.factor_base_lookups += w * o.factor_base_lookups;
        self.canonicalisations += w * o.canonicalisations;
        self.step_draws += w * o.step_draws;
        self.trials += w * o.trials;
        self.relations += w * o.relations;
        self.columns += w * o.columns;
    }
}

#[derive(serde::Serialize, Clone)]
struct InstRow {
    p: u64,
    instance: usize,
    seed: u64,
    r: u64,
    n_e: u64,
    jac_order: [u64; 2],
    hom_checks: [u32; 2],
    /// `phi^*(T)` rational and equal to `[x]D` on the rung (`null`: not rational).
    target_pull_matches: [Option<bool>; 2],
    rho_ops: f64,
    rho_steps: f64,
    rho_dps: f64,
    ic: [IcAgg; 2],
    /// One-time transfer, never in a total: Cantor operations of pulling `G`
    /// and `T` up each rung, and wall seconds to build the tower and pull up.
    transfer_pull_ops: [u64; 2],
    transfer_wall_s: f64,
    solve_wall_s_bookkeeping: f64,
    all_recovered: bool,
    certified_solves: u32,
}

#[derive(serde::Serialize, Clone)]
struct PRow {
    p: u64,
    instances: usize,
    seeds: u32,
    rho_seeds: u32,
    r_mean: f64,
    rho_ops: f64,
    rho_steps: f64,
    rho_dps: f64,
    /// Index 0 = genus 2, 1 = genus 3.
    ic: [IcAgg; 2],
    ic3_over_ic2: f64,
    trials_per_relation: [f64; 2],
    g_factorial: [f64; 2],
    /// `(IC + w LA) / rho`, rho unpriced, `w` in `WEIGHTS`.
    ratio_la: [[f64; 3]; 2],
    /// `(IC + w U_IC) / (rho + w U_rho)`, every emitted counter priced on both arms.
    ratio_all: [[f64; 3]; 2],
    /// `(IC + w LA) / rho_walk`, where `rho_walk` is the walk steps alone (the
    /// `sqrt(pi r / 2)` floor, without the jump-table set-up the instrument charges).
    ratio_la_vs_walk: [[f64; 3]; 2],
    /// Does the order of IC_g against rho hold at every weight (`above`, `below`, `unresolved`)?
    verdict_la: [String; 2],
    verdict_all: [String; 2],
    transfer_pull_ops_mean: [f64; 2],
    transfer_wall_s_mean: f64,
    all_recovered: bool,
}

#[derive(serde::Serialize, Clone)]
struct Fit {
    series: String,
    slope_all_points: f64,
    slope_p_ge_1000: Option<f64>,
}

fn slope(pts: &[(f64, f64)]) -> Option<f64> {
    if pts.len() < 2 || pts.iter().any(|&(x, y)| x <= 0.0 || y <= 0.0) {
        return None;
    }
    let l: Vec<(f64, f64)> = pts.iter().map(|&(x, y)| (x.ln(), y.ln())).collect();
    let n = l.len() as f64;
    let (mx, my) = (
        l.iter().map(|a| a.0).sum::<f64>() / n,
        l.iter().map(|a| a.1).sum::<f64>() / n,
    );
    let num: f64 = l.iter().map(|a| (a.0 - mx) * (a.1 - my)).sum();
    let den: f64 = l.iter().map(|a| (a.0 - mx) * (a.0 - mx)).sum();
    Some(num / den)
}

fn next_prime(mut n: u64) -> u64 {
    loop {
        if n >= 3 && crypto_lib::cryptanalysis::chained_transfer::field::is_prime(n) {
            return n;
        }
        n += 1;
    }
}

fn aggregate_ic(o: &ic::Ic64Outcome) -> IcAgg {
    IcAgg {
        ops: o.group_ops() as f64,
        la_mulmods: o.la_mulmods() as f64,
        la_inversions: o.counter("linalg_inversions_uncharged") as f64,
        oracle_fp_muls: o.counter("oracle_fp_muls_uncharged") as f64,
        factor_base_fp_muls: o.counter("factor_base_fp_muls_uncharged") as f64,
        factor_base_lookups: o.counter("factor_base_lookups_uncharged") as f64,
        canonicalisations: o.counter("canonicalisations_uncharged") as f64,
        step_draws: o.counter("step_draws_uncharged") as f64,
        trials: o.counter("trials") as f64,
        relations: o.counter("relations") as f64,
        columns: o.counter("factor_base_columns") as f64,
    }
}

/// Independent `[k]G == T` certificate on a rung (right-to-left multiplication).
fn certify_rung(c: &H64Curve, d: H64Div, t: H64Div, k: u64) -> bool {
    c.is_valid(&d) && c.is_valid(&t) && c.mul_rtl(d, k) == t
}

fn measure(t: &Tower64, instance: usize, seed: u64, seeds: u32, rho_seeds: u32) -> InstRow {
    let t0 = Instant::now();
    let mut ok = true;
    let mut certified = 0u32;
    // Rho on E.
    let eg = EcGroup(t.e);
    let (mut rho_ops, mut rho_steps, mut rho_dps) = (0f64, 0f64, 0f64);
    for s in 0..rho_seeds as u64 {
        let rho = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 26);
        let good = rho.recovered == Some(t.x)
            && rho
                .recovered
                .is_some_and(|k| t.e.mul(&t.gen, k as u128) == t.target);
        ok &= good;
        certified += u32::from(good);
        rho_ops += (rho.group_ops.adds + rho.group_ops.doubles) as f64;
        rho_steps += rho.steps as f64;
        rho_dps += rho.distinguished_points as f64;
    }
    let n = f64::from(rho_seeds);
    // Index calculus on both rungs.
    let params = crosscheck::ic_params();
    let mut ic_rows = [IcAgg::default(), IcAgg::default()];
    for (i, rg) in t.rungs.iter().enumerate() {
        let cur = &rg.cover.curve;
        for s in 0..seeds as u64 {
            match ic::solve(cur, rg.d, rg.t, t.r, rg.jac_order, 300 + s, &params) {
                Ok(out) => {
                    let good = out.recovered == Some(t.x)
                        && out
                            .recovered
                            .is_some_and(|k| certify_rung(cur, rg.d, rg.t, k));
                    ok &= good;
                    certified += u32::from(good);
                    ic_rows[i].add(&aggregate_ic(&out), 1.0 / f64::from(seeds));
                }
                Err(e) => {
                    eprintln!("ic p={} genus={}: {e}", t.p, rg.cover.genus);
                    ok = false;
                }
            }
        }
    }
    InstRow {
        p: t.p,
        instance,
        seed,
        r: t.r,
        n_e: t.e.order(),
        jac_order: [t.rungs[0].jac_order, t.rungs[1].jac_order],
        hom_checks: [t.rungs[0].hom_checks, t.rungs[1].hom_checks],
        target_pull_matches: [
            t.rungs[0].target_pull_matches,
            t.rungs[1].target_pull_matches,
        ],
        rho_ops: rho_ops / n,
        rho_steps: rho_steps / n,
        rho_dps: rho_dps / n,
        ic: ic_rows,
        transfer_pull_ops: [t.rungs[0].pull_ops, t.rungs[1].pull_ops],
        transfer_wall_s: t.transfer_wall_s,
        solve_wall_s_bookkeeping: t0.elapsed().as_secs_f64(),
        all_recovered: ok,
        certified_solves: certified,
    }
}

fn verdict(ratios: &[f64; 3]) -> String {
    if ratios.iter().all(|&r| r > 1.0) {
        "above rho at every weight".into()
    } else if ratios.iter().all(|&r| r < 1.0) {
        "below rho at every weight".into()
    } else {
        "unresolved (order changes inside the band)".into()
    }
}

fn aggregate(p: u64, rows: &[&InstRow], seeds: u32, rho_seeds: u32) -> PRow {
    let k = rows.len() as f64;
    let m = |f: &dyn Fn(&InstRow) -> f64| rows.iter().map(|r| f(r)).sum::<f64>() / k;
    let rho_ops = m(&|r| r.rho_ops);
    let rho_steps = m(&|r| r.rho_steps);
    let rho_dps = m(&|r| r.rho_dps);
    let mut ic = [IcAgg::default(), IcAgg::default()];
    for r in rows {
        for g in 0..2 {
            ic[g].add(&r.ic[g], 1.0 / k);
        }
    }
    let mut ratio_la = [[0f64; 3]; 2];
    let mut ratio_all = [[0f64; 3]; 2];
    let mut ratio_walk = [[0f64; 3]; 2];
    for g in 0..2 {
        for (j, &w) in WEIGHTS.iter().enumerate() {
            ratio_la[g][j] = (ic[g].ops + w * ic[g].la_mulmods) / rho_ops;
            ratio_walk[g][j] = (ic[g].ops + w * ic[g].la_mulmods) / rho_steps;
            ratio_all[g][j] =
                (ic[g].ops + w * ic[g].unpriced()) / (rho_ops + w * (rho_steps + rho_dps));
        }
    }
    PRow {
        p,
        instances: rows.len(),
        seeds,
        rho_seeds,
        r_mean: m(&|r| r.r as f64),
        rho_ops,
        rho_steps,
        rho_dps,
        ic3_over_ic2: ic[1].ops / ic[0].ops,
        trials_per_relation: [
            ic[0].trials / ic[0].relations,
            ic[1].trials / ic[1].relations,
        ],
        g_factorial: [2.0, 6.0],
        verdict_la: [verdict(&ratio_la[0]), verdict(&ratio_la[1])],
        verdict_all: [verdict(&ratio_all[0]), verdict(&ratio_all[1])],
        ratio_la,
        ratio_all,
        ratio_la_vs_walk: ratio_walk,
        ic,
        transfer_pull_ops_mean: [
            m(&|r| r.transfer_pull_ops[0] as f64),
            m(&|r| r.transfer_pull_ops[1] as f64),
        ],
        transfer_wall_s_mean: m(&|r| r.transfer_wall_s),
        all_recovered: rows.iter().all(|r| r.all_recovered),
    }
}

fn fits(rows: &[PRow]) -> Vec<Fit> {
    type Getter = Box<dyn Fn(&PRow) -> f64>;
    let series: Vec<(String, Getter)> = vec![
        ("rho on E (group ops)".into(), Box::new(|r| r.rho_ops)),
        ("rho on E walk steps only".into(), Box::new(|r| r.rho_steps)),
        ("mean r".into(), Box::new(|r| r.r_mean)),
        ("IC2 group ops".into(), Box::new(|r| r.ic[0].ops)),
        ("IC3 group ops".into(), Box::new(|r| r.ic[1].ops)),
        ("IC3 / IC2".into(), Box::new(|r| r.ic3_over_ic2)),
        (
            "IC2 linalg mulmods".into(),
            Box::new(|r| r.ic[0].la_mulmods),
        ),
        (
            "IC3 linalg mulmods".into(),
            Box::new(|r| r.ic[1].la_mulmods),
        ),
        (
            "IC2 ops + 0.1 x linalg".into(),
            Box::new(|r| r.ic[0].ops + 0.1 * r.ic[0].la_mulmods),
        ),
        (
            "IC3 ops + 0.1 x linalg".into(),
            Box::new(|r| r.ic[1].ops + 0.1 * r.ic[1].la_mulmods),
        ),
        (
            "IC2 ops + 1.0 x linalg".into(),
            Box::new(|r| r.ic[0].ops + r.ic[0].la_mulmods),
        ),
        (
            "IC3 ops + 1.0 x linalg".into(),
            Box::new(|r| r.ic[1].ops + r.ic[1].la_mulmods),
        ),
        (
            "IC2 trials per relation".into(),
            Box::new(|r| r.trials_per_relation[0]),
        ),
        (
            "IC3 trials per relation".into(),
            Box::new(|r| r.trials_per_relation[1]),
        ),
        (
            "IC2 oracle fp muls".into(),
            Box::new(|r| r.ic[0].oracle_fp_muls),
        ),
        (
            "IC3 oracle fp muls".into(),
            Box::new(|r| r.ic[1].oracle_fp_muls),
        ),
        (
            "factor-base columns, genus 2".into(),
            Box::new(|r| r.ic[0].columns),
        ),
    ];
    series
        .into_iter()
        .map(|(name, f)| {
            let all: Vec<(f64, f64)> = rows.iter().map(|r| (r.p as f64, f(r))).collect();
            let tail: Vec<(f64, f64)> = all.iter().copied().filter(|&(x, _)| x >= 1000.0).collect();
            Fit {
                series: name,
                slope_all_points: slope(&all).unwrap_or(f64::NAN),
                slope_p_ge_1000: if tail.len() >= 3 { slope(&tail) } else { None },
            }
        })
        .collect()
}

fn render_svg(rows: &[PRow]) -> String {
    let series: [(&str, &str, &str, Box<dyn Fn(&PRow) -> f64>); 6] = [
        (
            "#8e44ad",
            "",
            "IC on J(C3), genus 3 (group ops)",
            Box::new(|r| r.ic[1].ops),
        ),
        (
            "#c0392b",
            "",
            "IC on J(C2), genus 2 (group ops)",
            Box::new(|r| r.ic[0].ops),
        ),
        (
            "#2c3e50",
            "",
            "rho on E (matched reference)",
            Box::new(|r| r.rho_ops),
        ),
        (
            "#8e44ad",
            "6,4",
            "IC3 + 0.1 x linalg mulmods",
            Box::new(|r| r.ic[1].ops + 0.1 * r.ic[1].la_mulmods),
        ),
        (
            "#c0392b",
            "6,4",
            "IC2 + 0.1 x linalg mulmods",
            Box::new(|r| r.ic[0].ops + 0.1 * r.ic[0].la_mulmods),
        ),
        (
            "#7f8c8d",
            "2,3",
            "reference slope 1/2 (sqrt p)",
            Box::new(|_| f64::NAN),
        ),
    ];
    let xs: Vec<f64> = rows.iter().map(|r| r.p as f64).collect();
    let mut ys: Vec<f64> = vec![];
    for (_, _, _, f) in series.iter().take(5) {
        ys.extend(rows.iter().map(f));
    }
    let (xmin, xmax) = (
        xs.iter().cloned().fold(f64::MAX, f64::min) / 1.3,
        xs.iter().cloned().fold(0.0, f64::max) * 1.3,
    );
    let ymin = 10f64.powf(ys.iter().cloned().fold(f64::MAX, f64::min).log10().floor());
    let ymax = 10f64.powf(ys.iter().cloned().fold(0.0, f64::max).log10().ceil());
    let (x0, x1, y0, y1) = (xmin.ln(), xmax.ln(), ymin.ln(), ymax.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.max(ymin).ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"640\" height=\"440\" viewBox=\"0 0 640 440\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"640\" height=\"440\" fill=\"#fff\"/>\n");
    s.push_str("<text x=\"320\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Tower up at larger p: IC on genus 2 and 3 against rho on E</text>\n");
    s.push_str("<text x=\"320\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">counted group operations, log-log; solid = group ops only, dashed = linalg priced at 0.1 GAE; [pcc:unverified]</text>\n");
    let mut t = ymin;
    while t <= ymax * 1.0001 {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t:e}</text>\n", y + 4.0));
        t *= 10.0;
    }
    for r in rows {
        let x = xp(r.p as f64);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"360\" stroke=\"#eee\"/><text x=\"{x:.1}\" y=\"376\" text-anchor=\"middle\" fill=\"#444\">{}</text>\n", r.p));
    }
    s.push_str("<text x=\"340\" y=\"400\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"210\" text-anchor=\"middle\" transform=\"rotate(-90 22 210)\">operations</text>\n");
    // Reference slope 1/2 anchored at the first rho point.
    if let Some(first) = rows.first() {
        let (xa, ya) = (first.p as f64, first.rho_ops);
        let xb = rows.last().unwrap().p as f64;
        let yb = ya * (xb / xa).sqrt();
        s.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#7f8c8d\" stroke-dasharray=\"2,3\"/>\n", xp(xa), yp(ya), xp(xb), yp(yb)));
    }
    s.push_str(
        "<rect x=\"96\" y=\"64\" width=\"270\" height=\"112\" fill=\"#fff\" stroke=\"#ccc\"/>\n",
    );
    for (i, (col, dash, label, f)) in series.iter().enumerate() {
        if i < 5 {
            let pts: Vec<String> = rows
                .iter()
                .map(|r| format!("{:.1},{:.1}", xp(r.p as f64), yp(f(r))))
                .collect();
            s.push_str(&format!("<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", pts.join(" ")));
            for pt in &pts {
                let (x, y) = pt.split_once(',').unwrap();
                s.push_str(&format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"{col}\"/>"
                ));
            }
        }
        let ly = 80 + 16 * i;
        s.push_str(&format!("\n<line x1=\"104\" y1=\"{ly}\" x2=\"128\" y2=\"{ly}\" stroke=\"{col}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/><text x=\"134\" y=\"{}\">{label}</text>\n", ly + 4));
    }
    s.push_str("</svg>\n");
    s
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let primes: Vec<u64> = match arg("--primes") {
        Some(s) => s
            .split(',')
            .map(|v| next_prime(v.trim().parse().expect("prime")))
            .collect(),
        None => [250u64, 500, 1000, 2000, 4000, 8000]
            .map(next_prime)
            .to_vec(),
    };
    let instances: usize = arg("--instances").map_or(3, |v| v.parse().expect("instances"));
    let seeds: u32 = arg("--seeds").map_or(8, |v| v.parse().expect("seeds"));
    let rho_seeds: u32 = arg("--rho-seeds").map_or(64, |v| v.parse().expect("rho-seeds"));
    let min_r_div: u64 = arg("--min-r-div").map_or(4, |v| v.parse().expect("min-r-div"));
    let threads: usize = arg("--threads").map_or(2, |v| v.parse().expect("threads"));

    // ── Gate: hyper64 against HCurve on the p < 256 towers ──
    let mut gate = vec![];
    let mut gate_ok = true;
    if !has("--no-crosscheck") {
        for (i, p) in [53u64, 79, 101, 131, 151, 181, 211, 251]
            .into_iter()
            .enumerate()
        {
            match crosscheck::run(p, 11 + i as u64, 600, 8) {
                Ok(rep) => {
                    eprintln!(
                        "crosscheck p={p}: ok  built={} fields={} divisors={} ic={} rho={}",
                        rep.built,
                        rep.field_checks,
                        rep.divisors_compared,
                        rep.ic_solves_compared,
                        rep.rho_runs_compared
                    );
                    gate.push(serde_json::json!({"p": p, "ok": true, "report": rep}));
                }
                Err(e) => {
                    eprintln!("crosscheck p={p}: FAILED: {e}");
                    gate.push(serde_json::json!({"p": p, "ok": false, "error": e}));
                    gate_ok = false;
                }
            }
        }
        println!(
            "cross-check hyper64 vs HCurve on p in 53..251: {}",
            if gate_ok { "PASS" } else { "FAIL" }
        );
        if !gate_ok || has("--crosscheck-only") {
            if let Some(path) = arg("--json") {
                let doc = serde_json::json!({"crosscheck_pass": gate_ok, "crosscheck": gate});
                std::fs::write(path, serde_json::to_string_pretty(&doc).unwrap())
                    .expect("write json");
            }
            return if gate_ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            };
        }
    }

    // ── Run ──
    let jobs: Vec<(u64, usize)> = {
        let mut j: Vec<(u64, usize)> = primes
            .iter()
            .flat_map(|&p| (0..instances).map(move |i| (p, i)))
            .collect();
        j.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        j
    };
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<InstRow>> = Mutex::new(vec![]);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(p, inst)) = jobs.get(j) else { break };
                let seed = 1000 + 17 * inst as u64 + p % 997;
                let t0 = Instant::now();
                let Some(t) = build64(p, seed, p / min_r_div, 64) else {
                    eprintln!("p={p} instance {inst}: no tower instance (seed {seed})");
                    continue;
                };
                eprintln!(
                    "p={p} instance {inst}: built (r={}, #E={}) in {:.1}s",
                    t.r,
                    t.e.order(),
                    t0.elapsed().as_secs_f64()
                );
                let row = measure(&t, inst, seed, seeds, rho_seeds);
                eprintln!(
                    "p={p} instance {inst}: done in {:.1}s: rho={:.1} ic2={:.1} ic3={:.1} ok={}",
                    t0.elapsed().as_secs_f64(),
                    row.rho_ops,
                    row.ic[0].ops,
                    row.ic[1].ops,
                    row.all_recovered
                );
                results.lock().unwrap().push(row);
            });
        }
    });
    let mut inst_rows = results.into_inner().unwrap();
    inst_rows.sort_by_key(|r| (r.p, r.instance));
    let mut prows = vec![];
    for &p in &primes {
        let rs: Vec<&InstRow> = inst_rows.iter().filter(|r| r.p == p).collect();
        if rs.is_empty() {
            eprintln!("no instance at p = {p}");
            continue;
        }
        prows.push(aggregate(p, &rs, seeds, rho_seeds));
    }
    prows.sort_by_key(|r| r.p);
    let fit = fits(&prows);

    println!(
        "{:>6} {:>4} {:>8} {:>9} {:>10} {:>10} {:>7} {:>11} {:>11} {:>6} {:>6} {:>6} {:>6}  ok",
        "p",
        "inst",
        "r_mean",
        "rho_E",
        "IC2",
        "IC3",
        "IC3/IC2",
        "LA2",
        "LA3",
        "tr/rel2",
        "tr/rel3",
        "IC2/ρ",
        "IC3/ρ"
    );
    for r in &prows {
        println!(
            "{:>6} {:>4} {:>8.0} {:>9.1} {:>10.1} {:>10.1} {:>7.2} {:>11.0} {:>11.0} {:>6.2} {:>6.2} {:>6.2} {:>6.2}  {}",
            r.p, r.instances, r.r_mean, r.rho_ops, r.ic[0].ops, r.ic[1].ops, r.ic3_over_ic2,
            r.ic[0].la_mulmods, r.ic[1].la_mulmods, r.trials_per_relation[0], r.trials_per_relation[1],
            r.ratio_la[0][0], r.ratio_la[1][0], r.all_recovered
        );
    }
    for f in &fit {
        println!(
            "slope {:<28} all {:>6.3}  p>=1000 {}",
            f.series,
            f.slope_all_points,
            f.slope_p_ge_1000
                .map_or("n/a".into(), |s| format!("{s:.3}"))
        );
    }
    let all_ok = prows.iter().all(|r| r.all_recovered) && !prows.is_empty();
    if let Some(path) = arg("--svg") {
        std::fs::write(path, render_svg(&prows)).expect("write svg");
    }
    if let Some(path) = arg("--json") {
        let doc = serde_json::json!({
            "crosscheck_pass": gate_ok && !has("--no-crosscheck"),
            "crosscheck": gate,
            "parameters": {
                "primes": primes, "instances": instances, "seeds": seeds, "rho_seeds": rho_seeds,
                "min_r": format!("p/{min_r_div}"), "weights_gae_per_native_op": WEIGHTS,
                "ic_params": "fb_size=max, large_primes=false, extra_relations=8, branches=64, d_step_in=4, max_trials=5e6",
                "note": "operation counts only; wall seconds are bookkeeping, not cost claims; transfer columns are one-time and in no total",
            },
            "instances": inst_rows,
            "rows": prows,
            "fits": fit,
        });
        std::fs::write(path, serde_json::to_string_pretty(&doc).unwrap()).expect("write json");
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
