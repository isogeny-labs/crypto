// Per-curve jobs for binary and prime arms cells (H1-H5), including H2 census.
use crate::model::*;
use crate::solve::*;
use crate::util::*;
use std::time::Instant;

pub fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub struct JobOut {
    pub rec: Rec,
    pub samples: Vec<String>,
}

fn twist_count<M: Model>(md: &M) -> u64 {
    md.star().iter().filter(|&&x| !md.is_e(x)).count() as u64
}

// walk-based target stream for a binary curve (instance `inst`): setup counted in cc
pub struct BinStream {
    pub n: u64,
    pub p: Pt,
    pub q: Pt,
    pub d: u64,
    table: Vec<(u64, u64, Pt)>,
    r: Pt,
    a: u64,
    b: u64,
    rt: Rng,
}
impl BinStream {
    pub fn new(md: &BinModel, k: u32, m: u32, ci: usize, inst: usize, cc: &mut Cnt) -> BinStream {
        let n = md.n;
        let q = 1u64 << k;
        let mut rd = rng_for("dlp", k, m, ci, inst);
        let mut dummy = Cnt::default();
        let p;
        loop {
            let x = 1 + rd.below(q - 1);
            if let Some(pt) = md.lift(&mut dummy, x) {
                let p2 = smul(md, cc, 2, &pt);
                if !p2.inf {
                    p = p2;
                    break;
                }
            }
        }
        let d = 1 + rd.below(n - 1);
        let qq = smul(md, cc, d, &p);
        let mut table = vec![];
        for _ in 0..64 {
            let a = 1 + rd.below(n - 1);
            let b = 1 + rd.below(n - 1);
            let pa = smul(md, cc, a, &p);
            let qb = smul(md, cc, b, &qq);
            let w = md.add(cc, &pa, &qb);
            table.push((a, b, w));
        }
        let (a, b) = (1 + rd.below(n - 1), 1 + rd.below(n - 1));
        let pa = smul(md, cc, a, &p);
        let qb = smul(md, cc, b, &qq);
        let r = md.add(cc, &pa, &qb);
        BinStream { n, p, q: qq, d, table, r, a, b, rt: rng_for("target", k, m, ci, inst) }
    }
    pub fn next(&mut self, md: &BinModel, cc: &mut Cnt) -> Target {
        loop {
            let i = self.rt.below(64) as usize;
            self.r = md.add(cc, &self.r, &self.table[i].2);
            self.a = (self.a + self.table[i].0) % self.n;
            self.b = (self.b + self.table[i].1) % self.n;
            if self.r.inf || self.r.x == 0 {
                continue;
            }
            return Target { xr: self.r.x, rpt: self.r, a: self.a, b: self.b };
        }
    }
}

pub fn binary_targets(md: &BinModel, k: u32, m: u32, ci: usize, nt: usize, cc: &mut Cnt) -> (Vec<Target>, Pt, Pt, u64) {
    let mut st = BinStream::new(md, k, m, ci, 0, cc);
    let mut out = vec![];
    for _ in 0..nt {
        out.push(st.next(md, cc));
    }
    (out, st.p, st.q, st.d)
}

pub fn z_of(f: f64, mu: f64, s: f64) -> f64 {
    let sd = (mu * (1.0 - mu) / s).sqrt();
    (f - (1.0 - mu)) / sd
}

pub fn binary_job(m: usize, curve: &BinCurve, ci: usize, nt: usize, census_n: usize) -> JobOut {
    let k = curve.k;
    let l = ceil_div(k, m as u32);
    let t0 = Instant::now();
    let mut rv = rng_for("V", k, m as u32, ci, 0);
    let vtype = if ci % 2 == 0 { 0.0 } else { 1.0 };
    let v = if ci % 2 == 0 { Vset::rand(k, l, &mut rv, None) } else { Vset::poly(k, l) };
    let md = BinModel::new(curve, v);
    let mut cc = Cnt::default();
    let mut st = BinStream::new(&md, k, m as u32, ci, 0, &mut cc);
    let setup_g = cc.g;
    let targets: Vec<Target> = (0..nt).map(|_| st.next(&md, &mut cc)).collect();
    let tag = format!("B {} {}", k, ci);
    let ao = run_arms(&md, m, &targets, &tag, if ci < 2 { 100 } else { 0 }, true, false);
    let wall = t0.elapsed().as_secs_f64();
    let s = md.star().len() as f64;
    let tw = twist_count(&md) as f64;
    let mu = md.mu_e();
    let z_arm = z_of(tw / s, mu, s);
    // census over random V
    let (mut czn, mut czs, mut czs2, mut cfs) = (0f64, 0f64, 0f64, 0f64);
    for j in 0..census_n {
        let mut rc = rng_for("Vc", k, m as u32, ci, j);
        let vc = Vset::rand(k, l, &mut rc, None);
        let mc = md.with_v(vc);
        let f = twist_count(&mc) as f64 / s;
        let z = z_of(f, mu, s);
        czn += 1.0;
        czs += z;
        czs2 += z * z;
        cfs += f;
    }
    let mut rec = Rec::default();
    rec.put("field", 0.0);
    rec.put("k", k as f64);
    rec.put("m", m as f64);
    rec.put("curve", ci as f64);
    rec.put("l", l as f64);
    rec.put("vtype", vtype);
    rec.put("ne", curve.ne as f64);
    rec.put("n", curve.n as f64);
    rec.put("mu", mu);
    rec.put("s", s);
    rec.put("tw_arm", tw);
    rec.put("z_arm", z_arm);
    rec.put("cz_n", czn);
    rec.put("cz_sum", czs);
    rec.put("cz_sum2", czs2);
    rec.put("cf_sum", cfs);
    rec.put("ve", ao.ve.len() as f64);
    rec.put("wall_s", wall);
    rec.put("setup_g", setup_g as f64);
    rec.put_cnt("u", &ao.cu);
    rec.put_cnt("f", &ao.cf);
    rec.put_cnt("s", &ao.cs);
    rec.put_cnt("l", &ao.cl);
    rec.put_cnt("c", &cc);
    JobOut { rec, samples: ao.samples }
}

pub fn prime_targets(md: &PrimeModel, k: u32, m: u32, ci: usize, nt: usize) -> Vec<Target> {
    let mut rt = rng_for("ptarget", k, m, ci, 0);
    let mut out = vec![];
    let mut d = Cnt::default();
    while out.len() < nt {
        let x = 1 + rt.below(md.p - 1);
        if let Some(pt) = md.lift(&mut d, x) {
            out.push(Target { xr: x, rpt: pt, a: 0, b: 0 });
        }
    }
    out
}

pub fn prime_job(m: usize, curve: &PrimeCurve, ci: usize, nt: usize, census_n: usize) -> JobOut {
    let k = curve.k;
    let l = ceil_div(k, m as u32);
    let t0 = Instant::now();
    let mut rv = rng_for("pV", k, m as u32, ci, 0);
    let md = PrimeModel::new(curve, l, &mut rv, None);
    let targets = prime_targets(&md, k, m as u32, ci, nt);
    let tag = format!("P {} {}", k, ci);
    let ao = run_arms(&md, m, &targets, &tag, if ci < 2 { 100 } else { 0 }, true, false);
    let wall = t0.elapsed().as_secs_f64();
    let s = md.star().len() as f64;
    let tw = twist_count(&md) as f64;
    let mu = md.mu_e();
    let z_arm = z_of(tw / s, mu, s);
    let (mut czn, mut czs, mut czs2, mut cfs) = (0f64, 0f64, 0f64, 0f64);
    for j in 0..census_n {
        let mut rc = rng_for("pVc", k, m as u32, ci, j);
        let mc = PrimeModel::new(curve, l, &mut rc, None);
        let f = twist_count(&mc) as f64 / s;
        let z = z_of(f, mu, s);
        czn += 1.0;
        czs += z;
        czs2 += z * z;
        cfs += f;
    }
    let mut rec = Rec::default();
    rec.put("field", 1.0);
    rec.put("k", k as f64);
    rec.put("m", m as f64);
    rec.put("curve", ci as f64);
    rec.put("l", l as f64);
    rec.put("vtype", 0.0);
    rec.put("ne", curve.n as f64);
    rec.put("n", curve.n as f64);
    rec.put("mu", mu);
    rec.put("s", s);
    rec.put("tw_arm", tw);
    rec.put("z_arm", z_arm);
    rec.put("cz_n", czn);
    rec.put("cz_sum", czs);
    rec.put("cz_sum2", czs2);
    rec.put("cf_sum", cfs);
    rec.put("ve", ao.ve.len() as f64);
    rec.put("wall_s", wall);
    rec.put("setup_g", 0.0);
    rec.put_cnt("u", &ao.cu);
    rec.put_cnt("f", &ao.cf);
    rec.put_cnt("s", &ao.cs);
    rec.put_cnt("l", &ao.cl);
    rec.put_cnt("c", &Cnt::default());
    JobOut { rec, samples: ao.samples }
}
