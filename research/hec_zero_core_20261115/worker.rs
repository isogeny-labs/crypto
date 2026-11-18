// Baseline Fixture and worker bodies copied byte-for-byte from heldout.rs at fab11cf141.
#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
#[path = "../hec_lazy_forest_20261115/lazy.rs"]
mod lazy;
#[path = "../hec_anchor_relations_20261114/linear.rs"]
mod linear;
#[path = "../hec_table_oracle_20261114/pipeline.rs"]
mod pipeline;
#[path = "../hec_anchor_relations_20261114/rational.rs"]
mod rational;
#[path = "../hec_anchor_relations_20261114/rational_solve.rs"]
mod rational_solve;
#[path = "../hec_point_sieve_20261115/sieve.rs"]
mod sieve;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    fp::Fp,
    ic::{self, IcParams},
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
    solve::Solver,
};
use std::{fs, time::Instant};
fn prime(n: u64) -> bool {
    n >= 2 && !(2..).take_while(|d| d * d <= n).any(|d| n % d == 0)
}
fn upper(p: u64, g: usize) -> u128 {
    let s = (p as f64).sqrt() as u128 + 2;
    s.pow((2 * g) as u32)
}
struct Fixture {
    c: Curve,
    d1: Div,
    ell: u64,
    family: u64,
    attempt: u64,
    selection_ns: u64,
}
impl Fixture {
    fn encode(&self) -> String {
        let mut v = vec![
            "FIXTURE".to_string(),
            self.c.g.to_string(),
            self.c.fp.p.to_string(),
            self.family.to_string(),
            self.attempt.to_string(),
            self.ell.to_string(),
        ];
        for p in [&self.c.f, &self.d1.u, &self.d1.v] {
            v.push(p.n.to_string());
            v.extend(p.c[..p.n].iter().map(u64::to_string));
        }
        v.push(self.selection_ns.to_string());
        v.join(" ")
    }
    fn parse(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next(), Some("FIXTURE"));
        let mut take = || it.next().unwrap().parse::<u64>().unwrap();
        let g = take() as usize;
        let p = take();
        let family = take();
        let attempt = take();
        let ell = take();
        assert!(prime(p) && p > 3 && p <= 1_000_000);
        assert!(prime(ell));
        let mut polys = Vec::new();
        for _ in 0..3 {
            let n = take() as usize;
            assert!(n <= 24);
            let cs: Vec<_> = (0..n).map(|_| take()).collect();
            assert!(cs.iter().all(|&v| v < p));
            polys.push(Poly::from_coeffs(&cs));
        }
        let selection_ns = take();
        assert!(it.next().is_none());
        let c = Curve::new(Fp::new(p), polys[0]);
        let d1 = Div {
            u: polys[1],
            v: polys[2],
        };
        assert_eq!(c.g, g);
        assert_eq!(c.f.lc(), 1);
        assert!(c.is_valid(&d1) && !c.is_zero(&d1));
        assert!(c.is_zero(&c.mul(&d1, ell)));
        assert!((ell as u128).pow(2) > upper(p, g));
        Self {
            c,
            d1,
            ell,
            family,
            attempt,
            selection_ns,
        }
    }
    fn target(&self, s: u64) -> (u64, Div) {
        let seed =
            9000001 ^ ((self.c.g as u64) << 48) ^ (self.c.fp.p << 16) ^ (self.family << 8) ^ s;
        let k = 1 + Rng::new(seed).below(self.ell - 1);
        (k, self.c.mul(&self.d1, k))
    }
}
fn worker(path: &str, s: u64, arm: &str) {
    let f = Fixture::parse(&fs::read_to_string(path).unwrap());
    let (expected, target) = f.target(s);
    let (g, p) = (f.c.g, f.c.fp.p);
    if arm.starts_with("rational-") {
        let requested = if g == 2 {
            let mut h = 6;
            loop {
                let mut v = 1u64;
                for j in 0..6 {
                    v = v * (h - j) / (j + 1);
                }
                if 32 * v >= 4 * p {
                    break h as usize;
                }
                h += 1;
            }
        } else {
            (p as f64).powf(1.0 - 1.0 / g as f64).ceil() as usize
        };
        let gate = arm == "rational-linear-gate";
        let report = if arm == "rational-generic" {
            rational_solve::solve(
                &f.c,
                &f.d1,
                &target,
                f.ell,
                20260916 + s,
                requested,
                2,
                gate,
            )
        } else {
            assert!(matches!(
                arm,
                "rational-linear" | "rational-linear-aa" | "rational-linear-gate"
            ));
            rational_solve::solve_linear(&f.c, &f.d1, &target, f.ell, 20260916 + s, requested, gate)
        };
        let r = &report.base;
        let check = Instant::now();
        let correct = r.k == Some(expected) && r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":{gate},\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"anchors\":{},\"requested_h\":{},\"m\":{},\"trials\":{},\"constructed\":{},\"tuples\":{},\"splits\":{},\"admitted\":{},\"duplicates\":{},\"pure_rows\":{},\"unions\":{},\"pins\":{},\"descent_steps\":{},\"common\":{},\"normalized_admitted\":{},\"raw_checks\":{},\"pure_checks\":{},\"status\":\"{}\"}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,requested,r.m,r.draws,report.constructed,r.unique_tuples,r.splits,r.admitted,r.duplicate_relations,r.pure_rows,r.unions,r.pins,r.descent_steps,report.common,report.normalized_admitted,r.raw_checks,r.pure_checks,report.status);
        if !correct {
            std::process::exit(3);
        }
        return;
    }
    let gate = arm == "compact-gate";
    if matches!(arm, "compact" | "compact-gate" | "orbit") {
        let r = if arm == "orbit" {
            anchor::solve_orbit(&f.c, &f.d1, &target, f.ell, 20260916 + s, false)
        } else {
            anchor::solve_compact(&f.c, &f.d1, &target, f.ell, 20260916 + s, gate)
        };
        let check = Instant::now();
        let group = r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let correct = r.k == Some(expected) && group;
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":{gate},\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"anchors\":{},\"m\":{},\"trials\":{},\"tuples\":{},\"admitted\":{},\"pure_rows\":{},\"raw_checks\":{},\"pure_checks\":{},\"reported_bytes\":{}}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,r.m,r.draws,r.unique_tuples,r.admitted,r.pure_rows,r.raw_checks,r.pure_checks,r.memory_bytes);
        if !correct {
            std::process::exit(3);
        }
    } else {
        let (table, branches, solver, reference) = match arm {
            "n1fr" => (false, 1, Solver::Fast, true),
            "n1fd" => (false, 1, Solver::Fast, false),
            "n1hr" => (false, 1, Solver::Hybrid, true),
            "n1hd" => (false, 1, Solver::Hybrid, false),
            "n64fr" => (false, 64, Solver::Fast, true),
            "n64fd" => (false, 64, Solver::Fast, false),
            "n64hr" => (false, 64, Solver::Hybrid, true),
            "n64hd" => (false, 64, Solver::Hybrid, false),
            "t1f" | "t1faa" => (true, 1, Solver::Fast, true),
            "t1h" => (true, 1, Solver::Hybrid, true),
            "t64f" => (true, 64, Solver::Fast, true),
            "t64h" => (true, 64, Solver::Hybrid, true),
            _ => panic!("unknown arm"),
        };
        let params = IcParams {
            branches,
            solver,
            reference_oracle: reference,
            max_trials: 2_000_000,
            seed: 20260916 + s,
            ..IcParams::default()
        };
        let r = if table {
            pipeline::solve_table(&f.c, &f.d1, &target, f.ell, &params)
        } else {
            ic::solve_dlp(&f.c, &f.d1, &target, f.ell, &params)
        };
        let check = Instant::now();
        let correct = r.k == Some(expected) && r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":false,\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"fb_ns\":{},\"walk_ns\":{},\"oracle_ns\":{},\"row_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"m\":{},\"trials\":{},\"relations\":{},\"solve_rounds\":{}}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.fb_ns,r.walk_ns,r.oracle_ns,r.row_ns,r.solve_ns,r.m,r.trials,r.relations,r.solve_rounds);
        if !correct {
            std::process::exit(3);
        }
    }
}
#[path = "../hec_dlp_graph_20261115/core.rs"]
mod core;
mod engine;
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let seed: u64 = a[2].parse().unwrap();
    let arm = &a[3];
    if !matches!(arm.as_str(), "zero" | "positive" | "zero-aa" | "zero-gate") {
        worker(&a[1], seed, arm);
        return;
    }
    let fix = Fixture::parse(&fs::read_to_string(&a[1]).unwrap());
    let (expected, target) = fix.target(seed);
    assert_eq!(fix.c.g, 2);
    let h = (fix.c.fp.p as f64).sqrt().ceil() as usize;
    let gate = arm == "zero-gate";
    let hs = if gate {
        vec![0, 1, h]
    } else {
        vec![if arm == "positive" { h } else { 0 }]
    };
    let (r, tracks) = engine::run(
        &fix.c, &fix.d1, &target, fix.ell, seed, &hs, gate, true, 32768,
    );
    let check = Instant::now();
    let correct =
        r.k.map_or(true, |k| k == expected && fix.c.mul(&fix.d1, k) == target);
    assert!(correct);
    let extra = check.elapsed().as_nanos();
    let k = r.k.map_or("null".into(), |k| k.to_string());
    let status = if r.k.is_some() { "solved" } else { "trial_cap" };
    println!("CELL {{\"g\":2,\"p\":{},\"seed\":{seed},\"arm\":\"{arm}\",\"gate\":{gate},\"k\":{k},\"expected_k\":{expected},\"correct\":{correct},\"status\":\"{status}\",\"total_ns\":{},\"setup_ns\":{},\"walk_ns\":{},\"oracle_ns\":{},\"row_ns\":{},\"extra_check_ns\":{extra},\"trials\":{},\"smooth\":{},\"hash\":\"{}\",\"h\":{},\"forest_bytes\":{},\"target_u\":{:?},\"target_v\":{:?}}}",fix.c.fp.p,r.total,r.setup,r.walk,r.oracle,r.rows,r.trials,r.smooth,r.hash,hs[0],tracks[0].core.forest.bytes(),&target.u.c[..target.u.n],&target.v.c[..target.v.n]);
}
