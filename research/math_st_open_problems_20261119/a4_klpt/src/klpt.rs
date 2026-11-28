//! Basic KLPT (Kohel-Lauter-Petit-Tignol 2014), plain variant, for ell in {2,3}.
//!
//! (a) delta in I with nrd(delta)/N(I) = N prime  -> I' = I conj(delta)/N(I), norm N.
//! (b) gamma with nrd(gamma) = N ell^e0 (Cornacchia).
//! (c) mu0 = C j + D k = (C + D i) j with gamma mu0 in I' (linear algebra mod N).
//! (d) strong approximation: mu = N(a + b i) + c' j + d' k, (c',d') = lambda (C,D) mod N,
//!     nrd(mu) = ell^e1; (c,d) is a solution of the linear congruence mod N, taken with
//!     coordinates reduced mod N (original variant: no lattice/CVP reduction of (c,d)).
//! (e) beta = gamma mu in I', x = beta delta / N in I, nrd(x)/N(I) = ell^(e0+e1).

use crate::arith::*;
use crate::quat::*;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;

#[derive(Default, Clone, Debug)]
pub struct Stats {
    pub delta_box: i64,
    pub delta_tries: u64,
    pub gamma_pairs_tried: u64,
    pub gamma_found: u64,
    pub gamma_rejected_l2_parity: u64,
    pub gamma_rejected_nonprimitive: u64,
    pub gamma_rejected_no_cd: u64,
    pub gamma_rejected_cd_isotropic: u64,
    pub gamma_rejected_residuosity: u64,
    pub gamma_rejected_e1_cap: u64,
    pub e0_bumps: u64,
    pub e1_bumps: u64,
    pub e1_parity_bumps: u64,
    pub sa_candidates: u64,
    pub sa_parity_skips: u64,
    pub sa_negative: u64,
    pub sa_no_sos: u64,
    pub nonprimitive_rejects: u64,
    pub internal_errors: u64,
}

impl Stats {
    /// Retries = every restart of a step after a failure.
    pub fn retries(&self) -> u64 {
        self.gamma_rejected_l2_parity
            + self.gamma_rejected_nonprimitive
            + self.gamma_rejected_no_cd
            + self.gamma_rejected_cd_isotropic
            + self.gamma_rejected_residuosity
            + self.gamma_rejected_e1_cap
            + self.e0_bumps
            + self.e1_bumps
            + self.e1_parity_bumps
            + self.nonprimitive_rejects
    }
}

pub struct Output {
    pub x: [Z; 4],
    pub n_prime: Z,
    pub e0_rule: u32,
    pub e1_rule: u32,
    pub e0: u32,
    pub e1: u32,
    pub stats: Stats,
}

pub fn pow_z(b: u32, e: u32) -> Z {
    num_traits::pow(Z::from(b), e as usize)
}

fn centered(a: &Z, m: &Z) -> Z {
    let r = modp(a, m);
    if Z::from(2) * &r > *m {
        r - m
    } else {
        r
    }
}

fn to_i128(a: &Z, n: i128) -> i128 {
    let r = modp(a, &Z::from(n));
    r.to_i128().unwrap()
}

fn inv_i128(a: i128, n: i128) -> i128 {
    inv_mod(&Z::from(a), &Z::from(n)).expect("not invertible").to_i128().unwrap()
}

/// Annihilator (as functionals) of the row space of `rows` modulo prime n.
fn annihilator(rows: &[[i128; 4]], n: i128) -> (usize, Vec<[i128; 4]>) {
    let mut m: Vec<[i128; 4]> = rows.iter().map(|r| r.map(|x| x.rem_euclid(n))).collect();
    let mut pivots: Vec<usize> = Vec::new();
    let mut r = 0usize;
    for col in 0..4 {
        let piv = (r..m.len()).find(|&i| m[i][col] != 0);
        let Some(pi) = piv else { continue };
        m.swap(r, pi);
        let inv = inv_i128(m[r][col], n);
        for c in 0..4 {
            m[r][c] = m[r][c] * inv % n;
        }
        for i in 0..m.len() {
            if i != r && m[i][col] != 0 {
                let f = m[i][col];
                for c in 0..4 {
                    m[i][c] = (m[i][c] - f * m[r][c]).rem_euclid(n);
                }
            }
        }
        pivots.push(col);
        r += 1;
    }
    let rank = r;
    let free: Vec<usize> = (0..4).filter(|c| !pivots.contains(c)).collect();
    let mut out = Vec::new();
    for &fc in free.iter() {
        let mut v = [0i128; 4];
        v[fc] = 1;
        for (ri, &pc) in pivots.iter().enumerate() {
            v[pc] = (-m[ri][fc]).rem_euclid(n);
        }
        out.push(v);
    }
    (rank, out)
}

/// Step (a): random small combinations of the LLL-reduced basis, box [-m,m]^4,
/// m = 1, 2, ...; first combination with nrd/N(I) an odd prime != ell, != p.
fn step_a(
    rng: &mut ChaCha20Rng,
    p: &Z,
    ell: u32,
    nid: &Z,
    red: &[[Z; 4]; 4],
    st: &mut Stats,
) -> ([Z; 4], Z) {
    for m in 1..=6i64 {
        let mut cands: Vec<[i64; 4]> = Vec::new();
        for a in -m..=m {
            for b in -m..=m {
                for c in -m..=m {
                    for d in -m..=m {
                        let v = [a, b, c, d];
                        let first = v.iter().find(|&&t| t != 0);
                        if first.map_or(false, |&t| t > 0) {
                            cands.push(v);
                        }
                    }
                }
            }
        }
        cands.shuffle(rng);
        for c in cands {
            st.delta_tries += 1;
            let cz = c.map(z);
            let v = combo(&cz, red);
            let nv = order_norm(p, &v);
            let (q, r) = nv.div_rem(nid);
            assert!(r.is_zero());
            if q.is_odd() && q != Z::from(ell) && &q != p && is_prime(&q) {
                st.delta_box = m;
                return (v, q);
            }
        }
    }
    panic!("no prime-norm element in box 6");
}

/// Minimal e with base^e >= x.
fn min_exp(base: u32, x: &Z) -> u32 {
    let mut e = 0u32;
    let mut v = Z::one();
    while &v < x {
        v *= base;
        e += 1;
    }
    e
}

/// Candidate gamma for (u,v) in the disk: ell=3 -> gamma in Z<i,j>,
/// a^2+b^2 = T - p(u^2+v^2); ell=2 -> gamma = (X + Y i + u j + v k)/2 in O0,
/// X^2+Y^2 = 4T - p(u^2+v^2), X = u, Y = v mod 2.
fn gamma_from_pair(p: &Z, ell: u32, t: &Z, u: i64, v: i64, small: &[u32]) -> Option<(Q, bool)> {
    let (uz, vz) = (z(u), z(v));
    let s = p * (&uz * &uz + &vz * &vz);
    if ell == 2 {
        let m = Z::from(4) * t - s;
        let (x, y) = sum_two_squares(&m, small)?;
        let par = |a: &Z, b: &Z| (a.is_odd() == uz.is_odd()) && (b.is_odd() == vz.is_odd());
        let (x, y) = if par(&x, &y) {
            (x, y)
        } else if par(&y, &x) {
            (y, x)
        } else {
            return Some((Q::int([0, 0, 0, 0]), false)); // parity impossible
        };
        Some((Q::new([x, y, uz, vz], z(2)), true))
    } else {
        let m = t - s;
        let (a, b) = sum_two_squares(&m, small)?;
        Some((Q::new([a, b, uz, vz], z(1)), true))
    }
}

pub fn klpt(
    rng: &mut ChaCha20Rng,
    p: &Z,
    ell: u32,
    id: &Ideal,
    nid: &Z,
    red: &[[Z; 4]; 4],
    small: &[u32],
) -> Result<Output, String> {
    let mut st = Stats::default();
    let ellz = Z::from(ell);
    // (a)
    let (dv, n) = step_a(rng, p, ell, nid, red, &mut st);
    let delta = from_order(&dv);
    let idp = equivalent(p, id, nid, &delta).ok_or("step a: I' not integral")?;
    if idp.norm().as_ref() != Some(&n) {
        return Err("step a: N(I') != N".into());
    }
    let ni = n.to_i128().unwrap();
    let rows: Vec<[i128; 4]> = idp.b.iter().map(|r| r.clone().map(|x| to_i128(&x, ni))).collect();
    let (rank, ann) = annihilator(&rows, ni);
    if rank != 2 {
        return Err(format!("step c: I'/N O0 has rank {} != 2", rank));
    }
    // e0 rule: minimal e0 with N ell^e0 >= p ln p  (about pi ln p candidate pairs)
    let lnp = ln_big(p);
    let pl = (p.to_f64().unwrap() * lnp).ceil() as u64;
    let e0_rule = min_exp(ell, &((Z::from(pl) + &n - 1u32) / &n));
    // e1 rule: minimal e1 with ell^e1 >= p N^4 / 2 (every reduced (c',d') gives R >= 0)
    let n2 = &n * &n;
    let e1_rule = min_exp(ell, &((p * &n2 * &n2 + 1u32) / 2u32));
    let mut e0 = e0_rule;
    let jq = Q::int([0, 0, 1, 0]);
    let kq = Q::int([0, 0, 0, 1]);
    let one_plus_i = Q::int([1, 1, 0, 0]);
    for _e0_round in 0..8 {
        let t = &n * pow_z(ell, e0);
        let disk = if ell == 2 { Z::from(4) * &t } else { t.clone() };
        let rmax = (&disk / p).sqrt().to_i64().unwrap();
        let mut pairs: Vec<(i64, i64)> = Vec::new();
        for u in -rmax..=rmax {
            for v in -rmax..=rmax {
                if p * Z::from(u * u + v * v) <= disk {
                    pairs.push((u, v));
                }
            }
        }
        pairs.shuffle(rng);
        for (u, v) in pairs {
            st.gamma_pairs_tried += 1;
            let Some((gamma, ok)) = gamma_from_pair(p, ell, &t, u, v, small) else { continue };
            if !ok {
                st.gamma_rejected_l2_parity += 1;
                continue;
            }
            st.gamma_found += 1;
            if qnrd(p, &gamma).n != t || !qnrd(p, &gamma).d.is_one() {
                st.internal_errors += 1;
                return Err("gamma norm wrong".into());
            }
            if to_order(&gamma.scale(&Z::one(), &ellz)).is_some() {
                // gamma in ell O0 (for ell=3 this happens exactly when u = v = 0 mod 3, since 3 is
                // inert in Z[i]); then every beta = gamma mu is in ell O0: reject gamma at once.
                st.gamma_rejected_nonprimitive += 1;
                continue;
            }
            if ell == 2 {
                // index-4 issue: Z<i,j> = Z[i] + 2 O0 mod 2, so any mu in Z<i,j> of even norm
                // is = 1+i mod 2 O0; beta = gamma mu is primitive iff gamma (1+i) not in 2 O0.
                let g1 = qmul(p, &gamma, &one_plus_i).scale(&Z::one(), &z(2));
                if to_order(&gamma).is_none() || to_order(&g1).is_some() {
                    st.gamma_rejected_l2_parity += 1;
                    continue;
                }
            }
            // (c)
            let u1 = to_order(&qmul(p, &gamma, &jq)).unwrap().map(|x| to_i128(&x, ni));
            let u2 = to_order(&qmul(p, &gamma, &kq)).unwrap().map(|x| to_i128(&x, ni));
            let dot = |f: &[i128; 4], u: &[i128; 4]| -> i128 {
                (0..4).map(|k| f[k] * u[k] % ni).sum::<i128>().rem_euclid(ni)
            };
            let a: Vec<(i128, i128)> = ann.iter().map(|f| (dot(f, &u1), dot(f, &u2))).collect();
            let (c, d) = match a.iter().find(|r| r.0 != 0 || r.1 != 0) {
                None => (1i128, 0i128),
                Some(&(r0, r1)) => (r1, (-r0).rem_euclid(ni)),
            };
            if a.iter().any(|&(r0, r1)| (r0 * c + r1 * d).rem_euclid(ni) != 0) {
                st.gamma_rejected_no_cd += 1;
                continue;
            }
            let ss = (c * c + d * d).rem_euclid(ni);
            if ss == 0 {
                st.gamma_rejected_cd_isotropic += 1;
                continue;
            }
            let (cz, dz) = (Z::from(c), Z::from(d));
            let mu0 = Q::new([z(0), z(0), cz.clone(), dz.clone()], z(1));
            if !idp.contains_q(&qmul(p, &gamma, &mu0)) {
                st.internal_errors += 1;
                return Err("step c: gamma mu0 not in I'".into());
            }
            // (d)
            let base = modp(&(p * Z::from(ss)), &n);
            let ell_qr = legendre(&ellz, &n);
            let mut e1 = e1_rule;
            let mut bumps_here = 0;
            let result = 'sa: loop {
                if bumps_here > 6 {
                    break 'sa None;
                }
                let target = pow_z(ell, e1);
                let r = modp(&(&target * inv_mod(&base, &n).unwrap()), &n);
                let Some(lam) = sqrt_mod(&r, &n) else {
                    if ell_qr == -1 {
                        e1 += 1;
                        st.e1_parity_bumps += 1;
                        bumps_here += 1;
                        continue 'sa;
                    } else {
                        st.gamma_rejected_residuosity += 1;
                        break 'sa None;
                    }
                };
                let num = &target - p * &lam * &lam * (&cz * &cz + &dz * &dz);
                let (qn, rn) = num.div_rem(&n);
                assert!(rn.is_zero());
                let tt = modp(&(qn * inv_mod(&(Z::from(2) * p * &lam), &n).unwrap()), &n);
                let mut frees: Vec<i128> = (0..ni).collect();
                frees.shuffle(rng);
                let c_inv = inv_mod(&cz, &n);
                for fr in frees {
                    let frz = Z::from(fr);
                    let (cc, dd) = match &c_inv {
                        Some(ci) => (modp(&((&tt - &dz * &frz) * ci), &n), frz),
                        None => {
                            let di = inv_mod(&dz, &n).unwrap();
                            (frz.clone(), modp(&((&tt - &cz * &frz) * di), &n))
                        }
                    };
                    st.sa_candidates += 1;
                    let cp = centered(&(&lam * &cz + &n * &cc), &n2);
                    let dp = centered(&(&lam * &dz + &n * &dd), &n2);
                    if ell == 2 && (&cp + &dp).is_even() {
                        st.sa_parity_skips += 1;
                        continue;
                    }
                    let rem = &target - p * (&cp * &cp + &dp * &dp);
                    if rem.is_negative() {
                        st.sa_negative += 1;
                        continue;
                    }
                    let (rr, rz) = rem.div_rem(&n2);
                    if !rz.is_zero() {
                        st.internal_errors += 1;
                        return Err("step d: remainder not divisible by N^2".into());
                    }
                    let Some((mut aa, mut bb)) = sum_two_squares(&rr, small) else {
                        st.sa_no_sos += 1;
                        continue;
                    };
                    if ell == 2 && (aa.is_odd() == cp.is_odd()) {
                        std::mem::swap(&mut aa, &mut bb);
                    }
                    let mu = Q::new([&n * &aa, &n * &bb, cp.clone(), dp.clone()], z(1));
                    let nm = qnrd(p, &mu);
                    if nm.n != target || !nm.d.is_one() {
                        st.internal_errors += 1;
                        return Err("step d: nrd(mu) != ell^e1".into());
                    }
                    let beta = qmul(p, &gamma, &mu);
                    if !idp.contains_q(&beta) {
                        st.internal_errors += 1;
                        return Err("step d: beta not in I'".into());
                    }
                    if to_order(&beta.scale(&Z::one(), &ellz)).is_some() {
                        st.nonprimitive_rejects += 1;
                        continue;
                    }
                    break 'sa Some((beta, e1));
                }
                e1 += 1;
                st.e1_bumps += 1;
                bumps_here += 1;
            };
            let Some((beta, e1f)) = result else {
                if bumps_here > 6 {
                    st.gamma_rejected_e1_cap += 1;
                }
                continue;
            };
            // (e)
            let x = qmul(p, &beta, &delta).scale(&Z::one(), &n);
            let xo = to_order(&x).ok_or("step e: x not in O0")?;
            return Ok(Output {
                x: xo,
                n_prime: n,
                e0_rule,
                e1_rule,
                e0,
                e1: e1f,
                stats: st,
            });
        }
        e0 += 1;
        st.e0_bumps += 1;
    }
    Err("gave up after 8 e0 rounds".into())
}

pub struct Controls {
    pub in_i: bool,
    pub norm_ok: bool,
    pub ideal_ok: bool,
    pub primitive: bool,
}

/// Exact controls on the output element x of I.
pub fn controls(p: &Z, id: &Ideal, nid: &Z, x: &[Z; 4], ell: u32, e: u32) -> Controls {
    let in_i = id.coeffs(x).is_some();
    let target = pow_z(ell, e);
    let nx = order_norm(p, x);
    let norm_ok = {
        let (q, r) = nx.div_rem(nid);
        r.is_zero() && q == target
    };
    let mut ideal_ok = false;
    let mut primitive = false;
    if in_i {
        if let Some(j) = equivalent(p, id, nid, &from_order(x)) {
            ideal_ok = j.index() == &target * &target
                && j.norm().as_ref() == Some(&target)
                && j.left_closed(p);
            primitive = !j.divisible_by(&Z::from(ell));
        }
    }
    Controls { in_i, norm_ok, ideal_ok, primitive }
}
