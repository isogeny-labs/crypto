// Positive controls PC-A..PC-D and the prime/binary twist-target machinery.
use crate::cells::*;
use crate::model::*;
use crate::solve::*;
use crate::util::*;

fn bin_model(c: &BinCurve, m: u32, ci: usize) -> BinModel {
    let l = ceil_div(c.k, m);
    let mut rv = rng_for("V", c.k, m, ci, 0);
    let v = if ci % 2 == 0 { Vset::rand(c.k, l, &mut rv, None) } else { Vset::poly(c.k, l) };
    BinModel::new(c, v)
}
fn prime_model(c: &PrimeCurve, m: u32, ci: usize) -> PrimeModel {
    let l = ceil_div(c.k, m);
    let mut rv = rng_for("pV", c.k, m, ci, 0);
    PrimeModel::new(c, l, &mut rv, None)
}

// PC-A: classifier vs explicit lift in Rust; writes (x label) for the GP ellordinate check.
pub fn pca(dir: &str, bc: &[BinCurve], pc: &[PrimeCurve]) -> Vec<Rec> {
    std::fs::create_dir_all(format!("{}/results/pca", dir)).unwrap();
    let mut out = vec![];
    for &k in &[17u32, 19, 23, 29, 31] {
        for field in 0..2 {
            let mut txt = String::new();
            let (mut n, mut agree) = (0u64, 0u64);
            let mut rng = rng_for("pca", k, field as u32, 0, 0);
            let mut d = Cnt::default();
            if field == 0 {
                let c = bc.iter().find(|c| c.k == k && c.idx == 0).unwrap();
                let md = bin_model(c, 2, 0);
                for _ in 0..10000 {
                    let x = 1 + rng.below((1u64 << k) - 1);
                    let lab = md.is_e(x);
                    let lf = md.lift(&mut d, x).is_some();
                    n += 1;
                    if lab == lf {
                        agree += 1;
                    }
                    txt.push_str(&format!("{} {}\n", x, lab as u8));
                }
            } else {
                let c = pc.iter().find(|c| c.k == k && c.idx == 0).unwrap();
                let md = prime_model(c, 2, 0);
                for _ in 0..10000 {
                    let x = 1 + rng.below(c.p - 1);
                    let lab = md.is_e(x);
                    let lf = md.lift(&mut d, x).is_some();
                    n += 1;
                    if lab == lf {
                        agree += 1;
                    }
                    txt.push_str(&format!("{} {}\n", x, lab as u8));
                }
            }
            std::fs::write(format!("{}/results/pca/{}_k{}.txt", dir, if field == 0 { "B" } else { "P" }, k), txt).unwrap();
            let mut r = Rec::default();
            r.put("field", field as f64);
            r.put("k", k as f64);
            r.put("n", n as f64);
            r.put("agree_lift", agree as f64);
            out.push(r);
        }
    }
    out
}

fn twist_xs<M: Model>(md: &M, rng: &mut Rng, nt: usize, modulus: u64) -> Vec<u64> {
    let mut v = vec![];
    while v.len() < nt {
        let x = 1 + rng.below(modulus - 1);
        if md.is_e(x) || md.in_v_free(x) || md.sb_over(x).map_or(false, |y| md.in_v_free(y)) {
            continue;
        }
        v.push(x);
    }
    v
}

// PC-B: twist-type targets. returns per-curve records and samples
pub fn pcb_job(field: u32, m: usize, k: u32, ci: usize, nt: usize, bc: &[BinCurve], pc: &[PrimeCurve]) -> (Rec, Vec<String>) {
    let mut rng = rng_for("pcb", k, m as u32, ci, field as usize);
    let (to, tag);
    if field == 0 {
        let c = bc.iter().find(|c| c.k == k && c.idx == ci).unwrap();
        let md = bin_model(c, m as u32, ci);
        let xs = twist_xs(&md, &mut rng, nt, 1u64 << k);
        tag = format!("B {} {}", k, ci);
        to = run_twist_targets(&md, m, &xs, &tag, if ci == 0 { 40 } else { 0 });
    } else {
        let c = pc.iter().find(|c| c.k == k && c.idx == ci).unwrap();
        let md = prime_model(c, m as u32, ci);
        let xs = twist_xs(&md, &mut rng, nt, c.p);
        tag = format!("P {} {}", k, ci);
        to = run_twist_targets(&md, m, &xs, &tag, if ci == 0 { 40 } else { 0 });
    }
    let mut r = Rec::default();
    r.put("field", field as f64);
    r.put("k", k as f64);
    r.put("m", m as f64);
    r.put("curve", ci as f64);
    r.put("targets", nt as f64);
    r.put("nsol", to.nsol as f64);
    r.put("n_all_e", to.n_all_e as f64);
    r.put("n_mixed", to.n_mixed as f64);
    r.put("n_all_tw", to.n_all_tw as f64);
    r.put_cnt("c", &to.c);
    (r, to.samples)
}

fn collect_unique<M: Model>(md: &M, m: usize, xs: &[u64]) -> Vec<(u64, [u64; 3])> {
    let mut c = Cnt::default();
    let mut out = vec![];
    for &xr in xs {
        for s in dedup(solve(md, &mut c, m, md.star(), xr)) {
            out.push((xr, s));
        }
    }
    out
}

// PC-C: injection control on stored U solutions
pub fn pcc(bc: &[BinCurve], pc: &[PrimeCurve]) -> Vec<Rec> {
    let mut out = vec![];
    for field in 0..2u32 {
        for &(m, k, nt) in &[(2usize, 17u32, 1000usize), (3, 17, 1500)] {
            let mut rng = rng_for("inject", k, m as u32, 0, field as usize);
            let (sols, twpool, flagfn): (Vec<(u64, [u64; 3])>, Vec<u64>, Box<dyn Fn(u64) -> bool>);
            // build solutions and a pool of twist-type elements, plus a classifier closure
            if field == 0 {
                let c = bc.iter().find(|c| c.k == k && c.idx == 0).unwrap().clone();
                let md = std::sync::Arc::new(bin_model(&c, m as u32, 0));
                let mut r2 = rng_for("pcc_t", k, m as u32, 0, 0);
                let mut xs = vec![];
                let mut d = Cnt::default();
                while xs.len() < nt {
                    let x = 1 + r2.below((1u64 << k) - 1);
                    if md.lift(&mut d, x).is_some() && !md.in_v_free(x) && !md.sb_over(x).map_or(false, |y| md.in_v_free(y)) {
                        xs.push(x);
                    }
                }
                sols = collect_unique(&*md, m, &xs);
                twpool = twist_xs(&*md, &mut r2, 4000, 1u64 << k);
                let md2 = md.clone();
                flagfn = Box::new(move |x| !md2.is_e(x));
            } else {
                let c = pc.iter().find(|c| c.k == k && c.idx == 0).unwrap().clone();
                let md = std::sync::Arc::new(prime_model(&c, m as u32, 0));
                let mut r2 = rng_for("pcc_t", k, m as u32, 0, 1);
                let mut xs = vec![];
                while xs.len() < nt {
                    let x = 1 + r2.below(c.p - 1);
                    if md.is_e(x) && !md.in_v_free(x) {
                        xs.push(x);
                    }
                }
                sols = collect_unique(&*md, m, &xs);
                twpool = twist_xs(&*md, &mut r2, 4000, c.p);
                let md2 = md.clone();
                flagfn = Box::new(move |x| !md2.is_e(x));
            }
            let nrec = sols.len();
            for &rho in &[0.01f64, 0.05, 0.25] {
                let ninj = (rho * nrec as f64).round() as usize;
                // seeded selection of ninj distinct records
                let mut idx: Vec<usize> = (0..nrec).collect();
                for i in 0..ninj.min(nrec) {
                    let j = i + rng.below((nrec - i) as u64) as usize;
                    idx.swap(i, j);
                }
                let mut inj = vec![false; nrec];
                let mut recs: Vec<[u64; 3]> = sols.iter().map(|s| s.1).collect();
                for &i in idx.iter().take(ninj) {
                    inj[i] = true;
                    let ci = rng.below(m as u64) as usize;
                    recs[i][ci] = twpool[rng.below(twpool.len() as u64) as usize];
                }
                let (mut tp, mut fp, mut fnn, mut tn) = (0u64, 0u64, 0u64, 0u64);
                for i in 0..nrec {
                    let flagged = (0..m).any(|j| flagfn(recs[i][j]));
                    match (inj[i], flagged) {
                        (true, true) => tp += 1,
                        (true, false) => fnn += 1,
                        (false, true) => fp += 1,
                        (false, false) => tn += 1,
                    }
                }
                let mut r = Rec::default();
                r.put("field", field as f64);
                r.put("m", m as f64);
                r.put("k", k as f64);
                r.put("rho", rho);
                r.put("nrec", nrec as f64);
                r.put("ninj", ninj as f64);
                r.put("tp", tp as f64);
                r.put("fp", fp as f64);
                r.put("fn", fnn as f64);
                r.put("tn", tn as f64);
                r.put("observed_waste", (tp + fp) as f64 / nrec as f64);
                r.put("expected_waste", ninj as f64 / nrec as f64);
                out.push(r);
            }
        }
    }
    out
}

// PC-D: degenerate family, m = 3, xR forced into V
pub fn pcd_job(field: u32, k: u32, ci: usize, bc: &[BinCurve], pc: &[PrimeCurve]) -> Rec {
    let m = 3usize;
    let l = ceil_div(k, 3);
    let (mut vtw_sum, mut mix_d, mut mix_nd, mut pure, mut alle, mut skipped, mut ntargets, mut exact_ok) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    if field == 0 {
        let c = bc.iter().find(|c| c.k == k && c.idx == ci).unwrap();
        let md0 = bin_model(c, 3, ci);
        let mut cc = Cnt::default();
        let (tg, _, _, _) = binary_targets(&md0, k, 33, ci, 60, &mut cc);
        let mut t = 0;
        for tgt in tg.iter() {
            if ntargets >= 20 {
                break;
            }
            let mut rv = rng_for("Vd", k, 3, ci, t);
            t += 1;
            let v = Vset::rand(k, l, &mut rv, Some(tgt.xr));
            let md = md0.with_v(v);
            if md.in_v_free(md.sb_over(tgt.xr).unwrap()) {
                skipped += 1;
                continue;
            }
            ntargets += 1;
            let vtw = md.star().iter().filter(|&&x| !md.is_e(x)).count() as u64;
            let mut cn = Cnt::default();
            let sols = dedup(solve(&md, &mut cn, m, md.star(), tgt.xr));
            let (mut md_, mut mnd, mut pu, mut ae) = (0u64, 0u64, 0u64, 0u64);
            for s in &sols {
                let ntw = (0..m).filter(|&i| !md.is_e(s[i])).count();
                let d = dmatch(&md, s, m, tgt.xr);
                if ntw == 0 {
                    ae += 1;
                } else if ntw == m {
                    pu += 1;
                } else if d {
                    md_ += 1;
                } else {
                    mnd += 1;
                }
            }
            vtw_sum += vtw;
            mix_d += md_;
            mix_nd += mnd;
            pure += pu;
            alle += ae;
            if md_ == vtw && mnd == 0 && pu == 0 {
                exact_ok += 1;
            }
        }
    } else {
        let c = pc.iter().find(|c| c.k == k && c.idx == ci).unwrap();
        let mdp0 = prime_model(c, 3, ci);
        let tg = prime_targets(&mdp0, k, 33, ci, 20);
        for (t, tgt) in tg.iter().enumerate() {
            ntargets += 1;
            let mut rv = rng_for("pVd", k, 3, ci, t);
            let md = PrimeModel::new(c, l, &mut rv, Some(tgt.xr));
            let vtw = md.star().iter().filter(|&&x| !md.is_e(x)).count() as u64;
            let mut cn = Cnt::default();
            let sols = dedup(solve(&md, &mut cn, m, md.star(), tgt.xr));
            let (mut md_, mut mnd, mut pu, mut ae) = (0u64, 0u64, 0u64, 0u64);
            for s in &sols {
                let ntw = (0..m).filter(|&i| !md.is_e(s[i])).count();
                let d = dmatch(&md, s, m, tgt.xr);
                if ntw == 0 {
                    ae += 1;
                } else if ntw == m {
                    pu += 1;
                } else if d {
                    md_ += 1;
                } else {
                    mnd += 1;
                }
            }
            vtw_sum += vtw;
            mix_d += md_;
            mix_nd += mnd;
            pure += pu;
            alle += ae;
            if md_ == vtw && mnd == 0 && pu == 0 {
                exact_ok += 1;
            }
        }
    }
    let mut r = Rec::default();
    r.put("field", field as f64);
    r.put("k", k as f64);
    r.put("curve", ci as f64);
    r.put("targets", ntargets as f64);
    r.put("skipped_sb", skipped as f64);
    r.put("vtw_sum", vtw_sum as f64);
    r.put("mixed_D", mix_d as f64);
    r.put("mixed_nonD", mix_nd as f64);
    r.put("pure_tw", pure as f64);
    r.put("all_e", alle as f64);
    r.put("targets_exact", exact_ok as f64);
    r
}
