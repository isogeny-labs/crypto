//! Smoke run of the published-IC reference arm (card XP-20261024-published-ic-reference-arm):
//! Koblitz E(F_{2^n}), m = 3, base F_u = {P : u(P) in V}, u = 1/(x+1), V a random non-Frobenius-invariant
//! subspace containing 1; the lab's u-frame 2-torsion invariants only (per-point w_i, no reduction to
//! elementary symmetric functions, so NOT the FGHR S_m-symmetrised system); solved by external msolve.
//! Per target: write the system (+ field equations) as msolve input over GF(2), run
//! `msolve -g 2`, read the verdict (GB = [1] means no decomposition), and on a non-unit ideal
//! extract roots by evaluation (IMPROVISED: 2^n_vars sweep), lift, and verify the sum in the group.
//! CPU is the child process's rusage (utime+stime). No wall clock. No GAE pricing hook attached.
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_groebner::FieldStructure;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{points_with_x, KoblitzCurve};
use crypto_lib::cryptanalysis::koblitz_symmetrised::{
    build_symmetrised_factor_base_from_basis, build_symmetrised_system, enumerate_symmetrised,
    lift_symmetrised_root, random_subspace_containing_one,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::Write;
use std::process::Command;
use std::time::Duration;

fn children_cpu() -> Duration {
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, &mut ru) };
    let tv = |t: libc::timeval| Duration::new(t.tv_sec as u64, (t.tv_usec as u32) * 1000);
    tv(ru.ru_utime) + tv(ru.ru_stime)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(13);
    let targets: usize = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(4);
    let seed: u64 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(20261024);
    let m = 3usize;
    let msolve = std::env::var("MSOLVE").unwrap_or_else(|_| "/opt/homebrew/bin/msolve".into());
    let ca: u8 = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);
    let kc = KoblitzCurve::new(ca, n).expect("koblitz curve for this (a, n)");
    let ell = ((n + 1) as usize).div_ceil(m);
    let mut rng = StdRng::seed_from_u64(seed);
    let basis = random_subspace_containing_one(&kc, ell, &mut rng);
    let fb = build_symmetrised_factor_base_from_basis(&kc, basis).expect("base");
    let st = FieldStructure::new(kc.n, &kc.curve.irreducible);
    let mut pts: Vec<BinaryPoint> = Vec::new();
    for x in 0..(1u64 << n) {
        let el = crypto_lib::binary_ecc::F2mElement::from_biguint(&num_bigint::BigUint::from(x), n);
        pts.extend(points_with_x(&kc.curve, &el));
    }
    println!("{{\"n\":{n},\"m\":{m},\"ell\":{ell},\"base_points\":{},\"seed\":{seed}}}", fb.points.len());
    let dir = std::env::temp_dir().join(format!("ref_arm_smoke_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut done = 0;
    while done < targets {
        // Odd-numbered targets are planted positive controls: a sum of three base points.
        let t = if done % 2 == 1 {
            let mut acc = BinaryPoint::Infinity;
            for _ in 0..3 {
                acc = kc.add(&acc, &fb.points[rng.gen_range(0..fb.points.len())]);
            }
            acc
        } else {
            pts[rng.gen_range(0..pts.len())].clone()
        };
        if matches!(t, BinaryPoint::Infinity) { continue; }
        let sys = match build_symmetrised_system(&kc, &fb, &t, m, &st) {
            Some(s) => s,
            None => continue,
        };
        done += 1;
        let truth = enumerate_symmetrised(&kc, &fb, &t, m).is_some();
        let nv = sys.n_vars;
        let mut polys: Vec<String> = Vec::new();
        for eq in &sys.equations {
            if eq.terms.is_empty() {
                continue;
            }
            let ts: Vec<String> = eq
                .terms
                .iter()
                .map(|t| {
                    if t.mask == 0 {
                        "1".into()
                    } else {
                        (0..nv).filter(|i| (t.mask >> i) & 1 == 1).map(|i| format!("x{i}")).collect::<Vec<_>>().join("*")
                    }
                })
                .collect();
            polys.push(ts.join("+"));
        }
        let n_eqs = polys.len();
        for i in 0..nv {
            polys.push(format!("x{i}^2+x{i}"));
        }
        let inp = dir.join("in.ms");
        let out = dir.join("out.ms");
        let mut f = std::fs::File::create(&inp).unwrap();
        let vars: Vec<String> = (0..nv).map(|i| format!("x{i}")).collect();
        writeln!(f, "{}\n2\n{}", vars.join(","), polys.join(",\n")).unwrap();
        drop(f);
        let c0 = children_cpu();
        let st_ = Command::new(&msolve).args(["-g", "2", "-t", "1", "-f"]).arg(&inp).arg("-o").arg(&out).output().expect("msolve");
        let cpu = children_cpu().saturating_sub(c0);
        let body = std::fs::read_to_string(&out).unwrap_or_default();
        // Exact match: the reduced basis of the unit ideal is the single line `[1]:`.
        let unit: Option<bool> = body.lines().map(|l| l.trim()).find(|l| l.starts_with('[')).map(|l| l == "[1]:");
        // IMPROVISED root extraction by evaluation over all 2^nv assignments.
        let mut verified = None;
        let mut roots = 0u64;
        if unit == Some(false) && nv <= 24 {
            for r in 0..(1u64 << nv) {
                let ok = sys.equations.iter().all(|e| e.terms.iter().fold(false, |acc, t| acc ^ ((r & t.mask) == t.mask)) == false);
                if ok {
                    roots += 1;
                    if verified != Some(true) {
                        if let Some((idx, used_t)) = lift_symmetrised_root(&kc, &fb, &sys, r, &t) {
                            let s = fb.sum(&kc, &idx);
                            let tp = kc.add(&t, &fb.two_torsion);
                            verified = Some(s == t || (used_t && s == tp));
                        } else if verified.is_none() {
                            verified = Some(false);
                        }
                    }
                }
            }
        }
        let js = |o: Option<bool>| match o {
            Some(b) => b.to_string(),
            None => "null".to_string(),
        };
        println!(
            "{{\"target\":{done},\"planted\":{},\"n_vars\":{nv},\"n_eqs\":{n_eqs},\"msolve_ok\":{},\"gb_is_one\":{},\"enum_truth_decomposable\":{truth},\"roots\":{roots},\"relation_verified\":{},\"msolve_cpu_s\":{:.4}}}",
            done % 2 == 0,
            st_.status.success(),
            js(unit),
            js(verified),
            cpu.as_secs_f64()
        );
    }
}
