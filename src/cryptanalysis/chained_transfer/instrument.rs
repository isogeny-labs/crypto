//! # Instrument fixes for the chained-transfer measurements (2026-11-12).
//!
//! Protocol: `research/instrument_fixes_20261112/PROTOCOL.md`.  Three defects
//! touched every recorded distance to the matched rho:
//!
//! 1. **Genus-2 linear-algebra counter.**  `measure::genus2_index_calculus`
//!    reports `la_field_ops`, one tick per element update of a dense
//!    Gauss-Jordan; on the v3 rows it equals the matrix size
//!    `(fb + 2) x rows`.  [`solve_linear_counted`] runs the same elimination and
//!    counts every modular multiplication and every addition or subtraction,
//!    inversions included.  The old counter keeps its name and values.
//! 2. **Matched rho.**  [`rho_ref`] reports walk operations, set-up and other
//!    operations, and canonicalisations separately, for the negation-map walk
//!    (the primary reference) and the plain tuned walk.
//! 3. **Double large primes** for genus 2 as well as genus 3: [`ic_on_rung`]
//!    runs `ic_control::run_ic` (walk collection, graph reduction on the large
//!    primes, peeling, counted Wiedemann) on a [`H64Curve`] Jacobian of either
//!    genus, with the factor-base size rule [`fb_size_g`].
//!
//! Nothing here changes a group, a curve or a transfer; the one-time transfer
//! is in no per-instance total.  Every figure is `[pcc:unverified]` until a
//! validator replays it.

use serde::Serialize;

use super::ec::{Curve, Pt};
use super::hyper64::{H64Curve, H64Div};
use super::ic_control::{run_ic_with, Arm, IcParams, IcResult, Instance, Nat, RESTART_DEFAULT};
use super::measure::EcGroup;
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use crate::cryptanalysis::ic_boundary::{rho_reference_walk, CountedGroup, RhoWalk};

/// Host-calibrated native price of one relation-phase multiplication (mod `p`)
/// in group-operation equivalents, as measured by the positive control.
/// Informational: never part of a verdict.
pub const W_REL_CAL: f64 = 0.005;
/// Host-calibrated native price of one linear-algebra multiplication (mod `r`).
pub const W_LA_CAL: f64 = 0.013;

// ── genus-2 elimination with a real operation count ─────────────────────

fn mulmod_r(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn powmod_counted(a: u64, mut e: u64, r: u64, nat: &mut Nat) -> u64 {
    let (mut b, mut res) = (a % r, 1u64);
    while e > 0 {
        if e & 1 == 1 {
            nat.mul += 1;
            res = mulmod_r(res, b, r);
        }
        nat.mul += 1;
        b = mulmod_r(b, b, r);
        e >>= 1;
    }
    res
}

/// The elimination of `measure::solve_linear_free` (dense Gauss-Jordan over
/// `Z/r`, first zero-factor-base row with `beta != 0` gives `x = -alpha/beta`),
/// returning the same answer and the native operations it really performs:
/// one multiplication per normalised or updated entry, one subtraction (and
/// the reduction add) per updated entry, the Fermat inversion of each pivot
/// (square-and-multiply, counted) and of `beta`.
pub fn solve_linear_counted(
    rows: &[(Vec<i64>, u64, u64)],
    fb_size: usize,
    r: u64,
) -> (Option<u64>, Nat) {
    let mut nat = Nat::default();
    if rows.is_empty() {
        return (None, nat);
    }
    let rr = r as i128;
    let red = |v: i128| ((v % rr + rr) % rr) as u64;
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|(row, a, b)| {
            let mut v: Vec<u64> = row.iter().map(|&e| red(e as i128)).collect();
            v.push(*a);
            v.push(*b);
            v
        })
        .collect();
    let ncols = fb_size + 2;
    let mut pivot_row = 0usize;
    for col in 0..fb_size {
        let Some(sel) = (pivot_row..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(pivot_row, sel);
        let inv = powmod_counted(m[pivot_row][col], r - 2, r, &mut nat);
        for c in 0..ncols {
            nat.mul += 1;
            m[pivot_row][c] = mulmod_r(m[pivot_row][c], inv, r);
        }
        for i in 0..m.len() {
            if i != pivot_row && m[i][col] != 0 {
                let f = m[i][col];
                for c in 0..ncols {
                    nat.mul += 1;
                    nat.add += 2; // the subtraction and the reduction add
                    let t = mulmod_r(f, m[pivot_row][c], r);
                    m[i][c] = (m[i][c] + r - t) % r;
                }
            }
        }
        pivot_row += 1;
        if pivot_row >= m.len() {
            break;
        }
    }
    for row in &m {
        if row[..fb_size].iter().all(|&e| e == 0) {
            let (a, b) = (row[fb_size], row[fb_size + 1]);
            if b != 0 {
                nat.add += 1;
                let ib = powmod_counted(b, r - 2, r, &mut nat);
                nat.mul += 1;
                return (Some(mulmod_r((r - a % r) % r, ib, r)), nat);
            }
        }
    }
    (None, nat)
}

/// The recorded counter's value on a matrix of this shape: `(fb + 2) x rows`.
pub fn legacy_matrix_size(fb_size: usize, rows: usize) -> u64 {
    (fb_size as u64 + 2) * rows as u64
}

// ── loading the v3 genus-2 Jacobian into the word Jacobian ──────────────

/// The same curve `y^2 = f(x)` as an [`H64Curve`] (genus 2).
pub fn genus2_to_h64(c: &Genus2Curve) -> Result<H64Curve, String> {
    H64Curve::new(c.p(), 2, &c.f())
}

/// The divisor class `d` on `h` (`None` if it is not reduced there).
pub fn convert_div(h: &H64Curve, d: &Divisor) -> Option<H64Div> {
    h.from_coeffs(&d.u_coeffs(), &d.v_coeffs())
}

/// `n_ops` random additions, doublings and negations agree between the two
/// implementations (divisors converted after each operation).
pub fn crosscheck_genus2(c: &Genus2Curve, h: &H64Curve, n_ops: usize, seed: u64) -> bool {
    use rand::{rngs::StdRng, SeedableRng};
    let mut rng = StdRng::seed_from_u64(seed);
    for i in 0..n_ops {
        let a = c.random(&mut rng);
        let b = c.random(&mut rng);
        let (Some(ha), Some(hb)) = (convert_div(h, &a), convert_div(h, &b)) else {
            return false;
        };
        let (s, t, n) = (c.add(&a, &b), c.double(&a), c.neg(&a));
        let ok = match i % 3 {
            0 => convert_div(h, &s) == Some(h.cantor(ha, hb)),
            1 => convert_div(h, &t) == Some(h.cantor(ha, ha)),
            _ => convert_div(h, &n) == Some(h.negate(ha)),
        };
        if !ok {
            return false;
        }
    }
    true
}

// ── index calculus on a Jacobian rung of either genus ───────────────────

/// Factor-base columns for `arm` at genus `g`: `ceil(c p^(1 - 1/(g+1)))` for
/// one large prime, `ceil(c p^(1 - 1/g))` for two (`g = 3`: `p^(3/4)`,
/// `p^(2/3)`, the positive control's rule); [`Arm::Full`] takes every place.
pub fn fb_size_g(arm: Arm, g: u32, p: u64, c: f64) -> usize {
    let pf = p as f64;
    let gf = f64::from(g);
    match arm {
        Arm::Full => usize::MAX,
        Arm::SingleLp => (c * pf.powf(1.0 - 1.0 / (gf + 1.0))).ceil() as usize,
        Arm::DoubleLp => (c * pf.powf(1.0 - 1.0 / gf)).ceil() as usize,
    }
}

/// A rung's instance for `run_ic`: `#J = h r`, `r` prime, `h` prime to `r`,
/// `G` of order `r`, `T = [x]G`.  `Err` names why the rung is unusable.
pub fn instance_for_rung(
    curve: &H64Curve,
    jac_order: u64,
    r: u64,
    g: H64Div,
    target: H64Div,
    x: u64,
) -> Result<Instance, String> {
    if r < 3 || !jac_order.is_multiple_of(r) {
        return Err("r does not divide #J".into());
    }
    let h = jac_order / r;
    if h.is_multiple_of(r) {
        return Err("r divides #J/r: the r-part of J is not cyclic".into());
    }
    if g.is_identity() || !curve.mul_rtl(g, r).is_identity() {
        return Err("G does not have order r".into());
    }
    if curve.mul_rtl(g, x) != target {
        return Err("T != [x]G".into());
    }
    Ok(Instance {
        curve: curve.clone(),
        p: curve.p(),
        order: jac_order,
        h,
        r,
        g,
        t_secret: x,
        target,
    })
}

/// One index-calculus arm on a rung, with the factor-base rule [`fb_size_g`]
/// (`c = 2`) and a walk restart every `max(4, sqrt(r))` steps.  `run_ic` certifies the log by `[t]G == T` twice (counted and
/// right-to-left); the caller re-checks independently.
pub fn ic_on_rung(inst: &Instance, arm: Arm, seed: u64, max_trials: u64) -> IcResult {
    let nb = if arm == Arm::Full {
        0
    } else {
        fb_size_g(arm, inst.curve.genus(), inst.p, 2.0)
    };
    // A walk in a subgroup of order r repeats after about sqrt(r) steps: restart
    // from a fresh random point then (never later than the default).
    let restart = ((inst.r as f64).sqrt().ceil() as u64).clamp(4, RESTART_DEFAULT);
    run_ic_with(
        inst,
        IcParams {
            arm,
            nb,
            seed,
            max_trials,
            verify_relations: false,
        },
        restart,
    )
}

/// Total of an index-calculus run at `w` GAE per native multiplication.
pub fn ic_total(res: &IcResult, w: f64) -> f64 {
    res.total(w)
}

/// Informational host-calibrated total: relation-phase multiplications at
/// [`W_REL_CAL`], linear-algebra multiplications at [`W_LA_CAL`].
pub fn ic_total_cal(res: &IcResult) -> f64 {
    res.group_ops() as f64 + W_REL_CAL * res.rel.mul as f64 + W_LA_CAL * res.la.mul as f64
}

// ── matched rho with the walk and the set-up apart ──────────────────────

/// A matched rho run, split.
#[derive(Clone, Debug, Default, Serialize)]
pub struct RhoRef {
    pub recovered: Option<u64>,
    pub certified: bool,
    /// Every group operation: set-up, walk, verification.
    pub total_ops: u64,
    /// Walk steps (the `sqrt(pi r / 2)` quantity, charged operations of the walks).
    pub walk_ops: u64,
    /// Set-up (jump table, starts) and everything else.
    pub other_ops: u64,
    /// Canonicalisations the group-operation unit does not charge.
    pub native: u64,
    pub steps: u64,
    pub negation: bool,
}

impl RhoRef {
    /// Total at `w` GAE per canonicalisation.
    pub fn total(&self, w: f64) -> f64 {
        self.total_ops as f64 + w * self.native as f64
    }
}

/// Counted rho (`negation` selects the negation-map walk, otherwise the plain
/// tuned walk) on any counted group; `certify(k)` is the caller's independent
/// `[k]G == T`.
pub fn rho_ref<G: CountedGroup>(
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    negation: bool,
    certify: &dyn Fn(u64) -> bool,
) -> RhoRef {
    let max_steps = (60.0 * (r as f64).sqrt()) as u64 + 10_000;
    let walk = if negation {
        RhoWalk::negation()
    } else {
        RhoWalk::plain()
    };
    let rr = rho_reference_walk(g, generator, target, r, seed, max_steps, walk);
    let total = rr.group_ops.adds + rr.group_ops.doubles;
    let walk_ops = rr
        .counters
        .get("walk_operations")
        .copied()
        .unwrap_or(rr.steps);
    let native = if negation {
        rr.counters
            .get("canonicalisations_uncharged")
            .copied()
            .unwrap_or(0)
            .max(rr.steps)
    } else {
        0
    };
    RhoRef {
        recovered: rr.recovered,
        certified: rr.recovered.is_some_and(certify),
        total_ops: total,
        walk_ops,
        other_ops: total.saturating_sub(walk_ops),
        native,
        steps: rr.steps,
        negation,
    }
}

/// [`rho_ref`] on an elliptic curve, certified by `Curve::mul`.
pub fn rho_on_curve(e: &Curve, gen: Pt, target: Pt, r: u64, seed: u64, negation: bool) -> RhoRef {
    let eg = EcGroup(*e);
    rho_ref(&eg, gen, target, r, seed, negation, &|k| {
        e.mul(&gen, k as u128) == target
    })
}

// ── the small_de matched rho, split ─────────────────────────────────────

/// `small_de::rho_cost` with the walk and the set-up apart.  Same walks, same
/// seeds, same charges: `total_gae` equals `RhoCost::gae` of the old call.
#[derive(Clone, Debug, Serialize)]
pub struct RhoSplit {
    pub kind: &'static str,
    pub automorphisms: u32,
    pub total_gae: f64,
    pub walk_ops: u64,
    pub other_gae: f64,
    pub native: u64,
    pub steps: u64,
    pub certified: bool,
}

impl RhoSplit {
    pub fn total(&self, w: f64) -> f64 {
        self.total_gae + w * self.native as f64
    }
    pub fn walk_only(&self, w: f64) -> f64 {
        self.walk_ops as f64 + w * self.native as f64
    }
}

/// Matched rho on a `small_de` instance for `kind` (plain, negation, or the
/// automorphism walk of `D = -3, -4`); `None` where the old call is `None`.
pub fn rho_split_prime(
    inst: &super::small_de::Inst,
    kind: super::small_de::RhoKind,
    target: crate::cryptanalysis::ic_boundary::PrimePoint,
    seed: u64,
) -> Option<RhoSplit> {
    use super::small_de::RhoKind;
    use crate::cryptanalysis::ic_boundary::{
        rho_walk_with, CubicClasses, GroupOps, NegationClasses, PointClasses, QuarticClasses,
    };
    let c = &inst.curve;
    let (g, r) = (inst.g, inst.r);
    let max = 400 * r;
    let mut setup = GroupOps::default();
    let (res, name) = match kind {
        RhoKind::Plain => (
            rho_walk_with(c, &PointClasses, g, target, r, seed, max, RhoWalk::plain()),
            "plain",
        ),
        RhoKind::Negation => (
            rho_walk_with(
                c,
                &NegationClasses { r },
                g,
                target,
                r,
                seed,
                max,
                RhoWalk::negation(),
            ),
            "negation",
        ),
        RhoKind::Aut => match inst.d {
            -3 => {
                let cl = CubicClasses::new(c, g, r, &mut setup)?;
                (
                    rho_walk_with(c, &cl, g, target, r, seed, max, RhoWalk::negation()),
                    "aut",
                )
            }
            -4 => {
                let cl = QuarticClasses::new(c, g, r, &mut setup)?;
                (
                    rho_walk_with(c, &cl, g, target, r, seed, max, RhoWalk::negation()),
                    "aut",
                )
            }
            _ => return None,
        },
    };
    let k = res.recovered?;
    let mut o2 = GroupOps::default();
    let certified = c.mul(&mut o2, g, k) == target;
    let total = res.gae + setup.gae();
    let walk = res
        .counters
        .get("walk_operations")
        .copied()
        .unwrap_or(res.steps);
    Some(RhoSplit {
        kind: name,
        automorphisms: res.automorphisms,
        total_gae: total,
        walk_ops: walk,
        other_gae: total - walk as f64,
        native: res
            .counters
            .get("canonicalisations_uncharged")
            .copied()
            .unwrap_or(0),
        steps: res.steps,
        certified,
    })
}

// ── small statistics ────────────────────────────────────────────────────

/// Mean and standard error of the mean.
pub fn mean_se(xs: &[f64]) -> (f64, f64) {
    let n = xs.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    let m = xs.iter().sum::<f64>() / n as f64;
    if n == 1 {
        return (m, 0.0);
    }
    let v = xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n as f64 - 1.0);
    (m, (v / n as f64).sqrt())
}

/// Order of two totals across the 5a band: `below`, `above`, or `unresolved`
/// (the order changes inside the band).  `a` and `b` are the totals at each
/// price.
pub fn band_class(a: &[f64], b: &[f64]) -> &'static str {
    if a.iter().zip(b).all(|(x, y)| x < y) {
        "below"
    } else if a.iter().zip(b).all(|(x, y)| x > y) {
        "above"
    } else {
        "unresolved"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::chained_transfer::ic_control::instance_from;
    use crate::cryptanalysis::chained_transfer::measure::{
        genus2_collect_rows, genus2_index_calculus, jac_instance, solve_linear_free,
    };

    fn largest_cyclic_prime(mut n: u64) -> u64 {
        let (mut best, mut d) = (1, 2);
        while d * d <= n {
            if n.is_multiple_of(d) {
                let mut e = 0;
                while n.is_multiple_of(d) {
                    n /= d;
                    e += 1;
                }
                if e == 1 {
                    best = best.max(d);
                }
            }
            d += 1;
        }
        if n > 1 {
            best = best.max(n);
        }
        best
    }

    fn g2_instance(p: u64) -> (Genus2Curve, u64, u64, u64) {
        let c = Genus2Curve::x5_plus_b(p, 3).unwrap();
        let order = c.jacobian_order().unwrap();
        let r = largest_cyclic_prime(order);
        assert!(r > 50 && !(order / r).is_multiple_of(r));
        (c, order, r, 17 % r)
    }

    #[test]
    fn counted_elimination_agrees_with_the_recorded_one_and_counts_more() {
        let (c, order, r, x) = g2_instance(251);
        let (g, t) = jac_instance(&c, order, r, x, 3).unwrap();
        let rows = genus2_collect_rows(&c, &g, &t, r, 251, 2_000_000, 3);
        let (old, old_ops) = solve_linear_free(&rows.rows, rows.fb_size, r);
        let (new, nat) = solve_linear_counted(&rows.rows, rows.fb_size, r);
        assert_eq!(old, Some(x));
        assert_eq!(new, old);
        // every old tick is one multiplication of the new count; the new count
        // adds the pivot inversions and all additions
        assert!(nat.mul >= old_ops, "{} < {}", nat.mul, old_ops);
        assert!(nat.add > 0);
        // the recorded counter of the full path is untouched
        let full = genus2_index_calculus(&c, &g, &t, r, 251, 2_000_000, 3);
        assert_eq!(full.la_field_ops, old_ops);
        assert_eq!(full.group_ops, rows.group_ops);
    }

    #[test]
    fn counted_elimination_on_a_hand_matrix() {
        // x0 = 3 - 2 t ... rows: (x0 + a1 + b1 t), eliminate x0 with two rows:
        // row0 = (1 | a=5, b=3), row1 = (1 | a=7, b=2) over Z/101.
        let rows = vec![(vec![1i64], 5u64, 3u64), (vec![1i64], 7, 2)];
        let (x, nat) = solve_linear_counted(&rows, 1, 101);
        // row1 - row0 = (0 | 2, -1): x = -2 / -1 = 2
        assert_eq!(x, Some(2));
        // pivot inverse of 1 (7 squarings, 4 multiplies), 3 normalisations,
        // 3 updates, the final inverse of beta and one product
        assert!(nat.mul >= 11 + 3 + 3);
        assert_eq!(nat.add, 3 * 2 + 1);
    }

    #[test]
    fn genus2_loads_into_the_word_jacobian_and_arms_certify() {
        let (c, order, r, x) = g2_instance(251);
        let h = genus2_to_h64(&c).unwrap();
        assert!(crosscheck_genus2(&c, &h, 300, 9));
        let (g, t) = jac_instance(&c, order, r, x, 3).unwrap();
        let (hg, ht) = (convert_div(&h, &g).unwrap(), convert_div(&h, &t).unwrap());
        let inst = instance_for_rung(&h, order, r, hg, ht, x).unwrap();
        for arm in [Arm::Full, Arm::SingleLp, Arm::DoubleLp] {
            let out = ic_on_rung(&inst, arm, 5, 5_000_000);
            assert!(out.certified, "{arm:?}: {out:?}");
            assert_eq!(out.recovered.map(|k| k % r), Some(x));
            assert!(out.la.mul > 0 && out.rel.mul > 0);
            if arm == Arm::DoubleLp {
                assert!(out.rels_by_lp[2] > 0, "no double-large-prime relation");
            }
        }
    }

    #[test]
    fn instance_from_matches_rung_instance_at_genus_three() {
        // The genus-3 path is the positive control's: same instance, same arms.
        let curve = super::super::ic_control::random_curve(101, 3).unwrap();
        let n2 = curve.count_points(2);
        let order = super::super::ic_control::jacobian_order_bsgs(&curve, n2, 3).unwrap();
        if let Some((h, r)) = super::super::ic_control::split_order(order, 64) {
            if r > 200 {
                let inst = instance_from(curve, order, h, r, 7).unwrap();
                let rung = instance_for_rung(
                    &inst.curve,
                    inst.order,
                    inst.r,
                    inst.g,
                    inst.target,
                    inst.t_secret,
                )
                .unwrap();
                let out = ic_on_rung(&rung, Arm::DoubleLp, 11, 5_000_000);
                assert!(out.certified);
            }
        }
    }

    #[test]
    fn fb_rule_reproduces_the_positive_control_at_genus_three() {
        let pc = |arm, p| super::super::ic_control::fb_size(arm, p, 2.0, 2.0);
        for p in [131u64, 4099, 65537] {
            assert_eq!(fb_size_g(Arm::SingleLp, 3, p, 2.0), pc(Arm::SingleLp, p));
            assert_eq!(fb_size_g(Arm::DoubleLp, 3, p, 2.0), pc(Arm::DoubleLp, p));
        }
        assert!(
            fb_size_g(Arm::DoubleLp, 2, 10_000, 2.0) < fb_size_g(Arm::SingleLp, 2, 10_000, 2.0)
        );
    }

    #[test]
    fn rho_split_adds_up_and_certifies() {
        use crate::cryptanalysis::chained_transfer::field::Fp;
        let k = Fp::new(1009).unwrap();
        let mut found = false;
        for a6 in 1..200u64 {
            let Some(e) = Curve::new(k, 0, 3, a6) else {
                continue;
            };
            let n = e.order();
            let r = largest_cyclic_prime(n);
            if r < 150 {
                continue;
            }
            let cof = n / r;
            let mut gen = None;
            for s in 1..500u64 {
                let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
                if g.is_some() && e.mul(&g, r as u128).is_none() {
                    gen = Some(g);
                    break;
                }
            }
            let Some(g) = gen else { continue };
            let x = 37 % r;
            let t = e.mul(&g, x as u128);
            for neg in [false, true] {
                let rr = rho_on_curve(&e, g, t, r, 5, neg);
                assert!(rr.certified && rr.recovered.map(|k| k % r) == Some(x));
                assert_eq!(rr.total_ops, rr.walk_ops + rr.other_ops);
                assert!(rr.other_ops > 0, "set-up is charged");
                assert_eq!(rr.native > 0, neg);
            }
            found = true;
            break;
        }
        assert!(found);
    }

    #[test]
    fn band_classes() {
        assert_eq!(band_class(&[1.0, 2.0, 3.0], &[2.0, 3.0, 4.0]), "below");
        assert_eq!(band_class(&[3.0, 4.0, 5.0], &[2.0, 3.0, 4.0]), "above");
        assert_eq!(band_class(&[1.0, 4.0, 5.0], &[2.0, 3.0, 4.0]), "unresolved");
        let (m, se) = mean_se(&[1.0, 3.0]);
        assert_eq!(m, 2.0);
        assert!((se - 1.0).abs() < 1e-12);
    }
}
