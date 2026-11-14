//! Automorphism-folded (x6) MULTI-target distinguished-point rho on j=0 curves (affine union-find over the target logs).
//! usage: prime_fast_j0_rho_multi <out.jsonl> <bits> <T> <instances>
//!
//! Curve y^2 = x^3 + b over F_p, p = 1 mod 3, prime order n.  phi(x,y) = (beta x, y) = [lambda]P.
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::io::Write;
use std::time::Instant;

fn find_j0_curve(bits: u32, skip: u64) -> (FastCurve, Pt, u64, u64) {
    let mut p = (1u64 << bits) - 1;
    let mut skipped = 0;
    loop {
        while !(p % 3 == 1 && is_prime_u64(p)) {
            p -= 1;
        }
        let f = Fp64::new(p);
        for b in 1u64..2000 {
            let probe = FastCurve::new("probe", p, 0, b, 1);
            let Some(g) = (1u64..10_000).find_map(|x| probe.lift_x(x)) else {
                continue;
            };
            let Some(order) = point_order_hasse(&probe, g) else {
                continue;
            };
            if !is_prime_u64(order) || order % 3 != 1 {
                continue;
            }
            if skipped < skip {
                skipped += 1;
                continue;
            }
            let c = FastCurve::new(&format!("j0-{bits}-b{b}"), p, 0, b, order);
            let g = c.lift_x(probe.canonical(g).0).unwrap();
            // beta: cube root of unity mod p (Montgomery).
            let mut beta = f.one;
            for t in 2u64..100 {
                let r = f.pow(f.to_mont(t), (p - 1) / 3);
                if r != f.one {
                    beta = r;
                    break;
                }
            }
            assert!(beta != f.one);
            // lambda mod n with phi(G) = [lambda]G.
            let fnum = c.scalar_field();
            let mut lam = 0u64;
            for h in 2u64..100 {
                let r = fnum.from_mont(fnum.pow(fnum.to_mont(h), (order - 1) / 3));
                if r != 1 {
                    let phi = Pt {
                        x: f.mul(beta, g.x),
                        y: g.y,
                    };
                    for cand in [r, (r as u128 * r as u128 % order as u128) as u64] {
                        if c.mul(Some(g), cand) == Some(phi) {
                            lam = cand;
                        }
                    }
                    break;
                }
            }
            if lam == 0 {
                // try the other beta
                continue;
            }
            return (c, g, order, lam);
        }
        p -= 1;
    }
}

fn beta_of(c: &FastCurve, g: Pt, lam: u64) -> u64 {
    let f = c.f;
    let target = c.mul(Some(g), lam).unwrap();
    let want = f.mul(target.x, f.inv(g.x));
    assert_eq!(target.y, g.y);
    want
}

fn canon_pt(c: &FastCurve, beta: u64, beta2: u64, p: Pt) -> (Pt, usize, bool) {
    // returns canonical point, rotation s (canon = phi^s(p) up to sign), negated flag
    let f = c.f;
    let xs = [p.x, f.mul(beta, p.x), f.mul(beta2, p.x)];
    let mut s = 0;
    for k in 1..3 {
        if xs[k] < xs[s] {
            s = k;
        }
    }
    let q = Pt { x: xs[s], y: p.y };
    let (q2, neg) = c.canonical_sign(q);
    (q2, s, neg)
}

struct Dsu {
    n: u64,
    par: Vec<usize>,
    al: Vec<u64>,
    be: Vec<u64>,
    anchor: usize,
}
impl Dsu {
    fn new(n: u64, t: usize) -> Self {
        let mut d = Dsu {
            n,
            par: (0..=t).collect(),
            al: vec![1; t + 1],
            be: vec![0; t + 1],
            anchor: t,
        };
        d.par[t] = t;
        d
    }
    fn mm(&self, a: u64, b: u64) -> u64 {
        (a as u128 * b as u128 % self.n as u128) as u64
    }
    fn ad(&self, a: u64, b: u64) -> u64 {
        ((a as u128 + b as u128) % self.n as u128) as u64
    }
    fn ng(&self, a: u64) -> u64 {
        (self.n - a % self.n) % self.n
    }
    /// k_i = al * k_root + be
    fn find(&mut self, i: usize) -> (usize, u64, u64) {
        let (mut r, mut a, mut b) = (i, 1u64, 0u64);
        // k_i = a*k_r + b along the path
        while self.par[r] != r {
            let p = self.par[r];
            // k_r = al[r]*k_p + be[r]
            b = self.ad(self.mm(a, self.be[r]), b);
            a = self.mm(a, self.al[r]);
            r = p;
        }
        if i != r {
            self.par[i] = r;
            self.al[i] = a;
            self.be[i] = b;
        }
        (r, a, b)
    }
    fn inv(&self, a: u64) -> u64 {
        inv_mod_u64(a, self.n)
    }
    fn set_known(&mut self, r: usize, val: u64) {
        let an = self.anchor;
        self.par[r] = an;
        self.al[r] = 0;
        self.be[r] = val % self.n;
    }
    /// b1 k_i - b2 k_j = c (mod n); returns true if something new was learned.
    fn merge(&mut self, i: usize, j: usize, b1: u64, b2: u64, c: u64) -> bool {
        let (r1, a1, o1) = self.find(i);
        let (r2, a2, o2) = self.find(j);
        let x = self.mm(b1, a1);
        let y = self.mm(b2, a2);
        let cp = self.ad(self.ad(c, self.ng(self.mm(b1, o1))), self.mm(b2, o2));
        let an = self.anchor;
        let (k1known, k2known) = (r1 == an, r2 == an);
        if k1known && k2known {
            return false;
        }
        if r1 == r2 {
            let d = self.ad(x, self.ng(y));
            if d == 0 {
                return false;
            }
            let v = self.mm(cp, self.inv(d));
            self.set_known(r1, v);
            return true;
        }
        if k1known {
            if y == 0 {
                return false;
            }
            let v = self.mm(self.ad(x, self.ng(cp)), self.inv(y));
            self.set_known(r2, v);
            return true;
        }
        if k2known {
            if x == 0 {
                return false;
            }
            let v = self.mm(self.ad(cp, y), self.inv(x));
            self.set_known(r1, v);
            return true;
        }
        if x != 0 {
            let ix = self.inv(x);
            self.par[r1] = r2;
            self.al[r1] = self.mm(y, ix);
            self.be[r1] = self.mm(cp, ix);
            return true;
        }
        if y != 0 {
            let iy = self.inv(y);
            self.par[r2] = r1;
            self.al[r2] = self.mm(x, iy);
            self.be[r2] = self.ng(self.mm(cp, iy));
            return true;
        }
        false
    }
}

struct Lane {
    p: Pt,
    a: u64,
    b: u64,
    t: usize,
    len: u64,
    ring: [u64; 8],
    ri: usize,
}

#[allow(clippy::too_many_arguments)]
fn multi_rho(
    c: &FastCurve,
    g: Pt,
    lam: u64,
    qs: &[Pt],
    dp_bits: u32,
    seed: u64,
    lanes_n: usize,
) -> Option<(Vec<u64>, u64)> {
    let f = c.f;
    let n = c.n;
    let t_count = qs.len();
    let beta = beta_of(c, g, lam);
    let beta2 = f.mul(beta, beta);
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let mulm = |a: u64, b: u64| (a as u128 * b as u128 % n as u128) as u64;
    let mut rng = StdRng::seed_from_u64(seed);
    const R: usize = 32;
    let mut tb: Vec<(Pt, u64)> = Vec::new();
    while tb.len() < R {
        let u = rng.gen_range(1..n);
        if let Some(t) = c.mul(Some(g), u) {
            tb.push((t, u));
        }
    }
    let mut dsu = Dsu::new(n, t_count);
    let mut dps: std::collections::HashMap<u64, (usize, u64, u64)> =
        std::collections::HashMap::new();
    let mut steps = 0u64;
    let mask = (1u64 << dp_bits) - 1;
    let cap = 40u64 << dp_bits;
    let new_lane = |rng: &mut StdRng| -> Lane {
        loop {
            let t = rng.gen_range(0..=t_count);
            let qt = if t == t_count { g } else { qs[t] };
            let (a0, b0) = (rng.gen_range(1..n), rng.gen_range(1..n));
            let Some(p0) = c.combine(g, qt, a0, b0) else {
                continue;
            };
            let (p, s0, neg0) = canon_pt(c, beta, beta2, p0);
            let mut mu = lams[s0];
            if neg0 {
                mu = (n - mu) % n;
            }
            return Lane {
                p,
                a: mulm(a0, mu),
                b: mulm(b0, mu),
                t,
                len: 0,
                ring: [u64::MAX; 8],
                ri: 0,
            };
        }
    };
    let mut lanes: Vec<Lane> = (0..lanes_n).map(|_| new_lane(&mut rng)).collect();
    let mut d = vec![0u64; lanes_n];
    let mut scratch: Vec<u64> = Vec::new();
    loop {
        for (i, ln) in lanes.iter().enumerate() {
            let h = (f.from_mont(ln.p.x) as usize) % R;
            let dd = f.sub(tb[h].0.x, ln.p.x);
            d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut d, &mut scratch);
        for i in 0..lanes_n {
            let (h, p, dx0) = {
                let ln = &lanes[i];
                let h = (f.from_mont(ln.p.x) as usize) % R;
                (h, ln.p, f.sub(tb[h].0.x, ln.p.x))
            };
            let (mut t_pt, mut u) = tb[h];
            let p1 = if dx0 == 0 {
                match c.add(Some(p), Some(t_pt)) {
                    Some(q) => q,
                    None => {
                        lanes[i] = new_lane(&mut rng);
                        continue;
                    }
                }
            } else {
                c.add_with_inv(p, t_pt, d[i])
            };
            let (mut p2, mut s, mut neg) = canon_pt(c, beta, beta2, p1);
            if lanes[i].ring.contains(&p2.x) {
                (t_pt, u) = tb[(h + 1) % R];
                match c.add(Some(p), Some(t_pt)) {
                    Some(q1) => {
                        (p2, s, neg) = canon_pt(c, beta, beta2, q1);
                    }
                    None => {
                        lanes[i] = new_lane(&mut rng);
                        continue;
                    }
                }
            }
            let mut mu = lams[s];
            if neg {
                mu = (n - mu) % n;
            }
            let ln = &mut lanes[i];
            ln.ring[ln.ri & 7] = p2.x;
            ln.ri += 1;
            ln.a = mulm((ln.a + u) % n, mu);
            ln.b = mulm(ln.b, mu);
            ln.p = p2;
            ln.len += 1;
            steps += 1;
            let is_dp = f.from_mont(p2.x) & mask == 0;
            if is_dp {
                let key = f.from_mont(p2.x) ^ (f.from_mont(p2.y) << 32);
                let (t, a, b) = (ln.t, ln.a, ln.b);
                let mut learned = false;
                if let Some(&(t2, a2, b2)) = dps.get(&key) {
                    let diff = (a2 + n - a) % n;
                    learned = dsu.merge(t, t2, b, b2, diff);
                } else {
                    dps.insert(key, (t, a, b));
                }
                lanes[i] = new_lane(&mut rng);
                if learned {
                    let mut all = true;
                    for k in 0..t_count {
                        if dsu.find(k).0 != t_count {
                            all = false;
                            break;
                        }
                    }
                    if all {
                        let mut ks = Vec::with_capacity(t_count);
                        for k in 0..t_count {
                            let (_, al, be) = dsu.find(k);
                            ks.push((al + be) % n);
                        }
                        return Some((ks, steps));
                    }
                }
            } else if ln.len > cap {
                lanes[i] = new_lane(&mut rng);
            }
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let t_count: usize = a[3].parse().unwrap();
    let inst: u64 = a[4].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    let cs: u64 = std::env::var("CSKIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for curve_idx in cs..cs + 2 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        let dp_bits = ((bits / 2).saturating_sub(10)
            + (usize::BITS - 1 - t_count.max(1).leading_zeros()) / 2)
            .max(2);
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let t0 = Instant::now();
            let res = multi_rho(
                &c,
                g,
                lam,
                &qs,
                dp_bits,
                ins + 1,
                std::env::var("LANES")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(128),
            );
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            let (ok, steps) = match &res {
                Some((got, st)) => (got.iter().zip(&ks).filter(|(x, k)| x == k).count(), *st),
                None => (0, 0),
            };
            let row = json!({"bits": bits, "curve": c.name, "T": t_count, "inst": ins, "dp_bits": dp_bits,
                "ok": ok, "steps": steps, "folded_multi_ms": ms});
            writeln!(out, "{row}").unwrap();
            println!("{row}");
        }
    }
}
