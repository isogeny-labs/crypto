//! Tests of the presentation panel (`[pcc:unverified]` instrument tests, no claim).
use super::*;
use crate::algebra::Pol;
use crate::cell::{derive, Cell};
use crate::ec::{add_opt, Aff};
use crate::mpoly::{self, Last};
use crate::present::{self, Inst, PRESENTATIONS};
use crate::targets::symbols;
use crate::util::*;
use crypto_lib::cryptanalysis::semaev_higher::{s4_in_last, s5_in_last};
use crypto_lib::ecc::field::FieldElement;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::BTreeSet;
use std::sync::OnceLock;

fn cell_t2() -> &'static Cell {
    static C: OnceLock<Cell> = OnceLock::new();
    C.get_or_init(|| derive(2, 9, 777, false).unwrap().cell)
}
fn cell_t3() -> &'static Cell {
    static C: OnceLock<Cell> = OnceLock::new();
    C.get_or_init(|| derive(3, 6, 777, false).unwrap().cell)
}
fn inst(c: &Cell, m: usize, x_r: u64) -> Inst {
    Inst { p: c.p, a: c.a, b: c.b, g: c.g, zeta: c.zeta, m, t: c.t, x_r }
}
fn tmp(name: &str) -> String {
    let d = std::env::temp_dir().join(format!("pkm_present_{}_{name}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d.display().to_string()
}

/// Random ordered signed real base points (and their abscissa sum) with no identity prefix.
fn planted(c: &Cell, m: usize, rng: &mut StdRng) -> (Vec<Aff>, u64) {
    let sym = symbols(c);
    loop {
        let pts: Vec<Aff> = (0..m).map(|_| sym[rng.gen_range(0..sym.len())]).collect();
        let mut acc: Option<Aff> = None;
        let mut ok = true;
        for &q in &pts {
            acc = add_opt(c.p, c.a, acc, Some(q));
            if acc.is_none() {
                ok = false;
                break;
            }
        }
        if ok {
            return (pts.clone(), acc.unwrap().0);
        }
    }
}

fn distinct(pts: &[Aff]) -> bool {
    let s: BTreeSet<u64> = pts.iter().map(|q| q.0).collect();
    s.len() == pts.len()
}

#[test]
fn a_chain_tower_is_byte_identical_to_the_m8_exporter() {
    // generic writer and builder on several shapes
    for c in [cell_t2(), cell_t3()] {
        for m in 2..=5usize {
            let s = inst(c, m, 12345);
            let sys = present::build("chain-tower", &s).unwrap();
            let spec = system::Spec { p: c.p, a: c.a, b: c.b, g: c.g, zeta: c.zeta, m, t: c.t, x_r: 12345 };
            let (polys, _) = system::build(&spec);
            assert_eq!(sys.polys, polys);
            assert_eq!(present::ms_text(&sys), system::ms_text(&polys, m, c.t, c.p), "m={m}");
        }
    }
    // the generic term writer applied to the chain polynomials gives the same text as the tower-literal writer
    // on the head part (guards `present::ms_text` against drift)
    // end to end: files of both exporters
    let c = cell_t3();
    let (cd, td, o1, o2) = (tmp("a_cell"), tmp("a_targets"), tmp("a_new"), tmp("a_old"));
    write_json(&std::path::Path::new(&cd).join("cell.json"), &derive(3, 6, 777, false).unwrap().json).unwrap();
    targets::run(&[
        "--cell".into(), cd.clone(), "--n-sat".into(), "1".into(), "--n-unsat".into(), "1".into(), "--planted".into(), "1".into(),
        "--seed".into(), "5".into(), "--m".into(), "4".into(), "--out".into(), td.clone(),
    ])
    .unwrap();
    let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    cli::run_export(&a(&["--cell", &cd, "--targets", &td, "--target", "all", "--m", "4", "--presentation", "chain-tower", "--out", &o1])).unwrap();
    system::run_export(&a(&["--cell", &cd, "--targets", &td, "--out", &o2, "--format", "both"])).unwrap();
    let tj = read_json(&std::path::Path::new(&td).join("targets.json")).unwrap();
    for tg in tj["targets"].as_array().unwrap() {
        let name = tg["name"].as_str().unwrap();
        let new_ms = std::fs::read(format!("{o1}/{name}.chain-tower.ms")).unwrap();
        let old_ms = std::fs::read(format!("{o2}/{name}.ms")).unwrap();
        assert_eq!(new_ms, old_ms, "{name}: .ms bytes");
        let nj = read_json(std::path::Path::new(&format!("{o1}/{name}.chain-tower.system.json"))).unwrap();
        let oj = read_json(std::path::Path::new(&format!("{o2}/{name}.system.json"))).unwrap();
        assert!(check_seal(&nj));
        for (k, v) in oj.as_object().unwrap() {
            if k != "schema" {
                assert_eq!(&nj[k], v, "{name}: system.json key {k}");
            }
        }
    }
    let _ = c;
}

#[test]
fn b_planted_decompositions_are_zeros_of_every_presentation() {
    let mut rng = StdRng::seed_from_u64(11);
    for (c, ms) in [(cell_t2(), vec![3usize, 4]), (cell_t3(), vec![3, 4])] {
        for m in ms {
            let reps = if m == 3 { 4 } else { 2 };
            for _ in 0..reps {
                let (pts, x_r) = planted(c, m, &mut rng);
                let s = inst(c, m, x_r);
                for pres in PRESENTATIONS {
                    let sys = present::build(pres, &s).unwrap();
                    let Some(asg) = present::planted_assignment(&sys, &pts) else {
                        continue; // identity subtree sum in this arm's fixed order
                    };
                    let vals: Vec<u64> = sys.polys.iter().map(|f| present::eval_terms(f, &asg, c.p)).collect();
                    if present::is_symmetric(pres) && !distinct(&pts) {
                        // S part vanishes, divisibility part cannot (P not squarefree)
                        assert_eq!(vals[0], 0, "{pres} S part");
                        assert!(vals[1..].iter().any(|&v| v != 0), "repeated abscissa must be excluded");
                    } else {
                        assert!(vals.iter().all(|&v| v == 0), "t={} m={m} {pres}: {vals:?}", c.t);
                    }
                }
            }
        }
    }
}

fn fe(v: u64, p: u64) -> FieldElement {
    FieldElement::new(BigUint::from(v), BigUint::from(p))
}

#[test]
fn c_resultant_identity_against_semaev_higher() {
    let c = cell_t3();
    let (p, a, b) = (c.p, c.a, c.b);
    let mut rng = StdRng::seed_from_u64(21);
    for m in [3usize, 4] {
        let n = mpoly::s_ring(m);
        let f = mpoly::summation_full(m, Last::Var(2 * m), a, b, p);
        for _ in 0..6 {
            let xs: Vec<u64> = (0..=m).map(|_| rng.gen_range(0..p)).collect();
            let mut pt = vec![0u64; n];
            pt[..m].copy_from_slice(&xs[..m]);
            pt[2 * m] = xs[m];
            assert!(f.t.keys().all(|e| e[m..2 * m].iter().all(|&k| k == 0)), "wire left");
            let mine = f.eval(&pt, p);
            let f_: Vec<FieldElement> = xs.iter().map(|&v| fe(v, p)).collect();
            let up = if m == 3 {
                s4_in_last(&f_[0], &f_[1], &f_[2], &fe(a, p), &fe(b, p))
            } else {
                s5_in_last(&f_[0], &f_[1], &f_[2], &f_[3], &fe(a, p), &fe(b, p))
            };
            let theirs = up.eval(&f_[m]);
            assert_eq!(BigUint::from(mine), theirs.value, "m={m}");
        }
        // degree in each variable: S_4 has 4, S_5 has 8
        let deg = (0..=m).map(|i| f.t.keys().map(|e| e[if i == m { 2 * m } else { i }]).max().unwrap()).collect::<Vec<_>>();
        assert!(deg.iter().all(|&d| d as usize == 1 << (m - 1)), "m={m} degrees {deg:?}");
        // exact equality with the oracle program's S_(m+1) (interpolation / explicit resultant)
        let x_r = rng.gen_range(0..p);
        let mine = mpoly::summation(m, x_r, a, b, p);
        let theirs = algebra::summation_poly(m, x_r, a, b, p);
        assert_eq!(mine.t.len(), theirs.t.len());
        for (e, v) in &theirs.t {
            assert_eq!(mine.t.get(e), Some(v), "m={m} coefficient {e:?}");
        }
    }
}

#[test]
fn d_symmetric_rewrite_identity() {
    let c = cell_t3();
    let p = c.p;
    let mut rng = StdRng::seed_from_u64(31);
    for m in [3usize, 4] {
        let x_r = rng.gen_range(0..p);
        let s = mpoly::summation(m, x_r, c.a, c.b, p);
        let se = mpoly::to_elementary(&s, m, p).unwrap();
        for _ in 0..8 {
            let xs: Vec<u64> = (0..m).map(|_| rng.gen_range(0..p)).collect();
            let e = mpoly::elem_values(&xs, p);
            assert_eq!(s.eval(&xs, p), se.eval(&e, p), "m={m}");
        }
        // permutations are zeros together
        let mut xs: Vec<u64> = (0..m).map(|_| rng.gen_range(0..p)).collect();
        let v = s.eval(&xs, p);
        xs.reverse();
        assert_eq!(v, s.eval(&xs, p));
    }
    // a non-symmetric polynomial is refused
    let f = Pol::var(3, 0).mul(&Pol::var(3, 0), p).add(&Pol::var(3, 1), p);
    assert!(mpoly::to_elementary(&f, 3, p).is_err());
}

#[test]
fn e_remainder_vanishes_exactly_for_roots_in_the_kummer_base() {
    let mut rng = StdRng::seed_from_u64(41);
    for c in [cell_t2(), cell_t3()] {
        let (p, t) = (c.p, c.t);
        let big = 1u64 << t;
        let cc = powm(c.g, big, p);
        for m in 2..=4usize.min(big as usize) {
            let rem = mpoly::x_pow_2t_mod(m, t, p);
            let vanishes = |xs: &[u64]| -> bool {
                let e = mpoly::elem_values(xs, p);
                rem.iter().enumerate().all(|(k, r)| {
                    let v = r.eval(&e, p);
                    if k == 0 {
                        v == cc
                    } else {
                        v == 0
                    }
                })
            };
            let in_v = |x: u64| c.v.contains(&x);
            for _ in 0..40 {
                // roots: each from V or random in F_p, repeats allowed
                let xs: Vec<u64> = (0..m)
                    .map(|_| if rng.gen_bool(0.7) { c.v[rng.gen_range(0..c.v.len())] } else { rng.gen_range(1..p) })
                    .collect();
                let all_in = xs.iter().all(|&x| in_v(x));
                let set: BTreeSet<u64> = xs.iter().copied().collect();
                let want = all_in && set.len() == xs.len();
                assert_eq!(vanishes(&xs), want, "t={t} m={m} xs={xs:?}");
            }
            // every distinct m-subset of V vanishes
            let mut sub: Vec<u64> = c.v.clone();
            sub.truncate(m);
            assert!(vanishes(&sub));
            // a repeated root of V does not
            let mut rep = sub.clone();
            rep[1] = rep[0];
            assert!(!vanishes(&rep));
            // random e-vectors (random roots) do not
            let rnd: Vec<u64> = (0..m).map(|_| rng.gen_range(1..p)).collect();
            assert!(!vanishes(&rnd));
        }
    }
}

fn args(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn decomp_set(j: &serde_json::Value) -> BTreeSet<Vec<i64>> {
    j["extraction"]["decompositions_signed_base"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d.as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect())
        .collect()
}

#[test]
fn f_solve_extract_agrees_with_the_classifier_m3() {
    let c = cell_t3();
    let (cd, td, od) = (tmp("f_cell"), tmp("f_targets"), tmp("f_out"));
    write_json(&std::path::Path::new(&cd).join("cell.json"), &derive(3, 6, 777, false).unwrap().json).unwrap();
    targets::run(&args(&[
        "--cell", &cd, "--n-sat", "2", "--n-unsat", "2", "--planted", "1", "--seed", "5", "--m", "3", "--out", &td,
    ]))
    .unwrap();
    let tj = read_json(&std::path::Path::new(&td).join("targets.json")).unwrap();
    let mut sat_seen = 0;
    let mut unsat_seen = 0;
    for tg in tj["targets"].as_array().unwrap() {
        let name = tg["name"].as_str().unwrap();
        let want = tg["real_count"].as_u64().unwrap() as usize;
        let label = tg["label"].as_str().unwrap();
        if label == "sat" {
            sat_seen += 1;
            assert!(want >= 1);
        }
        if label == "unsat" {
            unsat_seen += 1;
            assert_eq!(want, 0);
        }
        cli::run_export(&args(&["--cell", &cd, "--targets", &td, "--target", name, "--m", "3", "--presentation", "all", "--out", &od])).unwrap();
        let mut sets: Vec<(&str, BTreeSet<Vec<i64>>)> = Vec::new();
        for pres in ["mono-direct", "chain-direct", "chain-tower", "sym-direct", "sym-tower"] {
            let sp = format!("{od}/{name}.{pres}.system.json");
            let out = format!("{od}/{name}.{pres}.solve.json");
            cli::run_solve(&args(&["--system", &sp, "--max-degree", "64", "--deadline", "60", "--out", &out, "--extract"])).unwrap();
            let j = read_json(std::path::Path::new(&out)).unwrap();
            assert!(check_seal(&j));
            assert_eq!(j["censored"], "no", "{name} {pres}");
            for k in ["solve_field_multiplications", "extract_field_multiplications", "threads_used", "fp_rational_solutions", "unlifted_solutions", "construction_field_multiplications", "construction_cell_field_multiplications", "nterms_total", "max_total_degree"] {
                assert!(j[k].is_u64(), "{k}");
            }
            assert!(j["verified_real_decompositions"].is_array());
            assert_eq!(j["verified_real_decompositions"].as_array().unwrap().len() as u64, j["verified_real_decompositions_count"].as_u64().unwrap());
            assert!(["ordered_repeats", "ordered_repeats_unlifted", "unordered_distinct"].contains(&j["solution_semantics"].as_str().unwrap()));
            let prog = std::fs::read_to_string(format!("{out}.progress.json")).unwrap();
            let last: serde_json::Value = serde_json::from_str(prog.lines().last().unwrap()).unwrap();
            for k in ["ops", "degree", "step", "elapsed_s"] {
                assert!(!last[k].is_null(), "progress {k}");
            }
            assert_eq!(j["presentation"], pres);
            for k in ["nvars", "nequations", "max_input_degree", "solving_degree", "degree_reached", "field_ops", "basis_size", "cpu_s"] {
                assert!(!j[k].is_null(), "{k}");
            }
            assert_eq!(j["extraction"]["equation_check_failures"], 0);
            sets.push((pres, decomp_set(&j)));
        }
        let mono = &sets[0].1;
        let chain_direct = &sets[1].1;
        let chain_tower = &sets[2].1;
        let sym = &sets[3].1;
        let sym_tower = &sets[4].1;
        assert_eq!(mono.len(), want, "{name} mono-direct vs classifier");
        assert_eq!(chain_direct, mono, "{name} chain-direct");
        assert_eq!(chain_tower, mono, "{name} chain-tower");
        let distinct_only: BTreeSet<Vec<i64>> = mono
            .iter()
            .filter(|d| {
                let s: BTreeSet<i64> = d.iter().map(|x| x.abs()).collect();
                s.len() == d.len()
            })
            .cloned()
            .collect();
        assert_eq!(sym, &distinct_only, "{name} sym-direct (unordered, distinct abscissae)");
        assert_eq!(sym_tower, &distinct_only, "{name} sym-tower (unordered, distinct abscissae)");
        let ct = tg["real_count"].as_u64().unwrap();
        let _ = ct;
    }
    assert!(sat_seen >= 2 && unsat_seen >= 2);
    let _ = c;
}

#[test]
fn g_msolve_parametrisation_path_if_msolve_is_installed() {
    let Ok(out) = std::process::Command::new("msolve").arg("-h").output() else { return };
    let _ = out;
    let (cd, td, od) = (tmp("g_cell"), tmp("g_targets"), tmp("g_out"));
    write_json(&std::path::Path::new(&cd).join("cell.json"), &derive(3, 6, 777, false).unwrap().json).unwrap();
    targets::run(&args(&["--cell", &cd, "--n-sat", "1", "--n-unsat", "0", "--planted", "0", "--seed", "5", "--m", "3", "--out", &td])).unwrap();
    let tj = read_json(&std::path::Path::new(&td).join("targets.json")).unwrap();
    let tg = &tj["targets"][0];
    let name = tg["name"].as_str().unwrap();
    cli::run_export(&args(&["--cell", &cd, "--targets", &td, "--target", name, "--m", "3", "--presentation", "all", "--out", &od])).unwrap();
    for pres in PRESENTATIONS {
        let ms = format!("{od}/{name}.{pres}.ms");
        let mo = format!("{od}/{name}.{pres}.msolve.txt");
        let st = std::process::Command::new("msolve").args(["-t", "1", "-P", "1", "-f", &ms, "-o", &mo]).output().unwrap();
        assert!(st.status.success(), "{pres}");
        let vo = format!("{od}/{name}.{pres}.verify.json");
        cli::run_verify_msolve(&args(&["--system-json", &format!("{od}/{name}.{pres}.system.json"), "--msolve-out", &mo, "--out", &vo])).unwrap();
        let j = read_json(std::path::Path::new(&vo)).unwrap();
        assert!(check_seal(&j));
        let real = tg["real_count"].as_u64().unwrap();
        let got = j["verified_real_decompositions_count"].as_u64().unwrap();
        if present::is_symmetric(pres) {
            assert_eq!(got, j["extraction"]["verified_distinct_abscissa"].as_u64().unwrap());
            assert!(got <= real);
        } else {
            assert_eq!(got, real, "{pres}");
        }
    }
}

#[test]
fn h_tree_has_the_documented_shape() {
    let c = cell_t2();
    let s = inst(c, 4, 777);
    let sys = present::build("tree-tower", &s).unwrap();
    assert_eq!(sys.wire_leaves, vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(sys.head, 3);
    let s5 = inst(c, 5, 777);
    let sys5 = present::build("tree-tower", &s5).unwrap();
    // pairs (0,1), (2,3), then (u0,u1) -> u2, last box (u2, x4)
    assert_eq!(sys5.wire_leaves, vec![vec![0, 1], vec![2, 3], vec![0, 1, 2, 3]]);
    assert_eq!(sys5.head, 4);
    // at m = 3 tree equals chain
    let s3 = inst(c, 3, 777);
    assert_eq!(present::build("tree-tower", &s3).unwrap().polys, present::build("chain-tower", &s3).unwrap().polys);
}


// ---- sym-tower against the prior study's construction ---------------------------------

/// Verbatim port of the prior study's `Rng` (`examples/pkm_symmetric_tower.rs`).
struct ExRng(u64);
impl ExRng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// The instance the prior study draws for `(p = 786433, m, t)` (same seeds, same draw order).
fn example_instance(m: usize, t: usize) -> Inst {
    let p = 786_433u64;
    let mut rng = ExRng(0x504B_4D53 ^ ((m as u64) << 32) ^ t as u64);
    let zeta = loop {
        let h = 2 + rng.below(p - 2);
        if powm(h, (p - 1) / 2, p) == p - 1 {
            break powm(h, (p - 1) >> t, p);
        }
    };
    let (a, b, g) = (rng.below(p), 1 + rng.below(p - 1), 2 + rng.below(p - 2));
    let x_r = rng.below(p);
    Inst { p, a, b, g, zeta, m, t, x_r }
}

#[test]
fn i_sym_tower_equals_the_prior_studys_system() {
    // The prior study's builder is a binary example (own `main`, private items, `//!` header), so it is not
    // callable from here. Its functions were ported verbatim into pkm_oracle_program/algebra.rs, which this
    // crate includes; (1) the texts are identical (up to the `pub(super)` and constructor names), (2) the
    // system of the port equals ours on the prior study's cells (p = 786433, same seeds).
    let ex = include_str!("../pkm_symmetric_tower.rs");
    let al = include_str!("../pkm_oracle_program/algebra.rs");
    let norm = |s: &str| {
        s.lines()
            .map(|l| l.trim().replace("pub(super) ", "").replace("Pol::constant(", "Pol::c("))
            .filter(|l| !l.starts_with("///") && !l.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let fn_text = |src: &str, head: &str| {
        let s = src.find(head).unwrap_or_else(|| panic!("{head}"));
        let e = src[s..].find("\n}\n").unwrap() + s + 3;
        norm(&src[s..e])
    };
    for head in ["fn symmetric_system(", "fn to_elementary(", "fn summation_poly(", "fn s3(", "fn summation_poly_5("] {
        let h2 = if head == "fn s3(" || head == "fn symmetric_system(" || head == "fn to_elementary(" || head == "fn summation_poly(" || head == "fn summation_poly_5(" { head } else { head };
        assert_eq!(fn_text(ex, h2), fn_text(al, h2), "{head}");
    }
    for (m, t) in [(2usize, 2usize), (3, 2), (3, 3), (4, 2)] {
        let s = example_instance(m, t);
        let inst = algebra::Instance { p: s.p, a: s.a, b: s.b, g: s.g, zeta: s.zeta, t };
        let (theirs, n) = algebra::symmetric_system(&inst, m, s.x_r);
        let sys = present::build("sym-tower", &s).unwrap();
        assert_eq!(sys.nvars(), n);
        let theirs: Vec<present::Terms> = theirs.iter().map(present::sorted_terms).filter(|f| !f.is_empty()).collect();
        assert_eq!(sys.polys, theirs, "m={m} t={t}");
        // the prior study's planted check: roots of Q give a zero of every equation
        let mut rng = ExRng(5);
        let idx: Vec<usize> = {
            let mut v = vec![];
            while v.len() < m.min(1 << t) {
                let k = rng.below(1 << t) as usize;
                if !v.contains(&k) {
                    v.push(k);
                }
            }
            v
        };
        if idx.len() == m {
            let w: Vec<u64> = idx.iter().map(|&k| powm(s.zeta, k as u64, s.p)).collect();
            let pt = mpoly::sym_tower_point(&w, t, s.p);
            // S_{m+1} vanishes only for a real decomposition; the tower part vanishes for every distinct subset
            for f in &sys.polys[1..] {
                assert_eq!(present::eval_terms(f, &pt, s.p), 0);
            }
        }
    }
}

#[test]
fn j_construction_is_counted_and_split() {
    let c = cell_t3();
    let s = inst(c, 3, 4242);
    let mut pre = present::CellPre::new(&s);
    for pres in PRESENTATIONS {
        let sys = present::build_pre(&mut pre, pres, &s).unwrap();
        assert!(sys.cost.target_muls > 0, "{pres}");
        let per_cell = pres.starts_with("mono") || present::is_symmetric(pres);
        assert_eq!(sys.cost.cell_muls > 0, per_cell, "{pres}");
        assert!(sys.nterms_total() > 0 && sys.max_input_degree() > 0);
    }
    // the per-cell part does not depend on the target; a second target reuses it
    let s2 = inst(c, 3, 99);
    let a = present::build_pre(&mut pre, "sym-direct", &s).unwrap().cost;
    let b = present::build_pre(&mut pre, "sym-direct", &s2).unwrap().cost;
    assert_eq!(a.cell_muls, b.cell_muls);
    // and the split build equals the one-shot S_(m+1) polynomial
    let one = mpoly::summation(3, s2.x_r, c.a, c.b, c.p);
    let split = mpoly::eval_last(&mpoly::summation_by_last(3, c.a, c.b, c.p), s2.x_r, c.p);
    assert_eq!(one.t, split.t);
}

#[test]
fn k_cell_wrapper_nests_bases_and_targets_record_distinct_counts() {
    let d = tmp("k_cell0");
    cli::run_cell(&args(&["--t", "4", "--ratio", "5", "--seed", "7", "--out", &d, "--require-breal", "6", "--slug", "kslug"])).unwrap();
    let c0 = Cell::load(std::path::Path::new(&d)).unwrap();
    assert_eq!(c0.slug, "kslug");
    assert!(c0.real.len() >= 6 && c0.t == 4);
    let d2 = tmp("k_cell1");
    let p = c0.p.to_string();
    cli::run_cell(&args(&["--t", "4", "--ratio", "5", "--seed", "7", "--out", &d2, "--prime", &p, "--nested-ts", "2,3", "--slug", "kslug2"])).unwrap();
    let big = Cell::load(std::path::Path::new(&d2)).unwrap();
    assert_eq!((big.p, big.a, big.b, big.g), (c0.p, c0.a, c0.b, c0.g));
    let rec = read_json(std::path::Path::new(&d2).join("cell.present.json").as_path()).unwrap();
    assert!(check_seal(&rec) && rec["flags"]["no_l1_reuse"] == true);
    for t2 in [3usize, 2] {
        let nd = format!("{d2}/nested_t{t2}");
        let n = Cell::load(std::path::Path::new(&nd)).unwrap();
        assert_eq!((n.p, n.a, n.b, n.g, n.r), (big.p, big.a, big.b, big.g, big.r));
        assert_eq!(n.t, t2);
        assert_eq!(n.zeta, powm(big.zeta, 1 << (4 - t2), big.p));
        assert!(n.v.iter().all(|x| big.v.contains(x)), "nested base is a subset");
        assert_ne!(n.slug, big.slug);
    }
    // a pinned prime that the rule would step off is an error, never a silent step
    assert!(cli::run_cell(&args(&["--t", "4", "--ratio", "5", "--seed", "7", "--out", &tmp("k_bad"), "--prime", &(c0.p + 16).to_string()])).is_err());
    // targets with planted-distinct on the nested t = 3 cell
    let nd = format!("{d2}/nested_t3");
    let td = tmp("k_targets");
    cli::run_targets(&args(&["--cell", &nd, "--n-sat", "1", "--n-unsat", "1", "--planted", "0", "--planted-distinct", "2", "--seed", "3", "--m", "3", "--out", &td])).unwrap();
    let tj = read_json(&std::path::Path::new(&td).join("targets.json")).unwrap();
    assert!(check_seal(&tj));
    assert_eq!(tj["slug"], "kslug2-nested-t3");
    let mut pd = 0;
    for tg in tj["targets"].as_array().unwrap() {
        assert!(tg["real_count_distinct"].as_u64().unwrap() <= tg["real_count"].as_u64().unwrap());
        if tg["label"] == "planted_distinct" {
            assert_eq!(tg["planted_abscissae"].as_array().unwrap().len(), 3);
            pd += 1;
            assert!(tg["real_count_distinct"].as_u64().unwrap() >= 1);
        }
    }
    assert_eq!(pd, 2);
}

#[test]
fn l_export_all_skips_symmetric_arms_when_not_applicable() {
    let (cd, td, od) = (tmp("l_cell"), tmp("l_targets"), tmp("l_out"));
    write_json(&std::path::Path::new(&cd).join("cell.json"), &derive(2, 9, 777, false).unwrap().json).unwrap();
    targets::run(&args(&["--cell", &cd, "--n-sat", "1", "--n-unsat", "1", "--planted", "0", "--seed", "5", "--m", "3", "--out", &td])).unwrap();
    cli::run_export(&args(&["--cell", &cd, "--targets", &td, "--target", "all", "--m", "3", "--presentation", "all", "--out", &od])).unwrap();
    let tj = read_json(&std::path::Path::new(&td).join("targets.json")).unwrap();
    let name = tj["targets"][0]["name"].as_str().unwrap();
    let man = read_json(&std::path::Path::new(&od).join(format!("{name}.manifest.json"))).unwrap();
    assert_eq!(man["sym_applicable"], false);
    let sk: Vec<&str> = man["entries"].as_array().unwrap().iter().filter(|e| e["skipped"] == true).map(|e| e["presentation"].as_str().unwrap()).collect();
    assert_eq!(sk, vec!["sym-direct", "sym-tower"]);
    assert!(!std::path::Path::new(&od).join(format!("{name}.sym-direct.ms")).exists());
}
