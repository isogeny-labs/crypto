//! Target-seeded anchor relations with coefficient-aware off-base elimination.
#[path = "compact.rs"]
mod compact;
#[path = "dense.rs"]
mod dense;
#[path = "orbit.rs"]
mod orbit;
use crate::table_factor::Tables;
use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::{collections::HashSet, time::Instant};
pub(crate) type Signed = Vec<(u32, i64)>;
fn mul(f: &Fp, a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else if a == 1 {
        b
    } else if a == f.p - 1 {
        f.neg(b)
    } else {
        f.mul(a, b)
    }
}
fn axpy(a: &Row, b: &Row, s: u64, f: &Fp) -> Row {
    if s == 0 {
        return a.clone();
    }
    let mut out = Row::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push(a[i]);
            i += 1;
        } else if i == a.len() || b[j].0 < a[i].0 {
            let v = mul(f, s, b[j].1);
            if v != 0 {
                out.push((b[j].0, v));
            }
            j += 1;
        } else {
            let v = f.add(a[i].1, mul(f, s, b[j].1));
            if v != 0 {
                out.push((a[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}
fn scale(a: &Row, s: u64, f: &Fp) -> Row {
    a.iter()
        .filter_map(|&(c, v)| {
            let v = mul(f, s, v);
            (v != 0).then_some((c, v))
        })
        .collect()
}
pub(crate) fn signed(mut r: Signed) -> Signed {
    r.sort_unstable_by_key(|e| e.0);
    let mut o: Signed = Vec::new();
    for (c, v) in r {
        if let Some((cc, vv)) = o.last_mut() {
            if *cc == c {
                *vv += v;
                continue;
            }
        }
        o.push((c, v));
    }
    o.retain(|e| e.1 != 0);
    o
}
pub(crate) fn modular(r: &Signed, f: &Fp) -> Row {
    r.iter()
        .filter_map(|&(c, v)| {
            let v = f.from_i64(v);
            (v != 0).then_some((c, v))
        })
        .collect()
}
pub(crate) fn sum_signed(c: &Curve, fb: &[(u64, u64)], r: &Signed) -> Div {
    let mut s = c.zero();
    for &(j, v) in r {
        let (x, y) = fb[j as usize];
        let d = c.mul(&c.place(x, y), v.unsigned_abs());
        s = c.add(&s, &if v < 0 { c.neg(&d) } else { d });
    }
    s
}
pub(crate) fn sum_mod(c: &Curve, points: &[Div], r: &Row) -> Div {
    let mut s = c.zero();
    for &(j, v) in r {
        s = c.add(&s, &c.mul(&points[j as usize], v));
    }
    s
}

/// L_u = weight[u] L_parent[u] + beta[u] dot anchors. Root0 is zero.
pub struct Forest {
    parent: Vec<usize>,
    size: Vec<usize>,
    weight: Vec<u64>,
    beta: Vec<Row>,
    pub unions: u64,
    pub pins: u64,
    f: Fp,
}
impl Forest {
    pub fn new(n: usize, ell: u64) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            weight: vec![1; n + 1],
            beta: vec![Row::new(); n + 1],
            unions: 0,
            pins: 0,
            f: Fp::new(ell),
        }
    }
    pub fn find(&mut self, u: usize) -> (usize, u64, Row) {
        let p = self.parent[u];
        if p == u {
            return (u, 1, Row::new());
        }
        let a = self.weight[u];
        let b = self.beta[u].clone();
        let (r, c, d) = self.find(p);
        let weight = mul(&self.f, a, c);
        let beta = axpy(&b, &d, a, &self.f);
        self.parent[u] = r;
        self.weight[u] = weight;
        self.beta[u] = beta.clone();
        (r, weight, beta)
    }
    /// Returns a pure-anchor row, or stores a certified endpoint relation.
    pub fn insert(&mut self, off: &[(usize, u64)], mut hot: Row) -> Option<Row> {
        let mut roots: Vec<(usize, u64)> = Vec::new();
        for &(node, c) in off {
            let (r, a, b) = self.find(node);
            hot = axpy(&hot, &b, c, &self.f);
            if r == 0 {
                continue;
            }
            let v = mul(&self.f, c, a);
            if let Some((_, old)) = roots.iter_mut().find(|e| e.0 == r) {
                *old = self.f.add(*old, v);
            } else {
                roots.push((r, v));
            }
        }
        roots.retain(|e| e.1 != 0);
        assert!(roots.len() <= 2);
        match roots.len() {
            0 => Some(hot),
            1 => {
                let (r, a) = roots[0];
                let s = self.f.neg(self.f.inv(a));
                self.parent[r] = 0;
                self.weight[r] = 0;
                self.beta[r] = scale(&hot, s, &self.f);
                self.pins += 1;
                self.unions += 1;
                None
            }
            _ => {
                if self.size[roots[0].0] > self.size[roots[1].0] {
                    roots.swap(0, 1);
                }
                let (u, a) = roots[0];
                let (v, b) = roots[1];
                let inv = self.f.inv(a);
                self.parent[u] = v;
                self.weight[u] = self.f.neg(mul(&self.f, b, inv));
                self.beta[u] = scale(&hot, self.f.neg(inv), &self.f);
                self.size[v] += self.size[u];
                self.unions += 1;
                None
            }
        }
    }
}
pub(crate) fn descend(
    c: &Curve,
    d: &Div,
    fb: &[(u64, u64)],
    by: &[u32],
    tab: &Tables,
    rng: &mut Rng,
    gate: bool,
    steps: &mut u64,
) -> Option<Signed> {
    let mut r = *d;
    let mut taken = vec![0i64; fb.len()];
    for t in 0..=10000 {
        if let Some(roots) = tab.split(&r.u) {
            let mut row = Signed::new();
            for &(x, m) in roots.as_slice() {
                let j = by[x as usize];
                let sign = if r.v.eval(x, &c.fp) == fb[j as usize].1 {
                    1
                } else {
                    -1
                };
                row.push((j, sign * m as i64));
            }
            for (j, &n) in taken.iter().enumerate() {
                if n != 0 {
                    row.push((j as u32, -n));
                }
            }
            let row = signed(row);
            if gate {
                assert_eq!(sum_signed(c, fb, &row), *d, "descent");
            }
            return Some(row);
        }
        if t == 10000 {
            break;
        }
        let j = rng.below(fb.len() as u64) as usize;
        taken[j] += 1;
        let (x, y) = fb[j];
        r = c.add_place(&r, x, y);
        *steps += 1;
    }
    None
}
pub(crate) fn projection(c: &Curve, ell: u64, fb: &[(u64, u64)]) -> Vec<Div> {
    let p = c.fp.p;
    let s = (p as f64).sqrt() as u64;
    let lo = (s.saturating_sub(1) as u128).pow((2 * c.g) as u32);
    let hi = (s as u128 + 2).pow((2 * c.g) as u32);
    assert!((ell as u128) * (ell as u128) > hi);
    let start = ((lo + ell as u128 - 1) / ell as u128).max(1);
    let end = hi / ell as u128;
    for h in start..=end {
        let n = u64::try_from(h * ell as u128).expect("bounded gate scalar");
        if fb
            .iter()
            .all(|&(x, y)| c.is_zero(&c.mul(&c.place(x, y), n)))
        {
            let h = h as u64;
            assert_ne!(h % ell, 0);
            let ps: Vec<_> = fb.iter().map(|&(x, y)| c.mul(&c.place(x, y), h)).collect();
            for d in &ps {
                assert!(c.is_zero(&c.mul(d, ell)));
            }
            return ps;
        }
    }
    panic!("projection certificate not found")
}
#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub m: usize,
    pub anchors: usize,
    pub draws: u64,
    pub unique_tuples: usize,
    pub splits: u64,
    pub admitted: u64,
    pub duplicate_relations: u64,
    pub pure_rows: usize,
    pub unions: u64,
    pub pins: u64,
    pub solves: u32,
    pub descent_steps: u64,
    pub raw_checks: u64,
    pub pure_checks: u64,
    pub memory_bytes: usize,
    pub setup_ns: u128,
    pub relation_ns: u128,
    pub forest_ns: u128,
    pub solve_ns: u128,
    pub total_ns: u128,
    pub solve: SolveStats,
}
pub fn solve(c: &Curve, d1: &Div, d2: &Div, ell: u64, seed: u64, gate: bool) -> Report {
    solve_variant(c, d1, d2, ell, seed, gate, false, false)
}

/// Integration gate for orbit reuse, dense forest and incremental elimination.
/// Tuple scheduling and inverse caching are still pending; not a timing arm.
pub fn solve_orbit(c: &Curve, d1: &Div, d2: &Div, ell: u64, seed: u64, gate: bool) -> Report {
    solve_variant(c, d1, d2, ell, seed, gate, true, false)
}

pub fn solve_compact(c: &Curve, d1: &Div, d2: &Div, ell: u64, seed: u64, gate: bool) -> Report {
    solve_variant(c, d1, d2, ell, seed, gate, true, true)
}

fn solve_variant(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    seed: u64,
    gate: bool,
    use_orbit: bool,
    use_compact: bool,
) -> Report {
    let all = Instant::now();
    let mut r = Report::default();
    let fp = &c.fp;
    let f = Fp::new(ell);
    let mut rng = Rng::new(seed);
    let tab = Tables::for_degree(fp.p, c.g);
    let fb = tab.factor_base(c);
    let m = fb.len();
    r.m = m;
    let s = (fp.p as f64).sqrt() as u128 + 2;
    assert!((ell as u128) * (ell as u128) > s.pow((2 * c.g) as u32));
    let mut by = vec![u32::MAX; fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by[x as usize] = j as u32;
    }
    let mut inv = vec![0u64; if use_orbit { 0 } else { fp.p as usize }];
    if !use_orbit {
        inv[1] = 1;
        for a in 2..fp.p {
            inv[a as usize] = fp.mul(fp.p - fp.p / a, inv[(fp.p % a) as usize]);
        }
    }
    let Some(a) = descend(c, d1, &fb, &by, &tab, &mut rng, gate, &mut r.descent_steps) else {
        r.total_ns = all.elapsed().as_nanos();
        return r;
    };
    let Some(b) = descend(c, d2, &fb, &by, &tab, &mut rng, gate, &mut r.descent_steps) else {
        r.total_ns = all.elapsed().as_nanos();
        return r;
    };
    let mut hot = vec![false; m];
    for &(j, _) in a.iter().chain(b.iter()) {
        hot[j as usize] = true;
    }
    let desired = match c.g {
        2 => (2.0 * (fp.p as f64).cbrt()).ceil() as usize,
        3 => (fp.p as f64).powf(2.0 / 3.0).ceil() as usize,
        _ => (fp.p as f64).powf(0.75).ceil() as usize,
    }
    .max(c.g + 1)
    .max(hot.iter().filter(|&&v| v).count())
    .min(m);
    let mut rest: Vec<_> = (0..m).filter(|&j| !hot[j]).collect();
    let needed = desired - hot.iter().filter(|&&v| v).count();
    for j in 0..needed {
        let k = j + rng.below((rest.len() - j) as u64) as usize;
        rest.swap(j, k);
        hot[rest[j]] = true;
    }
    let anchor: Vec<_> = (0..m).filter(|&j| hot[j]).collect();
    let h = anchor.len();
    let inverse_differences = if use_orbit {
        orbit::inverse_differences(&anchor.iter().map(|&j| fb[j].0).collect::<Vec<_>>(), fp)
    } else {
        Vec::new()
    };
    let mut triples = Vec::new();
    if use_orbit && c.g == 2 {
        for a in 0..h {
            for b in a + 1..h {
                for d in b + 1..h {
                    triples.push([a, b, d]);
                }
            }
        }
        for j in (1..triples.len()).rev() {
            let k = rng.below((j + 1) as u64) as usize;
            triples.swap(j, k);
        }
    }
    r.anchors = h;
    let mut col = vec![u32::MAX; m];
    for (j, &global) in anchor.iter().enumerate() {
        col[global] = j as u32;
    }
    let map = |raw: &Signed| -> Row {
        let raw: Signed = raw.iter().map(|&(j, v)| (col[j as usize], v)).collect();
        let raw = signed(raw);
        modular(&raw, &f)
    };
    let mut target = map(&b);
    target.push((h as u32, ell - 1));
    let mut rows = vec![(map(&a), 1), (target, 0)];
    let mut echelon = dense::Echelon::new(h, ell);
    if use_orbit {
        for (row, rhs) in &rows {
            echelon.insert(row, *rhs);
        }
    }
    let mut wanted = h + 8;
    let projected = if gate {
        let ps = projection(c, ell, &fb);
        Some(anchor.iter().map(|&j| ps[j]).collect::<Vec<_>>())
    } else {
        None
    };
    let mut forest = Forest::new(m, ell);
    let mut dense_forest = use_orbit.then(|| dense::DenseForest::new(m, h, ell));
    let mut tuples: HashSet<Vec<(u32, bool)>> = HashSet::new();
    let mut relations: HashSet<Row> = HashSet::new();
    let mut pure: HashSet<Row> = HashSet::new();
    r.memory_bytes = tab.bytes() + inv.len() * 8;
    r.memory_bytes +=
        inverse_differences.len() * 8 + triples.capacity() * std::mem::size_of::<[usize; 3]>();
    if let Some(forest) = &dense_forest {
        r.memory_bytes += forest.bytes();
    }
    r.setup_ns = all.elapsed().as_nanos();
    let mut pending = Vec::new();
    while r.draws < 1_000_000 && (use_orbit || r.solves < 16) {
        r.draws += 1;
        let clock = Instant::now();
        let (chosen, hp, q) = if use_orbit {
            if pending.is_empty() {
                let mut chosen: Vec<(u32, bool)> = Vec::with_capacity(c.g + 1);
                if c.g == 2 {
                    let Some(tuple) = triples.pop() else {
                        break;
                    };
                    chosen.extend(tuple.iter().map(|&j| (anchor[j] as u32, false)));
                }
                while chosen.len() < c.g + 1 {
                    let j = anchor[rng.below(h as u64) as usize] as u32;
                    if !chosen.iter().any(|e| e.0 == j) {
                        chosen.push((j, false));
                    }
                }
                chosen.sort_unstable_by_key(|e| e.0);
                if !tuples.insert(chosen.clone()) {
                    r.relation_ns += clock.elapsed().as_nanos();
                    continue;
                }
                let points: Vec<_> = chosen.iter().map(|&(j, _)| fb[j as usize]).collect();
                let weights: Vec<_> = chosen
                    .iter()
                    .enumerate()
                    .map(|(i, &(id, _))| {
                        chosen.iter().enumerate().filter(|&(j, _)| i != j).fold(
                            fb[id as usize].1,
                            |w, (_, &(other, _))| {
                                fp.mul(
                                    w,
                                    inverse_differences[col[id as usize] as usize * h
                                        + col[other as usize] as usize],
                                )
                            },
                        )
                    })
                    .collect();
                let reference;
                let compact;
                let pairs = if use_compact {
                    compact = compact::signed_orbit(c, &points, &weights);
                    compact.as_slice()
                } else {
                    reference = orbit::signed_orbit_with_weights(c, &points, &weights);
                    reference.as_slice()
                };
                if use_compact && gate {
                    assert_eq!(
                        pairs,
                        orbit::signed_orbit_with_weights(c, &points, &weights).as_slice(),
                        "complete generated orbit gate"
                    );
                }
                for (step, (hp, q)) in pairs.iter().copied().enumerate().rev() {
                    let gray = step ^ (step >> 1);
                    let signed = chosen
                        .iter()
                        .enumerate()
                        .map(|(j, &(id, _))| (id, j > 0 && gray & (1 << (j - 1)) != 0))
                        .collect::<Vec<_>>();
                    pending.push((signed, hp, q));
                }
            }
            pending.pop().unwrap()
        } else {
            let mut chosen: Vec<(u32, bool)> = Vec::with_capacity(c.g + 1);
            while chosen.len() < c.g + 1 {
                let j = anchor[rng.below(h as u64) as usize] as u32;
                if chosen.iter().any(|e| e.0 == j) {
                    continue;
                }
                chosen.push((j, false));
            }
            chosen.sort_unstable_by_key(|e| e.0);
            for p in &mut chosen[1..] {
                p.1 = rng.below(2) == 1;
            }
            if !tuples.insert(chosen.clone()) {
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            }
            let (mut hp, mut ap) = (Poly::zero(), Poly::one());
            for &(j, neg) in &chosen {
                let (x, y) = fb[j as usize];
                let y = if neg { fp.neg(y) } else { y };
                let den = ap.eval(x, fp);
                assert_ne!(den, 0);
                let k = fp.mul(fp.sub(y, hp.eval(x, fp)), inv[den as usize]);
                hp = hp.add(&ap.scale(k, fp), fp);
                ap = ap.mul(&Poly::x_minus(x, fp), fp);
            }
            let (q, rem) = c.f.sub(&hp.mul(&hp, fp), fp).divrem(&ap, fp);
            assert!(rem.is_zero());
            (chosen, hp, q)
        };
        let Some(roots) = tab.split(&q) else {
            r.relation_ns += clock.elapsed().as_nanos();
            continue;
        };
        r.splits += 1;
        if roots
            .as_slice()
            .iter()
            .filter(|&&(x, _)| !hot[by[x as usize] as usize])
            .count()
            > 2
        {
            r.relation_ns += clock.elapsed().as_nanos();
            continue;
        }
        let mut raw: Signed = chosen
            .iter()
            .map(|&(j, n)| (j, if n { -1 } else { 1 }))
            .collect();
        for &(x, mult) in roots.as_slice() {
            let j = by[x as usize];
            assert_ne!(j, u32::MAX);
            let sign = if hp.eval(x, fp) == fb[j as usize].1 {
                1
            } else {
                -1
            };
            raw.push((j, sign * mult as i64));
        }
        let raw = signed(raw);
        if raw.is_empty() {
            r.relation_ns += clock.elapsed().as_nanos();
            continue;
        }
        let mut row = modular(&raw, &f);
        let first = row[0].1;
        let norm = if first == 1 {
            1
        } else if first == ell - 1 {
            ell - 1
        } else {
            f.inv(first)
        };
        row = scale(&row, norm, &f);
        if !use_orbit && !relations.insert(row.clone()) {
            r.duplicate_relations += 1;
            r.relation_ns += clock.elapsed().as_nanos();
            continue;
        }
        r.admitted += 1;
        r.relation_ns += clock.elapsed().as_nanos();
        if gate {
            assert_eq!(sum_signed(c, &fb, &raw), c.zero(), "principal");
            r.raw_checks += 1;
        }
        let clock = Instant::now();
        let mut off = Vec::new();
        let mut on = Row::new();
        for (j, v) in row {
            if hot[j as usize] {
                on.push((col[j as usize], v));
            } else {
                off.push((j as usize + 1, v));
            }
        }
        on.sort_unstable();
        let reduced = if let Some(forest) = &mut dense_forest {
            forest.insert(&off, &on)
        } else {
            forest.insert(&off, on)
        };
        r.forest_ns += clock.elapsed().as_nanos();
        let Some(mut row) = reduced else {
            continue;
        };
        if row.is_empty() {
            continue;
        }
        row = scale(&row, f.inv(row[0].1), &f);
        if !pure.insert(row.clone()) {
            continue;
        }
        if let Some(ps) = &projected {
            assert_eq!(sum_mod(c, ps, &row), c.zero(), "pure anchor relation");
            r.pure_checks += 1;
        }
        rows.push((row, 0));
        r.pure_rows = rows.len() - 2;
        if use_orbit {
            let t = Instant::now();
            r.solves += 1;
            let k = echelon.insert(&rows.last().unwrap().0, 0);
            r.solve_ns += t.elapsed().as_nanos();
            if let Some(k) = k {
                assert_eq!(c.mul(d1, k), *d2, "orbit target verification");
                r.k = Some(k);
                break;
            }
            continue;
        }
        if rows.len() >= wanted {
            let t = Instant::now();
            r.solves += 1;
            let k = solve_fast(&rows, h, ell, &mut r.solve);
            r.solve_ns += t.elapsed().as_nanos();
            if let Some(k) = k {
                if c.mul(d1, k) == *d2 {
                    r.k = Some(k);
                    break;
                }
            }
            wanted += 8 + h / 16;
        }
    }
    r.unions = dense_forest.as_ref().map_or(forest.unions, |f| f.unions);
    r.pins = dense_forest.as_ref().map_or(forest.pins, |f| f.pins);
    r.unique_tuples = tuples.len();
    r.total_ns = all.elapsed().as_nanos();
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    fn eval(r: &Row, x: &[u64], f: &Fp) -> u64 {
        r.iter()
            .fold(0, |s, &(j, v)| f.add(s, f.mul(v, x[j as usize])))
    }
    #[test]
    fn planted_forest_invariants() {
        let f = Fp::new(101);
        let hot = [7, 29, 33];
        let values = [0, 13, 21, 44, 53, 87];
        let mut forest = Forest::new(5, 101);
        let mut rng = Rng::new(7654);
        for _ in 0..200 {
            let u = 1 + rng.below(5) as usize;
            let v = 1 + rng.below(5) as usize;
            let a = 1 + rng.below(100);
            let b = 1 + rng.below(100);
            let rhs = f.neg(f.add(f.mul(a, values[u]), f.mul(b, values[v])));
            let h = vec![(0, f.mul(rhs, f.inv(hot[0])))];
            if let Some(row) = forest.insert(&[(u, a), (v, b)], h) {
                assert_eq!(eval(&row, &hot, &f), 0);
            }
            for j in 1..=5 {
                let (root, a, b) = forest.find(j);
                assert_eq!(values[j], f.add(f.mul(a, values[root]), eval(&b, &hot, &f)));
            }
        }
    }
    #[test]
    fn checked_anchor_solves() {
        for (g, p) in [(2, 101), (2, 251), (3, 61), (3, 211), (4, 31), (4, 61)] {
            let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
            for seed in 0..2 {
                let k = 113 + seed * 23;
                let target = i.curve.mul(&i.d1, k);
                let r = solve(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, true);
                assert_eq!(r.k, Some(k), "g={g} p={p} seed={seed} {r:?}");
                assert_eq!(r.raw_checks, r.admitted);
                assert_eq!(r.pure_checks, r.pure_rows as u64);
            }
        }
    }

    #[test]
    fn checked_orbit_anchor_solves() {
        for (g, p) in [(2, 101), (2, 251), (3, 61), (3, 211), (4, 31), (4, 61)] {
            let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
            for seed in 0..2 {
                let k = 113 + seed * 23;
                let target = i.curve.mul(&i.d1, k);
                let r = solve_orbit(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, true);
                assert_eq!(r.k, Some(k), "g={g} p={p} seed={seed} {r:?}");
                assert_eq!(r.raw_checks, r.admitted);
                assert_eq!(r.pure_checks, r.pure_rows as u64);
                let compact = solve_compact(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, true);
                assert_eq!(compact.k, r.k);
                assert_eq!(
                    (
                        compact.draws,
                        compact.admitted,
                        compact.pure_rows,
                        compact.solves
                    ),
                    (r.draws, r.admitted, r.pure_rows, r.solves)
                );
            }
        }
    }
}
