use crate::arms::*;
use crate::curve::*;
use crate::exact::*;
use crate::field::Fp;
use crate::graph::Graph;
use crate::poly::*;

const A31: Coef = [[3, 10, 17, 24], [12, 20, 28, 5], [21, 30, 8, 17], [30, 9, 19, 29]];

fn fixture() -> (Fp, Curve) {
    let fp = Fp::new(31);
    let c = Curve::from_coeffs(&fp, A31, 0).expect("fixture accepted");
    (fp, c)
}

#[test]
fn t1_points() {
    let (_fp, c) = fixture();
    assert_eq!(c.n1(), 36);
    let exp: Vec<(u64, u64)> = vec![
        (0, 6), (1, 9), (1, 10), (1, 16), (2, 18), (3, 19), (4, 17), (7, 18), (8, 26), (9, 4), (9, 13), (9, 15),
        (10, 4), (10, 13), (10, 15), (12, 26), (13, 17), (15, 17), (16, 4), (16, 13), (16, 15), (17, 1),
        (18, 25), (19, 8), (19, 12), (19, 29), (20, 19), (22, 0), (23, 19), (24, 30), (26, 3), (26, 20),
        (26, 23), (28, 18), (29, 26), (30, 11),
    ];
    assert_eq!(c.pts, exp);
}

#[test]
fn t2_pencil_census() {
    let (fp, c) = fixture();
    let (mut sx, mut sy) = (0, 0);
    for t in 0..31 {
        if split_test(&fp, &fibre_cubic(&fp, &c.a, 1, t)) { sx += 1; }
        if split_test(&fp, &fibre_cubic(&fp, &c.a, 2, t)) { sy += 1; }
    }
    assert_eq!((sx, sy), (6, 7));
    let (mut px, mut py) = (0, 0);
    for t in 0..31 {
        px += roots_all(&fp, &fibre_cubic(&fp, &c.a, 1, t)).len();
        py += roots_all(&fp, &fibre_cubic(&fp, &c.a, 2, t)).len();
    }
    assert_eq!((px, py), (36, 36));
    let cx = fibre_cubic(&fp, &c.a, 1, 9);
    assert_eq!(cx, vec![26, 28, 30, 1]);
    assert_eq!(roots_split(&fp, &cx), vec![4, 13, 15]);
    let cy = fibre_cubic(&fp, &c.a, 2, 4);
    let xs = roots_split(&fp, &cy);
    let pts: Vec<(u64, u64)> = xs.iter().map(|&x| (x, 4)).collect();
    assert_eq!(pts, vec![(9, 4), (10, 4), (16, 4)]);
}

#[test]
fn t3_line_trials() {
    let (fp, c) = fixture();
    let id = |x: u64, y: u64| *c.map.get(&(x, y)).unwrap();
    let o = id(0, 6);
    let p1 = id(1, 9);
    let r = line_trial(&fp, &c, o, p1, id(1, 10));
    assert_eq!(r.b, [25, 6, 1, 30]);
    assert_eq!(r.outcome, Outcome::Degenerate);
    let r = line_trial(&fp, &c, o, p1, id(2, 18));
    assert_eq!(r.b, [16, 0, 18, 25]);
    assert_eq!(r.g, vec![0, 18, 0, 24, 29, 23, 30]);
    assert_eq!(r.r, vec![22, 2, 11, 1]);
    assert_eq!(r.outcome, Outcome::NoSplit);
    let r = line_trial(&fp, &c, o, p1, id(3, 19));
    assert_eq!(r.b, [6, 8, 30, 27]);
    assert_eq!(r.g, vec![0, 20, 2, 18, 28, 24, 1]);
    assert_eq!(r.r, vec![17, 13, 28, 1]);
    assert_eq!(r.outcome, Outcome::Ok);
    let mut res: Vec<(u64, u64)> = r.pts[3..].iter().map(|&i| c.pts[i as usize]).collect();
    res.sort();
    assert_eq!(res, vec![(9, 13), (26, 23), (30, 11)]);
    let r = line_trial(&fp, &c, o, p1, id(4, 17));
    assert_eq!(r.b, [25, 19, 1, 1]);
    assert_eq!(r.g, vec![0, 22, 3, 22, 9, 30, 7]);
    assert_eq!(r.r, vec![3, 26, 27, 1]);
    assert_eq!(r.outcome, Outcome::NoSplit);
}

#[test]
fn t4_polys() {
    let fp = Fp::new(7);
    let f = vec![fp.neg(1) , 0, 0, 0]; // dummy to avoid unused
    let _ = f;
    // (x-1)(x-2)(x-3) = x^3 - 6x^2 + 11x - 6 = x^3 + x^2 + 4x + 1 mod 7
    let f1 = vec![1, 4, 1, 1];
    assert!(split_test(&fp, &f1));
    assert_eq!(roots_split(&fp, &f1), vec![1, 2, 3]);
    assert!(!split_test(&fp, &vec![5, 0, 0, 1])); // x^3 - 2
    let f3 = vec![0, 6, 0, 1]; // x^3 - x
    assert!(split_test(&fp, &f3));
    assert_eq!(roots_split(&fp, &f3), vec![0, 1, 6]);
    // (x-1)^2 (x-2) = (x^2 - 2x + 1)(x - 2) = x^3 - 4x^2 + 5x - 2
    let f4 = vec![5, 5, 3, 1];
    assert!(!split_test(&fp, &f4));
    let before = fp.snap();
    let r = pmul(&fp, &vec![1, 2], &vec![3, 4]);
    let after = fp.snap();
    assert_eq!(after.0 - before.0, 4);
    assert_eq!(r, vec![3, 3, 1]);
}

#[test]
fn t5_counters() {
    let fp = Fp::new(31);
    let a = fp.snap();
    fp.mul(3, 4);
    assert_eq!(fp.snap(), (a.0 + 1, a.1));
    fp.inv(3);
    assert_eq!(fp.snap(), (a.0 + 1, a.1 + 1));
    let (_f, c) = fixture();
    let id = |x: u64, y: u64| *c.map.get(&(x, y)).unwrap();
    let f1 = Fp::new(31);
    line_trial(&f1, &c, id(0, 6), id(1, 9), id(3, 19));
    let s1 = f1.snap();
    line_trial(&f1, &c, id(0, 6), id(1, 9), id(3, 19));
    let s2 = f1.snap();
    assert_eq!((s2.0 - s1.0, s2.1 - s1.1), s1);
}

fn graph_y(edges: &[(usize, usize)], nv: usize) -> (i64, i64) {
    let mut g = Graph::new();
    for _ in 0..nv { g.new_vertex(); }
    for &(a, b) in edges { g.add_edge(a, b); }
    (g.y(), g.rank_lp_uf())
}

fn exact_rank_lp(edges: &[(usize, usize)], nv: usize) -> usize {
    let mut e = Echelon::new(nv);
    for &(a, b) in edges {
        let mut r = vec![0u64; nv];
        if a > 0 { r[a - 1] = 1; }
        if b > 0 { r[b - 1] = 1; }
        e.insert(r);
    }
    e.rank
}

#[test]
fn t6_t7_graph_and_exact() {
    let cases: Vec<(Vec<(usize, usize)>, usize, i64)> = vec![
        (vec![(0, 1), (1, 2)], 2, 0),
        (vec![(1, 2), (2, 3), (3, 1)], 3, 0),
        (vec![(1, 2), (2, 3), (3, 4), (4, 1)], 4, 1),
        (vec![(1, 2), (1, 2)], 2, 1),
        (vec![(0, 1), (0, 1)], 1, 1),
        (vec![(1, 2), (3, 4)], 4, 0),
    ];
    for (e, nv, y) in &cases {
        let (yy, r) = graph_y(e, *nv);
        assert_eq!(yy, *y, "{:?}", e);
        assert_eq!(r as usize, exact_rank_lp(e, *nv), "rank {:?}", e);
    }
    assert_eq!(graph_y(&[(1, 2), (3, 4)], 4).1, 2);
    // exact over 3 columns
    let mut e = Echelon::new(3);
    let m1 = ELL - 1;
    e.insert(vec![1, m1, 0]);
    e.insert(vec![0, 1, m1]);
    e.insert(vec![1, 0, m1]);
    assert_eq!(e.rank, 2);
}

#[test]
fn t8_dedup() {
    let inst = Inst::new(251, 4, 0);
    let mut st = State::new(&inst, inst.n_fb + 2, false);
    let mut ids: Vec<u32> = vec![];
    for x in 0..251u64 {
        let v: Vec<u32> = (0..inst.curve.n1() as u32).filter(|&i| inst.curve.pts[i as usize].0 == x).collect();
        if v.len() == 3 { ids = v; break; }
    }
    assert_eq!(ids.len(), 3);
    let row = make_row(&inst, 1, &ids, false, None);
    assert!(matches!(st.insert(&inst, &row), Ins::Accepted(_)));
    assert!(matches!(st.insert(&inst, &row), Ins::Dup));
}

#[test]
fn t9_determinism() {
    let a = cell_output(251, 4, 0, 1, &[Arm::A, Arm::B], false);
    let b = cell_output(251, 4, 0, 1, &[Arm::A, Arm::B], false);
    assert_eq!(a, b);
}

#[test]
fn t10_integration() {
    let s = cell_output(251, 5, 2, 1, &[Arm::A, Arm::A2, Arm::B, Arm::PC1, Arm::PC4], false);
    let mut ts = std::collections::HashMap::new();
    for line in s.lines() {
        let get = |k: &str| -> i64 {
            let key = format!("\"{}\":", k);
            let i = line.find(&key).unwrap() + key.len();
            let rest = &line[i..];
            let e = rest.find(|c| c == ',' || c == '}').unwrap();
            rest[..e].trim().parse().unwrap()
        };
        let arm = line.split("\"arm\":\"").nth(1).unwrap().split('"').next().unwrap().to_string();
        assert!(!line.contains("\"censored\":true"), "{}", arm);
        assert!(get("Y") >= get("target"), "{}", arm);
        ts.insert(arm, get("T"));
    }
    assert!(ts["B"] <= ts["A"] + 64);
    let e = cell_output(251, 5, 2, 1, &[Arm::A, Arm::B], true);
    for line in e.lines() {
        assert!(line.contains("\"g4_ok\":true"), "{}", line);
    }
}
