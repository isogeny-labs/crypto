//! j=0 order-3 automorphism factor base vs the plain +/- factor base at an equal covered x-set.
//! Curve y^2 = x^3 + b over F_p, p = 1 mod 3, prime order n.  phi(x,y) = (beta x, y) = [lambda]P.
//! Symmetric arm: B orbit representatives, 6B covered points, B unknowns.
//! Plain arm (`ic_solve`, 2 summands): 3B x-coordinates covered, 3B unknowns.
//! usage: prime_fast_j0_sym <out.jsonl> <bits> <B,B,...> <instances> <extra>
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::io::Write;
use std::time::Instant;

/// Open-addressing x -> (orbit element id) table.
struct XTable {
    keys: Vec<u64>,
    vals: Vec<u32>,
    mask: usize,
}
impl XTable {
    fn new(cap: usize) -> Self {
        let n = (cap * 4).next_power_of_two();
        XTable {
            keys: vec![u64::MAX; n],
            vals: vec![0; n],
            mask: n - 1,
        }
    }
    #[inline(always)]
    fn h(&self, k: u64) -> usize {
        (k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & self.mask
    }
    fn insert(&mut self, k: u64, v: u32) {
        let mut i = self.h(k);
        while self.keys[i] != u64::MAX {
            i = (i + 1) & self.mask;
        }
        self.keys[i] = k;
        self.vals[i] = v;
    }
    #[inline(always)]
    fn get(&self, k: u64) -> Option<u32> {
        let mut i = self.h(k);
        loop {
            let kk = self.keys[i];
            if kk == k {
                return Some(self.vals[i]);
            }
            if kk == u64::MAX {
                return None;
            }
            i = (i + 1) & self.mask;
        }
    }
}

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

struct SymStats {
    ms_total: f64,
    ms_fb: f64,
    ms_collect: f64,
    ms_la: f64,
    targets: u64,
    inversions: u64,
    rels: usize,
}

fn sym_solve(
    c: &FastCurve,
    g: Pt,
    q: Pt,
    lam: u64,
    b_size: usize,
    extra: usize,
    seed: u64,
) -> Option<(u64, SymStats)> {
    let f = c.f;
    let n = c.n;
    let fnum = c.scalar_field();
    let beta = beta_of(c, g, lam);
    let beta2 = f.mul(beta, beta);
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let t0 = Instant::now();
    // orbit representatives
    let mut reps: Vec<Pt> = Vec::with_capacity(b_size);
    let mut tab = XTable::new(3 * b_size);
    let mut x = 1u64;
    while reps.len() < b_size {
        if let Some(pt) = c.lift_x(x) {
            let xs = [pt.x, f.mul(beta, pt.x), f.mul(beta2, pt.x)];
            if xs.iter().all(|&v| tab.get(v).is_none()) {
                let i = reps.len() as u32;
                for k in 0..3u32 {
                    tab.insert(xs[k as usize], i * 3 + k);
                }
                reps.push(pt);
            }
        }
        x += 1;
    }
    // candidate points phi^k(F_i), id = 3i+k
    let mut cand: Vec<Pt> = Vec::with_capacity(3 * b_size);
    for r in &reps {
        cand.push(*r);
        cand.push(Pt {
            x: f.mul(beta, r.x),
            y: r.y,
        });
        cand.push(Pt {
            x: f.mul(beta2, r.x),
            y: r.y,
        });
    }
    let ms_fb = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let needed = b_size + extra;
    let cols = b_size + 1;
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    let fresh = |rng: &mut StdRng| -> (u64, u64, Pt) {
        loop {
            let a = rng.gen_range(1..n);
            let b = rng.gen_range(1..n);
            if let Some(r) = c.combine(g, q, a, b) {
                return (a, b, r);
            }
        }
    };
    let (mut a, mut b, mut r) = fresh(&mut rng);
    let mut targets = 0u64;
    let mut inversions = 0u64;
    let mut d = vec![0u64; cand.len()];
    let mut scratch = Vec::new();
    let max_targets = 50_000_000u64;
    while rows.len() < needed && targets < max_targets {
        targets += 1;
        for (i, p) in cand.iter().enumerate() {
            let dd = f.sub(r.x, p.x);
            d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut d, &mut scratch);
        inversions += 1;
        let mut productive = false;
        'scan: for (id, fi) in cand.iter().enumerate() {
            let inv = d[id];
            if f.sub(r.x, fi.x) == 0 {
                continue;
            }
            let lam_minus = f.mul(f.add(r.y, fi.y), inv);
            let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
            for (eps, l) in [(1i8, lam_minus), (-1i8, lam_plus)] {
                let x3 = f.sub(f.sub(f.sqr(l), r.x), fi.x);
                let Some(jid) = tab.get(x3) else { continue };
                let jid = jid as usize;
                if jid < id {
                    continue;
                }
                let y3 = f.sub(f.mul(l, f.sub(r.x, x3)), r.y);
                let tau = if y3 == cand[jid].y { 1i8 } else { -1 };
                // R = eps*cand[id] + tau*cand[jid]
                let t_pt = Pt { x: x3, y: y3 };
                let want = c.add(
                    Some(if eps > 0 { *fi } else { c.neg(*fi) }),
                    Some(if tau > 0 { cand[jid] } else { c.neg(cand[jid]) }),
                );
                let _ = t_pt;
                if want != Some(r) {
                    continue;
                }
                let (i, k) = (id / 3, id % 3);
                let (j, m) = (jid / 3, jid % 3);
                let mut row = vec![0u64; cols];
                let ci = if eps > 0 { lams[k] } else { n - lams[k] } % n;
                let cj = if tau > 0 { lams[m] } else { n - lams[m] } % n;
                row[i] = fnum.to_mont(ci);
                row[j] = fnum.add(row[j], fnum.to_mont(cj));
                row[b_size] = fnum.to_mont((n - b % n) % n);
                rows.push(row);
                rhs.push(fnum.to_mont(a % n));
                productive = true;
                if rows.len() >= needed {
                    break 'scan;
                }
            }
        }
        if productive {
            let (na, nb, nr) = fresh(&mut rng);
            a = na;
            b = nb;
            r = nr;
        } else {
            match c.add(Some(r), Some(g)) {
                Some(nr) => {
                    r = nr;
                    a = (a + 1) % n;
                }
                None => {
                    let (na, nb, nr) = fresh(&mut rng);
                    a = na;
                    b = nb;
                    r = nr;
                }
            }
        }
    }
    if rows.len() < needed {
        return None;
    }
    let ms_collect = t1.elapsed().as_secs_f64() * 1e3;
    let t2 = Instant::now();
    let nrows = rows.len();
    let mut pivot_row = vec![usize::MAX; cols];
    let mut row = 0usize;
    for col in 0..cols {
        if row >= nrows {
            break;
        }
        let Some(piv) = (row..nrows).find(|&rr| rows[rr][col] != 0) else {
            continue;
        };
        rows.swap(row, piv);
        rhs.swap(row, piv);
        let inv = fnum.inv(rows[row][col]);
        for cc in 0..cols {
            rows[row][cc] = fnum.mul(rows[row][cc], inv);
        }
        rhs[row] = fnum.mul(rhs[row], inv);
        let (pv, pr) = (rows[row].clone(), rhs[row]);
        for rr in 0..nrows {
            if rr == row || rows[rr][col] == 0 {
                continue;
            }
            let fct = rows[rr][col];
            for cc in col..cols {
                if pv[cc] != 0 {
                    rows[rr][cc] = fnum.sub(rows[rr][cc], fnum.mul(fct, pv[cc]));
                }
            }
            rhs[rr] = fnum.sub(rhs[rr], fnum.mul(fct, pr));
        }
        pivot_row[col] = row;
        row += 1;
    }
    let pr = pivot_row[b_size];
    if pr == usize::MAX {
        return None;
    }
    let xsol = fnum.from_mont(rhs[pr]);
    let ms_la = t2.elapsed().as_secs_f64() * 1e3;
    if c.mul(Some(g), xsol) != Some(q) {
        return None;
    }
    Some((
        xsol,
        SymStats {
            ms_total: t0.elapsed().as_secs_f64() * 1e3,
            ms_fb,
            ms_collect,
            ms_la,
            targets,
            inversions,
            rels: nrows,
        },
    ))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let bs: Vec<usize> = a[3].split(',').map(|s| s.parse().unwrap()).collect();
    let inst: u64 = a[4].parse().unwrap();
    let extra: usize = a[5].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for curve_idx in 0..2u64 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        let mut rho_ms = Vec::new();
        for t in 0..10u64 {
            let mut rng = StdRng::seed_from_u64(0xFEED ^ t ^ c.n);
            let q = c.mul(Some(g), rng.gen_range(2..c.n)).unwrap();
            let mut cfg = RhoConfig::for_bits(bits, true);
            cfg.threads = 1;
            cfg.seed = t + 1;
            let t0 = Instant::now();
            if rho_parallel(&c, g, q, &cfg).is_some() {
                rho_ms.push(t0.elapsed().as_secs_f64() * 1e3);
            }
        }
        rho_ms.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let rho1 = rho_ms.get(rho_ms.len() / 2).copied();
        for &bsz in &bs {
            for ins in 0..inst {
                let mut rng = StdRng::seed_from_u64(0xA11CE ^ ins ^ c.n ^ bsz as u64);
                let k = rng.gen_range(2..c.n);
                let q = c.mul(Some(g), k).unwrap();
                let sym = sym_solve(&c, g, q, lam, bsz, extra, ins + 1);
                let t0 = Instant::now();
                let plain = ic_solve(&c, g, q, 3 * bsz, extra, 2, u64::MAX, ins + 1);
                let plain_ms = t0.elapsed().as_secs_f64() * 1e3;
                let fbs3 = ((0.5 * (c.f.p as f64).cbrt()).round() as usize).max(8);
                let t3 = Instant::now();
                let p3 = ic_solve(&c, g, q, fbs3, extra, 3, u64::MAX, ins + 1);
                let p3_ms = t3.elapsed().as_secs_f64() * 1e3;
                let row = json!({
                    "plain3_ok": p3.as_ref().map(|s| s.0 == k).unwrap_or(false), "plain3_ms": p3_ms, "plain3_fb": fbs3,
                    "curve": c.name, "rho_neg_median_ms": rho1, "p": c.f.p, "n": c.n, "B": bsz, "inst": ins,
                    "sym_ok": sym.as_ref().map(|s| s.0 == k).unwrap_or(false),
                    "sym_ms": sym.as_ref().map(|s| s.1.ms_total),
                    "sym_fb_ms": sym.as_ref().map(|s| s.1.ms_fb),
                    "sym_collect_ms": sym.as_ref().map(|s| s.1.ms_collect),
                    "sym_la_ms": sym.as_ref().map(|s| s.1.ms_la),
                    "sym_targets": sym.as_ref().map(|s| s.1.targets),
                    "sym_rels": sym.as_ref().map(|s| s.1.rels),
                    "sym_inv": sym.as_ref().map(|s| s.1.inversions),
                    "plain_ok": plain.as_ref().map(|s| s.0 == k).unwrap_or(false),
                    "plain_ms": plain_ms,
                    "plain_fb_ms": plain.as_ref().map(|s| s.1.factor_base_ms),
                    "plain_collect_ms": plain.as_ref().map(|s| s.1.relations_ms),
                    "plain_la_ms": plain.as_ref().map(|s| s.1.linear_algebra_ms),
                    "plain_targets": plain.as_ref().map(|s| s.1.targets),
                    "plain_rels": plain.as_ref().map(|s| s.1.relations),
                });
                writeln!(out, "{row}").unwrap();
                println!("{row}");
            }
        }
    }
}
