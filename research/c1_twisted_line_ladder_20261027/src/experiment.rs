//! Experiment driver: cells (k, curve), yield arms, cost arms, validation.
use crate::common::*;
use crate::curve::*;
use crate::gf::*;
use crate::semaev::ETerm;
use crate::solver;
use crate::yield_::PairTable;
use std::io::Write;

pub const SEED: u64 = 0xC1_7715_7ED_2610_27;

pub struct CurveSpec {
    pub id: String,
    pub family: &'static str,
    pub a: E4,
    pub b: E4,
    pub tr_a: u32,
    pub tr_b16: Option<u32>,
    /// twisted arms: (name, u, u in F_16)
    pub us: Vec<(String, E4, bool)>,
}

pub fn curves_for_k(f: &Fq4) -> Vec<CurveSpec> {
    let k = f.f.k;
    let mut out = vec![];
    // orbits of F_16^* under squaring; tau = Tr_{16/2}(b)
    let mut seen = [false; 16];
    let mut reps_tau1 = vec![];
    for bv in 1..16u8 {
        if seen[bv as usize] {
            continue;
        }
        let mut x = f.f16(bv);
        let mut tau1 = false;
        loop {
            let packed = (x[0] | x[1] << 1 | x[2] << 2 | x[3] << 3) as usize;
            if seen[packed] {
                break;
            }
            seen[packed] = true;
            x = f.sq(&x);
        }
        if f.tr_abs(&f.f16(bv)) == 1 {
            tau1 = true;
        }
        if tau1 {
            reps_tau1.push(bv);
        }
    }
    assert_eq!(reps_tau1.len(), 2);
    let mk_a = |id: &str, fam: &'static str, a: E4, b: E4| -> CurveSpec {
        let vs = valid_u_f16(f, &b);
        assert!(vs.len() >= 2);
        CurveSpec {
            id: id.to_string(),
            family: fam,
            a,
            b,
            tr_a: f.tr_abs(&a),
            tr_b16: Some(f.tr_abs(&b)),
            us: vec![
                (format!("tw_a(u={})", vs[0]), f.f16(vs[0]), true),
                (format!("tw_b(u={})", vs[1]), f.f16(vs[1]), true),
            ],
        }
    };
    let a0 = f.f16(0);
    out.push(mk_a("A0", "A-tau0-control", a0, f.f16(1)));
    out.push(mk_a("A1", "A", a0, f.f16(reps_tau1[0])));
    out.push(mk_a("A2", "A", a0, f.f16(reps_tau1[1])));
    // negative control: Tr(a) = 1 (a = g^3), b of A1
    let a_neg = f.f16(8);
    assert_eq!(f.tr_abs(&a_neg), 1);
    out.push(mk_a("NEG", "negative-control", a_neg, f.f16(reps_tau1[0])));
    for i in 0..3 {
        let mut rng = Rng::new(SEED, &format!("Bcurve-{k}-{i}"));
        let b = loop {
            let b = rng.e4(f);
            if !is_zero(&b) {
                break b;
            }
        };
        let u1 = random_u_outside_f16(f, &b, &mut rng, &[]);
        let u2 = random_u_outside_f16(f, &b, &mut rng, &[u1]);
        out.push(CurveSpec {
            id: format!("B{}", i + 1),
            family: "B",
            a: a0,
            b,
            tr_a: 0,
            tr_b16: None,
            us: vec![("tw_a(u generic)".into(), u1, false), ("tw_b(u generic)".into(), u2, false)],
        });
    }
    out
}

pub fn order_direct_par(cv: &Curve, threads: usize) -> u64 {
    let f = cv.f;
    let k = f.f.k;
    let total = (1u64 << (4 * k)) - 1;
    let chunk = total.div_ceil(threads as u64);
    let mut sum = 0u64;
    std::thread::scope(|s| {
        let hs: Vec<_> = (0..threads as u64)
            .map(|t| {
                s.spawn(move || {
                    let lo = 1 + t * chunk;
                    let hi = (lo + chunk).min(total + 1);
                    let mut n = 0u64;
                    for v in lo..hi {
                        if cv.lifts(&f.unpack(v)) {
                            n += 1;
                        }
                    }
                    n
                })
            })
            .collect();
        for h in hs {
            sum += h.join().unwrap();
        }
    });
    2 + 2 * sum
}

fn hex(f: &Fq4, z: &E4) -> String {
    format!("\"{:#x}\"", f.pack(z))
}
fn opt<T: std::fmt::Display>(v: Option<T>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "null".into(),
    }
}

pub struct YieldCfg {
    pub k: u32,
    pub targets: usize,
    pub curves: Vec<&'static str>,
}

pub fn run_yield(cfg: &YieldCfg, out: &mut dyn Write, threads: usize) {
    let f = Fq4::new(cfg.k);
    let k = cfg.k;
    for spec in curves_for_k(&f) {
        if !cfg.curves.contains(&spec.id.as_str()) {
            continue;
        }
        let cv = Curve { f: &f, a: spec.a, b: spec.b };
        let t0 = std::time::Instant::now();
        // order
        let order: Option<u64> = if spec.family.starts_with("A") || spec.family == "negative-control" {
            let n16 = order_f16(&f, &spec.a, &spec.b);
            Some(order_ext(n16, k))
        } else if k <= 7 {
            Some(order_direct_par(&cv, threads))
        } else {
            None
        };
        let mut rng = Rng::new(SEED, &format!("targets-{k}-{}", spec.id));
        let targets = sample_targets(&cv, &mut rng, cfg.targets);
        let tpt = Pt { x: ZERO, y: f.sqrt(&spec.b), inf: false };
        assert!(cv.on(&tpt));
        let mut arms: Vec<(String, E4, bool)> = vec![("std".into(), ONE, false)];
        arms.extend(spec.us.iter().cloned());
        for (name, u, in16) in arms {
            let (fb, lifting) = build_fb(&cv, &u);
            let (counts, c_o, c_t) = if fb.is_empty() {
                (vec![0u64; targets.len()], 0u64, 0u64)
            } else {
                let tab = PairTable::build(&cv, &fb);
                let c_o = tab.count_o();
                let c_t = tab.count(&cv, &tpt);
                let chunk = targets.len().div_ceil(threads);
                let mut counts = vec![0u64; targets.len()];
                std::thread::scope(|s| {
                    let tabr = &tab;
                    let cvr = &cv;
                    let hs: Vec<_> = targets
                        .chunks(chunk)
                        .zip(counts.chunks_mut(chunk))
                        .map(|(tc, cc)| {
                            s.spawn(move || {
                                for (t, c) in tc.iter().zip(cc.iter_mut()) {
                                    *c = tabr.count(cvr, t);
                                }
                            })
                        })
                        .collect();
                    for h in hs {
                        h.join().unwrap();
                    }
                });
                (counts, c_o, c_t)
            };
            let m = fb.len() as f64;
            let pred_excl = order.map(|e| (m.powi(4) - (c_o + c_t) as f64) / (e - 2) as f64);
            let pred_incl = order.map(|e| m.powi(4) / e as f64);
            let cstr: Vec<String> = counts.iter().map(|c| c.to_string()).collect();
            writeln!(
                out,
                "{{\"kind\":\"yield\",\"k\":{k},\"curve\":\"{}\",\"family\":\"{}\",\"tr_a\":{},\"tr_b16\":{},\"a\":{},\"b\":{},\"arm\":\"{name}\",\"u\":{},\"u_in_F16\":{in16},\"lifting\":{lifting},\"q_minus_1\":{},\"fb_points\":{},\"order\":{},\"targets\":{},\"c_O\":{c_o},\"c_T\":{c_t},\"pred_excl\":{},\"pred_incl\":{},\"counts\":[{}]}}",
                spec.id,
                spec.family,
                spec.tr_a,
                opt(spec.tr_b16),
                hex(&f, &spec.a),
                hex(&f, &spec.b),
                hex(&f, &u),
                f.f.q - 1,
                fb.len(),
                opt(order),
                targets.len(),
                opt(pred_excl.map(|x| format!("{x:.9}"))),
                opt(pred_incl.map(|x| format!("{x:.9}"))),
                cstr.join(",")
            )
            .unwrap();
            out.flush().unwrap();
            eprintln!(
                "yield k={k} {} {name}: fb={} lifting={lifting} mean N = {:.4} [{:.1?}]",
                spec.id,
                fb.len(),
                counts.iter().sum::<u64>() as f64 / counts.len() as f64,
                t0.elapsed()
            );
        }
    }
}

pub struct CostCfg {
    pub k: u32,
    pub targets: usize,
    pub curves: Vec<&'static str>,
    pub ladder_max: u32,
}

pub fn run_cost(cfg: &CostCfg, terms: &[ETerm], out: &mut dyn Write, threads: usize) {
    let f = Fq4::new(cfg.k);
    let k = cfg.k;
    for spec in curves_for_k(&f) {
        if !cfg.curves.contains(&spec.id.as_str()) {
            continue;
        }
        let cv = Curve { f: &f, a: spec.a, b: spec.b };
        // same target stream as the yield run: the first `targets` of it
        let mut rng = Rng::new(SEED, &format!("targets-{k}-{}", spec.id));
        let targets = sample_targets(&cv, &mut rng, cfg.targets);
        let mut arms: Vec<(String, E4)> = vec![("std".into(), ONE)];
        arms.extend(spec.us.iter().map(|(n, u, _)| (n.clone(), *u)));
        // direct check that the twisted lines lift entirely (sampled)
        let mut vr = Rng::new(SEED, &format!("linecheck-{k}-{}", spec.id));
        for (name, u) in &arms {
            let mut lifts = 0;
            let n = 64;
            for _ in 0..n {
                let y = 1 + vr.below(f.f.q as u64 - 1) as u16;
                if cv.lifts(&f.mul(u, &fq_el(y))) {
                    lifts += 1;
                }
            }
            writeln!(out, "{{\"kind\":\"linecheck\",\"k\":{k},\"curve\":\"{}\",\"arm\":\"{name}\",\"sampled\":{n},\"lifting\":{lifts}}}", spec.id).unwrap();
        }
        for (ti, r) in targets.iter().enumerate() {
            for (name, u) in &arms {
                let t0 = std::time::Instant::now();
                let sys = solver::build_system(&f, terms, &spec.b, u, &r.x);
                let setup = sys.setup_f4_muls * 16; // one F_{Q^4} product = 16 F_Q products
                if ti == 0 && (spec.id == "A1" || spec.id == "B1") {
                    for d in 8..=cfg.ladder_max {
                        let (res, ..) = solver::macaulay(&f.f, &sys.polys, d, threads);
                        writeln!(out, "{{\"kind\":\"ladder\",\"k\":{k},\"curve\":\"{}\",\"arm\":\"{name}\",\"D\":{d},\"rows\":{},\"cols\":{},\"rank\":{},\"hf\":{},\"std_max_deg\":{},\"mults\":{},\"invs\":{}}}", spec.id, res.rows, res.cols, res.rank, res.hf, res.std_max_deg, res.mults, res.invs).unwrap();
                    }
                }
                let (res, ..) = solver::macaulay(&f.f, &sys.polys, 26, threads);
                let nz: Vec<usize> = sys.polys.iter().map(|p| p.len()).collect();
                writeln!(out, "{{\"kind\":\"cost\",\"k\":{k},\"curve\":\"{}\",\"family\":\"{}\",\"arm\":\"{name}\",\"target\":{ti},\"x_R\":{},\"poly_terms\":{:?},\"setup_muls\":{setup},\"D\":26,\"rows\":{},\"cols\":{},\"rank\":{},\"hf\":{},\"std_max_deg\":{},\"closed\":{},\"mults\":{},\"invs\":{},\"matrix_bytes\":{}}}", spec.id, spec.family, hex(&f, &r.x), nz, res.rows, res.cols, res.rank, res.hf, res.std_max_deg, res.std_max_deg < 26, res.mults, res.invs, res.rows * res.cols * 2).unwrap();
                out.flush().unwrap();
                eprintln!("cost k={k} {} t{ti} {name}: mults {} hf {} stdmax {} [{:.1?}]", spec.id, res.mults, res.hf, res.std_max_deg, t0.elapsed());
            }
        }
    }
}

/// Validation: explicit relations satisfy the descended equations.
pub fn run_validate(terms: &[ETerm], out: &mut dyn Write) {
    let k = 5;
    let f = Fq4::new(k);
    for spec in curves_for_k(&f) {
        if !["A1", "A2", "B1", "B2"].contains(&spec.id.as_str()) {
            continue;
        }
        let cv = Curve { f: &f, a: spec.a, b: spec.b };
        let mut rng = Rng::new(SEED, &format!("validate-{}", spec.id));
        let targets = sample_targets(&cv, &mut rng, 150);
        let mut arms: Vec<(String, E4)> = vec![("std".into(), ONE)];
        arms.extend(spec.us.iter().map(|(n, u, _)| (n.clone(), *u)));
        for (name, u) in &arms {
            let (fb, _) = build_fb(&cv, u);
            let tab = PairTable::build(&cv, &fb);
            let iu = f.inv(u);
            let mut inb = std::collections::HashSet::new();
            for p in &fb {
                inb.insert(f.pack(&p.x));
            }
            let (mut tuples, mut violations, mut not_in_fq, mut mitm_mismatch) = (0u64, 0u64, 0u64, 0u64);
            for r in &targets {
                let sys = solver::build_system(&f, terms, &spec.b, u, &r.x);
                let mut here = 0u64;
                for p1 in &fb {
                    let r1 = cv.add(r, &cv.neg(p1));
                    for p2 in &fb {
                        let r2 = cv.add(&r1, &cv.neg(p2));
                        for p3 in &fb {
                            let r3 = cv.add(&r2, &cv.neg(p3));
                            if !r3.inf && inb.contains(&f.pack(&r3.x)) {
                                here += 1;
                                let xs = [p1.x, p2.x, p3.x, r3.x];
                                let mut ys = [0u16; 4];
                                for i in 0..4 {
                                    let y = f.mul(&xs[i], &iu);
                                    if !in_fq(&f, &y) {
                                        not_in_fq += 1;
                                    }
                                    ys[i] = y[0];
                                }
                                let e = solver::e_coords(&f.f, &ys);
                                for p in &sys.polys {
                                    if solver::eval(&f.f, p, &e) != 0 {
                                        violations += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if tab.count(&cv, r) != here {
                    mitm_mismatch += 1;
                }
                tuples += here;
            }
            writeln!(out, "{{\"kind\":\"validate\",\"k\":{k},\"curve\":\"{}\",\"arm\":\"{name}\",\"targets\":{},\"ordered_relations\":{tuples},\"equation_violations\":{violations},\"abscissae_not_in_u_Fq\":{not_in_fq},\"mitm_vs_enumeration_mismatches\":{mitm_mismatch}}}", spec.id, targets.len()).unwrap();
            out.flush().unwrap();
            eprintln!("validate {} {name}: relations {tuples}, violations {violations}, not_in_Fq {not_in_fq}, mitm mismatches {mitm_mismatch}", spec.id);
        }
    }
}
