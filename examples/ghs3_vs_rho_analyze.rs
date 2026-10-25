//! Analysis of `research/ghs3_vs_rho_20261023/` (PROTOCOL.md there): reads the frozen
//! `raw/b<b>_c<c>.jsonl` records, checks gate 0 against round 4's records, the rho
//! calibration and the symmetry check, and writes `summary.json` and `TABLES.md`: per
//! instance IC / R_ideal and IC / R_meas at c_ell = 1 and 9, the §5a band, the per-size
//! decision and the fitted crossover with a bootstrap interval over curves.
//!
//! ```sh
//! cargo run --release --example ghs3_vs_rho_analyze -- research/ghs3_vs_rho_20261023
//! ```

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::fmt::Write as _;

const CAND: &str = "+C1C2";
const ROUND4: &str = "research/ic_improve4_20261015/01-genus3-relations/results/run";
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

struct Inst {
    b: u32,
    curve: u32,
    ell: f64,
    ic: f64,
    ic9: f64,
    ic_hash: f64,
    gae: f64,
    r_ideal_ops: f64,
    r_meas_ops: f64,
    rho_canon: f64,
    rho_dp: f64,
    rho_runs: usize,
    rho_correct: bool,
    ic_correct: bool,
    gate0: bool,
    j_in_fq: bool,
    r_meas_unbatched: f64,
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}

fn ols(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let slope = sxy / sxx;
    (slope, my - slope * mx)
}

fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let mut insts = vec![];
    for b in 14..=20u32 {
        for c in 0..3u32 {
            let path = format!("{dir}/raw/b{b}_c{c}.jsonl");
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
            let rec: Value = serde_json::from_str(text.lines().next().unwrap()).unwrap();
            let res = rec["results"].as_array().unwrap();
            let cand = res
                .iter()
                .find(|r| r["arm"].as_str().unwrap().ends_with(CAND))
                .expect("candidate arm");
            // gate 0: every arm's cost equals round 4's record
            let old_path = format!("{ROUND4}/b{b}_c{c}_w.jsonl");
            let old: Value = serde_json::from_str(
                std::fs::read_to_string(&old_path)
                    .unwrap_or_else(|e| panic!("{old_path}: {e}"))
                    .lines()
                    .next()
                    .unwrap(),
            )
            .unwrap();
            let olds = old["results"].as_array().unwrap();
            let gate0 = olds.len() == res.len()
                && olds.iter().zip(res).all(|(a, b)| {
                    a["arm"] == b["arm"] && a["cost"] == b["cost"] && a["correct"] == b["correct"]
                });
            let rho = &rec["rho"];
            let runs = rho["runs"].as_array().unwrap();
            let gae = f(&rho["price_batched_per_op"]);
            let nr = runs.len() as f64;
            let mean = |k: &str| runs.iter().map(|r| f(&r[k])).sum::<f64>() / nr;
            let lib = |k: &str| {
                runs.iter()
                    .map(|r| r["lib_counters"][k].as_f64().unwrap_or(0.0))
                    .sum::<f64>()
                    / nr
            };
            insts.push(Inst {
                b,
                curve: c,
                ell: f(&rec["ell"]),
                ic: f(&cand["cost"]),
                ic9: f(&cand["cost_cl9"]),
                ic_hash: f(&cand["hash_ops_uncharged"]),
                gae,
                r_ideal_ops: f(&rho["ideal_negation_ops"]),
                r_meas_ops: mean("group_ops"),
                rho_canon: lib("canonicalisations_uncharged"),
                rho_dp: lib("distinguished_points"),
                rho_runs: runs.len(),
                rho_correct: runs.iter().all(|r| r["correct"] == json!(true)),
                ic_correct: res.iter().all(|r| r["correct"] == json!(true)),
                gate0,
                j_in_fq: rec["checks"]["j_E0_in_Fq"] == json!(true),
                r_meas_unbatched: mean("cost_unbatched"),
            });
        }
    }
    let gate0 = insts.iter().all(|i| i.gate0);
    let correct = insts.iter().all(|i| i.ic_correct && i.rho_correct);
    let mut sizes = vec![];
    let mut md = String::new();
    writeln!(
        md,
        "# Tables: genus-3 GHS IC against the strong rho on E0(F_{{q^3}})\n"
    )
    .unwrap();
    writeln!(md, "Gate 0 (IC costs equal round 4's): **{}**. All answers correct: **{}**. Instances with j(E0) in F_q: **{}**.\n",
        gate0, correct, insts.iter().filter(|i| i.j_in_fq).count()).unwrap();
    writeln!(md, "Costs in `M_q + ceil(log2 q) I_q + M_ell`. `GAE` = batched Karatsuba price of one rho group operation. `R_ideal = sqrt(pi ell/4) GAE`; `R_meas` = mean measured group operations x GAE over the curve's rho targets.\n").unwrap();
    writeln!(md, "| log2 q | curve | log2 ell | IC (+C1C2) | GAE | R_ideal | R_meas | IC/R_ideal | IC/R_meas | IC/R_ideal (c_ell=9) | IC/R_ideal w=0.1 | IC/R_ideal w=1 | rho ops/ideal |").unwrap();
    writeln!(
        md,
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    )
    .unwrap();
    let mut rows = vec![];
    for i in &insts {
        let ri = i.r_ideal_ops * i.gae;
        let rm = i.r_meas_ops * i.gae;
        let band = |w: f64| {
            let ic = i.ic + w * i.ic_hash * i.gae;
            let ri_w = ri + w * i.r_ideal_ops * i.gae;
            let rm_w = rm + w * (i.rho_canon + 2.0 * i.rho_dp) * i.gae;
            (ic / ri_w, ic / rm_w)
        };
        let bands: Vec<(f64, f64)> = WEIGHTS.iter().map(|&w| band(w)).collect();
        writeln!(
            md,
            "| {} | {} | {:.2} | {:.4e} | {:.2} | {:.4e} | {:.4e} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |",
            i.b, i.curve, i.ell.log2(), i.ic, i.gae, ri, rm, i.ic / ri, i.ic / rm, i.ic9 / ri,
            bands[1].0, bands[2].0, i.r_meas_ops / i.r_ideal_ops
        )
        .unwrap();
        rows.push(json!({
            "bits": i.b, "curve": i.curve, "ell": i.ell, "ic_cost": i.ic, "ic_cost_cl9": i.ic9,
            "ic_hash_ops": i.ic_hash, "gae": i.gae, "r_ideal": ri, "r_meas": rm,
            "r_meas_unbatched_instrument": i.r_meas_unbatched,
            "rho_runs": i.rho_runs, "rho_ops_over_ideal": i.r_meas_ops / i.r_ideal_ops,
            "ratio_ideal": i.ic / ri, "ratio_meas": i.ic / rm, "ratio_ideal_cl9": i.ic9 / ri,
            "band": WEIGHTS.iter().zip(&bands).map(|(w, (a, b))| json!({"w": w, "ratio_ideal": a, "ratio_meas": b})).collect::<Vec<_>>(),
            "gate0": i.gate0, "j_in_fq": i.j_in_fq,
        }));
    }
    writeln!(md, "\n## Per-size decision (PROTOCOL.md rule)\n").unwrap();
    writeln!(md, "| log2 q | rho calibration (pooled ops/ideal) | calibration ok | geomean IC/R_ideal | max IC/R_ideal | max IC/R_meas | decision | c_ell = 9: max IC/R_ideal |").unwrap();
    writeln!(md, "|---:|---:|:--|---:|---:|---:|:--|---:|").unwrap();
    for b in 14..=20u32 {
        let s: Vec<&Inst> = insts.iter().filter(|i| i.b == b && !i.j_in_fq).collect();
        let tot_ops: f64 = s.iter().map(|i| i.r_meas_ops * i.rho_runs as f64).sum();
        let tot_ideal: f64 = s.iter().map(|i| i.r_ideal_ops * i.rho_runs as f64).sum();
        let calib = tot_ops / tot_ideal;
        let calib_ok = (0.85..=1.30).contains(&calib);
        let mut below_all = true;
        let mut above_all = true;
        let (mut mx_i, mut mx_m, mut mx9) = (0f64, 0f64, 0f64);
        for i in &s {
            for &w in &WEIGHTS {
                let ri = (i.r_ideal_ops + w * i.r_ideal_ops) * i.gae;
                let rm = (i.r_meas_ops + w * (i.rho_canon + 2.0 * i.rho_dp)) * i.gae;
                let ic = i.ic + w * i.ic_hash * i.gae;
                let below = ic < ri && (!calib_ok || ic < rm);
                let above = ic > ri && (!calib_ok || ic > rm);
                below_all &= below;
                above_all &= above;
            }
            mx_i = mx_i.max(i.ic / (i.r_ideal_ops * i.gae));
            mx_m = mx_m.max(i.ic / (i.r_meas_ops * i.gae));
            mx9 = mx9.max(i.ic9 / (i.r_ideal_ops * i.gae));
        }
        let geo = (s
            .iter()
            .map(|i| (i.ic / (i.r_ideal_ops * i.gae)).ln())
            .sum::<f64>()
            / s.len() as f64)
            .exp();
        let decision = if !(gate0 && correct) {
            "withheld (gate 0 or correctness)"
        } else if below_all {
            "IC below the strong rho"
        } else if above_all {
            "IC above rho"
        } else {
            "unresolved"
        };
        writeln!(md, "| {b} | {calib:.3} | {calib_ok} | {geo:.3} | {mx_i:.3} | {mx_m:.3} | {decision} | {mx9:.3} |").unwrap();
        sizes.push(
            json!({"bits": b, "rho_calibration": calib, "calibration_ok": calib_ok,
            "geomean_ratio_ideal": geo, "max_ratio_ideal": mx_i, "max_ratio_meas": mx_m,
            "max_ratio_ideal_cl9": mx9, "decision": decision}),
        );
    }
    // crossover: OLS of log2(IC/R_ideal) on b, bootstrap curves within size
    let pts: Vec<(f64, f64)> = insts
        .iter()
        .filter(|i| !i.j_in_fq)
        .map(|i| (i.b as f64, (i.ic / (i.r_ideal_ops * i.gae)).log2()))
        .collect();
    let (xs, ys): (Vec<f64>, Vec<f64>) = pts.iter().cloned().unzip();
    let (slope, icpt) = ols(&xs, &ys);
    let root = -icpt / slope;
    let mut rng = StdRng::seed_from_u64(20261023);
    let mut roots = vec![];
    let mut slopes = vec![];
    for _ in 0..10_000 {
        let mut bx = vec![];
        let mut by = vec![];
        for b in 14..=20u32 {
            let s: Vec<&(f64, f64)> = pts.iter().filter(|p| p.0 as u32 == b).collect();
            for _ in 0..s.len() {
                let p = s[rng.gen_range(0..s.len())];
                bx.push(p.0);
                by.push(p.1);
            }
        }
        let (sl, ic) = ols(&bx, &by);
        slopes.push(sl);
        roots.push(-ic / sl);
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |v: &Vec<f64>, p: f64| v[((v.len() as f64 - 1.0) * p).round() as usize];
    writeln!(md, "\n## Crossover against R_ideal\n").unwrap();
    writeln!(md, "OLS of log2(IC / R_ideal) on log2 q over {} instances: slope {:.4} [{:.4}, {:.4}] per bit; root log2 q = **{:.2}** [{:.2}, {:.2}] (95% bootstrap over curves within size, 10,000 resamples).",
        pts.len(), slope, q(&slopes, 0.025), q(&slopes, 0.975), root, q(&roots, 0.025), q(&roots, 0.975)).unwrap();
    // POST HOC (not in PROTOCOL.md): IC cost depends on q only, rho on ell = N / c, so the
    // ratio carries the cofactor c. Group by c and regress log2 ratio on (log2 q, log2 c).
    writeln!(
        md,
        "\n## Post hoc (not preregistered): the ratio by cofactor\n"
    )
    .unwrap();
    writeln!(md, "IC cost depends on `q` alone; rho on `ell = #E0 / c`. So `IC / R_ideal` carries `sqrt(c)`. Labelled post hoc; it moves no preregistered decision.\n").unwrap();
    writeln!(md, "| cofactor c | log2 q | curve | IC/R_ideal |").unwrap();
    writeln!(md, "|---:|---:|---:|---:|").unwrap();
    let mut by_c: Vec<(u64, u32, u32, f64, f64)> = insts
        .iter()
        .map(|i| {
            let n = i.ell * (2f64.powi(3 * i.b as i32) / i.ell).round();
            let c = (n / i.ell).round() as u64;
            (c, i.b, i.curve, i.ic / (i.r_ideal_ops * i.gae), i.ell)
        })
        .collect();
    by_c.sort_by_key(|a| (a.0, a.1, a.2));
    for (c, b, cu, r, _) in &by_c {
        writeln!(md, "| {c} | {b} | {cu} | {r:.3} |").unwrap();
    }
    // OLS of log2 ratio on (b, log2 c) by normal equations
    let rows3: Vec<(f64, f64, f64)> = insts
        .iter()
        .map(|i| {
            let c = 2f64.powi(3 * i.b as i32) / i.ell;
            (
                i.b as f64,
                c.log2(),
                (i.ic / (i.r_ideal_ops * i.gae)).log2(),
            )
        })
        .collect();
    let n = rows3.len() as f64;
    let (mx, mc, my) = (
        rows3.iter().map(|r| r.0).sum::<f64>() / n,
        rows3.iter().map(|r| r.1).sum::<f64>() / n,
        rows3.iter().map(|r| r.2).sum::<f64>() / n,
    );
    let (mut sxx, mut scc, mut sxc, mut sxy, mut scy) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for r in &rows3 {
        let (dx, dc, dy) = (r.0 - mx, r.1 - mc, r.2 - my);
        sxx += dx * dx;
        scc += dc * dc;
        sxc += dx * dc;
        sxy += dx * dy;
        scy += dc * dy;
    }
    let det = sxx * scc - sxc * sxc;
    let bq = (scc * sxy - sxc * scy) / det;
    let bc = (sxx * scy - sxc * sxy) / det;
    let a0 = my - bq * mx - bc * mc;
    let root_c4 = -(a0 + bc * 2.0) / bq;
    writeln!(md, "\nOLS `log2(IC/R_ideal) = a + b_q log2 q + b_c log2 c`: b_q = **{bq:.4}** per bit (theory 4/3 - 3/2 = -0.1667 per bit of `q` at fixed `c`), b_c = **{bc:.3}** (theory 0.5); a = {a0:.3}. At c = 4 the fitted ratio is 1 at log2 q = {root_c4:.2}. Point estimates only (21 instances).").unwrap();
    let summary = json!({
        "post_hoc_cofactor_fit": {"b_q": bq, "b_c": bc, "a": a0, "root_log2_q_at_c4": root_c4, "label": "post hoc, not preregistered"},
        "schema": "ghs3-vs-rho-summary/v1", "protocol": "research/ghs3_vs_rho_20261023/PROTOCOL.md",
        "candidate_arm": format!("w:lpsgebw2@cx{CAND}"),
        "gate0_identical_to_round4": gate0, "all_correct": correct,
        "instances": rows, "sizes": sizes,
        "crossover": {"slope_log2_ratio_per_bit": slope, "slope_ci95": [q(&slopes, 0.025), q(&slopes, 0.975)],
            "root_log2_q": root, "root_ci95": [q(&roots, 0.025), q(&roots, 0.975)], "bootstrap": 10000, "seed": 20261023},
        "weights_gae": WEIGHTS,
    });
    std::fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    std::fs::write(format!("{dir}/TABLES.md"), md).unwrap();
    println!("gate0 {gate0} correct {correct} root {root:.2} slope {slope:.4}");
}
