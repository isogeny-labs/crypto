//! # Tower up: one elliptic discrete log carried into genus-2 and genus-3 Jacobians.
//!
//! For a cubic model `E : Y² = c(s)`, `c(s) = c0 + c1 s + c2 s² + c3 s³`, and
//! `a ∈ F_p^*`, put `Y_ = X − a²` and
//!
//! ```text
//!   F(X) = c3·Y_ + c2·Y_² + c1·Y_³ + c0·Y_⁴,      C : y² = F(x²).
//! ```
//!
//! Then `φ : C → E`, `(x, y) ↦ (s, Y) = (1/(x² − a²), y·s²)`, is a degree-2
//! cover: `F(x²) = c(s)/s⁴`. The genus of `C` is `deg F − 1`: **3** when
//! `c0 ≠ 0`, **2** when `c0 = 0 ≠ c1` (a 2-torsion point of `E` at `s = 0`).
//! So the *same* curve `E`, written in two models related by a translation of
//! `s`, covers both a genus-2 and a genus-3 curve: one rung up the tower, then
//! another.
//!
//! The Jacobian is the repository's word-size Cantor group
//! [`HCurve`] (`p < 256`), on the odd model obtained by sending the rational
//! Weierstrass point `(a, 0)` to infinity: `t = 1/(x − a)`,
//! `Yc = y·t^{g+1}`.  The pull-back is explicit:
//!
//! ```text
//!   φ^*([Q] − [O]) = [P₊ − ∞] + [P₋ − ∞] + [W − ∞],     W = (−a, 0),
//! ```
//!
//! because `φ^*(O) = (a, 0) + (−a, 0) = ∞ + W` and `[W − ∞]` is 2-torsion.  It
//! is evaluated where the two preimages `P± = (±√(a² + 1/s), Y/s²)` are
//! `F_p`-rational; every other multiple of the generator is reached through
//! the homomorphism, and the homomorphism is checked on every multiple whose
//! preimages are rational.

use super::ec::{Curve, Pt};
use super::field::Fp;
use crate::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};

fn padd(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| k.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}

fn pmul(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = k.add(c[i + j], k.mul(x, y));
        }
    }
    c
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && *a.last().unwrap() == 0 {
        a.pop();
    }
    a
}

/// A degree-2 cover `C → E` of genus 2 or 3 in the shape above.
#[derive(Clone)]
pub struct Cover {
    pub p: u64,
    pub genus: u32,
    pub a: u64,
    /// Model cubic `c(s)`, lowest degree first.
    pub c: [u64; 4],
    pub curve: HCurve,
    k: Fp,
}

impl Cover {
    /// The cover for model `c` and parameter `a`; `Err` when the curve is
    /// singular, the genus is not 2 or 3, or `p` is out of `HCurve`'s range.
    pub fn new(p: u64, c: [u64; 4], a: u64) -> Result<Cover, String> {
        let k = Fp::new(p).ok_or("p is not an odd prime below 2^31")?;
        let c = c.map(|v| v % p);
        let a = a % p;
        if a == 0 {
            return Err("a must be nonzero".into());
        }
        let genus = if c[0] != 0 {
            3
        } else if c[1] != 0 {
            2
        } else {
            return Err("c0 = c1 = 0: not a cover of genus 2 or 3".into());
        };
        // F(Y_) with Y_ = x² − a², as a polynomial in x.
        let e = [0u64, c[3], c[2], c[1], c[0]];
        let ypoly = vec![k.neg(k.mul(a, a)), 0, 1];
        let mut f = vec![0u64];
        let mut ypow = vec![1u64];
        for &em in e.iter().skip(1) {
            ypow = pmul(&k, &ypow, &ypoly);
            f = padd(&k, &f, &pmul(&k, &ypow, &[em]));
        }
        let f = trim(f);
        let n = 2 * genus as usize + 2;
        if f.len() != n + 1 {
            return Err(format!("deg f = {} is not 2g + 2 = {n}", f.len() - 1));
        }
        // Odd model: g(t) = t^n f(a + 1/t) = Σ_j f_j (a t + 1)^j t^{n−j}.
        let mut g = vec![0u64; n + 1];
        let lin = [1u64, a];
        let mut powj = vec![1u64];
        for (j, &fj) in f.iter().enumerate() {
            if j > 0 {
                powj = pmul(&k, &powj, &lin);
            }
            for (i, &v) in powj.iter().enumerate() {
                let d = i + n - j;
                g[d] = k.add(g[d], k.mul(fj, v));
            }
        }
        if g[n] != 0 {
            return Err("(a, 0) is not on C".into());
        }
        let curve = HCurve::new(p, genus, &g[..n])?;
        Ok(Cover {
            p,
            genus,
            a,
            c,
            curve,
            k,
        })
    }

    /// `[P − ∞]` for an affine point `(x, y)` of `C` (original coordinates),
    /// `x ≠ a`.
    fn place(&self, x: u64, y: u64) -> Option<HDiv> {
        let k = &self.k;
        let dx = k.sub(x % self.p, self.a);
        if dx == 0 {
            return None;
        }
        let t = k.inv(dx);
        let yc = k.mul(y % self.p, k.pow(t, u64::from(self.genus) + 1));
        self.curve.point(t, yc)
    }

    /// `φ^*([Q] − [O])` for a model point `Q = (s, Y)`, when both preimages are
    /// `F_p`-rational and distinct; `None` otherwise.
    pub fn pull(&self, s: u64, yq: u64) -> Option<HDiv> {
        let k = &self.k;
        if s.is_multiple_of(self.p) {
            return None;
        }
        let x2 = k.add(k.mul(self.a, self.a), k.inv(s));
        if x2 == 0 {
            return None; // ramified (x = 0)
        }
        let r = k.sqrt(x2)?;
        let y = k.mul(yq, k.inv(k.mul(s, s)));
        let p1 = self.place(r, y)?;
        let p2 = self.place(k.neg(r), y)?;
        let w = self.place(k.neg(self.a), 0)?;
        let mut ops = GroupOps::default();
        let d = CountedGroup::add(&self.curve, &mut ops, p1, p2);
        Some(CountedGroup::add(&self.curve, &mut ops, d, w))
    }
}

/// `c(s + σ)` for a monic cubic `c = [a6, a4, a2, 1]`.
pub fn translate(k: &Fp, c: [u64; 4], sigma: u64) -> [u64; 4] {
    let (a6, a4, a2) = (c[0], c[1], c[2]);
    let s = sigma % k.p;
    let s2 = k.mul(s, s);
    let s3 = k.mul(s2, s);
    [
        k.add(k.add(s3, k.mul(a2, s2)), k.add(k.mul(a4, s), a6)),
        k.add(k.add(k.mul(3, s2), k.mul(k.mul(2, a2), s)), a4),
        k.add(k.mul(3, s), a2),
        1,
    ]
}

/// One target elliptic curve with its genus-2 and genus-3 covers and a
/// verified order-`r` discrete-log instance carried into both Jacobians.
pub struct TowerInstance {
    pub p: u64,
    pub e: Curve,
    pub r: u64,
    pub x: u64,
    pub gen: Pt,
    pub target: Pt,
    /// `(cover, translation σ, D = φ^*(P), T = [x]D, #J, homomorphism checks)`.
    pub rungs: Vec<Rung>,
}

pub struct Rung {
    pub cover: Cover,
    pub sigma: u64,
    pub d: HDiv,
    pub t: HDiv,
    pub jac_order: u64,
    pub hom_checks: u32,
}

fn largest_cyclic_odd_prime(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 && d > 2 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 2 {
        best = best.max(n);
    }
    best
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Rung {
    fn pull_pt(&self, pt: &Pt) -> Option<HDiv> {
        let (x, y) = (*pt)?;
        let k = &self.cover.k;
        self.cover.pull(k.sub(x.a, self.sigma), y.a)
    }
}

/// Build a tower instance over `F_p` (`p < 256`) from a seed, or `None`.
pub fn build(p: u64, seed: u64) -> Option<TowerInstance> {
    let k = Fp::new(p)?;
    for attempt in 0..4000u64 {
        let h = mix(seed.wrapping_mul(1_000_003).wrapping_add(attempt));
        let (a2, a4, a6) = (h % p, (h >> 16) % p, (h >> 32) % p);
        let Some(e) = Curve::new(k, a2, a4, a6) else {
            continue;
        };
        let roots = e.rational_two_torsion();
        if roots.is_empty() {
            continue;
        }
        let n = e.order();
        let r = largest_cyclic_odd_prime(n);
        if r < 11 {
            continue;
        }
        let c = [a6 % p, a4 % p, a2 % p, 1];
        let rho = roots[0].unwrap().0.a;
        // Genus-3 model: c(σ) ≠ 0.  Genus-2 model: σ = a rational root.
        let sigma3 = (0..p).find(|&s| translate(&k, c, s)[0] != 0)?;
        let mut rungs = vec![];
        for (sigma, want) in [(rho, 2u32), (sigma3, 3u32)] {
            let model = translate(&k, c, sigma);
            let mut found = None;
            for ai in 1..p {
                let a = (ai + (h >> 48)) % p;
                let Ok(cov) = Cover::new(p, model, a) else {
                    continue;
                };
                if cov.genus != want {
                    continue;
                }
                let Ok(jo) = cov.curve.jacobian_order(r, seed ^ ai) else {
                    continue;
                };
                if !jo.is_multiple_of(r) || (jo / r).is_multiple_of(r) {
                    continue;
                }
                found = Some((cov, jo));
                break;
            }
            let (cover, jac_order) = found?;
            rungs.push(Rung {
                cover,
                sigma,
                d: HDiv::IDENTITY,
                t: HDiv::IDENTITY,
                jac_order,
                hom_checks: 0,
            });
        }
        if rungs.len() != 2 {
            continue;
        }
        // A generator of E[r] whose image pulls back rationally on both rungs.
        let cof = (n / r) as u128;
        let mut chosen = None;
        for s in 1..3000u64 {
            let g = e.mul(&e.point_from_seed(mix(seed ^ s)), cof);
            if g.is_none() || e.mul(&g, r as u128).is_some() {
                continue;
            }
            let ds: Vec<Option<HDiv>> = rungs.iter().map(|rg| rg.pull_pt(&g)).collect();
            if ds.iter().all(|d| d.is_some()) {
                chosen = Some((g, ds));
                break;
            }
        }
        let (gen, ds) = chosen?;
        let x = 1 + mix(seed ^ 0x5151) % (r - 1);
        let target = e.mul(&gen, x as u128);
        for (rg, d) in rungs.iter_mut().zip(ds) {
            let d = d.unwrap();
            let cur = &rg.cover.curve;
            // ord(D) = r.
            if d.is_identity() || !cur.mul_uncounted(d, r).is_identity() {
                return None;
            }
            // Homomorphism: φ^*([k]P) = [k]φ^*(P) wherever the left side is defined.
            let mut checks = 0;
            for kk in 2..r.min(80) {
                let q = e.mul(&gen, kk as u128);
                if let Some(dq) = rg.pull_pt(&q) {
                    assert_eq!(dq, cur.mul_uncounted(d, kk), "φ^* is a homomorphism");
                    checks += 1;
                }
            }
            rg.d = d;
            rg.t = cur.mul_uncounted(d, x);
            rg.hom_checks = checks;
        }
        return Some(TowerInstance {
            p,
            e,
            r,
            x,
            gen,
            target,
            rungs,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_carries_the_log_into_genus_two_and_three() {
        let mut built = 0;
        for (i, p) in [101u64, 151, 199].into_iter().enumerate() {
            let Some(t) = build(p, 7 + i as u64) else {
                continue;
            };
            assert_eq!(t.rungs[0].cover.genus, 2);
            assert_eq!(t.rungs[1].cover.genus, 3);
            for rg in &t.rungs {
                assert!(
                    rg.hom_checks >= 3,
                    "enough homomorphism checks: {}",
                    rg.hom_checks
                );
                assert!(rg.jac_order.is_multiple_of(t.r));
                assert_eq!(rg.t, rg.cover.curve.mul_uncounted(rg.d, t.x));
            }
            built += 1;
        }
        assert!(built >= 2, "built {built} towers");
    }
}
