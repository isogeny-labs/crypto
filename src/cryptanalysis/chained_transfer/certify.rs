//! Independent solve certificates (`chained.solve.certificate/v1`) and their
//! verifier.
//!
//! A solver (Pollard rho or index calculus) claims a discrete logarithm `k`
//! for an instance `(G, T)` of prime order `r`. This module turns each claim
//! into a self-contained JSON record and re-checks it from the curve data
//! alone, with scalar multiplication written here (a fresh double-and-add, not
//! the solvers' `scalar_counted` or walk code). The in-process solvers are
//! therefore not trusted: a wrong claim is reported as wrong.
//!
//! No existing certificate schema in the repository fits an ECDLP solve claim
//! (`jacobian-certificate/v1` and `identity.certificate/v1` certify other
//! things), so this module defines its own.
//!
//! ## Schema `chained.solve.certificate/v1` (one JSON object per line)
//!
//! | field | meaning |
//! |---|---|
//! | `schema` | the constant `"chained.solve.certificate/v1"` |
//! | `arm` | which solver: `rho_e1`, `rho_e1prime`, `rho_jc` or `ic_jc` |
//! | `instance_type` | `direct` (generator and target picked on the curve) or `pullback` (the pulled-back pair `φ^*(P), φ^*([x]P)`) or `isogeny` |
//! | `p` | the prime field size |
//! | `group` | `{"kind":"ec","a2","a4","a6"}` for `y² = x³ + a2x² + a4x + a6`, or `{"kind":"jac","f":[f0..f5]}` for `y² = f(x)` (genus 2, quintic model) |
//! | `r` | the claimed prime order of `G` |
//! | `g`, `t` | generator and target. EC point: `null` (infinity) or `[x, y]`. Jacobian element: `{"u":[..],"v":[..]}` coefficients, lowest degree first, `u` monic |
//! | `claimed_k` | the solver's claimed log, or `null` when the solver returned none |
//! | `transfer` | for transferred instances, the data needed to re-evaluate the map (see below), else `null` |
//! | `seed`, `solver_ops` | informational (the solver's seed and counted group operations); not checked |
//!
//! `transfer` is `{"kind":"pullback","c":[c0..c3],"src_g","src_t"}` (the
//! bielliptic curve `y² = c0 + c1 x² + c2 x⁴ + c3 x⁶`, source points on its
//! quotient `E1`) or `{"kind":"isogeny","src":<ec group>,"kernel","src_g","src_t"}`
//! (the Vélu isogeny with the given rational kernel generator).
//!
//! ## What [`verify`] checks
//!
//! 1. `p` is prime, `r` is prime and the claim `claimed_k` exists and is `< r`;
//! 2. `G` and `T` are valid elements of the group (on the curve, or a reduced
//!    divisor with `u | f − v²`);
//! 3. `G ≠ 0`, `[r]G = 0` (so `G` has order exactly `r`), `[k]G = T`;
//! 4. for transferred instances: the source pair satisfies `src_g ≠ 0`,
//!    `[r]src_g = 0`, `[k]src_g = src_t`, and the map, re-evaluated from the
//!    certificate, sends `src_g ↦ G` and `src_t ↦ T`. With check 3 this gives
//!    `φ^*([k]P) = [k]φ^*(P)` on the instance, with `[k]P` computed by the
//!    independent code.
//!
//! ## Independence and its limits
//!
//! Elliptic-curve arithmetic here is a separate affine implementation over
//! `F_p` (rational points only). Genus-2 arithmetic reuses the library's
//! Cantor `add` and `double` for the group law (no second Cantor
//! implementation was written); only the scalar multiplication loop is new.
//! The transfer re-evaluation reuses the library's `Bielliptic::pull` and
//! `VeluIsogeny::eval`, so it checks the instance against the map, not the map
//! against the mathematics. The transfer (cover or isogeny construction and
//! `φ^*` evaluation) is a one-time cost and is never charged here or in a
//! per-instance total.

use serde::{Deserialize, Serialize};

use super::bielliptic::{Bielliptic, Quotient};
use super::ec::{Curve, Pt, VeluIsogeny};
use super::field::{Fp, F2};
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};

/// The schema identifier written into every certificate.
pub const SCHEMA: &str = "chained.solve.certificate/v1";

/// The group an instance lives in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum GroupData {
    #[serde(rename = "ec")]
    Ec { a2: u64, a4: u64, a6: u64 },
    #[serde(rename = "jac")]
    Jac { f: [u64; 6] },
}

/// A group element: an `F_p`-rational point or a Mumford divisor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Elem {
    Jac { u: Vec<u64>, v: Vec<u64> },
    Ec(Option<[u64; 2]>),
}

/// How the instance was transferred, with the data to re-evaluate the map.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Transfer {
    #[serde(rename = "pullback")]
    Pullback {
        c: [u64; 4],
        src_g: Elem,
        src_t: Elem,
    },
    #[serde(rename = "isogeny")]
    Isogeny {
        src: GroupData,
        kernel: Elem,
        src_g: Elem,
        src_t: Elem,
    },
}

/// One solve claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cert {
    pub schema: String,
    pub arm: String,
    pub instance_type: String,
    pub p: u64,
    pub group: GroupData,
    pub r: u64,
    pub g: Elem,
    pub t: Elem,
    pub claimed_k: Option<u64>,
    pub transfer: Option<Transfer>,
    pub seed: u64,
    pub solver_ops: u64,
}

/// The outcome of re-checking a certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    /// Every check passed.
    pub ok: bool,
    /// The solver returned no log (neither verified nor refuted).
    pub no_claim: bool,
    /// One line per failed check.
    pub failures: Vec<String>,
}

// ── Constructors ─────────────────────────────────────────────────────────────

pub fn ec_group(c: &Curve) -> GroupData {
    GroupData::Ec {
        a2: c.a2,
        a4: c.a4,
        a6: c.a6,
    }
}

pub fn jac_group(c: &Genus2Curve) -> GroupData {
    GroupData::Jac { f: c.f() }
}

/// An `F_p`-rational point as an [`Elem`]; an error if it has an `i` part.
pub fn pt_elem(p: &Pt) -> Result<Elem, String> {
    match p {
        None => Ok(Elem::Ec(None)),
        Some((x, y)) if x.b == 0 && y.b == 0 => Ok(Elem::Ec(Some([x.a, y.a]))),
        Some(_) => Err("point is not F_p-rational".into()),
    }
}

pub fn div_elem(d: &Divisor) -> Elem {
    Elem::Jac {
        u: d.u_coeffs(),
        v: d.v_coeffs(),
    }
}

// ── Independent elliptic-curve arithmetic over F_p (affine) ─────────────────

type EcPt = Option<(u64, u64)>;

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, p);
        }
        b = mulm(b, b, p);
        e >>= 1;
    }
    r
}

fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

fn ec_on_curve(p: u64, a2: u64, a4: u64, a6: u64, q: &EcPt) -> bool {
    match q {
        None => true,
        Some((x, y)) => {
            if *x >= p || *y >= p {
                return false;
            }
            let x2 = mulm(*x, *x, p);
            let rhs = (mulm(x2, *x, p) + mulm(a2, x2, p) + mulm(a4, *x, p) + a6) % p;
            mulm(*y, *y, p) == rhs
        }
    }
}

fn ec_add(p: u64, a2: u64, a4: u64, q1: &EcPt, q2: &EcPt) -> EcPt {
    let (x1, y1) = match q1 {
        None => return *q2,
        Some(v) => *v,
    };
    let (x2, y2) = match q2 {
        None => return *q1,
        Some(v) => *v,
    };
    let m = if x1 == x2 {
        if (y1 + y2) % p == 0 {
            return None;
        }
        // (3x² + 2·a2·x + a4) / (2y)
        let num = (3 * mulm(x1, x1, p) + 2 * mulm(a2, x1, p) + a4) % p;
        mulm(num, invm(2 * y1 % p, p), p)
    } else {
        mulm((y2 + p - y1) % p, invm((x2 + p - x1) % p, p), p)
    };
    let x3 = (mulm(m, m, p) + 2 * p - a2 + 2 * p - x1 - x2) % p;
    let y3 = (mulm(m, (x1 + p - x3) % p, p) + p - y1) % p;
    Some((x3, y3))
}

/// Fresh left-to-right double-and-add on the elliptic curve.
fn ec_scalar(p: u64, a2: u64, a4: u64, q: &EcPt, k: u64) -> EcPt {
    let mut acc: EcPt = None;
    for bit in (0..64).rev() {
        acc = ec_add(p, a2, a4, &acc, &acc);
        if (k >> bit) & 1 == 1 {
            acc = ec_add(p, a2, a4, &acc, q);
        }
    }
    acc
}

// ── Genus-2 scalar multiplication (library group law, new loop) ─────────────

/// Fresh left-to-right double-and-add on `J(C)`.
fn jac_scalar(c: &Genus2Curve, d: &Divisor, k: u64) -> Divisor {
    let mut acc = Divisor::IDENTITY;
    for bit in (0..64).rev() {
        acc = c.double(&acc);
        if (k >> bit) & 1 == 1 {
            acc = c.add(&acc, d);
        }
    }
    acc
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

// ── Verification ─────────────────────────────────────────────────────────────

fn ec_pt(e: &Elem) -> Result<EcPt, String> {
    match e {
        Elem::Ec(None) => Ok(None),
        Elem::Ec(Some([x, y])) => Ok(Some((*x, *y))),
        Elem::Jac { .. } => Err("expected an elliptic point".into()),
    }
}

fn jac_div(c: &Genus2Curve, e: &Elem) -> Result<Divisor, String> {
    match e {
        Elem::Jac { u, v } => c.divisor(u, v).map_err(|e| format!("invalid divisor: {e}")),
        Elem::Ec(_) => Err("expected a Mumford divisor".into()),
    }
}

fn to_f2(q: &EcPt) -> Pt {
    q.map(|(x, y)| (F2::base(x), F2::base(y)))
}

fn from_f2(q: &Pt) -> Option<EcPt> {
    match q {
        None => Some(None),
        Some((x, y)) if x.b == 0 && y.b == 0 => Some(Some((x.a, y.a))),
        Some(_) => None,
    }
}

/// Independent checks of an order-`r` pair on an elliptic curve; returns the
/// failures and, for later transfer checks, nothing else.
fn check_ec_pair(
    p: u64,
    (a2, a4, a6): (u64, u64, u64),
    g: &EcPt,
    t: &EcPt,
    r: u64,
    k: u64,
    label: &str,
    out: &mut Vec<String>,
) {
    if !ec_on_curve(p, a2, a4, a6, g) || !ec_on_curve(p, a2, a4, a6, t) {
        out.push(format!("{label}: point not on the curve"));
        return;
    }
    if g.is_none() {
        out.push(format!("{label}: G is the identity"));
    }
    if ec_scalar(p, a2, a4, g, r).is_some() {
        out.push(format!("{label}: [r]G != 0"));
    }
    if ec_scalar(p, a2, a4, g, k) != *t {
        out.push(format!("{label}: [k]G != T"));
    }
}

/// Re-check a certificate from its own data.
pub fn verify(cert: &Cert) -> Verdict {
    let mut f: Vec<String> = vec![];
    if cert.schema != SCHEMA {
        f.push(format!("schema is {:?}", cert.schema));
    }
    if !is_prime(cert.p) || cert.p < 5 {
        f.push("p is not an odd prime".into());
    }
    if !is_prime(cert.r) {
        f.push("r is not prime".into());
    }
    let Some(k) = cert.claimed_k else {
        return Verdict {
            ok: false,
            no_claim: true,
            failures: vec!["solver returned no log".into()],
        };
    };
    if k >= cert.r {
        f.push("claimed k is not below r".into());
    }
    if !f.is_empty() {
        return Verdict {
            ok: false,
            no_claim: false,
            failures: f,
        };
    }
    let p = cert.p;
    match &cert.group {
        GroupData::Ec { a2, a4, a6 } => match (ec_pt(&cert.g), ec_pt(&cert.t)) {
            (Ok(g), Ok(t)) => check_ec_pair(p, (*a2, *a4, *a6), &g, &t, cert.r, k, "ec", &mut f),
            (Err(e), _) | (_, Err(e)) => f.push(e),
        },
        GroupData::Jac { f: poly } => match Genus2Curve::new(p, *poly) {
            Err(e) => f.push(format!("invalid genus-2 curve: {e}")),
            Ok(c) => match (jac_div(&c, &cert.g), jac_div(&c, &cert.t)) {
                (Ok(g), Ok(t)) => {
                    if g.is_identity() {
                        f.push("jac: G is the identity".into());
                    }
                    if !jac_scalar(&c, &g, cert.r).is_identity() {
                        f.push("jac: [r]G != 0".into());
                    }
                    if jac_scalar(&c, &g, k) != t {
                        f.push("jac: [k]G != T".into());
                    }
                }
                (Err(e), _) | (_, Err(e)) => f.push(e),
            },
        },
    }
    if let Some(tr) = &cert.transfer {
        check_transfer(cert, tr, k, &mut f);
    }
    Verdict {
        ok: f.is_empty(),
        no_claim: false,
        failures: f,
    }
}

fn check_transfer(cert: &Cert, tr: &Transfer, k: u64, f: &mut Vec<String>) {
    let p = cert.p;
    match tr {
        Transfer::Pullback { c, src_g, src_t } => {
            let b = match Bielliptic::new(p, *c) {
                Ok(b) => b,
                Err(e) => return f.push(format!("transfer: bielliptic curve rejected: {e:?}")),
            };
            let e1 = b.curve(Quotient::E1);
            match &cert.group {
                GroupData::Jac { f: poly } if *poly == b.jac.f() => {}
                _ => return f.push("transfer: J(C) differs from the cover's Jacobian".into()),
            }
            let (Ok(sg), Ok(st)) = (ec_pt(src_g), ec_pt(src_t)) else {
                return f.push("transfer: source points are not elliptic".into());
            };
            let n = f.len();
            check_ec_pair(
                p,
                (e1.a2, e1.a4, e1.a6),
                &sg,
                &st,
                cert.r,
                k,
                "transfer src",
                f,
            );
            if f.len() != n {
                return;
            }
            let (Ok(g), Ok(t)) = (jac_div(&b.jac, &cert.g), jac_div(&b.jac, &cert.t)) else {
                return f.push("transfer: target elements invalid".into());
            };
            if b.pull(Quotient::E1, &to_f2(&sg)) != g {
                f.push("transfer: phi^*(src_g) != G".into());
            }
            if b.pull(Quotient::E1, &to_f2(&st)) != t {
                f.push("transfer: phi^*(src_t) != T".into());
            }
        }
        Transfer::Isogeny {
            src,
            kernel,
            src_g,
            src_t,
        } => {
            let GroupData::Ec { a2, a4, a6 } = src else {
                return f.push("transfer: isogeny source is not an elliptic curve".into());
            };
            let Some(kf) = Fp::new(p) else {
                return f.push("transfer: bad field".into());
            };
            let Some(e) = Curve::new(kf, *a2, *a4, *a6) else {
                return f.push("transfer: singular source curve".into());
            };
            let (Ok(sg), Ok(st), Ok(kern)) = (ec_pt(src_g), ec_pt(src_t), ec_pt(kernel)) else {
                return f.push("transfer: source data are not elliptic".into());
            };
            let n = f.len();
            check_ec_pair(p, (*a2, *a4, *a6), &sg, &st, cert.r, k, "transfer src", f);
            if f.len() != n {
                return;
            }
            let Some(iso) = VeluIsogeny::from_generator(&e, &to_f2(&kern)) else {
                return f.push("transfer: kernel does not give a Velu isogeny".into());
            };
            if ec_group(&iso.codomain) != cert.group {
                f.push("transfer: codomain differs from the certified curve".into());
            }
            let (Ok(g), Ok(t)) = (ec_pt(&cert.g), ec_pt(&cert.t)) else {
                return f.push("transfer: target elements invalid".into());
            };
            if from_f2(&iso.eval(&to_f2(&sg))) != Some(g) {
                f.push("transfer: isogeny(src_g) != G".into());
            }
            if from_f2(&iso.eval(&to_f2(&st))) != Some(t) {
                f.push("transfer: isogeny(src_t) != T".into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::chained_transfer::measure::jac_instance;

    fn small_ec_cert() -> Cert {
        let p = 211u64;
        let kf = Fp::new(p).unwrap();
        let e = Curve::new(kf, 3, 5, 7).unwrap();
        let order = e.order();
        // largest prime factor r, cofactor-clear a point to order r
        let mut r = 1;
        let mut n = order;
        let mut d = 2;
        while d * d <= n {
            while n % d == 0 {
                r = d;
                n /= d;
            }
            d += 1;
        }
        if n > 1 {
            r = r.max(n);
        }
        let g = (1..200u64)
            .map(|s| e.mul(&e.point_from_seed(s), (order / r) as u128))
            .find(|g| g.is_some())
            .unwrap();
        let x = 5 % r;
        let t = e.mul(&g, x as u128);
        Cert {
            schema: SCHEMA.into(),
            arm: "rho_e1".into(),
            instance_type: "direct".into(),
            p,
            group: ec_group(&e),
            r,
            g: pt_elem(&g).unwrap(),
            t: pt_elem(&t).unwrap(),
            claimed_k: Some(x),
            transfer: None,
            seed: 0,
            solver_ops: 0,
        }
    }

    #[test]
    fn good_ec_certificate_passes_and_tampering_fails() {
        let c = small_ec_cert();
        let v = verify(&c);
        assert!(v.ok, "{:?}", v.failures);
        let mut bad_k = c.clone();
        bad_k.claimed_k = Some((c.claimed_k.unwrap() + 1) % c.r);
        let v = verify(&bad_k);
        assert!(!v.ok && !v.no_claim);
        let mut bad_t = c.clone();
        bad_t.t = c.g.clone();
        assert!(!verify(&bad_t).ok);
        let mut none = c.clone();
        none.claimed_k = None;
        let v = verify(&none);
        assert!(!v.ok && v.no_claim);
        // JSON round trip
        let s = serde_json::to_string(&c).unwrap();
        let back: Cert = serde_json::from_str(&s).unwrap();
        assert_eq!(back, c);
        assert!(verify(&back).ok);
    }

    #[test]
    fn good_jacobian_certificate_passes_and_tampering_fails() {
        for p in [101u64, 251, 1009] {
            let c = Genus2Curve::x5_plus_b(p, 3).unwrap();
            let order = c.jacobian_order().unwrap();
            let mut r = 0;
            for cand in 2..order {
                if is_prime(cand) && order % cand == 0 && (order / cand) % cand != 0 && cand > 20 {
                    r = cand;
                }
            }
            if r == 0 {
                continue;
            }
            let x = 7 % r;
            let Some((g, t)) = jac_instance(&c, order, r, x, p) else {
                continue;
            };
            let cert = Cert {
                schema: SCHEMA.into(),
                arm: "ic_jc".into(),
                instance_type: "surrogate".into(),
                p,
                group: jac_group(&c),
                r,
                g: div_elem(&g),
                t: div_elem(&t),
                claimed_k: Some(x),
                transfer: None,
                seed: 0,
                solver_ops: 0,
            };
            let v = verify(&cert);
            assert!(v.ok, "{:?}", v.failures);
            let mut bad = cert.clone();
            bad.claimed_k = Some((x + 1) % r);
            assert!(!verify(&bad).ok);
            let mut bad_t = cert.clone();
            bad_t.t = div_elem(&c.double(&t));
            assert!(!verify(&bad_t).ok);
            let s = serde_json::to_string(&cert).unwrap();
            let back: Cert = serde_json::from_str(&s).unwrap();
            assert!(verify(&back).ok);
            return;
        }
        panic!("no usable genus-2 instance");
    }
}
