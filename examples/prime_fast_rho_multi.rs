//! Multi-target distinguished-point rho on the native engine, for the same planted targets as
//! `prime_fast_multi` / `prime_fast_bsgs_multi`.  Walks use an r-adding table of multiples of G only, so
//! a walk from `u G + Q_t` stays `a G + Q_t`; a collision between walks of targets t1, t2 gives
//! `a1 + k_t1 = +-(a2 + k_t2) (mod n)`.  Target 0 is the anchor Q_0 = G (log 1).  Signed union-find over
//! the targets; the run stops when every target's log is determined; every log is verified `k G = Q`.
//! usage: prime_fast_rho_multi <out.jsonl> <bits> <T> <instances>
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

struct Dsu {
    parent: Vec<usize>,
    alpha: Vec<i8>, // k_node = alpha * k_parent + beta
    beta: Vec<u64>,
    known: Vec<Option<u64>>, // value of k_root when known (indexed by root)
    n: u64,
}

fn addm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + b as u128) % n as u128) as u64
}
fn negm(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}
fn mulm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn invm(a: u64, n: u64) -> u64 {
    // n prime: a^(n-2)
    let (mut r, mut b, mut e) = (1u64, a % n, n - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, n);
        }
        b = mulm(b, b, n);
        e >>= 1;
    }
    r
}

impl Dsu {
    fn find(&mut self, x: usize) -> (usize, i8, u64) {
        // returns (root, alpha, beta) with k_x = alpha * k_root + beta
        let mut path = vec![];
        let mut cur = x;
        while self.parent[cur] != cur {
            path.push(cur);
            cur = self.parent[cur];
        }
        let root = cur;
        // compress from the top
        let mut acc_a: i8 = 1;
        let mut acc_b: u64 = 0;
        for &v in path.iter().rev() {
            // k_v = a_v * k_p + b_v with p = old parent (already compressed to root: k_p = acc_a*k_root+acc_b)
            let (av, bv) = (self.alpha[v], self.beta[v]);
            let na = av * acc_a;
            let nb = if av == 1 {
                addm(bv, acc_b, self.n)
            } else {
                addm(bv, negm(acc_b, self.n), self.n)
            };
            self.alpha[v] = na;
            self.beta[v] = nb;
            self.parent[v] = root;
            acc_a = na;
            acc_b = nb;
        }
        if x == root {
            (root, 1, 0)
        } else {
            (root, self.alpha[x], self.beta[x])
        }
    }
    /// relation k_x = eps * k_y + d
    fn relate(&mut self, x: usize, y: usize, eps: i8, d: u64) {
        let (rx, ax, bx) = self.find(x);
        let (ry, ay, by) = self.find(y);
        let n = self.n;
        if rx == ry {
            // ax r + bx = eps (ay r + by) + d  ->  (ax - eps ay) r = eps by + d - bx
            let lhs = ax - eps * ay; // in {-2, 0, 2}
            if lhs == 0 {
                return;
            }
            let rhs = addm(
                addm(if eps == 1 { by } else { negm(by, n) }, d, n),
                negm(bx, n),
                n,
            );
            let two_inv = invm(2, n);
            let mut r = mulm(rhs, two_inv, n);
            if lhs < 0 {
                r = negm(r, n);
            }
            if self.known[rx].is_none() {
                self.known[rx] = Some(r);
            }
            return;
        }
        // attach rx under ry: k_rx = alpha * k_ry + beta
        // ax k_rx + bx = eps (ay k_ry + by) + d  ->  k_rx = ax*(eps*ay k_ry + eps*by + d - bx)  (ax = +-1)
        let a_new = ax * eps * ay;
        let c = addm(
            addm(if eps == 1 { by } else { negm(by, n) }, d, n),
            negm(bx, n),
            n,
        );
        let b_new = if ax == 1 { c } else { negm(c, n) };
        self.parent[rx] = ry;
        self.alpha[rx] = a_new;
        self.beta[rx] = b_new;
        if let (Some(kx), None) = (self.known[rx], self.known[ry]) {
            // k_rx = a_new k_ry + b_new -> k_ry = a_new (k_rx - b_new)
            let t = addm(kx, negm(b_new, n), n);
            self.known[ry] = Some(if a_new == 1 { t } else { negm(t, n) });
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let t_count: usize = a[3].parse().unwrap();
    let inst: u64 = a[4].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for res in [3u64, 1] {
        let Some((c, g)) = find_a3_curve(bits, res) else {
            continue;
        };
        for ins in 0..inst {
            let n = c.n;
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..n)).collect();
            let mut qs: Vec<Pt> = vec![g]; // anchor
            let mut klog: Vec<u64> = vec![1];
            for &k in &ks {
                qs.push(c.mul(Some(g), k).unwrap());
                klog.push(k);
            }
            let m = qs.len();
            let t0 = Instant::now();
            let mut rng = StdRng::seed_from_u64(0x1234 ^ ins ^ n);
            let steps: Vec<(u64, Pt)> = (0..32)
                .map(|_| {
                    let s = rng.gen_range(1..n);
                    (s, c.mul(Some(g), s).unwrap())
                })
                .collect();
            let tab_a: Vec<(u64, Pt)> = (0..4096)
                .map(|_| {
                    let s = rng.gen_range(1..n);
                    (s, c.mul(Some(g), s).unwrap())
                })
                .collect();
            let tab_b: Vec<(u64, Pt)> = (0..4096)
                .map(|_| {
                    let s = rng.gen_range(1..n);
                    (s, c.mul(Some(g), s).unwrap())
                })
                .collect();
            let lanes = 2048usize.min(m * 2).max(64);
            let total_ops = ((n as f64) * (m as f64)).sqrt();
            let dp_period = ((total_ops / (lanes as f64 * 20.0)).max(1.0))
                .log2()
                .floor() as u32;
            let dp_mask: u64 = (1u64 << dp_period) - 1;
            let mut tgt = vec![0usize; lanes];
            let mut off = vec![0u64; lanes];
            let mut cur: Vec<Option<Pt>> = Vec::with_capacity(lanes);
            for l in 0..lanes {
                let t = rng.gen_range(0..m);
                let u = rng.gen_range(1..n);
                tgt[l] = t;
                off[l] = u;
                cur.push(c.add(c.mul(Some(g), u), Some(qs[t])));
            }
            let mut dsu = Dsu {
                parent: (0..m).collect(),
                alpha: vec![1; m],
                beta: vec![0; m],
                known: vec![None; m],
                n,
            };
            // anchor: k_0 = 1 -> root 0 known = 1
            dsu.known[0] = Some(1);
            let mut dps: HashMap<u64, (u64, usize, u64)> = HashMap::new();
            let mut scratch = BatchAddScratch::default();
            let mut total_steps = 0u64;
            let mut resolved_check = 0u64;
            let mut done = false;
            let mut sel: Vec<Option<Pt>> = vec![None; lanes];
            let mut restart = vec![false; lanes];
            while !done {
                for l in 0..lanes {
                    let p = match cur[l] {
                        Some(p) => p,
                        None => {
                            let t = rng.gen_range(0..m);
                            let u = rng.gen_range(1..n);
                            tgt[l] = t;
                            // the lane is advanced by steps[0] below, so account for it in the offset
                            off[l] = addm(u, steps[0].0, n);
                            cur[l] = c.add(c.mul(Some(g), u), Some(qs[t]));
                            sel[l] = Some(steps[0].1);
                            continue;
                        }
                    };
                    let xc = c.f.from_mont(p.x);
                    let h = xc.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 59; // 5 bits
                    let (s, sp) = steps[h as usize];
                    sel[l] = Some(sp);
                    off[l] = addm(off[l], s, n);
                    // distinguished point test (on the current x, before the step)
                    if xc & dp_mask == 0 {
                        restart[l] = true;
                        let key = xc;
                        if let Some(&(y0, t0_, a0)) = dps.get(&key) {
                            if t0_ != tgt[l] || a0 != off[l] {
                                // a0 + k_t0 = +-(off + k_t) ; off here already includes this step: undo
                                let aoff = (off[l] + n - s) % n;
                                if y0 == p.y {
                                    // a0 + k_t0 = aoff + k_t -> k_t0 = k_t + (aoff - a0)
                                    dsu.relate(t0_, tgt[l], 1, addm(aoff, negm(a0, n), n));
                                } else {
                                    // a0 + k_t0 = -(aoff + k_t) -> k_t0 = -k_t - aoff - a0
                                    dsu.relate(t0_, tgt[l], -1, negm(addm(aoff, a0, n), n));
                                }
                            }
                        } else {
                            let aoff = (off[l] + n - s) % n;
                            dps.insert(key, (p.y, tgt[l], aoff));
                        }
                    }
                }
                c.add_many_batched(&mut cur, &sel, &mut scratch);
                for l in 0..lanes {
                    if restart[l] {
                        restart[l] = false;
                        let t = rng.gen_range(0..m);
                        let (sa, pa) = tab_a[rng.gen_range(0..tab_a.len())];
                        let (sb, pb) = tab_b[rng.gen_range(0..tab_b.len())];
                        tgt[l] = t;
                        off[l] = addm(sa, sb, n);
                        cur[l] = c.add(c.add(Some(pa), Some(pb)), Some(qs[t]));
                    }
                }
                total_steps += lanes as u64;
                resolved_check += 1;
                if resolved_check.is_multiple_of(64) {
                    let mut all = true;
                    for t in 0..m {
                        let (r, _, _) = dsu.find(t);
                        if dsu.known[r].is_none() {
                            all = false;
                            break;
                        }
                    }
                    done = all;
                    if total_steps > 400 * (total_ops as u64 + 1000) {
                        break;
                    }
                }
            }
            let mut ok = 0usize;
            let mut resolved = 0usize;
            for t in 1..m {
                let (r, al, be) = dsu.find(t);
                if let Some(kr) = dsu.known[r] {
                    resolved += 1;
                    let k = if al == 1 {
                        addm(kr, be, n)
                    } else {
                        addm(negm(kr, n), be, n)
                    };
                    if c.mul(Some(g), k) == Some(qs[t]) && k == klog[t] {
                        ok += 1;
                    }
                }
            }
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            let row = json!({"bits":bits,"res":res,"n":n,"T":t_count,"inst":ins,"lanes":lanes,"dp_bits":dp_period,
                "steps":total_steps,"dps":dps.len(),"resolved":resolved,"ok":ok,"rho_multi_total_ms":ms});
            println!("{row}");
            writeln!(out, "{row}").unwrap();
        }
    }
}
