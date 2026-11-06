//! Dependency-free exact integer S4 derivation; also imported by the certificate probe.
use std::collections::BTreeMap;

pub(crate) type Exponent = [u8; 5];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ZPoly(pub(crate) BTreeMap<Exponent, i128>);

impl ZPoly {
    pub(crate) fn constant(c: i128) -> Self {
        if c == 0 {
            Self::default()
        } else {
            Self(BTreeMap::from([([0; 5], c)]))
        }
    }
    pub(crate) fn var(index: usize) -> Self {
        let mut e = [0; 5];
        e[index] = 1;
        Self(BTreeMap::from([(e, 1)]))
    }
    pub(crate) fn add_term(&mut self, e: Exponent, c: i128) {
        let value = self.0.get(&e).copied().unwrap_or(0).checked_add(c).unwrap();
        if value == 0 {
            self.0.remove(&e);
        } else {
            self.0.insert(e, value);
        }
    }
    pub(crate) fn add(&self, other: &Self) -> Self {
        let mut out = self.clone();
        for (&e, &c) in &other.0 {
            out.add_term(e, c);
        }
        out
    }
    pub(crate) fn scale(&self, c: i128) -> Self {
        let mut out = Self::default();
        for (&e, &v) in &self.0 {
            out.add_term(e, v.checked_mul(c).unwrap());
        }
        out
    }
    pub(crate) fn sub(&self, other: &Self) -> Self {
        self.add(&other.scale(-1))
    }
    pub(crate) fn mul(&self, other: &Self) -> Self {
        let mut out = Self::default();
        for (&a, &c) in &self.0 {
            for (&b, &d) in &other.0 {
                let e = std::array::from_fn(|i| a[i].checked_add(b[i]).unwrap());
                out.add_term(e, c.checked_mul(d).unwrap());
            }
        }
        out
    }
    pub(crate) fn pow(&self, n: u8) -> Self {
        (0..n).fold(Self::constant(1), |out, _| out.mul(self))
    }
    pub(crate) fn substitute(&self, values: &[Self; 5]) -> Self {
        self.0.iter().fold(Self::default(), |acc, (&exp, &c)| {
            let term = (0..5).fold(Self::constant(c), |term, i| {
                term.mul(&values[i].pow(exp[i]))
            });
            acc.add(&term)
        })
    }
}

fn s3_coeff(x: &ZPoly, y: &ZPoly, b: &ZPoly) -> [ZPoly; 3] {
    let sum = x.add(y);
    let product = x.mul(y);
    [
        product.pow(2).sub(&b.mul(&sum).scale(4)),
        sum.mul(&product).add(&b.scale(2)).scale(-2),
        x.sub(y).pow(2),
    ]
}

fn resultant(a: &[ZPoly; 3], b: &[ZPoly; 3]) -> ZPoly {
    let u = a[2].mul(&b[0]).sub(&a[0].mul(&b[2]));
    let v = a[2].mul(&b[1]).sub(&a[1].mul(&b[2]));
    let w = a[1].mul(&b[0]).sub(&a[0].mul(&b[1]));
    u.pow(2).sub(&v.mul(&w))
}

pub(crate) fn elementary() -> [ZPoly; 5] {
    let [x, y, z, target, b] = std::array::from_fn(ZPoly::var);
    [
        x.add(&y).add(&z),
        x.mul(&y).add(&x.mul(&z)).add(&y.mul(&z)),
        x.mul(&y).mul(&z),
        target,
        b,
    ]
}

pub(crate) fn derive() -> (ZPoly, ZPoly) {
    let [x, y, z, target, b] = std::array::from_fn(ZPoly::var);
    let original = resultant(&s3_coeff(&x, &y, &b), &s3_coeff(&z, &target, &b));
    let es = elementary();
    let mut remainder = original.clone();
    let mut symmetric = ZPoly::default();
    while let Some((&lead, &coefficient)) = remainder.0.last_key_value() {
        assert!(lead[0] >= lead[1] && lead[1] >= lead[2]);
        let powers = [
            lead[0] - lead[1],
            lead[1] - lead[2],
            lead[2],
            lead[3],
            lead[4],
        ];
        symmetric.add_term(powers, coefficient);
        let term = (0..5).fold(ZPoly::constant(coefficient), |acc, i| {
            acc.mul(&es[i].pow(powers[i]))
        });
        remainder = remainder.sub(&term);
    }
    assert_eq!(symmetric.substitute(&es), original);
    (original, symmetric)
}

pub(crate) fn full_elementary() -> [ZPoly; 5] {
    let x: [ZPoly; 4] = std::array::from_fn(ZPoly::var);
    let mut es: [ZPoly; 5] = std::array::from_fn(|_| ZPoly::default());
    for mask in 1u32..16 {
        let degree = mask.count_ones() as usize;
        let term = (0..4)
            .filter(|i| mask & (1 << i) != 0)
            .fold(ZPoly::constant(1), |a, i| a.mul(&x[i]));
        es[degree - 1] = es[degree - 1].add(&term);
    }
    es[4] = ZPoly::var(4);
    es
}

pub(crate) fn derive_full(original: &ZPoly) -> ZPoly {
    let es = full_elementary();
    let mut remainder = original.clone();
    let mut symmetric = ZPoly::default();
    while let Some((&lead, &coefficient)) = remainder.0.last_key_value() {
        assert!(lead[0] >= lead[1] && lead[1] >= lead[2] && lead[2] >= lead[3]);
        let powers = [
            lead[0] - lead[1],
            lead[1] - lead[2],
            lead[2] - lead[3],
            lead[3],
            lead[4],
        ];
        symmetric.add_term(powers, coefficient);
        let term = (0..5).fold(ZPoly::constant(coefficient), |a, i| {
            a.mul(&es[i].pow(powers[i]))
        });
        remainder = remainder.sub(&term);
    }
    assert_eq!(symmetric.substitute(&es), *original);
    symmetric
}

pub(crate) fn lean_source(h: &ZPoly) -> String {
    let mut out = String::from(
        r#"/- Exact j=0 S4 resultant identity, generated from the native integer coefficients. -/
import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 1000000
namespace TwistS4Identity
variable {R : Type} [Lean.Grind.CommRing R]
def c0 (x y B : R) : R := (x*y)^2 - 4*B*(x+y)
def c1 (x y B : R) : R := -2*((x+y)*(x*y)+2*B)
def c2 (x y : R) : R := (x-y)^2
def result (x y z X B : R) : R :=
  (c2 x y*c0 z X B-c0 x y B*c2 z X)^2 -
  (c2 x y*c1 z X B-c1 x y B*c2 z X)*
  (c1 x y B*c0 z X B-c0 x y B*c1 z X B)
def H (e1 e2 e3 X B : R) : R :=
"#,
    );
    for (i, (e, c)) in h.0.iter().enumerate() {
        out.push_str(if i == 0 { "  " } else { "  + " });
        out.push_str(&format!("({c} : R)"));
        for (name, power) in ["e1", "e2", "e3", "X", "B"].iter().zip(e) {
            if *power > 0 {
                out.push_str(&format!(" * {name}^{power}"));
            }
        }
        out.push('\n');
    }
    out.push_str(
        r#"theorem symmetrised_resultant (x y z X B : R) :
  result x y z X B = H (x+y+z) (x*y+x*z+y*z) (x*y*z) X B := by
  unfold result c0 c1 c2 H
  grind
#print axioms symmetrised_resultant
"#,
    );
    let (original, _) = derive();
    let full = derive_full(&original);
    out.push_str("def H4 (s1 s2 s3 s4 B : R) : R :=\n");
    for (i, (e, c)) in full.0.iter().enumerate() {
        out.push_str(if i == 0 { "  " } else { "  + " });
        out.push_str(&format!("({c} : R)"));
        for (name, power) in ["s1", "s2", "s3", "s4", "B"].iter().zip(e) {
            if *power > 0 {
                out.push_str(&format!(" * {name}^{power}"));
            }
        }
        out.push('\n');
    }
    out.push_str(
        r#"theorem fully_symmetrised_resultant (x y z X B : R) :
  result x y z X B = H4 (x+y+z+X) (x*y+x*z+x*X+y*z+y*X+z*X)
    (x*y*z+x*y*X+x*z*X+y*z*X) (x*y*z*X) B := by
  unfold result c0 c1 c2 H4
  grind
#print axioms fully_symmetrised_resultant
end TwistS4Identity
"#,
    );
    out
}

fn main() {
    let (original, symmetric) = derive();
    let full = derive_full(&original);
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--lean" {
        std::fs::write(&args[2], lean_source(&symmetric)).unwrap();
    } else {
        assert_eq!(
            args.len(),
            1,
            "usage: quartic_twist_s4_integer [--lean FILE]"
        );
    }
    assert!(symmetric
        .0
        .keys()
        .all(|e| e[0] + 2 * e[1] + 3 * e[2] + e[3] + 3 * e[4] == 12));
    let corrupt = symmetric.add(&ZPoly::constant(1)).substitute(&elementary());
    assert_ne!(corrupt, original);
    print!("{{\"schema\":\"quartic-twist-s4-integer/v1\",\"status\":\"pass\",\"variables\":[\"e1\",\"e2\",\"e3\",\"X\",\"B\"],\"expanded_resultant_terms\":{},\"symmetric_terms\":{},\"exact_integer_remainder_terms\":0,\"weighted_degree\":12,\"weights\":[1,2,3,1,3],\"mutation_rejected\":true,\"terms\":[", original.0.len(), symmetric.0.len());
    for (i, (e, c)) in symmetric.0.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{{\"exponents\":{e:?},\"coefficient\":{c}}}");
    }
    print!(
        "],\"full_symmetric_terms\":{},\"full_terms\":[",
        full.0.len()
    );
    for (i, (e, c)) in full.0.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{{\"exponents\":{e:?},\"coefficient\":{c}}}");
    }
    println!("]}}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_reconstruction_and_mutation() {
        let (original, symmetric) = derive();
        assert_eq!(symmetric.substitute(&elementary()), original);
        assert_ne!(
            symmetric.add(&ZPoly::constant(1)).substitute(&elementary()),
            original
        );
    }
}
