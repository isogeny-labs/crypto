//! `covariates`: the splitcount.rs statistic (same conditioned GL2 baseline)
//! plus division-polynomial root counts.
//!
//! The per-prime model is copied from research/split_prime_ecdlp_20261107/
//! tools/splitcount.rs `classify`: Frobenius mod l uniform in GL2(F_l) with
//! determinant p, conditioned on l not dividing n (the trace class tau = p+1
//! is removed and the rest renormalised). Differences from splitcount.rs, all
//! forced by small p: p and n are machine integers (not hex), and an l that
//! divides p (only possible when p <= 200000) is skipped like an l dividing n.
use crate::curves::{load, CurveRec};
use crate::poly::*;
use crate::util::*;

pub const CUTOFFS: &[u64] = &[100, 200, 500, 691, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000];

pub struct PerL {
    pub l: u64,
    pub p_mod: u64,
    pub ps: f64,
    pub pr: f64,
    pub pi: f64,
    pub log_split: f64,
    pub log_inert: f64,
}

/// Baseline probabilities for odd prime l given p mod l (splitcount.rs `classify`).
pub fn baseline(l: u64, p_mod: u64) -> (f64, f64, f64) {
    let lf = l as f64;
    let chi_p = legendre(p_mod, l) as f64;
    let ps = (lf - 2.0 - chi_p) / (2.0 * (lf - 1.0));
    let pr = (1.0 + chi_p) * lf / (lf * lf - 1.0);
    let p_is_one = p_mod % l == 1;
    let m = (lf + if p_is_one { 0.0 } else { 1.0 }) / (lf * lf - 1.0);
    let (ps_c, pr_c) = if p_is_one { (ps / (1.0 - m), (pr - m) / (1.0 - m)) } else { ((ps - m) / (1.0 - m), pr / (1.0 - m)) };
    (ps_c, pr_c, 1.0 - ps_c - pr_c)
}

pub fn tables(p: u64) -> Vec<PerL> {
    primes_up_to(200000)
        .into_iter()
        .filter(|&l| l > 2 && p % l != 0)
        .map(|l| {
            let p_mod = p % l;
            let (ps, pr, pi) = baseline(l, p_mod);
            let lf = l as f64;
            PerL { l, p_mod, ps, pr, pi, log_split: -(1.0 - 1.0 / lf).ln(), log_inert: -(1.0 + 1.0 / lf).ln() }
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
pub struct CutStat {
    pub cutoff: u64,
    pub primes: u64,
    pub split: u64,
    pub exp_split: f64,
    pub sd: f64,
    pub z: f64,
    pub log_l1_ratio: f64,
}

/// Statistic at every cutoff for one curve (p, n).
pub fn stats(p: u64, n: u64, tabs: &[PerL]) -> Vec<CutStat> {
    let t = p as i64 + 1 - n as i64;
    let t2 = (t as i128 * t as i128) as i128;
    let mut out = vec![];
    let (mut cnt, mut obs, mut es, mut var, mut ll, mut ell) = (0u64, 0u64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut k = 0;
    for r in tabs {
        while k < CUTOFFS.len() && r.l > CUTOFFS[k] {
            out.push(finish(CUTOFFS[k], cnt, obs, es, var, ll, ell));
            k += 1;
        }
        if n % r.l == 0 {
            continue;
        }
        let disc = (t2 - 4 * p as i128).rem_euclid(r.l as i128) as u64;
        let chi = legendre(disc, r.l);
        cnt += 1;
        if chi == 1 {
            obs += 1;
        }
        es += r.ps;
        var += r.ps * (1.0 - r.ps);
        let lf = r.l as f64;
        ll += -(1.0 - chi as f64 / lf).ln();
        ell += r.ps * r.log_split + r.pi * r.log_inert;
    }
    while k < CUTOFFS.len() {
        out.push(finish(CUTOFFS[k], cnt, obs, es, var, ll, ell));
        k += 1;
    }
    out
}
fn finish(cutoff: u64, cnt: u64, obs: u64, es: f64, var: f64, ll: f64, ell: f64) -> CutStat {
    let sd = var.sqrt();
    CutStat { cutoff, primes: cnt, split: obs, exp_split: es, sd, z: (obs as f64 - es) / sd, log_l1_ratio: ll - ell }
}

pub fn cmd(args: &[String]) {
    let curves = load(&opt(args, "--curves").expect("--curves"));
    let mut sink = Sink::open(opt(args, "--out"));
    let p = curves[0].c.p;
    let tabs = tables(p);
    for cr in &curves {
        sink.line(&record(cr, &tabs));
    }
    sink.flush();
}

pub fn record(cr: &CurveRec, tabs: &[PerL]) -> String {
    let p = cr.c.p;
    let st = stats(p, cr.n, tabs);
    let r3 = roots_in_fp(&psi3(cr.c.a, cr.c.b, p), p);
    let r5 = roots_in_fp(&psi5(cr.c.a, cr.c.b, p), p);
    let mut s = format!("{{\"kind\":\"cov\",\"idx\":{},\"p\":{},\"n\":{},\"t\":{}", cr.idx, p, cr.n, cr.t);
    for c in &st {
        s += &format!(",\"E_{}\":{},\"z_{}\":{:.6},\"logL1_{}\":{:.6}", c.cutoff, c.split, c.cutoff, c.z, c.cutoff, c.log_l1_ratio);
    }
    let c691 = st.iter().find(|c| c.cutoff == 691).unwrap();
    s += &format!(",\"exp_691\":{:.6},\"sd_691\":{:.6},\"primes_691\":{}", c691.exp_split, c691.sd, c691.primes);
    s += &format!(",\"psi3_roots\":{},\"psi5_roots\":{}}}", r3, r5);
    s
}
