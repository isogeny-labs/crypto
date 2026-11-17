//! Native Nagao materializer retaining a checked four-plus-quadratic option.
//! The F4 system is unchanged; class anchors never enter the algebraic solver.
use crate::{anchored, remainder};
use crypto_lib::cryptanalysis::{
    f4_fp::F4Options,
    jv_bitops as work,
    jv_cover::{self as cover, BaseEl, Ctx, Dec6, Div, Fld, ZeroDimStats, E2},
    jv_sieve::FpRing,
};
use rand::rngs::StdRng;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};

#[derive(Serialize)]
pub struct Cofactor {
    pub lambda: [[u64; 2]; 3],
    pub sextic: Vec<u64>,
    pub roots: Vec<u64>,
    pub outcome: String,
    pub partial: Option<anchored::Partial>,
}
pub struct Output {
    pub full: Vec<Dec6>,
    pub partial: Vec<anchored::Verified>,
    pub cofactors: Vec<Cofactor>,
    pub stats: ZeroDimStats,
    pub degenerate: bool,
}

fn full_row(
    ctx: &Ctx,
    base: &[BaseEl],
    by_x: &HashMap<u64, usize>,
    r: &Div<E2>,
    a: &[E2],
    c: &[E2],
    roots: &[u64],
) -> Vec<Dec6> {
    if roots.len() != 6 {
        return vec![];
    }
    let f = &ctx.f.f;
    let mut options = Vec::new();
    for &x in roots {
        work::probe(1);
        let Some(&i) = by_x.get(&x) else {
            return vec![];
        };
        let den = remainder::eval(f, c, x);
        if den == E2::ZERO {
            options.push((i, vec![1, -1]));
            continue;
        }
        let y = f.neg(&f.mul(&remainder::eval(f, a, x), &f.inv(&den)));
        if y == E2::ZERO || f.sq(&y) != remainder::eval(f, &ctx.cov.hx, x) {
            return vec![];
        }
        let sign = if y == base[i].y {
            -1
        } else if y == f.neg(&base[i].y) {
            1
        } else {
            return vec![];
        };
        options.push((i, vec![sign]));
    }
    let free: Vec<_> = options
        .iter()
        .enumerate()
        .filter_map(|(i, o)| (o.1.len() > 1).then_some(i))
        .collect();
    if free.len() > 4 {
        return vec![];
    }
    let mut out = Vec::new();
    for mask in 0..1usize << free.len() {
        let mut terms: Vec<_> = options.iter().map(|(i, s)| (*i, s[0])).collect();
        for (bit, &k) in free.iter().enumerate() {
            terms[k].1 = if mask & (1 << bit) == 0 { 1 } else { -1 };
        }
        terms.sort_unstable();
        let d = Dec6 {
            terms: core::array::from_fn(|i| terms[i]),
        };
        // Preserve the existing branch, including denominator-zero choices.
        // Replay all rows here, even when the ordinate formula was unique.
        if cover::verify_dec(&ctx.jac(), base, r, &d) {
            out.push(d);
        }
    }
    out
}

pub fn decompose(
    ctx: &Ctx,
    base: &[BaseEl],
    by_x: &HashMap<u64, usize>,
    r: &Div<E2>,
    opts: &F4Options,
    rng: &mut StdRng,
    retain_quadratics: bool,
) -> Output {
    decompose_using(
        ctx,
        base,
        by_x,
        r,
        opts,
        rng,
        retain_quadratics,
        |polys, p, opts, rng| {
            Ok(cover::solve_zero_dim_traced(
                polys, 6, p, opts, rng, ctx.trace,
            ))
        },
    )
    .expect("the original solver has no materializer error")
}

/// Opt-in algebraic backend; the existing ordinate, norm and group checks stay
/// in this materializer. Backend refusals are errors, never empty root sets.
pub fn decompose_using(
    ctx: &Ctx,
    base: &[BaseEl],
    by_x: &HashMap<u64, usize>,
    r: &Div<E2>,
    opts: &F4Options,
    rng: &mut StdRng,
    retain_quadratics: bool,
    solve: impl FnOnce(
        &[crypto_lib::cryptanalysis::f4_fp::Poly],
        u64,
        &F4Options,
        &mut StdRng,
    ) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str>,
) -> Result<Output, &'static str> {
    let mut out = Output {
        full: vec![],
        partial: vec![],
        cofactors: vec![],
        stats: ZeroDimStats::default(),
        degenerate: r.u.len() != 4,
    };
    if out.degenerate {
        return Ok(out);
    }
    let f = &ctx.f.f;
    let mut v = r.v.clone();
    v.resize(3, E2::ZERO);
    let forms = cover::weil_forms(f, &ctx.cov.hx, &r.u, &v);
    let polys = forms.polys(&ctx.reim, f.p);
    f.count_public(6 * 10 * 8);
    let (solutions, stats) = solve(&polys, f.p, opts, rng)?;
    out.stats = stats;
    let ring = FpRing::new(f.p);
    let mut full = BTreeSet::new();
    for solution in solutions {
        if solution.len() != 6 || solution.iter().any(|&x| x >= f.p) {
            return Err("noncanonical algebraic solution");
        }
        let lam = [
            E2([solution[0], solution[1]]),
            E2([solution[2], solution[3]]),
            E2([solution[4], solution[5]]),
        ];
        let mut sextic = Vec::new();
        for k in 0..6 {
            let value = forms.eval(f, k, &lam);
            if value.0[1] != 0 {
                break;
            }
            sextic.push(value.0[0]);
        }
        let mut event = Cofactor {
            lambda: lam.map(|x| x.0),
            sextic: sextic.clone(),
            roots: vec![],
            outcome: "non-prime-field quotient".into(),
            partial: None,
        };
        if sextic.len() != 6 {
            out.cofactors.push(event);
            continue;
        }
        sextic.push(1);
        event.sextic = sextic.clone();
        let roots = ring.roots(&sextic, rng);
        event.roots = roots.clone();
        if roots.len() != 6 && !(roots.len() == 4 && retain_quadratics) {
            event.outcome = "unused root count".into();
            out.cofactors.push(event);
            continue;
        }
        let uc = |k: isize| {
            if k >= 0 {
                r.u.get(k as usize).copied().unwrap_or(E2::ZERO)
            } else {
                E2::ZERO
            }
        };
        let vc = |k: isize| {
            if k >= 0 {
                v.get(k as usize).copied().unwrap_or(E2::ZERO)
            } else {
                E2::ZERO
            }
        };
        let a: Vec<_> = (0..5isize)
            .map(|k| {
                f.sub(
                    &f.add(
                        &f.add(&f.neg(&vc(k - 1)), &f.mul(&lam[0], &uc(k))),
                        &f.mul(&lam[1], &uc(k - 1)),
                    ),
                    &f.mul(&lam[2], &vc(k)),
                )
            })
            .collect();
        let c = vec![lam[2], E2::ONE];
        if roots.len() == 6 {
            let rows = full_row(ctx, base, by_x, r, &a, &c, &roots);
            event.outcome = if rows.is_empty() {
                "unsupported full split"
            } else {
                "full split"
            }
            .into();
            full.extend(rows);
        } else if roots.len() == 4 && retain_quadratics {
            let mut tail = sextic.clone();
            let mut terms = Vec::new();
            let mut reason = None;
            for &x in &roots {
                work::probe(1);
                let Some(&i) = by_x.get(&x) else {
                    reason = Some("unsupported partial root");
                    break;
                };
                let den = remainder::eval(f, &c, x);
                if den == E2::ZERO {
                    reason = Some("zero partial denominator");
                    break;
                }
                let y = f.neg(&f.mul(&remainder::eval(f, &a, x), &f.inv(&den)));
                let sign = if y == base[i].y {
                    1
                } else if y == f.neg(&base[i].y) {
                    -1
                } else {
                    reason = Some("partial ordinate outside base");
                    break;
                };
                terms.push((i, sign));
                let (q, rem) = ring.divrem(&tail, &[(f.p - x) % f.p, 1]);
                if !rem.is_empty() {
                    reason = Some("non-exact root division");
                    break;
                }
                tail = q;
            }
            terms.sort_unstable();
            if let Some(reason) = reason {
                event.outcome = reason.into();
            } else {
                match anchored::verify(f, &ctx.cov.hx, base, r, &a, &c, &sextic, &terms, &tail) {
                    Ok(verified) => {
                        event.outcome = "verified partial".into();
                        event.partial = Some(verified.certificate().clone());
                        out.partial.push(verified);
                    }
                    Err(reason) => event.outcome = format!("partial refusal: {reason}"),
                }
            }
        } else {
            event.outcome = "unused root count".into();
        }
        out.cofactors.push(event);
    }
    out.full = full.into_iter().collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn full_materializer_matches_original_nagao_on_identical_native_systems() {
        let spec = cover::generate_spec(101, 409);
        let ctx = Ctx::new(&spec);
        let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        let opts = cover::opts_for_mode(24, 30.0, cover::StopMode::Staircase(64));
        let mut algebraic_solutions = 0;
        for k in 1..=3 {
            let residual = ctx.jac().add(&ctx.jac().mul(&spec.gj, k), &spec.qj);
            let seed = 100 + k as u64;
            let (old, _) = cover::nagao_decompose(
                &ctx,
                &base,
                &by_x,
                &residual,
                &opts,
                &mut StdRng::seed_from_u64(seed),
            );
            let new = decompose(
                &ctx,
                &base,
                &by_x,
                &residual,
                &opts,
                &mut StdRng::seed_from_u64(seed),
                true,
            );
            assert_eq!(old, new.full);
            algebraic_solutions += new.cofactors.len();
            for partial in &new.partial {
                assert!(anchored::replay(
                    &ctx.jac(),
                    &base,
                    &residual,
                    partial.certificate()
                ));
            }
        }
        assert!(
            algebraic_solutions > 0,
            "control must materialize actual algebraic solutions"
        );
    }

    #[test]
    fn positive_full_materializer_preserves_denominator_zero_group_choices() {
        let spec = cover::generate_spec(101, 409);
        let ctx = Ctx::new(&spec);
        let f = &ctx.f.f;
        let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        let roots: Vec<_> = base.iter().take(6).map(|b| b.x).collect();
        assert_eq!(roots.len(), 6);
        let mut residual = ctx.jac().identity();
        for b in base.iter().take(6) {
            residual = ctx.jac().add(&residual, &b.d);
        }
        for c in [vec![E2::ONE], vec![f.neg(&f.from_fp(roots[0])), E2::ONE]] {
            // Interpolation exercises this ordinate/group materializer only;
            // admission of principal norms has separate exact constructors.
            let mut a = vec![E2::ZERO; 6];
            for i in 0..6 {
                let mut basis = vec![E2::ONE];
                let mut denominator = E2::ONE;
                for j in 0..6 {
                    if i != j {
                        basis = remainder::mul(f, &basis, &[f.neg(&f.from_fp(roots[j])), E2::ONE]);
                        denominator =
                            f.mul(&denominator, &f.from_fp((f.p + roots[i] - roots[j]) % f.p));
                    }
                }
                let scale = f.mul(
                    &f.mul(&remainder::eval(f, &c, roots[i]), &base[i].y),
                    &f.inv(&denominator),
                );
                for (v, b) in a.iter_mut().zip(basis) {
                    *v = f.add(v, &f.mul(&scale, &b));
                }
            }
            let rows = full_row(&ctx, &base, &by_x, &residual, &a, &c, &roots);
            assert!(rows.contains(&Dec6 {
                terms: core::array::from_fn(|i| (i, 1))
            }));
            for d in rows {
                assert!(cover::verify_dec(&ctx.jac(), &base, &residual, &d));
            }
        }
    }
}
