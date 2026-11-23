//! `cover_pushforward_deg3`: are the index-calculus relations on the
//! transferred subgroup of a genus-4 Jacobian (degree-3 cover of `E`) all full
//! fibres of the cover, and is the index calculus therefore a collision search
//! on `E`?
//!
//! ```text
//! cover_pushforward_deg3 [--primes 53,79,...] [--instances 3] [--seeds 15]
//!                        [--json F] [--svg F]
//! ```
//!
//! Protocol: `research/cover_pushforward_deg3_20261112/PROTOCOL.md`.
//! The cover construction is a one-time cost reported per cell and is in no total.
//! All figures are `[pcc:unverified]`.

use std::process::ExitCode;

use crypto_lib::cryptanalysis::chained_transfer::pushforward_deg3::{
    b_stats, build_instance, run_seed, BStats, SeedRow,
};
use serde_json::{json, Value};

const PRIMES: [u64; 8] = [53, 79, 101, 131, 151, 181, 211, 251];
const MAX_TRIALS: u64 = 5_000_000;

struct Cell {
    p: u64,
    j: u64,
    skipped: Vec<String>,
    r: u64,
    n_e: u64,
    jac_order: u64,
    h1: u64,
    h0: u64,
    h_attempts: u32,
    hom_checks: u32,
    tower_seed: u64,
    build_ms: f64,
    b: BStats,
    rows: Vec<SeedRow>,
}

fn run_prime(p: u64, i: u64, instances: u64, seeds: u64) -> Vec<Result<Cell, String>> {
    let mut out = vec![];
    let mut skipped: Vec<String> = vec![];
    let mut j = 0u64;
    while (out.len() as u64) < instances && j < 40 {
        let cand = j;
        j += 1;
        let inst = match build_instance(p, i, cand) {
            Ok(x) => x,
            Err(e) => {
                skipped.push(format!("j={cand}: {e}"));
                continue;
            }
        };
        let b = b_stats(&inst);
        if b.b3_in_g_cols < 2 {
            skipped.push(format!(
                "j={cand}: |B3 cap <G_E>| up to sign = {} < 2",
                b.b3_in_g_cols
            ));
            continue;
        }
        let rows: Vec<SeedRow> = (0..seeds).map(|s| run_seed(&inst, s, MAX_TRIALS)).collect();
        out.push(Ok(Cell {
            p,
            j: cand,
            skipped: std::mem::take(&mut skipped),
            r: inst.r,
            n_e: inst.n_e,
            jac_order: inst.jac_order,
            h1: inst.cov.h1,
            h0: inst.cov.h0,
            h_attempts: inst.h_attempts,
            hom_checks: inst.hom_checks,
            tower_seed: inst.tower_seed,
            build_ms: inst.build_ms,
            b,
            rows,
        }));
    }
    if (out.len() as u64) < instances {
        out.push(Err(format!(
            "p = {p}: only {} admissible instances in 40 candidates; skipped: {skipped:?}",
            out.len()
        )));
    }
    out
}

fn mean(v: impl Iterator<Item = f64>) -> f64 {
    let (s, n) = v.fold((0.0, 0.0), |(s, n), x| (s + x, n + 1.0));
    if n == 0.0 {
        0.0
    } else {
        s / n
    }
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

struct Agg {
    rho: f64,
    rho_neg: f64,
    ops_j: f64,
    ops_e: f64,
    la_j: f64,
    la_e: f64,
    fc_j: Vec<f64>,
    fc_e: Vec<f64>,
    fc_b: Vec<f64>,
    rho_all: Vec<f64>,
}

fn agg(c: &Cell) -> Agg {
    let fc = |f: &dyn Fn(&SeedRow) -> Option<u64>| -> Vec<f64> {
        c.rows
            .iter()
            .filter_map(|r| f(r).map(|x| x as f64))
            .collect()
    };
    Agg {
        rho: mean(c.rows.iter().map(|r| r.rho.ops as f64)),
        rho_neg: mean(c.rows.iter().map(|r| r.rho_neg.ops as f64)),
        ops_j: mean(c.rows.iter().map(|r| r.jc.ops as f64)),
        ops_e: mean(c.rows.iter().map(|r| r.e.ops as f64)),
        la_j: mean(c.rows.iter().map(|r| r.jc.la_mulmods as f64)),
        la_e: mean(c.rows.iter().map(|r| r.e.la_mulmods as f64)),
        fc_j: fc(&|r| r.jc.first_collision.as_ref().map(|x| x.ops)),
        fc_e: fc(&|r| r.e.first_collision.as_ref().map(|x| x.ops)),
        fc_b: fc(&|r| r.birthday.as_ref().map(|x| x.ops)),
        rho_all: c.rows.iter().map(|r| r.rho.ops as f64).collect(),
    }
}

fn svg(cells: &[Cell]) -> String {
    // per-prime means over instances
    let mut ps: Vec<u64> = cells.iter().map(|c| c.p).collect();
    ps.dedup();
    let series = |f: &dyn Fn(&Cell) -> f64| -> Vec<(u64, f64)> {
        ps.iter()
            .map(|&p| {
                let v: Vec<f64> = cells.iter().filter(|c| c.p == p).map(f).collect();
                (p, v.iter().sum::<f64>() / v.len() as f64)
            })
            .collect()
    };
    let data: Vec<(&str, &str, &str, Vec<(u64, f64)>)> = vec![
        (
            "#c0392b",
            "none",
            "IC on J(C), full stop (counted ops)",
            series(&|c| agg(c).ops_j),
        ),
        (
            "#2980b9",
            "2,3",
            "native E fibre arm (identical)",
            series(&|c| agg(c).ops_e),
        ),
        (
            "#8e44ad",
            "none",
            "J(C) IC at first collision (median)",
            series(&|c| median(agg(c).fc_j)),
        ),
        (
            "#27ae60",
            "none",
            "E birthday control, first collision (median)",
            series(&|c| median(agg(c).fc_b)),
        ),
        (
            "#2c3e50",
            "5,3",
            "matched rho on E (mean)",
            series(&|c| agg(c).rho),
        ),
        (
            "#7f8c8d",
            "1,3",
            "rho with negation map (mean)",
            series(&|c| agg(c).rho_neg),
        ),
    ];
    let (x0, x1) = (45f64.ln(), 300f64.ln());
    let (y0, y1) = (30f64.ln(), 30_000f64.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.max(30.0).ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"680\" height=\"440\" viewBox=\"0 0 680 440\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"680\" height=\"440\" fill=\"#fff\"/>\n");
    s.push_str("<text x=\"340\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Degree-3 cover: IC on J(C) is a collision search on E</text>\n");
    s.push_str("<text x=\"340\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">counted group operations, mean over instances and seeds; [pcc:unverified]</text>\n");
    for t in [100.0, 1000.0, 10000.0] {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
    }
    for t in [50.0, 100.0, 200.0] {
        let x = xp(t);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"360\" stroke=\"#e5e5e5\"/><text x=\"{x:.1}\" y=\"376\" text-anchor=\"middle\" fill=\"#444\">{t}</text>\n"));
    }
    s.push_str("<text x=\"340\" y=\"398\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"210\" text-anchor=\"middle\" transform=\"rotate(-90 22 210)\">group operations</text>\n");
    s.push_str(
        "<rect x=\"96\" y=\"64\" width=\"330\" height=\"112\" fill=\"#fff\" stroke=\"#ccc\"/>\n",
    );
    for (i, (col, dash, label, pts)) in data.iter().enumerate() {
        let xy: Vec<String> = pts
            .iter()
            .map(|&(p, v)| format!("{:.1},{:.1}", xp(p as f64), yp(v)))
            .collect();
        s.push_str(&format!("<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", xy.join(" ")));
        for pt in &xy {
            let (x, y) = pt.split_once(',').unwrap();
            s.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"{col}\"/>"
            ));
        }
        let ly = 80 + 17 * i;
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
    let primes: Vec<u64> = arg("--primes")
        .map(|s| s.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| PRIMES.to_vec());
    let instances: u64 = arg("--instances").and_then(|s| s.parse().ok()).unwrap_or(3);
    let seeds: u64 = arg("--seeds").and_then(|s| s.parse().ok()).unwrap_or(15);

    let results: Vec<Vec<Result<Cell, String>>> = std::thread::scope(|sc| {
        let hs: Vec<_> = primes
            .iter()
            .map(|&p| {
                let i = PRIMES.iter().position(|&q| q == p).unwrap_or(0) as u64;
                sc.spawn(move || run_prime(p, i, instances, seeds))
            })
            .collect();
        hs.into_iter()
            .map(|h| h.join().unwrap_or_else(|_| vec![Err("panic".into())]))
            .collect()
    });
    let mut cells: Vec<Cell> = vec![];
    let mut errors: Vec<String> = vec![];
    for rs in results {
        for r in rs {
            match r {
                Ok(c) => cells.push(c),
                Err(e) => errors.push(e),
            }
        }
    }
    for e in &errors {
        eprintln!("ERROR {e}");
    }

    // ── tables ──
    println!("## Cells\n");
    println!(
        "| p | j | r | #E | #J(C) | h1 | h0 | h tries | hom checks | build ms | skipped before |"
    );
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|");
    for c in &cells {
        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.1} | {} |",
            c.p,
            c.j,
            c.r,
            c.n_e,
            c.jac_order,
            c.h1,
            c.h0,
            c.h_attempts,
            c.hom_checks,
            c.build_ms,
            if c.skipped.is_empty() {
                "-".to_string()
            } else {
                c.skipped.join("; ")
            }
        );
    }
    println!("\n## Factor base B and fibres\n");
    println!("| p | j | #E | affine pts C | B1 places/E | B3 | B3/#E | B3 in <G> (k) | cols | hist 0/1/2/3 places over Q | ramified (split) | image fibre sizes 1/2/3 |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|--:|--|");
    for c in &cells {
        let b = &c.b;
        println!(
            "| {} | {} | {} | {} | {}/{} | {} | {:.4} | {} | {} | {}/{}/{}/{} | {} ({}) | {}/{}/{} |",
            c.p, c.j, b.n_e, b.c_affine_points, b.b1_from_places, b.b1_from_e, b.b3,
            b.b3 as f64 / b.n_e as f64, b.b3_in_g, b.b3_in_g_cols, b.places_hist[0],
            b.places_hist[1], b.places_hist[2], b.places_hist[3], b.ramified, b.ramified_split,
            b.image_places_hist[1], b.image_places_hist[2], b.image_places_hist[3]
        );
    }
    println!("\n## Relation classification and push-forward (sum over seeds)\n");
    println!("| p | j | seeds | trials | identity | class=c*(Q) | rows | single full fibre | union | other | ramified rows | pushed valid | invalid | not [3]Q | accept rate |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let (
        mut t_tr,
        mut t_id,
        mut t_eq,
        mut t_rows,
        mut t_sf,
        mut t_un,
        mut t_ot,
        mut t_ram,
        mut t_pv,
        mut t_pi,
        mut t_p3,
    ) = (
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    );
    let (mut all_same, mut all_cert, mut n_seedcells) = (true, true, 0u64);
    for c in &cells {
        let s = |f: &dyn Fn(&SeedRow) -> u64| c.rows.iter().map(f).sum::<u64>();
        let tr = s(&|r| r.jc.trials);
        let idn = s(&|r| r.jc.identity_trials);
        let eq = s(&|r| r.jc.class_eq_pullback);
        let rows = s(&|r| r.jc.relations);
        let sf = s(&|r| r.jc.single_full);
        let un = s(&|r| r.jc.union_full);
        let ot = s(&|r| r.jc.other);
        let ram = s(&|r| r.jc.rows_ramified);
        let pv = s(&|r| r.jc.push_valid);
        let pi = s(&|r| r.jc.push_invalid);
        let p3 = s(&|r| r.jc.push_not_3q);
        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} |",
            c.p,
            c.j,
            c.rows.len(),
            tr,
            idn,
            eq,
            rows,
            sf,
            un,
            ot,
            ram,
            pv,
            pi,
            p3,
            rows as f64 / (tr - idn).max(1) as f64
        );
        t_tr += tr;
        t_id += idn;
        t_eq += eq;
        t_rows += rows;
        t_sf += sf;
        t_un += un;
        t_ot += ot;
        t_ram += ram;
        t_pv += pv;
        t_pi += pi;
        t_p3 += p3;
        for r in &c.rows {
            n_seedcells += 1;
            all_same &= r.same_trials && r.same_ops && r.same_relations && r.same_log;
            all_cert &= r.jc.certified_j
                && r.jc.certified_e
                && r.e.certified
                && r.birthday_certified
                && r.rho.ok
                && r.rho_neg.ok
                && r.jc.reached_want;
        }
    }
    println!(
        "| all | - | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} |",
        n_seedcells,
        t_tr,
        t_id,
        t_eq,
        t_rows,
        t_sf,
        t_un,
        t_ot,
        t_ram,
        t_pv,
        t_pi,
        t_p3,
        t_rows as f64 / (t_tr - t_id).max(1) as f64
    );

    println!("\n## Arms (mean over seeds; counted group operations; LA = unpriced Z/r multiplications)\n");
    println!("| p | j | r | rho | rho+neg | IC J(C) ops | IC E ops | ledger ops J(C) | rows | trials | LA J(C) | LA E | same trials/ops/rows/log (seeds) | J(C)/rho @0 | @0.1 | @1 | E/rho @0 | @0.1 | @1 |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|--:|--:|--:|");
    for c in &cells {
        let a = agg(c);
        let same = c
            .rows
            .iter()
            .filter(|r| r.same_trials && r.same_ops && r.same_relations && r.same_log)
            .count();
        let ledger = mean(c.rows.iter().map(|r| r.jc.ledger_ops as f64));
        let rows = mean(c.rows.iter().map(|r| r.jc.relations as f64));
        let trials = mean(c.rows.iter().map(|r| r.jc.trials as f64));
        let g = |ops: f64, la: f64, w: f64| (ops + w * la) / a.rho;
        println!(
            "| {} | {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.0} | {:.0} | {}/{} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
            c.p, c.j, c.r, a.rho, a.rho_neg, a.ops_j, a.ops_e, ledger, rows, trials, a.la_j, a.la_e,
            same, c.rows.len(),
            g(a.ops_j, a.la_j, 0.0), g(a.ops_j, a.la_j, 0.1), g(a.ops_j, a.la_j, 1.0),
            g(a.ops_e, a.la_e, 0.0), g(a.ops_e, a.la_e, 0.1), g(a.ops_e, a.la_e, 1.0)
        );
    }
    println!("\n## First collision (median over seeds; counted group operations)\n");
    println!("| p | j | r | rho median | J(C) first collision | E fibre first collision | birthday control | seeds J(C) below rho (same seed) | seeds control below rho | control <= J(C) on every seed |");
    println!("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|");
    let (mut cells_fc_below, mut cells_ctrl_le) = (0usize, true);
    for c in &cells {
        let a = agg(c);
        let below_j = c
            .rows
            .iter()
            .filter(|r| {
                r.jc.first_collision
                    .as_ref()
                    .is_some_and(|x| x.ops < r.rho.ops)
            })
            .count();
        let below_b = c
            .rows
            .iter()
            .filter(|r| r.birthday.as_ref().is_some_and(|x| x.ops < r.rho.ops))
            .count();
        let ctrl_le = c
            .rows
            .iter()
            .all(|r| match (&r.birthday, &r.jc.first_collision) {
                (Some(b), Some(j)) => b.ops <= j.ops,
                _ => false,
            });
        cells_ctrl_le &= ctrl_le;
        let mj = median(a.fc_j.clone());
        let mr = median(a.rho_all.clone());
        if mj < mr {
            cells_fc_below += 1;
        }
        println!(
            "| {} | {} | {} | {:.0} | {:.0} | {:.0} | {:.0} | {}/{} | {}/{} | {} |",
            c.p,
            c.j,
            c.r,
            mr,
            mj,
            median(a.fc_e.clone()),
            median(a.fc_b.clone()),
            below_j,
            c.rows.len(),
            below_b,
            c.rows.len(),
            ctrl_le
        );
    }
    // verdict
    let single_ok = t_un == 0 && t_ot == 0 && t_sf == t_rows;
    let verdict = if single_ok && all_same && t_pi == 0 && t_p3 == 0 {
        "HOLDS: every relation is a single full fibre; J(C) and E arms identical; 0 invalid pushes"
    } else if !single_ok {
        "FALSIFIED: some relation is not a single full fibre"
    } else {
        "MECHANISM DIFFERS: fibres hold but arm equality or push validity fails"
    };
    println!(
        "\ncells: {}, seed-cells: {n_seedcells}, errors: {}",
        cells.len(),
        errors.len()
    );
    println!("all arms identical (trials, ops, rows, log): {all_same}");
    println!("all recovered logs certified and all runs reached their stop rule: {all_cert}");
    println!(
        "cells with J(C) first-collision median below rho median: {cells_fc_below} of {}",
        cells.len()
    );
    println!(
        "birthday control <= J(C) first collision on every seed of every cell: {cells_ctrl_le}"
    );
    println!("VERDICT: {verdict}");

    // ── json ──
    if let Some(path) = arg("--json") {
        let cj: Vec<Value> = cells
            .iter()
            .map(|c| {
                json!({
                    "p": c.p, "instance": c.j, "tower_seed": c.tower_seed, "skipped_before": c.skipped,
                    "r": c.r, "n_e": c.n_e, "jac_order": c.jac_order, "h1": c.h1, "h0": c.h0,
                    "h_attempts": c.h_attempts, "hom_checks": c.hom_checks, "build_ms": c.build_ms,
                    "b": c.b, "rows": c.rows,
                })
            })
            .collect();
        let doc = json!({
            "schema": "cover_pushforward_deg3/v1",
            "verdict": verdict,
            "errors": errors,
            "seeds": seeds,
            "totals": {
                "trials": t_tr, "identity_trials": t_id, "class_eq_pullback": t_eq, "relations": t_rows,
                "single_full": t_sf, "union_full": t_un, "other": t_ot, "ramified_rows": t_ram,
                "push_valid": t_pv, "push_invalid": t_pi, "push_not_3q": t_p3,
                "all_arms_identical": all_same, "all_certified": all_cert,
            },
            "cells": cj,
        });
        std::fs::write(path, serde_json::to_string_pretty(&doc).unwrap()).expect("write json");
    }
    if let Some(path) = arg("--svg") {
        std::fs::write(path, svg(&cells)).expect("write svg");
    }
    if errors.is_empty() && all_cert && !cells.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
