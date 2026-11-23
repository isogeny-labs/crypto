//! Opt-in unit elimination with the original full-only Nagao materializer.
//! Materializer body is copied from jv_cover::nagao_decompose; the coordinate
//! Public FpRing adapts the private baseline root finder. A matched six-variable
//! adapter control isolates this change. The default entry point stays intact.
use super::nagao_unit_elimination::{self as unit, Audit};
use crate::cryptanalysis::{
    f4_fp::F4Options,
    jv_bitops as work,
    jv_cover::{verify_dec, weil_forms, BaseEl, Ctx, Dec6, Div, Fld, NagaoCost, E2},
    jv_sieve::FpRing,
};
use rand::rngs::StdRng;
use std::{
    collections::{BTreeSet, HashMap},
    time::Instant,
};
fn peval(f: &crate::cryptanalysis::jv_cover::Fq, a: &[E2], x: &E2) -> E2 {
    a.iter()
        .rev()
        .fold(E2::ZERO, |v, c| f.add(&f.mul(&v, x), c))
}

/// Every six-point decomposition of the reduced divisor `r` over the base, by
/// the Weil-restricted Nagao system.
pub fn decompose(
    ctx: &Ctx,
    base: &[BaseEl],
    by_x: &HashMap<u64, usize>,
    r: &Div<E2>,
    opts: &F4Options,
    rng: &mut StdRng,
    eliminate: bool,
) -> Result<(Vec<Dec6>, NagaoCost, Option<Audit>), &'static str> {
    if ctx.trace {
        return Err("unit backend requires trace=false");
    }
    let f = &ctx.f.f;
    let p = f.p;
    let mut cost = NagaoCost::default();
    if r.u.len() != 4 {
        cost.degenerate = true;
        return Ok((Vec::new(), cost, None));
    }
    let t_all = Instant::now();
    let m0 = f.muls();
    let mut v = r.v.clone();
    v.resize(3, E2::ZERO);
    let forms = weil_forms(f, &ctx.cov.hx, &r.u, &v);
    let polys = forms.polys(&ctx.reim, p);
    f.count_public(6 * 10 * 8);
    cost.weil_muls = f.muls() - m0;
    let (sols, st, audit) = if eliminate {
        let (s, t, a) = unit::solve_audited(&polys, p, opts, rng)?;
        (s, t, Some(a))
    } else {
        let (s, t) = crate::cryptanalysis::jv_cover::solve_zero_dim(&polys, 6, p, opts, rng);
        (s, t, None)
    };
    cost.f4_muls = st.f4_muls;
    cost.lin_muls = st.lin_muls;
    cost.delta = st.delta;
    cost.degree_reached = st.degree_reached;
    cost.max_rows = st.max_rows;
    cost.max_cols = st.max_cols;
    cost.f4_ms = st.f4_ms;
    cost.lin_ms = st.lin_ms;
    cost.inconsistent = st.inconsistent;
    cost.incomplete = st.incomplete;
    cost.timed_out = st.timed_out;
    cost.nonseparating = st.nonseparating;
    cost.lin_fallback = st.lin_fallback;
    cost.stopped_at = st.stopped_at;
    cost.hilbert_applied = st.hilbert_applied;
    cost.hilbert_fallback = st.hilbert_fallback;
    cost.hilbert_check_muls = st.hilbert_check_muls;
    cost.replayed = st.replayed;
    cost.trace_mismatch = st.trace_mismatch;
    cost.fp_solutions = sols.len();
    let m1 = f.muls();
    let ring = FpRing::new(p);
    let mut out: BTreeSet<Dec6> = BTreeSet::new();
    let uc = |k: isize| -> E2 {
        if k >= 0 && (k as usize) < r.u.len() {
            r.u[k as usize]
        } else {
            E2::ZERO
        }
    };
    let vc = |k: isize| -> E2 {
        if k >= 0 && (k as usize) < v.len() {
            v[k as usize]
        } else {
            E2::ZERO
        }
    };
    'sol: for s in &sols {
        let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
        let mut coeffs = Vec::with_capacity(7);
        for k in 0..6 {
            let c = forms.eval(f, k, &lam);
            if c.0[1] != 0 {
                continue 'sol;
            }
            coeffs.push(c.0[0]);
        }
        coeffs.push(1);
        let roots = ring.roots(&coeffs, rng);
        if roots.len() != 6 {
            continue;
        }
        // A_k = −v_{k−1} + λ₀u_k + λ₁u_{k−1} − µ₀v_k
        let ak: Vec<E2> = (0..5isize)
            .map(|k| {
                let mut a = f.neg(&vc(k - 1));
                a = f.add(&a, &f.mul(&lam[0], &uc(k)));
                a = f.add(&a, &f.mul(&lam[1], &uc(k - 1)));
                f.sub(&a, &f.mul(&lam[2], &vc(k)))
            })
            .collect();
        // Per root, the admissible signs: one when `y = −A/µ` is determined, both
        // when `µ(x) = 0` (then `A(x) = 0` too, and `u(x) = 0`: the reduced
        // divisor contains a point over this abscissa, `y` is not determined by
        // the formula and the group decides).
        let jc = ctx.jac();
        let mut options: Vec<(usize, Vec<i8>)> = Vec::with_capacity(6);
        for &x in &roots {
            let xe = f.from_fp(x);
            work::probe(1);
            let Some(&i) = by_x.get(&x) else {
                continue 'sol;
            };
            let mu = f.add(&lam[2], &xe);
            if mu == E2::ZERO {
                options.push((i, vec![1, -1]));
                continue;
            }
            let y = f.neg(&f.mul(&peval(f, &ak, &xe), &f.inv(&mu)));
            if y == E2::ZERO || f.sq(&y) != peval(f, &ctx.cov.hx, &xe) {
                continue 'sol;
            }
            // R = −Σ (Q_i − ∞): the sign of the base element is the opposite
            let eps: i8 = if y == base[i].y {
                -1
            } else if y == f.neg(&base[i].y) {
                1
            } else {
                continue 'sol;
            };
            options.push((i, vec![eps]));
        }
        if options.len() != 6 {
            continue;
        }
        let free: Vec<usize> = (0..6).filter(|&k| options[k].1.len() > 1).collect();
        if free.len() > 4 {
            continue;
        }
        cost.split += 1;
        for combo in 0..(1u32 << free.len()) {
            let mut terms: Vec<(usize, i8)> = options.iter().map(|(i, e)| (*i, e[0])).collect();
            for (bit, &k) in free.iter().enumerate() {
                terms[k].1 = if combo & (1 << bit) == 0 { 1 } else { -1 };
            }
            terms.sort_unstable();
            let t: [(usize, i8); 6] = core::array::from_fn(|k| terms[k]);
            let d = Dec6 { terms: t };
            if free.is_empty() || verify_dec(&jc, base, r, &d) {
                out.insert(d);
            }
        }
    }
    cost.post_muls = f.muls() - m1 + ring.muls();
    cost.decompositions = out.len();
    cost.total_ms = t_all.elapsed().as_secs_f64() * 1e3;
    Ok((out.into_iter().collect(), cost, audit))
}

// ── The oracle: meet in the middle over the three-point sums ──────────────
