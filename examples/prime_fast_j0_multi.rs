//! Multi-target j=0 symmetric factor base (setup against G once, one relation per target) vs ic_multi_solve_ex.
//! usage: prime_fast_j0_multi <out.jsonl> <bits> <B> <T> <inst> <extra> <c_plain>
//! (header of the single-target example follows)
//!
//! Curve y^2 = x^3 + b over F_p, p = 1 mod 3, prime order n.  phi(x,y) = (beta x, y) = [lambda]P.
//! Symmetric arm: B orbit representatives, 6B covered points, B unknowns.
//! Plain arm (`ic_solve`, 2 summands): 3B x-coordinates covered, 3B unknowns.
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

type Term = (usize, usize, i8); // (rep, rotation k, sign)

struct Ctx<'a> {
    rot: bool,
    c: &'a FastCurve,
    reps: Vec<Pt>,
    cand: Vec<Pt>,
    tab: XTable,
    ents: Vec<(u64, u32, u32, u8, u8, u8)>,
    heads: Vec<u32>,
    next: Vec<u32>,
    hmask: usize,
    beta: u64,
    beta2: u64,
}

impl<'a> Ctx<'a> {
    fn canon(&self, x3: u64) -> (u64, usize) {
        let f = self.c.f;
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
    fn build(c: &'a FastCurve, g: Pt, lam: u64, b_size: usize, rot: bool) -> Self {
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
        let mut cand = Vec::with_capacity(3 * b_size);
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
        let mut me = Ctx {
            rot,
            c,
            reps,
            cand,
            tab,
            ents: vec![],
            heads: vec![],
            next: vec![],
            hmask: 0,
            beta,
            beta2,
        };
        let mut ents = Vec::new();
        let mut dd = Vec::new();
        let mut sc = Vec::new();
        for i in 0..b_size {
            let fi = me.reps[i];
            if let Some(dbl) = c.double(fi) {
                if rot {
                    let mut a1 = dbl.x;
                    for t2 in 0..3u8 {
                        ents.push((a1, i as u32, i as u32, 0, 0, t2));
                        a1 = f.mul(beta, a1);
                    }
                } else {
                    let (kk, se) = me.canon(dbl.x);
                    ents.push((kk, i as u32, i as u32, 0, 0, se as u8));
                }
            }
            for j in i..b_size {
                dd.clear();
                let ms: &[usize] = if j == i { &[1, 2] } else { &[0, 1, 2] };
                for &m in ms {
                    let dx = f.sub(me.cand[3 * j + m].x, fi.x);
                    dd.push(if dx == 0 { f.one } else { dx });
                }
                f.batch_inv(&mut dd, &mut sc);
                for (t, &m) in ms.iter().enumerate() {
                    let qj = me.cand[3 * j + m];
                    if f.sub(qj.x, fi.x) == 0 {
                        continue;
                    }
                    let sx = c.add_x_with_inv(fi, qj, dd[t]);
                    let dx2 = c.add_x_with_inv(fi, c.neg(qj), dd[t]);
                    if rot {
                        let mut a1 = sx;
                        let mut a2 = dx2;
                        for t2 in 0..3u8 {
                            ents.push((a1, i as u32, j as u32, m as u8, 0, t2));
                            ents.push((a2, i as u32, j as u32, m as u8, 1, t2));
                            a1 = f.mul(beta, a1);
                            a2 = f.mul(beta, a2);
                        }
                    } else {
                        let (k1, s1) = me.canon(sx);
                        ents.push((k1, i as u32, j as u32, m as u8, 0, s1 as u8));
                        let (k2, s2) = me.canon(dx2);
                        ents.push((k2, i as u32, j as u32, m as u8, 1, s2 as u8));
                    }
                }
            }
        }
        let hs = (ents.len() * 2).next_power_of_two().max(16);
        me.hmask = hs - 1;
        me.heads = vec![u32::MAX; hs];
        me.next = vec![u32::MAX; ents.len()];
        for (e, ent) in ents.iter().enumerate() {
            let h = (ent.0.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & me.hmask;
            me.next[e] = me.heads[h];
            me.heads[h] = e as u32;
        }
        me.ents = ents;
        me
    }

    /// All verified decompositions of R into 2 or 3 signed orbit elements.
    fn scan(&self, r: Pt, d: &mut Vec<u64>, scratch: &mut Vec<u64>, out: &mut Vec<Vec<Term>>) {
        let c = self.c;
        let f = c.f;
        out.clear();
        d.resize(self.cand.len(), 0);
        for (i, p) in self.cand.iter().enumerate() {
            let dd = f.sub(r.x, p.x);
            d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(d, scratch);
        for (id, fi) in self.cand.iter().enumerate() {
            if f.sub(r.x, fi.x) == 0 {
                continue;
            }
            let inv = d[id];
            let lm = f.mul(f.add(r.y, fi.y), inv);
            let lp = f.mul(f.sub(r.y, fi.y), inv);
            for (eps, l) in [(1i8, lm), (-1i8, lp)] {
                let x3 = f.sub(f.sub(f.sqr(l), r.x), fi.x);
                let (key, sft0) = if self.rot { (x3, 0) } else { self.canon(x3) };
                let h = (key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & self.hmask;
                let mut e = self.heads[h];
                while e != u32::MAX {
                    let ent = self.ents[e as usize];
                    e = self.next[e as usize];
                    if ent.0 != key {
                        continue;
                    }
                    let (ei, ej, em, sg) = (ent.1 as usize, ent.2 as usize, ent.3 as usize, ent.4);
                    let sft = if self.rot {
                        ent.5 as usize
                    } else {
                        (sft0 + 3 - ent.5 as usize) % 3
                    };
                    let fb_ = self.cand[3 * ej + em];
                    let fb_ = if sg == 1 { c.neg(fb_) } else { fb_ };
                    let Some(pp) = c.add(Some(self.reps[ei]), Some(fb_)) else {
                        continue;
                    };
                    let y3 = f.sub(f.mul(l, f.sub(r.x, x3)), r.y);
                    let tau = if y3 == pp.y { 1i8 } else { -1 };
                    let terms = vec![
                        (id / 3, id % 3, eps),
                        (ei, sft, tau),
                        (ej, (sft + em) % 3, if sg == 1 { -tau } else { tau }),
                    ];
                    let mut acc: Option<Pt> = None;
                    for &(ri, kk, sv) in &terms {
                        let pt = self.cand[3 * ri + kk];
                        acc = c.add(acc, Some(if sv > 0 { pt } else { c.neg(pt) }));
                    }
                    if acc == Some(r) {
                        out.push(terms);
                    }
                }
                if let Some(jid) = self.tab.get(x3) {
                    let jid = jid as usize;
                    if jid >= id {
                        let y3 = f.sub(f.mul(l, f.sub(r.x, x3)), r.y);
                        let tau = if y3 == self.cand[jid].y { 1i8 } else { -1 };
                        let terms = vec![(id / 3, id % 3, eps), (jid / 3, jid % 3, tau)];
                        let mut acc: Option<Pt> = None;
                        for &(ri, kk, sv) in &terms {
                            let pt = self.cand[3 * ri + kk];
                            acc = c.add(acc, Some(if sv > 0 { pt } else { c.neg(pt) }));
                        }
                        if acc == Some(r) {
                            out.push(terms);
                        }
                    }
                }
            }
        }
    }
}

fn coef(n: u64, lams: &[u64; 3], t: &Term) -> u64 {
    if t.2 > 0 {
        lams[t.1]
    } else {
        (n - lams[t.1]) % n
    }
}

struct MultiOut {
    setup_ms: f64,
    pair_ms: f64,
    collect_ms: f64,
    per_target_ms: f64,
    total_ms: f64,
    determined: usize,
    solved: usize,
    ok: usize,
}

fn sym_multi(
    c: &FastCurve,
    g: Pt,
    qs: &[Pt],
    ks: &[u64],
    lam: u64,
    b_size: usize,
    extra: usize,
    seed: u64,
) -> MultiOut {
    let n = c.n;
    let fnum = c.scalar_field();
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let t0 = Instant::now();
    let ctx = Ctx::build(c, g, lam, b_size, std::env::var_os("ROTKEYS").is_some());
    let pair_ms = t0.elapsed().as_secs_f64() * 1e3;
    let mut rng = StdRng::seed_from_u64(seed);
    let needed = b_size + extra;
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();
    let (mut d, mut scratch, mut found) = (Vec::new(), Vec::new(), Vec::new());
    let mut a = rng.gen_range(1..n);
    let mut r = c.mul(Some(g), a).unwrap();
    let mut seen: Vec<Vec<u64>> = Vec::new();
    while rows.len() < needed {
        ctx.scan(r, &mut d, &mut scratch, &mut found);
        seen.clear();
        let mut productive = false;
        for terms in &found {
            let mut row = vec![0u64; b_size];
            for t in terms {
                row[t.0] = fnum.add(row[t.0], fnum.to_mont(coef(n, &lams, t)));
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
            r = c.mul(Some(g), a).unwrap();
        } else {
            match c.add(Some(r), Some(g)) {
                Some(nr) => {
                    r = nr;
                    a = (a + 1) % n;
                }
                None => {
                    a = rng.gen_range(1..n);
                    r = c.mul(Some(g), a).unwrap();
                }
            }
        }
    }
    let collect_ms = t0.elapsed().as_secs_f64() * 1e3 - pair_ms;
    // Gauss-Jordan; a log is determined when its pivot row has no other nonzero entry.
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
    let mut solved = 0;
    let mut ok = 0;
    for (qi, q) in qs.iter().enumerate() {
        let mut tries = 0u64;
        'target: loop {
            tries += 1;
            if tries > 5_000_000 {
                break;
            }
            let aa = rng.gen_range(1..n);
            let bb = rng.gen_range(1..n);
            let Some(mut rr) = c.combine(g, *q, aa, bb) else {
                continue;
            };
            let mut a_eff = aa;
            for _ in 0..4000 {
                ctx.scan(rr, &mut d, &mut scratch, &mut found);
                for terms in &found {
                    if terms.iter().all(|t| logs[t.0].is_some()) {
                        let mut s = 0u64;
                        for t in terms {
                            s = fnum.add(
                                s,
                                fnum.mul(fnum.to_mont(coef(n, &lams, t)), logs[t.0].unwrap()),
                            );
                        }
                        let num = fnum.sub(s, fnum.to_mont(a_eff % n));
                        let x = fnum.from_mont(fnum.mul(num, fnum.inv(fnum.to_mont(bb % n))));
                        solved += 1;
                        if c.mul(Some(g), x) == Some(*q) && x == ks[qi] {
                            ok += 1;
                        }
                        break 'target;
                    }
                }
                match c.add(Some(rr), Some(g)) {
                    Some(nr) => {
                        rr = nr;
                        a_eff = (a_eff + 1) % n;
                    }
                    None => break,
                }
            }
        }
    }
    MultiOut {
        setup_ms,
        pair_ms,
        collect_ms,
        per_target_ms: t1.elapsed().as_secs_f64() * 1e3,
        total_ms: t0.elapsed().as_secs_f64() * 1e3,
        determined,
        solved,
        ok,
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let bsz: usize = a[3].parse().unwrap();
    let t_count: usize = a[4].parse().unwrap();
    let inst: u64 = a[5].parse().unwrap();
    let extra: usize = a[6].parse().unwrap();
    let cplain: f64 = a[7].parse().unwrap();
    let cs: u64 = std::env::var("CSKIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let mut out = std::fs::File::create(&a[1]).unwrap();
    for curve_idx in cs..cs + 2 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let s = sym_multi(&c, g, &qs, &ks, lam, bsz, extra, ins + 1);
            let fb = ((cplain * (c.f.p as f64).cbrt()).round() as usize).max(8);
            let t0 = Instant::now();
            let base = ic_multi_solve_ex(&c, g, &qs, fb, 6, 5_000_000, ins + 1, true, true);
            let base_ms = t0.elapsed().as_secs_f64() * 1e3;
            let (bok, bsetup) = match &base {
                Some((logs, rep)) => (
                    logs.iter()
                        .zip(&ks)
                        .filter(|(l, k)| **l == Some(**k))
                        .count(),
                    rep.setup_ms,
                ),
                None => (0, f64::NAN),
            };
            let row = json!({"bits": bits, "curve": c.name, "T": t_count, "inst": ins, "B": bsz, "base_fb": fb,
                "sym_total_ms": s.total_ms, "sym_setup_ms": s.setup_ms, "sym_pair_ms": s.pair_ms, "sym_collect_ms": s.collect_ms, "sym_per_target_ms": s.per_target_ms,
                "sym_determined": s.determined, "sym_solved": s.solved, "sym_ok": s.ok,
                "base_total_ms": base_ms, "base_setup_ms": bsetup, "base_ok": bok});
            writeln!(out, "{row}").unwrap();
            println!("{row}");
        }
    }
}
