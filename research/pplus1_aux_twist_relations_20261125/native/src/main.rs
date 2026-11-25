use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

const P: u64 = 127;
const R: u64 = 131;
const KNOWN_K: u64 = 37;
const QUERIES: usize = 1024;
type Point = Option<(u64, u64)>;

#[derive(Default)]
struct Curve {
    a: u64,
    b: u64,
    ops: Cell<u64>,
}

fn modp(x: i64) -> u64 {
    x.rem_euclid(P as i64) as u64
}

fn power(mut a: u64, mut n: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 != 0 { z = z * a % P; }
        a = a * a % P;
        n >>= 1;
    }
    z
}

impl Curve {
    fn new(a: u64, b: u64) -> Self {
        let (a, b) = (a % P, b % P);
        assert_ne!((4 * a * a * a + 27 * b * b) % P, 0);
        Self { a, b, ops: Cell::new(0) }
    }
    fn rhs(&self, x: u64) -> u64 { (x * x * x + self.a * x + self.b) % P }
    fn points(&self) -> Vec<Point> {
        let mut out = vec![None];
        for x in 0..P {
            for y in 0..P {
                if y * y % P == self.rhs(x) { out.push(Some((x, y))); }
            }
        }
        out
    }
    fn neg(q: Point) -> Point { q.map(|(x, y)| (x, (P - y) % P)) }
    fn add(&self, q: Point, z: Point) -> Point {
        self.ops.set(self.ops.get() + 1);
        let (Some((x1, y1)), Some((x2, y2))) = (q, z) else { return q.or(z) };
        if x1 == x2 && (y1 + y2) % P == 0 { return None; }
        let slope = if x1 == x2 {
            (3 * x1 * x1 + self.a) % P * power(2 * y1 % P, P - 2) % P
        } else {
            modp(y2 as i64 - y1 as i64) * power(modp(x2 as i64 - x1 as i64), P - 2) % P
        };
        let x = modp(slope as i64 * slope as i64 - x1 as i64 - x2 as i64);
        let y = modp(slope as i64 * (x1 as i64 - x as i64) - y1 as i64);
        Some((x, y))
    }
    fn mul(&self, mut n: u64, mut q: Point) -> Point {
        let mut z = None;
        while n > 0 {
            if n & 1 != 0 { z = self.add(z, q); }
            q = self.add(q, q);
            n >>= 1;
        }
        z
    }
    fn order(&self, q: Point) -> u64 {
        let mut z = None;
        for n in 1..=300 {
            z = self.add(z, q);
            if z.is_none() { return n; }
        }
        panic!("order cap")
    }
}

fn lift_x(e: &Curve, xs: &[u64]) -> Vec<u64> {
    let mut found = BTreeSet::new();
    for &x in xs {
        if (0..P).any(|y| y * y % P == e.rhs(x)) { found.insert(x); }
    }
    found.into_iter().collect()
}

fn fiber(e: &Curve, n: u64, h_order: u64) -> Vec<u64> {
    let pts = e.points();
    assert_eq!(pts.len() as u64, n);
    let h = pts.iter().copied().flatten().find_map(|q| {
        let candidate = e.mul(n / h_order, Some(q));
        (e.order(candidate) == h_order).then_some(candidate)
    }).expect("smooth subgroup point");
    let subgroup: Vec<_> = (0..h_order).map(|j| e.mul(j, h)).collect();
    let base = pts.iter().copied().flatten().find(|&q| !subgroup.contains(&e.mul(2, Some(q)))).expect("coset");
    let xs: Vec<_> = subgroup.iter().map(|&q| e.add(Some(base), q).unwrap().0).collect();
    assert_eq!(xs.iter().copied().collect::<BTreeSet<_>>().len(), h_order as usize);
    xs
}

type F2 = (u64, u64);
fn f2mul(x: F2, y: F2) -> F2 { ((x.0 * y.0 + P - x.1 * y.1 % P) % P, (x.0 * y.1 + x.1 * y.0) % P) }
fn f2pow(mut x: F2, mut n: u64) -> F2 {
    let mut z = (1, 0);
    while n > 0 {
        if n & 1 != 0 { z = f2mul(z, x); }
        x = f2mul(x, x);
        n >>= 1;
    }
    z
}
fn torus_fiber() -> Vec<u64> {
    let g = (0..P).flat_map(|a| (0..P).map(move |b| (a, b))).find(|&(a, b)|
        (a * a + b * b) % P == 1 && f2pow((a, b), 64) != (1, 0)).expect("torus generator");
    assert_eq!(f2pow(g, 128), (1, 0));
    let h = f2pow(g, 16);
    let xs: Vec<_> = (0..8).map(|j| 2 * f2mul(g, f2pow(h, j)).0 % P).collect();
    assert_eq!(xs.iter().copied().collect::<BTreeSet<_>>().len(), 8);
    let c = 2 * f2pow(g, 8).0 % P;
    assert_ne!(c, 2);
    assert_ne!(c, P - 2);
    for x in 0..P {
        let (mut u, mut v) = (2, x);
        for _ in 2..=8 { (u, v) = (v, (x * v + P - u) % P); }
        assert_eq!(xs.contains(&x), v == c);
    }
    xs
}

fn signed_base(e: &Curve, xs: &[u64]) -> (Vec<Point>, Vec<u64>) {
    let selected: Vec<_> = lift_x(e, xs).into_iter().take(2).collect();
    if selected.len() < 2 { return (vec![], selected); }
    let mut points = Vec::new();
    for &x in &selected {
        let y = (0..P).find(|&y| y * y % P == e.rhs(x)).unwrap();
        assert_ne!(y, 0);
        points.push(Some((x, y)));
        points.push(Some((x, (P - y) % P)));
    }
    (points, selected)
}

fn rank(rows: &[[u64; 3]]) -> usize {
    let mut a = rows.to_vec();
    let mut pivot = 0;
    for col in 0..3 {
        let Some(src) = (pivot..a.len()).find(|&j| a[j][col] != 0) else { continue };
        a.swap(pivot, src);
        let inv = modr_pow(a[pivot][col], R - 2);
        for k in 0..3 { a[pivot][k] = a[pivot][k] * inv % R; }
        for j in 0..a.len() {
            if j == pivot { continue; }
            let f = a[j][col];
            for k in 0..3 { a[j][k] = (a[j][k] + R - f * a[pivot][k] % R) % R; }
        }
        pivot += 1;
    }
    pivot
}
fn modr_pow(mut x: u64, mut n: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 != 0 { z = z * x % R; }
        x = x * x % R;
        n >>= 1;
    }
    z
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn scalar(&mut self) -> u64 { self.next() % R }
}

fn rho(e: &Curve, p: Point, q: Point, seed: u64) -> (usize, u64, Option<u64>) {
    let before = e.ops.get();
    let mut rng = Rng(seed);
    let (a, b) = (rng.scalar(), rng.scalar());
    let start = (e.add(e.mul(a, p), e.mul(b, q)), a, b);
    let step = |(z, x, y): (Point, u64, u64)| -> (Point, u64, u64) {
        match z.map(|pt| pt.0 % 3).unwrap_or(0) {
            0 => (e.add(z, p), (x + 1) % R, y),
            1 => (e.add(z, q), x, (y + 1) % R),
            _ => (e.add(z, z), 2 * x % R, 2 * y % R),
        }
    };
    let mut slow = step(start);
    let mut fast = step(step(start));
    for i in 1..=(10 * R as usize) {
        if slow.0 == fast.0 {
            let den = (slow.2 + R - fast.2) % R;
            let k = (den != 0).then(|| (fast.1 + R - slow.1) * modr_pow(den, R - 2) % R);
            let ops = e.ops.get() - before;
            return (i, ops, k.filter(|&k| e.mul(k, p) == q));
        }
        slow = step(slow);
        fast = step(step(fast));
    }
    (10 * R as usize, e.ops.get() - before, None)
}

fn main() {
    let out = env::args().nth(1).expect("usage: prime-aux-relations-panel OUT_DIR");
    fs::create_dir_all(&out).unwrap();
    let e = Curve::new(1, 26);
    assert_eq!(e.points().len(), R as usize);
    let p = e.points()[1];
    assert_eq!(e.mul(R, p), None);
    let q = e.mul(KNOWN_K, p);
    let mut small_x = Vec::new();
    for x in 0..P {
        if (0..P).any(|y| y * y % P == e.rhs(x)) { small_x.push(x); }
        if small_x.len() == 2 { break; }
    }
    let d = (2..P).find(|&v| power(v, (P - 1) / 2) == P - 1).unwrap();
    let tw = Curve::new(d * d, d * d * d * 26);
    let tw_start = Instant::now();
    let tw_x = fiber(&tw, 125, 5);
    let normalized: Vec<_> = tw_x.iter().map(|&x| x * power(d, P - 2) % P).collect();
    assert!(lift_x(&e, &normalized).is_empty());
    let tw_setup_ns = tw_start.elapsed().as_nanos();
    let shift = (1..P).find(|&c| lift_x(&e, &normalized.iter().map(|&x| (x + c) % P).collect::<Vec<_>>()).len() >= 2).unwrap();
    let aux = Curve::new(1, 0);
    let aux_start = Instant::now();
    let aux_x = fiber(&aux, 128, 8);
    let aux_setup_ns = aux_start.elapsed().as_nanos();
    let tor_start = Instant::now();
    let tor_x = torus_fiber();
    let tor_setup_ns = tor_start.elapsed().as_nanos();
    let arms = [
        ("small_x", small_x, 0, 0),
        ("pplus1_torus", tor_x, tor_setup_ns, 0),
        ("unrelated_aux", aux_x, aux_setup_ns, aux.ops.get()),
        ("twist_raw", normalized.clone(), tw_setup_ns, tw.ops.get()),
        ("twist_shifted", normalized.iter().map(|&x| (x + shift) % P).collect(), tw_setup_ns, tw.ops.get()),
    ];
    let mut rng = Rng(20261125);
    let stream: Vec<_> = (0..QUERIES).map(|_| (rng.scalar(), rng.scalar())).collect();
    let mut stream_txt = String::from("query,a,b\n");
    for (i, &(a, b)) in stream.iter().enumerate() { stream_txt.push_str(&format!("{},{},{}\n", i + 1, a, b)); }
    fs::write(Path::new(&out).join("stream.csv"), stream_txt).unwrap();
    let mut summary = String::from("arm,fiber_size,target_lift_x,columns,queries,hit_queries,verified_rows,rank,target_ops,aux_ops,setup_wall_ns,relation_wall_ns,status\n");
    let mut records = String::from("arm,query,a,b,i,j,r0,r1,r2,rhs\n");
    let mut bases = String::from("arm,index,x,y\n");
    for (name, xs, setup_ns, aux_ops) in arms {
        let start = Instant::now();
        let op0 = e.ops.get();
        let lifted = lift_x(&e, &xs);
        let (fb, selected) = signed_base(&e, &xs);
        for (i, point) in fb.iter().enumerate() {
            let (x, y) = point.unwrap();
            bases.push_str(&format!("{name},{i},{x},{y}\n"));
        }
        let mut seen = BTreeMap::<Point, Vec<(usize, usize)>>::new();
        for (i, &x) in fb.iter().enumerate() {
            for (j, &y) in fb.iter().enumerate() {
                seen.entry(e.add(x, y)).or_default().push((i, j));
            }
        }
        let (mut rows, mut hit_queries, mut count, mut used) = (Vec::new(), 0, 0, 0);
        if fb.len() == 4 {
            for (qi, &(a, b)) in stream.iter().enumerate() {
                let z = e.add(e.mul(a, p), e.mul(b, q));
                if let Some(pairs) = seen.get(&z) {
                    hit_queries += 1;
                    for &(i, j) in pairs {
                        assert_eq!(e.add(fb[i], fb[j]), z);
                        let mut row = [b, 0, 0];
                        for &index in &[i, j] {
                            let coef = if index % 2 == 0 { R - 1 } else { 1 };
                            row[1 + index / 2] = (row[1 + index / 2] + coef) % R;
                        }
                        let rhs = (R - a) % R;
                        records.push_str(&format!("{name},{},{a},{b},{i},{j},{},{},{},{rhs}\n", qi + 1, row[0], row[1], row[2]));
                        rows.push(row); count += 1;
                    }
                }
                if rank(&rows) == 3 { used = qi + 1; break; }
            }
            if used == 0 { used = QUERIES; }
        }
        let relation_ns = start.elapsed().as_nanos();
        let target_ops = e.ops.get() - op0;
        let status = if fb.len() < 4 { "underfilled" } else if rank(&rows) == 3 { "full_rank" } else { "censored" };
        summary.push_str(&format!("{name},{},{},{},{used},{hit_queries},{count},{},{target_ops},{aux_ops},{setup_ns},{relation_ns},{status}\n", xs.len(), lifted.len(), selected.len(), rank(&rows)));
        // Keep the validated relation source distinct from the priced query path.
        assert!(rows.iter().all(|row| row.iter().all(|&v| v < R)));
    }
    fs::write(Path::new(&out).join("summary.csv"), summary).unwrap();
    fs::write(Path::new(&out).join("relations.csv"), records).unwrap();
    fs::write(Path::new(&out).join("bases.csv"), bases).unwrap();
    let mut rhos = String::from("seed,steps,group_ops,recovered_k\n");
    for i in 0..16 {
        let (steps, ops, k) = rho(&e, p, q, 20261125 + i);
        assert_eq!(k, Some(KNOWN_K));
        rhos.push_str(&format!("{},{},{},{}\n", 20261125 + i, steps, ops, k.unwrap()));
    }
    fs::write(Path::new(&out).join("rho.csv"), rhos).unwrap();
    fs::write(Path::new(&out).join("fixture.txt"), format!("p={P}\na=1\nb=26\nr={R}\nP={p:?}\nQ={q:?}\nd={d}\ntwist_shift={shift}\nseed=20261125\n" )).unwrap();
    println!("native panel complete: {out}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_and_fibers() {
        let e = Curve::new(1, 26);
        assert_eq!(e.points().len(), R as usize);
        let g = e.points()[1];
        assert_eq!(e.mul(R, g), None);
        assert_eq!(torus_fiber().len(), 8);
        assert_eq!(fiber(&Curve::new(1, 0), 128, 8).len(), 8);
    }
}
