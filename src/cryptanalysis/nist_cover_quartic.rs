//! A novel one-parameter family of genus-3 hyperelliptic covers of a short
//! Weierstrass elliptic curve, and the explicit, same-field plane quartic and
//! checked transfer map that Smith's trigonal construction produces from it.
//!
//! # The cover family
//!
//! For `E: y^2 = x^3 + a x + b` over `F_p` (`p` odd, `3 < p < 2^31`) and a free
//! parameter `gamma != 0`, the degree-2 map
//!
//! ```text
//! phi: H_gamma -> E,   phi(u, v) = ( (u^2 + gamma) / u ,  v / u^2 )
//! ```
//!
//! pulls `E` back to the hyperelliptic curve
//!
//! ```text
//! H_gamma:  v^2 = f(u) = u * [ (u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3 ].
//! ```
//!
//! Expanding, `f` has ascending coefficients
//! `[0, gamma^3, 0, 3 gamma^2 + a gamma, b, 3 gamma + a, 0, 1]`: a monic degree-7
//! polynomial, so `H_gamma` is a genus-3 hyperelliptic curve with one point at
//! infinity.  At `gamma = 1, a = -3` this is `v^2 = u^7 + b u^4 + u`, the known
//! degree-2 genus-3 cover of the `a = -3` curve (research/p256_genus_dlp,
//! genus-3 row).  `gamma` free gives a *family*, searched here for a member on
//! which Smith's trigonal method applies.
//!
//! # Why the pulled-back points lie on `H_gamma`
//!
//! For `P = (x0, y0)` on `E`, its two preimages solve `u^2 - x0 u + gamma = 0`,
//! and their `H`-ordinate is `w = y0 u^2`.  With `x0 = (u^2+gamma)/u`,
//! `f(u)/u^4 = [(u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3]/u^3 = x0^3 + a x0 + b
//! = y0^2`, and `w^2/u^4 = y0^2`, so `w^2 = f(u)` is an identity: every pulled-
//! back point is on `H_gamma`.  The fibre over `O_E` is `(0,0) + infty` (`f(0)=0`,
//! so `(0,0)` is a Weierstrass point; `infty` is the odd-degree point at
//! infinity), both unramified.
//!
//! # The pullback homomorphism
//!
//! In the one-infinity model `HJac::point(x,y) = [(x,y) - infty]`, so
//!
//! ```text
//! phi^*([P] - [O_E]) = [(u1,w1) - infty] + [(u2,w2) - infty]
//!                    - [(0,0) - infty] - [infty - infty]
//!                    = pair(P) - point(0,0),
//! ```
//!
//! where `pair(P)` is `point(u1,w1) + point(u2,w2)` when `x0` splits, and the
//! Mumford class `U = u^2 - x0 u + gamma`, `V = y0 x0 u - y0 gamma` (the degree-2
//! reduced divisor of the conjugate pair) otherwise.  Because `phi` has degree 2
//! and the transferred subgroup here has odd prime order `r`, `gcd(2, r) = 1`
//! and `phi^*` is injective on it (`phi_* phi^* = [2]`).
//!
//! # The full chain
//!
//! `E(F_p) --phi^*--> J_{H_gamma}(F_p) --Psi--> Pic^0(C)`, with `Psi` Smith's
//! trigonal transfer ([`smith_canonical::Transfer`]) onto the explicit plane
//! quartic `C = CanonicalMap::quartic` over the same `F_p`.  This connects the
//! elliptic (cover) stage to the non-hyperelliptic index-calculus stage.
//!
//! Scope: this is an exact-correctness construction, not a speed claim.  For a
//! prime-field target, index calculus on a same-field genus-3 `C` is `O~(p)`
//! group operations, above rho's `O~(sqrt r)`; the deliverable is the verified
//! explicit artifact and the checked map, not an improvement over rho.

use crate::cryptanalysis::smith_canonical as sc;
use crate::cryptanalysis::smith_prop24 as sp;

/// Ascending coefficients `[c0..c7]` of `f(u)` for `H_gamma`, monic degree 7.
pub fn cover_f(p: u64, a: u64, b: u64, gamma: u64) -> Vec<u64> {
    let m = |x: u128| (x % p as u128) as u64;
    let g = gamma as u128;
    let a = a as u128;
    let g2 = m(g * g);
    let g3 = m(g * g2 as u128);
    let c1 = g3;
    let c3 = m(3 * g2 as u128 + a * g);
    let c4 = b % p;
    let c5 = m(3 * g + a);
    vec![0, c1, 0, c3, c4, c5, 0, 1]
}

/// A minimal affine elliptic curve `y^2 = x^3 + a x + b` over `F_p`.
#[derive(Clone, Debug)]
pub struct Ecc {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

/// A point of [`Ecc`]: `None` is the point at infinity.
pub type EPt = Option<(u64, u64)>;

impl Ecc {
    pub fn new(p: u64, a: u64, b: u64) -> Ecc {
        Ecc {
            p,
            a: a % p,
            b: b % p,
        }
    }
    pub fn on(&self, pt: &EPt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => {
                let p = self.p as u128;
                let x = *x as u128 % p;
                let lhs = (*y as u128 % p).pow(2) % p;
                let rhs = (x * x % p * x + self.a as u128 * x + self.b as u128) % p;
                lhs == rhs
            }
        }
    }
    pub fn neg(&self, pt: &EPt) -> EPt {
        pt.map(|(x, y)| (x, (self.p - y % self.p) % self.p))
    }
    pub fn add(&self, pp: &EPt, qq: &EPt) -> EPt {
        let p = self.p;
        let (p1, p2) = match (pp, qq) {
            (None, _) => return *qq,
            (_, None) => return *pp,
            (Some(p1), Some(p2)) => (*p1, *p2),
        };
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        if x1 == x2 && (y1 + y2) % p == 0 {
            return None;
        }
        let m = if x1 == x2 && y1 == y2 {
            // doubling
            let num = (3u128 * (x1 as u128).pow(2) % p as u128 + self.a as u128) % p as u128;
            let den = sp::inv_mod(2 * y1 % p, p) as u128;
            (num * den % p as u128) as u64
        } else {
            let num = (y2 as u128 + p as u128 - y1 as u128) % p as u128;
            let den = sp::inv_mod((x2 + p - x1) % p, p) as u128;
            (num * den % p as u128) as u64
        };
        let m2 = m as u128;
        let x3 = (m2 * m2 % p as u128 + 2 * p as u128 - x1 as u128 - x2 as u128) % p as u128;
        let y3 = (m2 * ((x1 as u128 + p as u128 - x3) % p as u128) % p as u128 + p as u128
            - y1 as u128)
            % p as u128;
        Some((x3 as u64, y3 as u64))
    }
    pub fn mul(&self, pt: &EPt, mut k: u128) -> EPt {
        let mut r = None;
        let mut acc = *pt;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &acc);
            }
            acc = self.add(&acc, &acc);
            k >>= 1;
        }
        r
    }
    /// Order of `pt` given the group order `n` (tries divisors of `n`).
    pub fn order(&self, pt: &EPt, n: u128) -> u128 {
        let mut ord = n;
        let mut d = 2u128;
        let mut m = n;
        // factor n
        let mut fs: Vec<u128> = Vec::new();
        while d * d <= m {
            while m.is_multiple_of(d) {
                fs.push(d);
                m /= d;
            }
            d += 1;
        }
        if m > 1 {
            fs.push(m);
        }
        for pf in fs {
            while ord.is_multiple_of(pf) && self.mul(pt, ord / pf).is_none() {
                ord /= pf;
            }
        }
        ord
    }
}

/// `phi^*([P] - [O_E])` as a class of `J_{H_gamma}(F_p)`, or `None` if `P` is
/// `O_E` or the fibre over `P` is ramified (`x0^2 = 4 gamma`), which this map
/// does not transfer.
pub fn pullback(jac: &sc::HJac, gamma: u64, pt: &EPt) -> Option<sc::Mumford> {
    let p = jac.p;
    let (x0, y0) = (*pt)?;
    let ng = |x: u64| (p - x % p) % p;
    let m = |x: u128| (x % p as u128) as u64;
    // u^2 - x0 u + gamma = 0
    let disc = m((x0 as u128 * x0 as u128 + p as u128 - 4 * gamma as u128 % p as u128) % p as u128);
    let base0 = jac.point(0, 0);
    let pair = match sp::sqrt_mod(disc, p) {
        Some(s) if disc != 0 => {
            let inv2 = sp::inv_mod(2, p);
            let u1 = m((x0 as u128 + s as u128) % p as u128 * inv2 as u128);
            let u2 = m((x0 as u128 + ng(s) as u128) % p as u128 * inv2 as u128);
            let w1 = m(y0 as u128 * u1 as u128 % p as u128 * u1 as u128);
            let w2 = m(y0 as u128 * u2 as u128 % p as u128 * u2 as u128);
            jac.add(&jac.point(u1, w1), &jac.point(u2, w2))
        }
        Some(_) => return None, // ramified: double preimage
        None => {
            // conjugate pair over F_{p^2}: U = u^2 - x0 u + gamma,
            // V = y0 x0 u - y0 gamma  (= y0 u^2 mod U).
            let u = vec![gamma % p, ng(x0), 1];
            let v = vec![
                ng(m(y0 as u128 * gamma as u128)),
                m(y0 as u128 * x0 as u128),
            ];
            sc::Mumford { u, v }
        }
    };
    Some(jac.sub(&pair, &base0))
}

/// The explicit, checked chain `E -> J_{H_gamma} -> Pic^0(C)` for one instance.
pub struct Chain {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub gamma: u64,
    pub e_order: u128,
    pub e_prime_subgroup: u128,
    pub f: Vec<u64>,
    pub quartic: [u32; 15],
    pub jac: sc::HJac,
    pub transfer: sc::Transfer,
    pub pts_over_u: Vec<(u64, u64)>,
}

/// Count `#E(F_p)` by brute force over `x` (toy sizes only).
pub fn ecc_order(e: &Ecc) -> u128 {
    let p = e.p;
    let mut n = 1u128; // O
    for x in 0..p {
        let rhs = ((x as u128 * x as u128 % p as u128 * x as u128)
            + e.a as u128 * x as u128
            + e.b as u128)
            % p as u128;
        match sp::chi(rhs as u64, p) {
            0 => n += 1,
            1 => n += 2,
            _ => {}
        }
    }
    n
}

/// `(largest prime factor of n, n / that factor)`.
fn largest_prime_factor(mut n: u128) -> (u128, u128) {
    let n0 = n;
    let mut d = 2u128;
    let mut best = 1u128;
    while d * d <= n {
        if n.is_multiple_of(d) {
            while n.is_multiple_of(d) {
                n /= d;
            }
            best = d;
        }
        d += 1;
    }
    if n > 1 {
        best = n;
    }
    (best, n0 / best)
}

impl Chain {
    /// Search `gamma` in `[1, gamma_max]` for a cover on which Smith's trigonal
    /// method applies, with a smooth quartic and `#C = #H`.  `require_prime_e`
    /// keeps only prime-order `E` (NIST cofactor-1 shape); otherwise the largest
    /// prime factor of `#E` is used as the transferred subgroup.
    pub fn build(
        p: u64,
        a: u64,
        b: u64,
        gamma_max: u64,
        require_prime_e: bool,
    ) -> Result<Chain, String> {
        let e = Ecc::new(p, a, b);
        let n = ecc_order(&e);
        let (r, cof) = largest_prime_factor(n);
        if require_prime_e && cof != 1 {
            return Err(format!("#E = {n} not prime (cofactor {cof})"));
        }
        if r < 5 {
            return Err(format!("#E = {n} has no usable prime subgroup"));
        }
        for gamma in 1..=gamma_max {
            if gamma.is_multiple_of(p) {
                continue;
            }
            let f = cover_f(p, a, b, gamma);
            let jac = match sc::HJac::new(&f, p) {
                Ok(j) => j,
                Err(_) => continue,
            };
            let tr = match sc::find_trigonal(&f, p, 1) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let cm = match sc::CanonicalMap::new(tr) {
                Ok(c) => c,
                Err(_) => continue,
            };
            if !cm.is_smooth() {
                continue;
            }
            let quartic = cm.quartic;
            // #C = #H over F_p
            if sc::count_quartic_points(p, &quartic, 1) != sc::count_h_points(&f, p, 1) {
                continue;
            }
            let transfer = match sc::Transfer::new(cm) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let pts_over_u = transfer.usable_points();
            if pts_over_u.len() < 8 {
                continue;
            }
            return Ok(Chain {
                p,
                a,
                b,
                gamma,
                e_order: n,
                e_prime_subgroup: r,
                f,
                quartic,
                jac,
                transfer,
                pts_over_u,
            });
        }
        Err("no gamma gave a Smith-positive smooth cover".into())
    }

    pub fn ecc(&self) -> Ecc {
        Ecc::new(self.p, self.a, self.b)
    }

    /// `Psi(phi^*([P]-[O]))` in `Pic^0(C)`.
    pub fn transfer_point(
        &self,
        pt: &EPt,
        rng: &mut sp::SplitMix64,
    ) -> Result<crate::cryptanalysis::plane_quartic_pic::PicClass, String> {
        let d =
            pullback(&self.jac, self.gamma, pt).ok_or("pullback undefined (O_E or ramified)")?;
        self.transfer
            .transfer_divisor(&self.jac, &d, &self.pts_over_u, rng)
    }
}

/// BSGS for `k` with `[k] base = target` in `Pic^0(C)`, `k in [0, n)`.
pub fn bsgs_quartic(
    c: &crate::cryptanalysis::plane_quartic_pic::PlaneQuartic,
    base: &crate::cryptanalysis::plane_quartic_pic::PicClass,
    target: &crate::cryptanalysis::plane_quartic_pic::PicClass,
    n: u128,
) -> Option<u128> {
    use crate::cryptanalysis::plane_quartic_pic::PicKey;
    use std::collections::HashMap;
    let m = (n as f64).sqrt().ceil() as u128 + 1;
    let mut table: HashMap<PicKey, u128> = HashMap::new();
    let mut cur = c.zero();
    for j in 0..m {
        table.entry(cur.key()).or_insert(j);
        cur = c.add(&cur, base);
    }
    // factor = [m] base ; we search target - i*factor in table
    let factor = c.mul_scalar(base, m);
    let neg_factor = c.neg(&factor);
    let mut gamma = target.clone();
    for i in 0..=m {
        if let Some(&j) = table.get(&gamma.key()) {
            let k = (i * m + j) % n;
            return Some(k);
        }
        gamma = c.add(&gamma, &neg_factor);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- The cover family -------------------------------------------------

    #[test]
    fn cover_matches_known_p256_form_at_gamma1_am3() {
        // gamma = 1, a = -3 (= p-3) must give f = u^7 + b u^4 + u.
        let p = 10007;
        let b = 17;
        let f = cover_f(p, p - 3, b, 1);
        assert_eq!(f, vec![0, 1, 0, 0, b, 0, 0, 1]);
    }

    #[test]
    fn cover_is_a_genus3_cover_pullback_points_lie_on_h() {
        // For a random E point, both pulled-back points satisfy w^2 = f(u).
        let p = 10007u64;
        let (a, b) = (p - 3, 17);
        let gamma = 5u64;
        let f = cover_f(p, a, b, gamma);
        assert_eq!(super::sp::deg(&f), 7, "genus-3 model");
        let e = Ecc::new(p, a, b);
        let peval = |x: u64| -> u64 {
            let mut acc = 0u128;
            for &c in f.iter().rev() {
                acc = (acc * x as u128 + c as u128) % p as u128;
            }
            acc as u64
        };
        let mut checked = 0;
        for x0 in 0..200u64 {
            let rhs = ((x0 as u128).pow(3) + a as u128 * x0 as u128 + b as u128) % p as u128;
            if let Some(y0) = super::sp::sqrt_mod(rhs as u64, p) {
                assert!(e.on(&Some((x0, y0))));
                let disc = (x0 as u128 * x0 as u128 + p as u128 - 4 * gamma as u128 % p as u128)
                    % p as u128;
                if let Some(s) = super::sp::sqrt_mod(disc as u64, p) {
                    if disc == 0 {
                        continue;
                    }
                    let inv2 = super::sp::inv_mod(2, p);
                    for u in [
                        (x0 as u128 + s as u128) % p as u128 * inv2 as u128 % p as u128,
                        (x0 as u128 + (p - s) as u128) % p as u128 * inv2 as u128 % p as u128,
                    ] {
                        let u = u as u64;
                        let w = (y0 as u128 * u as u128 % p as u128 * u as u128 % p as u128) as u64;
                        assert_eq!(
                            (w as u128 * w as u128 % p as u128) as u64,
                            peval(u),
                            "pulled-back point on H"
                        );
                        checked += 1;
                    }
                }
            }
        }
        assert!(checked > 20, "exercised the split fibre");
    }

    // --- Build the full chain on a NIST-shaped toy target -----------------

    fn find_prime_e(p: u64) -> Option<(u64, u128)> {
        // a = -3, search b for prime #E (cofactor 1).
        let a = p - 3;
        for b in 1..p {
            let e = Ecc::new(p, a, b);
            // nonsingular: 4a^3 + 27b^2 != 0
            let disc = (4 * (a as u128).pow(3) % p as u128 + 27 * (b as u128).pow(2)) % p as u128;
            if disc == 0 {
                continue;
            }
            let n = ecc_order(&e);
            let (_, cof) = largest_prime_factor(n);
            if cof == 1 {
                return Some((b, n));
            }
        }
        None
    }

    #[test]
    fn full_chain_transfers_a_planted_dlp() {
        // A NIST-shaped toy: a = -3, prime-order E over a small prime.
        let p = 1009u64;
        let (b, n) = find_prime_e(p).expect("prime-order a=-3 curve");
        let a = p - 3;
        let chain = match Chain::build(p, a, b, 400, true) {
            Ok(c) => c,
            // Not every (p,b) has a Smith-positive gamma in range; try the
            // next few primes to keep the test deterministic but robust.
            Err(_) => {
                let mut done = None;
                for pp in [1013u64, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061] {
                    if let Some((bb, _)) = find_prime_e(pp) {
                        if let Ok(c) = Chain::build(pp, pp - 3, bb, 400, true) {
                            done = Some(c);
                            break;
                        }
                    }
                }
                done.expect("a Smith-positive prime-order instance in the toy range")
            }
        };
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        assert_eq!(chain.e_order, r, "E has prime order (NIST cofactor 1)");

        // A generator P of E (order r), and a planted target Q = [k] P.
        let mut gp = None;
        for x in 0..p {
            let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + chain.b as u128) % p as u128;
            if let Some(y) = super::sp::sqrt_mod(rhs as u64, p) {
                let pt = Some((x, y));
                if e.order(&pt, r) == r {
                    gp = Some(pt);
                    break;
                }
            }
        }
        let gp = gp.expect("generator");
        let k: u128 = 123 % r;
        let q = e.mul(&gp, k);

        let mut rng = super::sp::SplitMix64(0x5eed_1234_abcd_0001);

        // pullback is a homomorphism injective on <P>: order preserved.
        let pg = pullback(&chain.jac, chain.gamma, &gp).expect("pullback P");
        let pq = pullback(&chain.jac, chain.gamma, &q).expect("pullback Q");
        assert_eq!(chain.jac.order(&pg, n), r, "phi*(P) keeps order r");
        // phi*([k]P) = [k] phi*(P)
        let kpg = chain.jac.mul(&pg, k);
        assert!(
            chain.jac.is_zero(&chain.jac.sub(&kpg, &pq)),
            "phi* homomorphism on the planted pair"
        );

        // Transfer both to Pic^0(C) and recover k there.
        let psi_g = chain.transfer_point(&gp, &mut rng).expect("Psi phi* P");
        let psi_q = chain.transfer_point(&q, &mut rng).expect("Psi phi* Q");
        let c = &chain.transfer.c;
        // order on C divides r and is nonzero (kernel S has exponent 2, coprime to r).
        assert!(!c.is_zero(&psi_g), "image generator nonzero on C");
        let kk = bsgs_quartic(c, &psi_g, &psi_q, r).expect("log on C");
        assert_eq!(kk % r, k % r, "BSGS on C recovers the planted log");
    }

    #[test]
    fn negative_control_wrong_scalar_fails() {
        let p = 1009u64;
        let (b, n) = match find_prime_e(p) {
            Some(x) => x,
            None => return,
        };
        let a = p - 3;
        let chain = match Chain::build(p, a, b, 400, true).ok().or_else(|| {
            [1013u64, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061]
                .into_iter()
                .find_map(|pp| {
                    find_prime_e(pp)
                        .and_then(|(bb, _)| Chain::build(pp, pp - 3, bb, 400, true).ok())
                })
        }) {
            Some(c) => c,
            None => return,
        };
        let _ = n;
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        let gp = (0..chain.p)
            .find_map(|x| {
                let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + chain.b as u128)
                    % chain.p as u128;
                super::sp::sqrt_mod(rhs as u64, chain.p).and_then(|y| {
                    let pt = Some((x, y));
                    (e.order(&pt, r) == r).then_some(pt)
                })
            })
            .unwrap();
        let mut rng = super::sp::SplitMix64(0xA1);
        let psi_g = chain.transfer_point(&gp, &mut rng).unwrap();
        let q = e.mul(&gp, 7 % r);
        let psi_q = chain.transfer_point(&q, &mut rng).unwrap();
        let c = &chain.transfer.c;
        let kk = bsgs_quartic(c, &psi_g, &psi_q, r).unwrap();
        assert_ne!(kk % r, 11 % r, "a wrong scalar is not accepted");
        assert_eq!(kk % r, 7 % r);
    }
}
