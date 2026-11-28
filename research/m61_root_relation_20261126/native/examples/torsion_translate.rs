//! Rational torsion translations and normalized Velu images of root bases.
//! Run: cargo run --release --example torsion_translate
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pt {
    x: u64,
    y: u64,
    o: bool,
}
const O: Pt = Pt {
    x: 0,
    y: 0,
    o: true,
};
fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pw(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 == 1 {
            z = mm(z, a, p)
        }
        a = mm(a, a, p);
        e >>= 1;
    }
    z
}
fn neg(q: Pt, p: u64) -> Pt {
    if q.o {
        q
    } else {
        Pt {
            y: if q.y == 0 { 0 } else { p - q.y },
            ..q
        }
    }
}
fn add(u: Pt, v: Pt, p: u64, adds: &mut u64) -> Pt {
    *adds += 1;
    if u.o {
        return v;
    }
    if v.o {
        return u;
    }
    if u.x == v.x && (u.y + v.y).is_multiple_of(p) {
        return O;
    }
    let (num, den) = if u.x == v.x {
        ((3 * mm(u.x, u.x, p) + p - 3) % p, (2 * u.y) % p)
    } else {
        ((v.y + p - u.y) % p, (v.x + p - u.x) % p)
    };
    let lam = mm(num, pw(den, p - 2, p), p);
    let x = (mm(lam, lam, p) + 2 * p - u.x - v.x) % p;
    Pt {
        x,
        y: (mm(lam, (u.x + p - x) % p, p) + p - u.y) % p,
        o: false,
    }
}
fn lift(x: u64, p: u64) -> Option<Pt> {
    let rhs = (mm(mm(x, x, p), x, p) + p - mm(3, x, p) + 40) % p;
    if rhs == 0 {
        return Some(Pt { x, y: 0, o: false });
    }
    let y = pw(rhs, (p + 1) / 4, p);
    (mm(y, y, p) == rhs).then_some(Pt {
        x,
        y: y.min(p - y),
        o: false,
    })
}
fn roots(p: u64, d: u64, z: u64) -> Vec<Pt> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut x = 1;
    for _ in 0..d {
        assert!(seen.insert(x));
        if let Some(q) = lift(x, p) {
            if q.y != 0 {
                out.push(q)
            }
        }
        x = mm(x, z, p)
    }
    assert_eq!(x, 1);
    out
}
fn rnd(state: &mut u64) -> u64 {
    let mut z = (*state).wrapping_add(0x9e3779b97f4a7c15);
    *state = z;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn random_base(p: u64, b: usize, seed: u64) -> Vec<Pt> {
    let mut st = seed;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    while out.len() < b {
        let x = 1 + rnd(&mut st) % (p - 1);
        if seen.insert(x) {
            if let Some(q) = lift(x, p) {
                if q.y != 0 {
                    out.push(q)
                }
            }
        }
    }
    out
}
fn mul(mut k: u64, mut q: Pt, p: u64, ops: &mut u64) -> Pt {
    let mut out = O;
    while k > 0 {
        if k & 1 == 1 {
            out = add(out, q, p, ops)
        }
        k >>= 1;
        if k > 0 {
            q = add(q, q, p, ops)
        }
    }
    out
}
fn plus(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn minus(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn velu_scan(
    base: &[Pt],
    p: u64,
    t: Pt,
    h: u64,
    root_order: u64,
) -> (u64, u64, usize, usize, usize, usize, usize, u64, u64) {
    assert_eq!(h, 7, "the rational x-formula below uses three kernel pairs");
    let a = p - 3;
    let b = 40;
    let mut kernel = Vec::new();
    let mut cur = O;
    let mut prep = 0;
    for _ in 1..h {
        cur = add(cur, t, p, &mut prep);
        assert!(!cur.o);
        kernel.push(cur)
    }
    let mut st = 0;
    let mut sw = 0;
    for q in &kernel {
        let x2 = mm(q.x, q.x, p);
        let x3 = mm(x2, q.x, p);
        st = plus(st, plus(mm(3, x2, p), a, p), p);
        sw = plus(
            sw,
            plus(
                plus(mm(5, x3, p), mm(3, mm(a, q.x, p), p), p),
                mm(2, b, p),
                p,
            ),
            p,
        );
    }
    let aq = minus(a, mm(5, st, p), p);
    let bq = minus(b, mm(7, sw, p), p);
    let mut adds = 0;
    let mut hits = 0;
    let mut xs = HashSet::new();
    let mut cosets: HashMap<u64, usize> = HashMap::new();
    let mut zero_x = 0;
    let mut corrections = 0;
    for &point in base {
        let mut xi = point.x;
        let mut yi = point.y;
        for &q in &kernel {
            let out = add(point, q, p, &mut adds);
            assert!(!out.o, "base point lies in torsion kernel");
            xi = plus(xi, minus(out.x, q.x, p), p);
            yi = plus(yi, minus(out.y, q.y, p), p);
            corrections += 1;
        }
        let mut rational_x = point.x;
        for q in kernel.iter().take(3) {
            let denominator = mm(minus(point.x, q.x, p), minus(point.x, q.x, p), p);
            assert_ne!(denominator, 0);
            let u = minus(
                mm(
                    plus(mm(point.y, point.y, p), mm(q.y, q.y, p), p),
                    pw(denominator, p - 2, p),
                    p,
                ),
                plus(point.x, q.x, p),
                p,
            );
            rational_x = plus(rational_x, mm(2, minus(u, q.x, p), p), p);
        }
        assert_eq!(rational_x, xi, "rational and direct Vélu x disagree");
        let lhs = mm(yi, yi, p);
        let rhs = plus(plus(mm(mm(xi, xi, p), xi, p), mm(aq, xi, p), p), bq, p);
        assert_eq!(lhs, rhs, "Vélu image misses quotient curve");
        if xi == 0 {
            zero_x += 1;
        } else {
            let label = pw(xi, root_order, p);
            *cosets.entry(label).or_default() += 1;
            if label == 1 {
                hits += 1;
            }
        }
        xs.insert(xi);
    }
    assert_eq!(corrections, (base.len() as u64) * (h - 1));
    let max_coset = cosets.values().copied().max().unwrap_or(0);
    (
        aq,
        bq,
        hits,
        xs.len(),
        cosets.len(),
        max_coset,
        zero_x,
        prep + adds,
        corrections,
    )
}
fn exact_torsion(t: Pt, h: u64, p: u64) -> bool {
    if t.o || !mul(h, t, p, &mut 0).o {
        return false;
    }
    let mut v = h;
    let mut d = 2;
    while d * d <= v {
        if v.is_multiple_of(d) {
            if mul(h / d, t, p, &mut 0).o {
                return false;
            }
            while v.is_multiple_of(d) {
                v /= d
            }
        }
        d += 1
    }
    !(v > 1 && mul(h / v, t, p, &mut 0).o)
}
fn find_t(p: u64, r: u64, h: u64, known: Option<Pt>) -> (Pt, Pt, u64) {
    let mut ops = 0;
    if let Some(q) = known {
        assert_eq!(
            mm(q.y, q.y, p),
            (mm(mm(q.x, q.x, p), q.x, p) + p - mm(3, q.x, p) + 40) % p
        );
        let t = mul(r, q, p, &mut ops);
        assert!(exact_torsion(t, h, p));
        return (q, t, ops);
    }
    for x in 0..p {
        if let Some(q) = lift(x, p) {
            let t = mul(r, q, p, &mut ops);
            if exact_torsion(t, h, p) {
                return (q, t, ops);
            }
        }
    }
    panic!("no full cofactor torsion point found")
}
#[allow(dead_code)]
#[derive(Debug)]
struct Hits {
    directed: usize,
    cross_directed: usize,
    self_hits: usize,
    unordered_cross_pairs: usize,
    distinct_sources: usize,
    distinct_endpoints: usize,
    point_hits: usize,
    translation_adds: u64,
    example: Vec<(usize, usize, u64, Pt)>,
}
fn scan(base: &[Pt], p: u64, h: u64, t: Pt) -> Hits {
    let xmap: HashMap<u64, usize> = base.iter().enumerate().map(|(i, q)| (q.x, i)).collect();
    assert_eq!(xmap.len(), base.len());
    let mut multiples = Vec::new();
    let mut cur = O;
    let mut prep = 0;
    for _ in 1..h {
        cur = add(cur, t, p, &mut prep);
        multiples.push(cur)
    }
    assert_eq!(add(cur, t, p, &mut 0), O);
    let mut directed = 0;
    let mut cross_directed = 0;
    let mut self_hits = 0;
    let mut cross_pairs = HashSet::new();
    let mut point_hits = 0;
    let mut sources = HashSet::new();
    let mut endpoints = HashSet::new();
    let mut example = Vec::new();
    let mut adds = 0;
    for (i, &q) in base.iter().enumerate() {
        for (j, &tj) in multiples.iter().enumerate() {
            let out = add(q, tj, p, &mut adds);
            if out.o {
                continue;
            }
            if let Some(&k) = xmap.get(&out.x) {
                directed += 1;
                sources.insert(i);
                endpoints.insert(k);
                if i == k {
                    self_hits += 1
                } else {
                    cross_directed += 1;
                    cross_pairs.insert((i.min(k), i.max(k)));
                }
                let hit = out == base[k] || out == neg(base[k], p);
                assert!(hit);
                point_hits += usize::from(hit);
                if example.len() < 8 {
                    example.push((i, k, (j + 1) as u64, out))
                }
            }
        }
    }
    Hits {
        directed,
        cross_directed,
        self_hits,
        unordered_cross_pairs: cross_pairs.len(),
        distinct_sources: sources.len(),
        distinct_endpoints: endpoints.len(),
        point_hits,
        translation_adds: adds + prep,
        example,
    }
}
fn main() {
    for (p, d, z, h, r, known) in [
        (
            (1u64 << 61) - 1,
            4950,
            636328216386749903,
            7,
            329406144237565159,
            Some(Pt {
                x: 0,
                y: 313662643971843112,
                o: false,
            }),
        ),
        (131071, 102, 37206, 37, 3541, None),
        (524287, 171, 477103, 32, 16421, None),
    ] {
        let (rpoint, t, t_ops) = find_t(p, r, h, known);
        assert!(!mul(h, rpoint, p, &mut 0).o);
        assert!(mul(h * r, rpoint, p, &mut 0).o);
        let root = roots(p, d, z);
        let result = scan(&root, p, h, t);
        let n = (h as u128) * (r as u128);
        let expected = (root.len() as f64) * ((h - 1) as f64) * (2 * root.len()) as f64 / n as f64;
        println!("p={p} h={h} r={r} R={rpoint:?} T={t:?} torsion_scalar_ops={t_ops} B={} root={result:?} uniform_expected_directed={expected:.9e}",root.len());
        if p == (1u64 << 61) - 1 {
            let velu = velu_scan(&root, p, t, h, d);
            assert_eq!((velu.4, velu.5, velu.6), (root.len(), 1, 0));
            println!("p={p} normalized_velu=(quotient_a,quotient_b,root_x_hits,distinct_image_x,distinct_nonzero_mu_cosets,max_coset_occupancy,zero_x,group_additions,coordinate_corrections) {velu:?}");
            let ctl = random_base(p, root.len(), 2026);
            let s = scan(&ctl, p, h, t);
            println!("p={p} random_control={s:?}");
            let control_velu = velu_scan(&ctl, p, t, h, d);
            println!("p={p} random_normalized_velu=(quotient_a,quotient_b,root_x_hits,distinct_image_x,distinct_nonzero_mu_cosets,max_coset_occupancy,zero_x,group_additions,coordinate_corrections) {control_velu:?}");
        }
        if p < (1u64 << 61) - 1 {
            let mut counts = Vec::new();
            let mut total_adds = 0;
            for seed in 0..16 {
                let ctl = random_base(p, root.len(), 2026 + seed);
                let s = scan(&ctl, p, h, t);
                counts.push(s.directed);
                total_adds += s.translation_adds;
            }
            println!("p={p} random_directed_16={counts:?} control_translation_adds={total_adds}");
        }
    }
}
