//! Exhaustive census over GF(2^n): every root triple of S_3(x1, x2, X) = 0, classified
//! by which abscissae lie on E (vs its quadratic twist) and whether a slope lambda in
//! GF(2^n) exists (lambda^2 + lambda = x1 + x2 + x3 + a solvable iff its trace is 0).

use crate::curve::Curve;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Census {
    /// (#abscissae on E among the three, lambda exists) -> count of root triples
    pub cells: BTreeMap<(u8, bool), u64>,
    pub triples: u64,
    pub lifted_checked: u64,
    pub lifted_ok: u64,
}

pub fn run(c: &Curve, verify_every: u64) -> Census {
    let f = &c.f;
    let mut out = Census::default();
    let top = 1u64 << f.n;
    for x1 in 1..top {
        for x2 in (x1 + 1)..top {
            let (s1, s2) = (f.sq(x1), f.sq(x2));
            let a = s1 ^ s2; // nonzero since x1 != x2
            let b = f.mul(x1, x2);
            let cc = f.mul(s1, s2) ^ c.b;
            // a X^2 + b X + cc = 0  ->  X = (b/a) z,  z^2 + z = a cc / b^2
            let rhs = f.mul(f.mul(a, cc), f.inv(f.sq(b)));
            if f.trace(rhs) != 0 {
                continue;
            }
            let z = f.half_trace(rhs);
            let scale = f.mul(b, f.inv(a));
            for x3 in [f.mul(scale, z), f.mul(scale, z ^ 1)] {
                if x3 == 0 {
                    continue;
                }
                let on = [x1, x2, x3].iter().filter(|&&x| c.x_on_curve(x)).count() as u8;
                let lam = f.trace(x1 ^ x2 ^ x3 ^ c.a) == 0;
                *out.cells.entry((on, lam)).or_default() += 1;
                out.triples += 1;
                if on == 3 && out.triples % verify_every == 0 {
                    out.lifted_checked += 1;
                    let (p1, p2) = (c.lift(x1).unwrap(), c.lift(x2).unwrap());
                    let hit = [c.add(p1, p2), c.add(p1, c.neg(p2))]
                        .iter()
                        .any(|s| matches!(s, crate::curve::Pt::A(x, _) if *x == x3));
                    out.lifted_ok += hit as u64;
                }
            }
        }
    }
    out
}
