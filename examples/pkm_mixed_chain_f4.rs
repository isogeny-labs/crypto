//! One `m = 2` PKM relation system on a **mixed-degree** Vélu chain, handed to
//! the generic F4, to measure how the system size trades against the step
//! degree at a fixed composed degree.
//!
//! PKM (PKC 2016, §3.4) asks for a chain `L = L_n0 ∘ … ∘ L_1` with
//! `∏ deg L_j ≈ p^{1/m}` and every `deg L_j` small. The paper's Table 5, and
//! every run in `research/pkm_paper_faithful_20261124/`, take all steps of
//! degree 2, which costs `n0 = log2(deg L)` tower levels per summand. A larger
//! smoothness bound `B` buys fewer levels (`n0 ≈ log_B(deg L)`) at a higher
//! degree per equation. This binary builds the same relation system both ways
//! and reports what F4 does with it.
//!
//! Usage:
//! ```text
//! pkm_mixed_chain_f4 P S degrees CSR_SEED MAX_CURVES X_R DEGREE_BOUND BUDGET_S
//! ```
//! e.g. `pkm_mixed_chain_f4 1000003 1001 7,11,13 7 200000 12345 14 120`

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::pkm_smooth_tower as st;
use std::time::{Duration, Instant};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 9 {
        eprintln!("usage: pkm_mixed_chain_f4 P S degrees CSR_SEED MAX_CURVES X_R DEGREE_BOUND BUDGET_S");
        std::process::exit(2);
    }
    let p: u64 = a[1].parse().expect("p");
    let s: u64 = a[2].parse().expect("S");
    let degrees: Vec<u64> = a[3].split(',').map(|x| x.parse().expect("degree")).collect();
    let seed: u64 = a[4].parse().expect("seed");
    let max_curves: u64 = a[5].parse().expect("max_curves");
    let xr: u64 = a[6].parse().expect("xr");
    let deg_bound: u32 = a[7].parse().expect("degree_bound");
    let budget: u64 = a[8].parse().expect("budget");

    let prod: u64 = degrees.iter().product();
    assert_eq!(prod, s, "degrees do not multiply to S");

    let t_search = Instant::now();
    let found = st::search_aux_exact(p, s, seed, max_curves).expect("aux search");
    let search_s = t_search.elapsed().as_secs_f64();
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

    // The DLP curve: any curve over the same field fixes S_3's coefficients.
    let target = st::Curve { p, a: 1, b: 3 };
    let (polys, n_vars) = st::relation_system_polys(&target, &tower, 2, xr);
    let input: Vec<Poly> = polys
        .iter()
        .map(|t| f4_fp::normalise(t, p, Ordering::Grevlex))
        .filter(|f| !f.is_empty())
        .collect();
    let max_deg: u32 = degrees.iter().copied().max().unwrap().max(4) as u32;

    let opts = F4Options::new(Ordering::Grevlex, deg_bound)
        .with_budget(Duration::from_secs(budget));
    let t0 = Instant::now();
    let rep = f4_fp::f4(&input, n_vars, p, &opts);
    let wall = t0.elapsed().as_secs_f64();

    // 0-dimensional: every variable carries a pure power in the basis.
    let pure = |k: usize| {
        rep.basis.iter().any(|f| {
            f.first()
                .map(|(e, _)| e.iter().enumerate().all(|(i, &x)| (i == k) == (x > 0)))
                .unwrap_or(false)
        })
    };
    let zero_dim = rep.inconsistent || (0..n_vars).all(pure);

    let json = format!(
        "{{\"p\":{p},\"S\":{s},\"degrees\":\"{deg}\",\"n0\":{n0},\"n_vars\":{nv},\
\"max_eq_degree\":{md},\"degree_bound\":{db},\"solving_degree_max\":{sd},\
\"degree_reached\":{dr},\"max_cols\":{mc},\"max_rows\":{mr},\"steps\":{stp},\
\"inconsistent\":{inc},\"timed_out\":{to},\"pairs_above_bound\":{pab},\"zero_dim\":{zd},\"basis_len\":{bl},\"ms\":{ms:.1},\"wall_s\":{wall:.1},\
\"aux_curves_tried\":{tried},\"aux_order\":{order},\"search_s\":{search:.1}}}",
        deg = a[3],
        n0 = degrees.len(),
        nv = n_vars,
        md = max_deg,
        db = deg_bound,
        sd = rep.solving_degree_max,
        dr = rep.degree_reached,
        mc = rep.max_cols,
        mr = rep.max_rows,
        stp = rep.steps,
        inc = rep.inconsistent,
        to = rep.timed_out,
        pab = rep.pairs_above_bound,
        zd = zero_dim,
        bl = rep.basis.len(),
        ms = rep.ms,
        tried = found.curves_tried,
        order = tower.size(),
        search = search_s,
    );
    println!("{json}");
}
