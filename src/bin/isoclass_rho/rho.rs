//! Single-walker distinguished-point Pollard rho with an `r`-adding walk
//! (Teske), generic over the field, with optional folding of the walk by a
//! group of cheap maps on `⟨P⟩`: negation, the automorphisms of a `j = 0`
//! or `j = 1728` curve, or the Frobenius `τ` of a Koblitz curve (with or
//! without negation).  A folded walk steps from the canonical
//! representative of the orbit, tracks the scalar the fold multiplied by,
//! prevents 2-cycles by the look-ahead rule and escapes longer fruitless
//! cycles deterministically from the orbit's minimum.  The unfolded walk
//! with `r = 32` reproduces `volcano-rho`'s walk exactly (same hash, same
//! seeds, same steps).

use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::field::{inv_mod, mulmod, Curve, Fq};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fold {
    None,
    Neg,
    /// Automorphisms of a `j = 0` (order 6) or `j = 1728` (order 4) curve.
    Aut,
    /// Frobenius `τ: (x, y) ↦ (x², y²)` of a Koblitz curve (orbit size `m`).
    Frob,
    /// `τ` and negation (orbit size `2m`).
    FrobNeg,
}

impl Fold {
    pub fn name(&self) -> &'static str {
        match self {
            Fold::None => "none",
            Fold::Neg => "neg",
            Fold::Aut => "aut",
            Fold::Frob => "frob",
            Fold::FrobNeg => "frobneg",
        }
    }
}

/// A group of maps on the curve with their scalars on `⟨P⟩`: each element
/// is `(x-multiplier, y-multiplier, λ)` for `(x, y) ↦ (ζx, ηy) = [λ](x, y)`
/// (prime fields), or a Frobenius power (binary).
#[derive(Clone, Debug)]
pub struct Folder {
    pub fold: Fold,
    /// Prime-field linear maps, the identity first.
    pub maps: Vec<(u64, u64, u64)>,
    /// Frobenius eigenvalue `λ_τ` on `⟨P⟩` and its powers `λ_τ^i`.
    pub tau_pows: Vec<u64>,
    pub m: u32,
    pub orbit: u64,
}

impl Folder {
    pub fn none() -> Self {
        Folder {
            fold: Fold::None,
            maps: vec![(1, 1, 1)],
            tau_pows: vec![1],
            m: 0,
            orbit: 1,
        }
    }

    /// The canonical representative of the orbit of `pt` and the scalar
    /// `λ` with `rep = [λ]pt`.
    #[inline]
    pub fn canon<F: Fq>(&self, f: &F, c: &Curve, n: u64, pt: (u64, u64)) -> ((u64, u64), u64) {
        match self.fold {
            Fold::None => (pt, 1),
            Fold::Neg => {
                let neg = f.ec_neg(c, Some(pt)).unwrap();
                if (neg.1, neg.0) < (pt.1, pt.0) {
                    (neg, n - 1)
                } else {
                    (pt, 1)
                }
            }
            Fold::Aut => {
                let mut best = (pt, 1u64);
                for &(zx, zy, lam) in &self.maps[1..] {
                    let q = (f.fmul(zx, pt.0), f.fmul(zy, pt.1));
                    if (q.0, q.1) < (best.0 .0, best.0 .1) {
                        best = (q, lam);
                    }
                }
                best
            }
            Fold::Frob | Fold::FrobNeg => {
                let with_neg = self.fold == Fold::FrobNeg;
                let (mut x, mut y) = pt;
                let mut best = (pt, 1u64);
                if with_neg {
                    let q = (x, x ^ y);
                    if q.1 < y {
                        best = (q, n - 1);
                    }
                }
                for i in 1..self.m as usize {
                    x = f.fsqr(x);
                    y = f.fsqr(y);
                    let lam = self.tau_pows[i];
                    if (x, y) < (best.0 .0, best.0 .1) {
                        best = ((x, y), lam);
                    }
                    if with_neg {
                        let q = (x, x ^ y);
                        if (q.0, q.1) < (best.0 .0, best.0 .1) {
                            best = (q, mulmod(n - 1, lam, n));
                        }
                    }
                }
                best
            }
        }
    }
}

pub struct Instance {
    pub curve: Curve,
    pub p: (u64, u64),
    pub q: (u64, u64),
}

#[derive(Clone, Copy, Debug)]
pub struct Solve {
    pub k: Option<u64>,
    pub iters: u64,
    pub restarts: u32,
    pub dps: u64,
    pub escapes: u32,
    pub ns: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct RhoCfg {
    /// Adding-walk terms (a power of two, ≥ 4).
    pub r: usize,
    pub dp_bits: u32,
    /// Restart after this many DP-free steps, in units of `2^dp_bits`.
    pub max_run_factor: u64,
}

const CYCLE_BETA: u64 = 48;

/// One solve of `Q = kP`, `P` of prime order `n`.
pub fn rho<F: Fq>(f: &F, inst: &Instance, n: u64, seed: u64, cfg: &RhoCfg, fold: &Folder) -> Solve {
    let e = &inst.curve;
    let (p, q) = (Some(inst.p), Some(inst.q));
    let mut rng = SmallRng::seed_from_u64(seed);
    let t0 = Instant::now();
    let r = cfg.r;
    let shift = 64 - r.trailing_zeros();
    let steps: Vec<(u64, u64, (u64, u64))> = (0..r)
        .map(|_| loop {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(0..n));
            if let Some(s) = f.ec_add(e, f.ec_mul(e, p, a), f.ec_mul(e, q, b)) {
                break (a, b, s);
            }
        })
        .collect();
    let mut table: HashMap<u64, (u64, u64, u64)> = HashMap::new();
    let (mut iters, mut restarts, mut dps, mut escapes) = (0u64, 0u32, 0u64, 0u32);
    let dp_mask = (1u64 << cfg.dp_bits) - 1;
    let max_run = cfg.max_run_factor << cfg.dp_bits;
    let folded = fold.fold != Fold::None;
    let part = |x: u64| (f.hash_x(x).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> shift) as usize;
    let is_dp = |x: u64| f.hash_x(x).wrapping_mul(0xC2B2_AE3D_27D4_EB4F) >> 40 & dp_mask == 0;
    let finish = |k: Option<u64>, iters: u64, restarts: u32, dps: u64, escapes: u32| Solve {
        k,
        iters,
        restarts,
        dps,
        escapes,
        ns: t0.elapsed().as_nanos(),
    };
    'restart: loop {
        let (mut a, mut b) = (rng.gen_range(0..n), rng.gen_range(0..n));
        let Some(mut x) = f.ec_add(e, f.ec_mul(e, p, a), f.ec_mul(e, q, b)) else {
            restarts += 1;
            continue;
        };
        if folded {
            let (cx, lam) = fold.canon(f, e, n, x);
            x = cx;
            a = mulmod(a, lam, n);
            b = mulmod(b, lam, n);
        }
        let mut run = 0u64;
        let mut saved: Option<((u64, u64), u64)> = None; // (point, step) for cycle checks
        let mut step_no = 0u64;
        loop {
            // one step: x ← canon(x + M_j), with look-ahead against 2-cycles
            let mut j = part(x.0);
            let (nx, na, nb) = loop {
                iters += 1;
                match f.ec_add(e, Some(x), Some(steps[j].2)) {
                    Some(y) => {
                        let (a2, b2) = ((a + steps[j].0) % n, (b + steps[j].1) % n);
                        if !folded {
                            break (y, a2, b2);
                        }
                        let (cy, lam) = fold.canon(f, e, n, y);
                        if part(cy.0) == j {
                            // the same term would be added back: skip to the next term
                            j = (j + 1) % r;
                            continue;
                        }
                        break (cy, mulmod(a2, lam, n), mulmod(b2, lam, n));
                    }
                    None => {
                        // aP + bQ + M_j = O
                        let (a2, b2) = ((a + steps[j].0) % n, (b + steps[j].1) % n);
                        if b2 != 0 {
                            let k = mulmod(n - a2 % n, inv_mod(b2, n), n) % n;
                            return finish(Some(k), iters, restarts, dps, escapes);
                        }
                        restarts += 1;
                        continue 'restart;
                    }
                }
            };
            x = nx;
            a = na;
            b = nb;
            run += 1;
            step_no += 1;
            if is_dp(x.0) {
                dps += 1;
                if let Some(&(y2, a2, b2)) = table.get(&x.0) {
                    // same point: (a − a2)P = (b2 − b)Q; opposite: (a + a2)P = −(b + b2)Q
                    let (num, den) = if y2 == x.1 {
                        ((a + n - a2) % n, (b2 + n - b) % n)
                    } else {
                        ((n - (a + a2) % n) % n, (b + b2) % n)
                    };
                    if den != 0 {
                        let k = mulmod(num, inv_mod(den, n), n);
                        return finish(Some(k), iters, restarts, dps, escapes);
                    }
                    restarts += 1;
                    continue 'restart;
                }
                table.insert(x.0, (x.1, a, b));
                run = 0;
            }
            if folded {
                // fruitless-cycle check against the point saved CYCLE_BETA steps ago
                match saved {
                    Some((sp, s0)) if sp == x => {
                        // in a cycle of length dividing step_no − s0: walk it,
                        // escape from its minimum by doubling
                        let len = step_no - s0;
                        let (mut cur, mut ca, mut cb) = (x, a, b);
                        let (mut best, mut ba, mut bb) = (x, a, b);
                        for _ in 0..len {
                            let jj = part(cur.0);
                            let (mut jj2, mut nxt) = (jj, None);
                            while nxt.is_none() {
                                let y = f.ec_add(e, Some(cur), Some(steps[jj2].2));
                                let Some(y) = y else {
                                    restarts += 1;
                                    continue 'restart;
                                };
                                let (cy, lam) = fold.canon(f, e, n, y);
                                if part(cy.0) == jj2 {
                                    jj2 = (jj2 + 1) % r;
                                    continue;
                                }
                                nxt = Some((
                                    cy,
                                    mulmod((ca + steps[jj2].0) % n, lam, n),
                                    mulmod((cb + steps[jj2].1) % n, lam, n),
                                ));
                            }
                            let (cy, a2, b2) = nxt.unwrap();
                            iters += 1;
                            cur = cy;
                            ca = a2;
                            cb = b2;
                            if (cur.0, cur.1) < (best.0, best.1) {
                                best = cur;
                                ba = ca;
                                bb = cb;
                            }
                        }
                        // escape: double the minimum
                        let Some(d) = f.ec_dbl(e, Some(best)) else {
                            restarts += 1;
                            continue 'restart;
                        };
                        let (cd, lam) = fold.canon(f, e, n, d);
                        iters += 1;
                        x = cd;
                        a = mulmod((ba + ba) % n, lam, n);
                        b = mulmod((bb + bb) % n, lam, n);
                        escapes += 1;
                        saved = None;
                    }
                    _ => {}
                }
                if step_no.is_multiple_of(CYCLE_BETA) {
                    saved = Some((x, step_no));
                }
            }
            if run > max_run {
                restarts += 1;
                continue 'restart;
            }
        }
    }
}

/// Verify that `(zx, zy)` acts on `P` as multiplication by `lam`: find the
/// scalar among the candidates.
pub fn eigenvalue<F: Fq>(
    f: &F,
    c: &Curve,
    p: (u64, u64),
    image: (u64, u64),
    cands: &[u64],
) -> Option<u64> {
    cands
        .iter()
        .copied()
        .find(|&l| f.ec_mul(c, Some(p), l) == Some(image))
}

/// Build the automorphism folder of a `j = 0` or `j = 1728` curve over
/// `F_p`, with its scalars on `⟨P⟩` of order `n`.
pub fn folder_aut(
    fp: &super::field::Fp,
    c: &Curve,
    p: (u64, u64),
    n: u64,
) -> Result<Folder, String> {
    let f = *fp;
    let nf = super::field::Fp { p: n };
    let j = f.ec_j(c).ok_or("singular")?;
    let mut maps = vec![(1u64, 1u64, 1u64)];
    if j == 0 {
        // ζ₃ in F_p, (x, y) ↦ (ζx, y) = [λ₃]; with negation the group of order 6
        let s = f.sqrt(f.fneg(3)).ok_or("p ≢ 1 mod 3")?;
        let z3 = f.fmul(f.fsub(s, 1), f.finv(2));
        let sn = nf.sqrt(nf.fneg(3)).ok_or("n ≢ 1 mod 3")?;
        let l3a = nf.fmul(nf.fsub(sn, 1), nf.finv(2));
        let l3b = nf.fmul(nf.fsub(nf.fneg(sn), 1), nf.finv(2));
        let img = (f.fmul(z3, p.0), p.1);
        let l3 = eigenvalue(&f, c, p, img, &[l3a, l3b]).ok_or("no eigenvalue for ζ₃")?;
        let z3sq = f.fmul(z3, z3);
        let l3sq = nf.fmul(l3, l3);
        maps.push((z3, 1, l3));
        maps.push((z3sq, 1, l3sq));
        maps.push((1, f.fneg(1), n - 1));
        maps.push((z3, f.fneg(1), nf.fneg(l3)));
        maps.push((z3sq, f.fneg(1), nf.fneg(l3sq)));
    } else if j == 1728 % f.p {
        // i in F_p, (x, y) ↦ (−x, iy) = [λ₄]
        let i = f.sqrt(f.fneg(1)).ok_or("p ≢ 1 mod 4")?;
        let ni = nf.sqrt(nf.fneg(1)).ok_or("n ≢ 1 mod 4")?;
        let img = (f.fneg(p.0), f.fmul(i, p.1));
        let l4 = eigenvalue(&f, c, p, img, &[ni, nf.fneg(ni)]).ok_or("no eigenvalue for i")?;
        maps.push((f.fneg(1), i, l4));
        maps.push((1, f.fneg(1), n - 1));
        maps.push((f.fneg(1), f.fneg(i), nf.fneg(l4)));
    } else {
        return Err("fold aut needs j = 0 or j = 1728".into());
    }
    let orbit = maps.len() as u64;
    Ok(Folder {
        fold: Fold::Aut,
        maps,
        tau_pows: vec![1],
        m: 0,
        orbit,
    })
}

/// Build the Frobenius folder of a Koblitz curve (`a, b ∈ F_2`) over
/// `F_{2^m}`: `τ(P) = [λ_τ]P` with `λ_τ² − μλ_τ + 2 ≡ 0 (mod n)`.
pub fn folder_frob(
    f2: &super::field::F2m,
    c: &Curve,
    p: (u64, u64),
    n: u64,
    with_neg: bool,
) -> Result<Folder, String> {
    let f = *f2;
    if c.a > 1 || c.b != 1 {
        return Err("fold frob needs a Koblitz curve y² + xy = x³ + ax² + 1, a ∈ {0, 1}".into());
    }
    let nf = super::field::Fp { p: n };
    let mu = if c.a == 1 { 1u64 } else { n - 1 };
    let s = nf.sqrt(nf.fneg(7)).ok_or("−7 is not a square mod n")?;
    let inv2 = nf.finv(2);
    let la = nf.fmul(nf.fadd(mu, s), inv2);
    let lb = nf.fmul(nf.fsub(mu, s), inv2);
    let img = (f.fsqr(p.0), f.fsqr(p.1));
    let lam = eigenvalue(&f, c, p, img, &[la, lb]).ok_or("no eigenvalue for τ")?;
    let mut pows = vec![1u64];
    for i in 1..f.m as usize {
        pows.push(nf.fmul(pows[i - 1], lam));
    }
    assert_eq!(nf.fmul(pows[f.m as usize - 1], lam), 1, "λ_τ^m ≠ 1 mod n");
    Ok(Folder {
        fold: if with_neg { Fold::FrobNeg } else { Fold::Frob },
        maps: vec![(1, 1, 1)],
        tau_pows: pows,
        m: f.m,
        orbit: if with_neg { 2 * f.m as u64 } else { f.m as u64 },
    })
}
