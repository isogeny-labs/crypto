#[allow(dead_code, clippy::all)]
#[path = "../../../../../src/cryptanalysis/plane_quartic_pic.rs"]
mod plane_quartic_pic;

mod curves;
mod enumerate;
mod fp;
mod jac;
mod la;
mod lines;
mod measure;
mod picrho;
mod rhocost;
mod tiny;

use plane_quartic_pic::{PicClass, PlaneQuartic, Pt};
use std::collections::HashMap;
use std::io::Write;

pub struct Ctx {
    pub curve: curves::Curve,
    pub pts: Vec<Pt>,
    pub idx: HashMap<Pt, usize>,
    pub p0idx: usize,
}

pub fn build_ctx(curve: curves::Curve) -> Ctx {
    let pts = enumerate::points_bruteforce(&curve.f, curve.p);
    let idx: HashMap<Pt, usize> = pts.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let p0idx = *idx.get(&curve.p0).expect("p0 not in points");
    Ctx { curve, pts, idx, p0idx }
}

/// Verify (up to `limit`) accepted relations in Pic. For Q the reference is the first row's class,
/// otherwise zero. Returns (rows verified, all ok).
pub fn verify_rows(
    pq: &PlaneQuartic,
    pts: &[Pt],
    rels: &[[usize; 4]],
    limit: usize,
    ref_first: bool,
) -> (usize, bool) {
    let mut cache: HashMap<usize, PicClass> = HashMap::new();
    let mut reference: Option<PicClass> = None;
    let mut ok = true;
    let mut n = 0;
    for rel in rels.iter().take(limit) {
        let mut acc = pq.zero();
        for &i in rel.iter() {
            let c = cache.entry(i).or_insert_with(|| pq.point_class(pts[i])).clone();
            acc = pq.add(&acc, &c);
        }
        if ref_first {
            match &reference {
                None => reference = Some(acc),
                Some(r) => ok &= *r == acc,
            }
        } else {
            ok &= pq.is_zero(&acc);
        }
        n += 1;
    }
    (n, ok)
}

pub fn append_line(path: &str, line: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("open out");
    writeln!(f, "{}", line).unwrap();
}

pub fn jb(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
pub fn jopt_b(b: Option<bool>) -> String {
    match b {
        Some(x) => jb(x).to_string(),
        None => "null".to_string(),
    }
}
pub fn jopt<T: std::fmt::Display>(b: Option<T>) -> String {
    match b {
        Some(x) => format!("{}", x),
        None => "null".to_string(),
    }
}

pub fn arg(args: &[String], name: &str) -> Option<String> {
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == name {
            return Some(args[i + 1].clone());
        }
        i += 1;
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: l10 tiny|rhocost|measure ...");
        std::process::exit(2);
    }
    let out = arg(&args, "--out").unwrap_or_else(|| "/dev/stdout".to_string());
    match args[1].as_str() {
        "tiny" => {
            let p: u32 = arg(&args, "--p").unwrap().parse().unwrap();
            tiny::run(p, &out);
        }
        "rhocost" => {
            let p: u32 = arg(&args, "--p").unwrap().parse().unwrap();
            let s: u64 = arg(&args, "--curve-seed").unwrap().parse().unwrap();
            let fam = curves::Family::parse(&arg(&args, "--family").unwrap());
            rhocost::run(fam, p, s, &out);
        }
        "selftest" => selftest::run(&arg(&args, "--name").unwrap()),
        "measure" => {
            let p: u32 = arg(&args, "--p").unwrap().parse().unwrap();
            let fams: Vec<curves::Family> = arg(&args, "--families")
                .unwrap()
                .split(',')
                .map(curves::Family::parse)
                .collect();
            measure::run(p, &fams, &out);
        }
        _ => {
            eprintln!("unknown subcommand");
            std::process::exit(2);
        }
    }
}

mod selftest {
    use crate::curves::*;
    use crate::enumerate::*;
    use crate::fp::*;
    use crate::jac::*;
    use crate::la::*;
    use crate::lines::*;
    use crate::picrho::*;
    use crate::plane_quartic_pic::*;
    use crate::*;

    pub fn run(name: &str) {
        match name {
            "u1_eval_counts" => u1_eval_counts(),
            "u2_zeta_and_lambda" => u2_zeta_and_lambda(),
            "u3_jac" => u3_jac(),
            "u4_line_trials" => u4_line_trials(),
            "u5_krylov" => u5_krylov(),
            "u6_bm_fib" => u6_bm_fib(),
            "u7_identity_quotient" => u7_identity_quotient(),
            "u8_keys" => u8_keys(),
            "u9_enum_p13" => u9_enum_p13(),
            "u10_t5_p31" => u10_t5_p31(),
            "u11_family_q" => u11_family_q(),
            _ => panic!("unknown selftest {}", name),
        }
    }

    pub fn u1_eval_counts() {
        let p = 2053u32;
        let fp = Fp::new(p as u64);
        for (a, want) in [((5, 7, 11), 15u64), ((0, 7, 11), 13), ((5, 1, 11), 14)] {
            let f = picard_from(p, a.0, a.1, a.2);
            let mut c = Cnt::default();
            eval_f(&f, [1, 3, 5], &fp, &mut c);
            assert_eq!(c.mul, 9 + want - 9 + 0 * want, "counts {:?}", a);
            assert_eq!(c.inv, 0);
        }
    }

    pub fn u2_zeta_and_lambda() {
        let t = [(7, 4), (13, 3), (19, 7), (31, 25), (37, 26), (43, 36), (271, 242), (523, 462),
            (1033, 837), (2053, 1855), (4099, 2017), (8209, 3265), (16411, 13654)];
        for (p, z) in t {
            assert_eq!(zeta_of(p), z, "zeta {}", p);
        }
        let md = Modulus::new(ELL);
        let l = LAMBDA_PROXY;
        assert_eq!(md.add(md.add(md.mul(l, l), l), 1) % ELL, 0);
    }

    pub fn u3_jac() {
        let cases = [(7u32, (0u32, 0u32, 1u32), (4u64, 20u64, 364u64), 144i128),
            (13, (0, 0, 1), (4, 212, 2452), 1200),
            (13, (1, 2, 3), (11, 149, 2408), 1701)];
        for (p, a, n, j) in cases {
            let (n1, n2, n3, jac) = jac_order(p, a.0, a.1, a.2);
            assert_eq!((n1, n2, n3), n, "N counts p={} {:?}", p, a);
            assert_eq!(jac, j);
            // U3b
            let f = picard_from(p, a.0, a.1, a.2);
            let pq = PlaneQuartic::new(p, f, [0, 0, 1]).unwrap();
            let pts = pq.rational_points_bruteforce();
            let mut rng = 99u64;
            for _ in 0..8 {
                let x = random_class(&pq, &pts, &mut rng);
                assert!(pq.is_zero(&pq.mul_scalar(&x, j as u128)));
            }
        }
    }

    pub fn u4_line_trials() {
        let p = 2053u32;
        let f = picard_from(p, 5, 7, 11);
        let pts = points_bruteforce(&f, p);
        let idx: std::collections::HashMap<Pt, usize> = pts.iter().enumerate().map(|(i, q)| (*q, i)).collect();
        let inp = SearchInput { arm: Arm::PFull, p, f: &f, pts: &pts, idx: &idx, p0: [0, 0, 1], orbits: None,
            curve_seed: 1, solver_seed: 1, rows_target: 100000, cap: 199 };
        let r = run_search(&inp);
        let c = &r.counters;
        assert_eq!(c.trials, 200);
        assert!(c.nonsquare > 0);
        assert_eq!(c.line_mul - c.line_setup_mul, 51 * c.trials + 8 * c.line_inv);
        for m in r.rel_pts.iter() {
            let pp = pts[m[0]];
            let qq = pts[m[1]];
            for k in 2..4 {
                let rr = pts[m[k]];
                assert_eq!(eval_plain(&f, [rr[0] as u64, rr[1] as u64, rr[2] as u64], p as u64), 0);
                let a = [pp, qq, rr];
                let pu = p as i128;
                let g = |i: usize, j: usize| a[i][j] as i128;
                let det = g(0, 0) * (g(1, 1) * g(2, 2) - g(1, 2) * g(2, 1))
                    - g(0, 1) * (g(1, 0) * g(2, 2) - g(1, 2) * g(2, 0))
                    + g(0, 2) * (g(1, 0) * g(2, 1) - g(1, 1) * g(2, 0));
                assert_eq!(det.rem_euclid(pu), 0);
            }
        }
    }

    fn mat3() -> Vec<Row> {
        vec![Row::Full(vec![(0, 1), (1, 2)]), Row::Full(vec![(1, 1), (2, 3)]), Row::Full(vec![(0, 4), (2, 1)])]
    }

    pub fn u5_krylov() {
        let md = Modulus::new(ELL);
        let a = build_matrix(&mat3(), 3, &md, 1);
        assert_eq!(a.w(), 6);
        let la = wiedemann_count(&a, &md, 1, 7);
        assert_eq!(la.krylov_mul, 54);
    }

    pub fn u6_bm_fib() {
        let md = Modulus::new(ELL);
        let mut s = vec![0u64, 1];
        for i in 2..40 {
            let v = md.add(s[i - 1], s[i - 2]);
            s.push(v);
        }
        let (mut c, l, _, _) = bm(&s, &md);
        while c.last() == Some(&0) {
            c.pop();
        }
        assert_eq!(l, 2);
        assert_eq!(c, vec![1, ELL - 1, ELL - 1]);
    }

    pub fn u7_identity_quotient() {
        let p = 271u32;
        let ctx = build_ctx(gen_curve(Family::P, p, 1));
        let nf = ctx.pts.len() - 1;
        let run = |arm: Arm, orb: Option<&crate::enumerate::Orbits>| {
            let inp = SearchInput { arm, p, f: &ctx.curve.f, pts: &ctx.pts, idx: &ctx.idx, p0: ctx.curve.p0,
                orbits: orb, curve_seed: 1, solver_seed: 1, rows_target: nf + 10, cap: 60 * (nf as u64 + 10) };
            run_search(&inp)
        };
        let a = run(Arm::PFull, None);
        let id = orbits_identity(&ctx.pts, ctx.curve.p0);
        let b = run(Arm::POrbit, Some(&id));
        let (x, y) = (&a.counters, &b.counters);
        assert_eq!((x.trials, x.accepted, x.dup, x.nonsquare, x.tangent, x.line_mul, x.line_inv),
            (y.trials, y.accepted, y.dup, y.nonsquare, y.tangent, y.line_mul, y.line_inv));
    }

    pub fn u8_keys() {
        let r1: Vec<(u32, [u8; 3])> = vec![(5, [1, 1, 1])];
        let red: Vec<(u32, [u8; 3])> = r1.into_iter().filter_map(|(o, t)| {
            let mn = *t.iter().min().unwrap();
            let t2 = [t[0] - mn, t[1] - mn, t[2] - mn];
            if t2 != [0, 0, 0] { Some((o, t2)) } else { None }
        }).collect();
        assert!(red.is_empty());
        let a = vec![(1u32, [1u8, 0, 0]), (2, [0, 1, 0])];
        let b = vec![(1u32, [0u8, 1, 0]), (2, [0, 0, 1])];
        assert_eq!(orbit_key(&a), orbit_key(&b));
        assert_ne!(orbit_key(&[(1, [1, 0, 0])]), orbit_key(&[(1, [1, 1, 0])]));
    }

    pub fn u9_enum_p13() {
        let mut c = Cnt::default();
        let pts = points_picard_counted(0, 0, 1, 13, &mut c);
        assert_eq!(pts.len(), 4);
        assert_eq!(c.mul, 2 * 12 + 3 * 13);
        assert_eq!(c.inv, 0);
    }

    pub fn u10_t5_p31() {
        let p = 31u32;
        let curve = gen_curve(Family::P, p, 1);
        let pq = PlaneQuartic::new(p, curve.f, curve.p0).unwrap();
        let pts = pq.rational_points_bruteforce();
        let rho = Rho::new(p, zeta_of(p));
        let mut rng = 5u64;
        assert!(check_t5(&pq, &rho, &pts, &mut rng, 20, 20));
    }

    pub fn u11_family_q() {
        let c = gen_curve(Family::Q, 2053, 1);
        let mut s = 1001u64;
        let (f, p0) = next_smooth_quartic_with_point(2053, &mut s);
        assert_eq!(c.f, f);
        let pq = PlaneQuartic::new(2053, f, p0).unwrap();
        assert_eq!(c.p0, pq.normalize_point(p0));
    }
}
