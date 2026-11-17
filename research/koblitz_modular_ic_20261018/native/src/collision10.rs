//! Sparse Phi11 collision resultant in native F2[X], with exact rational Euclidean arithmetic.
use crate::{
    round02,
    round04::{self, Field},
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::json;
use std::{
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};
static OPS: AtomicUsize = AtomicUsize::new(0);
fn tick() {
    assert!(
        OPS.fetch_add(1, Ordering::Relaxed) < 20_000_000,
        "frozen polynomial operation budget"
    );
}
fn mul(a: &BigUint, b: &BigUint) -> BigUint {
    let mut z = BigUint::zero();
    for i in 0..b.bits() {
        tick();
        if b.bit(i) {
            z ^= a << i
        }
    }
    assert!(z.bits() <= 4097, "frozen degree budget");
    z
}
fn div(a: &BigUint, b: &BigUint) -> (BigUint, BigUint) {
    assert!(!b.is_zero());
    let mut r = a.clone();
    let mut q = BigUint::zero();
    while !r.is_zero() && r.bits() >= b.bits() {
        tick();
        let i = r.bits() - b.bits();
        q ^= BigUint::one() << i;
        r ^= b << i;
    }
    (q, r)
}
fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let r = div(&a, &b).1;
        a = b;
        b = r;
    }
    a
}
#[derive(Clone)]
struct Rat {
    n: BigUint,
    d: BigUint,
}
impl Rat {
    fn new(n: BigUint, d: BigUint) -> Self {
        assert!(!d.is_zero());
        let g = gcd(n.clone(), d.clone());
        Self {
            n: div(&n, &g).0,
            d: div(&d, &g).0,
        }
    }
    fn zero() -> Self {
        Self {
            n: BigUint::zero(),
            d: BigUint::one(),
        }
    }
    fn one() -> Self {
        Self {
            n: BigUint::one(),
            d: BigUint::one(),
        }
    }
    fn xpow(n: usize) -> Self {
        Self {
            n: BigUint::one() << n,
            d: BigUint::one(),
        }
    }
    fn add(&self, b: &Self) -> Self {
        Self::new(mul(&self.n, &b.d) ^ mul(&b.n, &self.d), mul(&self.d, &b.d))
    }
    fn mul(&self, b: &Self) -> Self {
        Self::new(mul(&self.n, &b.n), mul(&self.d, &b.d))
    }
    fn inv(&self) -> Self {
        assert!(!self.n.is_zero());
        Self {
            n: self.d.clone(),
            d: self.n.clone(),
        }
    }
    fn pow(&self, n: usize) -> Self {
        (0..n).fold(Self::one(), |a, _| a.mul(self))
    }
}
fn trim(mut a: Vec<Rat>) -> Vec<Rat> {
    while a.last().is_some_and(|a| a.n.is_zero()) {
        a.pop();
    }
    a
}
fn rem(mut a: Vec<Rat>, b: &[Rat]) -> Vec<Rat> {
    a = trim(a);
    let inv = b.last().unwrap().inv();
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let t = a.last().unwrap().mul(&inv);
        for i in 0..b.len() {
            a[k + i] = a[k + i].add(&t.mul(&b[i]));
        }
        a = trim(a);
    }
    a
}
fn resultant(a: Vec<Rat>, b: Vec<Rat>) -> Rat {
    if b.is_empty() {
        return Rat::zero();
    }
    if b.len() == 1 {
        return b[0].pow(a.len() - 1);
    }
    let m = a.len() - 1;
    let r = rem(a, &b);
    if r.is_empty() {
        return Rat::zero();
    }
    let d = r.len() - 1;
    b.last().unwrap().pow(m - d).mul(&resultant(b, r))
}
fn ev(f: Field, p: &BigUint, x: u32) -> u32 {
    (0..p.bits())
        .rev()
        .fold(0, |a, i| f.mul(a, x) ^ u32::from(p.bit(i)))
}
pub fn run(dir: &Path) {
    let input = round04::read(&dir.join("m1-modular-base.json"));
    let terms: Vec<_> = input["mod2_phi11_terms"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v[0].as_u64().unwrap() as usize,
                v[1].as_u64().unwrap() as usize,
            )
        })
        .collect();
    let mut p = vec![Rat::zero(); 13];
    for (x, y) in &terms {
        p[*y] = p[*y].add(&Rat::xpow(*x));
    }
    let g = vec![
        Rat::xpow(4),
        Rat::zero(),
        Rat::zero(),
        Rat::xpow(3),
        Rat::one().add(&Rat::xpow(4)),
    ];
    let res = resultant(p, g);
    assert_eq!(res.d, BigUint::one());
    assert!(!res.n.is_zero());
    let mut remaining = res.n.clone();
    let mut factors = vec![];
    // Trial division through degree16 also certifies irreducibility: smaller factors already removed.
    for d in 1..=16 {
        for low in 0..(1u64 << d) {
            let candidate: BigUint = (BigUint::one() << d) | BigUint::from(low);
            if d > 1 && !candidate.bit(0) {
                continue;
            }
            let mut n = 0;
            loop {
                let (q, r) = div(&remaining, &candidate);
                if !r.is_zero() {
                    break;
                }
                remaining = q;
                n += 1;
            }
            if n > 0 {
                factors.push(json!({"polynomial_bits":candidate.to_str_radix(16),"degree":d,"multiplicity":n}));
            }
            if remaining == BigUint::one() {
                break;
            }
        }
        if remaining == BigUint::one() {
            break;
        }
    }
    let mut checks = 0;
    for m in 1..=10 {
        let f = Field::find(m);
        for x in 0..1u32 << m {
            let mut a = vec![0; 13];
            for (i, j) in &terms {
                a[*j] ^= f.pow(x, *i as u64)
            }
            let b = vec![f.pow(x, 4), 0, 0, f.pow(x, 3), 1 ^ f.pow(x, 4)];
            let common = round04::gcd(f, &round04::trim(a), &round04::trim(b)).len() > 1;
            assert_eq!(ev(f, &res.n, x) == 0, common);
            checks += 1;
        }
    }
    round02::save(
        dir,
        "m14-Phi11-collision-resultant.json",
        &json!({"schema":"koblitz.round10/m14-v1","Phi11_terms":terms,"derivative_factorization":"dPhi/dY=X^3*Y^2*((1+X^4)Y^4+X^3Y^3+X^4)^2","resultant_bits":res.n.to_str_radix(16),"resultant_degree":res.n.bits()-1,"factors":factors,"remaining_bits":remaining.to_str_radix(16),"fully_factored_through_degree16":remaining==BigUint::one(),"small_field_gcd_controls":checks,"polynomial_operations":OPS.load(Ordering::Relaxed),"scope":"necessary source-j collision support for equal eleven-neighbor j values;exclude j=0 for ordinary interpretation;not all x-equality components and not a transported cyclic49/full-level divisor"}),
    );
    println!(
        "M14:degree{} collision resultant;{checks} finite-field gcd controls;remaining {}",
        res.n.bits() - 1,
        remaining.to_str_radix(16)
    );
}
