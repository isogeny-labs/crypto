//! Smith's genus-3 isogeny as a DLP transfer: the explicit map
//! `Psi: J_H(F_p) -> Pic^0(C)(F_p)` from a Smith-positive hyperelliptic
//! Jacobian to a plane-quartic Jacobian, wired to `quartic_ladder`'s solvers.
//!
//! Source: B. Smith, "Isogenies and the discrete logarithm problem in
//! Jacobians of genus 3 hyperelliptic curves", Eurocrypt 2008 / J. Cryptology
//! 22 (2009), arXiv:0806.2995 (the (2,2,2)-isogeny `Psi = psi_* . phi`, kernel
//! `S` of order 8, composed of Recillas' trigonal correspondence `R` and the
//! canonical map `psi: X -> C`).
//!
//! # What this module is and is not
//!
//! The isogeny itself is NOT new code here: the explicit map is built by
//! [`smith_trigonal`](super::smith_trigonal) (Recillas' curve `X`, the
//! correspondence `R`) and [`smith_canonical`](super::smith_canonical)
//! (`CanonicalMap`: `psi` as a linear map of the Veronese point of `X`;
//! `Transfer`: `Psi` on classes of `J_H(F_p)` in Mumford form, by moving them
//! onto rational points over `U`). `research/genus_ladder_20261114/RESULTS.md`
//! section 3 ("not built") predates that study. This module is the piece that
//! was missing between it and the solver instrument:
//!
//! 1. [`SmithIso`]: one handle (curve, `Psi`, `#J_H`, usable points);
//! 2. [`SmithIso::check_homomorphism`]: `Psi(D1 + D2) = Psi(D1) + Psi(D2)` on
//!    random classes, `Psi(D) = 0` iff `D` is a rational 2-torsion class in
//!    `S`, and `[#J_H] Psi(D) = 0`;
//! 3. [`SmithIso::planted`]: a planted DLP `(G, T = [t]G)` on `J_H` of prime
//!    order `r` (odd, hence `Psi` is injective on the subgroup), both classes
//!    transferred independently, packaged as a [`QInstance`] for
//!    `quartic_ladder::run_diem` / `run_rho` with the secret withheld;
//! 4. [`SmithIso::certify`]: a solver's answer `t'` is accepted only if
//!    `[t'] G = T` on the HYPERELLIPTIC side (Cantor arithmetic in `HJac`).
//!
//! Scope: toy primes (`HJac`, point counting and root finding are brute
//! force), `deg F = 7`. Nothing here is a cost claim; the transfer's own
//! counted cost is `smith_canonical::CountedTransfer`.

use super::plane_quartic_pic::PicClass;
use super::quartic_ladder::{make_instance_from_classes, QInstance};
use super::smith_canonical::{
    count_h_points, find_trigonal, jac_order, rational_two_torsion, two_torsion_in_s, CanonicalMap,
    HJac, Mumford, Transfer,
};
use super::smith_prop24::{deg, SplitMix64};

/// The divisor-class Smith map (`Smith`, `Hyp`, `Mum`, `Pic`, `W1Bridge`, the
/// T1-T6 engines), restored from before the 2026-11-16 sparse merge; used by
/// [`super::smith_route_s`] and `examples/smith_isogeny.rs`.
pub mod divisor_map;
pub use divisor_map::*;

/// Smith's isogeny on one curve, with everything needed to move classes.
pub struct SmithIso {
    pub p: u64,
    pub f: Vec<u64>,
    pub jac: HJac,
    pub tf: Transfer,
    /// Rational points of `H` over `U` on which `Psi` is evaluated.
    pub pts: Vec<(u64, u64)>,
    /// `#J_H(F_p)` from `N_1, N_2, N_3`.
    pub order: u128,
}

/// A planted DLP on `J_H` and its image on `C`.
pub struct Planted {
    pub g_h: Mumford,
    pub t_h: Mumford,
    /// The planted logarithm (kept for checking; the instance does not carry it).
    pub t: u64,
    pub r: u64,
    pub h: u64,
    /// The instance on `C` with `t_secret = 0` (unknown to the solvers).
    pub inst: QInstance,
}

#[derive(Clone, Debug, Default)]
pub struct HomReport {
    pub additivity: (usize, usize),
    pub annihilated: (usize, usize),
    pub two_torsion: (usize, usize),
    pub nonzero_images: usize,
}

impl HomReport {
    pub fn all_pass(&self) -> bool {
        self.additivity.0 == self.additivity.1
            && self.annihilated.0 == self.annihilated.1
            && self.two_torsion.0 == self.two_torsion.1
            && self.additivity.1 > 0
    }
}

fn largest_prime_factor(mut n: u128) -> u128 {
    let mut best = 1;
    let mut d = 2u128;
    while d * d <= n {
        while n % d == 0 {
            best = d;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        best = n;
    }
    best
}

impl SmithIso {
    /// Build `Psi` for `y^2 = f(x)`, `deg f = 7`. `Err` when the curve is not
    /// Smith-positive, the fitted quartic is singular, or `J_H` has no
    /// usable arithmetic.
    pub fn new(f: &[u64], p: u64, seed: u64) -> Result<SmithIso, String> {
        let tr = find_trigonal(f, p, seed)?;
        Self::from_trigonal_map(tr)
    }

    pub fn from_trigonal_map(tr: super::smith_trigonal::Trigonal) -> Result<SmithIso, String> {
        let p = tr.p;
        let f = tr.f.clone();
        if deg(&f) != 7 {
            return Err("smith_isogeny needs deg f = 7".into());
        }
        let n = [
            count_h_points(&f, p, 1),
            count_h_points(&f, p, 2),
            count_h_points(&f, p, 3),
        ];
        let order = jac_order(p, n);
        if order <= 0 {
            return Err("nonpositive #J".into());
        }
        let jac = HJac::new(&f, p)?;
        let cm = CanonicalMap::new(tr)?;
        let tf = Transfer::new(cm)?;
        let pts = tf.usable_points();
        if pts.len() < 8 {
            return Err("too few usable points".into());
        }
        Ok(SmithIso {
            p,
            f,
            jac,
            tf,
            pts,
            order: order as u128,
        })
    }

    /// A class of `J_H(F_p)` built from usable points.
    pub fn random_class(&self, rng: &mut SplitMix64) -> Mumford {
        let p = self.p;
        let mut m = self.jac.zero();
        for _ in 0..3 {
            let a = self.pts[rng.below(self.pts.len() as u64) as usize];
            let b = self.pts[rng.below(self.pts.len() as u64) as usize];
            m = self.jac.add(&m, &self.jac.point(a.0, a.1));
            m = self.jac.add(&m, &self.jac.point(b.0, (p - b.1 % p) % p));
        }
        m
    }

    /// `Psi(D)` (representation-independent; checked by the tests).
    pub fn image(&self, d: &Mumford, rng: &mut SplitMix64) -> Result<PicClass, String> {
        self.tf.transfer_divisor(&self.jac, d, &self.pts, rng)
    }

    /// Homomorphism checks on `n` random pairs and every rational 2-torsion
    /// class. A failure of any entry is a bug or a broken construction.
    pub fn check_homomorphism(&self, n: usize, seed: u64) -> HomReport {
        let mut rng = SplitMix64(seed);
        let c = &self.tf.c;
        let mut rep = HomReport::default();
        for _ in 0..n {
            let d1 = self.random_class(&mut rng);
            let d2 = self.random_class(&mut rng);
            let (Ok(a1), Ok(a2), Ok(a12)) = (
                self.image(&d1, &mut rng),
                self.image(&d2, &mut rng),
                self.image(&self.jac.add(&d1, &d2), &mut rng),
            ) else {
                continue;
            };
            rep.additivity.1 += 1;
            rep.additivity.0 += (a12 == c.add(&a1, &a2)) as usize;
            rep.nonzero_images += (!c.is_zero(&a1)) as usize;
            rep.annihilated.1 += 1;
            rep.annihilated.0 += c.is_zero(&c.mul_scalar(&a1, self.order)) as usize;
        }
        for u in rational_two_torsion(&self.f, self.p, &mut rng) {
            let d = Mumford {
                u: u.clone(),
                v: Vec::new(),
            };
            let Ok(img) = self.image(&d, &mut rng) else {
                continue;
            };
            rep.two_torsion.1 += 1;
            rep.two_torsion.0 += (c.is_zero(&img) == two_torsion_in_s(&self.tf.cm.tr, &u)) as usize;
        }
        rep
    }

    /// Plant `(G, T = [t]G)` of prime order `r = largest prime factor of
    /// #J_H` (odd, cofactor `h <= max_h`). Both classes are transferred
    /// independently (`T` is `[t]G` computed on `H`, then mapped).
    pub fn planted(&self, max_h: u64, seed: u64) -> Result<Planted, String> {
        let r128 = largest_prime_factor(self.order);
        if r128 < 5 || r128 > u64::MAX as u128 / 4 {
            return Err(format!("bad largest prime factor {r128}"));
        }
        let h128 = self.order / r128;
        if h128 > max_h as u128 || h128 >= r128 {
            return Err(format!(
                "not admitted: #J = {} = {h128} * {r128}",
                self.order
            ));
        }
        let (r, h) = (r128 as u64, h128 as u64);
        let mut rng = SplitMix64(seed ^ 0x5eed_1507_0910);
        let mut g_h = None;
        for _ in 0..64 {
            let d = self.random_class(&mut rng);
            let g = self.jac.mul(&d, h as u128);
            if !self.jac.is_zero(&g) {
                g_h = Some(g);
                break;
            }
        }
        let g_h = g_h.ok_or("no generator of order r on J_H")?;
        if !self.jac.is_zero(&self.jac.mul(&g_h, r as u128)) {
            return Err("[r] G != 0 on J_H".into());
        }
        let t = 1 + rng.below(r - 1);
        let t_h = self.jac.mul(&g_h, t as u128);
        let g_c = self.image(&g_h, &mut rng)?;
        let t_c = self.image(&t_h, &mut rng)?;
        let c = &self.tf.c;
        let n_points = c.rational_points_bruteforce().len() as u64;
        let q = self.p as u32;
        let order = u64::try_from(self.order).map_err(|_| "order too large")?;
        // `Some(t)` makes the constructor verify T = [t] G on C (uncharged);
        // the stored secret is then cleared so no solver can read it.
        let mut inst = make_instance_from_classes(
            q,
            self.tf.cm.quartic,
            c.base_point(),
            &g_c,
            &t_c,
            order,
            h,
            r,
            n_points,
            Some(t),
        )?;
        inst.t_secret = 0;
        Ok(Planted {
            g_h,
            t_h,
            t,
            r,
            h,
            inst,
        })
    }

    /// A recovered logarithm is accepted only if `[t'] G = T` on `J_H`.
    pub fn certify(&self, pl: &Planted, recovered: Option<u64>) -> bool {
        recovered.is_some_and(|t| self.jac.mul(&pl.g_h, t as u128) == pl.t_h)
    }
}

/// Smoke search: the first Smith-positive curve over `F_p` (random monic-free
/// `deg 7` models from `seed`) with `#J_H = h r`, `r` prime, `h <= max_h`.
pub fn find_admitted(p: u64, seed: u64, max_h: u64, tries: u64) -> Option<SmithIso> {
    let mut rng = SplitMix64(seed ^ p.rotate_left(29));
    for _ in 0..tries {
        let mut f: Vec<u64> = (0..7).map(|_| rng.below(p)).collect();
        f.push(1 + rng.below(p - 1));
        let Ok(tr) = find_trigonal(&f, p, rng.next_u64()) else {
            continue;
        };
        let n = [
            count_h_points(&f, p, 1),
            count_h_points(&f, p, 2),
            count_h_points(&f, p, 3),
        ];
        let ord = jac_order(p, n);
        if ord <= 0 {
            continue;
        }
        let r = largest_prime_factor(ord as u128);
        if r < 5 || (ord as u128 / r) > max_h as u128 || ord as u128 / r >= r {
            continue;
        }
        if let Ok(iso) = SmithIso::from_trigonal_map(tr) {
            return Some(iso);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::super::quartic_ladder::{run_diem, run_rho, QArm, QIcParams};
    use super::super::smith_prop24::{EXAMPLE1_F, EXAMPLE1_ND, EXAMPLE1_P};
    use super::super::smith_trigonal::Trigonal;
    use super::*;

    fn example1() -> SmithIso {
        let (n1, n0, d1, d0) = EXAMPLE1_ND;
        let tr = Trigonal::new(&EXAMPLE1_F, EXAMPLE1_P, n1, n0, d1, d0).unwrap();
        SmithIso::from_trigonal_map(tr).unwrap()
    }

    #[test]
    fn ex1_order_and_homomorphism() {
        let iso = example1();
        assert_eq!(iso.order, 55666); // printed zeta numerator, Smith Example 1
        let rep = iso.check_homomorphism(12, 0x1507);
        eprintln!("ISO ex1 hom {rep:?}");
        assert!(rep.all_pass(), "{rep:?}");
        assert!(rep.nonzero_images >= 6);
    }

    #[test]
    fn ex1_planted_dlp_all_solvers() {
        let iso = example1();
        let pl = iso.planted(32, 3).unwrap();
        assert_eq!((pl.h, pl.r), (26, 2141)); // 55666 = 2 * 13 * 2141
        assert_eq!(pl.inst.t_secret, 0);
        // Negative control: a wrong logarithm is rejected on H.
        assert!(!iso.certify(&pl, Some(pl.t % pl.r + 1)));
        assert!(!iso.certify(&pl, None));
        for arm in [QArm::Full, QArm::DoubleLp] {
            let res = run_diem(
                &pl.inst,
                QIcParams {
                    arm,
                    nb: (3.0 * f64::from(pl.inst.q).sqrt()).ceil() as usize,
                    seed: 5,
                    max_lines: 1 << 22,
                    verify_relations: true,
                },
            );
            eprintln!(
                "ISO ex1 {arm:?} rec={:?} cert_c={} ops={}",
                res.recovered, res.certified, res.group_ops
            );
            if arm == QArm::Full {
                assert_eq!(res.recovered, Some(pl.t), "{arm:?}");
            }
            // At q = 37 the 2LP base (19 of ~42 points) can exhaust without a
            // solution; a reported answer must still certify on H.
            assert!(res.recovered.is_none() || iso.certify(&pl, res.recovered));
        }
        let rho = run_rho(&pl.inst, 11);
        assert_eq!(rho.recovered, Some(pl.t));
        assert!(iso.certify(&pl, rho.recovered));
    }

    /// SMOKE, not a verdict: larger toy primes. `cargo test --release
    /// smoke_transfer -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn smoke_transfer() {
        for (p, seed) in [(67u64, 1u64), (101, 2), (131, 3)] {
            let Some(iso) = find_admitted(p, seed, 16, 20_000) else {
                eprintln!("SMOKE p={p}: no admitted Smith-positive curve found");
                continue;
            };
            let rep = iso.check_homomorphism(10, seed);
            let pl = iso.planted(16, seed).unwrap();
            let d = run_diem(
                &pl.inst,
                QIcParams {
                    arm: QArm::DoubleLp,
                    nb: (3.0 * (p as f64).sqrt()).ceil() as usize,
                    seed: 1,
                    max_lines: 1 << 24,
                    verify_relations: false,
                },
            );
            let rho = run_rho(&pl.inst, 1);
            eprintln!(
                "SMOKE p={p} #J={} h={} r={} hom_ok={} diem2lp_rec_ok={} ops={} rho_ok={} rho_ops={}",
                iso.order, pl.h, pl.r, rep.all_pass(),
                iso.certify(&pl, d.recovered), d.group_ops,
                iso.certify(&pl, rho.recovered), rho.group_ops
            );
            assert!(rep.all_pass());
        }
    }
}
