//! Exact nontrivial signed pair-sum energy on toy fields; sampled M61 control.
//! Run: cargo run --release --example root_pair_energy
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
        assert!(seen.insert(x), "root generator has order below d");
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
#[derive(Default, Debug)]
struct Stats {
    pairs: u64,
    finite: u64,
    distinct: usize,
    collisions: u64,
    disjoint: u64,
    max_bucket: usize,
    adds: u64,
}
fn energy(base: &[Pt], p: u64) -> Stats {
    let mut buckets: HashMap<Pt, Vec<(usize, usize)>> = HashMap::new();
    let mut s = Stats::default();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for si in 0..2 {
                for sj in 0..2 {
                    let u = if si == 0 { base[i] } else { neg(base[i], p) };
                    let v = if sj == 0 { base[j] } else { neg(base[j], p) };
                    let q = add(u, v, p, &mut s.adds);
                    s.pairs += 1;
                    if q.o {
                        continue;
                    }
                    s.finite += 1;
                    let bucket = buckets.entry(q).or_default();
                    s.collisions += bucket.len() as u64;
                    s.disjoint += bucket
                        .iter()
                        .filter(|&&(a, b)| a != i && a != j && b != i && b != j)
                        .count() as u64;
                    bucket.push((i, j));
                    s.max_bucket = s.max_bucket.max(bucket.len());
                }
            }
        }
    }
    s.distinct = buckets.len();
    s
}
fn sampled(base: &[Pt], p: u64, seed: u64, count: usize) -> (usize, u64, u64, u64) {
    let mut st = seed;
    let mut sums = HashSet::new();
    let mut inputs = HashSet::new();
    let mut adds = 0;
    let mut collisions = 0;
    for _ in 0..count {
        let i = (rnd(&mut st) as usize) % base.len();
        let mut j = (rnd(&mut st) as usize) % (base.len() - 1);
        if j >= i {
            j += 1
        }
        let si = rnd(&mut st) & 1;
        let sj = rnd(&mut st) & 1;
        let u = if si == 0 { base[i] } else { neg(base[i], p) };
        let v = if sj == 0 { base[j] } else { neg(base[j], p) };
        let q = add(u, v, p, &mut adds);
        let key = if i < j {
            (i, j, si, sj)
        } else {
            (j, i, sj, si)
        };
        if inputs.insert(key) && !sums.insert(q) {
            collisions += 1
        }
    }
    (
        inputs.len(),
        (count - inputs.len()) as u64,
        collisions,
        adds,
    )
}
fn main() {
    for (p, d, z) in [
        (8191, 42, 7036),
        (131071, 102, 37206),
        (524287, 171, 477103),
    ] {
        let base = roots(p, d, z);
        let root = energy(&base, p);
        let mut controls = Vec::new();
        for seed in 0..16 {
            controls.push(energy(&random_base(p, base.len(), 1000 + seed), p))
        }
        let mean = |f: fn(&Stats) -> u64| {
            controls.iter().map(f).sum::<u64>() as f64 / controls.len() as f64
        };
        println!("p={p} d={d} B={} root={root:?} random_mean_disjoint={:.2} random_mean_collisions={:.2} random_mean_distinct={:.2} control_adds={}",base.len(),mean(|s|s.disjoint),mean(|s|s.collisions),controls.iter().map(|s|s.distinct).sum::<usize>() as f64/16.0,controls.iter().map(|s|s.adds).sum::<u64>());
    }
    let p = (1u64 << 61) - 1;
    let base = roots(p, 4950, 636328216386749903);
    let ctl = random_base(p, base.len(), 2026);
    let a = sampled(&base, p, 77, 100_000);
    let b = sampled(&ctl, p, 77, 100_000);
    println!("M61 B={} 100k signed-pair draws root={a:?} random={b:?} (unique_inputs, repeated_inputs, genuine_output_collisions, additions)",base.len());
}
