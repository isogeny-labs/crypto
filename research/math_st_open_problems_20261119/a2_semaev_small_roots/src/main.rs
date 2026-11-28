//! A2 (P1.2/Q004): small roots of Semaev summation polynomials modulo p via
//! Coppersmith / Howgrave-Graham lattices. See ../REPORT.md.
mod copper;
mod dd;
mod fl;
mod fp;
mod fq;
mod lll;
mod semaev;

use copper::{QP, ZPoly};
use fp::*;
use num_bigint::BigInt;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use semaev::MP;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Poly { S3, S4 }
impl Poly {
    fn name(&self) -> &'static str { match self { Poly::S3 => "S3", Poly::S4 => "S4" } }
    fn nu(&self) -> usize { match self { Poly::S3 => 2, Poly::S4 => 3 } }
    fn box_d(&self) -> [u8; 3] { match self { Poly::S3 => [2, 2, 0], Poly::S4 => [4, 4, 4] } }
}

struct Inst { p: u64, a: u64, b: u64, xs: Vec<i64>, target: u64, xb: i64, f: MP }

fn seed_of(poly: Poly, bits: u32, alpha_c: u32, i: usize) -> u64 {
    // fixed, documented: instances are identical across lattice sizes
    let mut h: u64 = 0xA2_5EAE_0000_0000 ^ ((poly as u64) << 40) ^ ((bits as u64) << 24) ^ ((alpha_c as u64) << 8) ^ i as u64;
    h = h.wrapping_mul(0x9E3779B97F4A7C15);
    h ^ (h >> 31)
}

/// Semaev polynomial in the unknowns with the target substituted (MP over F_p).
fn spoly(poly: Poly, a: u64, b: u64, p: u64, target: u64) -> MP {
    match poly {
        Poly::S3 => semaev::s3_vars(0, 1, 2, a, b, p).subst(2, target),
        Poly::S4 => semaev::s4(a, b, p).subst(3, target),
    }
}

fn make_instance(poly: Poly, bits: u32, alpha: f64, seed: u64) -> Inst {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let p = random_prime(bits, &mut rng);
    let xb = ((alpha * (p as f64).log2()).exp2().floor() as i64).max(1);
    let nu = poly.nu();
    loop {
        let mut xs: Vec<i64> = vec![];
        while xs.len() < nu {
            let v = rng.gen_range(-xb..=xb);
            if !xs.contains(&v) { xs.push(v); }
        }
        let xr: Vec<u64> = xs.iter().map(|&v| from_i64(v, p)).collect();
        for _ in 0..200 {
            let a = rng.gen_range(0..p);
            let y1 = rng.gen_range(1..p);
            let b = subm(subm(mulm(y1, y1, p), mulm(mulm(xr[0], xr[0], p), xr[0], p), p), mulm(a, xr[0], p), p);
            let c = Curve { p, a, b };
            if !c.nonsingular() { continue; }
            let mut pts = vec![Pt::A(xr[0], y1)];
            let mut ok = true;
            for &x in &xr[1..] {
                let r = c.rhs(x);
                if r == 0 || !is_qr(r, p) { ok = false; break; }
                pts.push(c.lift_x(x).unwrap());
            }
            if !ok { continue; }
            let mut s = pts[0];
            for q in &pts[1..] { s = c.add(s, if rng.gen::<bool>() { *q } else { c.neg(*q) }); }
            let Pt::A(target, _) = s else { continue };
            let f = spoly(poly, a, b, p, target);
            let mut pt4 = [0u64; 4];
            for i in 0..nu { pt4[i] = xr[i]; }
            assert_eq!(f.eval(&pt4), 0, "planted root does not vanish");
            return Inst { p, a, b, xs, target, xb, f };
        }
    }
}

/// Check by substitution mod p and by the point sum on the curve.
fn verify_root(_poly: Poly, inst: &Inst, r: &[i64]) -> (bool, bool) {
    let p = inst.p;
    let xr: Vec<u64> = r.iter().map(|&v| from_i64(v, p)).collect();
    let mut pt4 = [0u64; 4];
    for i in 0..xr.len() { pt4[i] = xr[i]; }
    let subst = inst.f.eval(&pt4) == 0;
    let c = Curve { p, a: inst.a, b: inst.b };
    let pts: Option<Vec<Pt>> = xr.iter().map(|&x| c.lift_x(x)).collect();
    let mut sum_ok = false;
    if let Some(pts) = pts {
        let nsign = 1usize << (pts.len() - 1);
        for mask in 0..nsign {
            let mut s = pts[0];
            for (i, q) in pts[1..].iter().enumerate() { s = c.add(s, if mask >> i & 1 == 1 { c.neg(*q) } else { *q }); }
            if let Pt::A(x, _) = s { if x == inst.target { sum_ok = true; } }
        }
    }
    (subst, sum_ok)
}

fn mp_to_z(f: &MP, nv: usize, corner: [u8; 3]) -> ZPoly {
    let p = f.p;
    let mut ce = [0u8; 4];
    ce[..3].copy_from_slice(&corner);
    let lc = *f.t.get(&ce).expect("corner coefficient is zero");
    let inv = invm(lc, p);
    let mut z = HashMap::new();
    for (e, c) in &f.t {
        assert!(e[nv..].iter().all(|&t| t == 0));
        z.insert([e[0], e[1], e[2]], BigInt::from(mulm(*c, inv, p)));
    }
    z
}

struct Rec {
    i: usize, p: u64, prec: &'static str, xs: Vec<i64>, target: u64, xb: i64, dim: usize, lll_s: f64, swaps: u64,
    lll_ok: bool, ext_s: f64, n_hg: usize, n_cand: usize, n_vanish: usize, ext_status: String,
    n_roots: usize, n_verified: usize, success: bool, all_sum_ok: bool,
}

const CAND_MAX: usize = 3;
/// Root extraction uses the first K reduced vectors (all pairs for S3, all
/// triples for S4).
const K_S3: usize = 6;
const K_S4: usize = 4;

fn subsets_of(n: usize, r: usize) -> Vec<Vec<usize>> {
    let mut out = vec![];
    if r == 2 { for i in 0..n { for j in i + 1..n { out.push(vec![i, j]); } } }
    else if r == 3 { for i in 0..n { for j in i + 1..n { for k in j + 1..n { out.push(vec![i, j, k]); } } } }
    out
}

fn run_instance(poly: Poly, inst: &Inst, m: u32, i: usize, seed: u64) -> Rec {
    let nv = poly.nu();
    let d = poly.box_d();
    let fz = mp_to_z(&inst.f, nv, d);
    let mut lat = copper::build(&fz, nv, d, m, inst.p, inst.xb as u64);
    let dim = lat.rows.len();
    let t0 = Instant::now();
    let mut rows = std::mem::take(&mut lat.rows);
    if std::env::var("A2_SORT").is_ok() {
        // order rows by the size of their diagonal entry (smallest first)
        let mut keyed: Vec<(f64, Vec<BigInt>)> = rows.into_iter().enumerate().map(|(j, r)| (dd::big_to_log2(&r[j]), r)).collect();
        keyed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        rows = keyed.into_iter().map(|t| t.1).collect();
    }
    let delta: f64 = std::env::var("A2_DELTA").ok().and_then(|s| s.parse().ok()).unwrap_or(0.99);
    let res = lll::lll(&mut rows, delta);
    let lll_s = t0.elapsed().as_secs_f64();
    let (lll_ok, swaps, prec) = match &res { Ok(s) => (true, s.swaps, s.precision), Err(_) => (false, 0, "failed") };
    let pm_log2 = (m as f64) * (inst.p as f64).log2();
    let half_log_n = 0.5 * (dim as f64).log2();
    let norms: Vec<f64> = rows.iter().map(|r| copper::row_norm_log2(r)).collect();
    let n_hg = norms.iter().filter(|&&l| l < pm_log2 - half_log_n).count();
    let kfirst = match poly { Poly::S3 => K_S3, Poly::S4 => K_S4 }.min(dim);
    // diagnostic only (uses the planted root): first 10 rows vanishing over Z
    let polys: Vec<Vec<([u8; 3], BigInt)>> = rows.iter().take(10.max(kfirst)).map(|r| copper::row_poly(&lat, r)).collect();
    let mut x3 = [0i64; 3];
    for (k, v) in inst.xs.iter().enumerate() { x3[k] = *v; }
    let n_vanish = polys.iter().take(10).filter(|h| h.len() > 1 && copper::eval_z(h, &x3) == BigInt::from(0)).count();
    let n_cand = kfirst;
    if std::env::var("A2_DEBUG").is_ok() && i == 0 { for h in &polys[..kfirst] { let mut t: Vec<String> = h.iter().map(|(e, c)| format!("{c}*{e:?}")).collect(); t.sort(); eprintln!("h: {}", t.join(" + ")); } }
    // extraction (does not use the planted root): every nv-subset of the first
    // kfirst reduced vectors; candidates are kept only if they pass the checks
    let t1 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x5151);
    let hs: Vec<QP> = polys[..kfirst].iter().map(|h| QP::from_z(h, nv)).collect();
    let mut roots: Vec<Vec<i64>> = vec![];
    let mut errs = 0usize;
    let mut subsets = 0usize;
    for sub in subsets_of(kfirst, nv) {
        subsets += 1;
        let set: Vec<QP> = sub.iter().map(|&j| hs[j].clone()).collect();
        match copper::solve(&set, inst.xb, &mut rng) {
            Ok(r) => { for t in r { if !roots.contains(&t) { roots.push(t); } } }
            Err(_) => errs += 1,
        }
    }
    let ext_status = format!("subsets={subsets};degenerate={errs}");
    let ext_s = t1.elapsed().as_secs_f64();
    let mut n_verified = 0;
    let mut success = false;
    let mut all_sum_ok = true;
    let mut planted = inst.xs.clone();
    planted.sort();
    for r in &roots {
        let (sub_ok, sum_ok) = verify_root(poly, inst, r);
        if sub_ok {
            n_verified += 1;
            if !sum_ok { all_sum_ok = false; }
            let mut rs = r.clone();
            rs.sort();
            if rs == planted && sum_ok { success = true; }
        }
    }
    Rec { i, p: inst.p, prec, xs: inst.xs.clone(), target: inst.target, xb: inst.xb, dim, lll_s, swaps, lll_ok, ext_s,
          n_hg, n_cand, n_vanish, ext_status, n_roots: roots.len(), n_verified, success, all_sum_ok }
}

fn outdir() -> std::path::PathBuf {
    let d = match std::env::var("A2_OUT") { Ok(o) => std::path::PathBuf::from(o), Err(_) => std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results") };
    std::fs::create_dir_all(d.join("cells")).unwrap();
    d
}

fn run_cell(poly: Poly, bits: u32, m: u32, alpha_c: u32, ninst: usize) -> (usize, usize, f64) {
    let alpha = alpha_c as f64 / 100.0;
    let recs: Vec<Rec> = (0..ninst).into_par_iter().map(|i| {
        let seed = seed_of(poly, bits, alpha_c, i);
        let inst = make_instance(poly, bits, alpha, seed);
        run_instance(poly, &inst, m, i, seed)
    }).collect();
    let dim = recs[0].dim;
    let succ = recs.iter().filter(|r| r.success).count();
    let mut s = String::from("instance,p_bits,p,x_true,target,X,dim,lll_s,lll_swaps,lll_ok,lll_precision,extract_s,n_hg_short,n_first_vectors_used,n_vanish_first10,extract_status,n_roots_found,n_verified_roots,all_verified_pass_point_sum,success\n");
    for r in &recs {
        let xs: Vec<String> = r.xs.iter().map(|v| v.to_string()).collect();
        writeln!(s, "{},{},{},{},{},{},{},{:.4},{},{},{},{:.4},{},{},{},{},{},{},{},{}", r.i, bits, r.p, xs.join(";"), r.target, r.xb, r.dim,
                 r.lll_s, r.swaps, r.lll_ok, r.prec, r.ext_s, r.n_hg, r.n_cand, r.n_vanish, r.ext_status, r.n_roots, r.n_verified, r.all_sum_ok, r.success).unwrap();
    }
    let od = outdir();
    let name = format!("{}_p{}_m{}_n{}_a{:.2}.csv", poly.name(), bits, m, dim, alpha);
    std::fs::write(od.join("cells").join(&name), s).unwrap();
    let lt: Vec<f64> = recs.iter().map(|r| r.lll_s).collect();
    let mean_l = lt.iter().sum::<f64>() / lt.len() as f64;
    let max_l = lt.iter().cloned().fold(0.0, f64::max);
    let mean_x = recs.iter().map(|r| r.xb as f64).sum::<f64>() / recs.len() as f64;
    let lll_fail = recs.iter().filter(|r| !r.lll_ok).count();
    let vanish_ge2 = recs.iter().filter(|r| r.n_vanish >= poly.nu()).count();
    let hg_ge = recs.iter().filter(|r| r.n_hg >= poly.nu()).count();
    let sum_fail = recs.iter().filter(|r| !r.all_sum_ok).count();
    let line = format!("{},{},{},{},{:.2},{:.1},{},{},{:.4},{:.4},{},{},{},{},{}\n", poly.name(), bits, m, dim, alpha, mean_x, succ, ninst,
                       mean_l, max_l, lll_fail, hg_ge, vanish_ge2, sum_fail, name);
    let sp = od.join("cells_summary.csv");
    let new = !sp.exists();
    let mut fh = std::fs::OpenOptions::new().create(true).append(true).open(&sp).unwrap();
    if new { fh.write_all(b"poly,p_bits,m,dim,alpha,X_mean,successes,instances,lll_mean_s,lll_max_s,lll_failures,cells_hg_ge_nu,cells_vanish_ge_nu,verified_root_point_sum_failures,csv\n").unwrap(); }
    fh.write_all(line.as_bytes()).unwrap();
    eprintln!("{} p{} m{} n{} alpha {:.2}: {}/{}  LLL mean {:.2}s max {:.2}s  hg>=nu {} vanish>=nu {}", poly.name(), bits, m, dim, alpha, succ, ninst, mean_l, max_l, hg_ge, vanish_ge2);
    (succ, dim, mean_l)
}

fn grid(poly: Poly, bits: u32, m: u32, amax_c: u32, ninst: usize, zero_stop: usize) {
    let mut zeros = 0;
    let mut a = 2;
    while a <= amax_c {
        let (succ, _, _) = run_cell(poly, bits, m, a, ninst);
        if succ == 0 { zeros += 1; } else { zeros = 0; }
        if zeros >= zero_stop { eprintln!("stop: {zero_stop} consecutive 0/{ninst} cells"); break; }
        a += 2;
    }
}

/// Budget variant (used for m = 5 only, see REPORT.md "Deviations"): start at
/// alpha_hi; go up until 3 consecutive 0/20 cells are on record above, then go
/// down from alpha_hi - 0.02 until 3 consecutive 20/20 cells.
fn grid_down(poly: Poly, bits: u32, m: u32, ahi_c: u32, ninst: usize) {
    let mut a = ahi_c;
    let mut zeros = 0;
    loop {
        let (succ, _, _) = run_cell(poly, bits, m, a, ninst);
        if succ == 0 { zeros += 1; } else { zeros = 0; }
        if zeros >= 3 { break; }
        a += 2;
    }
    let mut full = 0;
    let mut a = ahi_c - 2;
    while a >= 2 {
        let (succ, _, _) = run_cell(poly, bits, m, a, ninst);
        if succ == ninst { full += 1; } else { full = 0; }
        if full >= 3 { eprintln!("stop: 3 consecutive {ninst}/{ninst} cells going down"); break; }
        a -= 2;
    }
}

// ---------------- controls ----------------

fn control_semaev() -> serde_json::Value {
    let p = 101u64;
    let mut rng = ChaCha8Rng::seed_from_u64(101);
    let mut out = serde_json::Map::new();
    let mut curves_checked = 0;
    let (mut s3_pos, mut s3_pos_ok, mut s3_exact_fail) = (0, 0, 0);
    let (mut s4_pos, mut s4_pos_ok) = (0, 0);
    let (mut s4_neg, mut s4_neg_zero) = (0, 0);
    let mut s4_rand_zero_but_no_relation = 0;
    let mut s4_relation_nonzero = 0;
    for _ in 0..5 {
        let (a, b) = loop {
            let a = rng.gen_range(0..p); let b = rng.gen_range(0..p);
            if (Curve { p, a, b }).nonsingular() { break (a, b); }
        };
        curves_checked += 1;
        let c = Curve { p, a, b };
        let pts: Vec<Pt> = (0..p).flat_map(|x| {
            let r = c.rhs(x);
            match sqrtm(r, p) { Some(y) if is_qr(r, p) => if y == 0 { vec![Pt::A(x, 0)] } else { vec![Pt::A(x, y), Pt::A(x, p - y)] }, _ => vec![] }
        }).collect();
        let s3 = semaev::s3_vars(0, 1, 2, a, b, p);
        let s4 = semaev::s4(a, b, p);
        // S3 exhaustive: for every pair of points, the roots in x3 of S3(x1,x2,.) over F_p
        // are exactly {x(P1+P2), x(P1-P2)} (finite ones), when x1 != x2.
        for &u in &pts { for &v in &pts {
            let (Pt::A(x1, _), Pt::A(x2, _)) = (u, v) else { continue };
            if x1 == x2 { continue; }
            let mut expect: Vec<u64> = vec![];
            for w in [c.add(u, v), c.add(u, c.neg(v))] { if let Pt::A(x, _) = w { expect.push(x); } }
            for &x3 in &expect { s3_pos += 1; if s3.eval(&[x1, x2, x3, 0]) == 0 { s3_pos_ok += 1; } }
            let roots: Vec<u64> = (0..p).filter(|&x3| s3.eval(&[x1, x2, x3, 0]) == 0).collect();
            let mut e2 = expect.clone(); e2.sort(); e2.dedup();
            if roots != e2 { s3_exact_fail += 1; }
        } }
        // S4: random quadruples summing to zero
        for _ in 0..300 {
            let p1 = pts[rng.gen_range(0..pts.len())]; let p2 = pts[rng.gen_range(0..pts.len())]; let p3 = pts[rng.gen_range(0..pts.len())];
            let s = c.add(c.add(p1, p2), p3);
            let Pt::A(x4, _) = s else { continue };
            let (Pt::A(x1, _), Pt::A(x2, _), Pt::A(x3, _)) = (p1, p2, p3) else { continue };
            s4_pos += 1;
            if s4.eval(&[x1, x2, x3, x4]) == 0 { s4_pos_ok += 1; }
        }
        // S4 negatives: random x-coordinates of points; zero iff some signed sum vanishes
        for _ in 0..300 {
            let q: Vec<Pt> = (0..4).map(|_| pts[rng.gen_range(0..pts.len())]).collect();
            let xs: Vec<u64> = q.iter().map(|t| match t { Pt::A(x, _) => *x, _ => 0 }).collect();
            let mut rel = false;
            for mask in 0..8u32 {
                let mut s = q[0];
                for i in 1..4 { s = c.add(s, if mask >> (i - 1) & 1 == 1 { c.neg(q[i]) } else { q[i] }); }
                if s == Pt::Inf { rel = true; }
            }
            let z = s4.eval(&[xs[0], xs[1], xs[2], xs[3]]) == 0;
            s4_neg += 1;
            if z { s4_neg_zero += 1; if !rel { s4_rand_zero_but_no_relation += 1; } }
            if rel && !z { s4_relation_nonzero += 1; }
        }
    }
    out.insert("prime".into(), p.into());
    out.insert("curves".into(), curves_checked.into());
    out.insert("s3_relations_checked".into(), s3_pos.into());
    out.insert("s3_relations_vanishing".into(), s3_pos_ok.into());
    out.insert("s3_root_set_mismatches".into(), s3_exact_fail.into());
    out.insert("s4_relations_checked".into(), s4_pos.into());
    out.insert("s4_relations_vanishing".into(), serde_json::json!(s4_pos_ok));
    out.insert("s4_random_point_quadruples".into(), s4_neg.into());
    out.insert("s4_random_zero".into(), s4_neg_zero.into());
    out.insert("s4_zero_without_Fp_relation".into(), s4_rand_zero_but_no_relation.into());
    out.insert("s4_relation_but_nonzero".into(), s4_relation_nonzero.into());
    let pass = s3_pos == s3_pos_ok && s3_exact_fail == 0 && s4_pos == s4_pos_ok && s4_relation_nonzero == 0;
    out.insert("pass".into(), pass.into());
    serde_json::Value::Object(out)
}

/// Code-path control for 3 unknowns: f = xyz + (random lower multilinear terms),
/// planted root of size X, box d=(1,1,1). Same build/LLL/solve as S4.
fn control_trivariate(bits: u32, m: u32, xlog2: f64, ninst: usize) -> serde_json::Value {
    let recs: Vec<(bool, f64, usize)> = (0..ninst).into_par_iter().map(|i| {
        let mut rng = ChaCha8Rng::seed_from_u64(7000 + i as u64);
        let p = random_prime(bits, &mut rng);
        let xb = xlog2.exp2().floor() as i64;
        let x0: Vec<i64> = (0..3).map(|_| rng.gen_range(-xb..=xb)).collect();
        let mut f = MP::zero(p);
        let monos: [[u8; 4]; 7] = [[1,1,0,0],[1,0,1,0],[0,1,1,0],[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,0]];
        f.t.insert([1, 1, 1, 0], 1);
        for e in &monos[..6] { f.t.insert(*e, rng.gen_range(1..p)); }
        let xr = [from_i64(x0[0], p), from_i64(x0[1], p), from_i64(x0[2], p), 0];
        let v = f.eval(&xr);
        f.t.insert([0, 0, 0, 0], subm(0, v, p));
        f.t.retain(|_, c| *c != 0);
        assert_eq!(f.eval(&xr), 0);
        let fz = mp_to_z(&f, 3, [1, 1, 1]);
        let mut lat = copper::build(&fz, 3, [1, 1, 1], m, p, xb as u64);
        let dim = lat.rows.len();
        let t0 = Instant::now();
        let mut rows = std::mem::take(&mut lat.rows);
        lll::lll(&mut rows, 0.99).unwrap();
        let ls = t0.elapsed().as_secs_f64();
        let pm_log2 = m as f64 * (p as f64).log2();
        let half = 0.5 * (dim as f64).log2();
        let cand: Vec<QP> = rows.iter().filter(|r| copper::row_norm_log2(r) < pm_log2 + half).take(CAND_MAX)
            .map(|r| QP::from_z(&copper::row_poly(&lat, r), 3)).collect();
        let mut ok = false;
        if cand.len() >= 2 {
            if let Ok(rs) = copper::solve(&cand, xb, &mut rng) {
                for r in rs { let rr = [from_i64(r[0], p), from_i64(r[1], p), from_i64(r[2], p), 0]; if f.eval(&rr) == 0 && r == x0 { ok = true; } }
            }
        }
        (ok, ls, dim)
    }).collect();
    let succ = recs.iter().filter(|r| r.0).count();
    serde_json::json!({"poly": "xyz + multilinear (code-path control for 3 unknowns)", "p_bits": bits, "m": m, "dim": recs[0].2,
                       "X_log2": xlog2, "successes": succ, "instances": ninst, "lll_mean_s": recs.iter().map(|r| r.1).sum::<f64>() / ninst as f64})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "controls" => {
            let mut o = serde_json::Map::new();
            let s = control_semaev();
            eprintln!("semaev identity: {s}");
            o.insert("semaev_identity_p101".into(), s);
            let mut tri = vec![];
            for (bits, m, xl) in [(32u32, 1u32, 2.0f64), (64, 1, 4.0), (64, 2, 4.0), (64, 2, 6.0)] {
                let c = control_trivariate(bits, m, xl, 20);
                eprintln!("trivariate control: {c}");
                tri.push(c);
            }
            o.insert("trivariate_codepath".into(), serde_json::Value::Array(tri));
            std::fs::write(outdir().join("controls.json"), serde_json::to_string_pretty(&serde_json::Value::Object(o)).unwrap()).unwrap();
        }
        "cell" => {
            let poly = if args[2] == "S3" { Poly::S3 } else { Poly::S4 };
            let bits: u32 = args[3].parse().unwrap();
            let m: u32 = args[4].parse().unwrap();
            let ac: u32 = args[5].parse().unwrap();
            let n: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(20);
            run_cell(poly, bits, m, ac, n);
        }
        "grid" => {
            let poly = if args[2] == "S3" { Poly::S3 } else { Poly::S4 };
            let bits: u32 = args[3].parse().unwrap();
            let m: u32 = args[4].parse().unwrap();
            let amax: u32 = args[5].parse().unwrap();
            let zs: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            grid(poly, bits, m, amax, 20, zs);
        }
        "summarize" => summarize(),
        "grid_down" => {
            let poly = if args[2] == "S3" { Poly::S3 } else { Poly::S4 };
            let bits: u32 = args[3].parse().unwrap();
            let m: u32 = args[4].parse().unwrap();
            let ahi: u32 = args[5].parse().unwrap();
            grid_down(poly, bits, m, ahi, 20);
        }
        _ => eprintln!("usage: a2 controls | cell S3|S4 bits m alpha_centi [n] | grid S3|S4 bits m alpha_max_centi [zero_stop]"),
    }
}

/// Heuristic determinant bound for the box lattice: X^{sum|e|} p^{sum(m-k)} < p^{m n}
/// with X = p^alpha gives alpha < sum k / sum |e| (ignores sqrt(n) and the LLL factor).
fn det_bound(poly: Poly, m: u32) -> f64 {
    let nv = poly.nu();
    let d = poly.box_d();
    let monos = copper::box_monomials(nv, d, m);
    let (mut sk, mut se) = (0u64, 0u64);
    for e in &monos {
        let mut k = m;
        for i in 0..nv { k = k.min(e[i] as u32 / d[i] as u32); }
        sk += k as u64;
        se += e.iter().map(|&t| t as u64).sum::<u64>();
    }
    sk as f64 / se as f64
}

fn summarize() {
    let od = outdir();
    let txt = std::fs::read_to_string(od.join("cells_summary.csv")).unwrap();
    // key (poly, bits, m) -> Vec<(alpha_c, succ, inst, dim, lll_mean)>
    let mut cells: std::collections::BTreeMap<(String, u32, u32), Vec<(u32, usize, usize, usize, f64)>> = Default::default();
    for line in txt.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        let ac = (f[4].parse::<f64>().unwrap() * 100.0).round() as u32;
        cells.entry((f[0].to_string(), f[1].parse().unwrap(), f[2].parse().unwrap())).or_default()
            .push((ac, f[6].parse().unwrap(), f[7].parse().unwrap(), f[3].parse().unwrap(), f[8].parse().unwrap()));
    }
    let mut entries = vec![];
    let mut per_pp: std::collections::BTreeMap<(String, u32), Vec<(u32, usize, Option<f64>)>> = Default::default();
    let mut controls = vec![];
    for ((poly, bits, m), v) in &cells {
        let grid: Vec<_> = v.iter().filter(|c| c.0 % 2 == 0).collect();
        let pick = |thr: usize| grid.iter().filter(|c| c.1 >= thr).map(|c| c.0).max().map(|a| a as f64 / 100.0);
        let a50 = pick(10);
        let aany = pick(1);
        let aall = pick(20);
        let pl = if poly == "S3" { Poly::S3 } else { Poly::S4 };
        let dim = v[0].3;
        let mut sorted: Vec<_> = grid.iter().map(|c| serde_json::json!({"alpha": c.0 as f64 / 100.0, "successes": c.1, "instances": c.2, "lll_mean_s": c.4})).collect();
        sorted.sort_by(|a, b| a["alpha"].as_f64().partial_cmp(&b["alpha"].as_f64()).unwrap());
        for c in v.iter().filter(|c| c.0 == 5) {
            controls.push(serde_json::json!({"poly": poly, "p_bits": bits, "m": m, "dim": dim, "alpha": 0.05, "successes": c.1, "instances": c.2}));
        }
        entries.push(serde_json::json!({"poly": poly, "p_bits": bits, "m": m, "dim": dim,
            "alpha_max": a50, "alpha_max_any_success": aany, "alpha_max_all_20": aall,
            "det_bound_alpha": det_bound(pl, *m), "grid_cells_run": grid.len(), "cells": sorted}));
        per_pp.entry((poly.clone(), *bits)).or_default().push((*m, dim, a50));
    }
    let mut verdict_rows = vec![];
    let mut stop_rows = vec![];
    for ((poly, bits), v) in &per_pp {
        let thr = if poly == "S3" { 0.30 } else { 0.20 };
        let best = v.iter().filter_map(|t| t.2).fold(f64::NEG_INFINITY, f64::max);
        let seq: Vec<f64> = v.iter().map(|t| t.2.unwrap_or(0.0)).collect();
        let increases = seq.windows(2).filter(|w| w[1] > w[0] + 1e-9).count();
        // stop rule: two consecutive dimension increases with no improvement
        let mut stop_at = None;
        for i in 2..seq.len() { if seq[i] <= seq[i - 2] + 1e-9 && seq[i - 1] <= seq[i - 2] + 1e-9 { stop_at = Some(v[i].1); break; } }
        verdict_rows.push(serde_json::json!({"poly": poly, "p_bits": bits, "threshold_0p6": thr,
            "best_alpha_max": if best.is_finite() { serde_json::json!(best) } else { serde_json::Value::Null },
            "below_threshold": !(best >= thr), "dims": v.iter().map(|t| t.1).collect::<Vec<_>>(),
            "alpha_max_by_dim": v.iter().map(|t| t.2).collect::<Vec<_>>(), "dimension_steps_with_increase": increases}));
        stop_rows.push(serde_json::json!({"poly": poly, "p_bits": bits, "stop_rule_triggered_at_dim": stop_at}));
    }
    let out = serde_json::json!({
        "definition": "alpha_max = largest grid alpha (step 0.02) with >= 10/20 planted roots recovered; also given: >= 1/20 and 20/20",
        "entries": entries, "control_alpha_0p05": controls, "h2_by_poly_and_p": verdict_rows, "stop_rule": stop_rows});
    std::fs::write(od.join("summary.json"), serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("wrote summary.json");
}
