// Pollard rho with negation map on the binary curves (matched baseline).
use crate::model::*;
use crate::util::*;
use std::collections::HashMap;

fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % n;
        }
        a = a * a % n;
        e >>= 1;
    }
    r
}

// DEVIATION D-02: x-coordinates of G have structured low bits in the polynomial basis (Tr(x)=Tr(a) pins bit 0 for trinomials),
// so 'low bits of x' gives no distinguished point. Branch index and DP test use bits of a multiplicative hash of x (one integer multiply).
#[inline(always)]
fn hx(x: u64) -> u64 {
    x.wrapping_mul(0x9E3779B97F4A7C15) >> 32
}

pub struct RhoOut {
    pub g: u64,
    pub nat: u64,
    pub lookups: u64,
    pub steps: u64,
    pub walks: u64,
    pub solved: bool,
    pub dps: u64,
}

pub fn rho_instance(md: &BinModel, p: &Pt, q: &Pt, rng: &mut Rng) -> RhoOut {
    let n = md.n;
    let mut c = Cnt::default();
    let mut nat = 0u64;
    let mut lookups = 0u64;
    let mut steps = 0u64;
    let mut walks = 0u64;
    let mut dps = 0u64;
    // 16 branch points; D-03: 12-bit table scalars (cheap setup)
    let mut tab = vec![];
    for _ in 0..16 {
        let a = 1 + rng.below(4095u64.min(n - 1));
        let b = 1 + rng.below(4095u64.min(n - 1));
        let pa = smul(md, &mut c, a, p);
        let qb = smul(md, &mut c, b, q);
        tab.push((a, b, md.add(&mut c, &pa, &qb)));
    }
    let dpbits = ((n as f64).log2() / 4.0).round() as u32;
    let mask = (1u64 << dpbits) - 1;
    let cap = 20u64 << dpbits;
    let mut dpt: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut solved = false;
    'outer: loop {
        walks += 1;
        // D-03: cheap randomized start = sum of 6 random table points
        let mut a = 0u64;
        let mut b = 0u64;
        let mut r = INF;
        for _ in 0..6 {
            let j = rng.below(16) as usize;
            r = md.add(&mut c, &r, &tab[j].2);
            a = (a + tab[j].0) % n;
            b = (b + tab[j].1) % n;
        }
        if r.inf {
            continue;
        }
        if r.y > r.x ^ r.y {
            r = md.neg(&r);
            a = (n - a) % n;
            b = (n - b) % n;
        }
        let mut hist = [u64::MAX; 4]; // x history: hist[0]=1 step back .. hist[3]=4 steps back
        for _ in 0..cap {
            let j = (hx(r.x) & 15) as usize;
            nat += 1; // branch index
            r = md.add(&mut c, &r, &tab[j].2);
            a = (a + tab[j].0) % n;
            b = (b + tab[j].1) % n;
            steps += 1;
            if r.inf {
                continue 'outer;
            }
            nat += 1; // canonicalisation compare
            if r.y > r.x ^ r.y {
                r = md.neg(&r);
                a = (n - a) % n;
                b = (n - b) % n;
            }
            nat += 2; // D-03: fruitless 2- and 4-cycle tests (x equals the x two or four steps back) -> escape by doubling
            if r.x == hist[1] || r.x == hist[3] {
                r = md.add(&mut c, &r, &r.clone());
                a = (2 * a) % n;
                b = (2 * b) % n;
                if r.inf {
                    continue 'outer;
                }
                if r.y > r.x ^ r.y {
                    r = md.neg(&r);
                    a = (n - a) % n;
                    b = (n - b) % n;
                }
                hist = [u64::MAX; 4];
            }
            hist = [r.x, hist[0], hist[1], hist[2]];
            nat += 1; // DP test
            if (hx(r.x) >> 4) & mask == 0 {
                dps += 1;
                lookups += 1;
                nat += 1;
                match dpt.get(&r.x) {
                    Some(&(a2, b2)) => {
                        if a2 == a && b2 == b {
                            continue 'outer; // trivial cycle
                        }
                        if b2 == b {
                            continue 'outer;
                        }
                        // (a-a2) P = (b2-b) d P
                        let num = (a + n - a2) % n;
                        let den = (b2 + n - b) % n;
                        let d = num * powmod(den, n - 2, n) % n;
                        let chk = smul(md, &mut c, d, p);
                        if chk == *q {
                            solved = true;
                            break 'outer;
                        }
                        continue 'outer;
                    }
                    None => {
                        dpt.insert(r.x, (a, b));
                    }
                }
            }
        }
    }
    RhoOut { g: c.g, nat, lookups, steps, walks, solved, dps }
}

pub fn rho_job(c: &BinCurve, ci: usize, ninst: usize) -> Rec {
    let k = c.k;
    let mut rv = rng_for("V", k, 2, ci, 0);
    let md = BinModel::new(c, Vset::rand(k, 2, &mut rv, None)); // V unused by rho
    let mut rp = rng_for("rhoP", k, 0, ci, 0);
    let mut dummy = Cnt::default();
    let p;
    loop {
        let x = 1 + rp.below((1u64 << k) - 1);
        if let Some(pt) = md.lift(&mut dummy, x) {
            let mut c0 = Cnt::default();
            let p2 = smul(&md, &mut c0, 2, &pt);
            if !p2.inf {
                p = p2;
                break;
            }
        }
    }
    let (mut sg, mut sn, mut sl, mut ss, mut sw, mut ok, mut sd) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
    let mut gs = vec![];
    for inst in 0..ninst {
        let mut rd = rng_for("dlp", k, 7, ci, inst);
        let d = 1 + rd.below(md.n - 1);
        let mut c0 = Cnt::default();
        let q = smul(&md, &mut c0, d, &p);
        let mut rw = rng_for("walk", k, 0, ci, inst);
        let o = rho_instance(&md, &p, &q, &mut rw);
        sg += o.g as f64;
        sn += o.nat as f64;
        sl += o.lookups as f64;
        ss += o.steps as f64;
        sw += o.walks as f64;
        sd += o.dps as f64;
        if o.solved {
            ok += 1.0;
        }
        gs.push(o.g as f64);
    }
    let ni = ninst as f64;
    let mut r = Rec::default();
    r.put("k", k as f64);
    r.put("curve", ci as f64);
    r.put("n", md.n as f64);
    r.put("ninst", ni);
    r.put("solved", ok);
    r.put("g_mean", sg / ni);
    r.put("nat_mean", sn / ni);
    r.put("lookups_mean", sl / ni);
    r.put("steps_mean", ss / ni);
    r.put("walks_mean", sw / ni);
    r.put("dps_mean", sd / ni);
    r.put("sqrt_pi_n_over_2", (std::f64::consts::PI * md.n as f64).sqrt() / 2.0);
    r
}
