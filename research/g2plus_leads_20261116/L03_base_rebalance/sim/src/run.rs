use std::collections::{HashMap, HashSet};

use crate::jac::{GroupOps, Instance, JDiv};
use crate::la::{
    addm, build_rows, mulmod, peel, subm, wiedemann, Dsu, Nat, Rel, REL_FB,
};
use crate::rel::{disc_character, roots_of, splits_completely};
use crate::rng::{mix64, Rng};

pub const GENUS: u32 = 3;
pub const RESTART: u64 = 4096;
pub const MAX_TRIALS: u64 = 60_000_000;
pub fn grid() -> Vec<f64> {
    (-6..=4).map(|k| 2.0f64.powf(k as f64 / 2.0)).collect()
}
pub fn fb_size(p: u64, cc: f64) -> usize {
    let nb = (cc * (p as f64).powf(1.0 - 1.0 / 3.0)).ceil() as usize;
    nb.min(p as usize)
}

#[derive(Clone, Debug, Default)]
pub struct RunOut {
    pub fb_cols: usize,
    pub n_full: u64,
    pub beta: f64,
    pub trials: u64,
    pub candidates: u64,
    pub prefilter_rejects: u64,
    pub splits: u64,
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub dup_relations: u64,
    pub lp_vertices: u64,
    pub cycles_final: i64,
    pub attempts: u32,
    pub core_rows: usize,
    pub core_cols: usize,
    pub core_nnz: u64,
    pub core_nonunit: u64,
    pub core_max_row: usize,
    pub cantor_mul: u64,
    pub cantor_inv: u64,
    pub cm_setup: u64,
    pub ci_setup: u64,
    pub cm_loop: u64,
    pub ci_loop: u64,
    pub cm_verify: u64,
    pub ci_verify: u64,
    pub group_ops_setup: u64,
    pub group_ops_loop: u64,
    pub group_ops_verify: u64,
    pub rel_fixed: Nat,
    pub rel_trial: Nat,
    pub la: Nat,
    pub la_graph_add: u64,
    pub la_peel_add: u64,
    pub la_wied: Nat,
    pub dsu_ops: u64,
    pub walk_fp1000: u64,
    pub recovered: Option<u64>,
    pub t_planted: u64,
    pub certified: bool,
    pub exhausted: bool,
    pub bad_relations: u64,
    pub verified_relations: u64,
}

pub fn run_ic(inst: &Instance, nb: usize, seed_state: u64, verify_relations: bool) -> RunOut {
    let curve = &inst.curve;
    let (p, r, h) = (inst.p, inst.r, inst.h);
    let g = inst.genus as usize;
    assert!(g <= REL_FB);
    let snap = |ops: &GroupOps| (curve.fmul(), curve.finv(), ops.adds + ops.doubles);
    let mut rng = Rng::new(seed_state);
    let mut res = RunOut {
        t_planted: inst.t_secret,
        ..Default::default()
    };
    let mut ops = GroupOps::default();
    let s0 = snap(&ops);
    let f = curve.f_coeffs();

    // factor base
    let mut chi = vec![-1i8; p as usize];
    chi[0] = 0;
    for y in 1..=(p - 1) / 2 {
        chi[(y * y % p) as usize] = 1;
    }
    res.rel_fixed.mul += (p - 1) / 2;
    let want = nb;
    let mut col = vec![u32::MAX; p as usize];
    let mut ncols = 0usize;
    for x in 0..p {
        if ncols >= want {
            break;
        }
        let mut fx = 0u64;
        for &cf in f.iter().rev() {
            fx = (fx * x + cf) % p;
        }
        res.rel_fixed.mul += f.len() as u64;
        res.rel_fixed.add += f.len() as u64;
        if chi[fx as usize] == 1 {
            col[x as usize] = ncols as u32;
            ncols += 1;
        }
    }
    res.fb_cols = ncols;
    // M3: n_full, uncounted
    {
        let mut n = 0u64;
        for x in 0..p {
            let mut fx = 0u64;
            for &cf in f.iter().rev() {
                fx = (fx * x + cf) % p;
            }
            if chi[fx as usize] == 1 {
                n += 1;
            }
        }
        res.n_full = n;
        res.beta = ncols as f64 / n as f64;
    }
    let tcol = ncols as u32;
    let nunk = ncols + 1;

    // walk set-up
    let short = r.min(1 << 12);
    let mut jumps: Vec<(JDiv, u64, u64)> = Vec::new();
    for _ in 0..16 {
        let a = rng.range(1, short);
        let b = rng.range(1, short);
        let ag = curve.smul(&mut ops, inst.g, a);
        let bt = curve.smul(&mut ops, inst.target, b);
        jumps.push((curve.add(&mut ops, ag, bt), a, b));
    }
    let s1 = snap(&ops);

    let mut rels: Vec<Rel> = Vec::new();
    let mut vid: HashMap<u32, u32> = HashMap::new();
    let mut dsu = Dsu::new();
    let margin = |n: usize| 8 + n / 100;
    let mut cur = JDiv::IDENTITY;
    let (mut ca, mut cb) = (0u64, 0u64);
    let mut since_restart = u64::MAX;
    let mut next_check = nunk as u64;
    let mut solved = false;
    let mut dupset: HashSet<JDiv> = HashSet::new();
    let mut s2 = s1;
    let mut s3 = s1;

    'outer: while !solved {
        while (rels.len() as u64) < next_check {
            if res.trials >= MAX_TRIALS {
                res.exhausted = true;
                break 'outer;
            }
            if since_restart >= RESTART.max(1) {
                let a = rng.range(1, r);
                let b = rng.range(1, r);
                let ag = curve.smul(&mut ops, inst.g, a);
                let bt = curve.smul(&mut ops, inst.target, b);
                cur = curve.add(&mut ops, ag, bt);
                ca = a;
                cb = b;
                since_restart = 0;
            } else {
                let j = (mix64(curve.key_of(&cur)) >> 24) as usize % 16;
                let (m, aj, bj) = jumps[j];
                cur = curve.add(&mut ops, cur, m);
                ca = addm(ca, aj, r);
                cb = addm(cb, bj, r);
                since_restart += 1;
            }
            res.trials += 1;
            if res.trials <= 1000 {
                res.walk_fp1000 = mix64(res.walk_fp1000 ^ curve.key_of(&cur));
            }
            if cur.deg as usize != g {
                continue;
            }
            res.candidates += 1;
            let u = curve.u_coeffs(&cur);
            let ch = disc_character(p, &u, &chi, &mut res.rel_trial);
            if ch != 1 {
                res.prefilter_rejects += 1;
                continue;
            }
            // g = 3: decided = false
            if !splits_completely(p, &u, &mut res.rel_trial) {
                continue;
            }
            res.splits += 1;
            let Some(roots) = roots_of(p, &u, &mut rng, &mut res.rel_trial) else {
                continue;
            };
            let vc = curve.v_coeffs(&cur);
            let mut rel = Rel {
                fb: [(0, 0); REL_FB],
                nfb: 0,
                lp: [(0, 0); 2],
                nlp: 0,
                tcoef: 0,
                rhs: 0,
            };
            let mut lps: Vec<(u32, i8)> = Vec::new();
            let mut places: Vec<(u64, u64)> = Vec::new();
            for &x in &roots {
                let mut y = 0u64;
                for &cf in vc.iter().rev() {
                    y = (y * x + cf) % p;
                }
                res.rel_trial.mul += vc.len() as u64;
                res.rel_trial.add += vc.len() as u64;
                places.push((x, y));
                if y == 0 {
                    continue;
                }
                let eps: i8 = if y > (p - 1) / 2 { -1 } else { 1 };
                if col[x as usize] != u32::MAX {
                    rel.fb[rel.nfb as usize] = (col[x as usize], eps);
                    rel.nfb += 1;
                } else {
                    lps.push((x as u32, eps));
                }
            }
            if lps.len() > 2 {
                continue;
            }
            if rel.nfb == 0 && lps.is_empty() {
                continue;
            }
            res.rel_trial.mul += 2;
            rel.rhs = mulmod(h % r, ca, r);
            rel.tcoef = subm(0, mulmod(h % r, cb, r), r);
            if verify_relations {
                let mut sum = JDiv::IDENTITY;
                for &(x, y) in &places {
                    let pt = curve.point(x, y).expect("a root of u lies on the curve");
                    sum = curve.cantor(sum, pt);
                }
                if sum != cur {
                    res.bad_relations += 1;
                } else {
                    res.verified_relations += 1;
                }
            }
            for (x, e) in lps {
                let id = *vid.entry(x).or_insert_with(|| dsu.add_vertex());
                rel.lp[rel.nlp as usize] = (id, e);
                rel.nlp += 1;
            }
            let (a, _, b, _) = rel.endpoints();
            dsu.edge(a, b);
            res.rels_by_lp[rel.nlp as usize] += 1;
            res.relations += 1;
            rels.push(rel);
            if !dupset.insert(cur) {
                res.dup_relations += 1;
            }
        }

        res.cycles_final = dsu.cycles0();
        let nver = dsu.parent.len();
        {
            let need = (nunk + margin(nunk)) as i64;
            if res.cycles_final < need {
                next_check = rels.len() as u64 + (need - res.cycles_final).max(1) as u64;
                continue;
            }
        }

        res.attempts += 1;
        let mut g_nat = Nat::default();
        let rows = build_rows(&rels, nver, tcol, r, &mut g_nat);
        let mut p_nat = Nat::default();
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut p_nat);
        res.la_graph_add += g_nat.add;
        res.la_peel_add += p_nat.add;
        res.la.merge(g_nat);
        res.la.merge(p_nat);
        res.core_rows = core.len();
        res.core_cols = cc;
        res.core_nnz = core.iter().map(|(t, _)| t.len() as u64).sum();
        res.core_nonunit = core
            .iter()
            .flat_map(|(t, _)| t.iter())
            .filter(|&&(_, v)| v != 1 && v != r - 1)
            .count() as u64;
        res.core_max_row = core.iter().map(|(t, _)| t.len()).max().unwrap_or(0);
        if core.len() < cc + margin(cc) {
            next_check = rels.len() as u64 + (nunk as u64 / 20).max(8);
            continue;
        }
        let mut w_nat = Nat::default();
        let dense = res.attempts > 1;
        let sol = wiedemann(&core, cc, r, &mut rng, dense, &mut w_nat).map(|x| x[ct as usize]);
        res.la_wied.merge(w_nat);
        res.la.merge(w_nat);
        if let Some(t) = sol {
            s2 = snap(&ops);
            let chk = curve.smul(&mut ops, inst.g, t);
            s3 = snap(&ops);
            let (cm, ci) = (curve.fmul(), curve.finv());
            let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;
            curve.k.mul.set(cm);
            curve.k.inv.set(ci);
            if ok {
                res.recovered = Some(t);
                res.certified = true;
                solved = true;
                continue;
            }
        }
        next_check = rels.len() as u64 + (nunk as u64 / 20).max(16);
    }
    res.la.add += dsu.ops;
    res.dsu_ops = dsu.ops;
    res.lp_vertices = (dsu.parent.len() - 1) as u64;
    if res.certified {
        res.cm_setup = s1.0 - s0.0;
        res.ci_setup = s1.1 - s0.1;
        res.cm_loop = s2.0 - s1.0;
        res.ci_loop = s2.1 - s1.1;
        res.cm_verify = s3.0 - s2.0;
        res.ci_verify = s3.1 - s2.1;
        res.group_ops_setup = s1.2 - s0.2;
        res.group_ops_loop = s2.2 - s1.2;
        res.group_ops_verify = s3.2 - s2.2;
    } else {
        // exhausted: no certificate; whole walk counted in the loop
        let se = snap(&ops);
        res.cm_setup = s1.0 - s0.0;
        res.ci_setup = s1.1 - s0.1;
        res.cm_loop = se.0 - s1.0;
        res.ci_loop = se.1 - s1.1;
        res.group_ops_setup = s1.2 - s0.2;
        res.group_ops_loop = se.2 - s1.2;
    }
    res.cantor_mul = res.cm_setup + res.cm_loop + res.cm_verify;
    res.cantor_inv = res.ci_setup + res.ci_loop + res.ci_verify;
    if verify_relations {
        res.cantor_mul = 0;
        res.cantor_inv = 0;
        res.cm_setup = 0;
        res.ci_setup = 0;
        res.cm_loop = 0;
        res.ci_loop = 0;
        res.cm_verify = 0;
        res.ci_verify = 0;
    }
    res
}
