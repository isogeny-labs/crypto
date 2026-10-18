//! Round-6 lead R6.7: the ζ-fold of the Gaudry base over `F_{p²}`, phase
//! two.  Round 4 (lead 16.4) measured the T-saturated m = 3 arm with its
//! own sweep, left inversions unpriced, and saw the 3-torsion curves'
//! full-rank streams stall.  This example re-runs the arms with the
//! repository's `line_oracle::LineOracle` as the two-summand step and
//! prices every inversion at the field's measured I/M ratio.
//!
//! Instances, bases, `φ`, `ψ`, `ζ` and the folds are copied unchanged from
//! `examples/r4_16_4_zeta_fold_fp2.rs` (frozen round-4 evidence).
//!
//! Arms at m = 3 (torsion family; `T = (0, √b)` rational):
//!
//! * **A, plain F** — `F = {x ∈ F_p^*}` on `E`, columns `⟨−1, ζ⟩`; sweep
//!   `P₁` over `F`, `LineOracle` on `R − P₁` over `F`.
//! * **B, T-saturated F on E** — `F_c = φ⁻¹(F'')` on `E` (18 points a
//!   column, `⟨−1, ζ, τ_T⟩`); sweep `P₁` over all of `F_c`.  `LineOracle`
//!   cannot be prepared on `F_c` (its abscissae are not on a line), so the
//!   two-summand step is the wrapper below: `S' = φ(R − P₁)` on `E'`,
//!   `LineOracle` over `F''`, lifted back through a precomputed preimage
//!   table plus one `⟨T⟩` correction.  This is the enumeration over `F`
//!   that Prop. 15.2 prices at `|M|^{m−2} = 3`.
//! * **C, quotient F/⟨T⟩** — the same arm on `E'`: sweep over `F''`,
//!   `LineOracle` over `F''`, target `φ(R)` (charged).
//!
//! The wrapper `CountedLine` runs the repository `LineOracle` for every
//! decision and, on the same descended system, a line-for-line copy of its
//! private solver whose only change is an inversion counter (the
//! repository solver calls `residual_walk::inv_mod`, which no counter
//! sees).  Every call asserts that the copy's multiplication count equals
//! the repository solver's and that a hit has solutions in the copy.
//!
//! Counted unit: `F_p` multiplications.  Inversions are counted three ways
//! (field arithmetic; solver as run; solver essential = no constant
//! arguments, one per distinct modulus in a call) and priced at
//! `I_p` = the multiplications of a Fermat inversion `a^{p−2}` at that `p`.
//!
//!     cargo run --release --example r6_7_zeta_fold_phase2 -- \
//!         --exp m3 --bits 9 --seeds 2 --targets 50 --json out.json

use std::cell::Cell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::ext_curve::{
    random_point, take_field_counters, ExtCurve, ExtDiagonalAutomorphism, ExtField, ExtPoint, Fp2,
    Fp2El,
};
use crypto_lib::cryptanalysis::glv_invariant_base::{
    addm, cornacchia_4p, factor_u64, fold_by_endomorphisms, mulm, subm, verify_endomorphism,
    Closure, Endomorphism, Negation,
};
use crypto_lib::cryptanalysis::glv_invariant_experiments::full_rank_stream;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, FactorBase, GroupOps, OracleCounters};
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crypto_lib::cryptanalysis::line_oracle::{semaev_s3, LineOracle};
use crypto_lib::cryptanalysis::residual_walk::{inv_mod, is_prime_u64, pow_mod, sqrt_mod};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

type Curve = ExtCurve<Fp2>;
type Pt = ExtPoint<Fp2El>;
type Fb = FactorBase<Pt>;

// ── Instance ────────────────────────────────────────────────────────

struct Inst {
    p: u64,
    f: Fp2,
    b: Fp2El,
    e: Curve,
    ep: Curve,
    n: u64,
    r: u64,
    h: u64,
    torsion: bool,
    split: Option<(u64, u64)>,
    trace: i64,
    g: Pt,
    gp: Pt,
    t: Pt,
    omega: u64,
    zeta: ExtDiagonalAutomorphism<Fp2El>,
    zetap: ExtDiagonalAutomorphism<Fp2El>,
    k3: u64,
}

/// The six candidate orders `p² + 1 − Tr(ζ₆ᵏ π₁²)` of `j = 0` curves over
/// `F_{p²}`, with `π₁ = (a + v√−3)/2`, `4p = a² + 3v²`.
fn j0_fp2_candidates(p: u64) -> Result<Vec<(u64, i64)>, String> {
    let (a, v) = cornacchia_4p(p, 3).ok_or("no Cornacchia solution for 4p = a² + 3v²")?;
    let (a, v) = (a as i128, v as i128);
    // (A + B√−3)/2 = π₁².
    let mut big_a = (a * a - 3 * v * v) / 2;
    let mut big_b = a * v;
    let p2 = (p as i128) * (p as i128);
    let mut out = Vec::new();
    for _ in 0..6 {
        if big_a * big_a + 3 * big_b * big_b != 4 * p2 {
            return Err("norm of the Frobenius candidate is not p²".into());
        }
        out.push(((p2 + 1 - big_a) as u64, big_a as i64));
        let (na, nb) = ((big_a - 3 * big_b) / 2, (big_a + big_b) / 2);
        big_a = na;
        big_b = nb;
    }
    Ok(out)
}

fn inf(f: &Fp2) -> Pt {
    ExtPoint::infinity(f.zero())
}

/// `φ_b : y² = x³ + b → y² = x³ − 27b`, kernel `x = 0`.
fn phi(f: &Fp2, b: Fp2El, pt: Pt) -> Pt {
    if pt.infinity || f.is_zero(pt.x) {
        return inf(f);
    }
    let x2 = f.sqr(pt.x);
    let x3 = f.mul(x2, pt.x);
    let inv3 = f.inv(x3).expect("x ≠ 0");
    let xx = f.mul(f.mul(f.add(x3, f.scale(b, 4)), pt.x), inv3);
    let yy = f.mul(f.mul(pt.y, f.sub(x3, f.scale(b, 8))), inv3);
    ExtPoint::affine(xx, yy)
}

/// `E' → E`: `φ_{b'}` to `y² = x³ + 729b`, then `(x, y) ↦ (x/9, y/27)`.
fn psi(f: &Fp2, bp: Fp2El, pt: Pt) -> Pt {
    let q = phi(f, bp, pt);
    if q.infinity {
        return q;
    }
    let p = f.p;
    ExtPoint::affine(f.scale(q.x, inv_mod(9, p)), f.scale(q.y, inv_mod(27, p)))
}

fn zeta_on(
    c: &Curve,
    gen: Pt,
    r: u64,
    omega: u64,
    seed: u64,
) -> Result<ExtDiagonalAutomorphism<Fp2El>, String> {
    let f = &c.f;
    let s = sqrt_mod(r - 3, r).ok_or("−3 is not a square mod r")?;
    let half = inv_mod(2, r);
    let cands = [
        ((r - 1 + s) % r * half) % r,
        (((r - 1) + (r - s)) % r * half) % r,
    ];
    let img = ExtPoint::affine(f.scale(gen.x, omega), gen.y);
    let mut ops = GroupOps::default();
    let lambda = cands
        .into_iter()
        .find(|&l| c.mul(&mut ops, gen, l) == img)
        .ok_or("neither root of λ² + λ + 1 is ζ's eigenvalue")?;
    let z = ExtDiagonalAutomorphism {
        cx: f.from_fp(omega),
        cy: f.one(),
        eigenvalue: lambda,
        order: 3,
        label: "zeta3",
    };
    verify_endomorphism(c, gen, r, &z, 20, seed)?;
    Ok(z)
}

/// `#E = Norm(α − 1)·Norm(α + 1)` for some `α = ζ₆ʲ π₁`: the order of the
/// base change of an `F_p` curve (and its quadratic twist).
fn base_change_split(p: u64, n: u64) -> Option<(u64, u64)> {
    let (a, v) = cornacchia_4p(p, 3)?;
    let (mut big_a, mut big_b) = (a as i128, v as i128);
    for _ in 0..6 {
        let n1 = ((big_a - 2) * (big_a - 2) + 3 * big_b * big_b) / 4;
        let n2 = ((big_a + 2) * (big_a + 2) + 3 * big_b * big_b) / 4;
        if n1 * n2 == n as i128 {
            return Some((n1 as u64, n2 as u64));
        }
        let (na, nb) = ((big_a - 3 * big_b) / 2, (big_a + big_b) / 2);
        big_a = na;
        big_b = nb;
    }
    None
}

/// `torsion = true`: `b` a square (rational `T = (0, √b)` of order 3),
/// `9 ∤ #E`, `r` the largest prime factor.  `torsion = false`: `b` a
/// non-square (no rational 3-torsion), `#E = h·r` with `h ≤ 16`.
fn build(p_bits: u32, seed: u64, torsion: bool) -> Result<Inst, String> {
    let mut rng =
        StdRng::seed_from_u64(seed ^ 0x1640_F2F2 ^ ((p_bits as u64) << 40) ^ u64::from(torsion));
    let mut ops = GroupOps::default();
    for _ in 0..2000 {
        let p = loop {
            let c = rng.gen_range((1u64 << (p_bits - 1))..(1u64 << p_bits)) | 1;
            if c % 3 == 1 && is_prime_u64(c) {
                break c;
            }
        };
        let f = Fp2::new(p)?;
        let cands = j0_fp2_candidates(p)?;
        for _ in 0..12 {
            let b: Fp2El = [rng.gen_range(0..p), rng.gen_range(1..p)];
            if f.is_square(b) != torsion {
                continue;
            }
            let e = Curve { f, a: f.zero(), b };
            let pts: Vec<Pt> = (0..4).map(|_| random_point(&e, &mut rng)).collect();
            let mut matches: Vec<(u64, i64)> = cands
                .iter()
                .copied()
                .filter(|&(n, _)| pts.iter().all(|&q| e.mul(&mut ops, q, n).infinity))
                .collect();
            matches.dedup();
            if matches.is_empty() {
                return Err(format!("no sextic-twist candidate order fits at p = {p}"));
            }
            if matches.len() > 1 {
                continue;
            }
            let (n, trace) = matches[0];
            let factors = factor_u64(n);
            let &(r, er) = factors.last().ok_or("n = 1")?;
            let h = n / r;
            if torsion {
                if n % 3 != 0 {
                    return Err(format!("b square but 3 ∤ #E at p = {p}"));
                }
                // r ≤ (√p + 1)² here (the order is a base-change product);
                // keep the curves whose r is a large part of one factor.
                if n % 9 == 0 || r <= 3 || er != 1 || r < p / 4 {
                    continue;
                }
            } else if h > 16 || er != 1 || r <= 3 {
                continue;
            }
            let g = loop {
                let q = e.mul(&mut ops, random_point(&e, &mut rng), h);
                if !q.infinity {
                    break q;
                }
            };
            if !e.mul(&mut ops, g, r).infinity {
                return Err("[r]G ≠ O".into());
            }
            let omega = (2..p)
                .map(|x| pow_mod(x, (p - 1) / 3, p))
                .find(|&w| w != 1)
                .ok_or("no cube root of unity")?;
            let zeta = zeta_on(&e, g, r, omega, seed)?;
            let split = base_change_split(p, n);
            if !torsion {
                return Ok(Inst {
                    p,
                    f,
                    b,
                    e,
                    ep: e,
                    n,
                    r,
                    h,
                    torsion,
                    split,
                    trace,
                    g,
                    gp: g,
                    t: inf(&f),
                    omega,
                    zeta,
                    zetap: zeta,
                    k3: 0,
                });
            }
            let t = ExtPoint::affine(f.zero(), f.sqrt(b).ok_or("b not a square")?);
            if !e.is_on_curve(t) || !e.mul(&mut ops, t, 3).infinity {
                return Err("T is not a rational 3-torsion point".into());
            }
            let bp = f.neg(f.scale(b, 27));
            let ep = Curve {
                f,
                a: f.zero(),
                b: bp,
            };
            if !phi(&f, b, t).infinity {
                return Err("φ(T) ≠ O".into());
            }
            for _ in 0..10 {
                let x = random_point(&e, &mut rng);
                let y = random_point(&e, &mut rng);
                let (px, py) = (phi(&f, b, x), phi(&f, b, y));
                if !ep.is_on_curve(px) {
                    return Err("φ(P) is not on E'".into());
                }
                if phi(&f, b, e.add(&mut ops, x, y)) != ep.add(&mut ops, px, py) {
                    return Err("φ is not additive".into());
                }
            }
            let gp = phi(&f, b, g);
            if gp.infinity || !ep.mul(&mut ops, gp, r).infinity {
                return Err("φ(G) is not of order r".into());
            }
            let zetap = zeta_on(&ep, gp, r, omega, seed ^ 1)?;
            return Ok(Inst {
                p,
                f,
                b,
                e,
                ep,
                n,
                r,
                h,
                torsion,
                split,
                trace,
                g,
                gp,
                t,
                omega,
                zeta,
                zetap,
                // φ(E) is the prime-to-3 part of E' (9 ∤ #E), where [3] is invertible.
                k3: inv_mod(3, n / 3),
            });
        }
    }
    Err(format!(
        "no instance at {p_bits} bits (torsion = {torsion})"
    ))
}

// ── Bases ───────────────────────────────────────────────────────────

/// Translation by `T`: on `[h]`-projected logarithms it acts as `1`.
struct TranslateT {
    t: Pt,
}

impl Endomorphism<Curve> for TranslateT {
    fn name(&self) -> String {
        "translate-T".into()
    }
    fn degree(&self) -> u64 {
        1
    }
    fn eigenvalue(&self) -> u64 {
        1
    }
    fn apply(&self, g: &Curve, p: Pt) -> Pt {
        g.add(&mut GroupOps::default(), p, self.t)
    }
}

fn fold(
    c: &Curve,
    r: u64,
    h: u64,
    seed: Vec<Pt>,
    gens: &[&dyn Endomorphism<Curve>],
    d: &str,
) -> Fb {
    let (fb, _) = fold_by_endomorphisms(
        c,
        r,
        h,
        seed,
        gens,
        Closure::Strict,
        32,
        |q| c.key(q),
        |q| c.f.pack(q.x),
        d.to_string(),
    )
    .unwrap_or_else(|e| panic!("fold {d}: {e}"));
    fb
}

fn line_points(c: &Curve, p: u64) -> Vec<Pt> {
    (1..p).flat_map(|t| c.lift_x(c.f.from_fp(t))).collect()
}

struct Bases {
    fa: Fb,          // E, x ∈ F_p^*, ⟨−1⟩
    fb: Fb,          // same points, ⟨−1, ζ⟩
    cz: Fb,          // E', F'' ⊂ ⟨G'⟩, ⟨−1, ζ'⟩
    cn: Fb,          // E', F'', ⟨−1⟩
    c18: Option<Fb>, // E, F_c = φ⁻¹(F''), ⟨−1, ζ, τ_T⟩
    c6: Option<Fb>,  // E, F_c, ⟨−1, τ_T⟩
    info: Value,
}

fn build_bases(inst: &Inst, with_fc_on_e: bool) -> Bases {
    let started = Instant::now();
    let (e, ep, f, r) = (&inst.e, &inst.ep, &inst.f, inst.r);
    let neg = Negation { r };
    let seed_a = line_points(e, inst.p);
    let mut ops = GroupOps::default();
    let in_g = seed_a
        .iter()
        .filter(|&&q| e.mul(&mut ops, q, r).infinity)
        .count();
    let fa = fold(e, r, inst.h, seed_a.clone(), &[&neg], "x ∈ F_p^*, ⟨−1⟩");
    let fb = fold(
        e,
        r,
        inst.h,
        seed_a,
        &[&neg, &inst.zeta],
        "x ∈ F_p^*, ⟨−1, ζ⟩",
    );
    // F'' on E': x ∈ F_p^*, filtered to ⟨G'⟩ = φ(E); the cube test is
    // checked against [r]Q = O on every point.
    let beta = if inst.torsion {
        f.sqrt(ep.b).expect("b' square")
    } else {
        f.zero()
    };
    let cube_exp = (inst.p * inst.p - 1) / 3;
    let (mut agree, mut disagree, mut line_pts) = (0u64, 0u64, 0u64);
    let mut seed_c = Vec::new();
    for q in if inst.torsion {
        line_points(ep, inst.p)
    } else {
        Vec::new()
    } {
        line_pts += 1;
        let truth = ep.mul(&mut ops, q, inst.n / 3).infinity;
        let z = f.sub(q.y, beta);
        let cube = f.pow(z, cube_exp) == f.one();
        if cube == truth {
            agree += 1;
        } else {
            disagree += 1;
        }
        if truth {
            seed_c.push(q);
        }
    }
    // Without rational 3-torsion there is no arm (c); the slots repeat (b)
    // and (a) so the drivers stay uniform, and every (c) field is marked
    // invalid in the output.
    if !inst.torsion {
        seed_c = line_points(e, inst.p);
    }
    let cz = fold(
        ep,
        r,
        inst.h,
        seed_c.clone(),
        &[&neg, &inst.zetap],
        "E': F'', ⟨−1, ζ'⟩",
    );
    let cn = fold(ep, r, inst.h, seed_c, &[&neg], "E': F'', ⟨−1⟩");
    // F_c on E: preimages [3⁻¹]ψ(Q) + ⟨T⟩.
    let tt = TranslateT { t: inst.t };
    let mut fc_pts = Vec::new();
    let mut fc_ok = true;
    if with_fc_on_e {
        let mut keys = HashSet::new();
        let mt = e.neg(inst.t);
        for &q in &cz.points {
            let p1 = e.mul(&mut ops, psi(f, ep.b, q), inst.k3);
            let img = phi(f, inst.b, p1);
            if cz.index_of_key(ep.key(&img)).is_none() {
                fc_ok = false;
            }
            for s in [p1, e.add(&mut ops, p1, inst.t), e.add(&mut ops, p1, mt)] {
                if keys.insert(e.key(&s)) {
                    fc_pts.push(s);
                }
            }
        }
        if fc_pts.len() != 3 * cz.points.len() {
            fc_ok = false;
        }
    }
    let (c18, c6) = if with_fc_on_e {
        (
            Some(fold(
                e,
                r,
                inst.h,
                fc_pts.clone(),
                &[&neg, &inst.zeta, &tt],
                "F_c, ⟨−1, ζ, τ_T⟩",
            )),
            Some(fold(e, r, inst.h, fc_pts, &[&neg, &tt], "F_c, ⟨−1, τ_T⟩")),
        )
    } else {
        (None, None)
    };
    let info = json!({
        "points_a": fa.points.len(),
        "fraction_of_F_in_G": in_g as f64 / fa.points.len() as f64,
        "U_a": fa.columns, "U_b": fb.columns,
        "points_per_col_a": fa.points.len() as f64 / fa.columns as f64,
        "points_per_col_b": fb.points.len() as f64 / fb.columns as f64,
        "zeta_closed_strict": true,
        "c_arm_valid": inst.torsion,
        "E_prime_line_points": line_pts,
        "F2_points_on_E_prime": cz.points.len(),
        "cube_test_agree": agree, "cube_test_disagree": disagree,
        "U_c_on_E_prime": cz.columns, "U_c_neg_on_E_prime": cn.columns,
        "Fc_on_E_built": with_fc_on_e,
        "Fc_points_on_E": c18.as_ref().map(|b| b.points.len()),
        "Fc_preimage_checks_ok": fc_ok,
        "U_c18_on_E": c18.as_ref().map(|b| b.columns), "U_c6_on_E": c6.as_ref().map(|b| b.columns),
        "build_wall_s_contended": started.elapsed().as_secs_f64(),
    });
    Bases {
        fa,
        fb,
        cz,
        cn,
        c18,
        c6,
        info,
    }
}

fn inst_json(inst: &Inst) -> Value {
    json!({
        "family": if inst.torsion {
            "y^2 = x^3 + b over F_{p^2}, b not in F_p, b a square (T = (0, sqrt b) rational), 9 does not divide #E"
        } else {
            "y^2 = x^3 + b over F_{p^2}, b a non-square (no rational 3-torsion), #E = h r, h <= 16"
        },
        "p": inst.p, "b": inst.b, "nu": inst.f.nu,
        "group_order": inst.n, "r": inst.r, "log2_r": (inst.r as f64).log2(),
        "cofactor": inst.h, "rational_3_torsion": inst.torsion,
        "order_is_base_change_product_N1_N2": inst.split, "trace_fp2": inst.trace, "omega": inst.omega,
        "lambda_zeta": inst.zeta.eigenvalue, "lambda_zeta_prime": inst.zetap.eigenvalue,
    })
}

fn random_targets(inst: &Inst, n: usize, seed: u64) -> Vec<Pt> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x7A12_6E75);
    let mut ops = GroupOps::default();
    (0..n)
        .map(|_| inst.e.mul(&mut ops, inst.g, rng.gen_range(1..inst.r)))
        .collect()
}

// ── Projection of relations ─────────────────────────────────────────

fn proj_row(idx: &[usize], fb: &Fb, r: u64) -> Vec<(usize, u64)> {
    let mut m: BTreeMap<usize, u64> = BTreeMap::new();
    for &i in idx {
        let e = m.entry(fb.col_of[i]).or_insert(0);
        *e = (*e + fb.coef_of[i]) % r;
    }
    m.into_iter().filter(|&(_, c)| c != 0).collect()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

fn ctx_for<'a>(c: &'a Curve, gen: Pt, target: Pt, inst: &Inst) -> InstanceCtx<'a, Curve> {
    InstanceCtx {
        group: c,
        generator: gen,
        target,
        r: inst.r,
        cofactor: inst.h,
        group_order: inst.n,
        name: format!("j0-fp2-p{}", inst.p),
        field_degree: Some(2),
    }
}

// ── Inversion pricing ───────────────────────────────────────────────

/// `F_p` multiplications of the Fermat inversion `a^{p−2}` by left-to-right
/// square-and-multiply: `(bits − 1)` squarings and `(popcount − 1)`
/// multiplications.  Checked against `inv_mod` by `fermat_inv`.
fn fermat_cost(p: u64) -> u64 {
    let e = p - 2;
    (63 - e.leading_zeros() as u64) + (e.count_ones() as u64 - 1)
}

fn fermat_inv(a: u64, p: u64, muls: &mut u64) -> u64 {
    let e = p - 2;
    let mut acc = a;
    for bit in (0..(63 - e.leading_zeros())).rev() {
        acc = mulm(acc, acc, p);
        *muls += 1;
        if (e >> bit) & 1 == 1 {
            acc = mulm(acc, a, p);
            *muls += 1;
        }
    }
    acc
}

thread_local! {
    /// Solver inversions of the counting copy: (as run, essential).
    static SOLVER_INV: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
    /// Moduli already inverted in this call (`poly_rem`'s leading coefficient).
    static SEEN_MODULI: std::cell::RefCell<HashSet<Vec<u64>>> = std::cell::RefCell::new(HashSet::new());
}

fn cinv(a: u64, p: u64, essential: bool) -> u64 {
    SOLVER_INV.with(|c| {
        let (x, y) = c.get();
        c.set((x + 1, y + u64::from(essential)));
    });
    inv_mod(a, p)
}

fn take_solver_inv() -> (u64, u64) {
    SEEN_MODULI.with(|s| s.borrow_mut().clear());
    SOLVER_INV.with(|c| c.replace((0, 0)))
}

// ── Counting copy of the repository line solver ─────────────────────
// Line-for-line copy of `line_oracle::{quadratic_from_values, det4,
// interpolate, eval_poly, LineOracle::descend, LineOracle::solve}` and
// `glv_invariant_base::{poly_*, poly_roots_fp_counted}`; the only change
// is `inv_mod` → `cinv` (an inversion counter).

fn quadratic_from_values(f: &Fp2, v: [Fp2El; 3]) -> [Fp2El; 3] {
    let p = f.p();
    let half = cinv(2, p, false);
    let c2 = f.scale(f.add(f.sub(v[0], f.scale(v[1], 2)), v[2]), half);
    let c1 = f.scale(
        f.sub(f.add(f.scale(v[1], 4), f.neg(f.scale(v[0], 3))), v[2]),
        half,
    );
    [v[0], c1, c2]
}

fn descend(f: &Fp2, c: Fp2El, curve: &Curve, x_r: Fp2El) -> Vec<[[u64; 3]; 3]> {
    let mut grid = [[f.zero(); 3]; 3];
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let x1 = f.scale(c, i as u64);
            let x2 = f.scale(c, j as u64);
            *cell = semaev_s3(f, curve.a, curve.b, x1, x2, x_r);
        }
    }
    let mut by_j = [[f.zero(); 3]; 3];
    for j in 0..3 {
        by_j[j] = quadratic_from_values(f, [grid[0][j], grid[1][j], grid[2][j]]);
    }
    let mut m = [[f.zero(); 3]; 3];
    for i in 0..3 {
        m[i] = quadratic_from_values(f, [by_j[0][i], by_j[1][i], by_j[2][i]]);
    }
    let mut out = vec![[[0u64; 3]; 3]; 2];
    for i in 0..3 {
        for j in 0..3 {
            for (comp, &v) in f.coords(m[i][j]).iter().enumerate() {
                out[comp][i][j] = v;
            }
        }
    }
    out
}

fn det4(m: &[[u64; 4]; 4], p: u64, muls: &mut u64) -> u64 {
    let mut a = *m;
    let mut det = 1u64;
    for col in 0..4 {
        let Some(piv) = (col..4).find(|&r| a[r][col] != 0) else {
            return 0;
        };
        if piv != col {
            a.swap(piv, col);
            det = subm(0, det, p);
        }
        det = mulm(det, a[col][col], p);
        *muls += 1;
        let inv = cinv(a[col][col], p, true);
        for r in (col + 1)..4 {
            if a[r][col] == 0 {
                continue;
            }
            let factor = mulm(a[r][col], inv, p);
            *muls += 1;
            for c in col..4 {
                a[r][c] = subm(a[r][c], mulm(factor, a[col][c], p), p);
                *muls += 1;
            }
        }
    }
    det
}

fn interpolate(values: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
    let n = values.len();
    let mut dd: Vec<u64> = values.to_vec();
    let mut coef = vec![dd[0]];
    for level in 1..n {
        let inv = cinv(level as u64, p, false);
        for i in 0..(n - level) {
            dd[i] = mulm(subm(dd[i + 1], dd[i], p), inv, p);
            *muls += 1;
        }
        coef.push(dd[0]);
    }
    let mut out = vec![0u64; n];
    let mut basis = vec![1u64];
    for (k, &c) in coef.iter().enumerate() {
        for (i, &bi) in basis.iter().enumerate() {
            out[i] = addm(out[i], mulm(c, bi, p), p);
            *muls += 1;
        }
        let mut next = vec![0u64; basis.len() + 1];
        for (i, &bi) in basis.iter().enumerate() {
            next[i + 1] = addm(next[i + 1], bi, p);
            next[i] = subm(next[i], mulm(bi, k as u64, p), p);
            *muls += 1;
        }
        basis = next;
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}

fn eval_poly(c: &[u64], t: u64, p: u64, muls: &mut u64) -> u64 {
    let mut acc = 0u64;
    for &ci in c.iter().rev() {
        acc = addm(mulm(acc, t, p), ci, p);
        *muls += 1;
    }
    acc
}

fn poly_trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

fn poly_mul(a: &[u64], b: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = addm(out[i + j], mulm(x, y, p), p);
            *muls += 1;
        }
    }
    poly_trim(out)
}

fn poly_rem(a: &[u64], m: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
    let mut a = a.to_vec();
    let dm = m.len() - 1;
    let fresh = SEEN_MODULI.with(|s| s.borrow_mut().insert(m.to_vec()));
    let inv_lead = cinv(m[dm], p, fresh);
    while a.len() > dm {
        let da = a.len() - 1;
        let c = mulm(a[da], inv_lead, p);
        *muls += 1;
        if c != 0 {
            for k in 0..=dm {
                a[da - dm + k] = subm(a[da - dm + k], mulm(c, m[k], p), p);
            }
            *muls += dm as u64 + 1;
        }
        a.pop();
        a = poly_trim(a);
    }
    a
}

fn poly_gcd(a: &[u64], b: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
    let (mut a, mut b) = (poly_trim(a.to_vec()), poly_trim(b.to_vec()));
    while !b.is_empty() {
        let r = poly_rem(&a, &b, p, muls);
        a = b;
        b = r;
    }
    if let Some(&lead) = a.last() {
        let inv = cinv(lead, p, true);
        for c in &mut a {
            *c = mulm(*c, inv, p);
        }
        *muls += a.len() as u64;
    }
    a
}

fn poly_powmod(base: &[u64], mut e: u64, m: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
    let mut acc = vec![1u64];
    let mut b = poly_rem(base, m, p, muls);
    while e > 0 {
        if e & 1 == 1 {
            acc = poly_rem(&poly_mul(&acc, &b, p, muls), m, p, muls);
        }
        b = poly_rem(&poly_mul(&b, &b, p, muls), m, p, muls);
        e >>= 1;
    }
    acc
}

fn poly_roots(f: &[u64], p: u64, seed: u64, muls: &mut u64) -> Vec<u64> {
    let f = poly_trim(f.to_vec());
    if f.len() <= 1 {
        return Vec::new();
    }
    if f.len() == 2 {
        return vec![subm(0, mulm(f[0], cinv(f[1], p, true), p), p)];
    }
    let xp = poly_powmod(&[0, 1], p, &f, p, muls);
    let mut xp_minus_x = xp;
    if xp_minus_x.len() < 2 {
        xp_minus_x.resize(2, 0);
    }
    xp_minus_x[1] = subm(xp_minus_x[1], 1, p);
    let g = poly_gcd(&f, &poly_trim(xp_minus_x), p, muls);
    let mut rng = StdRng::seed_from_u64(seed ^ 0x524F_4F54);
    let mut out = Vec::new();
    let mut stack = vec![g];
    while let Some(h) = stack.pop() {
        match h.len() {
            0 | 1 => {}
            2 => out.push(subm(0, mulm(h[0], cinv(h[1], p, true), p), p)),
            _ => {
                let delta = rng.gen_range(0..p);
                let mut s = poly_powmod(&[delta, 1], (p - 1) / 2, &h, p, muls);
                if s.is_empty() {
                    s.push(0);
                }
                s[0] = subm(s[0], 1, p);
                let d = poly_gcd(&h, &poly_trim(s), p, muls);
                if d.len() <= 1 || d.len() == h.len() {
                    stack.push(h);
                    continue;
                }
                let mut q = h.clone();
                let mut quotient = vec![0u64; h.len() - d.len() + 1];
                let inv_lead = cinv(*d.last().unwrap(), p, true);
                while q.len() >= d.len() && !q.is_empty() {
                    let dq = q.len() - 1;
                    let c = mulm(q[dq], inv_lead, p);
                    quotient[dq - (d.len() - 1)] = c;
                    for k in 0..d.len() {
                        q[dq - (d.len() - 1) + k] =
                            subm(q[dq - (d.len() - 1) + k], mulm(c, d[k], p), p);
                    }
                    *muls += d.len() as u64 + 1;
                    q.pop();
                    q = poly_trim(q);
                }
                stack.push(d);
                stack.push(poly_trim(quotient));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn solve(p: u64, polys: &[[[u64; 3]; 3]], seed: u64) -> (Vec<(u64, u64)>, u64) {
    let mut muls = 0u64;
    let coef_in_t1 = |q: &[[u64; 3]; 3], j: usize| -> Vec<u64> { vec![q[0][j], q[1][j], q[2][j]] };
    let nonzero: Vec<&[[u64; 3]; 3]> = polys
        .iter()
        .filter(|q| q.iter().any(|row| row.iter().any(|&v| v != 0)))
        .collect();
    let mut res: Vec<u64> = Vec::new();
    'pairs: for (ia, f0) in nonzero.iter().enumerate() {
        for f1 in nonzero.iter().skip(ia + 1) {
            let mut values = Vec::with_capacity(9);
            for t1 in 0..9u64 {
                let a: Vec<u64> = (0..3)
                    .map(|j| eval_poly(&coef_in_t1(f0, j), t1, p, &mut muls))
                    .collect();
                let b: Vec<u64> = (0..3)
                    .map(|j| eval_poly(&coef_in_t1(f1, j), t1, p, &mut muls))
                    .collect();
                let m = [
                    [a[2], a[1], a[0], 0],
                    [0, a[2], a[1], a[0]],
                    [b[2], b[1], b[0], 0],
                    [0, b[2], b[1], b[0]],
                ];
                values.push(det4(&m, p, &mut muls));
            }
            let candidate = interpolate(&values, p, &mut muls);
            if !candidate.is_empty() {
                res = candidate;
                break 'pairs;
            }
        }
    }
    if res.is_empty() {
        return (Vec::new(), muls);
    }
    let t1s = poly_roots(&res, p, seed, &mut muls);
    let mut pairs = Vec::new();
    for t1 in t1s {
        let mut cands: Option<Vec<u64>> = None;
        for q in polys {
            let a: Vec<u64> = (0..3)
                .map(|j| eval_poly(&coef_in_t1(q, j), t1, p, &mut muls))
                .collect();
            if a.iter().all(|&v| v == 0) {
                continue;
            }
            cands = Some(poly_roots(&a, p, seed ^ t1, &mut muls));
            break;
        }
        let Some(cands) = cands else {
            continue;
        };
        for t2 in cands {
            let all = polys.iter().all(|q| {
                let a: Vec<u64> = (0..3)
                    .map(|j| eval_poly(&coef_in_t1(q, j), t1, p, &mut muls))
                    .collect();
                eval_poly(&a, t2, p, &mut muls) == 0
            });
            if all {
                pairs.push((t1, t2));
            }
        }
    }
    (pairs, muls)
}

// ── The counted repository oracle ───────────────────────────────────

/// Per-target work: `F_p` multiplications and the three inversion counts.
#[derive(Default, Clone, Copy)]
struct Acc {
    muls: f64,
    inv_field: f64,
    inv_run: f64,
    inv_ess: f64,
}

impl Acc {
    fn absorb_field(&mut self) {
        let (m, i) = take_field_counters();
        self.muls += m as f64;
        self.inv_field += i as f64;
    }
}

struct CountedLine {
    line: LineOracle<Fp2>,
    c: Fp2El,
    calls: u64,
    mismatches: u64,
}

impl CountedLine {
    fn new(c: &Curve, ctx: &InstanceCtx<Curve>, fb: &Fb) -> Self {
        let mut line: LineOracle<Fp2> = LineOracle::new(c.f, c.f.one());
        line.prepare(ctx, fb, &Params::default(), &mut GroupOps::default())
            .expect("line base");
        take_field_counters();
        Self {
            line,
            c: c.f.one(),
            calls: 0,
            mismatches: 0,
        }
    }

    fn call(
        &mut self,
        ctx: &InstanceCtx<Curve>,
        fb: &Fb,
        ops: &mut GroupOps,
        ctr: &mut OracleCounters,
        pt: Pt,
        acc: &mut Acc,
    ) -> Option<Vec<usize>> {
        acc.absorb_field();
        let s0 = self.line.solver_totals().unwrap().ops;
        let res = self.line.decompose(ctx, fb, ops, ctr, pt);
        let s1 = self.line.solver_totals().unwrap().ops;
        acc.muls += (s1 - s0) as f64;
        acc.absorb_field();
        // The counting copy, on the same system; its field work is discarded.
        take_solver_inv();
        let polys = descend(&ctx.group.f, self.c, ctx.group, pt.x);
        let (pairs, muls) = solve(ctx.group.f.p(), &polys, ctx.group.key(&pt));
        take_field_counters();
        let (ir, ie) = take_solver_inv();
        acc.inv_run += ir as f64;
        acc.inv_ess += ie as f64;
        self.calls += 1;
        if muls != s1 - s0 || (res.is_some() && pairs.is_empty()) {
            self.mismatches += 1;
        }
        res
    }
}

// ── Per-arm records and the paired bootstrap ────────────────────────

#[derive(Default)]
struct ArmRec {
    muls: Vec<f64>,
    inv_field: Vec<f64>,
    inv_run: Vec<f64>,
    inv_ess: Vec<f64>,
    yield_: Vec<f64>,
    calls: Vec<f64>,
}

impl ArmRec {
    fn push(&mut self, a: &Acc, y: f64, calls: f64) {
        self.muls.push(a.muls);
        self.inv_field.push(a.inv_field);
        self.inv_run.push(a.inv_run);
        self.inv_ess.push(a.inv_ess);
        self.yield_.push(y);
        self.calls.push(calls);
    }
    /// Cost per target under pricing `k`: 0 unpriced, 1 as run, 2 essential.
    fn cost(&self, t: usize, k: usize, ip: f64) -> f64 {
        match k {
            0 => self.muls[t],
            1 => self.muls[t] + ip * (self.inv_field[t] + self.inv_run[t]),
            _ => self.muls[t] + ip * (self.inv_field[t] + self.inv_ess[t]),
        }
    }
    fn summary(&self, ip: f64) -> Value {
        json!({
            "muls_per_target": mean(&self.muls),
            "field_invs_per_target": mean(&self.inv_field),
            "solver_invs_as_run_per_target": mean(&self.inv_run),
            "solver_invs_essential_per_target": mean(&self.inv_ess),
            "calls_per_target": mean(&self.calls),
            "yield_per_target": mean(&self.yield_),
            "cost_per_target_unpriced": mean(&self.muls),
            "cost_per_target_priced_as_run": (0..self.muls.len()).map(|t| self.cost(t, 1, ip)).sum::<f64>() / self.muls.len() as f64,
            "cost_per_target_priced_essential": (0..self.muls.len()).map(|t| self.cost(t, 2, ip)).sum::<f64>() / self.muls.len() as f64,
            "inversion_share_of_priced_as_run": 1.0 - mean(&self.muls) / ((0..self.muls.len()).map(|t| self.cost(t, 1, ip)).sum::<f64>() / self.muls.len() as f64),
        })
    }
}

/// `U_x · Σcost_x / Σyield_x` over the multiset `idx`.
fn total(a: &ArmRec, u: f64, idx: &[usize], k: usize, ip: f64) -> f64 {
    let (mut c, mut y) = (0.0, 0.0);
    for &t in idx {
        c += a.cost(t, k, ip);
        y += a.yield_[t];
    }
    u * c / y
}

const PRICINGS: [&str; 3] = ["unpriced", "priced_as_run", "priced_essential"];

/// Ratio `num/den` of totals (times `scale`) under each pricing, with a
/// paired bootstrap over targets; also the shift priced − unpriced.
fn ratio_block(
    num: (&ArmRec, f64),
    den: (&ArmRec, f64),
    scale: f64,
    ip: f64,
    seed: u64,
    nrep: usize,
) -> Value {
    let n = num.0.muls.len();
    let all: Vec<usize> = (0..n).collect();
    let point: Vec<f64> = (0..3)
        .map(|k| scale * total(num.0, num.1, &all, k, ip) / total(den.0, den.1, &all, k, ip))
        .collect();
    let mut rng = StdRng::seed_from_u64(seed ^ 0xB007_5EED);
    let mut reps: Vec<[f64; 3]> = Vec::with_capacity(nrep);
    let mut idx = vec![0usize; n];
    for _ in 0..nrep {
        for v in idx.iter_mut() {
            *v = rng.gen_range(0..n);
        }
        let mut r = [0.0; 3];
        for (k, slot) in r.iter_mut().enumerate() {
            *slot = scale * total(num.0, num.1, &idx, k, ip) / total(den.0, den.1, &idx, k, ip);
        }
        reps.push(r);
    }
    let ci = |f: &dyn Fn(&[f64; 3]) -> f64| {
        let mut v: Vec<f64> = reps.iter().map(f).collect();
        v.sort_by(|a, b| a.total_cmp(b));
        let lo = v[(0.025 * nrep as f64) as usize];
        let hi = v[((0.975 * nrep as f64) as usize).min(nrep - 1)];
        [lo, hi]
    };
    let mut out = serde_json::Map::new();
    for k in 0..3 {
        out.insert(
            PRICINGS[k].into(),
            json!({ "point": point[k], "ci95": ci(&|r: &[f64; 3]| r[k]) }),
        );
    }
    for k in 1..3 {
        let ci0 = ci(&|r: &[f64; 3]| r[0]);
        out.insert(
            format!("shift_{}", PRICINGS[k]),
            json!({
                "point": point[k] - point[0],
                "ci95_paired": ci(&|r: &[f64; 3]| r[k] - r[0]),
                "priced_point_outside_unpriced_ci95": point[k] < ci0[0] || point[k] > ci0[1],
            }),
        );
    }
    // Thinned replicates for the cross-curve summary.
    out.insert(
        "reps".into(),
        json!(reps.iter().step_by((nrep / 200).max(1)).collect::<Vec<_>>()),
    );
    Value::Object(out)
}

// ── m = 3: arms A, B, C ─────────────────────────────────────────────

fn rows_of(triples: &HashSet<[usize; 3]>, fb: &Fb, r: u64) -> f64 {
    let rows: HashSet<Vec<(usize, u64)>> = triples.iter().map(|t| proj_row(t, fb, r)).collect();
    rows.len() as f64
}

/// Sweep `P₁` over `fb`, `LineOracle` on `R − P₁` (arms A and C).
fn sweep_line(
    c: &Curve,
    gen: Pt,
    inst: &Inst,
    fb: &Fb,
    targets: &[Pt],
    map_phi: bool,
) -> (ArmRec, u64, u64) {
    let ctx = ctx_for(c, gen, gen, inst);
    let mut cl = CountedLine::new(c, &ctx, fb);
    let mut ops = GroupOps::default();
    let mut ctr = OracleCounters::default();
    let mut rec = ArmRec::default();
    for &r0 in targets {
        take_field_counters();
        let mut acc = Acc::default();
        let rt = if map_phi {
            phi(&inst.f, inst.b, r0)
        } else {
            r0
        };
        let mut triples: HashSet<[usize; 3]> = HashSet::new();
        let mut calls = 0u64;
        for (i, &p1) in fb.points.iter().enumerate() {
            let s = c.add(&mut ops, rt, c.neg(p1));
            if s.infinity {
                continue;
            }
            calls += 1;
            if let Some(v) = cl.call(&ctx, fb, &mut ops, &mut ctr, s, &mut acc) {
                let mut t = [i, v[0], v[1]];
                t.sort_unstable();
                triples.insert(t);
            }
        }
        acc.absorb_field();
        rec.push(&acc, rows_of(&triples, fb, inst.r), calls as f64);
    }
    (rec, cl.calls, cl.mismatches)
}

/// Arm B: sweep `P₁` over all of `F_c` on `E`; the pair step is
/// `LineOracle` over `F''` on `E'` at `φ(R − P₁)`, lifted back.
fn sweep_b(inst: &Inst, bases: &Bases, targets: &[Pt]) -> (ArmRec, u64, u64, u64) {
    let (e, ep, f) = (&inst.e, &inst.ep, &inst.f);
    let c18 = bases.c18.as_ref().expect("F_c on E");
    let cz = &bases.cz;
    let mut ops = GroupOps::default();
    // Preimage table (setup, like the base itself; not charged).
    let pre: Vec<usize> = cz
        .points
        .iter()
        .map(|&q| {
            let p1 = e.mul(&mut ops, psi(f, ep.b, q), inst.k3);
            c18.index_of_key(e.key(&p1)).expect("preimage in F_c")
        })
        .collect();
    let ctx = ctx_for(ep, inst.gp, inst.gp, inst);
    let mut cl = CountedLine::new(ep, &ctx, cz);
    let mut ctr = OracleCounters::default();
    let mut rec = ArmRec::default();
    let tkeys: HashSet<u64> = [inf(f), inst.t, e.neg(inst.t)]
        .iter()
        .map(|q| e.key(q))
        .collect();
    let mut lift_fail = 0u64;
    for &rt in targets {
        take_field_counters();
        let mut acc = Acc::default();
        let mut triples: HashSet<[usize; 3]> = HashSet::new();
        let mut calls = 0u64;
        for (i, &p1) in c18.points.iter().enumerate() {
            let s = e.add(&mut ops, rt, e.neg(p1));
            let sp = phi(f, inst.b, s);
            if sp.infinity {
                continue;
            }
            calls += 1;
            if let Some(v) = cl.call(&ctx, cz, &mut ops, &mut ctr, sp, &mut acc) {
                let (j, k) = (pre[v[0]], pre[v[1]]);
                let pj_pk = e.add(&mut ops, c18.points[j], c18.points[k]);
                let rest = e.add(&mut ops, s, e.neg(pj_pk));
                let p3 = e.add(&mut ops, c18.points[k], rest);
                acc.absorb_field();
                let ok = tkeys.contains(&e.key(&rest));
                let k2 = c18.index_of_key(e.key(&p3));
                match (ok, k2) {
                    (true, Some(k2)) => {
                        // Verification (not charged).
                        let s12 = e.add(&mut ops, p1, c18.points[j]);
                        let sum = e.add(&mut ops, s12, p3);
                        take_field_counters();
                        assert_eq!(sum, rt, "lifted B relation does not sum to R");
                        let mut t = [i, j, k2];
                        t.sort_unstable();
                        triples.insert(t);
                    }
                    _ => lift_fail += 1,
                }
            }
        }
        acc.absorb_field();
        rec.push(&acc, rows_of(&triples, c18, inst.r), calls as f64);
    }
    (rec, cl.calls, cl.mismatches, lift_fail)
}

fn m3(bits: &[u32], seeds: &[u64], ntargets: usize, nrep: usize) -> Vec<Value> {
    let mut rows = Vec::new();
    for &b in bits {
        for &s in seeds {
            let started = Instant::now();
            let inst = match build(b, s, true) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("m3 {b} seed {s}: {e}");
                    continue;
                }
            };
            let ip = fermat_cost(inst.p) as f64;
            let bases = build_bases(&inst, true);
            let targets = random_targets(&inst, ntargets, s ^ 0x6733);
            let (ra, ca, xa) = sweep_line(&inst.e, inst.g, &inst, &bases.fb, &targets, false);
            let (rb, cb, xb, lf) = sweep_b(&inst, &bases, &targets);
            let (rc, cc, xc) = sweep_line(&inst.ep, inst.gp, &inst, &bases.cz, &targets, true);
            assert_eq!(xa + xb + xc, 0, "counting copy disagrees with LineOracle");
            let c18 = bases.c18.as_ref().unwrap();
            let (ua, ub, uc) = (
                bases.fb.columns as f64,
                c18.columns as f64,
                bases.cz.columns as f64,
            );
            let sf = 3.0 * bases.cz.points.len() as f64 / bases.fa.points.len() as f64;
            let ba = ratio_block((&rb, ub), (&ra, ua), 1.0, ip, s ^ 0xBA, nrep);
            let ca_ = ratio_block((&rc, uc), (&ra, ua), 1.0, ip, s ^ 0xCA, nrep);
            let ba_m = ratio_block((&rb, ub), (&ra, ua), sf, ip, s ^ 0xBA, nrep);
            let ca_m = ratio_block((&rc, uc), (&ra, ua), sf, ip, s ^ 0xCA, nrep);
            let bc = ratio_block((&rb, ub), (&rc, uc), 1.0, ip, s ^ 0xBC, nrep);
            let g = |v: &Value, k: &str| v[k]["point"].as_f64().unwrap();
            eprintln!(
                "m3 {b} seed {s} p={} r={} I_p={ip}: U {ua}/{ub}/{uc} rows A {:.1} B {:.1} C {:.1} | size-matched B/A {:.3} [{:.3} priced] C/A {:.3} [{:.3} priced] | B/C {:.3} | calls {ca}/{cb}/{cc} liftfail {lf} [{:.1}s]",
                inst.p, inst.r, mean(&ra.yield_), mean(&rb.yield_), mean(&rc.yield_),
                g(&ba_m, "unpriced"), g(&ba_m, "priced_as_run"),
                g(&ca_m, "unpriced"), g(&ca_m, "priced_as_run"), g(&bc, "unpriced"),
                started.elapsed().as_secs_f64()
            );
            rows.push(json!({
                "experiment": "m3", "bits": b, "seed": s,
                "instance": inst_json(&inst), "bases": bases.info, "targets": ntargets,
                "I_p_fermat_muls": ip,
                "U_A": ua, "U_B": ub, "U_C": uc,
                "points_A": bases.fb.points.len(), "points_B": c18.points.len(), "points_C": bases.cz.points.len(),
                "size_factor_Fc_over_F": sf,
                "arm_A": ra.summary(ip), "arm_B": rb.summary(ip), "arm_C": rc.summary(ip),
                "oracle_calls": [ca, cb, cc], "copy_mismatches": [xa, xb, xc], "B_lift_failures": lf,
                "ratio_B_over_A": ba, "ratio_C_over_A": ca_,
                "ratio_B_over_A_size_matched": ba_m, "ratio_C_over_A_size_matched": ca_m,
                "ratio_B_over_C": bc,
                "law": {"B_over_A": 3.0, "C_over_A": 1.0, "B_over_C": 3.0},
                "wall_s_contended": started.elapsed().as_secs_f64(),
            }));
        }
    }
    rows
}

// ── m = 2: arms a, b (E) and c (E') ─────────────────────────────────

fn line_m2(
    c: &Curve,
    gen: Pt,
    inst: &Inst,
    fb: &Fb,
    targets: &[Pt],
    map_phi: bool,
) -> (ArmRec, u64, u64) {
    let ctx = ctx_for(c, gen, gen, inst);
    let mut cl = CountedLine::new(c, &ctx, fb);
    let mut ops = GroupOps::default();
    let mut ctr = OracleCounters::default();
    let mut rec = ArmRec::default();
    for &r0 in targets {
        take_field_counters();
        let mut acc = Acc::default();
        let rt = if map_phi {
            phi(&inst.f, inst.b, r0)
        } else {
            r0
        };
        let res = cl.call(&ctx, fb, &mut ops, &mut ctr, rt, &mut acc);
        acc.absorb_field();
        rec.push(&acc, if res.is_some() { 1.0 } else { 0.0 }, 1.0);
    }
    (rec, cl.calls, cl.mismatches)
}

fn m2(bits: &[u32], seeds: &[u64], ntargets: usize, torsion: bool, nrep: usize) -> Vec<Value> {
    let mut rows = Vec::new();
    for &b in bits {
        for &s in seeds {
            let started = Instant::now();
            let inst = match build(b, s, torsion) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("m2 {b} seed {s}: {e}");
                    continue;
                }
            };
            let ip = fermat_cost(inst.p) as f64;
            let bases = build_bases(&inst, false);
            let targets = random_targets(&inst, ntargets, s);
            let (rab, n1, x1) = line_m2(&inst.e, inst.g, &inst, &bases.fb, &targets, false);
            assert_eq!(x1, 0, "counting copy disagrees with LineOracle");
            let (ua, ub) = (bases.fa.columns as f64, bases.fb.columns as f64);
            let ba = ratio_block((&rab, ub), (&rab, ua), 1.0, ip, s ^ 0xBA, nrep);
            let mut row = json!({
                "experiment": "m2", "bits": b, "seed": s, "family": if torsion {"torsion"} else {"prime"},
                "instance": inst_json(&inst), "bases": bases.info, "targets": ntargets,
                "I_p_fermat_muls": ip, "U_a": ua, "U_b": ub,
                "arm_ab": rab.summary(ip), "oracle_calls_ab": n1,
                "ratio_b_over_a": ba, "law": {"b_over_a": 1.0/3.0, "c_over_b": 1.0},
            });
            let mut line = format!(
                "m2 {b} seed {s} p={} I_p={ip}: hit {:.3} muls/call {:.0} invs/call field {:.1} run {:.1} ess {:.1} b/a {:.4}",
                inst.p, mean(&rab.yield_), mean(&rab.muls), mean(&rab.inv_field),
                mean(&rab.inv_run), mean(&rab.inv_ess),
                ba["priced_as_run"]["point"].as_f64().unwrap()
            );
            if torsion {
                let (rc, _, x2) = line_m2(&inst.ep, inst.gp, &inst, &bases.cz, &targets, true);
                assert_eq!(x2, 0, "counting copy disagrees with LineOracle");
                let uc = bases.cz.columns as f64;
                let sf = 3.0 * bases.cz.points.len() as f64 / bases.fa.points.len() as f64;
                let cb = ratio_block((&rc, uc), (&rab, ub), sf, ip, s ^ 0xCB, nrep);
                line += &format!(
                    " | c/b size-matched {:.3} priced {:.3}",
                    cb["unpriced"]["point"].as_f64().unwrap(),
                    cb["priced_as_run"]["point"].as_f64().unwrap()
                );
                row["U_c"] = json!(uc);
                row["size_factor_Fc_over_F"] = json!(sf);
                row["arm_c"] = rc.summary(ip);
                row["ratio_c_over_b_size_matched"] = cb;
            }
            row["wall_s_contended"] = json!(started.elapsed().as_secs_f64());
            eprintln!("{line} [{:.1}s]", started.elapsed().as_secs_f64());
            rows.push(row);
        }
    }
    rows
}

// ── I/M measurement ─────────────────────────────────────────────────

fn iratio(bits: &[u32], seeds: &[u64]) -> Vec<Value> {
    let mut rows = Vec::new();
    for &b in bits {
        for &s in seeds {
            let inst = match build(b, s, false) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("iratio {b} seed {s}: {e}");
                    continue;
                }
            };
            let (p, f) = (inst.p, inst.f);
            let mut rng = StdRng::seed_from_u64(s ^ 0x1A);
            // Counted: Fermat inversion in F_p, checked against inv_mod.
            let mut total = 0u64;
            for _ in 0..1000 {
                let a = rng.gen_range(1..p);
                let mut m = 0u64;
                assert_eq!(fermat_inv(a, p, &mut m), inv_mod(a, p));
                total += m;
            }
            let ip = total as f64 / 1000.0;
            assert_eq!(ip, fermat_cost(p) as f64);
            // Counted F_{p²} operations of the repository arithmetic.
            take_field_counters();
            let x: Fp2El = [rng.gen_range(0..p), rng.gen_range(1..p)];
            let y: Fp2El = [rng.gen_range(0..p), rng.gen_range(1..p)];
            let _ = f.mul(x, y);
            let (m_mul, _) = take_field_counters();
            let _ = f.inv(x);
            let (m_inv, i_inv) = take_field_counters();
            // Wall (contended): F_p inv_mod against mulm.
            let n = 2_000_000u64;
            let xs: Vec<u64> = (0..1024).map(|_| rng.gen_range(1..p)).collect();
            let t0 = Instant::now();
            let mut acc = 1u64;
            for k in 0..n {
                acc = mulm(acc, xs[(k & 1023) as usize], p);
            }
            let tm = t0.elapsed().as_secs_f64();
            let t1 = Instant::now();
            let mut acc2 = 0u64;
            for k in 0..n {
                acc2 ^= inv_mod(xs[((k ^ acc2) & 1023) as usize], p);
            }
            let ti = t1.elapsed().as_secs_f64();
            let fp2_i = m_inv as f64 + i_inv as f64 * ip;
            eprintln!(
                "iratio {b} seed {s} p={p}: I_p/M_p counted {ip} | F_p2: M = {m_mul} F_p muls, I = {m_inv} muls + {i_inv} F_p inv = {fp2_i} muls, I/M = {:.2} | wall F_p inv/mul {:.1} (contended) [{acc}{acc2}]",
                fp2_i / m_mul as f64, ti / tm
            );
            let _ = (acc, acc2);
            rows.push(json!({
                "experiment": "iratio", "bits": b, "seed": s, "p": p,
                "I_p_over_M_p_counted_fermat": ip,
                "fermat_formula": "(bitlen(p-2) - 1) squarings + (popcount(p-2) - 1) multiplications",
                "fp2_mul_in_fp_muls": m_mul, "fp2_inv_fp_muls": m_inv, "fp2_inv_fp_invs": i_inv,
                "fp2_inv_priced_in_fp_muls": fp2_i,
                "I_over_M_fp2_counted": fp2_i / m_mul as f64,
                "I_over_M_fp_wall_contended": ti / tm,
            }));
        }
    }
    rows
}

// ── Streams: full rank on the 3-torsion curves, 4× budget ───────────

fn stream(bits: &[u32], seeds: &[u64], max_trials: u64, torsion: bool) -> Vec<Value> {
    let mut rows = Vec::new();
    for &b in bits {
        for &s in seeds {
            let started = Instant::now();
            let inst = match build(b, s, torsion) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("stream {b} seed {s}: {e}");
                    continue;
                }
            };
            let bases = build_bases(&inst, false);
            let mut rng = StdRng::seed_from_u64(s ^ 0x5EED);
            let planted = rng.gen_range(2..inst.r);
            let mut ops = GroupOps::default();
            let q = inst.e.mul(&mut ops, inst.g, planted);
            let qp = phi(&inst.f, inst.b, q);
            let run = |c: &Curve, gen: Pt, tgt: Pt, folded: &Fb, control: &Fb| {
                let ctx = ctx_for(c, gen, tgt, &inst);
                let mut line: LineOracle<Fp2> = LineOracle::new(c.f, c.f.one());
                line.prepare(&ctx, folded, &Params::default(), &mut GroupOps::default())
                    .unwrap();
                take_field_counters();
                let rep = full_rank_stream(
                    c,
                    gen,
                    tgt,
                    inst.r,
                    inst.h,
                    planted,
                    folded,
                    control,
                    s,
                    max_trials,
                    2,
                    None,
                    |o, ctr, pt| line.decompose(&ctx, folded, o, ctr, pt),
                )
                .unwrap();
                let (m, i) = take_field_counters();
                let solver = line.solver_totals().unwrap().ops;
                (rep, m + solver, i)
            };
            let (r_ab, m_ab, i_ab) = run(&inst.e, inst.g, q, &bases.fb, &bases.fa);
            let (r_c, m_c, i_c) = if inst.torsion {
                run(&inst.ep, inst.gp, qp, &bases.cz, &bases.cn)
            } else {
                run(&inst.e, inst.g, q, &bases.cz, &bases.cn)
            };
            let arm = |a: &crypto_lib::cryptanalysis::glv_invariant_experiments::ArmReport| {
                json!({
                    "columns": a.columns, "points": a.points,
                    "square_relations": a.square_relations, "square_trials": a.square_trials,
                    "first_pin_relations": a.first_pin_relations, "first_pin_trials": a.first_pin_trials,
                    "rank_final": a.rank_final, "deficiency_total": a.deficiency_total,
                    "recovered": a.recovered, "verified": a.verified,
                    "achievable_rank": a.columns + 1 - a.deficiency_total,
                    "relations_total": a.relations_total,
                })
            };
            let sq = |a: &crypto_lib::cryptanalysis::glv_invariant_experiments::ArmReport| {
                a.square_trials.map(|t| t as f64)
            };
            let tb = sq(&r_ab.folded);
            let tc = sq(&r_c.folded);
            let row = json!({
                "experiment": "stream", "bits": b, "seed": s,
                "instance": inst_json(&inst), "planted": planted,
                "ab_stream": {
                    "trials": r_ab.trials, "relations": r_ab.relations, "hit_rate": r_ab.hit_rate,
                    "field_muls_total": m_ab, "field_invs_total": i_ab,
                    "b": arm(&r_ab.folded), "a": arm(&r_ab.control),
                    "square_ratio_a_over_b": r_ab.square_ratio, "column_ratio": r_ab.column_ratio,
                },
                "c_stream": {
                    "trials": r_c.trials, "relations": r_c.relations, "hit_rate": r_c.hit_rate,
                    "field_muls_total": m_c, "field_invs_total": i_c,
                    "c_zeta": arm(&r_c.folded), "c_neg": arm(&r_c.control),
                    "square_ratio_neg_over_zeta": r_c.square_ratio, "column_ratio": r_c.column_ratio,
                },
                "c_arm_valid": inst.torsion,
                "square_trials_c_over_b": match (tc, tb) { (Some(c), Some(b)) => Some(c / b), _ => None },
                "law_square_trials_c_over_b": 1.0,
                "max_trials": max_trials,
                "distinct_targets_available": inst.r - 1,
                "ab_stopped_before_budget": r_ab.trials < max_trials,
                "c_stopped_before_budget": r_c.trials < max_trials,
                "ab_exhausted": r_ab.exhausted, "c_exhausted": r_c.exhausted,
                "wall_s_contended": started.elapsed().as_secs_f64(),
            });
            eprintln!(
                "stream {b} seed {s} p={}: ab trials {} rel {} | b sq rel {:?} trials {:?} ok {} | a sq rel {:?} ok {} | c trials {} sq rel {:?} trials {:?} ok {} | cneg sq rel {:?} ok {} [{:.1}s]",
                inst.p, r_ab.trials, r_ab.relations,
                r_ab.folded.square_relations, r_ab.folded.square_trials, r_ab.folded.verified,
                r_ab.control.square_relations, r_ab.control.verified,
                r_c.trials, r_c.folded.square_relations, r_c.folded.square_trials, r_c.folded.verified,
                r_c.control.square_relations, r_c.control.verified,
                started.elapsed().as_secs_f64()
            );
            rows.push(row);
        }
    }
    rows
}

// ── Cross-curve summary (hierarchical bootstrap) ────────────────────

/// Pooled mean over curves of one ratio block, every pricing, with a
/// bootstrap that resamples curves and, inside each, one stored replicate.
fn pool(blocks: &[&Value], seed: u64) -> Value {
    let n = blocks.len();
    let reps: Vec<Vec<[f64; 3]>> = blocks
        .iter()
        .map(|b| {
            b["reps"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    let v = r.as_array().unwrap();
                    [
                        v[0].as_f64().unwrap(),
                        v[1].as_f64().unwrap(),
                        v[2].as_f64().unwrap(),
                    ]
                })
                .collect()
        })
        .collect();
    let point: Vec<f64> = (0..3)
        .map(|k| {
            blocks
                .iter()
                .map(|b| b[PRICINGS[k]]["point"].as_f64().unwrap())
                .sum::<f64>()
                / n as f64
        })
        .collect();
    let mut rng = StdRng::seed_from_u64(seed);
    let nb = 4000;
    let mut draws: Vec<[f64; 3]> = Vec::with_capacity(nb);
    for _ in 0..nb {
        let mut acc = [0.0; 3];
        for _ in 0..n {
            let c = rng.gen_range(0..n);
            let r = reps[c][rng.gen_range(0..reps[c].len())];
            for k in 0..3 {
                acc[k] += r[k] / n as f64;
            }
        }
        draws.push(acc);
    }
    let ci = |f: &dyn Fn(&[f64; 3]) -> f64| {
        let mut v: Vec<f64> = draws.iter().map(f).collect();
        v.sort_by(|a, b| a.total_cmp(b));
        [
            v[(0.025 * nb as f64) as usize],
            v[(0.975 * nb as f64) as usize],
        ]
    };
    let ci0 = ci(&|r: &[f64; 3]| r[0]);
    let per_curve: Vec<f64> = blocks
        .iter()
        .map(|b| b["unpriced"]["point"].as_f64().unwrap())
        .collect();
    let mut out = json!({
        "curves": n,
        "per_curve_unpriced": per_curve,
        "per_curve_priced_outside_unpriced_ci95": blocks.iter().map(|b| [
            b["shift_priced_as_run"]["priced_point_outside_unpriced_ci95"].as_bool().unwrap(),
            b["shift_priced_essential"]["priced_point_outside_unpriced_ci95"].as_bool().unwrap(),
        ]).collect::<Vec<_>>(),
    });
    for k in 0..3 {
        out[PRICINGS[k]] = json!({"mean": point[k], "ci95": ci(&|r: &[f64; 3]| r[k])});
    }
    for k in 1..3 {
        out[format!("shift_{}", PRICINGS[k])] = json!({
            "mean": point[k] - point[0],
            "ci95_paired": ci(&|r: &[f64; 3]| r[k] - r[0]),
            "relative": point[k] / point[0] - 1.0,
            "priced_mean_outside_unpriced_ci95": point[k] < ci0[0] || point[k] > ci0[1],
        });
    }
    out
}

fn arms_at_full_rank(r: &Value) -> f64 {
    [
        ("ab_stream", "b"),
        ("ab_stream", "a"),
        ("c_stream", "c_zeta"),
        ("c_stream", "c_neg"),
    ]
    .iter()
    .filter(|(x, y)| !r[*x][*y]["square_relations"].is_null())
    .count() as f64
        / 4.0
}

fn summarize(inputs: &[String]) -> Value {
    let mut by: HashMap<String, Vec<Value>> = HashMap::new();
    for path in inputs {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).expect("read")).unwrap();
        let exp = v["experiment"].as_str().unwrap().to_string();
        let fam = v["family"].as_str().unwrap_or("").to_string();
        for r in v["rows"].as_array().unwrap() {
            by.entry(format!("{exp}:{fam}"))
                .or_default()
                .push(r.clone());
        }
    }
    let mut out = serde_json::Map::new();
    let mut keys: Vec<&String> = by.keys().collect();
    keys.sort();
    for key in keys {
        let rows = &by[key];
        let ratio_keys: &[&str] = if key.starts_with("m3") {
            &[
                "ratio_B_over_A_size_matched",
                "ratio_C_over_A_size_matched",
                "ratio_B_over_C",
                "ratio_B_over_A",
                "ratio_C_over_A",
            ]
        } else if key.starts_with("m2") {
            &["ratio_b_over_a", "ratio_c_over_b_size_matched"]
        } else {
            &[]
        };
        let mut o = serde_json::Map::new();
        for (j, rk) in ratio_keys.iter().enumerate() {
            let blocks: Vec<&Value> = rows.iter().filter_map(|r| r.get(*rk)).collect();
            if blocks.is_empty() {
                continue;
            }
            o.insert(rk.to_string(), pool(&blocks, 0x5EED + j as u64));
        }
        if key.starts_with("m3") {
            let p = &o["ratio_C_over_A_size_matched"];
            let blk = |r: &Value, k: &str, i: usize| {
                r["ratio_C_over_A_size_matched"][k]["ci95"][i]
                    .as_f64()
                    .unwrap()
            };
            let pts = |k: &str| -> Vec<f64> {
                rows.iter()
                    .map(|r| {
                        r["ratio_C_over_A_size_matched"][k]["point"]
                            .as_f64()
                            .unwrap()
                    })
                    .collect()
            };
            let f2 = json!({
                "pooled_mean": [p["unpriced"]["mean"], p["priced_as_run"]["mean"], p["priced_essential"]["mean"]],
                "pooled_ci95": [p["unpriced"]["ci95"], p["priced_as_run"]["ci95"], p["priced_essential"]["ci95"]],
                "curves_point_over_1_5": PRICINGS.iter().map(|k| pts(k).iter().filter(|&&x| x > 1.5).count()).collect::<Vec<_>>(),
                "curves_ci95_lower_over_1_5": PRICINGS.iter().map(|k| rows.iter().filter(|r| blk(r, k, 0) > 1.5).count()).collect::<Vec<_>>(),
                "fires": PRICINGS.iter().any(|k| p[*k]["mean"].as_f64().unwrap() > 1.5),
                "pricings": PRICINGS,
            });
            o.insert("falsifier_2_quotient_pays_over_1_5".into(), f2);
        }
        if key.starts_with("stream") {
            let reached = |arm: &str, sub: &str| {
                rows.iter()
                    .filter(|r| !r[arm][sub]["square_relations"].is_null())
                    .count()
            };
            let flag = |k: &str| rows.iter().filter(|r| r[k].as_bool().unwrap()).count();
            let n = rows.len();
            let verified = rows.iter().all(|r| {
                ["b", "a"]
                    .iter()
                    .all(|a| r["ab_stream"][a]["verified"].as_bool().unwrap())
                    && ["c_zeta", "c_neg"]
                        .iter()
                        .all(|a| r["c_stream"][a]["verified"].as_bool().unwrap())
            });
            let counts = json!({
                "curves": n,
                "b_reached_full_rank": reached("ab_stream", "b"),
                "a_reached_full_rank": reached("ab_stream", "a"),
                "c_zeta_reached_full_rank": reached("c_stream", "c_zeta"),
                "c_neg_reached_full_rank": reached("c_stream", "c_neg"),
                "ab_stopped_before_budget": flag("ab_stopped_before_budget"),
                "c_stopped_before_budget": flag("c_stopped_before_budget"),
                "log_verified_all_arms": verified,
            });
            let per: Vec<f64> = rows.iter().map(arms_at_full_rank).collect();
            let mut rng = StdRng::seed_from_u64(0xF0);
            let mut d: Vec<f64> = (0..4000)
                .map(|_| (0..n).map(|_| per[rng.gen_range(0..n)]).sum::<f64>() / n as f64)
                .collect();
            d.sort_by(|a, b| a.total_cmp(b));
            o.insert("full_rank".into(), counts);
            o.insert(
                "falsifier_3_streams".into(),
                json!({
                    "fraction_of_arms_at_full_rank": mean(&per),
                    "ci95_over_curves": [d[100], d[3899]],
                    "fires": mean(&per) < 1.0,
                }),
            );
        }
        out.insert(key.clone(), Value::Object(o));
    }
    Value::Object(out)
}

// ── Driver ──────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let exp = get("--exp").unwrap_or_else(|| "m3".into());
    let bits: Vec<u32> = get("--bits")
        .unwrap_or_else(|| "9".into())
        .split(',')
        .map(|x| x.parse().expect("bits"))
        .collect();
    let seeds: Vec<u64> = match get("--seed-list") {
        Some(l) => l.split(',').map(|x| x.parse().unwrap()).collect(),
        None => (1..=get("--seeds").map_or(3, |x| x.parse().unwrap())).collect(),
    };
    let targets: usize = get("--targets").map_or(1000, |x| x.parse().unwrap());
    let nrep: usize = get("--boot").map_or(1000, |x| x.parse().unwrap());
    let max_trials: u64 = get("--max-trials").map_or(8_000_000, |x| x.parse().unwrap());
    let family = get("--family").unwrap_or_else(|| "torsion".into());
    let torsion = match family.as_str() {
        "torsion" => true,
        "prime" => false,
        other => panic!("unknown --family {other}; torsion or prime"),
    };
    let started = Instant::now();
    let out = if exp == "summarize" {
        let inputs: Vec<String> = get("--inputs")
            .expect("--inputs")
            .split(',')
            .map(String::from)
            .collect();
        json!({"lead": "R6.7", "experiment": "summarize", "inputs": inputs, "summary": summarize(&inputs)})
    } else {
        let rows = match exp.as_str() {
            "iratio" => iratio(&bits, &seeds),
            "m2" => m2(&bits, &seeds, targets, torsion, nrep),
            "m3" => m3(&bits, &seeds, targets, nrep),
            "stream" => stream(&bits, &seeds, max_trials, torsion),
            other => panic!("unknown --exp {other}"),
        };
        json!({
            "lead": "R6.7", "experiment": exp, "family": family, "bits": bits, "seeds": seeds,
            "targets": targets, "boot": nrep, "max_trials": max_trials, "rows": rows,
            "unit": "F_p multiplications (ext_curve thread-local counter + LineOracle solver count); inversions counted (field counter; solver as run and essential, from the counting copy) and priced at I_p = Fermat-inversion multiplications; wall times are contended",
            "wall_s_contended": started.elapsed().as_secs_f64(),
        })
    };
    let text = serde_json::to_string_pretty(&out).unwrap();
    match get("--json") {
        Some(path) => fs::write(&path, text).expect("write json"),
        None => println!("{text}"),
    }
}
