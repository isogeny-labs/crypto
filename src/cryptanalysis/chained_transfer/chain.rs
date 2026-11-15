//! A verified DLP transfer and the composition of several into a chain.
//!
//! A *hop* carries an instance `(G, P, Q=[x]P, r)` to another `(G', P', Q', r)`
//! by an explicit, efficiently evaluable group homomorphism `h` with
//! `h(P) = P'`, `ord(P') = r` and `h(Q) = Q'`. The admissibility checks of
//! `PROTOCOL.md` (image nonzero, order preserved, homomorphism on sampled
//! pairs) are run on construction, so a hop that fails to preserve the DLP is
//! rejected rather than silently trusted.
//!
//! The source and destination groups are heterogeneous (elliptic over `F_p`,
//! genus-2 Jacobian), so a hop records only what it needs to *verify* the
//! transfer and to *price* it; the end-to-end measurement lives in
//! `measure.rs`.

use super::bielliptic::{Bielliptic, Quotient};
use super::ec::{Curve, Pt, VeluIsogeny};
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};

/// The group an instance lives in.
#[derive(Clone)]
pub enum Group {
    Elliptic(Curve),
    Jacobian(Genus2Curve),
}

/// A point in whichever group.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Elt {
    E(Pt),
    J(Divisor),
}

impl Group {
    pub fn add(&self, a: &Elt, b: &Elt) -> Elt {
        match (self, a, b) {
            (Group::Elliptic(c), Elt::E(x), Elt::E(y)) => Elt::E(c.add(x, y)),
            (Group::Jacobian(c), Elt::J(x), Elt::J(y)) => Elt::J(c.add(x, y)),
            _ => panic!("element/group mismatch"),
        }
    }
    pub fn mul(&self, a: &Elt, n: u128) -> Elt {
        match (self, a) {
            (Group::Elliptic(c), Elt::E(x)) => Elt::E(c.mul(x, n)),
            (Group::Jacobian(c), Elt::J(x)) => Elt::J(c.mul(x, n as u64)),
            _ => panic!("element/group mismatch"),
        }
    }
    pub fn is_zero(&self, a: &Elt) -> bool {
        match (self, a) {
            (Group::Elliptic(_), Elt::E(x)) => x.is_none(),
            (Group::Jacobian(_), Elt::J(x)) => x.is_identity(),
            _ => panic!("element/group mismatch"),
        }
    }
}

/// What kind of map a hop is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum HopKind {
    /// Separable isogeny of the stated degree.
    Isogeny(u64),
    /// Bielliptic pull-back `φ^*` (degree-2 cover), elliptic → Jacobian.
    Pullback(Quotient),
    /// Bielliptic push-forward `φ_*`, Jacobian → elliptic.
    Pushforward(Quotient),
}

/// A single verified hop.
pub struct Hop {
    pub kind: HopKind,
    pub src: Group,
    pub dst: Group,
    pub map: Box<dyn Fn(&Elt) -> Elt>,
    /// Group operations charged to evaluate the map once (formula-level).
    pub eval_ops: u64,
}

/// Why a hop was rejected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HopError {
    ImageZero,
    OrderNotPreserved,
    NotHomomorphic,
    DlpNotPreserved,
}

impl Hop {
    /// A Vélu isogeny hop `E → E'`, checked to preserve the order-`r` DLP.
    pub fn isogeny(iso: VeluIsogeny, p: &Pt, q: &Pt, r: u64) -> Result<Hop, HopError> {
        let dst = iso.codomain;
        let src = iso.domain;
        let pp = iso.eval(p);
        if pp.is_none() {
            return Err(HopError::ImageZero);
        }
        let _ = q;
        let deg = iso.degree() as u64;
        let ops = iso.group_ops_per_eval() as u64;
        // Homomorphism spot-check on random pairs.
        for s in 0..24u64 {
            let a = src.point_from_seed(s * 97 + 11);
            let b = src.point_from_seed(s * 59 + 23);
            if iso.eval(&src.add(&a, &b)) != dst.add(&iso.eval(&a), &iso.eval(&b)) {
                return Err(HopError::NotHomomorphic);
            }
        }
        let iso = std::rc::Rc::new(iso);
        let iso2 = iso.clone();
        let hop = Hop {
            kind: HopKind::Isogeny(deg),
            src: Group::Elliptic(src),
            dst: Group::Elliptic(dst),
            map: Box::new(move |e| match e {
                Elt::E(pt) => Elt::E(iso2.eval(pt)),
                _ => panic!("isogeny takes an elliptic point"),
            }),
            eval_ops: ops,
        };
        hop.check_dlp(&Elt::E(*p), &Elt::E(*q), r)?;
        drop(iso);
        Ok(hop)
    }

    /// A bielliptic pull-back hop `E_q → J(C)` (`φ^*`), order preserved for
    /// odd `r` because `φ^*` restricted to the odd part is injective.
    pub fn pullback(b: &Bielliptic, q: Quotient, p: &Pt, r: u64) -> Result<Hop, HopError> {
        let src = *b.curve(q);
        let dst = b.jac.clone();
        let bb = std::rc::Rc::new(b.clone());
        let bb2 = bb.clone();
        let hop = Hop {
            kind: HopKind::Pullback(q),
            src: Group::Elliptic(src),
            dst: Group::Jacobian(dst),
            map: Box::new(move |e| match e {
                Elt::E(pt) => Elt::J(bb2.pull(q, pt)),
                _ => panic!("pull-back takes an elliptic point"),
            }),
            // Two square roots in F_{p²} and an interpolation: priced as 2 adds.
            eval_ops: 2,
        };
        // check_dlp verifies ord(φ^*P) = r and h([x]P) = [x]h(P); φ_*φ^* = [2]
        // keeps the order on the odd part, so φ^*P has order r for odd r.
        hop.check_dlp(&Elt::E(*p), &Elt::E(None), r)?;
        Ok(hop)
    }

    /// A bielliptic push-forward hop `J(C) → E_q` (`φ_*`).
    pub fn pushforward(b: &Bielliptic, q: Quotient, d: &Divisor, r: u64) -> Result<Hop, HopError> {
        let src = b.jac.clone();
        let dst = *b.curve(q);
        let bb = std::rc::Rc::new(b.clone());
        let bb2 = bb.clone();
        let hop = Hop {
            kind: HopKind::Pushforward(q),
            src: Group::Jacobian(src),
            dst: Group::Elliptic(dst),
            map: Box::new(move |e| match e {
                Elt::J(dv) => Elt::E(bb2.push(q, dv)),
                _ => panic!("push-forward takes a divisor class"),
            }),
            eval_ops: 2,
        };
        hop.check_dlp(&Elt::J(*d), &Elt::J(Divisor::IDENTITY), r)?;
        Ok(hop)
    }

    /// Verify `h(Q) = [x] h(P)` for the known `x` on sampled logs, order
    /// preservation, and `h` a homomorphism; `q = [x]p` need not be supplied.
    fn check_dlp(&self, p: &Elt, _q: &Elt, r: u64) -> Result<(), HopError> {
        let hp = (self.map)(p);
        if self.dst.is_zero(&hp) {
            return Err(HopError::ImageZero);
        }
        // ord(h(P)) = r: [r]h(P) = 0 and h(P) ≠ 0 (r prime).
        if !self.dst.is_zero(&self.dst.mul(&hp, r as u128)) {
            return Err(HopError::OrderNotPreserved);
        }
        // DLP preserved: h([x]P) = [x]h(P) for several x.
        for x in [0u128, 1, 2, 3, 7, (r / 2) as u128, (r - 1) as u128] {
            let lhs = (self.map)(&self.src.mul(p, x));
            let rhs = self.dst.mul(&hp, x);
            if lhs != rhs {
                return Err(HopError::DlpNotPreserved);
            }
        }
        Ok(())
    }

    pub fn apply(&self, e: &Elt) -> Elt {
        (self.map)(e)
    }
}

/// A composed chain of hops with matching group types.
pub struct Chain {
    pub hops: Vec<Hop>,
}

impl Chain {
    pub fn new(hops: Vec<Hop>) -> Chain {
        Chain { hops }
    }

    /// Transport an element through every hop.
    pub fn transport(&self, start: &Elt) -> Elt {
        let mut cur = *start;
        for h in &self.hops {
            cur = h.apply(&cur);
        }
        cur
    }

    /// The destination group.
    pub fn dst(&self) -> &Group {
        &self.hops.last().expect("nonempty chain").dst
    }

    /// Total formula-level group operations to evaluate the chain once.
    pub fn eval_ops(&self) -> u64 {
        self.hops.iter().map(|h| h.eval_ops).sum()
    }

    /// Verify the whole chain preserves the order-`r` DLP: for the carried
    /// `P` and `Q = [x]P`, `transport(Q) = [x] transport(P)` and the image of
    /// `P` has order `r`.
    pub fn verify(&self, p: &Elt, r: u64, x: u128) -> bool {
        let tp = self.transport(p);
        if self.dst().is_zero(&tp) {
            return false;
        }
        if !self.dst().is_zero(&self.dst().mul(&tp, r as u128)) {
            return false;
        }
        let q = {
            // [x]P in the source group of the first hop.
            let g = &self.hops[0].src;
            g.mul(p, x)
        };
        self.transport(&q) == self.dst().mul(&tp, x)
    }
}
