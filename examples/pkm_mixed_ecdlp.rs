//! PKM's Algorithm 1 end to end on a **mixed-degree** Vélu chain (`B > 2`).
//!
//! `research/pkm_paper_faithful_20261124/` runs Algorithm 1 on the paper's
//! 2-power chains only (`B = 2`). This driver runs the same algorithm on a
//! chain of small primes `q_1, …, q_n0` with `∏ q_j ≈ p^{1/m}`:
//!
//! 1. `search_aux_exact` finds `E'/F_p` with a point `Q` of order `S = ∏ q_j`;
//! 2. `isogeny_tower_on` builds the Vélu chain and `V = x(R' + ⟨Q⟩)`;
//! 3. the factor base is every point of the target curve whose abscissa is in
//!    `V`, one column per abscissa;
//! 4. for random `a, b`: `R = aP + bQ`, `algebraic_decompose` solves
//!    `S_3(x_1, x_2, X)` plus the chain (the paper's system (4)) and returns
//!    every checked relation `P_1 + P_2 = R` with `x(P_i) ∈ V`;
//! 5. Gaussian elimination mod `r` recovers `k` from `L_1 + L_2 = a + b k`.
//!
//! Usage:
//! ```text
//! pkm_mixed_ecdlp P S degrees SEED MAX_CURVES A B GX GY R K DMAX BUDGET_S MAX_TRIALS
//! ```
//! e.g. `pkm_mixed_ecdlp 1048291 1001 7,11,13 7 400000 1 11 1 123004 1046999 12345 20 60 4000`

use crypto_lib::cryptanalysis::pkm_smooth_tower as st;
use std::time::{Duration, Instant};

fn main() {
    let av: Vec<String> = std::env::args().collect();
    if av.len() != 15 {
        eprintln!(
            "usage: pkm_mixed_ecdlp P S degrees SEED MAX_CURVES A B GX GY R K DMAX BUDGET_S MAX_TRIALS"
        );
        std::process::exit(2);
    }
    let p: u64 = av[1].parse().unwrap();
    let s: u64 = av[2].parse().unwrap();
    let degrees: Vec<u64> = av[3].split(',').map(|x| x.parse().unwrap()).collect();
    let seed: u64 = av[4].parse().unwrap();
    let max_curves: u64 = av[5].parse().unwrap();
    let (a, b): (u64, u64) = (av[6].parse().unwrap(), av[7].parse().unwrap());
    let (gx, gy): (u64, u64) = (av[8].parse().unwrap(), av[9].parse().unwrap());
    let r: u64 = av[10].parse().unwrap();
    let k: u64 = av[11].parse().unwrap();
    let dmax: u32 = av[12].parse().unwrap();
    let budget: u64 = av[13].parse().unwrap();
    let max_trials: u64 = av[14].parse().unwrap();
    assert_eq!(degrees.iter().product::<u64>(), s, "degrees do not multiply to S");

    let e = st::Curve { p, a, b };
    let g = (gx, gy);
    assert!(e.on_curve(Some(g)), "generator not on E");
    assert_eq!(e.mul(r, Some(g)), None, "generator does not have order r");
    let q_pt = e.mul(k % r, Some(g)).expect("Q = kP");

    let t_start = Instant::now();
    let found = st::search_aux_exact(p, s, seed, max_curves).expect("aux search");
    let tower = st::isogeny_tower_on(
        found.aux,
        found.order,
        found.q,
        &degrees,
        seed,
        found.curves_tried,
    )
    .expect("tower build");
    tower.verify().expect("tower verify");

    // Factor base: one column per abscissa of V that lifts to E.
    let mut cols: Vec<(u64, u64)> = Vec::new();
    for &x in &tower.v {
        if let Some(y) = st::sqrtm(e.rhs(x), p) {
            cols.push((x, y));
        }
    }
    let c = cols.len();
    let setup_s = t_start.elapsed().as_secs_f64();

    // Relation collection: rows over [L_1 .. L_c, k], RHS a, since
    // sum(c_i L_i) = a + b k.
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut rng = st::Rng(seed ^ 0x9e37_79b9_7f4a_7c15);
    let (mut trials, mut found_rel, mut undetermined) = (0u64, 0u64, 0u64);
    let t_rel = Instant::now();
    while rows.len() < c + 1 && trials < max_trials {
        trials += 1;
        let (aa, bb) = (rng.below(r), rng.below(r));
        let Some(rr) = e.add(e.mul(aa, Some(g)), e.mul(bb, Some(q_pt))) else {
            continue;
        };
        let dec = st::algebraic_decompose(&e, &tower, 2, rr, dmax, Duration::from_secs(budget));
        if dec.undetermined {
            undetermined += 1;
            continue;
        }
        for rel in &dec.relations {
            let mut row = vec![0u64; c + 2]; // c coefficients, k, rhs
            let mut ok = true;
            for &(x, y) in rel {
                match cols.iter().position(|&(cx, _)| cx == x) {
                    Some(i) => {
                        let coef = if y == cols[i].1 { 1 } else { r - 1 };
                        row[i] = (row[i] + coef) % r;
                    }
                    None => ok = false,
                }
            }
            if !ok {
                continue;
            }
            row[c] = (r - bb % r) % r; // L1 + L2 - b k = a
            row[c + 1] = aa % r;
            rows.push(row);
            found_rel += 1;
        }
    }
    let rel_s = t_rel.elapsed().as_secs_f64();

    // Gauss-Jordan mod r.
    let ncol = c + 1;
    let mut pivot_row: Vec<Option<usize>> = vec![None; ncol];
    let mut rank = 0usize;
    let mut inconsistent = false;
    for i in 0..rows.len() {
        // Reduce against the existing pivots first; the lead is what remains.
        for pr in 0..ncol {
            if let Some(pj) = pivot_row[pr] {
                let f = rows[i][pr];
                if f != 0 {
                    for j in pr..=ncol {
                        let v = st::mulm(f, rows[pj][j], r);
                        rows[i][j] = st::subm(rows[i][j], v, r);
                    }
                }
            }
        }
        let mut lead = None;
        for col in 0..ncol {
            if rows[i][col] % r != 0 {
                lead = Some(col);
                break;
            }
        }
        let Some(col) = lead else {
            if rows[i][ncol] % r != 0 {
                inconsistent = true;
            }
            continue;
        };
        let inv = st::invm(rows[i][col], r);
        for j in col..=ncol {
            rows[i][j] = st::mulm(rows[i][j], inv, r);
        }
        let cur = rows[i].clone();
        for pr in 0..ncol {
            if let Some(pj) = pivot_row[pr] {
                let f = rows[pj][col];
                if f != 0 {
                    for j in col..=ncol {
                        let v = st::mulm(f, cur[j], r);
                        rows[pj][j] = st::subm(rows[pj][j], v, r);
                    }
                }
            }
        }
        pivot_row[col] = Some(i);
        rank += 1;
        if pivot_row[ncol - 1].is_some() {
            break;
        }
    }
    let recovered = pivot_row[ncol - 1].map(|i| rows[i][ncol] % r);
    let verified = recovered.map(|d| e.mul(d, Some(g)) == Some(q_pt)).unwrap_or(false);

    println!(
        "{{\"p\":{p},\"S\":{s},\"degrees\":\"{}\",\"n0\":{},\"B\":{},\"cols\":{c},\
\"trials\":{trials},\"relations\":{found_rel},\"undetermined\":{undetermined},\
\"rank\":{rank},\"inconsistent\":{inconsistent},\"k\":{k},\"recovered\":{},\
\"verified\":{verified},\"setup_s\":{setup_s:.1},\"relation_s\":{rel_s:.1},\
\"aux_curves_tried\":{},\"dmax\":{dmax},\"budget_s\":{budget}}}",
        av[3],
        degrees.len(),
        degrees.iter().copied().max().unwrap(),
        recovered.map(|d| d.to_string()).unwrap_or_else(|| "null".into()),
        found.curves_tried,
    );
}
