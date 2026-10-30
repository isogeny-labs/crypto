// Offline factor-base shape comparison: distinct pair-sum x-keys for
// contiguous, strided and coverage-selected 45-abscissa bases on the
// 20-bit profile curve. Counts only; no claim about the pipeline yet.
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
use std::collections::HashSet;

struct C {
    p: u64,
    a: u64,
}

fn inv(a: u64, m: u64) -> u64 {
    let (mut t, mut newt, mut r, mut newr) = (0i128, 1i128, m as i128, (a % m) as i128);
    while newr != 0 {
        let q = r / newr;
        let nt = t - q * newt;
        t = newt;
        newt = nt;
        let nr = r - q * newr;
        r = newr;
        newr = nr;
    }
    if t < 0 {
        (t + m as i128) as u64
    } else {
        t as u64
    }
}

impl C {
    fn add(&self, p: (u64, u64), q: (u64, u64)) -> Option<(u64, u64)> {
        let p_ = self.p;
        if p == q {
            if p.1 == 0 {
                return None;
            }
            let lam = (3 * p.0 % p_ * p.0 % p_ + self.a) % p_ * inv((2 * p.1) % p_, p_) % p_;
            let x = (lam * lam % p_ + 2 * (p_ - p.0 % p_)) % p_;
            let y = (lam * ((p.0 + p_ - x) % p_) % p_ + p_ - p.1) % p_;
            Some((x, y))
        } else {
            let dx = (q.0 + p_ - p.0) % p_;
            if dx == 0 {
                return None;
            }
            let lam = (q.1 + p_ - p.1) % p_ * inv(dx, p_) % p_;
            let x = (lam * lam % p_ + 2 * (p_ - p.0 % p_) + p_ - q.0) % p_;
            let y = (lam * ((p.0 + p_ - x) % p_) % p_ + p_ - p.1) % p_;
            Some((x, y))
        }
    }
}

fn coverage(c: &C, abs: &[(u64, u64)]) -> usize {
    let mut pts: Vec<(u64, u64)> = Vec::new();
    for &(x, y) in abs {
        pts.push((x, y));
        if y != 0 && c.p - y != y {
            pts.push((x, c.p - y));
        }
    }
    let mut keys = HashSet::new();
    for i in 0..pts.len() {
        for j in i..pts.len() {
            if let Some((x, _)) = c.add(pts[i], pts[j]) {
                keys.insert(x.min(c.p - x));
            }
        }
    }
    keys.len()
}

fn main() {
    let spec = CurveSpec::PrimeSearch {
        bits: 20,
        seed: 59297,
    };
    let inst = spec.build().unwrap();
    let Instance::Prime(pi) = &inst else {
        panic!("prime expected")
    };
    let c = C {
        p: pi.curve.p,
        a: pi.curve.a,
    };
    let mut pool = Vec::new();
    let mut x = 1u64;
    while pool.len() < 200 && x < c.p {
        let rhs = (x * x % c.p * x + c.a * x % c.p + pi.curve.b) % c.p;
        if pi.curve.legendre(rhs) >= 0 {
            if let Some(y) = pi.curve.sqrt(rhs) {
                pool.push((x, y.min(c.p - y)));
            }
        }
        x += 1;
    }
    let cont: Vec<_> = pool[..45].to_vec();
    let strided: Vec<_> = (0..45).map(|i| pool[i * pool.len() / 45]).collect();
    let mut state = 0x9e3779b97f4a7c15u64;
    let mut rnd = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut best: Vec<(u64, u64)> = cont.clone();
    let mut best_c = coverage(&c, &best);
    for _ in 0..200 {
        let mut pick: Vec<(u64, u64)> = Vec::new();
        let mut used = vec![false; pool.len()];
        while pick.len() < 45 {
            let k = (rnd() as usize) % pool.len();
            if !used[k] {
                used[k] = true;
                pick.push(pool[k]);
            }
        }
        let cc = coverage(&c, &pick);
        if cc > best_c {
            best_c = cc;
            best = pick;
        }
    }
    let r = pi.r;
    println!("p={} a={} b={} r={}", c.p, c.a, pi.curve.b, r);
    for (name, base) in [
        ("contiguous", cont),
        ("strided", strided),
        ("coverage-selected", best),
    ] {
        let k = coverage(&c, &base);
        println!(
            "{:<18} keys={:5}  coverage={:.4}%",
            name,
            k,
            100.0 * k as f64 / r as f64
        );
    }
}
