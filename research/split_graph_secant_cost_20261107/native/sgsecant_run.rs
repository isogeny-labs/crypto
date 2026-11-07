//! Relation collection, independent-row rank (linear algebra mod n), final
//! log recovery, Pollard rho reference, and per-arm run records.
use crate::ec::*;
use crate::ops::*;
use crate::sys::*;
use std::collections::BTreeSet;

pub struct Rank {
    k: usize, // abscissa columns; unknown vector = (u_0..u_{k-1}, lambda); rhs separate
    n: u64,
    rows: Vec<(usize, Vec<u64>)>, // (pivot, row of length k+2), pivot normalized to 1
    pub inconsistent: bool,
}
impl Rank {
    pub fn new(k: usize, n: u64) -> Self {
        Rank { k, n, rows: vec![], inconsistent: false }
    }
    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    pub fn full(&self) -> bool {
        self.rows.len() == self.k + 1
    }
    /// Insert one relation row; returns true if it raised the rank. Charged (mod n).
    pub fn insert(&mut self, mut w: Vec<u64>) -> bool {
        let n = self.n;
        let nc = self.k + 2;
        for (piv, rv) in self.rows.iter() {
            let c = w[*piv];
            if c == 0 {
                continue;
            }
            for i in 0..nc {
                let t = fmul(c, rv[i], n);
                w[i] = fsub(w[i], t, n);
            }
        }
        match w[..self.k + 1].iter().position(|x| *x != 0) {
            None => {
                if w[self.k + 1] != 0 {
                    self.inconsistent = true;
                }
                false
            }
            Some(piv) => {
                let iv = finv(w[piv], n);
                for x in w.iter_mut() {
                    *x = fmul(*x, iv, n);
                }
                self.rows.push((piv, w));
                true
            }
        }
    }
    /// Back-substitute; returns lambda.
    pub fn solve_lambda(&self) -> Option<u64> {
        if !self.full() {
            return None;
        }
        let n = self.n;
        let mut x = vec![0u64; self.k + 1];
        for (piv, rv) in self.rows.iter().rev() {
            // row: x[piv] + sum_{c != piv} rv[c] x[c] = rv[k+1]  (rows are stored as coef.x = rhs)
            let mut acc = rv[self.k + 1];
            for c in 0..self.k + 1 {
                if c != *piv && rv[c] != 0 {
                    let t = fmul(rv[c], x[c], n);
                    acc = fsub(acc, t, n);
                }
            }
            x[*piv] = acc;
        }
        Some(x[self.k])
    }
}
/// Row for sigma_u u_u + sigma_w u_w - b lambda = a  (all mod n).
pub fn relation_row(k: usize, n: u64, pair: (SP, SP), a: u64, b: u64) -> Vec<u64> {
    let mut w = vec![0u64; k + 2];
    for (col, sg) in [pair.0, pair.1] {
        w[col] = if sg > 0 { fadd(w[col], 1, n) } else { fsub(w[col], 1, n) };
    }
    w[k] = fneg(b % n, n);
    w[k + 1] = a % n;
    w
}

#[derive(Clone, Debug, Default)]
pub struct RunRec {
    pub led: Vec<Led>, // per phase
    pub targets: u64,
    pub targets_with_rel: u64,
    pub distinct_rel: u64,
    pub rank: usize,
    pub completed: bool,
    pub censored_systems: u64,
    pub nonzero_dim_systems: u64,
    pub replay_fail: u64,
    pub unresolved: u64,
    pub wrong_lambda: bool,
    pub inconsistent: bool,
    pub handled_degree_max: u32,
    pub dim_sum: u64,
    pub solves: u64,
    pub fallbacks: u64,
    pub exact_mismatch: u64, // roots vs exhaustive reference (uncharged check)
}

pub struct Inst<'a> {
    pub c: &'a Curve,
    pub g: Pt,
    pub h: Pt,
    pub lambda: u64,
}

/// One relation-collection run for one arm. Counters are snapshot-diffed so the
/// record contains only this run's work.
pub fn run_arm(
    inst: &Inst,
    b: &Base,
    pres: Pres,
    seed_words: &[u64],
    tmax: u64,
    dup_solve: bool,
    check_exact: bool,
    solve_budget: u64,
) -> RunRec {
    let c = inst.c;
    let k = b.xs.len();
    let n = c.n;
    let mut rank = Rank::new(k, n);
    let mut rec = RunRec::default();
    let s0 = snapshot();
    let mut seen: BTreeSet<((usize, i8), (usize, i8), u64)> = BTreeSet::new(); // (pair, target index) not needed across targets
    let _ = &mut seen;
    let mut j = 0u64;
    while !rank.full() && j < tmax {
        // target R = a G + b H  (charged to the target phase)
        phase(PH_TARGET);
        let mut ws = seed_words.to_vec();
        ws.push(j);
        let hh = hash_words("sgsecant/v1/target", &ws);
        let a = 1 + word(&hh[..8]) % (n - 1);
        let bb = 1 + word(&hh[8..16]) % (n - 1);
        let ag = smul(c, a, inst.g);
        let bh = smul(c, bb, inst.h);
        let rt = add(c, ag, bh);
        j += 1;
        rec.targets += 1;
        let Some(rt) = rt else { continue };
        if rt.1 == 0 {
            continue;
        }
        set_budget(solve_budget);
        let sol = solve(c, b, rt, pres);
        if dup_solve {
            let _ = solve(c, b, rt, pres);
        }
        set_budget(0);
        rec.censored_systems += sol.censored as u64;
        rec.nonzero_dim_systems += sol.not_zero_dim as u64;
        rec.unresolved += sol.unresolved as u64;
        rec.replay_fail += sol.replay_fail as u64;
        rec.handled_degree_max = rec.handled_degree_max.max(sol.handled_degree);
        rec.dim_sum += sol.dim_sum as u64;
        rec.solves += sol.solves as u64;
        rec.fallbacks += sol.fallbacks as u64;
        if check_exact {
            // uncharged exhaustive reference (plain arithmetic, no counters)
            if !sol.censored {
                let rf = reference_set(c, b, rt);
                let got: BTreeSet<(SP, SP)> = sol.pairs.clone();
                if rf != got {
                    rec.exact_mismatch += 1;
                }
            }
        }
        // distinct unordered relations
        phase(PH_DEDUP);
        let mut uniq: BTreeSet<(SP, SP)> = BTreeSet::new();
        for (p1, q1) in sol.pairs.iter() {
            native_n(1);
            let (x, y) = if (p1.0, p1.1) <= (q1.0, q1.1) { (*p1, *q1) } else { (*q1, *p1) };
            uniq.insert((x, y));
        }
        if !uniq.is_empty() {
            rec.targets_with_rel += 1;
        }
        phase(PH_LA);
        for pr in uniq {
            rec.distinct_rel += 1;
            if !rank.full() {
                let row = relation_row(k, n, pr, a, bb);
                rank.insert(row);
            }
        }
    }
    phase(PH_LA);
    if let Some(l) = rank.solve_lambda() {
        // verify lambda * G == H (charged)
        phase(PH_VERIFY);
        let q = smul(c, l, inst.g);
        rec.wrong_lambda = q != Some(inst.h);
    }
    rec.inconsistent = rank.inconsistent;
    rec.rank = rank.rank();
    rec.completed = rank.full() && !rec.wrong_lambda;
    phase(PH_CONSTRUCT);
    let s1 = snapshot();
    rec.led = diff(&s1, &s0).to_vec();
    rec
}

/// Pollard rho (r-adding walk, Floyd) on (G,H); counted in PH_RHO. Returns (ledger, steps, ok).
pub fn rho(c: &Curve, g: Pt, h: Pt, lambda: u64, seed_words: &[u64]) -> (Led, u64, bool) {
    let n = c.n;
    let p = c.p;
    let s0 = snapshot();
    let old = phase(PH_RHO);
    let mut steps = 0u64;
    let mut ok = false;
    'attempt: for attempt in 0..8u64 {
        let r = 16usize;
        let mut tab: Vec<(Pt, u64, u64)> = Vec::new();
        for i in 0..r {
            let mut ws = seed_words.to_vec();
            ws.extend_from_slice(&[attempt, i as u64]);
            let hh = hash_words("sgsecant/v1/rho", &ws);
            let a = 1 + word(&hh[..8]) % (n - 1);
            let b = 1 + word(&hh[8..16]) % (n - 1);
            let q = add(c, smul(c, a, g), smul(c, b, h)).unwrap();
            tab.push((q, a, b));
        }
        let mut ws = seed_words.to_vec();
        ws.push(attempt);
        let hh = hash_words("sgsecant/v1/rho-start", &ws);
        let a0 = 1 + word(&hh[..8]) % (n - 1);
        let b0 = 1 + word(&hh[8..16]) % (n - 1);
        let start = add(c, smul(c, a0, g), smul(c, b0, h));
        let Some(st) = start else { continue };
        let step = |x: (Pt, u64, u64)| -> Option<(Pt, u64, u64)> {
            native_n(1);
            let idx = (x.0 .0 as usize) % r;
            let q = add(c, Some(x.0), Some(tab[idx].0))?;
            Some((q, fadd(x.1, tab[idx].1, n), fadd(x.2, tab[idx].2, n)))
        };
        let (mut t, mut hr) = ((st, a0, b0), (st, a0, b0));
        let lim = 40 * (((n as f64).sqrt() as u64) + 8);
        let mut cnt = 0u64;
        loop {
            let Some(t1) = step(t) else { continue 'attempt };
            let Some(h1) = step(hr) else { continue 'attempt };
            let Some(h2) = step(h1) else { continue 'attempt };
            t = t1;
            hr = h2;
            cnt += 1;
            steps += 3;
            if t.0 == hr.0 {
                break;
            }
            if cnt > lim {
                continue 'attempt;
            }
        }
        let da = fsub(t.1, hr.1, n);
        let db = fsub(hr.2, t.2, n);
        if db == 0 {
            continue;
        }
        let l = fmul(da, finv(db, n), n);
        let _ = p;
        if l == lambda {
            ok = true;
            break;
        }
    }
    phase(old);
    let s1 = snapshot();
    (total(&diff(&s1, &s0)), steps, ok)
}
