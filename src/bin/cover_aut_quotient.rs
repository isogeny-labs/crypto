//! Automorphism quotient of the Joux-Vitse cover factor base (research/cover-aut-quotient_20261115).
//!
//! Commands (see PLAN.md of the study):
//!   census-inst   --sizes 503,1009 --seeds 3          exhaustive Moebius census on generate_spec instances
//!   census-random --p 101 --n 4000 [--kummer]         census on random (or Kummer) alpha at small p
//!   control                                           detector controls (planted symmetric h)
//!   paired --p 1301 --seed 1 [--naive] --out f.json    Nagao arm: baseline rows vs orbit-quotient rows
//! Local development check; every number is [pcc:unverified].

#[path = "../../research/cover-aut-quotient_20261115/src/aut.rs"]
mod aut;
#[path = "../../research/cover-aut-quotient_20261115/src/orbit.rs"]
#[expect(
    clippy::manual_is_multiple_of,
    reason = "archived orbit module is included by this maintained binary"
)]
mod orbit;

use std::collections::HashMap;
use std::time::Instant;

use crypto_lib::cryptanalysis::f4_fp;
use crypto_lib::cryptanalysis::gaudry_cubic::{square_core, wiedemann_u64, SparseRel};
use crypto_lib::cryptanalysis::jv_cover::{
    factor_base, generate_spec, nagao_decompose, opts_for_mode, search_fb_line, unit_costs, Cover,
    Ctx, Div, EllE, Fld, Fq3, Hyp, StopMode, E2, E6,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::json;

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("census-inst") => census_inst(&args),
        Some("census-random") => census_random(&args),
        Some("control") => control(),
        Some("paired") => paired(&args),
        _ => eprintln!("usage: cover_aut_quotient census-inst|census-random|control|paired ..."),
    }
}

fn census_json(c: &aut::AutCensus) -> serde_json::Value {
    json!({
        "rational_finite_roots": c.rational_finite_roots,
        "linear_total_incl_identity": c.linear_total,
        "linear_nontrivial": c.linear_nontrivial,
        "linear_nontrivial_square_kappa": c.linear_nontrivial_square_kappa,
        "linear_nontrivial_fp_line": c.linear_nontrivial_fp_line,
        "semilinear_total": c.semilinear_total,
        "semilinear_square_kappa": c.semilinear_square_kappa,
        "semilinear_fp_line": c.semilinear_fp_line,
    })
}

fn census_inst(args: &[String]) {
    let sizes: Vec<u64> = args
        .iter()
        .position(|a| a == "--sizes")
        .map(|i| args[i + 1].split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![503, 1009, 1301, 1451, 1601]);
    let seeds: u64 = arg(args, "--seeds", 3);
    for &p in &sizes {
        for seed in 1..=seeds {
            let t = Instant::now();
            let spec = generate_spec(p, seed);
            let f3 = Fq3::new(p);
            let cov = Cover::new(&f3, &spec.alpha).expect("cover");
            let jac = Hyp {
                f: &f3.f,
                h: cov.hx.clone(),
                g: 3,
            };
            // iota acts as lambda = -1 on <Phi G>
            let neg_gj = jac.neg(&spec.gj);
            let iota_lambda_ok = neg_gj == jac.mul(&spec.gj, (spec.l - 1) as u128);
            let neg_qj = jac.neg(&spec.qj);
            let iota_lambda_ok_q = neg_qj == jac.mul(&spec.qj, (spec.l - 1) as u128);
            let c = aut::census(&f3.f, &cov.hx);
            println!(
                "{}",
                json!({"p": p, "seed": seed, "l": spec.l, "deg_h": cov.hx.len() - 1,
                       "iota_lambda_minus1_on_G": iota_lambda_ok, "iota_lambda_minus1_on_Q": iota_lambda_ok_q,
                       "lambda_iota": spec.l - 1,
                       "census": census_json(&c), "wall_s": t.elapsed().as_secs_f64()})
            );
        }
    }
}

fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % m as u128) as u64;
        }
        b = (b as u128 * b as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

fn census_random(args: &[String]) {
    let p: u64 = arg(args, "--p", 101);
    let n: u64 = arg(args, "--n", 1000);
    let kummer = flag(args, "--kummer");
    let f3 = Fq3::new(p);
    let mut rng = StdRng::seed_from_u64(arg(args, "--seed", 1u64));
    let omega = if kummer {
        assert!(p % 3 == 1, "Kummer control needs p = 1 mod 3");
        let g = (2..p).find(|&g| pow_mod(g, (p - 1) / 3, p) != 1).unwrap();
        Some(f3.from_fq(f3.f.from_fp(pow_mod(g, (p - 1) / 3, p))))
    } else {
        None
    };
    let (mut tried, mut with_nontrivial, mut with_semi, mut covers) = (0u64, 0u64, 0u64, 0u64);
    let mut nontriv_total = 0usize;
    let mut j_in_fp = 0u64;
    let mut details = Vec::new();
    while tried < n {
        let mut alpha = f3.random(&mut rng);
        if let Some(w) = omega {
            // eigenvector of sigma: sum omega^{-i} sigma^i beta, so sigma alpha = omega alpha
            let wi = f3.inv(&w);
            let s1 = f3.sigma(&alpha);
            let s2 = f3.sigma(&s1);
            alpha = f3.add(
                &f3.add(&alpha, &f3.mul(&wi, &s1)),
                &f3.mul(&f3.mul(&wi, &wi), &s2),
            );
            assert_eq!(f3.sigma(&alpha), f3.mul(&w, &alpha));
        }
        if alpha.in_fq() {
            continue;
        }
        let Some(cov) = Cover::new(&f3, &alpha) else {
            continue;
        };
        covers += 1;
        tried += 1;
        if let Some(w) = omega {
            // Kummer class: a2^2/a4 = (1+w)^2/w lies in F_p, so j(E) in F_p and E is a twist of a
            // curve over F_p; its group over F_(p^6) is then divisible by the order of an F_p curve.
            let ec = EllE::new(&f3, &alpha);
            let ratio = f3.mul(&f3.mul(&ec.a2, &ec.a2), &f3.inv(&ec.a4));
            let one_w = f3.add(&f3.from_fq(f3.f.from_fp(1)), &w);
            let expect = f3.mul(&f3.mul(&one_w, &one_w), &f3.inv(&w));
            if ratio == expect {
                j_in_fp += 1;
            }
        }
        if cov.hx.len() != 8 {
            continue;
        }
        let c = aut::census(&f3.f, &cov.hx);
        nontriv_total += c.linear_nontrivial;
        if c.linear_nontrivial > 0 {
            with_nontrivial += 1;
            if details.len() < 5 {
                details.push(census_json(&c));
            }
        }
        if c.semilinear_total > 0 {
            with_semi += 1;
        }
    }
    println!(
        "{}",
        json!({"p": p, "n_alpha": tried, "kummer": kummer, "covers": covers,
               "with_nontrivial_linear_aut": with_nontrivial, "nontrivial_linear_total": nontriv_total,
               "with_semilinear": with_semi, "kummer_j_in_fp": j_in_fp, "examples": details})
    );
}

fn control() {
    let p = 1301u64;
    let f = crypto_lib::cryptanalysis::jv_cover::Fq::new(p);
    let mut h = vec![E2::ZERO; 8];
    h[7] = E2::ONE;
    h[1] = f.from_fp(5);
    let lin = aut::search(&f, &h, &h);
    let nonid = lin.iter().filter(|m| !m.is_identity).count();
    // a random monic septic of the same field: identity only
    let mut rng = StdRng::seed_from_u64(99);
    let mut g: Vec<E2> = (0..7).map(|_| f.random(&mut rng)).collect();
    g.push(E2::ONE);
    let rnd = aut::census(&f, &g);
    println!(
        "{}",
        json!({"p": p, "planted_x7_plus_5x": {"linear_total": lin.len(), "nontrivial": nonid,
               "fp_line_preserving": lin.iter().filter(|m| m.preserves_fp_line).count(),
               "expected_total": 6},
               "random_septic": census_json(&rnd)})
    );
}

// ---------------------------------------------------------------------------------------------
// paired Nagao arm: baseline rows (A), orbit-quotient rows (Q), optional unquotiented arm (N)

#[derive(Clone, Debug, Default)]
struct Snap {
    residuals: u64,
    relations: u64,
    decompositions: u64,
    weil: u64,
    f4: u64,
    f4_native: u64,
    lin: u64,
    post: u64,
    stream: u64,
    verify: u64,
    la_ops: u64,
    unknowns_core: usize,
    filtered_out: usize,
    la_attempts: u64,
    wall_s: f64,
}

struct Arm {
    rows: Vec<SparseRel>,
    keep: usize,
    n_unknowns: usize,
    next_attempt: usize,
    la_ops: u64,
    attempts: u64,
    solved: Option<(u64, usize, usize, u64)>, // d, core size, filtered, la_ops of last attempt
}

impl Arm {
    fn new(keep: usize) -> Arm {
        Arm {
            rows: Vec::new(),
            keep,
            n_unknowns: keep + 1,
            next_attempt: keep + 1,
            la_ops: 0,
            attempts: 0,
            solved: None,
        }
    }
    /// The library's schedule: try when the row count reaches `next_attempt`.
    fn try_solve(&mut self, l: u64, rng: &mut StdRng, verify: &dyn Fn(u64) -> bool) {
        if self.solved.is_some() || self.rows.len() < self.next_attempt {
            return;
        }
        let unknowns = self.n_unknowns;
        if let Some(core) = square_core(&self.rows, self.keep) {
            self.attempts += 1;
            let mut map = vec![usize::MAX; unknowns];
            let mut k = 0;
            for c in 0..unknowns {
                if core.columns.get(c).copied().unwrap_or(false) {
                    map[c] = k;
                    k += 1;
                }
            }
            let sel: Vec<SparseRel> = core
                .rows
                .iter()
                .map(|&i| SparseRel {
                    cols: self.rows[i]
                        .cols
                        .iter()
                        .map(|&(c, v)| (map[c], v))
                        .collect(),
                    rhs: self.rows[i].rhs,
                })
                .collect();
            let before = self.la_ops;
            let x = wiedemann_u64(&sel, sel.len(), l, rng, &mut self.la_ops);
            let dd = x.map(|x| x[map[self.keep]]);
            if let Some(d) = dd.filter(|&d| verify(d)) {
                self.solved = Some((
                    d,
                    sel.len(),
                    self.rows.len() - sel.len(),
                    self.la_ops - before,
                ));
                return;
            }
            self.next_attempt = self.rows.len() + (sel.len() / 20).max(1);
        } else {
            self.next_attempt = self.rows.len() + (unknowns / 20).max(1);
        }
    }
}

/// Dense elimination mod `l` (d column last): `Some(d)` iff the rows determine d.
fn dense_solve_d(rows: &[SparseRel], ncols: usize, l: u64) -> Option<(u64, usize)> {
    let mm = |a: u64, b: u64| (a as u128 * b as u128 % l as u128) as u64;
    let pw = |mut b: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mm(r, b);
            }
            b = mm(b, b);
            e >>= 1;
        }
        r
    };
    let w = ncols + 1;
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| {
            let mut v = vec![0u64; w];
            for &(c, x) in &r.cols {
                v[c] = x;
            }
            v[ncols] = r.rhs;
            v
        })
        .collect();
    let covered = (0..ncols - 1)
        .filter(|&c| m.iter().any(|r| r[c] != 0))
        .count();
    let mut rk = 0;
    let mut dpivot = None;
    for c in 0..ncols {
        let Some(pv) = (rk..m.len()).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(rk, pv);
        let inv = pw(m[rk][c], l - 2);
        for x in m[rk].iter_mut() {
            *x = mm(*x, inv);
        }
        let prow = m[rk].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != rk && row[c] != 0 {
                let f = row[c];
                for k in c..w {
                    row[k] = (row[k] + l - mm(f, prow[k])) % l;
                }
            }
        }
        if c == ncols - 1 {
            dpivot = Some(rk);
        }
        rk += 1;
    }
    dpivot.map(|i| (m[i][ncols], covered))
}

fn s_value(snap: &Snap, setup: u64, base: u64, c_e: f64, l: u64) -> (f64, f64, f64) {
    let oracle = snap.weil + snap.f4 + snap.lin + snap.post;
    let rel = setup + base + snap.stream + oracle + snap.verify;
    let la = snap.la_ops * 16;
    let sq = (l as f64).sqrt();
    (
        (rel + la) as f64 / (c_e * sq),
        rel as f64 / (c_e * sq),
        la as f64 / (c_e * sq),
    )
}

fn snap_json(
    name: &str,
    snap: &Snap,
    setup: u64,
    base: u64,
    c_e: f64,
    l: u64,
    n_unknowns_full: usize,
    solved_correct: Option<bool>,
) -> serde_json::Value {
    let (s, s_rel, s_la) = s_value(snap, setup, base, c_e, l);
    let sq = (l as f64).sqrt();
    json!({
        "arm": name, "S": s, "S_rel": s_rel, "S_la": s_la,
        "native_per_sqrt_l": snap.f4_native as f64 / sq,
        "S_w0": s, "S_w0.1": s + 0.1 * snap.f4_native as f64 / sq, "S_w1": s + snap.f4_native as f64 / sq,
        "residuals": snap.residuals, "relations": snap.relations, "decompositions": snap.decompositions,
        "unknowns_full": n_unknowns_full, "unknowns_core": snap.unknowns_core, "filtered_out": snap.filtered_out,
        "la_ops": snap.la_ops, "la_attempts": snap.la_attempts, "f4_native_ops": snap.f4_native,
        "weil_muls": snap.weil, "f4_muls": snap.f4, "lin_muls": snap.lin, "post_muls": snap.post,
        "stream_muls": snap.stream, "verify_muls": snap.verify,
        "solved_correct": solved_correct, "wall_s_at_solve": snap.wall_s,
    })
}

fn paired(args: &[String]) {
    let p: u64 = arg(args, "--p", 1301);
    let seed: u64 = arg(args, "--seed", 1);
    let naive = flag(args, "--naive");
    let lines: u64 = arg(args, "--lines", 65536);
    let out: String = arg(args, "--out", String::new());
    let start = Instant::now();
    let spec0 = generate_spec(p, seed);
    let (spec, ls) = search_fb_line(&spec0, lines, seed);
    let line_muls = ls.muls;
    let ctx = Ctx::with_trace(&spec, false);
    let jac = ctx.jac();
    assert!(jac.is_valid(&spec.gj) && jac.is_valid(&spec.qj));
    let l = spec.l;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x3D1F6);
    let (c_e, _c_j) = unit_costs(&spec, &ctx);
    ctx.f.reset_muls();
    let base = factor_base(&ctx, &mut rng);
    let base_muls = ctx.f.muls();
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let small = base.len();
    ctx.f.reset_muls();
    let (al, be) = (rng.gen_range(1..l), rng.gen_range(1..l));
    let step = jac.add(
        &jac.mul(&spec.gj, al as u128),
        &jac.mul(&spec.qj, be as u128),
    );
    let mut a = rng.gen_range(0..l);
    let mut b = rng.gen_range(1..l);
    let mut r = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let setup_muls = ctx.f.muls() + spec.transfer_muls + line_muls;
    ctx.f.reset_muls();

    // the automorphism group on the signed factor base: iota only (census); see PLAN.md
    let iota: Vec<usize> = (0..2 * small).map(|s| s ^ 1).collect();
    let omap = orbit::build_orbits(2 * small, l, &[(iota, l - 1)]).expect("orbit map");
    assert_eq!(omap.n_orbits, small);

    let ec = EllE::new(&ctx.f, &spec.alpha);
    let verify = |d: u64| ec.mul(&spec.g, d) == spec.q;
    let mut arm_a = Arm::new(small);
    let mut arm_n = Arm::new(2 * small);
    let mut rows_q: Vec<SparseRel> = Vec::new();
    let mut cum = Snap::default();
    let mut snap_a: Option<Snap> = None;
    let mut snap_n: Option<Snap> = None;
    let mode = StopMode::Full;
    let cap = 400 * (small as u64 + 1) * 720;
    let mut mismatching_rows = 0u64;
    let mut digest: u64 = 0xcbf2_9ce4_8422_2325;
    'collect: while cum.residuals < cap {
        let mut batch: Vec<(u64, u64, Div<E2>, u64)> = Vec::with_capacity(64);
        for _ in 0..64 {
            let idx = cum.residuals + batch.len() as u64;
            batch.push((a, b, r.clone(), idx));
            r = jac.add(&r, &step);
            a = (a + al) % l;
            b = (b + be) % l;
        }
        cum.stream += ctx.f.muls();
        ctx.f.reset_muls();
        let f4_before = f4_fp::field_ops_total();
        let native_before = f4_fp::native_ops_total();
        let found: Vec<_> = batch
            .par_iter()
            .map_init(
                || Ctx::with_trace(&spec, false),
                |c, (a, b, r, idx)| {
                    let opts = opts_for_mode(24, 600.0, mode);
                    let mut lrng =
                        StdRng::seed_from_u64(seed ^ idx.wrapping_mul(0x9E37_79B9_7F4A_7C15));
                    let (decs, cost) = nagao_decompose(c, &base, &by_x, r, &opts, &mut lrng);
                    let jc = c.jac();
                    let v0 = c.f.muls();
                    let ok: Vec<_> = decs
                        .into_iter()
                        .filter(|d| {
                            crypto_lib::cryptanalysis::jv_cover::verify_dec(&jc, &base, r, d)
                        })
                        .collect();
                    let vm = c.f.muls() - v0;
                    (*a, *b, ok, cost, vm)
                },
            )
            .collect();
        if (cum.residuals / 64) % 100 == 0 {
            eprintln!(
                "progress p={p} seed={seed} residuals={} decomps={} rowsA={} rowsN={} attemptsA={} attemptsN={} t={:.0}s",
                cum.residuals, cum.decompositions, arm_a.rows.len(), arm_n.rows.len(),
                arm_a.attempts, arm_n.attempts, start.elapsed().as_secs_f64()
            );
        }
        cum.f4 += f4_fp::field_ops_total() - f4_before;
        cum.f4_native += f4_fp::native_ops_total() - native_before;
        for (a, b, decs, cost, vm) in found {
            cum.residuals += 1;
            cum.weil += cost.weil_muls;
            cum.lin += cost.lin_muls;
            cum.post += cost.post_muls;
            cum.verify += vm;
            let mut any = false;
            for d in decs {
                any = true;
                cum.decompositions += 1;
                for w in std::iter::once(a)
                    .chain(std::iter::once(b))
                    .chain(d.terms.iter().flat_map(|&(i, e)| [i as u64, e as u64]))
                {
                    digest = (digest ^ w).wrapping_mul(0x0000_0100_0000_01b3);
                }
                let dcol_neg = (l - b) % l;
                // arm A: the library's rows
                let mut cols: Vec<(usize, u64)> = d
                    .terms
                    .iter()
                    .map(|&(i, e)| (i, if e == 1 { 1 } else { l - 1 }))
                    .collect();
                cols.push((small, dcol_neg));
                cols.sort_unstable();
                // arm Q: signed ids through the orbit map
                let terms: Vec<(usize, u64)> = d
                    .terms
                    .iter()
                    .map(|&(i, e)| (2 * i + usize::from(e != 1), 1u64))
                    .collect();
                let mut qc = omap.row(&terms);
                qc.push((omap.n_orbits, dcol_neg));
                qc.sort_unstable();
                if qc != cols {
                    mismatching_rows += 1;
                }
                rows_q.push(SparseRel { cols: qc, rhs: a });
                if arm_a.solved.is_none() {
                    arm_a.rows.push(SparseRel { cols, rhs: a });
                }
                if naive && arm_n.solved.is_none() {
                    let mut nc: Vec<(usize, u64)> = terms.iter().map(|&(s, _)| (s, 1u64)).collect();
                    nc.push((2 * small, dcol_neg));
                    nc.sort_unstable();
                    arm_n.rows.push(SparseRel { cols: nc, rhs: a });
                }
            }
            if any {
                cum.relations += 1;
            }
            if arm_a.solved.is_none() {
                arm_a.try_solve(l, &mut rng, &verify);
                if let Some((dd, core, filt, last)) = arm_a.solved {
                    let mut s = cum.clone();
                    s.la_ops = arm_a.la_ops;
                    s.unknowns_core = core;
                    s.filtered_out = filt;
                    s.la_attempts = arm_a.attempts;
                    s.wall_s = start.elapsed().as_secs_f64();
                    let _ = (dd, last);
                    snap_a = Some(s);
                    if !naive {
                        break 'collect;
                    }
                }
            }
            if let Ok(path) = std::env::var("DUMP_N_ROWS") {
                if arm_n.rows.len() >= 420 {
                    let v: Vec<_> = arm_n
                        .rows
                        .iter()
                        .map(|r| json!({"cols": r.cols, "rhs": r.rhs}))
                        .collect();
                    std::fs::write(
                        path,
                        json!({"l": l, "d": spec.d, "small": small, "rows": v}).to_string(),
                    )
                    .unwrap();
                    std::process::exit(0);
                }
            }
            if naive && snap_n.is_none() && arm_n.rows.len() >= arm_n.next_attempt {
                arm_n.attempts += 1;
                if let Some((dd, covered)) = dense_solve_d(&arm_n.rows, 2 * small + 1, l) {
                    if verify(dd) {
                        // LA cost of N: model, the measured Wiedemann constant of arm A times n^2
                        let wa = arm_a.solved.map_or(0.0, |(_, core, _, last)| {
                            last as f64 / (core as f64).powi(2)
                        });
                        let mut s = cum.clone();
                        s.la_ops = (wa * ((2 * small + 1) as f64).powi(2)) as u64;
                        s.unknowns_core = covered + 1;
                        s.filtered_out = arm_n.rows.len();
                        s.la_attempts = arm_n.attempts;
                        s.wall_s = start.elapsed().as_secs_f64();
                        arm_n.solved = Some((dd, covered + 1, 0, 0));
                        snap_n = Some(s);
                        break 'collect;
                    }
                }
                arm_n.next_attempt = arm_n.rows.len() + 10;
            }
        }
    }
    // rows of arm Q against arm A, compared on the common prefix (arm A stops collecting once solved)
    let common = arm_a.rows.len().min(rows_q.len());
    let q_equal_a = mismatching_rows == 0
        && arm_a.rows[..common]
            .iter()
            .zip(&rows_q[..common])
            .all(|(x, y)| x.cols == y.cols && x.rhs == y.rhs);
    let correct = |arm: &Arm| arm.solved.map(|(d, ..)| d == spec.d);
    let sa = snap_a.clone().unwrap_or_default();
    let mut doc = json!({
        "p": p, "seed": seed, "l": l, "base_B": small, "c_add_e": c_e, "lines": lines,
        "relation_digest": digest,
        "git_head": std::env::var("GIT_HEAD").unwrap_or_default(),
        "group_G": {"generators": ["iota (lambda = l-1)"], "order": 2, "orbits": omap.n_orbits,
                    "signed_ids": 2 * small, "reduction": (2 * small) as f64 / omap.n_orbits as f64},
        "rows_Q_equal_rows_A": q_equal_a, "mismatching_rows": mismatching_rows,
        "arm_A": snap_json("A baseline (library rows, iota folded in)", &sa, setup_muls, base_muls, c_e, l, small + 1, correct(&arm_a)),
        "arm_Q": snap_json("Q orbit-quotient rows (G = {1, iota})", &sa, setup_muls, base_muls, c_e, l, omap.n_orbits + 1, correct(&arm_a)),
        "wall_s": start.elapsed().as_secs_f64(),
    });
    if naive {
        let sn = snap_n.clone().unwrap_or_default();
        doc["arm_N"] = snap_json(
            "N unquotiented signed unknowns",
            &sn,
            setup_muls,
            base_muls,
            c_e,
            l,
            2 * small + 1,
            correct(&arm_n),
        );
    }
    let text = serde_json::to_string_pretty(&doc).unwrap();
    if out.is_empty() {
        println!("{text}");
    } else {
        std::fs::write(&out, text).unwrap();
    }
    let _ = (E6::ZERO,);
}
