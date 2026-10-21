//! Counted genus-3 source rho for Tian's F257 mapped DLP fixture.
use five_leads_study::three_leads::Field;
use serde_json::json;
use std::cell::Cell;
use std::collections::HashMap;

type Poly = Vec<u64>;
fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn padd(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    let mut r = vec![0; a.len().max(b.len())];
    for (i, v) in r.iter_mut().enumerate() {
        *v = f.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(r)
}
fn pneg(f: &Field, a: &[u64]) -> Poly {
    a.iter().map(|&x| f.neg(x)).collect()
}
fn psub(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    padd(f, a, &pneg(f, b))
}
fn pscale(f: &Field, a: &[u64], s: u64) -> Poly {
    trim(a.iter().map(|&x| f.mul(x, s)).collect())
}
fn pmul(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = f.add(r[i + j], f.mul(x, y));
        }
    }
    trim(r)
}
fn pdiv(f: &Field, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut r = trim(a.to_vec());
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let il = if b.last() == Some(&1) {
        1
    } else {
        f.inv(*b.last().unwrap())
    };
    while !r.is_empty() && r.len() >= b.len() {
        let j = r.len() - b.len();
        let c = if il == 1 {
            *r.last().unwrap()
        } else {
            f.mul(*r.last().unwrap(), il)
        };
        q[j] = c;
        for (k, &x) in b.iter().enumerate() {
            r[j + k] = f.sub(r[j + k], f.mul(c, x));
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn prem(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    pdiv(f, a, b).1
}
fn pexact(f: &Field, a: &[u64], b: &[u64]) -> Poly {
    let (q, r) = pdiv(f, a, b);
    assert!(r.is_empty(), "inexact polynomial division");
    q
}
fn pmonic(f: &Field, a: &[u64]) -> Poly {
    if a.is_empty() {
        vec![]
    } else if a.last() == Some(&1) {
        a.to_vec()
    } else {
        pscale(f, a, f.inv(*a.last().unwrap()))
    }
}
fn pegcd(f: &Field, a: &[u64], b: &[u64]) -> (Poly, Poly, Poly) {
    let (mut r0, mut r1, mut s0, mut s1, mut t0, mut t1) =
        (a.to_vec(), b.to_vec(), vec![1], vec![], vec![], vec![1]);
    while !r1.is_empty() {
        let (q, r) = pdiv(f, &r0, &r1);
        r0 = r1;
        r1 = r;
        let s = psub(f, &s0, &pmul(f, &q, &s1));
        s0 = s1;
        s1 = s;
        let t = psub(f, &t0, &pmul(f, &q, &t1));
        t0 = t1;
        t1 = t;
    }
    let inv = f.inv(*r0.last().unwrap());
    (
        pscale(f, &r0, inv),
        pscale(f, &s0, inv),
        pscale(f, &t0, inv),
    )
}
fn peval(f: &Field, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |r, &c| f.add(f.mul(r, x), c))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Div {
    degree: usize,
    u: [u64; 4],
    v: [u64; 3],
}
impl Div {
    const ZERO: Self = Self {
        degree: 0,
        u: [1, 0, 0, 0],
        v: [0, 0, 0],
    };
    fn new(u: &[u64], v: &[u64]) -> Self {
        assert!(!u.is_empty() && u.len() <= 4 && v.len() < u.len());
        let mut out = Self::ZERO;
        out.degree = u.len() - 1;
        out.u = [0; 4];
        out.u[..u.len()].copy_from_slice(u);
        out.v[..v.len()].copy_from_slice(v);
        out
    }
    fn up(self) -> Poly {
        self.u[..=self.degree].to_vec()
    }
    fn vp(self) -> Poly {
        trim(self.v[..self.degree].to_vec())
    }
}

struct Jac {
    f: Field,
    polynomial: Poly,
    additions: Cell<u64>,
}
impl Jac {
    fn new() -> Self {
        Self {
            f: Field::new(257, 1, 0),
            polynomial: vec![167, 104, 125, 138, 6, 13, 0, 1],
            additions: Cell::new(0),
        }
    }
    fn point(&self, x: u64, y: u64) -> Div {
        assert_eq!(self.f.sq(y), peval(&self.f, &self.polynomial, x));
        Div::new(&[self.f.neg(x), 1], &trim(vec![y]))
    }
    fn valid(&self, d: Div) -> bool {
        d.degree <= 3
            && d.u[d.degree] == 1
            && prem(
                &self.f,
                &psub(&self.f, &self.polynomial, &pmul(&self.f, &d.vp(), &d.vp())),
                &d.up(),
            )
            .is_empty()
    }
    fn neg(&self, d: Div) -> Div {
        let mut out = d;
        for x in &mut out.v[..d.degree] {
            *x = self.f.neg(*x);
        }
        out
    }
    fn reduce(&self, mut u: Poly, mut v: Poly) -> Div {
        let f = &self.f;
        u = pmonic(f, &u);
        v = prem(f, &v, &u);
        while u.len() > 4 {
            let next = pmonic(
                f,
                &pexact(f, &psub(f, &self.polynomial, &pmul(f, &v, &v)), &u),
            );
            v = prem(f, &pneg(f, &v), &next);
            assert!(next.len() < u.len(), "Cantor reduction did not descend");
            u = next;
        }
        Div::new(&u, &v)
    }
    fn add(&self, a: Div, b: Div) -> Div {
        self.additions.set(self.additions.get() + 1);
        if a.degree == 0 {
            return b;
        }
        if b.degree == 0 {
            return a;
        }
        let f = &self.f;
        let (ua, ub, va, vb) = (a.up(), b.up(), a.vp(), b.vp());
        let (d1, e1, e2) = pegcd(f, &ua, &ub);
        let (d, c1, c2) = pegcd(f, &d1, &padd(f, &va, &vb));
        let s1 = pmul(f, &c1, &e1);
        let s2 = pmul(f, &c1, &e2);
        let u = pexact(f, &pmul(f, &ua, &ub), &pmul(f, &d, &d));
        let vnum = padd(
            f,
            &padd(
                f,
                &pmul(f, &pmul(f, &s1, &ua), &vb),
                &pmul(f, &pmul(f, &s2, &ub), &va),
            ),
            &pmul(f, &c2, &padd(f, &pmul(f, &va, &vb), &self.polynomial)),
        );
        let v = prem(f, &pexact(f, &vnum, &d), &u);
        self.reduce(u, v)
    }
    fn mul(&self, mut a: Div, mut k: u64) -> Div {
        let mut out = Div::ZERO;
        while k > 0 {
            if k & 1 != 0 {
                out = self.add(out, a);
            }
            k >>= 1;
            if k > 0 {
                a = self.add(a, a);
            }
        }
        out
    }
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            out.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push(n);
    }
    out
}
fn partition(x: Div, seed: u64) -> usize {
    let mut h = seed ^ 0x9e3779b97f4a7c15;
    for &word in x.u.iter().chain(x.v.iter()) {
        h ^= word
            .wrapping_add(0x9e3779b97f4a7c15)
            .wrapping_add(h << 6)
            .wrapping_add(h >> 2);
    }
    (h & 15) as usize
}
fn order_by_collision(j: &Jac, p: Div) -> (u64, u64, u64) {
    let steps: Vec<_> = (0..16).map(|i| j.mul(p, 1u64 << i)).collect();
    let after_steps = j.f.counts();
    let mut seen = HashMap::<Div, u64>::new();
    let (mut x, mut coefficient) = (Div::ZERO, 0u64);
    for n in 0..100_000u64 {
        if let Some(&prior) = seen.get(&x) {
            let mut order = coefficient.abs_diff(prior);
            assert!(order > 0 && j.mul(p, order) == Div::ZERO);
            let mut factors_found = factors(order);
            for &prime in &factors_found {
                while order % prime == 0 && j.mul(p, order / prime) == Div::ZERO {
                    order /= prime;
                }
            }
            factors_found = factors(order);
            assert!(factors_found
                .iter()
                .all(|&prime| j.mul(p, order / prime) != Div::ZERO));
            assert_eq!(j.mul(p, order), Div::ZERO);
            return (order, n, after_steps.m);
        }
        seen.insert(x, coefficient);
        let slot = partition(x, 0);
        x = j.add(x, steps[slot]);
        coefficient += 1u64 << slot;
    }
    panic!("order collision not found within cap")
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn inverse(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1, mut t0, mut t1) = (m as i64, a as i64, 0i64, 1i64);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert_eq!(r0, 1);
    t0.rem_euclid(m as i64) as u64
}
fn lcg(x: &mut u64) -> u64 {
    *x = x
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *x
}
fn rho(
    j: &Jac,
    p: Div,
    q: Div,
    order: u64,
    seed: u64,
    negation_opt: bool,
) -> (u64, u64, u64, u64, Vec<serde_json::Value>) {
    let mut total_walk_steps = 0u64;
    let mut failures = Vec::new();
    for restart in 0..100u64 {
        let walk_seed = seed + 1009 * restart;
        let mut rng = walk_seed ^ 0xd1b54a32d192ed03;
        let steps: Vec<_> = (0..16)
            .map(|_| {
                let a = lcg(&mut rng) % order;
                let b = lcg(&mut rng) % order;
                (a, b, j.add(j.mul(p, a), j.mul(q, b)))
            })
            .collect();
        let mut seen = HashMap::<Div, (u64, u64)>::new();
        let (mut x, mut a, mut b) = (Div::ZERO, 0u64, 0u64);
        for n in 0..100_000u64 {
            if let Some(&(pa, pb)) = seen.get(&x) {
                let da = (pa + order - a) % order;
                let db = (b + order - pb) % order;
                let g = gcd(db, order);
                if db != 0 && da % g == 0 && g <= 64 {
                    let reduced = order / g;
                    let base = ((da / g) * inverse(db / g, reduced)) % reduced;
                    for h in 0..g {
                        let k = base + h * reduced;
                        if j.mul(p, k) == q {
                            return (k, n, total_walk_steps, g, failures);
                        }
                    }
                }
                failures.push(json!({"restart":restart,"collision_steps":n,"gcd":g,
                    "usable_congruence":db != 0 && da % g == 0 && g <= 64}));
                if !negation_opt {
                    break;
                }
            }
            seen.insert(x, (a, b));
            // A time-varying offset breaks the short fruitless cycles created
            // by quotienting a deterministic walk by the negation involution.
            let slot = (partition(x, walk_seed) + if negation_opt { n as usize } else { 0 }) & 15;
            let (sa, sb, step) = steps[slot];
            x = j.add(x, step);
            a = (a + sa) % order;
            b = (b + sb) % order;
            if negation_opt {
                let opposite = j.neg(x);
                if opposite < x {
                    x = opposite;
                    a = (order - a) % order;
                    b = (order - b) % order;
                }
            }
            total_walk_steps += 1;
        }
    }
    panic!("rho did not solve after restart cap")
}
fn main() {
    let j = Jac::new();
    let p = j.point(2, 7);
    let q = j.point(121, 5);
    assert!(j.valid(p) && j.valid(q));
    assert_eq!(j.add(p, j.neg(p)), Div::ZERO);
    j.f.reset();
    j.additions.set(0);
    let (order, discovery_steps, _) = order_by_collision(&j, p);
    assert_eq!(j.mul(q, order), Div::ZERO);
    let discovery_counts = j.f.counts();
    let discovery_adds = j.additions.get();
    let mut trials = Vec::new();
    let mut negation_trials = Vec::new();
    let mut recovered_for_transfer = None;
    for seed in 1..=5 {
        j.f.reset();
        j.additions.set(0);
        let (recovered, collision_steps, total_walk_steps, gcd, failed_collisions) =
            rho(&j, p, q, order, seed, false);
        recovered_for_transfer.get_or_insert(recovered);
        assert_eq!(recovered, 86_241 % order);
        trials.push(json!({"seed":seed,"recovered":recovered,
            "collision_steps":collision_steps,"collision_gcd":gcd,
            "total_walk_steps":total_walk_steps,"failed_collisions":failed_collisions,
            "source_verified":j.mul(p, recovered)==q,
            "jac_additions":j.additions.get(),"field_counts":j.f.counts()}));
        j.f.reset();
        j.additions.set(0);
        let (recovered, collision_steps, total_walk_steps, gcd, failed_collisions) =
            rho(&j, p, q, order, seed, true);
        assert_eq!(recovered, 86_241 % order);
        negation_trials.push(json!({"seed":seed,"recovered":recovered,
            "collision_steps":collision_steps,"collision_gcd":gcd,
            "total_walk_steps":total_walk_steps,"failed_collisions":failed_collisions,
            "source_verified":j.mul(p, recovered)==q,
            "jac_additions":j.additions.get(),"field_counts":j.f.counts()}));
    }
    j.f.reset();
    j.additions.set(0);
    let source_verified = j.mul(p, recovered_for_transfer.unwrap()) == q;
    assert!(source_verified);
    let source_verification_counts = j.f.counts();
    let source_verification_additions = j.additions.get();
    println!(
        "{}",
        json!({"schema":"tian-genus3-f257-source-rho/v1",
        "source_order":order,"source_order_factors":factors(order),
        "order_discovery_steps":discovery_steps,"order_discovery_additions":discovery_adds,
        "order_discovery_field_counts":discovery_counts,
        "rho_trials":trials,"rho_negation_trials":negation_trials,
        "independent_cantor_law":true,
        "transfer_source_verified":source_verified,
        "transfer_source_verification_counts":source_verification_counts,
        "transfer_source_verification_additions":source_verification_additions})
    );
}
