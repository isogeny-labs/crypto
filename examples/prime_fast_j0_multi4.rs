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

struct Ctx4 {
    reps: Vec<Pt>,
    cand: Vec<Pt>,
    ents: Vec<(u64, [u32; 3], [u8; 4])>,
    heads: Vec<u32>,
    next: Vec<u32>,
    hmask: usize,
    bits: Vec<u64>,
    bmask: usize,
    beta: u64,
    beta2: u64,
}

impl Ctx4 {
    fn canon(&self, f: &Fp64, x3: u64) -> (u64, usize) {
        let x1 = f.mul(self.beta, x3);
        let x2 = f.mul(self.beta2, x3);
        if x3 <= x1 && x3 <= x2 {
            (x3, 0)
        } else if x1 <= x2 {
            (x1, 2)
        } else {
            (x2, 1)
        }
    }
}

fn build4(c: &FastCurve, g: Pt, lam: u64, b_size: usize) -> Ctx4 {
    let f = c.f;
    let beta = beta_of(c, g, lam);
    let beta2 = f.mul(beta, beta);
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
    let mut ctx = Ctx4 {
        reps,
        cand,
        ents: Vec::new(),
        heads: Vec::new(),
        next: Vec::new(),
        hmask: 0,
        bits: Vec::new(),
        bmask: 0,
        beta,
        beta2,
    };
    let mut dd = Vec::new();
    let mut sc = Vec::new();
    for i in 0..b_size {
        let fi = ctx.reps[i];
        for j in (i + 1)..b_size {
            for m in 0..3usize {
                for s1 in 0..2u8 {
                    let qj = if s1 == 1 {
                        c.neg(ctx.cand[3 * j + m])
                    } else {
                        ctx.cand[3 * j + m]
                    };
                    let Some(pp) = c.add(Some(fi), Some(qj)) else {
                        continue;
                    };
                    dd.clear();
                    for l in (j + 1)..b_size {
                        for nn in 0..3usize {
                            let dx = f.sub(ctx.cand[3 * l + nn].x, pp.x);
                            dd.push(if dx == 0 { f.one } else { dx });
                        }
                    }
                    f.batch_inv(&mut dd, &mut sc);
                    let mut pos = 0usize;
                    for l in (j + 1)..b_size {
                        for nn in 0..3usize {
                            let ql = ctx.cand[3 * l + nn];
                            let inv = dd[pos];
                            pos += 1;
                            if f.sub(ql.x, pp.x) == 0 {
                                continue;
                            }
                            let sx = c.add_x_with_inv(pp, ql, inv);
                            let dx2 = c.add_x_with_inv(pp, c.neg(ql), inv);
                            let (k1, e1) = ctx.canon(&f, sx);
                            ctx.ents.push((
                                k1,
                                [i as u32, j as u32, l as u32],
                                [m as u8, nn as u8, s1 * 2, e1 as u8],
                            ));
                            let (k2, e2) = ctx.canon(&f, dx2);
                            ctx.ents.push((
                                k2,
                                [i as u32, j as u32, l as u32],
                                [m as u8, nn as u8, s1 * 2 + 1, e2 as u8],
                            ));
                        }
                    }
                }
            }
        }
    }
    let hs = (ctx.ents.len() * 2).next_power_of_two().max(16);
    ctx.hmask = hs - 1;
    ctx.heads = vec![u32::MAX; hs];
    ctx.next = vec![u32::MAX; ctx.ents.len()];
    for (e, ent) in ctx.ents.iter().enumerate() {
        let h = (ent.0.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & ctx.hmask;
        ctx.next[e] = ctx.heads[h];
        ctx.heads[h] = e as u32;
    }
    let nb = (ctx.ents.len() * 3 * 16).next_power_of_two().max(64);
    ctx.bmask = nb - 1;
    ctx.bits = vec![0u64; nb / 64];
    for ent in ctx.ents.iter() {
        let k0 = ent.0;
        for kx in [k0, f.mul(beta, k0), f.mul(beta2, k0)] {
            let h = (kx.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 24) as usize & ctx.bmask;
            ctx.bits[h >> 6] |= 1u64 << (h & 63);
        }
    }
    ctx
}

type Terms4 = [(usize, usize, i8); 4];

/// All verified decompositions R = sum of four signed orbit elements (exact point addition).
fn scan4(
    c: &FastCurve,
    ctx: &Ctx4,
    r: Pt,
    d: &mut Vec<u64>,
    scratch: &mut Vec<u64>,
    found: &mut Vec<Terms4>,
) {
    let f = c.f;
    found.clear();
    d.clear();
    for p in ctx.cand.iter() {
        let dd = f.sub(r.x, p.x);
        d.push(if dd == 0 { f.one } else { dd });
    }
    f.batch_inv(d, scratch);
    for (id, fi) in ctx.cand.iter().enumerate() {
        let inv = d[id];
        if f.sub(r.x, fi.x) == 0 {
            continue;
        }
        let lam_minus = f.mul(f.add(r.y, fi.y), inv);
        let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
        for (eps, l) in [(1i8, lam_minus), (-1i8, lam_plus)] {
            let x3 = f.sub(f.sub(f.sqr(l), r.x), fi.x);
            let h = (x3.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 24) as usize & ctx.bmask;
            if ctx.bits[h >> 6] >> (h & 63) & 1 == 0 {
                continue;
            }
            let (key, sft0) = ctx.canon(&f, x3);
            let h = (key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & ctx.hmask;
            let mut e = ctx.heads[h];
            while e != u32::MAX {
                let ent = &ctx.ents[e as usize];
                e = ctx.next[e as usize];
                if ent.0 != key {
                    continue;
                }
                let [ei, ej, el] = ent.1;
                let [em, en, sg, se] = ent.2;
                let (s1, s2) = (sg / 2, sg % 2);
                let sft = (sft0 + 3 - se as usize) % 3;
                let fa = ctx.reps[ei as usize];
                let fb_ = ctx.cand[3 * ej as usize + em as usize];
                let fb_ = if s1 == 1 { c.neg(fb_) } else { fb_ };
                let fc_ = ctx.cand[3 * el as usize + en as usize];
                let fc_ = if s2 == 1 { c.neg(fc_) } else { fc_ };
                let Some(p2) = c.add(Some(fa), Some(fb_)) else {
                    continue;
                };
                let Some(p3) = c.add(Some(p2), Some(fc_)) else {
                    continue;
                };
                let y3 = f.sub(f.mul(l, f.sub(r.x, x3)), r.y);
                let tau = if y3 == p3.y { 1i8 } else { -1 };
                let sgn = |v: i8, neg: bool| if neg { -v } else { v };
                let (ei, ej, el) = (ei as usize, ej as usize, el as usize);
                let (em, en) = (em as usize, en as usize);
                let terms: Terms4 = [
                    (id / 3, id % 3, eps),
                    (ei, sft, tau),
                    (ej, (sft + em) % 3, sgn(tau, s1 == 1)),
                    (el, (sft + en) % 3, sgn(tau, s2 == 1)),
                ];
                let mut acc: Option<Pt> = None;
                for &(ri, kk, sv) in &terms {
                    let pt = ctx.cand[3 * ri + kk];
                    acc = c.add(acc, Some(if sv > 0 { pt } else { c.neg(pt) }));
                }
                if acc == Some(r) {
                    found.push(terms);
                    if found.len() >= 16 {
                        return;
                    }
                }
            }
        }
    }
}

struct Multi4Out {
    setup_ms: f64,
    build_ms: f64,
    collect_ms: f64,
    per_target_ms: f64,
    total_ms: f64,
    determined: usize,
    solved: usize,
    ok: usize,
    recs: Vec<Option<u64>>,
}

fn multi4(
    c: &FastCurve,
    g: Pt,
    qs: &[Pt],
    ks: &[u64],
    lam: u64,
    b_size: usize,
    extra: usize,
    seed: u64,
) -> Option<Multi4Out> {
    let n = c.n;
    let fnum = c.scalar_field();
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let t0 = Instant::now();
    let ctx = build4(c, g, lam, b_size);
    let build_ms = t0.elapsed().as_secs_f64() * 1e3;
    let mut rng = StdRng::seed_from_u64(seed);
    let needed = b_size + extra;
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();
    let (mut d, mut scratch, mut found) = (Vec::new(), Vec::new(), Vec::new());
    let mut a = rng.gen_range(1..n);
    let mut r = c.mul(Some(g), a)?;
    let mut tries = 0u64;
    while rows.len() < needed && tries < 50_000_000 {
        tries += 1;
        scan4(c, &ctx, r, &mut d, &mut scratch, &mut found);
        let mut productive = false;
        let mut seen: Vec<Vec<u64>> = Vec::new();
        for terms in &found {
            let mut row = vec![0u64; b_size];
            for &(ri, kk, sv) in terms {
                let cv = if sv > 0 { lams[kk] } else { n - lams[kk] } % n;
                row[ri] = fnum.add(row[ri], fnum.to_mont(cv));
            }
            if row.iter().all(|&v| v == 0) || seen.contains(&row) {
                continue;
            }
            seen.push(row.clone());
            rows.push(row);
            rhs.push(fnum.to_mont(a % n));
            productive = true;
            if rows.len() >= needed {
                break;
            }
        }
        if productive {
            a = rng.gen_range(1..n);
            r = c.mul(Some(g), a)?;
        } else {
            match c.add(Some(r), Some(g)) {
                Some(nr) => {
                    r = nr;
                    a = (a + 1) % n;
                }
                None => {
                    a = rng.gen_range(1..n);
                    r = c.mul(Some(g), a)?;
                }
            }
        }
    }
    if rows.len() < needed {
        return None;
    }
    let collect_ms = t0.elapsed().as_secs_f64() * 1e3 - build_ms;
    let nrows = rows.len();
    let cols = b_size;
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
    let mut logs: Vec<Option<u64>> = vec![None; cols];
    let mut determined = 0;
    for col in 0..cols {
        let pr = pivot_row[col];
        if pr == usize::MAX {
            continue;
        }
        if (0..cols).all(|cc| cc == col || rows[pr][cc] == 0) {
            logs[col] = Some(rhs[pr]); // Montgomery form
            determined += 1;
        }
    }
    let setup_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let (mut solved, mut ok) = (0usize, 0usize);
    let mut recs: Vec<Option<u64>> = vec![None; qs.len()];
    for (qi, q) in qs.iter().enumerate() {
        let (mut a, mut b);
        let mut rr;
        loop {
            a = rng.gen_range(1..n);
            b = rng.gen_range(1..n);
            if let Some(p) = c.combine(g, *q, a, b) {
                rr = p;
                break;
            }
        }
        let mut steps = 0u64;
        'target: while steps < 20_000_000 {
            steps += 1;
            scan4(c, &ctx, rr, &mut d, &mut scratch, &mut found);
            for terms in &found {
                if terms.iter().all(|t| logs[t.0].is_some()) {
                    let mut s = 0u64;
                    for &(ri, kk, sv) in terms {
                        let cv = if sv > 0 { lams[kk] } else { n - lams[kk] } % n;
                        s = fnum.add(s, fnum.mul(fnum.to_mont(cv), logs[ri].unwrap()));
                    }
                    let num = fnum.sub(s, fnum.to_mont(a % n));
                    let x = fnum.from_mont(fnum.mul(num, fnum.inv(fnum.to_mont(b % n))));
                    solved += 1;
                    recs[qi] = Some(x);
                    if c.mul(Some(g), x) == Some(*q) && x == ks[qi] {
                        ok += 1;
                    }
                    break 'target;
                }
            }
            match c.add(Some(rr), Some(g)) {
                Some(nr) => {
                    rr = nr;
                    a = (a + 1) % n;
                }
                None => break 'target,
            }
        }
    }
    let per_target_ms = t1.elapsed().as_secs_f64() * 1e3;
    Some(Multi4Out {
        setup_ms,
        build_ms,
        collect_ms,
        per_target_ms,
        total_ms: t0.elapsed().as_secs_f64() * 1e3,
        determined,
        solved,
        ok,
        recs,
    })
}

fn main() {
    // usage: prime_fast_j0_multi4 <out.jsonl> <bits> <B4> <T> <instances> <extra> <c_plain>
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let bsz: usize = a[3].parse().unwrap();
    let t_count: usize = a[4].parse().unwrap();
    let inst: u64 = a[5].parse().unwrap();
    let extra: usize = a[6].parse().unwrap();
    let cplain: f64 = a[7].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    let cs: u64 = std::env::var("CSKIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for curve_idx in cs..cs + 2 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let s = multi4(&c, g, &qs, &ks, lam, bsz, extra, ins + 1);
            let fb = ((cplain * (c.f.p as f64).cbrt()).round() as usize).max(8);
            let t0 = Instant::now();
            let base = ic_multi_solve_ex(&c, g, &qs, fb, 6, 5_000_000, ins + 1, true, true);
            let base_ms = t0.elapsed().as_secs_f64() * 1e3;
            let (bok, base_rec) = match &base {
                Some((logs, _)) => (
                    logs.iter()
                        .zip(&ks)
                        .filter(|(l, k)| **l == Some(**k))
                        .count(),
                    Some(logs.clone()),
                ),
                None => (0, None),
            };
            let row = match &s {
                Some(s) => {
                    json!({"bits": bits, "curve": c.name, "T": t_count, "inst": ins, "B4": bsz, "base_fb": fb,
                    "sym4_total_ms": s.total_ms, "sym4_setup_ms": s.setup_ms, "sym4_build_ms": s.build_ms, "sym4_collect_ms": s.collect_ms,
                    "sym4_per_target_ms": s.per_target_ms, "sym4_determined": s.determined, "sym4_solved": s.solved, "sym4_ok": s.ok,
                    "base_total_ms": base_ms, "base_ok": bok,
                    "cert": {"g": [c.canonical(g).0, c.canonical(g).1], "k": ks,
                        "q": qs.iter().map(|q| { let z = c.canonical(*q); [z.0, z.1] }).collect::<Vec<_>>(),
                        "p": c.f.p, "n": c.n, "sym_rec": s.recs, "base_rec": base_rec}})
                }
                None => {
                    json!({"bits": bits, "curve": c.name, "T": t_count, "inst": ins, "B4": bsz, "sym4_failed": true,
                    "base_total_ms": base_ms, "base_ok": bok})
                }
            };
            writeln!(out, "{row}").unwrap();
            println!("{row}");
        }
    }
}
