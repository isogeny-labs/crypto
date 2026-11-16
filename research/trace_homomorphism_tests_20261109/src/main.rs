//! Exact tests of the trace-homomorphism claims in the 2026-11-09 assessment.
//! std only, single thread, deterministic seed. Every result is an exact count
//! or an exact algebraic statement on toy fields; no timing is measured.
//!
//! T1  Kosters–Yeo class chi(P) = Tr(x + a) on y^2 + xy = x^3 + a x^2 + b:
//!     homomorphism E(F_2^n) -> F_2, kernel 2E, vanishes on the prime subgroup.
//! T2  Filter ceiling: trace / cofactor-class pruning of m-tuples never loses a
//!     relation landing in <G>; kept fractions 1/2 and 1/|E/<G>|.
//! T3  Point trace Tr_{2^n/2} on Koblitz curves: lands in E(F_2), Tr o iota = [n],
//!     kills <G>.
//! T4  Tr_{N/d} shifted line u*F_{2^d} (PR #2686 construction): lift counts and
//!     exact four-summand relation yield against the standard subfield line.
//! T5  Weil-descent algebra: degree at which the trace linear form enters the
//!     Macaulay span of the S3 system (m = 2) and of the split S3 system
//!     (m = 3), against first-fall and solving degrees; column counts for
//!     appending versus substituting it.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

// ---------------------------------------------------------------- rng
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

// ---------------------------------------------------------------- F_2[x]
fn pdeg(a: u64) -> i32 {
    if a == 0 {
        -1
    } else {
        63 - a.leading_zeros() as i32
    }
}
fn pmulmod(mut a: u64, mut b: u64, f: u64) -> u64 {
    let n = pdeg(f) as u32;
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 != 0 {
            a ^= f;
        }
    }
    r
}
fn pmod(mut a: u64, f: u64) -> u64 {
    let df = pdeg(f);
    while pdeg(a) >= df {
        a ^= f << (pdeg(a) - df);
    }
    a
}
fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = pmod(a, b);
        a = b;
        b = r;
    }
    a
}
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            v.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        v.push(n);
    }
    v
}
/// Rabin irreducibility test.
fn irreducible(f: u64) -> bool {
    let n = pdeg(f) as u32;
    let x = 2u64;
    let frob = |k: u32| {
        let mut t = x;
        for _ in 0..k {
            t = pmulmod(t, t, f);
        }
        t
    };
    if frob(n) != x {
        return false;
    }
    for q in prime_factors(n as u64) {
        if pgcd(f, frob(n / q as u32) ^ x) != 1 {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------- GF(2^n)
#[derive(Clone)]
struct Gf {
    n: u32,
    f: u64,
    /// XOR basis for z -> z^2 + z, indexed by leading bit: (image, preimage).
    as_basis: Vec<Option<(u64, u64)>>,
}
impl Gf {
    fn new(n: u32) -> Gf {
        let mut f = (1u64 << n) | 1;
        while !irreducible(f) {
            f += 2;
        }
        let mut g = Gf { n, f, as_basis: vec![None; n as usize] };
        for k in 0..n {
            let (mut v, mut pre) = (g.sqr(1 << k) ^ (1 << k), 1u64 << k);
            while v != 0 {
                let lb = pdeg(v) as usize;
                match g.as_basis[lb] {
                    Some((img, p)) => {
                        v ^= img;
                        pre ^= p;
                    }
                    None => {
                        g.as_basis[lb] = Some((v, pre));
                        break;
                    }
                }
            }
        }
        g
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        pmulmod(a, b, self.f)
    }
    fn sqr(&self, a: u64) -> u64 {
        pmulmod(a, a, self.f)
    }
    fn frob(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sqr(a);
        }
        a
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        // a^(2^n - 2)
        let mut r = 1;
        let mut s = a;
        for _ in 1..self.n {
            s = self.sqr(s);
            r = self.mul(r, s);
        }
        r
    }
    fn tr(&self, a: u64) -> u64 {
        self.rel_tr(a, 1)
    }
    /// Tr_{2^n / 2^d}(a), d | n.
    fn rel_tr(&self, a: u64, d: u32) -> u64 {
        let mut t = 0;
        let mut x = a;
        for _ in 0..self.n / d {
            t ^= x;
            x = self.frob(x, d);
        }
        t
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.frob(a, self.n - 1)
    }
    /// One root of z^2 + z = w, if Tr(w) = 0.
    fn solve_as(&self, mut w: u64) -> Option<u64> {
        let mut z = 0;
        while w != 0 {
            let lb = pdeg(w) as usize;
            let (img, pre) = self.as_basis[lb]?;
            w ^= img;
            z ^= pre;
        }
        Some(z)
    }
}

// ---------------------------------------------------------------- curve
type Pt = Option<(u64, u64)>;
fn key(p: Pt) -> u64 {
    match p {
        None => u64::MAX,
        Some((x, y)) => (x << 32) | y,
    }
}
#[derive(Clone)]
struct Curve {
    gf: Gf,
    a: u64,
    b: u64,
}
impl Curve {
    fn on(&self, p: Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let g = &self.gf;
                let l = g.sqr(y) ^ g.mul(x, y);
                let x2 = g.sqr(x);
                let r = g.mul(x2, x) ^ g.mul(self.a, x2) ^ self.b;
                l == r
            }
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let g = &self.gf;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        if x1 == x2 {
            if y1 != y2 || x1 == 0 {
                return None; // Q = -P, or P = T of order 2
            }
            let l = x1 ^ g.mul(y1, g.inv(x1));
            let x3 = g.sqr(l) ^ l ^ self.a;
            let y3 = g.sqr(x1) ^ g.mul(l ^ 1, x3);
            return Some((x3, y3));
        }
        let l = g.mul(y1 ^ y2, g.inv(x1 ^ x2));
        let x3 = g.sqr(l) ^ l ^ x1 ^ x2 ^ self.a;
        let y3 = g.mul(l, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
    fn smul(&self, k: u64, p: Pt) -> Pt {
        let mut r = None;
        for i in (0..64).rev() {
            r = self.add(r, r);
            if (k >> i) & 1 != 0 {
                r = self.add(r, p);
            }
        }
        r
    }
    fn lift(&self, x: u64) -> Option<u64> {
        let g = &self.gf;
        if x == 0 {
            return Some(g.sqrt(self.b));
        }
        let w = x ^ self.a ^ g.mul(self.b, g.inv(g.sqr(x)));
        g.solve_as(w).map(|z| g.mul(x, z))
    }
    fn points(&self) -> Vec<Pt> {
        let mut v = vec![None];
        for x in 0..(1u64 << self.gf.n) {
            if let Some(y) = self.lift(x) {
                let p = Some((x, y));
                debug_assert!(self.on(p));
                v.push(p);
                if x != 0 {
                    v.push(Some((x, x ^ y)));
                }
            }
        }
        v
    }
    fn chi(&self, p: Pt) -> u64 {
        match p {
            None => 0,
            Some((x, _)) => self.gf.tr(x ^ self.a),
        }
    }
    fn frob(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (self.gf.sqr(x), self.gf.sqr(y)))
    }
}
/// (rv, h): rv = r^v is the full power of the largest prime r in n, h = n / rv,
/// so gcd(rv, h) = 1 and <G> below means the r-primary component.
fn order_split(n: u64) -> (u64, u64) {
    let r = *prime_factors(n).iter().max().unwrap();
    let mut rv = 1;
    while n % (rv * r) == 0 {
        rv *= r;
    }
    (rv, n / rv)
}
fn subgroup_gen(c: &Curve, pts: &[Pt], r: u64, h: u64, rng: &mut Rng) -> Pt {
    loop {
        let p = pts[rng.below(pts.len() as u64) as usize];
        let g = c.smul(h, p);
        if g.is_some() {
            assert!(c.smul(r, g).is_none(), "rv*G != O");
            return g;
        }
    }
}

// ---------------------------------------------------------------- T1
fn t1(n: u32, a: u64, b: u64, label: &str, rng: &mut Rng, out: &mut String) {
    let c = Curve { gf: Gf::new(n), a, b };
    let pts = c.points();
    let order = pts.len() as u64;
    // self-test: group law on random triples (associativity) and on-curve
    for _ in 0..200 {
        let p = pts[rng.below(order) as usize];
        let q = pts[rng.below(order) as usize];
        let s = pts[rng.below(order) as usize];
        assert!(c.on(c.add(p, q)));
        assert_eq!(c.add(c.add(p, q), s), c.add(p, c.add(q, s)));
        assert!(c.smul(order, p).is_none());
    }
    let mut viol = 0u64;
    let pairs;
    if order <= 1500 {
        pairs = order * order;
        for &p in &pts {
            for &q in &pts {
                if c.chi(c.add(p, q)) != c.chi(p) ^ c.chi(q) {
                    viol += 1;
                }
            }
        }
    } else {
        pairs = 300_000;
        for _ in 0..pairs {
            let p = pts[rng.below(order) as usize];
            let q = pts[rng.below(order) as usize];
            if c.chi(c.add(p, q)) != c.chi(p) ^ c.chi(q) {
                viol += 1;
            }
        }
    }
    let doubles: HashSet<u64> = pts.iter().map(|&p| key(c.add(p, p))).collect();
    let ker: HashSet<u64> = pts.iter().filter(|&&p| c.chi(p) == 0).map(|&p| key(p)).collect();
    let (r, h) = order_split(order);
    let g = subgroup_gen(&c, &pts, r, h, rng);
    let mut on_g_nonzero = 0u64;
    let mut checked = 0u64;
    let mut q = None;
    let lim = r.min(200_000);
    for _ in 0..lim {
        q = c.add(q, g);
        checked += 1;
        on_g_nonzero += c.chi(q);
    }
    let _ = writeln!(
        out,
        "{{\"test\":\"T1\",\"curve\":\"{label}\",\"n\":{n},\"a\":{a},\"b\":{b},\"order\":{order},\"r\":{r},\"h\":{h},\
\"pairs_checked\":{pairs},\"exhaustive_pairs\":{},\"additivity_violations\":{viol},\"kernel_size\":{},\"two_E_size\":{},\
\"kernel_equals_2E\":{},\"surjective\":{},\"G_multiples_checked\":{checked},\"chi_nonzero_on_G\":{on_g_nonzero}}}",
        order <= 1500,
        ker.len(),
        doubles.len(),
        ker == doubles,
        ker.len() as u64 * 2 == order
    );
}

// ---------------------------------------------------------------- T2
fn t2(n: u32, a: u64, b: u64, k: usize, label: &str, rng: &mut Rng, out: &mut String) {
    let c = Curve { gf: Gf::new(n), a, b };
    let pts = c.points();
    let order = pts.len() as u64;
    let (r, h) = order_split(order);
    // base: k random liftable abscissae, both points each
    let mut xs: Vec<u64> = pts.iter().filter_map(|p| p.map(|v| v.0)).filter(|&x| x != 0).collect();
    xs.sort();
    xs.dedup();
    let mut base = Vec::new();
    for _ in 0..k {
        let i = rng.below(xs.len() as u64) as usize;
        let x = xs.swap_remove(i);
        let y = c.lift(x).unwrap();
        base.push(Some((x, y)));
        base.push(Some((x, x ^ y)));
    }
    let class: Vec<u64> = base.iter().map(|&p| key(c.smul(r, p))).collect();
    let rb: Vec<Pt> = base.iter().map(|&p| c.smul(r, p)).collect();
    for m in [2usize, 3] {
        let (mut total, mut keep_tr, mut keep_cl, mut hits, mut lost_tr, mut lost_cl) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        let nb = base.len();
        let mut idx = vec![0usize; m];
        // ordered non-decreasing index tuples (multisets)
        loop {
            let mut s = None;
            let mut chi = 0;
            let mut cl = None;
            for &i in &idx {
                s = c.add(s, base[i]);
                chi ^= c.chi(base[i]);
                cl = c.add(cl, rb[i]);
            }
            let _ = &class;
            total += 1;
            let in_g = s.is_some() && c.smul(r, s).is_none();
            if chi == 0 {
                keep_tr += 1;
            }
            if cl.is_none() {
                keep_cl += 1;
            }
            if in_g {
                hits += 1;
                if chi != 0 {
                    lost_tr += 1;
                }
                if cl.is_some() {
                    lost_cl += 1;
                }
            }
            // next multiset
            let mut j = m;
            loop {
                if j == 0 {
                    break;
                }
                j -= 1;
                if idx[j] + 1 < nb {
                    idx[j] += 1;
                    for t in j + 1..m {
                        idx[t] = idx[j];
                    }
                    break;
                }
                if j == 0 {
                    j = usize::MAX;
                    break;
                }
            }
            if j == usize::MAX {
                break;
            }
        }
        let _ = writeln!(
            out,
            "{{\"test\":\"T2\",\"curve\":\"{label}\",\"n\":{n},\"order\":{order},\"r\":{r},\"h\":{h},\"base_points\":{},\"m\":{m},\
\"tuples\":{total},\"kept_by_trace\":{keep_tr},\"kept_by_cofactor_class\":{keep_cl},\"tuples_in_G\":{hits},\
\"lost_by_trace\":{lost_tr},\"lost_by_cofactor_class\":{lost_cl},\"trace_keep_fraction\":{:.6},\"class_keep_fraction\":{:.6}}}",
            base.len(),
            keep_tr as f64 / total as f64,
            keep_cl as f64 / total as f64
        );
    }
}

// ---------------------------------------------------------------- T3
fn t3(n: u32, a: u64, rng: &mut Rng, out: &mut String) {
    let c = Curve { gf: Gf::new(n), a, b: 1 };
    let pts = c.points();
    let order = pts.len() as u64;
    let (r, h) = order_split(order);
    let trace = |p: Pt| {
        let mut s = None;
        let mut q = p;
        for _ in 0..n {
            s = c.add(s, q);
            q = c.frob(q);
        }
        s
    };
    let mut not_in_base = 0u64;
    let mut image: HashSet<u64> = HashSet::new();
    for &p in &pts {
        let t = trace(p);
        if let Some((x, y)) = t {
            if x > 1 || y > 1 {
                not_in_base += 1;
            }
        }
        image.insert(key(t));
    }
    // E(F_2) embedded
    let small: Vec<Pt> = pts.iter().cloned().filter(|p| p.map_or(true, |(x, y)| x <= 1 && y <= 1)).collect();
    let mut iota_bad = 0;
    for &q in &small {
        if trace(q) != c.smul(n as u64, q) {
            iota_bad += 1;
        }
    }
    let g = subgroup_gen(&c, &pts, r, h, rng);
    let mut g_nonzero = 0u64;
    let mut q = None;
    let lim = r.min(100_000);
    for _ in 0..lim {
        q = c.add(q, g);
        if trace(q).is_some() {
            g_nonzero += 1;
        }
    }
    let _ = writeln!(
        out,
        "{{\"test\":\"T3\",\"n\":{n},\"a\":{a},\"b\":1,\"order\":{order},\"r\":{r},\"h\":{h},\"E_F2_order\":{},\
\"points_traced\":{order},\"trace_outside_E_F2\":{not_in_base},\"image_size\":{},\"trace_iota_ne_n\":{iota_bad},\
\"G_multiples_checked\":{lim},\"trace_nonzero_on_G\":{g_nonzero}}}",
        small.len(),
        image.len()
    );
}

// ---------------------------------------------------------------- T4
fn t4(d: u32, b: u64, rng: &mut Rng, out: &mut String) -> Option<()> {
    let nn = 4 * d;
    let c = Curve { gf: Gf::new(nn), a: 0, b };
    let g = &c.gf;
    let pts = c.points();
    let order = pts.len() as u64;
    let (r, h) = order_split(order);
    let sub: Vec<u64> = (0..(1u64 << nn)).filter(|&x| g.frob(x, d) == x).collect();
    assert_eq!(sub.len(), 1 << d);
    let cc = g.sqrt(b);
    let mut u = 0;
    let mut scanned = 0;
    for cand in 1..(1u64 << nn) {
        scanned += 1;
        if g.rel_tr(cand, d) == 0 && g.rel_tr(g.mul(cc, g.inv(cand)), d) == 0 {
            u = cand;
            break;
        }
    }
    if u == 0 {
        return None;
    }
    let mut res = Vec::new();
    for (name, line) in [("standard", sub.clone()), ("trace", sub.iter().map(|&v| g.mul(u, v)).collect::<Vec<_>>())] {
        let mut base = Vec::new();
        let mut lifts = 0;
        for &x in &line {
            if x == 0 {
                continue;
            }
            if let Some(y) = c.lift(x) {
                lifts += 1;
                base.push(Some((x, y)));
                base.push(Some((x, x ^ y)));
            }
        }
        // ordered pair sums
        let mut pair: HashMap<u64, (Pt, u64)> = HashMap::new();
        for &p in &base {
            for &q in &base {
                let s = c.add(p, q);
                pair.entry(key(s)).or_insert((s, 0)).1 += 1;
            }
        }
        // class [r]S; S1 + S2 in <G> iff class(S1) + class(S2) = O
        let mut by_class: HashMap<u64, (Pt, u64)> = HashMap::new();
        for (_, &(s, cnt)) in &pair {
            let cl = c.smul(r, s);
            by_class.entry(key(cl)).or_insert((cl, 0)).1 += cnt;
        }
        let mut in_g: u128 = 0;
        for (_, &(cl, cnt)) in &by_class {
            if let Some(&(_, c2)) = by_class.get(&key(c.neg(cl))) {
                in_g += cnt as u128 * c2 as u128;
            }
        }
        let mut zero: u128 = 0;
        for (_, &(s, cnt)) in &pair {
            if let Some(&(_, c2)) = pair.get(&key(c.neg(s))) {
                zero += cnt as u128 * c2 as u128;
            }
        }
        let rel = in_g - zero; // ordered 4-tuples summing to a nonzero element of <G>
        let per_target = rel as f64 / (r - 1) as f64;
        res.push((name, lifts, base.len(), rel, per_target));
    }
    let _ = rng;
    let ratio = res[1].4 / res[0].4;
    for (name, lifts, bl, rel, pt) in &res {
        let _ = writeln!(
            out,
            "{{\"test\":\"T4\",\"d\":{d},\"N\":{nn},\"b\":{b},\"order\":{order},\"r\":{r},\"h\":{h},\"u\":{u},\"u_scanned\":{scanned},\
\"line\":\"{name}\",\"nonzero_x\":{},\"lifting_x\":{lifts},\"base_points\":{bl},\"ordered_4tuples_to_Gstar\":{rel},\
\"mean_decompositions_per_target\":{pt:.6},\"trace_over_standard\":{ratio:.6}}}",
            (1u64 << d) - 1
        );
    }
    Some(())
}

// ---------------------------------------------------------------- T5: Boolean algebra
type Mono = u32;
/// Polynomial with GF(2^n) coefficients over multilinear Boolean monomials.
#[derive(Clone, Default)]
struct FPoly(HashMap<Mono, u64>);
impl FPoly {
    fn konst(c: u64) -> FPoly {
        let mut p = FPoly::default();
        if c != 0 {
            p.0.insert(0, c);
        }
        p
    }
    fn add(&self, o: &FPoly) -> FPoly {
        let mut r = self.0.clone();
        for (&m, &c) in &o.0 {
            let e = r.entry(m).or_insert(0);
            *e ^= c;
            if *e == 0 {
                r.remove(&m);
            }
        }
        FPoly(r)
    }
    fn mul(&self, o: &FPoly, g: &Gf) -> FPoly {
        let mut r: HashMap<Mono, u64> = HashMap::new();
        for (&m1, &c1) in &self.0 {
            for (&m2, &c2) in &o.0 {
                *r.entry(m1 | m2).or_insert(0) ^= g.mul(c1, c2);
            }
        }
        r.retain(|_, c| *c != 0);
        FPoly(r)
    }
    fn sq(&self, g: &Gf) -> FPoly {
        let mut r: HashMap<Mono, u64> = HashMap::new();
        for (&m, &c) in &self.0 {
            *r.entry(m).or_insert(0) ^= g.sqr(c);
        }
        r.retain(|_, c| *c != 0);
        FPoly(r)
    }
    fn linear(vars: &[u32], coeffs: &[u64]) -> FPoly {
        let mut p = FPoly::default();
        for (&v, &c) in vars.iter().zip(coeffs) {
            if c != 0 {
                *p.0.entry(1 << v).or_insert(0) ^= c;
            }
        }
        p
    }
    /// Weil descent: n Boolean polynomials (component k of every coefficient).
    fn descend(&self, n: u32) -> Vec<Vec<Mono>> {
        (0..n)
            .map(|k| {
                let mut v: Vec<Mono> = self.0.iter().filter(|(_, &c)| (c >> k) & 1 != 0).map(|(&m, _)| m).collect();
                v.sort();
                v
            })
            .filter(|v| !v.is_empty())
            .collect()
    }
}
/// char-2 Semaev S3 for y^2 + xy = x^3 + a x^2 + b.
fn s3(x1: &FPoly, x2: &FPoly, x3: &FPoly, b: u64, g: &Gf) -> FPoly {
    let e = x1.mul(x2, g).add(&x1.mul(x3, g)).add(&x2.mul(x3, g));
    e.sq(g).add(&x1.mul(x2, g).mul(x3, g)).add(&FPoly::konst(b))
}
fn s3f(x1: u64, x2: u64, x3: u64, b: u64, g: &Gf) -> u64 {
    let e = g.mul(x1, x2) ^ g.mul(x1, x3) ^ g.mul(x2, x3);
    g.sqr(e) ^ g.mul(g.mul(x1, x2), x3) ^ b
}

struct Macaulay {
    nv: u32,
    cols: HashMap<Mono, usize>,
    deg_of_col: Vec<u32>,
    words: usize,
    pivots: HashMap<usize, Vec<u64>>,
}
impl Macaulay {
    fn new(nv: u32, dmax: u32) -> Macaulay {
        // graded, highest degree first
        let mut monos: Vec<Mono> = Vec::new();
        for d in (0..=dmax).rev() {
            let mut v: Vec<Mono> = (0..(1u64 << nv)).map(|m| m as Mono).filter(|m| m.count_ones() == d).collect();
            v.sort_unstable_by(|a, b| b.cmp(a));
            monos.extend(v);
        }
        let cols: HashMap<Mono, usize> = monos.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let deg_of_col = monos.iter().map(|m| m.count_ones()).collect();
        let words = (monos.len() + 63) / 64;
        Macaulay { nv, cols, deg_of_col, words, pivots: HashMap::new() }
    }
    fn row(&self, p: &[Mono]) -> Vec<u64> {
        let mut r = vec![0u64; self.words];
        for m in p {
            let c = self.cols[m];
            r[c / 64] ^= 1 << (c % 64);
        }
        r
    }
    fn lead(r: &[u64]) -> Option<usize> {
        for (i, &w) in r.iter().enumerate() {
            if w != 0 {
                return Some(i * 64 + w.trailing_zeros() as usize);
            }
        }
        None
    }
    fn reduce(&self, mut r: Vec<u64>) -> Vec<u64> {
        while let Some(l) = Self::lead(&r) {
            match self.pivots.get(&l) {
                Some(p) => {
                    for (a, b) in r.iter_mut().zip(p) {
                        *a ^= b;
                    }
                }
                None => break,
            }
        }
        r
    }
    fn insert(&mut self, r: Vec<u64>) -> bool {
        let r = self.reduce(r);
        match Self::lead(&r) {
            Some(l) => {
                self.pivots.insert(l, r);
                true
            }
            None => false,
        }
    }
    fn in_span(&self, p: &[Mono]) -> bool {
        Self::lead(&self.reduce(self.row(p))).is_none()
    }
    fn lin_rank(&self) -> usize {
        self.pivots.keys().filter(|&&c| self.deg_of_col[c] <= 1).count()
    }
    fn inconsistent(&self) -> bool {
        self.pivots.keys().any(|&c| self.deg_of_col[c] == 0)
    }
}
fn mul_bool(p: &[Mono], m: Mono) -> Vec<Mono> {
    let mut s: HashMap<Mono, u8> = HashMap::new();
    for &t in p {
        *s.entry(t | m).or_insert(0) ^= 1;
    }
    let mut v: Vec<Mono> = s.into_iter().filter(|(_, c)| *c == 1).map(|(m, _)| m).collect();
    v.sort();
    v
}
fn binom_le(nv: u32, d: u32) -> u64 {
    let mut s = 0u64;
    let mut c = 1u64;
    for k in 0..=d.min(nv) {
        if k > 0 {
            c = c * (nv - k + 1) as u64 / k as u64;
        }
        s += c;
    }
    s
}
fn build(nv: u32, dmax: u32, eqs: &[Vec<Mono>], extra: Option<&Vec<Mono>>) -> Vec<(u32, Macaulay)> {
    let mut res = Vec::new();
    for d in 1..=dmax {
        let mut mac = Macaulay::new(nv, d);
        let mut all: Vec<&Vec<Mono>> = eqs.iter().collect();
        if let Some(e) = extra {
            all.push(e);
        }
        for e in all {
            let de = e.iter().map(|m| m.count_ones()).max().unwrap_or(0);
            if de > d {
                continue;
            }
            for m in 0..(1u64 << nv) {
                let m = m as Mono;
                if m.count_ones() <= d - de {
                    let p = mul_bool(e, m);
                    if p.iter().all(|t| t.count_ones() <= d) {
                        let r = mac.row(&p);
                        mac.insert(r);
                    }
                }
            }
        }
        res.push((d, mac));
    }
    res
}

struct T5Case {
    label: String,
    n: u32,
    ell: u32,
    m: u32,
    planted: bool,
}
fn t5(cs: &T5Case, rng: &mut Rng, out: &mut String) {
    let g = Gf::new(cs.n);
    let n = cs.n;
    let ell = cs.ell;
    // random curve with a = 0 or 1, b != 0
    let a = rng.below(2);
    let b = 1 + rng.below((1 << n) - 1);
    let c = Curve { gf: g.clone(), a, b };
    // factor base V = span{theta^0 .. theta^(ell-1)}
    let vbasis: Vec<u64> = (0..ell).map(|i| 1u64 << i).collect();
    let span = |bits: u64| -> u64 { (0..ell).filter(|i| (bits >> i) & 1 != 0).fold(0, |acc, i| acc ^ vbasis[i as usize]) };
    // self-test of S3 on random point sums
    let pts = c.points();
    for _ in 0..200 {
        let p = pts[1 + rng.below(pts.len() as u64 - 1) as usize];
        let q = pts[1 + rng.below(pts.len() as u64 - 1) as usize];
        if let Some((x3, _)) = c.add(p, q) {
            assert_eq!(s3f(p.unwrap().0, q.unwrap().0, x3, b, &g), 0, "S3 self-test");
        }
    }
    // target R
    let base_pts: Vec<Pt> = (1..(1u64 << ell)).filter_map(|bits| {
        let x = span(bits);
        c.lift(x).map(|y| Some((x, y)))
    }).collect();
    let xr = if cs.planted {
        loop {
            let mut s = None;
            for _ in 0..cs.m {
                s = c.add(s, base_pts[rng.below(base_pts.len() as u64) as usize]);
            }
            if let Some((x, _)) = s {
                break x;
            }
        }
    } else {
        loop {
            let p = pts[rng.below(pts.len() as u64) as usize];
            if let Some((x, _)) = p {
                break x;
            }
        }
    };
    // variables: X_0..X_(ell-1), Y_.., [Z_..], [U_0..U_(n-1)]
    let xv: Vec<u32> = (0..ell).collect();
    let yv: Vec<u32> = (ell..2 * ell).collect();
    let zv: Vec<u32> = (2 * ell..3 * ell).collect();
    let uoff = if cs.m == 3 { 3 * ell } else { 2 * ell };
    let uv: Vec<u32> = (uoff..uoff + n).collect();
    let nv = if cs.m == 3 { 3 * ell + n } else { 2 * ell };
    let x1 = FPoly::linear(&xv, &vbasis);
    let x2 = FPoly::linear(&yv, &vbasis);
    let xrp = FPoly::konst(xr);
    let mut eqs: Vec<Vec<Mono>> = Vec::new();
    let mut lin: Vec<Mono> = Vec::new();
    let trv: Vec<u64> = vbasis.iter().map(|&v| g.tr(v)).collect();
    if cs.m == 2 {
        eqs.extend(s3(&x1, &x2, &xrp, b, &g).descend(n));
        for (i, &t) in trv.iter().enumerate() {
            if t == 1 {
                lin.push(1 << xv[i]);
                lin.push(1 << yv[i]);
            }
        }
        // Tr(x1)+Tr(x2)+Tr(xR)+(m+1)Tr(a) = 0, m = 2
        if (g.tr(xr) ^ (3 * g.tr(a)) & 1) & 1 == 1 {
            lin.push(0);
        }
    } else {
        let x3 = FPoly::linear(&zv, &vbasis);
        let ub: Vec<u64> = (0..n).map(|k| 1u64 << k).collect();
        let up = FPoly::linear(&uv, &ub);
        eqs.extend(s3(&x1, &x2, &up, b, &g).descend(n));
        eqs.extend(s3(&up, &x3, &xrp, b, &g).descend(n));
        for (i, &t) in trv.iter().enumerate() {
            if t == 1 {
                lin.push(1 << xv[i]);
                lin.push(1 << yv[i]);
                lin.push(1 << zv[i]);
            }
        }
        // m = 3: (m+1)Tr(a) = 0
        if g.tr(xr) == 1 {
            lin.push(0);
        }
    }
    lin.sort();
    // negative control: a random linear form on the same variables
    let mut ctrl: Vec<Mono> = Vec::new();
    for v in 0..nv {
        if rng.below(2) == 1 {
            ctrl.push(1 << v);
        }
    }
    if ctrl.is_empty() {
        ctrl.push(1);
    }
    // brute force: solutions of the descent system (over x1,x2[,x3] in V, u in F_2^n)
    let mut sols = 0u64;
    let mut sols_violate_l = 0u64;
    let mut sols_nonlift = 0u64;
    let eval_l = |assign: u64| -> u64 {
        lin.iter().fold(0, |acc, &mo| acc ^ (((assign & mo as u64) == mo as u64) as u64))
    };
    if cs.m == 2 {
        for bx in 0..(1u64 << ell) {
            for by in 0..(1u64 << ell) {
                if s3f(span(bx), span(by), xr, b, &g) == 0 {
                    sols += 1;
                    let assign = bx | (by << ell);
                    sols_violate_l += eval_l(assign);
                    if c.lift(span(bx)).is_none() || c.lift(span(by)).is_none() {
                        sols_nonlift += 1;
                    }
                }
            }
        }
    } else {
        for bx in 0..(1u64 << ell) {
            for by in 0..(1u64 << ell) {
                for bz in 0..(1u64 << ell) {
                    for u in 0..(1u64 << n) {
                        if s3f(span(bx), span(by), u, b, &g) == 0 && s3f(u, span(bz), xr, b, &g) == 0 {
                            sols += 1;
                            let assign = bx | (by << ell) | (bz << (2 * ell)) | (u << uoff);
                            sols_violate_l += eval_l(assign);
                            if [bx, by, bz].iter().any(|&t| c.lift(span(t)).is_none()) {
                                sols_nonlift += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let dmax = if nv <= 14 { 5 } else { 4 };
    let sys = build(nv, dmax, &eqs, None);
    let sysl = build(nv, dmax, &eqs, Some(&lin));
    let mut rows = String::new();
    for ((d, ms), (_, ml)) in sys.iter().zip(sysl.iter()) {
        let _ = write!(
            rows,
            "{}{{\"D\":{d},\"cols\":{},\"cols_substituted\":{},\"rank\":{},\"lin_rank\":{},\"inconsistent\":{},\"L_in_span\":{},\"ctrl_in_span\":{},\
\"rank_with_L\":{},\"lin_rank_with_L\":{}}}",
            if rows.is_empty() { "" } else { "," },
            binom_le(nv, *d),
            binom_le(nv - 1, *d),
            ms.pivots.len(),
            ms.lin_rank(),
            ms.inconsistent(),
            ms.in_span(&lin),
            ms.in_span(&ctrl),
            ml.pivots.len(),
            ml.lin_rank()
        );
    }
    let _ = ms_unused(&sys);
    let _ = writeln!(
        out,
        "{{\"test\":\"T5\",\"case\":\"{}\",\"n\":{n},\"ell\":{ell},\"m\":{},\"planted\":{},\"a\":{a},\"b\":{b},\"xR\":{xr},\"vars\":{nv},\
\"equations\":{},\"eq_degrees\":{:?},\"L_terms\":{},\"solutions\":{sols},\"solutions_violating_L\":{sols_violate_l},\
\"solutions_with_nonlifting_x\":{sols_nonlift},\"by_degree\":[{rows}]}}",
        cs.label,
        cs.m,
        cs.planted,
        eqs.len(),
        eqs.iter().map(|e| e.iter().map(|m| m.count_ones()).max().unwrap_or(0)).collect::<Vec<_>>(),
        lin.len()
    );
}
fn ms_unused(_: &[(u32, Macaulay)]) {}

fn main() {
    let seed = 0x5eed_2026_1109_0001u64;
    eprintln!("seed {seed:#x}");
    let mut rng = Rng(seed);
    let mut out = String::new();
    let only: Vec<String> = std::env::args().skip(1).collect();
    let run = |t: &str| only.is_empty() || only.iter().any(|o| o == t);

    if run("T1") {
        for &n in &[5u32, 7, 11, 13, 17] {
            eprintln!("T1 n={n}"); t1(n, 0, 1, "koblitz_a0", &mut rng, &mut out);
            t1(n, 1, 1, "koblitz_a1", &mut rng, &mut out);
        }
        for &n in &[8u32, 9, 10, 12, 14] {
            let a = rng.below(1 << n);
            let b = 1 + rng.below((1 << n) - 1);
            t1(n, a, b, "random", &mut rng, &mut out);
        }
        eprintln!("T1 done");
    }
    if run("T2") {
        t2(13, 0, 1, 40, "koblitz_a0", &mut rng, &mut out);
        // a random curve with a larger cofactor
        loop {
            let n = 12;
            let a = rng.below(1 << n);
            let b = 1 + rng.below((1 << n) - 1);
            let c = Curve { gf: Gf::new(n), a, b };
            let ord = c.points().len() as u64;
            let (_, h) = order_split(ord);
            if h >= 8 {
                t2(n, a, b, 40, "random_h>=8", &mut rng, &mut out);
                break;
            }
        }
        eprintln!("T2 done");
    }
    if run("T3") {
        for &n in &[7u32, 11, 13, 17] {
            t3(n, 0, &mut rng, &mut out);
            t3(n, 1, &mut rng, &mut out);
        }
        eprintln!("T3 done");
    }
    if run("T4") {
        for &d in &[3u32, 4, 5] {
            let mut done = 0;
            while done < 4 {
                let b = 1 + rng.below((1 << (4 * d)) - 1);
                if t4(d, b, &mut rng, &mut out).is_some() {
                    done += 1;
                }
            }
            eprintln!("T4 d={d} done");
        }
    }
    if run("T5") {
        let cases = [
            T5Case { label: "S3_n11_l3_planted".into(), n: 11, ell: 3, m: 2, planted: true },
            T5Case { label: "S3_n13_l4_planted".into(), n: 13, ell: 4, m: 2, planted: true },
            T5Case { label: "S3_n13_l4_random".into(), n: 13, ell: 4, m: 2, planted: false },
            T5Case { label: "split_n7_l2_planted".into(), n: 7, ell: 2, m: 3, planted: true },
            T5Case { label: "split_n7_l2_random".into(), n: 7, ell: 2, m: 3, planted: false },
            T5Case { label: "split_n11_l3_planted".into(), n: 11, ell: 3, m: 3, planted: true },
        ];
        for cs in &cases {
            t5(cs, &mut rng, &mut out);
            eprintln!("T5 {} done", cs.label);
        }
    }
    print!("{out}");
}
