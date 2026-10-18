//! Prime-field volcanoes: CM instance search, exact exploration from the crater,
//! blind descent, and per-level curves for ecbench.
//!
//! Replaces the Python/PARI dataset builder of `research/volcano_descent_dataset_20261011`
//! with code built on the repository's own `isogeny_walk` machinery (`Φ_ℓ mod p` from the
//! `q`-expansion of `j`, Cantor–Zassenhaus root finding) and this module's Montgomery
//! arithmetic.  Craters are class-number-one CM curves (`D ∈ {-3,-4,-7,-8,-11,-19,-43,-67,
//! -163}`, conductor 1), so the whole volcano is the set of curves with trace `t`
//! reachable by ℓ-isogenies from `j(O_K)`.
//!
//! * [`find_instance`]: `p = (t² − D f²)/4` prime with `f = ℓ^k`, depth `k`; `m` is the
//!   requested lower bound on `v_ℓ(A)`, which makes `E[ℓ^m]` rational at the crater
//!   (the group structure then varies with the level).
//! * [`explore`]: multi-source BFS over the roots of `Φ_ℓ(X, j)`; BFS distance to the
//!   crater is the level.  Checked against the class numbers `h(O_{ℓ^i})` and the degree
//!   rule (ℓ + 1 rational kernels off the floor, 1 on it).
//! * [`blind_descent`]: from any vertex and the trace alone, find the floor.

use super::arith::*;
use super::level1::{class_one_j, curve_with_order};
use super::profile;
use crate::cryptanalysis::isogeny_walk::field::{Fe, Field};
use crate::cryptanalysis::isogeny_walk::modpoly::{modular_polynomial, ModPoly};
use crate::cryptanalysis::isogeny_walk::poly;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Instance {
    pub ell: u64,
    pub d: i64,
    pub k: u32,
    pub m: u32,
    pub p: u64,
    pub t: i64,
}

impl Instance {
    pub fn conductor(&self) -> u64 {
        self.ell.pow(self.k)
    }
    pub fn order(&self) -> u64 {
        (self.p as i128 + 1 - self.t as i128) as u64
    }
}

fn isqrt(n: i128) -> i128 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// Random `(p, t)` with `t² − 4p = D (ℓ^k)²`, `v_ℓ(A) ≥ m`, `p` of about `bits` bits.
/// `smooth_slack = Some(s)` also requires `#E = (product of primes below 1230, < 2^s) × (one prime)`.
pub fn find_instance(
    ell: u64,
    d: i64,
    k: u32,
    m: u32,
    bits: u32,
    seed: u64,
    smooth_slack: Option<u32>,
) -> Result<Instance, String> {
    class_one_j(d).ok_or("D must be a class-number-one fundamental discriminant")?;
    if !(3..=62).contains(&bits) || m > k {
        return Err("need 3 <= bits <= 62 and m <= k".into());
    }
    let f = ell.pow(k) as i128;
    let dd = d as i128;
    if (d.rem_euclid(8) == 1) && f % 2 == 1 {
        return Err(
            "local obstruction: D ≡ 1 (mod 8) with odd f makes N(π) even; use an even conductor"
                .into(),
        );
    }
    if (1i128 << (bits + 1)) <= -dd * f * f {
        return Err("bits too small for this depth: need p >= |D| f² / 4".into());
    }
    let t_hi = isqrt(4 * (1i128 << bits) + dd * f * f);
    let t_lo = isqrt(((1i128 << (bits + 1)) + dd * f * f).max(1)) + 1;
    let modulus = 2 * (ell as i128).pow(m);
    let r0 = (f * dd + 2).rem_euclid(modulus);
    let mut rng = Rng(seed | 1);
    let small_primes: Vec<u64> = (2..1230u64).filter(|&x| is_prime_u64(x)).collect();
    for _ in 0..2_000_000 {
        let mut t = t_lo + (rng.next() as i128).rem_euclid((t_hi - t_lo).max(1));
        t += (r0 - t).rem_euclid(modulus);
        if t >= t_hi || (t - f * dd).rem_euclid(2) != 0 {
            continue;
        }
        let p = (t * t - dd * f * f) / 4;
        if p < 7 || p >= (1i128 << 62) || p % ell as i128 == 0 || t % p == 0 {
            continue;
        }
        let p = p as u64;
        if !is_prime_u64(p) {
            continue;
        }
        if let Some(s) = smooth_slack {
            let mut n = (p as i128 + 1 - t) as u64;
            let mut smooth_bits = 0u32;
            for &q in &small_primes {
                while n % q == 0 {
                    n /= q;
                    smooth_bits += 64 - q.leading_zeros() - 1;
                }
            }
            if !(n == 1 || is_prime_u64(n)) || smooth_bits >= s {
                continue;
            }
        }
        return Ok(Instance {
            ell,
            d,
            k,
            m,
            p,
            t: t as i64,
        });
    }
    Err("no instance found (a local congruence may forbid this combination)".into())
}

/// `Φ_ℓ mod p` and the field it lives in.
pub struct Ctx {
    pub f: Field,
    pub phi: ModPoly,
    pub ell: u64,
    pub p: u64,
}

impl Ctx {
    pub fn new(p: u64, ell: u64) -> Result<Self, String> {
        let f = Field::new(&BigUint::from(p)).ok_or("p must be an odd prime")?;
        let phi = modular_polynomial(&f, ell)?;
        Ok(Ctx { f, phi, ell, p })
    }
    fn fe(&self, j: u64) -> Fe {
        self.f.from_u64(j)
    }
    /// Rational ℓ-neighbours of `j`: `(root, multiplicity)` and the total multiplicity.
    pub fn neighbors(&self, j: u64) -> (Vec<(u64, u32)>, u32) {
        let poly_y = self.phi.at_x(&self.f, &self.fe(j));
        let roots = poly::roots(&self.f, &poly_y, 0xA11CE);
        let mut out = Vec::new();
        let mut total = 0u32;
        for r in roots {
            let lin = vec![self.f.neg(&r), self.f.one()];
            let mut q = poly_y.clone();
            let mut mult = 0u32;
            loop {
                let (quo, rem) = poly::divrem(&self.f, &q, &lin);
                if poly::trim(&self.f, rem).is_empty() {
                    mult += 1;
                    q = quo;
                } else {
                    break;
                }
            }
            total += mult;
            out.push((self.f.to_big(&r).to_u64().unwrap(), mult));
        }
        (out, total)
    }
}

#[derive(Debug)]
pub struct Volcano {
    pub level: HashMap<u64, u32>,
    pub totals: HashMap<u64, u32>,
    /// `(src, dst, multiplicity)`.
    pub edges: Vec<(u64, u64, u32)>,
    pub order: Vec<u64>,
}

pub fn crater_j(inst: &Instance) -> u64 {
    (class_one_j(inst.d).unwrap() as i128).rem_euclid(inst.p as i128) as u64
}

pub fn explore(inst: &Instance, ctx: &Ctx, max_vertices: usize) -> Result<Volcano, String> {
    explore_to(inst, ctx, u32::MAX, max_vertices)
}

/// Breadth-first search from the crater that does not expand vertices at `max_level`: every
/// vertex of levels `0..=max_level` is found, and `totals` holds only the expanded ones.
pub fn explore_to(
    inst: &Instance,
    ctx: &Ctx,
    max_level: u32,
    max_vertices: usize,
) -> Result<Volcano, String> {
    let j0 = crater_j(inst);
    let mut level = HashMap::new();
    let mut totals = HashMap::new();
    let mut edges = Vec::new();
    let mut order = vec![j0];
    level.insert(j0, 0u32);
    let mut dq = VecDeque::from([j0]);
    while let Some(j) = dq.pop_front() {
        if level[&j] >= max_level {
            continue;
        }
        let (nb, tot) = ctx.neighbors(j);
        totals.insert(j, tot);
        for (r, e) in nb {
            edges.push((j, r, e));
            if !level.contains_key(&r) {
                level.insert(r, level[&j] + 1);
                order.push(r);
                dq.push_back(r);
                if level.len() > max_vertices {
                    return Err(format!("vertex cap {max_vertices} exceeded"));
                }
            }
        }
    }
    Ok(Volcano {
        level,
        totals,
        edges,
        order,
    })
}

/// Level sizes expected from the class numbers `h(O_{ℓ^i})`, `i = 0..=k`.
pub fn expected_level_sizes(inst: &Instance) -> Vec<BigUint> {
    (0..=inst.k)
        .map(|i| profile::class_number_of_order(inst.d, 1, &BigUint::from(inst.ell.pow(i))))
        .collect()
}

pub fn level_sizes(v: &Volcano, k: u32) -> Vec<usize> {
    let mut out = vec![0usize; k as usize + 1];
    for &l in v.level.values() {
        if (l as usize) < out.len() {
            out[l as usize] += 1;
        } else {
            out.resize(l as usize + 1, 0);
            out[l as usize] += 1;
        }
    }
    out
}

/// Degree rule: ℓ + 1 rational kernels (with multiplicity) off the floor, one on it.
pub fn degree_violations(inst: &Instance, v: &Volcano) -> usize {
    v.level
        .iter()
        .filter(|(j, &l)| v.totals[*j] != if l < inst.k { inst.ell as u32 + 1 } else { 1 })
        .count()
}

// ---------------------------------------------------------------- curves and groups
pub fn group_order_factors(n: u64) -> Vec<(u64, u32)> {
    let (f, cof) = profile::factor(&BigUint::from(n));
    assert!(cof.is_none());
    f.into_iter()
        .map(|(p, e)| (p.to_u64().unwrap(), e))
        .collect()
}

fn point_order(ec: &Ec, p: Pt, n: u64, fac: &[(u64, u32)]) -> u64 {
    let mut o = n;
    for &(q, _) in fac {
        while o % q == 0 && ec.mul(o / q, p).is_none() {
            o /= q;
        }
    }
    o
}

fn gcd64(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd64(b, a % b)
    }
}

/// `(n1, n2)` with `E(F_p) = Z/n1 × Z/n2` from the exponent of 24 random points.
pub fn group_structure(
    ec: &Ec,
    n: u64,
    fac: &[(u64, u32)],
    rng: &mut Rng,
) -> Result<(u64, u64), String> {
    let mut e = 1u64;
    for _ in 0..24 {
        let p = ec.random_point(rng);
        let o = point_order(ec, p, n, fac);
        e = e / gcd64(e, o) * o;
    }
    if n % e != 0 || e % (n / e) != 0 {
        return Err("exponent too small or group not 2-generated".into());
    }
    Ok((n / e, e))
}

/// `(n1, n2)` predicted by `O_d/(π − 1)` for a vertex at `level`.
pub fn predicted_structure(inst: &Instance, level: u32) -> Result<(u64, u64), String> {
    let prof = profile::analyze(
        BigUint::from(inst.p),
        BigInt::from(inst.t),
        Some((inst.d, BigUint::from(inst.conductor()))),
    )?;
    let rows = profile::level_table(&prof, &BigUint::from(inst.ell), &BigUint::one())?;
    let r = rows.get(level as usize).ok_or("level out of range")?;
    Ok((
        r.n1.to_u64().ok_or("n1 too large")?,
        r.n2.to_u64().ok_or("n2 too large")?,
    ))
}

pub fn curve_of(inst: &Instance, j: u64, rng: &mut Rng) -> Result<Ec, String> {
    let f = Mont::new(inst.p);
    curve_with_order(&f, j as i128, inst.order(), rng)
}

// ---------------------------------------------------------------- blind descent
fn avail(nb: &[(u64, u32)], prev: Option<u64>) -> Vec<(u64, u32)> {
    nb.iter()
        .filter_map(|&(r, e)| {
            let e2 = if Some(r) == prev { e - 1 } else { e };
            (e2 > 0).then_some((r, e2))
        })
        .collect()
}

fn walk_len(ctx: &Ctx, start: u64, prev: u64, rng: &mut Rng, cap: u32) -> Option<u32> {
    let (mut cur, mut pv) = (start, prev);
    for n in 0..=cap {
        let (nb, _) = ctx.neighbors(cur);
        let av = avail(&nb, Some(pv));
        if av.is_empty() {
            return Some(n);
        }
        let total: u32 = av.iter().map(|x| x.1).sum();
        let mut pick = (rng.next() % total as u64) as u32;
        let mut next = av[0].0;
        for (r, e) in &av {
            if pick < *e {
                next = *r;
                break;
            }
            pick -= *e;
        }
        pv = cur;
        cur = next;
    }
    None
}

#[derive(Debug)]
pub struct Descent {
    pub path: Vec<u64>,
    pub start_level: u32,
    pub k: u32,
}

/// Blind descent: needs only `p`, the trace (for the depth `k = v_ℓ(f)`), `ℓ` and a start `j`.
pub fn blind_descent(ctx: &Ctx, k: u32, j0: u64, rng: &mut Rng) -> Result<Descent, String> {
    if k == 0 {
        return Ok(Descent {
            path: vec![j0],
            start_level: 0,
            k,
        });
    }
    let (nb, tot) = ctx.neighbors(j0);
    let mut path = vec![j0];
    if tot == 1 {
        return Ok(Descent {
            path,
            start_level: k,
            k,
        });
    }
    let mut best: Option<(u64, u32)> = None;
    for &(r, _) in &nb {
        if r == j0 {
            continue; // horizontal loop
        }
        let mut shortest = None;
        for _ in 0..3 {
            if let Some(l) = walk_len(ctx, r, j0, rng, 2 * k + 3) {
                shortest = Some(shortest.map_or(l, |s: u32| s.min(l)));
            }
        }
        if let Some(l) = shortest {
            let m = l + 1;
            if best.is_none_or(|(_, bl)| m < bl) {
                best = Some((r, m));
            }
        }
    }
    let (first, first_len) = best.ok_or("no descending neighbour found")?;
    let start_level = k
        .checked_sub(first_len)
        .ok_or("walk longer than the depth")?;
    let (mut prev, mut cur) = (j0, first);
    path.push(cur);
    loop {
        let (nb, _) = ctx.neighbors(cur);
        let av = avail(&nb, Some(prev));
        match av.first() {
            None => break,
            Some(&(r, _)) => {
                prev = cur;
                cur = r;
                path.push(cur);
            }
        }
    }
    Ok(Descent {
        path,
        start_level,
        k,
    })
}

/// A random descent from the crater `j0` of a volcano whose crater is one vertex (class
/// number one): leave the crater along a uniformly chosen non-loop kernel, then take a uniformly
/// chosen kernel other than the one back to the parent, `depth` steps in all.  Returns the path
/// with each vertex's rational-kernel count, checked against the degree rule (ℓ + 1 above the
/// floor, 1 on it); a violation is an error, never a silent skip.
pub fn random_descent(
    ctx: &Ctx,
    j0: u64,
    depth: u32,
    rng: &mut Rng,
) -> Result<Vec<(u64, u32)>, String> {
    let want = |l: u32| if l < depth { ctx.ell as u32 + 1 } else { 1 };
    let (nb, tot) = ctx.neighbors(j0);
    if tot != want(0) {
        return Err(format!(
            "crater has {tot} rational kernels, expected {}",
            want(0)
        ));
    }
    let mut path = vec![(j0, tot)];
    let mut av: Vec<(u64, u32)> = nb.into_iter().filter(|&(r, _)| r != j0).collect();
    for l in 1..=depth {
        let total: u32 = av.iter().map(|x| x.1).sum();
        if total == 0 {
            return Err(format!("dead end at level {}", l - 1));
        }
        let mut pick = (rng.next() % total as u64) as u32;
        let mut next = av[0].0;
        for &(r, e) in &av {
            if pick < e {
                next = r;
                break;
            }
            pick -= e;
        }
        let (nb, tot) = ctx.neighbors(next);
        if tot != want(l) {
            return Err(format!(
                "vertex {next} at level {l} has {tot} rational kernels, expected {}",
                want(l)
            ));
        }
        path.push((next, tot));
        av = avail(&nb, Some(path[path.len() - 2].0));
    }
    Ok(path)
}

// ---------------------------------------------------------------- ecbench curves
#[derive(Clone, Debug)]
pub struct VertexCurve {
    pub level: u32,
    pub j: u64,
    pub a: u64,
    pub b: u64,
    pub group_order: u64,
    pub r: u64,
    pub gx: u64,
    pub gy: u64,
    pub n1: u64,
    pub n2: u64,
}

/// A curve with trace `t` at `j`, its group structure, and a generator of the largest
/// prime-order subgroup (for ecbench's `prime_explicit`).
pub fn vertex_curve(
    inst: &Instance,
    level: u32,
    j: u64,
    rng: &mut Rng,
) -> Result<VertexCurve, String> {
    let ec = curve_of(inst, j, rng)?;
    let n = inst.order();
    let fac = group_order_factors(n);
    let r = fac.iter().map(|x| x.0).max().unwrap();
    let cof = n / r;
    let (n1, n2) = group_structure(&ec, n, &fac, rng)?;
    let g = loop {
        let p = ec.random_point(rng);
        let q = ec.mul(cof, p);
        if q.is_some() {
            break q.unwrap();
        }
    };
    let f = ec.f;
    Ok(VertexCurve {
        level,
        j,
        a: f.from_mont(ec.a),
        b: f.from_mont(ec.b),
        group_order: n,
        r,
        gx: f.from_mont(g.0),
        gy: f.from_mont(g.1),
        n1,
        n2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(ell: u64, d: i64, k: u32, m: u32, bits: u32) -> (Instance, Ctx, Volcano) {
        let inst = find_instance(ell, d, k, m, bits, 7, None).unwrap();
        let ctx = Ctx::new(inst.p, ell).unwrap();
        let v = explore(&inst, &ctx, 1_000_000).unwrap();
        (inst, ctx, v)
    }

    #[test]
    fn level_sizes_equal_class_numbers_and_degrees_hold() {
        for (ell, d, k, m, bits) in [
            (2u64, -7i64, 6u32, 0u32, 36u32),
            (2, -4, 5, 3, 36),
            (3, -8, 4, 2, 36),
            (5, -4, 3, 1, 36),
            (2, -3, 5, 2, 36),
            (3, -3, 4, 0, 36),
        ] {
            let (inst, _ctx, v) = run(ell, d, k, m, bits);
            let want: Vec<usize> = expected_level_sizes(&inst)
                .iter()
                .map(|x| x.to_usize().unwrap())
                .collect();
            assert_eq!(level_sizes(&v, k), want, "ell={ell} D={d}");
            assert_eq!(degree_violations(&inst, &v), 0, "ell={ell} D={d}");
        }
    }

    #[test]
    fn group_structure_matches_the_ideal_quotient_at_every_level() {
        for (ell, d, k, m, bits) in [
            (2u64, -7i64, 6u32, 6u32, 40u32),
            (3, -8, 4, 2, 40),
            (2, -4, 5, 5, 40),
        ] {
            let (inst, _ctx, v) = run(ell, d, k, m, bits);
            let mut rng = Rng(11);
            let mut by_level: HashMap<u32, Vec<u64>> = HashMap::new();
            for (&j, &l) in &v.level {
                by_level.entry(l).or_default().push(j);
            }
            for (l, js) in by_level {
                let j = js[0];
                let ec = curve_of(&inst, j, &mut rng).unwrap();
                let fac = group_order_factors(inst.order());
                let got = group_structure(&ec, inst.order(), &fac, &mut rng).unwrap();
                assert_eq!(
                    got,
                    predicted_structure(&inst, l).unwrap(),
                    "level {l}, ell={ell} D={d}"
                );
            }
        }
    }

    #[test]
    fn blind_descent_agrees_with_exploration_including_the_automorphism_craters() {
        for (ell, d, k, m, bits) in [
            (2u64, -3i64, 5u32, 3u32, 36u32),
            (2, -4, 5, 2, 36),
            (3, -4, 4, 1, 36),
            (2, -7, 6, 0, 36),
        ] {
            let (inst, ctx, v) = run(ell, d, k, m, bits);
            let mut rng = Rng(5);
            let mut starts: Vec<u64> = v
                .level
                .iter()
                .filter(|(_, &l)| l <= 1)
                .map(|(&j, _)| j)
                .collect();
            starts.extend(v.order.iter().step_by(7).take(20));
            for j in starts {
                let dsc = blind_descent(&ctx, k, j, &mut rng).unwrap();
                let floor = *dsc.path.last().unwrap();
                assert_eq!(v.level[&floor], k, "ell={ell} D={d} start {j}");
                assert_eq!(dsc.start_level, v.level[&j], "ell={ell} D={d} start {j}");
                assert_eq!(dsc.path.len() as u32 - 1, k - v.level[&j]);
            }
            let _ = inst;
        }
    }

    #[test]
    fn truncated_search_and_random_descents_agree_with_full_exploration() {
        for (ell, d, k, m, bits) in [
            (2u64, -7i64, 7u32, 0u32, 36u32),
            (2, -3, 6, 0, 36),
            (2, -4, 6, 0, 36),
            (3, -3, 4, 0, 36),
            (3, -11, 4, 0, 36),
            (5, -19, 3, 0, 40),
        ] {
            let (inst, ctx, v) = run(ell, d, k, m, bits);
            let top = k / 2;
            let t = explore_to(&inst, &ctx, top, 1_000_000).unwrap();
            assert_eq!(
                level_sizes(&t, top),
                level_sizes(&v, k)[..=top as usize].to_vec(),
                "ell={ell} D={d}"
            );
            let mut rng = Rng(3);
            for _ in 0..8 {
                let path = random_descent(&ctx, crater_j(&inst), k, &mut rng).unwrap();
                assert_eq!(path.len() as u32, k + 1);
                for (l, (j, _)) in path.iter().enumerate() {
                    assert_eq!(v.level[j], l as u32, "ell={ell} D={d}");
                }
            }
        }
    }

    #[test]
    fn local_obstruction_is_reported() {
        assert!(find_instance(11, -7, 3, 0, 40, 1, None).is_err());
    }
}
