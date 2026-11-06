//! 2-torsion pairing of lift-dense lines u*F_q on y^2 + xy = x^3 + a x^2 + b
//! over F_{2^N}, N = d n (PROTOCOL.md, frozen at c7878791e4).
//!
//! Independent of `lift_dense_factor_base_20261027`: carry-less arithmetic
//! with Euclidean inversion (no log tables), own irreducibility test, own
//! Artin-Schreier solver and group law. Only the field polynomials are shared,
//! so that study's integer (a, b, u) values can be read.
//!
//!   pairing run <out_dir> <oct27 lines_amend3/rows.jsonl>
//!   pairing summarize <out_dir>

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write as _;

// ------------------------------------------------------------- field ----

#[derive(Clone)]
struct F {
    n: u32,
    poly: u64,
}

fn deg(a: u64) -> i32 {
    63 - a.leading_zeros() as i32
}

fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        while a != 0 && deg(a) >= deg(b) {
            a ^= b << (deg(a) - deg(b));
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut v = vec![];
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

impl F {
    fn new(n: u32, poly: u64) -> F {
        assert!(deg(poly) == n as i32 && n <= 24);
        let f = F { n, poly };
        // Rabin test: x^(2^n) = x mod f, gcd(x^(2^(n/p)) - x, f) = 1.
        assert_eq!(f.frob(2, n), 2, "x^(2^N) != x: not irreducible");
        for p in prime_factors(n) {
            assert_eq!(pgcd(f.frob(2, n / p) ^ 2, poly), 1, "reducible");
        }
        f
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        let mut r = 0u64;
        let mut bb = b;
        let mut i = 0;
        while bb != 0 {
            if bb & 1 == 1 {
                r ^= a << i;
            }
            bb >>= 1;
            i += 1;
        }
        let n = self.n as i32;
        while r != 0 && deg(r) >= n {
            r ^= self.poly << (deg(r) - n);
        }
        r
    }
    #[inline]
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    /// Binary extended Euclid (Hankerson-Menezes-Vanstone Alg. 2.48).
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        let (mut u, mut v, mut g1, mut g2) = (a, self.poly, 1u64, 0u64);
        while u != 1 {
            let mut j = deg(u) - deg(v);
            if j < 0 {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g1, &mut g2);
                j = -j;
            }
            u ^= v << j;
            g1 ^= g2 << j;
        }
        debug_assert_eq!(self.mul(a, g1), 1);
        g1
    }
    fn frob(&self, a: u64, k: u32) -> u64 {
        let mut z = a;
        for _ in 0..k {
            z = self.sq(z);
        }
        z
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.frob(a, self.n - 1)
    }
    /// Tr_{F_{2^N}/F_{2^d}}.
    fn tr_rel(&self, a: u64, d: u32) -> u64 {
        let (mut t, mut z) = (0u64, a);
        for _ in 0..self.n / d {
            t ^= z;
            z = self.frob(z, d);
        }
        t
    }
    fn tr(&self, a: u64) -> u64 {
        let t = self.tr_rel(a, 1);
        assert!(t <= 1);
        t
    }
    fn in_sub(&self, z: u64, d: u32) -> bool {
        self.frob(z, d) == z
    }
}

/// z^2 + z = w over F_{2^N}: echelon form of the F_2-linear map, pivots on
/// the lowest set bit, with tracked preimages.
struct As {
    piv: Vec<(u64, u64)>, // (image, preimage), distinct lowest bits
}
impl As {
    fn new(f: &F) -> As {
        let mut piv: Vec<(u64, u64)> = vec![];
        for i in 0..f.n {
            let (mut im, mut pre) = (f.sq(1u64 << i) ^ (1u64 << i), 1u64 << i);
            for &(r, p) in &piv {
                if im & (r & r.wrapping_neg()) != 0 {
                    im ^= r;
                    pre ^= p;
                }
            }
            if im != 0 {
                // keep reduced: clear the new pivot bit from older rows
                let lb = im & im.wrapping_neg();
                for e in piv.iter_mut() {
                    if e.0 & lb != 0 {
                        e.0 ^= im;
                        e.1 ^= pre;
                    }
                }
                piv.push((im, pre));
            }
        }
        assert_eq!(piv.len() as u32, f.n - 1);
        As { piv }
    }
    fn solve(&self, w: u64) -> Option<u64> {
        let (mut x, mut z) = (w, 0u64);
        for &(r, p) in &self.piv {
            if x & (r & r.wrapping_neg()) != 0 {
                x ^= r;
                z ^= p;
            }
        }
        (x == 0).then_some(z)
    }
}

// ------------------------------------------------------------- curve ----

type Pt = Option<(u64, u64)>; // None = O

struct Curve<'a> {
    f: &'a F,
    asv: &'a As,
    a: u64,
    b: u64,
    rb: u64, // sqrt(b)
}

impl<'a> Curve<'a> {
    fn new(f: &'a F, asv: &'a As, a: u64, b: u64) -> Curve<'a> {
        Curve {
            f,
            asv,
            a,
            b,
            rb: f.sqrt(b),
        }
    }
    fn on(&self, p: Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let f = self.f;
                let x2 = f.sq(x);
                f.sq(y) ^ f.mul(x, y) == f.mul(x2, x) ^ f.mul(self.a, x2) ^ self.b
            }
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        if x1 == x2 {
            if y2 == y1 ^ x1 {
                return None; // q = -p (includes T + T)
            }
            let lam = x1 ^ f.mul(y1, f.inv(x1));
            let x3 = f.sq(lam) ^ lam ^ self.a;
            let y3 = f.sq(x1) ^ f.mul(lam ^ 1, x3);
            return Some((x3, y3));
        }
        let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a;
        let y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
    fn mulk(&self, p: Pt, mut k: u64) -> Pt {
        let (mut r, mut s) = (None, p);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, s);
            }
            s = self.add(s, s);
            k >>= 1;
        }
        r
    }
    fn lifts(&self, x: u64) -> bool {
        x != 0 && self.f.tr(x ^ self.a ^ self.f.mul(self.rb, self.f.inv(x))) == 0
    }
    fn lift(&self, x: u64) -> Pt {
        if x == 0 {
            return None;
        }
        let f = self.f;
        let w = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        let z = self.asv.solve(w)?;
        let p = Some((x, f.mul(x, z)));
        assert!(self.on(p));
        p
    }
    fn t(&self) -> Pt {
        Some((0, self.rb))
    }
}

// --------------------------------------------------------------- rng ----

struct Rng(u64);
impl Rng {
    fn new(tag: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        Rng(h)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        // rejection, no modulo bias
        let zone = u64::MAX - (u64::MAX % m);
        loop {
            let v = self.next();
            if v < zone {
                return v % m;
            }
        }
    }
}

// ------------------------------------------------------------- cells ----

/// Same polynomials as the Oct 27 FIELDS table (irreducibility rechecked).
const POLYS: &[(u32, u64)] = &[
    (12, (1 << 12) | (1 << 3) | 1),
    (15, (1 << 15) | (1 << 1) | 1),
    (16, (1 << 16) | (1 << 5) | (1 << 3) | (1 << 1) | 1),
    (18, (1 << 18) | (1 << 3) | 1),
    (20, (1 << 20) | (1 << 3) | 1),
    (21, (1 << 21) | (1 << 2) | 1),
];
const CELLS: &[(u32, u32)] = &[
    (12, 6),
    (16, 8),
    (20, 10),
    (18, 9),
    (15, 5),
    (21, 7),
    (18, 6),
    (16, 4),
    (20, 5),
];

fn poly_of(n: u32) -> u64 {
    POLYS.iter().find(|p| p.0 == n).expect("field").1
}

fn targets_for(n: u32) -> usize {
    match n {
        2 => 2000,
        3 => 400,
        _ => 100,
    }
}

struct Ctx<'a> {
    f: &'a F,
    asv: &'a As,
    d: u32,
    n: u32,
    sub: &'a [u64],   // F_q^*
    kern: &'a [u64],  // nonzero u with Tr_{N/d}(u) = 0
}

fn canon(ctx: &Ctx, u: u64) -> u64 {
    ctx.sub.iter().map(|&c| ctx.f.mul(u, c)).min().unwrap()
}

/// Valid lines (Tr(u) = Tr(sqrt b / u) = 0), as sorted canonical reps.
fn valid_lines(ctx: &Ctx, cv: &Curve) -> (u64, Vec<u64>) {
    let f = ctx.f;
    let mut seen: HashSet<u64> = HashSet::new();
    let mut reps = vec![];
    let mut count = 0u64;
    for &u in ctx.kern {
        if f.tr_rel(f.mul(cv.rb, f.inv(u)), ctx.d) == 0 {
            count += 1;
            if !seen.contains(&u) {
                let r = canon(ctx, u);
                for &c in ctx.sub {
                    seen.insert(f.mul(u, c));
                }
                reps.push(r);
            }
        }
    }
    reps.sort_unstable();
    (count, reps)
}

fn is_tstable(ctx: &Ctx, cv: &Curve, u: u64) -> bool {
    let f = ctx.f;
    f.in_sub(f.mul(cv.rb, f.inv(f.sq(u))), ctx.d)
}

fn on_line(ctx: &Ctx, u: u64, x: u64) -> bool {
    x != 0 && ctx.f.in_sub(ctx.f.mul(x, ctx.f.inv(u)), ctx.d)
}

struct LineOut {
    k: usize,
    d_cols: usize,
    pair_ok: usize,
    tplus_ok: usize,
    x0_lifts: bool,
    raw: u64,
    rows: u64,     // PROTOCOL definition: distinct multisets {[2^e]P_i}
    rows_vec: u64, // post hoc: distinct signed column vectors (after +/- cancellation)
}

fn measure(ctx: &Ctx, cv: &Curve, u: u64, e: u32, targets: &[Pt]) -> LineOut {
    let f = ctx.f;
    let proj = |p: Pt| -> Pt {
        let mut r = p;
        for _ in 0..e {
            r = cv.add(r, r);
        }
        r
    };
    let mut fb: Vec<Pt> = vec![];
    let (mut pair_ok, mut tplus_ok, mut k) = (0, 0, 0);
    let mut cols: HashSet<u64> = HashSet::new();
    let b4 = f.sqrt(cv.rb);
    let mut x0_lifts = false;
    for &c in ctx.sub {
        let x = f.mul(u, c);
        if !cv.lifts(x) {
            continue;
        }
        k += 1;
        if x == b4 {
            x0_lifts = true;
        }
        let p = cv.lift(x);
        let xp = f.mul(cv.rb, f.inv(x));
        if on_line(ctx, u, xp) && cv.lifts(xp) {
            pair_ok += 1;
        }
        if cv.add(p, cv.t()).map(|v| v.0) == Some(xp) {
            tplus_ok += 1;
        }
        if let Some((px, _)) = proj(p) {
            cols.insert(px);
        }
        fb.push(p);
        fb.push(cv.neg(p));
    }
    // relations: multisets of m = n FB points summing to R
    let idx: HashMap<(u64, u64), usize> = fb
        .iter()
        .enumerate()
        .map(|(i, p)| (p.unwrap(), i))
        .collect();
    let pj: Vec<Option<(u64, i64)>> = fb
        .iter()
        .map(|&p| {
            proj(p).map(|(x, y)| {
                let canon_y = y.min(x ^ y);
                (x, if y == canon_y { 1 } else { -1 })
            })
        })
        .collect();
    let m = ctx.n as usize;
    let (mut raw, mut rows, mut rows_vec) = (0u64, 0u64, 0u64);
    let pp: Vec<Pt> = fb.iter().map(|&p| proj(p)).collect();
    for &r in targets {
        let mut found: Vec<Vec<usize>> = vec![];
        let mut chosen: Vec<usize> = vec![];
        rec(cv, &fb, &idx, m, 0, r, &mut chosen, &mut found);
        raw += found.len() as u64;
        let mut msset: HashSet<Vec<Pt>> = HashSet::new();
        let mut rowset: HashSet<Vec<(u64, i64)>> = HashSet::new();
        for t in &found {
            let mut ms: Vec<Pt> = t.iter().map(|&i| pp[i]).collect();
            ms.sort_unstable();
            msset.insert(ms);
            let mut acc: HashMap<u64, i64> = HashMap::new();
            for &i in t {
                if let Some((x, s)) = pj[i] {
                    *acc.entry(x).or_default() += s;
                }
            }
            let mut v: Vec<(u64, i64)> = acc.into_iter().filter(|e| e.1 != 0).collect();
            v.sort_unstable();
            rowset.insert(v);
        }
        rows += msset.len() as u64;
        rows_vec += rowset.len() as u64;
    }
    LineOut {
        k,
        d_cols: cols.len(),
        pair_ok,
        tplus_ok,
        x0_lifts,
        raw,
        rows,
        rows_vec,
    }
}

#[allow(clippy::too_many_arguments)]
fn rec(
    cv: &Curve,
    fb: &[Pt],
    idx: &HashMap<(u64, u64), usize>,
    left: usize,
    start: usize,
    acc: Pt,
    chosen: &mut Vec<usize>,
    found: &mut Vec<Vec<usize>>,
) {
    if left == 1 {
        if let Some(v) = acc {
            if let Some(&j) = idx.get(&v) {
                if j >= start {
                    let mut t = chosen.clone();
                    t.push(j);
                    found.push(t);
                }
            }
        }
        return;
    }
    for i in start..fb.len() {
        chosen.push(i);
        rec(cv, fb, idx, left - 1, i, cv.add(acc, cv.neg(fb[i])), chosen, found);
        chosen.pop();
    }
}

fn curve_order(f: &F, cv: &Curve) -> u64 {
    let mut cnt = 0u64;
    for x in 1..(1u64 << f.n) {
        cnt += cv.lifts(x) as u64;
    }
    2 + 2 * cnt
}

fn rand_el(rng: &mut Rng, f: &F) -> u64 {
    1 + rng.below((1u64 << f.n) - 1)
}

#[allow(clippy::too_many_arguments)]
fn run_curve(
    ctx: &Ctx,
    arm: &str,
    ci: usize,
    a: u64,
    b: u64,
    nl_max: usize,
    seed_tag: &str,
) -> Vec<String> {
    let f = ctx.f;
    let cv = Curve::new(f, ctx.asv, a, b);
    let order = curve_order(f, &cv);
    let e = order.trailing_zeros();
    let mut rng = Rng::new(seed_tag);
    // order sanity: [#E] P = O on 4 random points
    for _ in 0..4 {
        let p = loop {
            let x = rand_el(&mut rng, f);
            if cv.lifts(x) {
                break cv.lift(x);
            }
        };
        assert!(cv.mulk(p, order).is_none(), "order check");
    }
    let b4 = f.sqrt(cv.rb);
    let trb4 = f.tr_rel(b4, ctx.d);
    let (u_count, reps) = valid_lines(ctx, &cv);
    let ts: Vec<u64> = reps
        .iter()
        .copied()
        .filter(|&u| is_tstable(ctx, &cv, u))
        .collect();
    let p1_ok = ts.len() == (trb4 == 0) as usize && ts.iter().all(|&u| u == canon(ctx, b4));
    let mut out = vec![format!(
        "{{\"kind\":\"curve\",\"arm\":\"{arm}\",\"N\":{},\"d\":{},\"n\":{},\"curve\":{ci},\"a\":{a},\"b\":{b},\"order\":{order},\"e2\":{e},\"tr_b4_zero\":{},\"u_count\":{u_count},\"lines\":{},\"tstable_lines\":{},\"p1_ok\":{p1_ok}}}",
        f.n, ctx.d, ctx.n, trb4 == 0, reps.len(), ts.len()
    )];
    // lines to measure
    let mut lines: Vec<(&str, u64)> = vec![];
    match arm {
        "tstable" | "generic" => {
            for &u in &ts {
                lines.push(("L_T", u));
            }
            for &u in reps.iter().filter(|u| !ts.contains(u)) {
                if lines.len() >= nl_max {
                    break;
                }
                lines.push(("valid", u));
            }
            if arm == "generic" {
                lines.push(("std", 1));
            }
        }
        "poscontrol" => lines.push(("std_bFq", 1)),
        _ => unreachable!(),
    }
    let nt = targets_for(ctx.n);
    let targets: Vec<Pt> = (0..nt)
        .map(|_| loop {
            let x = rand_el(&mut rng, f);
            if cv.lifts(x) {
                let p = cv.lift(x);
                break if rng.next() & 1 == 1 { cv.neg(p) } else { p };
            }
        })
        .collect();
    for (role, u) in lines {
        let o = measure(ctx, &cv, u, e, &targets);
        let tst = is_tstable(ctx, &cv, u);
        let valid = f.tr_rel(u, ctx.d) == 0 && f.tr_rel(f.mul(cv.rb, f.inv(u)), ctx.d) == 0;
        out.push(format!(
            "{{\"kind\":\"line\",\"arm\":\"{arm}\",\"N\":{},\"d\":{},\"n\":{},\"curve\":{ci},\"role\":\"{role}\",\"u\":{u},\"tstable\":{tst},\"valid\":{valid},\"q1\":{},\"k\":{},\"cols\":{},\"pair_ok\":{},\"tplus_ok\":{},\"x0_lifts\":{},\"targets\":{nt},\"raw\":{},\"rows\":{},\"rows_vec\":{}}}",
            f.n, ctx.d, ctx.n, ctx.sub.len(), o.k, o.d_cols, o.pair_ok, o.tplus_ok, o.x0_lifts, o.raw, o.rows, o.rows_vec
        ));
    }
    out
}

fn run_cell(nf: u32, d: u32) -> Vec<String> {
    let f = F::new(nf, poly_of(nf));
    let asv = As::new(&f);
    let sub: Vec<u64> = (1..(1u64 << nf)).filter(|&z| f.in_sub(z, d)).collect();
    assert_eq!(sub.len() as u64, (1u64 << d) - 1);
    let kern: Vec<u64> = (1..(1u64 << nf)).filter(|&u| f.tr_rel(u, d) == 0).collect();
    let ctx = Ctx {
        f: &f,
        asv: &asv,
        d,
        n: nf / d,
        sub: &sub,
        kern: &kern,
    };
    // draw curves (sequential, deterministic), then measure in parallel
    let mut specs: Vec<(String, usize, u64, u64, String)> = vec![];
    for (arm, count) in [("tstable", 4usize), ("generic", 4), ("poscontrol", 2)] {
        let mut rng = Rng::new(&format!("pairing-20261106/{arm}/{nf}/{d}"));
        for ci in 0..count {
            let a = loop {
                let a = rng.below(1u64 << nf);
                if f.tr(a) == 0 {
                    break a;
                }
            };
            let b = loop {
                let b = if arm == "poscontrol" {
                    sub[rng.below(sub.len() as u64) as usize]
                } else {
                    rand_el(&mut rng, &f)
                };
                let z = f.tr_rel(f.sqrt(f.sqrt(b)), d) == 0;
                match arm {
                    "tstable" if z => break b,
                    "generic" if !z => break b,
                    "poscontrol" => break b,
                    _ => {}
                }
            };
            specs.push((
                arm.to_string(),
                ci,
                a,
                b,
                format!("pairing-20261106/targets/{arm}/{nf}/{d}/{ci}"),
            ));
        }
    }
    let mut results: Vec<Vec<String>> = vec![vec![]; specs.len()];
    std::thread::scope(|s| {
        let chunks: Vec<_> = results.chunks_mut(1).zip(specs.iter()).collect();
        let mut handles = vec![];
        for (slot, (arm, ci, a, b, tag)) in chunks {
            let ctx = &ctx;
            handles.push(s.spawn(move || {
                let nl = if arm == "tstable" { 4 } else { 4 };
                slot[0] = run_curve(ctx, arm, *ci, *a, *b, nl, tag);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
    results.into_iter().flatten().collect()
}

// ------------------------------------------------------- flat json ----

fn jget<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = if let Some(stripped) = rest.strip_prefix('"') {
        return stripped.find('"').map(|j| &stripped[..j]);
    } else {
        rest.find([',', '}']).unwrap_or(rest.len())
    };
    Some(rest[..end].trim())
}
fn jnum(line: &str, key: &str) -> u64 {
    jget(line, key).and_then(|v| v.parse().ok()).unwrap_or_else(|| panic!("{key} in {line}"))
}
fn jbool(line: &str, key: &str) -> bool {
    jget(line, key) == Some("true")
}

fn prior_rows(path: &str) -> Vec<String> {
    let text = std::fs::read_to_string(path).expect("oct27 rows");
    let mut out = vec![];
    let mut fields: HashMap<u32, (F, As)> = HashMap::new();
    for l in text.lines() {
        if jget(l, "line") != Some("twisted") || jget(l, "u") == Some("null") {
            continue;
        }
        let (nf, d, a, b, u) = (
            jnum(l, "N") as u32,
            jnum(l, "d") as u32,
            jnum(l, "a"),
            jnum(l, "b"),
            jnum(l, "u"),
        );
        let (f, asv) = fields.entry(nf).or_insert_with(|| {
            let f = F::new(nf, poly_of(nf));
            let a = As::new(&f);
            (f, a)
        });
        let cv = Curve::new(f, asv, a, b);
        let valid = f.tr_rel(u, d) == 0 && f.tr_rel(f.mul(cv.rb, f.inv(u)), d) == 0;
        let tst = f.in_sub(f.mul(cv.rb, f.inv(f.sq(u))), d);
        let trb4 = f.tr_rel(f.sqrt(cv.rb), d) == 0;
        out.push(format!(
            "{{\"kind\":\"prior\",\"N\":{nf},\"d\":{d},\"n\":{},\"curve\":{},\"Tr_a\":{},\"a\":{a},\"b\":{b},\"u\":{u},\"u_count\":{},\"valid\":{valid},\"tstable\":{tst},\"tr_b4_zero\":{trb4}}}",
            nf / d,
            jnum(l, "curve"),
            jnum(l, "Tr_a"),
            jnum(l, "u_count")
        ));
    }
    out
}

// ------------------------------------------------------- summarize ----

fn summarize(dir: &str) {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let rows: Vec<&str> = text.lines().collect();
    let curves: Vec<&str> = rows.iter().copied().filter(|l| jget(l, "kind") == Some("curve")).collect();
    let lines: Vec<&str> = rows.iter().copied().filter(|l| jget(l, "kind") == Some("line")).collect();
    let prior: Vec<&str> = rows.iter().copied().filter(|l| jget(l, "kind") == Some("prior")).collect();
    let mut md = String::new();
    let mut js = String::from("{");

    // P1
    let p1_bad = curves.iter().filter(|l| !jbool(l, "p1_ok")).count();
    let n_ts = curves.iter().filter(|l| jnum(l, "tstable_lines") > 0).count();
    let p1 = p1_bad == 0;
    writeln!(md, "## P1: T-stable valid lines = [Tr_{{N/d}}(b^{{1/4}}) = 0], and equal b^{{1/4}}F_q\n\n{} curves, {} with a T-stable valid line, {} mismatches: **{}**\n", curves.len(), n_ts, p1_bad, if p1 { "PASS" } else { "FAIL" }).unwrap();
    write!(js, "\"P1\":{{\"curves\":{},\"tstable\":{n_ts},\"mismatch\":{p1_bad},\"pass\":{p1}}}", curves.len()).unwrap();

    // P2 / P2'
    let ts_all: Vec<&str> = lines.iter().copied().filter(|l| jbool(l, "tstable") && jnum(l, "k") == jnum(l, "q1")).collect();
    let mut p2_exact_bad = 0;
    let mut p2_eq = 0;
    let mut p2_le_bad = 0;
    for l in &ts_all {
        let (k, c) = (jnum(l, "k"), jnum(l, "cols"));
        if jnum(l, "pair_ok") != k || jnum(l, "tplus_ok") != k {
            p2_exact_bad += 1;
        }
        let bound = (k - 1) / 2;
        if c > bound {
            p2_le_bad += 1;
        }
        if c == bound {
            p2_eq += 1;
        }
    }
    let pos: Vec<&str> = lines.iter().copied().filter(|l| jget(l, "role") == Some("std_bFq")).collect();
    let mut pos_bad = 0;
    for l in &pos {
        let k = jnum(l, "k");
        let bound = (k - jbool(l, "x0_lifts") as u64) / 2;
        if jnum(l, "cols") > bound || jnum(l, "pair_ok") != k || jnum(l, "tplus_ok") != k || !jbool(l, "tstable") {
            pos_bad += 1;
        }
    }
    let p2 = !ts_all.is_empty()
        && p2_exact_bad == 0
        && p2_le_bad == 0
        && (p2_eq as f64) >= 0.9 * ts_all.len() as f64
        && pos_bad == 0;
    for nn in 2..=4u64 {
        let g: Vec<&&str> = ts_all.iter().filter(|l| jnum(l, "n") == nn).collect();
        let eq = g.iter().filter(|l| jnum(l, "cols") == (jnum(l, "k") - 1) / 2).count();
        writeln!(md, "- post hoc, n = {nn}: D = (k-1)/2 on {eq}/{} T-stable all-lift lines", g.len()).unwrap();
    }
    let nts: Vec<&str> = lines.iter().copied().filter(|l| !jbool(l, "tstable")).collect();
    let nts_ok = nts.iter().filter(|l| jnum(l, "cols") + 2 >= jnum(l, "k")).count();
    let p2n = !nts.is_empty() && (nts_ok as f64) >= 0.95 * nts.len() as f64;
    writeln!(md, "## P2: pairing on T-stable all-lift lines\n\n{} lines; pairing/translation exact on all but {}; D <= (k-1)/2 violated on {}; equality on {}/{}. Positive control (F_q, b in F_q): {} lines, {} violations. **{}**\n", ts_all.len(), p2_exact_bad, p2_le_bad, p2_eq, ts_all.len(), pos.len(), pos_bad, if p2 { "PASS" } else { "FAIL" }).unwrap();
    writeln!(md, "## P2': no pairing elsewhere\n\n{} non-T-stable lines, D >= k-2 on {}: **{}**\n", nts.len(), nts_ok, if p2n { "PASS" } else { "FAIL" }).unwrap();
    write!(js, ",\"P2\":{{\"lines\":{},\"exact_bad\":{p2_exact_bad},\"le_bad\":{p2_le_bad},\"eq\":{p2_eq},\"pos_lines\":{},\"pos_bad\":{pos_bad},\"pass\":{p2}}},\"P2n\":{{\"lines\":{},\"ok\":{nts_ok},\"pass\":{p2n}}}", ts_all.len(), pos.len(), nts.len()).unwrap();

    // P3, P4 per cell
    writeln!(md, "## P3 / P4 per cell (m = n; pooled over curves)\n\n| N | d | n | group | lines | k (mean) | D (mean) | raw | rows | raw/rows | rho = rows/(targets D) | rho / rho(std) | rows_vec (post hoc) |\n|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    let mut p3_bad: Vec<String> = vec![];
    let mut p4_bad: Vec<String> = vec![];
    let mut p3_pos_ok = true;
    let mut cells_js: Vec<String> = vec![];
    let mut fig: Vec<(String, u32, f64, f64, f64)> = vec![]; // label, n, raw/rows T, raw/rows other, rhoT/std
    for &(nf, d) in CELLS {
        let n = nf / d;
        let in_cell: Vec<&str> = lines.iter().copied().filter(|l| jnum(l, "N") as u32 == nf && jnum(l, "d") as u32 == d).collect();
        let groups: [(&str, Box<dyn Fn(&str) -> bool>); 4] = [
            ("L_T (T-stable)", Box::new(|l: &str| jget(l, "role") == Some("L_T"))),
            ("valid, not T-stable", Box::new(|l: &str| jget(l, "role") == Some("valid"))),
            ("std (generic b)", Box::new(|l: &str| jget(l, "role") == Some("std"))),
            ("poscontrol F_q, b in F_q", Box::new(|l: &str| jget(l, "role") == Some("std_bFq"))),
        ];
        let mut stat: Vec<Option<(f64, f64)>> = vec![];
        let mut rho_std = None;
        let mut tmp: Vec<(usize, f64, f64, f64, f64, u64, u64, f64, u64)> = vec![];
        for (gi, (_, pred)) in groups.iter().enumerate() {
            let g: Vec<&str> = in_cell.iter().copied().filter(|l| pred(l)).collect();
            if g.is_empty() {
                stat.push(None);
                continue;
            }
            let raw: u64 = g.iter().map(|l| jnum(l, "raw")).sum();
            let rws: u64 = g.iter().map(|l| jnum(l, "rows")).sum();
            let rwv: u64 = g.iter().map(|l| jnum(l, "rows_vec")).sum();
            let kmean = g.iter().map(|l| jnum(l, "k") as f64).sum::<f64>() / g.len() as f64;
            let dmean = g.iter().map(|l| jnum(l, "cols") as f64).sum::<f64>() / g.len() as f64;
            // rho pooled: sum rows / sum (targets * D)
            let denom: f64 = g.iter().map(|l| (jnum(l, "targets") * jnum(l, "cols")) as f64).sum();
            let rho = if denom > 0.0 { rws as f64 / denom } else { f64::NAN };
            let rr = if rws > 0 { raw as f64 / rws as f64 } else { f64::NAN };
            if gi == 2 {
                rho_std = Some(rho);
            }
            stat.push(Some((rr, rho)));
            tmp.push((gi, g.len() as f64, kmean, dmean, rr, raw, rws, rho, rwv));
        }
        let mut cell_js = format!("{{\"N\":{nf},\"d\":{d},\"n\":{n}");
        for (gi, cnt, kmean, dmean, rr, raw, rws, rho, rwv) in &tmp {
            let rel = rho_std.map(|s| rho / s);
            writeln!(md, "| {nf} | {d} | {n} | {} | {} | {:.1} | {:.1} | {} | {} | {:.3} | {:.3e} | {} | {} |", groups[*gi].0, *cnt as u64, kmean, dmean, raw, rws, rr, rho, rel.map(|r| format!("{r:.3}")).unwrap_or("n/a".into()), rwv).unwrap();
            write!(cell_js, ",\"g{gi}\":{{\"lines\":{},\"raw\":{raw},\"rows\":{rws},\"rows_vec\":{rwv},\"raw_over_rows\":{rr:.6},\"rho\":{rho:.6e},\"rho_rel_std\":{}}}", *cnt as u64, rel.map(|r| format!("{r:.6}")).unwrap_or("null".into())).unwrap();
        }
        cell_js.push('}');
        cells_js.push(cell_js);
        let hi = (1u64 << (n - 1)) as f64;
        // P3
        if let Some((rr, _)) = stat[0] {
            if !(rr >= 0.6 * hi - 1e-9 && rr <= hi + 1e-9) {
                p3_bad.push(format!("({nf},{d}) L_T raw/rows {rr:.3}"));
            }
        }
        for gi in [1, 2] {
            if let Some((rr, _)) = stat[gi] {
                if rr > 1.05 {
                    p3_bad.push(format!("({nf},{d}) g{gi} raw/rows {rr:.3}"));
                }
            }
        }
        if let Some((rr, _)) = stat[3] {
            if rr.is_nan() || rr < 1.3 {
                p3_pos_ok = false;
            }
        }
        // P4
        if let Some(s) = rho_std {
            if let Some((_, r)) = stat[0] {
                let v = r / s;
                if !(1.3..=3.0).contains(&v) {
                    p4_bad.push(format!("({nf},{d}) rho(L_T)/rho(std) {v:.3}"));
                }
            }
            if let Some((_, r)) = stat[1] {
                let v = r / s;
                if !(v >= 0.6 * hi && v <= 1.6 * hi) {
                    p4_bad.push(format!("({nf},{d}) rho(valid)/rho(std) {v:.3} vs 2^(n-1) = {hi}"));
                }
            }
        }
        fig.push((
            format!("({nf},{d})"),
            n,
            stat[0].map(|s| s.0).unwrap_or(f64::NAN),
            stat[1].map(|s| s.0).unwrap_or(f64::NAN),
            match (stat[0], rho_std) {
                (Some((_, r)), Some(s)) => r / s,
                _ => f64::NAN,
            },
        ));
    }
    // post hoc (not a decision rule): rows_vec, the signed column vectors
    writeln!(md, "\n### Post hoc: rows as signed column vectors (rows_vec), relative to std and to other valid lines\n\n| N | d | n | rho_vec(L_T)/rho_vec(std) | rho_vec(valid)/rho_vec(std) | rho_vec(L_T)/rho_vec(valid) | predicted L_T/valid = 2/2^(n-1) |\n|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for &(nf, d) in CELLS {
        let n = nf / d;
        let rv = |role: &str| -> Option<f64> {
            let g: Vec<&str> = lines.iter().copied().filter(|l| jnum(l, "N") as u32 == nf && jnum(l, "d") as u32 == d && jget(l, "role") == Some(role)).collect();
            if g.is_empty() { return None; }
            let num: u64 = g.iter().map(|l| jnum(l, "rows_vec")).sum();
            let den: f64 = g.iter().map(|l| (jnum(l, "targets") * jnum(l, "cols")) as f64).sum();
            Some(num as f64 / den)
        };
        let (t, v, s0) = (rv("L_T"), rv("valid"), rv("std"));
        let f = |a: Option<f64>, b: Option<f64>| match (a, b) { (Some(a), Some(b)) if b > 0.0 => format!("{:.3}", a / b), _ => "n/a".into() };
        writeln!(md, "| {nf} | {d} | {n} | {} | {} | {} | {:.3} |", f(t, s0), f(v, s0), f(t, v), 2.0 / (1u64 << (n - 1)) as f64).unwrap();
    }
    let p3 = p3_bad.is_empty() && p3_pos_ok;
    let p4 = p4_bad.is_empty();
    writeln!(md, "\nP3 (raw/rows in [0.6, 1] x 2^(n-1) on L_T, <= 1.05 elsewhere, poscontrol >= 1.3): **{}** {:?} poscontrol_ok={p3_pos_ok}\n\nP4 (rho(L_T)/rho(std) in [1.3, 3]; rho(valid)/rho(std) in [0.6, 1.6] x 2^(n-1)): **{}** {:?}\n", if p3 { "PASS" } else { "FAIL" }, p3_bad, if p4 { "PASS" } else { "FAIL" }, p4_bad).unwrap();
    write!(js, ",\"P3\":{{\"pass\":{p3},\"poscontrol_ok\":{p3_pos_ok},\"violations\":{:?}}},\"P4\":{{\"pass\":{p4},\"violations\":{:?}}},\"cells\":[{}]", p3_bad, p4_bad, cells_js.join(",")).unwrap();

    // P6
    let pv = prior.iter().filter(|l| jbool(l, "valid")).count();
    let pt: Vec<&str> = prior.iter().copied().filter(|l| jbool(l, "tstable")).collect();
    let pta0 = pt.iter().filter(|l| jnum(l, "Tr_a") == 0).count();
    writeln!(md, "## P6: Oct 27 twisted lines on L_T\n\n{} recorded twisted rows ({} valid by this instrument); {} T-stable ({} of them with Tr(a) = 0).\n", prior.len(), pv, pt.len(), pta0).unwrap();
    for l in &pt {
        writeln!(md, "- N = {}, d = {}, curve {}, Tr(a) = {}, u = {}", jnum(l, "N"), jnum(l, "d"), jnum(l, "curve"), jnum(l, "Tr_a"), jnum(l, "u")).unwrap();
    }
    write!(js, ",\"P6\":{{\"rows\":{},\"valid\":{pv},\"tstable\":{},\"tstable_tra0\":{pta0}}}}}", prior.len(), pt.len()).unwrap();
    std::fs::write(format!("{dir}/SUMMARY.md"), md).unwrap();
    std::fs::write(format!("{dir}/summary.json"), js + "\n").unwrap();
    std::fs::write(format!("{dir}/fig1_pairing.svg"), svg(&fig)).unwrap();
    eprintln!("P1 {p1} P2 {p2} P2' {p2n} P3 {p3} P4 {p4}");
}

/// Two panels: raw/rows per cell (log2) and rho(L_T)/rho(std), with the
/// predicted values 2^(n-1) and 2 drawn as ticks.
fn svg(fig: &[(String, u32, f64, f64, f64)]) -> String {
    let (w, h) = (860.0, 360.0);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n");
    let panel = |s: &mut String, x0: f64, title: &str, ymax: f64, vals: &dyn Fn(&(String, u32, f64, f64, f64)) -> Vec<(f64, &'static str)>, pred: &dyn Fn(u32) -> f64| {
        let (pw, ph, top) = (380.0, 250.0, 50.0);
        let y = |v: f64| top + ph - (v.max(0.0) / ymax).min(1.0) * ph;
        writeln!(s, "<text x=\"{}\" y=\"28\" font-size=\"13\" font-weight=\"bold\">{title}</text>", x0).unwrap();
        writeln!(s, "<line x1=\"{x0}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\"/>", top + ph, x0 + pw, top + ph).unwrap();
        writeln!(s, "<line x1=\"{x0}\" y1=\"{top}\" x2=\"{x0}\" y2=\"{}\" stroke=\"black\"/>", top + ph).unwrap();
        let mut t = 0.0;
        while t <= ymax + 1e-9 {
            writeln!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{t}</text><line x1=\"{x0}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#ddd\"/>", x0 - 4.0, y(t) + 4.0, y(t), x0 + pw, y(t)).unwrap();
            t += if ymax > 6.0 { 2.0 } else { 1.0 };
        }
        let bw = pw / fig.len() as f64;
        for (i, c) in fig.iter().enumerate() {
            let cx = x0 + bw * i as f64;
            let vs = vals(c);
            let sub = (bw - 6.0) / vs.len() as f64;
            for (j, (v, col)) in vs.iter().enumerate() {
                if v.is_nan() {
                    writeln!(s, "<text x=\"{}\" y=\"{}\" font-size=\"8\" fill=\"#888\">n/a</text>", cx + 3.0 + sub * j as f64, top + ph - 3.0).unwrap();
                    continue;
                }
                writeln!(s, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{col}\"/>", cx + 3.0 + sub * j as f64, y(*v), sub - 1.0, top + ph - y(*v)).unwrap();
            }
            let pv = pred(c.1);
            writeln!(s, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#c0392b\" stroke-width=\"2\" stroke-dasharray=\"3,2\"/>", cx + 1.0, y(pv), cx + bw - 1.0, y(pv)).unwrap();
            writeln!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"9\">{}</text><text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"9\">n={}</text>", cx + bw / 2.0, top + ph + 14.0, c.0, cx + bw / 2.0, top + ph + 26.0, c.1).unwrap();
        }
    };
    panel(&mut s, 50.0, "raw relations / distinct rows (m = n)", 8.0, &|c| vec![(c.2, "#2e86c1"), (c.3, "#95a5a6")], &|n| (1u64 << (n - 1)) as f64);
    panel(&mut s, 470.0, "rows per column, L_T / standard line (registered rows)", 5.0, &|c| vec![(c.4, "#2e86c1")], &|_| 2.0);
    writeln!(s, "<rect x=\"50\" y=\"330\" width=\"10\" height=\"10\" fill=\"#2e86c1\"/><text x=\"64\" y=\"339\">L_T = b^(1/4) F_q (T-stable)</text><rect x=\"230\" y=\"330\" width=\"10\" height=\"10\" fill=\"#95a5a6\"/><text x=\"244\" y=\"339\">other valid lines</text><line x1=\"380\" y1=\"335\" x2=\"400\" y2=\"335\" stroke=\"#c0392b\" stroke-width=\"2\" stroke-dasharray=\"3,2\"/><text x=\"404\" y=\"339\">preregistered prediction (2^(n-1); 2)</text>").unwrap();
    s.push_str("</svg>\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => {
            let out = &args[2];
            std::fs::create_dir_all(out).unwrap();
            let mut fh = std::fs::File::create(format!("{out}/rows.jsonl")).unwrap();
            for r in prior_rows(&args[3]) {
                writeln!(fh, "{r}").unwrap();
            }
            for &(nf, d) in CELLS {
                let t = std::time::Instant::now();
                for r in run_cell(nf, d) {
                    writeln!(fh, "{r}").unwrap();
                }
                fh.flush().unwrap();
                eprintln!("cell ({nf},{d}) {:.1} s", t.elapsed().as_secs_f64());
            }
        }
        Some("summarize") => summarize(&args[2]),
        _ => eprintln!("usage: pairing run <out> <oct27 rows.jsonl> | pairing summarize <out>"),
    }
}
