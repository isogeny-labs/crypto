//! Subgroup traps in lift-dense factor bases (PROTOCOL.md, Amendment 1).
//!
//! The field, curve, factor-base and counting code is the frozen C1
//! instrument, included unchanged from research/c1_twisted_line_ladder_20261027
//! (checked against its FREEZE.sha256 before every run).
//!
//! Modes:
//!   c1 <c1_run_dir> <k>     P1 (A0 membership), P3 (2E zeros), identity check
//!   mass <k>                P2 (exact mass of A0 std over E(F_{Q^2}))
//!   audit <c2pnb_params> <oakley_rows.json>   P4 (trap condition on every
//!                           applicable line)
#![allow(dead_code)]

#[path = "../../c1_twisted_line_ladder_20261027/src/common.rs"]
mod common;
#[path = "../../c1_twisted_line_ladder_20261027/src/curve.rs"]
mod curve;
#[path = "../../c1_twisted_line_ladder_20261027/src/experiment.rs"]
mod experiment;
#[path = "../../c1_twisted_line_ladder_20261027/src/gf.rs"]
mod gf;
#[path = "../../c1_twisted_line_ladder_20261027/src/json.rs"]
mod json;
#[path = "../../c1_twisted_line_ladder_20261027/src/semaev.rs"]
mod semaev;
#[path = "../../c1_twisted_line_ladder_20261027/src/solver.rs"]
mod solver;
#[path = "../../c1_twisted_line_ladder_20261027/src/yield_.rs"]
mod yield_;

mod gf2n;

use common::*;
use curve::*;
use experiment::{curves_for_k, SEED};
use gf::*;
use yield_::PairTable;

/// sigma_Q on points
fn frob_pt(f: &Fq4, p: &Pt) -> Pt {
    if p.inf {
        *p
    } else {
        Pt { x: f.frob(&p.x), y: f.frob(&p.y), inf: false }
    }
}

fn in_eq2(f: &Fq4, p: &Pt) -> bool {
    let q = frob_pt(f, &frob_pt(f, p));
    q == *p
}

/// absolute trace of an element of F_Q (as u16)
fn tr_fq(f: &Fq4, s: u16) -> u16 {
    let (mut t, mut z) = (0u16, s);
    for _ in 0..f.f.k {
        t ^= z;
        z = f.f.sq(z);
    }
    assert!(t <= 1);
    t
}

/// R in H = E(F_Q) + ker(sigma_Q + 1)  <=>  R in E(F_{Q^2}) and
/// R + sigma_Q R in 2E(F_Q)  (halving criterion over F_Q: Tr_k(x) = Tr_k(a)).
fn in_h(cv: &Curve, r: &Pt, tr_a_fq: u16) -> bool {
    let f = cv.f;
    if !in_eq2(f, r) {
        return false;
    }
    let s = cv.add(r, &frob_pt(f, r));
    if s.inf {
        return true;
    }
    assert!(in_fq(f, &s.x) && in_fq(f, &s.y));
    tr_fq(f, s.x[0]) == tr_a_fq
}

/// least j >= 1 with z^(2^j) = z
fn deg_abs(f: &Fq4, z: &E4) -> u32 {
    let mut w = f.sq(z);
    let mut j = 1;
    while w != *z {
        w = f.sq(&w);
        j += 1;
    }
    j
}

fn lcm(a: u32, b: u32) -> u32 {
    fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    a / gcd(a, b) * b
}

/// trap iff [K:F_2] < N and 2 [K:F_2] != N (Amendment 1)
fn trapped(deg_k: u32, n: u32) -> bool {
    deg_k < n && 2 * deg_k != n
}

fn count_all(cv: &Curve, tab: &PairTable, pts: &[Pt], threads: usize) -> Vec<u64> {
    let mut out = vec![0u64; pts.len()];
    if pts.is_empty() {
        return out;
    }
    let chunk = pts.len().div_ceil(threads);
    std::thread::scope(|s| {
        for (tc, cc) in pts.chunks(chunk).zip(out.chunks_mut(chunk)) {
            s.spawn(move || {
                for (t, c) in tc.iter().zip(cc.iter_mut()) {
                    *c = if t.inf { tab.count_o() } else { tab.count(cv, t) };
                }
            });
        }
    });
    out
}

fn threads() -> usize {
    std::env::var("THREADS").ok().and_then(|s| s.parse().ok()).unwrap_or(4)
}

fn mode_c1(dir: &str, k: u32) {
    let f = Fq4::new(k);
    let path = format!("{dir}/yield_k{k}.jsonl");
    let txt = std::fs::read_to_string(&path).expect("yield rows (hydrate from the artifact store)");
    let rows: Vec<json::J> = txt.lines().filter(|l| !l.trim().is_empty()).map(json::parse).collect();
    for spec in curves_for_k(&f) {
        let my: Vec<&json::J> = rows.iter().filter(|r| r.get("curve").str() == spec.id).collect();
        if my.is_empty() {
            continue;
        }
        let cv = Curve { f: &f, a: spec.a, b: spec.b };
        let nt = my[0].get("targets").num().unwrap() as usize;
        let mut rng = Rng::new(SEED, &format!("targets-{k}-{}", spec.id));
        let targets = sample_targets(&cv, &mut rng, nt);
        let tr_a = spec.tr_a;
        let tr_x: Vec<u32> = targets.iter().map(|t| f.tr_abs(&t.x)).collect();
        let a_fq_tr = if in_fq(&f, &spec.a) { Some(tr_fq(&f, spec.a[0])) } else { None };
        let in_e2: Vec<bool> = targets.iter().map(|t| in_eq2(&f, t)).collect();
        let in_hv: Vec<Option<bool>> = targets
            .iter()
            .map(|t| a_fq_tr.map(|ta| in_h(&cv, t, ta)))
            .collect();
        for r in &my {
            let arm = r.get("arm").str().to_string();
            let uhex = r.get("u").str();
            let u = f.unpack(u64::from_str_radix(uhex.trim_start_matches("0x"), 16).unwrap());
            let (fb, lifting) = build_fb(&cv, &u);
            assert_eq!(fb.len() as f64, r.get("fb_points").num().unwrap(), "fb size");
            assert_eq!(lifting as f64, r.get("lifting").num().unwrap(), "lifting");
            let frozen: Vec<u64> = r.get("counts").nums().iter().map(|&c| c as u64).collect();
            assert_eq!(frozen.len(), nt);
            // identity check: recompute every count with the frozen instrument
            let recomputed = if fb.is_empty() {
                vec![0u64; nt]
            } else {
                let tab = PairTable::build(&cv, &fb);
                count_all(&cv, &tab, &targets, threads())
            };
            let id_mismatch = frozen.iter().zip(&recomputed).filter(|(a, b)| a != b).count();
            // field generated by the line, a, b
            let deg_k = lcm(lcm(lcm(k, deg_abs(&f, &u)), if is_zero(&spec.a) { 1 } else { deg_abs(&f, &spec.a) }), deg_abs(&f, &spec.b));
            let n = 4 * k;
            // P1: FB membership
            let fb_in_e2 = fb.iter().filter(|p| in_eq2(&f, p)).count();
            let fb_in_h = a_fq_tr.map(|ta| fb.iter().filter(|p| in_h(&cv, p, ta)).count());
            let fb_tr0 = fb.iter().filter(|p| f.tr_abs(&p.x) == 0).count();
            // P1: targets
            let mut nz_outside_h = 0usize;
            let mut nz_outside_e2 = 0usize;
            let mut targets_in_h = 0usize;
            let mut targets_in_e2 = 0usize;
            for i in 0..nt {
                if in_e2[i] {
                    targets_in_e2 += 1;
                } else if frozen[i] > 0 {
                    nz_outside_e2 += 1;
                }
                if let Some(h) = in_hv[i] {
                    if h {
                        targets_in_h += 1;
                    } else if frozen[i] > 0 {
                        nz_outside_h += 1;
                    }
                }
            }
            // P3: zeros off 2E
            let mut viol_2e = 0usize;
            let mut zeros_on = 0usize; // post hoc
            let (mut n_on, mut sum_on) = (0usize, 0u64);
            for i in 0..nt {
                if tr_x[i] != tr_a {
                    if frozen[i] != 0 {
                        viol_2e += 1;
                    }
                } else {
                    n_on += 1;
                    sum_on += frozen[i];
                    if frozen[i] == 0 {
                        zeros_on += 1;
                    }
                }
            }
            let pred = r.get("pred_excl").num();
            let mean_on = if n_on > 0 { sum_on as f64 / n_on as f64 } else { f64::NAN };
            let ratio = pred.map(|p| mean_on / p);
            // post hoc (not in PROTOCOL): borderline [K:F_2] = N/2 with K = F_{Q^2};
            // a 4-sum is generic only with exactly 2 points off E(K)
            let generic_share = if deg_k == 2 * k {
                let p = 1.0 - fb_in_e2 as f64 / fb.len() as f64;
                format!("{:.6}", 6.0 * p * p * (1.0 - p) * (1.0 - p))
            } else {
                "null".into()
            };
            println!(
                "{{\"k\":{k},\"curve\":\"{}\",\"arm\":\"{arm}\",\"u\":\"{uhex}\",\"deg_u\":{},\"deg_K\":{deg_k},\"N\":{n},\"trapped\":{},\"fb\":{},\"fb_in_EQ2\":{fb_in_e2},\"fb_in_H\":{},\"fb_tr0\":{fb_tr0},\"T\":{nt},\"identity_mismatch\":{id_mismatch},\"targets_in_EQ2\":{targets_in_e2},\"targets_in_H\":{},\"nonzero_outside_EQ2\":{nz_outside_e2},\"nonzero_outside_H\":{},\"tr_a\":{tr_a},\"frac_tr_eq_tr_a\":{:.6},\"viol_2E\":{viol_2e},\"mean_on_2E\":{:.6},\"pred_excl\":{},\"mean_on_2E_over_pred\":{},\"posthoc_zeros_on_2E\":{zeros_on},\"posthoc_n_on_2E\":{n_on},\"posthoc_generic_share_pred\":{generic_share}}}",
                spec.id,
                deg_abs(&f, &u),
                trapped(deg_k, n),
                fb.len(),
                fb_in_h.map(|x| x.to_string()).unwrap_or("null".into()),
                if a_fq_tr.is_some() { targets_in_h.to_string() } else { "null".into() },
                if a_fq_tr.is_some() { nz_outside_h.to_string() } else { "null".into() },
                n_on as f64 / nt as f64,
                mean_on,
                pred.map(|p| format!("{p:.6}")).unwrap_or("null".into()),
                ratio.map(|x| format!("{x:.6}")).unwrap_or("null".into()),
            );
        }
    }
}

/// P2: exact N(R) for A0 std over all R in E(F_{Q^2}).
fn mode_mass(k: u32) {
    let f = Fq4::new(k);
    let spec = curves_for_k(&f).into_iter().find(|s| s.id == "A0").unwrap();
    let cv = Curve { f: &f, a: spec.a, b: spec.b };
    let ta = tr_fq(&f, spec.a[0]);
    // theta in F_{Q^2} \ F_Q: w + w^{Q^2}
    let mut rng = Rng::new(SEED, &format!("subgroup-trap-theta-{k}"));
    let theta = loop {
        let w = rng.e4(&f);
        let t = add(&w, &f.frob(&f.frob(&w)));
        if !in_fq(&f, &t) {
            break t;
        }
    };
    assert_eq!(f.frob(&f.frob(&theta)), theta);
    let q = f.f.q as u32;
    let mut pts = vec![O];
    for s in 0..q {
        for t in 0..q {
            let x = add(&fq_el(s as u16), &f.scale(t as u16, &theta));
            if is_zero(&x) {
                let tp = Pt { x: ZERO, y: f.sqrt(&spec.b), inf: false };
                if in_eq2(&f, &tp) {
                    pts.push(tp);
                }
                continue;
            }
            if let Some(p) = cv.lift(&x) {
                if in_eq2(&f, &p) {
                    pts.push(p);
                    pts.push(cv.neg(&p));
                }
            }
        }
    }
    let e2 = pts.len() as u64;
    let (fb, _) = build_fb(&cv, &ONE);
    let m = fb.len() as u64;
    let tab = PairTable::build(&cv, &fb);
    let counts = count_all(&cv, &tab, &pts, threads());
    let total: u128 = counts.iter().map(|&c| c as u128).sum();
    let h: Vec<bool> = pts.iter().map(|p| in_h(&cv, p, ta)).collect();
    let h_size = h.iter().filter(|&&b| b).count();
    let support = counts.iter().filter(|&&c| c > 0).count();
    let support_outside_h = counts.iter().zip(&h).filter(|(&c, &b)| c > 0 && !b).count();
    let sum_on_h: u128 = counts.iter().zip(&h).filter(|(_, &b)| b).map(|(&c, _)| c as u128).sum();
    let min_on_h = counts.iter().zip(&h).filter(|(_, &b)| b).map(|(&c, _)| c).min().unwrap();
    let max_on_h = counts.iter().zip(&h).filter(|(_, &b)| b).map(|(&c, _)| c).max().unwrap();
    println!(
        "{{\"k\":{k},\"curve\":\"A0\",\"arm\":\"std\",\"E_FQ2\":{e2},\"H\":{h_size},\"m\":{m},\"m4\":{},\"sum_N_over_EQ2\":{total},\"mass_equal\":{},\"support\":{support},\"support_outside_H\":{support_outside_h},\"mean_on_H\":{:.6},\"m4_over_H\":{:.6},\"min_on_H\":{min_on_h},\"max_on_H\":{max_on_h},\"sum_on_H\":{sum_on_h}}}",
        (m as u128).pow(4),
        total == (m as u128).pow(4),
        sum_on_h as f64 / h_size as f64,
        (m as f64).powi(4) / h_size as f64,
    );
}

fn read_jsonl(path: &str) -> Vec<json::J> {
    std::fs::read_to_string(path)
        .map(|t| t.lines().filter(|l| !l.trim().is_empty()).map(json::parse).collect())
        .unwrap_or_default()
}

/// Verdicts P1-P4 exactly as frozen in PROTOCOL.md (with Amendment 1).
fn mode_score(dir: &str) {
    let mut c1 = vec![];
    for k in [5, 7, 9, 11] {
        c1.extend(read_jsonl(&format!("{dir}/c1_k{k}.jsonl")));
    }
    let mass = read_jsonl(&format!("{dir}/mass.jsonl"));
    let audit = read_jsonl(&format!("{dir}/audit.jsonl"));
    let n = |r: &json::J, k: &str| r.get(k).num().unwrap_or(f64::NAN);
    let mut out = String::new();
    use std::fmt::Write as _;
    // identity
    let id_bad: Vec<String> = c1
        .iter()
        .filter(|r| n(r, "identity_mismatch") != 0.0)
        .map(|r| format!("k={} {} {}", n(r, "k"), r.get("curve").str(), r.get("arm").str()))
        .collect();
    writeln!(out, "identity (regenerated targets reproduce every frozen count): rows {}, mismatching rows {}", c1.len(), id_bad.len()).unwrap();
    // P1
    let mut p1 = true;
    writeln!(out, "\nP1 (A0 subfield trap):").unwrap();
    for r in c1.iter().filter(|r| r.get("curve").str() == "A0") {
        let arm = r.get("arm").str();
        let trapped_arm = arm == "std" || arm.starts_with("tw_a");
        let ok = if trapped_arm {
            n(r, "fb_in_H") == n(r, "fb") && n(r, "nonzero_outside_H") == 0.0 && n(r, "nonzero_outside_EQ2") == 0.0 && r.get("trapped").bool()
        } else {
            n(r, "fb_in_EQ2") < n(r, "fb")
        };
        p1 &= ok;
        writeln!(
            out,
            "  k={} {arm:<10} deg_K={} trapped={} fb={} in_E(F_Q2)={} in_H={} | targets T={} in_E(F_Q2)={} in_H={} nonzero_outside_H={} nonzero_outside_E(F_Q2)={} => {}",
            n(r, "k"), n(r, "deg_K"), r.get("trapped").bool(), n(r, "fb"), n(r, "fb_in_EQ2"), n(r, "fb_in_H"), n(r, "T"), n(r, "targets_in_EQ2"), n(r, "targets_in_H"), n(r, "nonzero_outside_H"), n(r, "nonzero_outside_EQ2"),
            if ok { "ok" } else { "VIOLATION" }
        )
        .unwrap();
    }
    let ks: Vec<f64> = c1.iter().filter(|r| r.get("curve").str() == "A0").map(|r| n(r, "k")).collect();
    let p1_complete = [5.0, 7.0, 9.0].iter().all(|k| ks.contains(k));
    let p1v = p1 && p1_complete && id_bad.is_empty();
    writeln!(out, "P1 => {}", if p1v { "PASS" } else { "FAIL" }).unwrap();
    // P2
    writeln!(out, "\nP2 (A0 std, exact mass over E(F_Q2)):").unwrap();
    let mut p2 = mass.len() == 2;
    for r in &mass {
        let ok = r.get("mass_equal").bool() && n(r, "support_outside_H") == 0.0 && 2.0 * n(r, "H") == n(r, "E_FQ2");
        p2 &= ok;
        writeln!(
            out,
            "  k={} #E(F_Q2)={} |H|={} m={} sum N = {} (m^4 = {}) support={} outside H={} mean on H={:.3} = m^4/|H|={:.3} min/max on H {}/{} => {}",
            n(r, "k"), n(r, "E_FQ2"), n(r, "H"), n(r, "m"), n(r, "sum_N_over_EQ2"), n(r, "m4"), n(r, "support"), n(r, "support_outside_H"), n(r, "mean_on_H"), n(r, "m4_over_H"), n(r, "min_on_H"), n(r, "max_on_H"),
            if ok { "ok" } else { "VIOLATION" }
        )
        .unwrap();
    }
    writeln!(out, "P2 => {}", if p2 { "PASS" } else { "FAIL" }).unwrap();
    // P3
    writeln!(out, "\nP3 (zero count whenever Tr(x_R) != Tr(a)):").unwrap();
    let viol: f64 = c1.iter().map(|r| n(r, "viol_2E")).sum();
    let targets: f64 = c1.iter().map(|r| n(r, "T") * (1.0 - n(r, "frac_tr_eq_tr_a"))).sum();
    let mut ratios: Vec<f64> = c1.iter().filter(|r| r.get("curve").str() != "A0").map(|r| n(r, "mean_on_2E_over_pred")).filter(|x| x.is_finite()).collect();
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let fr: Vec<f64> = c1.iter().map(|r| n(r, "frac_tr_eq_tr_a")).collect();
    let (fmin, fmax) = fr.iter().fold((1.0f64, 0.0f64), |(a, b), &x| (a.min(x), b.max(x)));
    let med = if ratios.is_empty() { f64::NAN } else { ratios[ratios.len() / 2] };
    writeln!(
        out,
        "  rows {}, (row, target) pairs with Tr(x_R) != Tr(a): {:.0}, nonzero among them: {viol}\n  reported: fraction with Tr(x_R) = Tr(a) in [{fmin:.3}, {fmax:.3}]; mean on that half / pred_excl (non-A0 rows) median {med:.3}, range [{:.3}, {:.3}]",
        c1.len(), targets, ratios.first().copied().unwrap_or(f64::NAN), ratios.last().copied().unwrap_or(f64::NAN)
    )
    .unwrap();
    let p3 = viol == 0.0 && !c1.is_empty();
    writeln!(out, "P3 => {}", if p3 { "PASS" } else { "FAIL" }).unwrap();
    // P4
    writeln!(out, "\nP4 (no applicable line trapped):").unwrap();
    let mut p4 = !audit.is_empty();
    for r in &audit {
        let lines = r.get("lines").arr();
        let trapped = lines.iter().filter(|l| l.get("trapped").bool()).count();
        let pi16 = r.get("c2pnb_rs_pi16_lines");
        let pi16_trap = pi16.get("any_u_can_trap").bool();
        p4 &= trapped == 0 && !pi16_trap;
        writeln!(
            out,
            "  {} N={} d={} Tr(a)={} deg a={} deg b={} lines {} trapped {}; F_2^16 lines can trap: {}",
            r.get("curve").str(), n(r, "N"), n(r, "d"), n(r, "Tr_a"), n(r, "deg_a"), n(r, "deg_b"), lines.len(), trapped,
            if matches!(pi16, json::J::Null) { "n/a".to_string() } else { pi16_trap.to_string() }
        )
        .unwrap();
    }
    let c1_lines: Vec<&json::J> = c1.iter().filter(|r| r.get("curve").str() != "A0" && r.get("curve").str() != "NEG").collect();
    let c1_trapped = c1_lines.iter().filter(|r| r.get("trapped").bool()).count();
    p4 &= c1_trapped == 0;
    writeln!(out, "  C1 A1 A2 B1-B3 (all k, all arms): {} rows, trapped {c1_trapped}", c1_lines.len()).unwrap();
    writeln!(out, "P4 => {}", if p4 { "PASS" } else { "FAIL" }).unwrap();
    print!("{out}");
    let js = format!(
        "{{\"identity_bad_rows\":{},\"P1\":\"{}\",\"P2\":\"{}\",\"P3\":\"{}\",\"P3_violations\":{viol},\"P3_pairs_tested\":{targets:.0},\"P3_ratio_median\":{med:.4},\"P4\":\"{}\"}}\n",
        id_bad.len(),
        if p1v { "PASS" } else { "FAIL" },
        if p2 { "PASS" } else { "FAIL" },
        if p3 { "PASS" } else { "FAIL" },
        if p4 { "PASS" } else { "FAIL" }
    );
    std::fs::write(format!("{dir}/summary.json"), js).unwrap();
    std::fs::write(format!("{dir}/summary.txt"), out).unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("c1") => mode_c1(&a[2], a[3].parse().unwrap()),
        Some("mass") => mode_mass(a[2].parse().unwrap()),
        Some("audit") => gf2n::audit(&a[2], &a[3]),
        Some("score") => mode_score(&a[2]),
        _ => eprintln!("usage: subgroup_trap c1 <run_dir> <k> | mass <k> | audit <c2pnb_params> <oakley_rows.json> | score <run_dir>"),
    }
}
