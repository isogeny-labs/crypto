mod common;
mod curve;
mod experiment;
mod gf;
mod semaev;
mod solver;
mod yield_;
use common::*;
use curve::*;
use gf::*;

const SEED: u64 = 0xC1_7715_7ED_2610_27;

fn selfcheck() {
    let s5 = semaev::s5_poly();
    let terms = semaev::symmetrise(&s5, 8).expect("sym");
    println!("S5: {} monomials in x1..x5,b; {} e-terms", s5.len(), terms.len());
    for k in [5u32, 7] {
        let f = Fq4::new(k);
        let mut rng = Rng::new(SEED, "selfcheck");
        // field axioms
        for _ in 0..200 {
            let (a, b, c) = (rng.e4(&f), rng.e4(&f), rng.e4(&f));
            assert_eq!(f.mul(&a, &add(&b, &c)), add(&f.mul(&a, &b), &f.mul(&a, &c)));
            assert_eq!(f.mul(&f.mul(&a, &b), &c), f.mul(&a, &f.mul(&b, &c)));
            assert_eq!(f.sq(&a), f.mul(&a, &a));
            if !is_zero(&a) {
                assert_eq!(f.mul(&a, &f.inv(&a)), ONE);
            }
            assert_eq!(f.sqrt(&f.sq(&a)), a);
            assert_eq!(f.pow(&a, (1u64 << (4 * k)) - 1), if is_zero(&a) { ZERO } else { ONE });
            let z = f.artin_schreier(&a);
            assert_eq!(z.is_some(), f.tr_abs(&a) == 0);
            if let Some(z) = z {
                assert_eq!(add(&f.sq(&z), &z), a);
            }
        }
        println!("k={k}: field arithmetic, Artin-Schreier: ok");
        for (av, bv) in [(0u8, 8u8), (0, 1), (8, 9)] {
            let (a, b) = (f.f16(av), f.f16(bv));
            let cv = Curve { f: &f, a, b };
            let n16 = order_f16(&f, &a, &b);
            let formula = order_ext(n16, k);
            let direct = order_direct(&cv);
            assert_eq!(formula, direct, "order mismatch k={k} a={av} b={bv}");
            println!("k={k} a={av} b={bv}: #E(F16)={n16}, #E(F_2^{}) = {direct} (formula equal)", 4 * k);
            // group law and S5 on random points
            let ts = sample_targets(&cv, &mut rng, 40);
            for w in ts.chunks(4) {
                let mut r = w[0];
                for p in &w[1..] {
                    r = cv.add(&r, p);
                }
                assert!(cv.on(&r));
                if r.inf || is_zero(&r.x) {
                    continue;
                }
                let x = [w[0].x, w[1].x, w[2].x, w[3].x];
                // sum of four points: R = P1+..+P4 up to signs => S5(x, x_R) = 0
                let v = eval_s5(&f, &terms, &b, &x, &r.x);
                assert!(is_zero(&v), "S5 not zero on a true relation");
            }
            // and nonzero on random data
            let x = [rng.e4(&f), rng.e4(&f), rng.e4(&f), rng.e4(&f)];
            assert!(!is_zero(&eval_s5(&f, &terms, &b, &x, &rng.e4(&f))));
        }
        println!("k={k}: S5(x(P1..P4), x(sum)) = 0 on random relations: ok");
    }
    // MITM vs brute force
    let k = 5;
    let f = Fq4::new(k);
    for (av, bv, u) in [(0u8, 8u8, 2u8), (0, 12, 2), (0, 1, 1)] {
        let (a, b) = (f.f16(av), f.f16(bv));
        let cv = Curve { f: &f, a, b };
        for (name, uu) in [("std", ONE), ("tw", f.f16(u))] {
            let (fb, _) = build_fb(&cv, &uu);
            let tab = yield_::PairTable::build(&cv, &fb);
            let mut rng = Rng::new(SEED, "mitm");
            let ts = sample_targets(&cv, &mut rng, 6);
            for r in &ts {
                let a1 = tab.count(&cv, r);
                let a2 = yield_::count_brute(&cv, &fb, r);
                assert_eq!(a1, a2, "MITM != brute (a={av} b={bv} {name})");
            }
            let o1 = tab.count_o();
            let o2 = yield_::count_brute(&cv, &fb, &O);
            assert_eq!(o1, o2, "N(O)");
            let t = Pt { x: ZERO, y: f.sqrt(&b), inf: false };
            assert_eq!(tab.count(&cv, &t), yield_::count_brute(&cv, &fb, &t), "N(T)");
            println!("k=5 a={av} b={bv} {name}: |FB|={}, MITM == brute on 6 targets, O and T", fb.len());
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selfcheck") => selfcheck(),
        Some("run") => {
            // run <outdir> <what>
            let dir = &args[2];
            std::fs::create_dir_all(dir).unwrap();
            let what = args[3].as_str();
            let terms = semaev::symmetrise(&semaev::s5_poly(), 8).unwrap();
            let threads: usize = std::env::var("THREADS").ok().and_then(|x| x.parse().ok()).unwrap_or(4);
            use experiment::*;
            match what {
                "validate" => {
                    let mut o = std::fs::File::create(format!("{dir}/validate.jsonl")).unwrap();
                    run_validate(&terms, &mut o);
                }
                "yield" => {
                    let ks: Vec<u32> = args[4].split(',').map(|x| x.parse().unwrap()).collect();
                    for k in ks {
                        let (t, curves): (usize, Vec<&'static str>) = match k {
                            5 => (20000, vec!["A0", "A1", "A2", "NEG", "B1", "B2", "B3"]),
                            7 => (8000, vec!["A0", "A1", "A2", "NEG", "B1", "B2", "B3"]),
                            9 => (1500, vec!["A0", "A1", "A2", "NEG", "B1", "B2", "B3"]),
                            11 => (150, vec!["A1", "A2", "B1"]),
                            _ => panic!("no yield grid for k={k}"),
                        };
                        let mut o = std::fs::File::create(format!("{dir}/yield_k{k}.jsonl")).unwrap();
                        run_yield(&YieldCfg { k, targets: t, curves }, &mut o, threads);
                    }
                }
                "cost" => {
                    let ks: Vec<u32> = args[4].split(',').map(|x| x.parse().unwrap()).collect();
                    for k in ks {
                        let curves: Vec<&'static str> = if k <= 9 { vec!["A0", "A1", "A2", "B1", "B2", "B3"] } else { vec!["A1", "A2", "B1", "B2", "B3"] };
                        let mut o = std::fs::File::create(format!("{dir}/cost_k{k}.jsonl")).unwrap();
                        run_cost(&CostCfg { k, targets: 6, curves, ladder_max: 27 }, &terms, &mut o, threads);
                    }
                }
                _ => panic!("what?"),
            }
        }
        Some("hfb") => {
            let k: u32 = args[2].parse().unwrap();
            let seed: u64 = args[3].parse().unwrap();
            let d0: u32 = args[4].parse().unwrap();
            let d1: u32 = args[5].parse().unwrap();
            let f = Fq4::new(k);
            let terms = semaev::symmetrise(&semaev::s5_poly(), 8).unwrap();
            let mut rng = Rng::new(SEED ^ seed, "hfb");
            let a = f.f16(0);
            let b = loop { let b = rng.e4(&f); if !is_zero(&b) { break b; } };
            let cv = Curve { f: &f, a, b };
            let u = random_u_outside_f16(&f, &b, &mut rng, &[]);
            let r = sample_targets(&cv, &mut rng, 1)[0];
            for (name, uu) in [("std", ONE), ("tw", u)] {
                let sys = solver::build_system(&f, &terms, &b, &uu, &r.x);
                let topp: Vec<Vec<(solver::Mono4, u16)>> = sys.polys.iter().map(|p| p.iter().filter(|(m, _)| solver::wdeg(m) == 8).cloned().collect()).collect();
                let (rt, ..) = solver::macaulay(&f.f, &topp, 30, 4);
                println!("{name}: HF_top(30)={}", rt.std_by_deg[30]);
                for d in d0..=d1 {
                    let (r, ..) = solver::macaulay(&f.f, &sys.polys, d, 4);
                    println!("{name} D={d}: rows {} cols {} rank {} HF {} stdmaxdeg {} mults {}", r.rows, r.cols, r.rank, r.hf, r.std_max_deg, r.mults);
                }
            }
        }
        Some("topscan") => {
            let k: u32 = args[2].parse().unwrap();
            let f = Fq4::new(k);
            let terms = semaev::symmetrise(&semaev::s5_poly(), 8).unwrap();
            let mut rng = Rng::new(SEED, "topscan");
            let hft = |b: &E4, u: &E4, xr: &E4| -> usize {
                let sys = solver::build_system(&f, &terms, b, u, xr);
                let topp: Vec<Vec<(solver::Mono4, u16)>> = sys.polys.iter().map(|p| p.iter().filter(|(m, _)| solver::wdeg(m) == 8).cloned().collect()).collect();
                let (rt, ..) = solver::macaulay(&f.f, &topp, 28, 1);
                rt.std_by_deg[28]
            };
            // family A
            for bv in [8u8, 9, 10, 11] {
                let b = f.f16(bv);
                let cv = Curve { f: &f, a: f.f16(0), b };
                let ts = sample_targets(&cv, &mut rng, 6);
                for uv in valid_u_f16(&f, &b) {
                    let v: Vec<usize> = ts.iter().map(|t| hft(&b, &f.f16(uv), &t.x)).collect();
                    println!("A b={bv} u={uv}: HF_top over 6 targets {:?}", v);
                }
                let v: Vec<usize> = ts.iter().map(|t| hft(&b, &ONE, &t.x)).collect();
                println!("A b={bv} std: {:?}", v);
            }
            for sd in 0..6u64 {
                let mut r2 = Rng::new(SEED ^ sd, "topscanB");
                let b = loop { let b = r2.e4(&f); if !is_zero(&b) { break b; } };
                let cv = Curve { f: &f, a: f.f16(0), b };
                let u = random_u_outside_f16(&f, &b, &mut r2, &[]);
                let ts = sample_targets(&cv, &mut r2, 6);
                let v: Vec<usize> = ts.iter().map(|t| hft(&b, &u, &t.x)).collect();
                let w: Vec<usize> = ts.iter().map(|t| hft(&b, &ONE, &t.x)).collect();
                println!("B seed {sd}: tw {:?} std {:?}", v, w);
            }
        }
        Some("hf") => {
            let k: u32 = args[2].parse().unwrap();
            let bv: u8 = args[3].parse().unwrap();
            let uv: u8 = args[4].parse().unwrap();
            let d0: u32 = args[5].parse().unwrap();
            let d1: u32 = args[6].parse().unwrap();
            let f = Fq4::new(k);
            let terms = semaev::symmetrise(&semaev::s5_poly(), 8).unwrap();
            let (a, b) = (f.f16(0), f.f16(bv));
            let cv = Curve { f: &f, a, b };
            let u = if uv == 0 { ONE } else { f.f16(uv) };
            let mut rng = Rng::new(SEED, "hf");
            let r = sample_targets(&cv, &mut rng, 1)[0];
            let sys = solver::build_system(&f, &terms, &b, &u, &r.x);
            for (i, p) in sys.polys.iter().enumerate() {
                println!("poly {i}: {} terms, wdeg {}", p.len(), p.iter().map(|(m, _)| solver::wdeg(m)).max().unwrap());
            }
            for (i, p) in sys.polys.iter().enumerate() {
                let top: Vec<_> = p.iter().filter(|(m, _)| solver::wdeg(m) == 8).collect();
                println!("poly {i}: top-degree terms {}; E4^8 coeff {:?}", top.len(), p.iter().find(|(m, _)| *m == [0, 0, 0, 8]).map(|x| x.1));
            }
            if args.len() > 7 && args[7] == "top" {
                let topp: Vec<Vec<(solver::Mono4, u16)>> = sys.polys.iter().map(|p| p.iter().filter(|(m, _)| solver::wdeg(m) == 8).cloned().collect()).collect();
                for d in d0..=d1 {
                    let (r, ..) = solver::macaulay(&f.f, &topp, d, 4);
                    println!("top forms D={d}: HF_top(D) = {} (monomials of degree D: {})", r.std_by_deg[d as usize], solver::monomials_desc(d).iter().filter(|m| solver::wdeg(m) == d).count());
                }
                return;
            }
            for d in d0..=d1 {
                let t = std::time::Instant::now();
                let (r, pr, _rows, cols) = solver::macaulay(&f.f, &sys.polys, d, 4);
                if args.len() > 7 {
                    for (c, m) in cols.iter().enumerate() {
                        if pr[c] == u32::MAX && solver::wdeg(m) + 3 >= d {
                            println!("   std {:?} wdeg {}", m, solver::wdeg(m));
                        }
                    }
                }
                println!("D={d}: rows {} cols {} rank {} HF {} stdmaxdeg {} mults {} invs {} maxnnz {}  [{:.1?}]", r.rows, r.cols, r.rank, r.hf, r.std_max_deg, r.mults, r.invs, r.max_row_nnz_after, t.elapsed());
            }
        }
        _ => eprintln!("usage"),
    }
}
