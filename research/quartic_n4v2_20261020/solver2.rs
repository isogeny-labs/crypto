// N4 v2 pipelines. Family S (sorted bucketing; LA variants S, L0, L1, L) and family D (double
// large primes; LA variants D, DL1, DL). The fb / relations / matrix phases run once per attempt and
// are charged in full to every variant; log recovery, descent and verification run once per distinct
// normalised kernel vector. Downstream logic follows N4's solver::ic_solve.
#![allow(dead_code)]
use crate::common::*;
use crate::ghs::*;
use crate::ic::{split3, Curve, Row, P3};
use crate::ic2::*;
use crate::la2;
use crate::pic;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fam {
    S,
    D,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Var {
    B0Lanczos,
    Pm1Lanczos,
    SgeScalar,
    SgeBlock,
}

#[derive(Clone)]
pub struct Cfg {
    pub fam: Fam,
    pub c: f64,
    pub rows_mult: f64,
    pub gamma: f64,
    pub block: usize,
    pub max_desc: usize,
}

#[derive(Default, Clone)]
pub struct ArmRep {
    pub ok: bool,
    pub fail: String,
    pub k_found: Option<u64>,
    pub phases: Vec<(String, Cnt, f64)>,
    pub stats: Vec<(String, f64)>,
}

fn pc<R>(cv: &pic::PlaneQuartic, f: impl FnOnce() -> R) -> R {
    let b = cv.counter();
    let r = f();
    let d = cv.counter().since(&b);
    bump_mul(d.mul);
    bump_inv(d.inv);
    bump_add(d.add);
    r
}
fn to_u32(p: &P3) -> [u32; 3] {
    [p[0] as u32, p[1] as u32, p[2] as u32]
}

struct Meter {
    mark: (Cnt, std::time::Instant),
}
impl Meter {
    fn new() -> Meter {
        Meter { mark: (snap(), std::time::Instant::now()) }
    }
    fn take(&mut self, name: &str) -> (String, Cnt, f64) {
        let now = snap();
        let r = (name.to_string(), now.since(&self.mark.0), self.mark.1.elapsed().as_secs_f64());
        self.mark = (now, std::time::Instant::now());
        r
    }
}

enum LpData {
    Star(crate::ic::RelOut),
    Graph(Graph, DOut),
}

pub fn run_family(inst: &Inst, cfg: &Cfg, vars: &[Var], k_true: u64, seed: u64, verify_rel: bool) -> Vec<ArmRep> {
    let mut shared: Vec<(String, Cnt, f64)> = Vec::new();
    let mut st: Vec<(String, f64)> = Vec::new();
    let fail_all = |shared: &Vec<(String, Cnt, f64)>, st: &Vec<(String, f64)>, msg: String| -> Vec<ArmRep> {
        vars.iter().map(|_| ArmRep { ok: false, fail: msg.clone(), k_found: None, phases: shared.clone(), stats: st.clone() }).collect()
    };
    let q = inst.q;
    let ell = inst.ell;
    let mut rng = Rng::new(seed ^ 0xC0FFEE);
    let curve = Curve::new(q, &inst.f);
    let f = curve.f;
    let mut m = Meter::new();
    let target = inst.ec.mul(&inst.g, k_true as u128);

    // 1. factor base
    let nfb = match cfg.fam {
        Fam::S => ((cfg.c * (q as f64).powf(0.6)).ceil() as usize).min(q as usize + 1),
        Fam::D => ((cfg.c * (q as f64).sqrt()).ceil() as usize).min(q as usize + 1),
    };
    let mut fb = sample_points2(&curve, nfb, &mut rng);
    let mut cvo = None;
    for i in 0..fb.len() {
        if let Ok(c) = pic::PlaneQuartic::new(q as u32, inst.f, to_u32(&fb[i])) {
            fb.swap(0, i);
            let sc = c.setup_cost();
            bump_mul(sc.mul);
            bump_inv(sc.inv);
            bump_add(sc.add);
            cvo = Some(c);
            break;
        }
    }
    let cv = match cvo {
        Some(c) => c,
        None => {
            shared.push(m.take("fb"));
            return fail_all(&shared, &st, "no base point".into());
        }
    };
    cv.reset_counter();
    let n = fb.len();
    st.push(("n_fb".into(), n as f64));
    shared.push(m.take("fb"));

    // 2. relations
    let ncols_all = n;
    let eta_col = (n - 1) as u32;
    let want = (cfg.rows_mult * ncols_all as f64).ceil() as usize + 30;
    let mut full_rows: Vec<Row>;
    let mut rest: Vec<Row>;
    let lpd: LpData;
    match cfg.fam {
        Fam::S => {
            let (ro, ss) = find_relations_sorted(&curve, &fb);
            st.push(("pairs".into(), ro.pairs as f64));
            st.push(("multi_lines".into(), ro.multi_lines as f64));
            st.push(("rel_full".into(), ro.full.len() as f64));
            st.push(("rel_lp".into(), ro.lp.len() as f64));
            st.push(("lp_distinct".into(), ro.lp_pts.len() as f64));
            st.push(("rel_unsure".into(), ro.unsure as f64));
            st.push(("sort_keys".into(), ss.sort_keys as f64));
            st.push(("sort_cmp".into(), ss.sort_cmp as f64));
            st.push(("dup_skipped".into(), ss.dup_skipped as f64));
            shared.push(m.take("relations"));
            if verify_rel {
                let (t, b) = verify_star(&cv, &fb, &ro);
                st.push(("relations_verified".into(), t as f64));
                st.push(("relations_bad".into(), b as f64));
                cv.reset_counter();
            }
            let (fr, lr) = rows_from_relout(&ro, n, ell);
            st.push(("rows_full".into(), fr.len() as f64));
            st.push(("rows_lp_combined".into(), lr.len() as f64));
            full_rows = fr;
            rest = lr;
            lpd = LpData::Star(ro);
        }
        Fam::D => {
            let d = find_relations_2lp(&curve, &fb);
            st.push(("pairs".into(), d.pairs as f64));
            st.push(("rational_pairs".into(), d.rational_pairs as f64));
            st.push(("rel_full".into(), d.full.len() as f64));
            st.push(("rel_1lp".into(), d.n1lp as f64));
            st.push(("rel_2lp".into(), d.n2lp as f64));
            st.push(("rel_loop".into(), d.nloop as f64));
            st.push(("lp_vertices".into(), d.lp_pts.len() as f64));
            st.push(("dup_skipped".into(), d.dup_skipped as f64));
            st.push(("rel_unsure".into(), d.unsure as f64));
            shared.push(m.take("relations"));
            let mut g = build_graph(&d, n);
            let mut fr: Vec<Row> = Vec::new();
            for r in &d.full {
                let lab = label_of(&r.c, n); // eta - sum = 0  ->  row sum m x - eta = 0
                let row: Row = {
                    let mut v: Vec<(u32, u64)> = lab.iter().map(|&(c, x)| (c, to_mod(-x, ell))).filter(|x| x.1 != 0).collect();
                    v.sort();
                    v
                };
                fr.push(row);
            }
            let need = (want + 60).saturating_sub(fr.len());
            let (cr, anchors, skipped) = cycle_rows(&mut g, &d, need, ell);
            let root_comp = g.order.iter().filter(|&&v| g.troot[v as usize] == 0).count();
            st.push(("graph_components".into(), g.comps as f64));
            st.push(("root_component".into(), root_comp as f64));
            st.push(("anchors".into(), anchors as f64));
            st.push(("cycle_rows".into(), cr.len() as f64));
            st.push(("cycle_rows_empty".into(), skipped as f64));
            st.push(("cycle_row_weight_mean".into(), if cr.is_empty() { 0.0 } else { cr.iter().map(|r| r.len()).sum::<usize>() as f64 / cr.len() as f64 }));
            if verify_rel {
                let (t, b) = verify_d(&cv, &fb, &d, &cr, ell);
                st.push(("relations_verified".into(), t as f64));
                st.push(("relations_bad".into(), b as f64));
                cv.reset_counter();
            }
            st.push(("rows_full".into(), fr.len() as f64));
            full_rows = fr;
            rest = cr;
            lpd = LpData::Graph(g, d);
        }
    }

    // 3. matrix: row selection as B0 (S: 3 full rows first, rest shuffled; D: full rows, then
    // cycle rows in weight order)
    let mut rows: Vec<Row> = Vec::new();
    let mut hold: Vec<Row> = Vec::new();
    match cfg.fam {
        Fam::S => {
            if full_rows.is_empty() {
                shared.push(m.take("matrix"));
                return fail_all(&shared, &st, "no full relation (eta unanchored)".into());
            }
            let mut rr: Vec<Row> = Vec::new();
            for (i, r) in full_rows.drain(..).enumerate() {
                if i < 3 {
                    rows.push(r);
                } else {
                    rr.push(r);
                }
            }
            rr.extend(rest.drain(..));
            for i in (1..rr.len()).rev() {
                let j = rng.below(i as u64 + 1) as usize;
                rr.swap(i, j);
            }
            for r in rr {
                if rows.len() < want {
                    rows.push(r);
                } else if hold.len() < 60 {
                    hold.push(r);
                }
            }
        }
        Fam::D => {
            let mut all: Vec<Row> = full_rows.drain(..).collect();
            all.extend(rest.drain(..));
            for r in all {
                if rows.len() < want {
                    rows.push(r);
                } else if hold.len() < 60 {
                    hold.push(r);
                }
            }
        }
    }
    let mut used = vec![false; ncols_all];
    for r in &rows {
        for &(c, _) in r {
            used[c as usize] = true;
        }
    }
    used[eta_col as usize] = true;
    let mut remap = vec![u32::MAX; ncols_all];
    let mut nact = 0usize;
    for c in 0..ncols_all {
        if used[c] {
            remap[c] = nact as u32;
            nact += 1;
        }
    }
    let rows_c: Vec<Row> = rows.iter().map(|r| r.iter().map(|&(c, a)| (remap[c as usize], a)).collect()).collect();
    st.push(("rows_used".into(), rows_c.len() as f64));
    st.push(("cols_active".into(), nact as f64));
    st.push(("nnz".into(), rows_c.iter().map(|r| r.len()).sum::<usize>() as f64));
    if rows_c.len() < nact {
        shared.push(m.take("matrix"));
        return fail_all(&shared, &st, format!("too few relations: {} rows for {} columns", rows_c.len(), nact));
    }
    let eta_r = remap[eta_col as usize];
    shared.push(m.take("matrix"));

    // 4. linear algebra, per variant
    struct VOut {
        la: (String, Cnt, f64),
        z: Option<Vec<Option<u64>>>,
        fail: String,
        st: Vec<(String, f64)>,
    }
    let mut vouts: Vec<VOut> = Vec::new();
    let rng_la = rng.0;
    for &var in vars {
        let mut lrng = Rng(rng_la);
        let mut vst: Vec<(String, f64)> = Vec::new();
        let mut m2 = Meter::new();
        let mut z: Option<Vec<Option<u64>>> = None;
        for _ in 0..3 {
            let cand: Option<Vec<Option<u64>>> = match var {
                Var::B0Lanczos => la2::lanczos(&rows_c, nact, ell, &mut lrng, false).map(|v| v.into_iter().map(Some).collect()),
                Var::Pm1Lanczos => la2::lanczos(&rows_c, nact, ell, &mut lrng, true).map(|v| v.into_iter().map(Some).collect()),
                Var::SgeScalar | Var::SgeBlock => {
                    let s = la2::sge(&rows_c, nact, eta_r, cfg.gamma, 32, ell);
                    vst.push(("sge_cols".into(), s.keep.len() as f64));
                    vst.push(("sge_rows".into(), s.rows.len() as f64));
                    vst.push(("sge_nnz".into(), s.rows.iter().map(|r| r.len()).sum::<usize>() as f64));
                    vst.push(("sge_singletons".into(), s.singletons as f64));
                    vst.push(("sge_merges".into(), s.merges as f64));
                    vst.push(("sge_dropped".into(), s.dropped as f64));
                    vst.push(("sge_free".into(), s.free.len() as f64));
                    let nr = s.keep.len();
                    if s.rows.len() < nr || nr == 0 {
                        None
                    } else {
                        let zr = if var == Var::SgeScalar {
                            la2::lanczos(&s.rows, nr, ell, &mut lrng, true)
                        } else {
                            let mut bs = la2::BlStats { iters: 0, narrowed: 0 };
                            let r = la2::block_lanczos(&s.rows, nr, ell, cfg.block, &mut lrng, &mut bs);
                            vst.push(("bl_iters".into(), bs.iters as f64));
                            vst.push(("bl_narrowed".into(), bs.narrowed as f64));
                            r
                        };
                        zr.map(|zr| la2::back_substitute(&s, &zr, nact, ell))
                    }
                }
            };
            if let Some(v) = cand {
                // verify A z = 0 on every row whose columns are all determined
                let mut bad = false;
                let mut unver = 0;
                for r in &rows_c {
                    if r.iter().all(|&(c, _)| v[c as usize].is_some()) {
                        let zz: Vec<u64> = Vec::new();
                        let _ = zz;
                        let mut s = 0u64;
                        for &(c, a) in r {
                            s = la2(s, lm2(a, v[c as usize].unwrap(), ell), ell);
                        }
                        if s != 0 {
                            bad = true;
                            break;
                        }
                    } else {
                        unver += 1;
                    }
                }
                if !bad && v[eta_r as usize].map(|e| e != 0).unwrap_or(false) {
                    vst.push(("rows_unverifiable".into(), unver as f64));
                    z = Some(v);
                    break;
                }
            }
        }
        let mut fail = String::new();
        let zn = match z {
            None => {
                fail = "linear algebra failed".into();
                None
            }
            Some(v) => {
                // hold-out rows
                let (mut hok, mut hbad) = (0, 0);
                for r in &hold {
                    if r.iter().all(|&(c, _)| remap[c as usize] != u32::MAX && v[remap[c as usize] as usize].is_some()) {
                        let mut s = 0u64;
                        for &(c, a) in r {
                            s = la2(s, lm2(a, v[remap[c as usize] as usize].unwrap(), ell), ell);
                        }
                        if s == 0 {
                            hok += 1;
                        } else {
                            hbad += 1;
                        }
                    }
                }
                vst.push(("holdout_ok".into(), hok as f64));
                vst.push(("holdout_bad".into(), hbad as f64));
                vst.push(("cols_known".into(), v.iter().filter(|x| x.is_some()).count() as f64));
                if hbad > 0 {
                    fail = "kernel not unique (hold-out relations violated)".into();
                    None
                } else {
                    let ei = linv(v[eta_r as usize].unwrap(), ell);
                    Some(v.into_iter().map(|x| x.map(|y| lm2(y, ei, ell))).collect::<Vec<Option<u64>>>())
                }
            }
        };
        let la = m2.take("linear_algebra");
        vouts.push(VOut { la, z: zn, fail, st: vst });
    }
    let rng_after = Rng(rng_la ^ 0x5EED);

    // 5. downstream per distinct vector
    let mut reps: Vec<ArmRep> = Vec::new();
    let mut groups: Vec<(Vec<Option<u64>>, Vec<(String, Cnt, f64)>, Option<u64>, bool, String, Vec<(String, f64)>)> = Vec::new();
    for vo in &vouts {
        let mut rep = ArmRep { ok: false, fail: vo.fail.clone(), k_found: None, phases: shared.clone(), stats: st.clone() };
        rep.phases.push(vo.la.clone());
        rep.stats.extend(vo.st.iter().cloned());
        let z = match &vo.z {
            None => {
                reps.push(rep);
                continue;
            }
            Some(z) => z,
        };
        let gi = match groups.iter().position(|g| &g.0 == z) {
            Some(i) => {
                rep.stats.push(("downstream_shared".into(), 1.0));
                i
            }
            None => {
                let (ph, kf, ok, fail, dst) = downstream(inst, &cv, &curve, &fb, &used, &remap, z, &lpd, &target, rng_after.0, cfg.max_desc, n);
                groups.push((z.clone(), ph, kf, ok, fail, dst));
                rep.stats.push(("downstream_shared".into(), 0.0));
                groups.len() - 1
            }
        };
        let g = &groups[gi];
        rep.phases.extend(g.1.iter().cloned());
        rep.k_found = g.2;
        rep.ok = g.3;
        rep.fail = g.4.clone();
        rep.stats.extend(g.5.iter().cloned());
        reps.push(rep);
    }
    let _ = f;
    reps
}

fn downstream(
    inst: &Inst,
    cv: &pic::PlaneQuartic,
    curve: &Curve,
    fb: &[P3],
    used: &[bool],
    remap: &[u32],
    z: &[Option<u64>],
    lpd: &LpData,
    target: &EP,
    rseed: u64,
    max_desc: usize,
    n: usize,
) -> (Vec<(String, Cnt, f64)>, Option<u64>, bool, String, Vec<(String, f64)>) {
    let ell = inst.ell;
    let f = curve.f;
    let mut rng = Rng(rseed);
    let mut ph = Vec::new();
    let mut st = Vec::new();
    let mut m = Meter::new();
    let eta = 1u64; // normalised
    let xcol = |c: u32| -> Option<u64> {
        if remap[c as usize] == u32::MAX {
            None
        } else {
            z[remap[c as usize] as usize]
        }
    };
    let mut logs: HashMap<u64, u64> = HashMap::new();
    let mut active_idx: Vec<u32> = Vec::new();
    logs.insert(pack2(&fb[0]), 0);
    bump_hash(1);
    for i in 1..n {
        let c = (i - 1) as u32;
        if used[c as usize] {
            if let Some(v) = xcol(c) {
                logs.insert(pack2(&fb[i]), v);
                bump_hash(1);
                active_idx.push(i as u32);
            }
        }
    }
    let mut lp_known = 0u64;
    match lpd {
        LpData::Star(ro) => {
            let mut seen: std::collections::HashSet<u32> = std::collections::HashSet::new();
            for r in &ro.lp {
                let l = r.lp.unwrap();
                if !seen.insert(l) {
                    continue;
                }
                let mut s = 0u64;
                let mut okr = true;
                for &(i, mu) in &r.c {
                    bump_hash(1);
                    match logs.get(&pack2(&fb[i as usize])) {
                        Some(&v) => s = la2(s, lm2(mu as u64, v, ell), ell),
                        None => {
                            okr = false;
                            break;
                        }
                    }
                }
                if okr {
                    bump_hash(1);
                    logs.insert(pack2(&ro.lp_pts[l as usize]), ls2(eta, s, ell));
                    lp_known += 1;
                }
            }
        }
        LpData::Graph(g, d) => {
            let x = propagate_logs(g, d, &xcol, ell);
            for v in 1..g.nv {
                if let Some(val) = x[v] {
                    bump_hash(1);
                    logs.insert(pack2(&d.lp_pts[v - 1]), val);
                    lp_known += 1;
                }
            }
        }
    }
    st.push(("lp_known".to_string(), lp_known as f64));
    st.push(("known_points".to_string(), logs.len() as f64));
    ph.push(m.take("log_recovery"));

    // descent (N4's)
    let mons2 = pic::mons_of_degree(2);
    let mut cls_cache: HashMap<u32, pic::PicClass> = HashMap::new();
    let mut total_trials = 0u64;
    let mut ys: Vec<u64> = Vec::new();
    let mut rtries = 0;
    let mut desc_ok = true;
    let fail_ret = |ph: Vec<(String, Cnt, f64)>, st: Vec<(String, f64)>, msg: &str| (ph, None, false, msg.to_string(), st);
    if active_idx.is_empty() {
        ph.push(m.take("descent"));
        return fail_ret(ph, st, "no known factor-base logs");
    }
    'outer: loop {
        rtries += 1;
        if rtries > 400 {
            ph.push(m.take("descent"));
            return fail_ret(ph, st, "no valid transfer triple");
        }
        let r = rng.below(ell - 1) + 1;
        let r0 = inst.ec.mul(&inst.g, r as u128);
        let r1 = inst.ec.add(&r0, &inst.g);
        let r2 = inst.ec.add(&r0, target);
        let mut rows3 = Vec::new();
        for p in [r0, r1, r2] {
            match inst.transfer_cubics(&p) {
                Some(c) => rows3.push(c),
                None => continue 'outer,
            }
        }
        ys.clear();
        for rows in rows3 {
            let mut x = pc(cv, || cv.reduce_deg6(&pic::Gen { k: 3, rows }));
            let mut added: Vec<u32> = Vec::new();
            let mut got = None;
            for _t in 0..max_desc {
                total_trials += 1;
                if let pic::PicClass::Typ(mm) = &x {
                    if let Some(pts) = split3(&f, mm, &mons2, &mut rng) {
                        bump_hash(3);
                        let l: Vec<Option<u64>> = pts.iter().map(|p| logs.get(&pack2(p)).copied()).collect();
                        if l.iter().all(|v| v.is_some()) {
                            let mut s = 0u64;
                            for v in l {
                                s = la2(s, v.unwrap(), ell);
                            }
                            s = ls2(s, eta, ell);
                            for &j in &added {
                                bump_hash(1);
                                s = ls2(s, logs[&pack2(&fb[j as usize])], ell);
                            }
                            got = Some(s);
                            break;
                        }
                    }
                }
                let j = active_idx[rng.below(active_idx.len() as u64) as usize];
                let cj = cls_cache.entry(j).or_insert_with(|| pc(cv, || cv.point_class(to_u32(&fb[j as usize])))).clone();
                x = pc(cv, || cv.add(&x, &cj));
                added.push(j);
            }
            match got {
                Some(v) => ys.push(v),
                None => {
                    desc_ok = false;
                    break 'outer;
                }
            }
        }
        break;
    }
    st.push(("descent_trials".to_string(), total_trials as f64));
    ph.push(m.take("descent"));
    if !desc_ok {
        return fail_ret(ph, st, "descent exhausted");
    }
    let den = ls2(ys[1], ys[0], ell);
    if den == 0 {
        ph.push(m.take("verify"));
        return fail_ret(ph, st, "degenerate denominator");
    }
    let kk = lm2(ls2(ys[2], ys[0], ell), inv_mod_u64(den, ell), ell);
    let chk = inst.ec.mul(&inst.g, kk as u128);
    let ok = chk == *target;
    ph.push(m.take("verify"));
    (ph, Some(kk), ok, if ok { String::new() } else { "recovered k does not satisfy kG = Q".into() }, st)
}

/// B0's relation check (10 full, 30 large-prime relations sum to one class), uncharged.
fn verify_star(cv: &pic::PlaneQuartic, fb: &[P3], ro: &crate::ic::RelOut) -> (u64, u64) {
    let mut kappa: Option<pic::PicClass> = None;
    let (mut bad, mut tested) = (0, 0);
    let cls = |p: &P3| cv.point_class(to_u32(p));
    let mut allrels: Vec<(Vec<(u32, u8)>, Option<P3>)> = Vec::new();
    for r in ro.full.iter().take(10) {
        allrels.push((r.c.clone(), None));
    }
    for r in ro.lp.iter().take(30) {
        allrels.push((r.c.clone(), r.lp.map(|l| ro.lp_pts[l as usize])));
    }
    for (c, lp) in allrels {
        let mut s = cv.zero();
        for (i, mm) in c {
            let x = cls(&fb[i as usize]);
            for _ in 0..mm {
                s = cv.add(&s, &x);
            }
        }
        if let Some(l) = lp {
            s = cv.add(&s, &cls(&l));
        }
        match &kappa {
            None => kappa = Some(s),
            Some(k0) => {
                if *k0 != s {
                    bad += 1;
                }
            }
        }
        tested += 1;
    }
    (tested, bad)
}

/// D's check: 10 full and 30 large-prime line relations sum to one class kappa; 10 cycle rows give
/// the zero class (sum c_i [P_i - P0] + c_eta kappa = 0), uncharged.
fn verify_d(cv: &pic::PlaneQuartic, fb: &[P3], d: &DOut, cr: &[Row], ell: u64) -> (u64, u64) {
    let cls = |p: &P3| cv.point_class(to_u32(p));
    let (mut bad, mut tested) = (0u64, 0u64);
    let mut kappa: Option<pic::PicClass> = None;
    let mut rels: Vec<&DRel> = d.full.iter().take(10).collect();
    rels.extend(d.edges.iter().take(30));
    for r in rels {
        let mut s = cv.zero();
        for &(i, mm) in &r.c {
            let x = cls(&fb[i as usize]);
            for _ in 0..mm {
                s = cv.add(&s, &x);
            }
        }
        for v in [r.u, r.v] {
            if v != 0 && v != u32::MAX {
                s = cv.add(&s, &cls(&d.lp_pts[v as usize - 1]));
            }
        }
        if r.u == r.v && r.u != u32::MAX && r.u != 0 {
            // self-loop: the vertex appears twice (already added twice above)
        }
        match &kappa {
            None => kappa = Some(s),
            Some(k0) => {
                if *k0 != s {
                    bad += 1;
                }
            }
        }
        tested += 1;
    }
    let kap = match kappa {
        Some(k) => k,
        None => return (tested, bad),
    };
    let n = fb.len();
    for r in cr.iter().take(10) {
        let mut pos = cv.zero();
        let mut neg = cv.zero();
        for &(c, a) in r {
            let (sgn, mag) = if a > ell / 2 { (-1, ell - a) } else { (1, a) };
            let x = if c as usize == n - 1 { kap.clone() } else { cls(&fb[c as usize + 1]) };
            let y = cv.mul_scalar(&x, mag as u128);
            if sgn > 0 {
                pos = cv.add(&pos, &y);
            } else {
                neg = cv.add(&neg, &y);
            }
        }
        if pos != neg {
            bad += 1;
        }
        tested += 1;
    }
    (tested, bad)
}
