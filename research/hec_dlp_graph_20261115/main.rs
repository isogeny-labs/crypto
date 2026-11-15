mod core;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
mod ode;
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split, OracleStats},
    poly::Poly,
    rng::Rng,
    solve::Row,
};
use std::{fs, path::Path};
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
#[derive(Clone)]
struct Snap {
    t: u64,
    known: u64,
    depth: u64,
    root_rows: u64,
    root_empty: u64,
    root_unique: u64,
    root_entries: u64,
    root_rank: usize,
    root_at: u64,
    full_rows: u64,
    full_unique: u64,
    full_entries: u64,
    full_rank: usize,
    full_at: u64,
    cycle_sum: u64,
    cycle_count: u64,
}
fn fits(observed: f64, predicted: f64) -> bool {
    observed + 5.0 >= predicted / 2.0 && observed <= 2.0 * predicted + 5.0
}
fn run(fix: &Fixture, d2: &Div, seed: u64) -> (Option<u64>, Option<u64>) {
    let c = &fix.c;
    let ell = fix.ell;
    let f = Fp::new(ell);
    let fb = factor_base(c);
    let n = fb.len();
    let h = ((c.fp.p as f64).powf(1.0 - 1.0 / c.g as f64).ceil() as usize).min(n);
    let l = n - h;
    assert!(l > 0);
    let mut full = core::Core::new(n, h, ell);
    let mut root = core::Core::new(n, h, ell);
    let mut known = vec![false; n + 1];
    known[0] = true;
    let mut depth = vec![0u64; n + 1];
    let (mut reached, mut depth_sum, mut cycle_sum, mut cycle_count) = (0, 0, 0, 0);
    let mut rng = Rng::new(202611150000 + seed);
    let mut jumps = Vec::new();
    for _ in 0..16 {
        let a = rng.below(ell);
        let b = rng.below(ell);
        jumps.push((a, b, c.add(&c.mul(&fix.d1, a), &c.mul(d2, b))));
    }
    let (mut a, mut b, mut r) = (0, 0, c.zero());
    let mut classes = [0u64; 4];
    let mut train = [0u64; 3];
    let mut snapshots = Vec::new();
    let mut checks = 0;
    let mut ramified = 0;
    let mut repeated = 0;
    for t in 1..=32768u64 {
        let (da, db, jump) = jumps[rng.below(16) as usize];
        a = f.add(a, da);
        b = f.add(b, db);
        r = c.add(&r, &jump);
        if let Some(roots) = split(&r.u, &c.fp, &mut OracleStats::default()) {
            let mut sum = c.zero();
            let (mut on, mut off) = (Row::new(), Vec::new());
            for &(x, m) in roots.as_slice() {
                let j = fb.binary_search_by_key(&x, |&(x, _)| x).unwrap();
                let y = r.v.eval(x, &c.fp);
                assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
                sum = c.add(&sum, &c.mul(&c.place(x, y), m as u64));
                let v = if y == fb[j].1 {
                    m as u64
                } else {
                    assert_eq!(y, c.fp.neg(fb[j].1));
                    f.neg(m as u64)
                };
                if j < h {
                    on.push((j as u32, v));
                } else {
                    off.push((j + 1, v));
                }
                ramified += u64::from(y == 0);
                repeated += u64::from(m > 1);
            }
            assert_eq!(sum, r, "whole-Jacobian decomposition");
            checks += 1;
            classes[off.len().min(3)] += 1;
            if off.len() <= 2 {
                if a != 0 {
                    on.push((h as u32, f.neg(a)));
                }
                if b != 0 {
                    on.push((h as u32 + 1, f.neg(b)));
                }
                if let Some(k) = full.feed(&off, &on, t) {
                    assert_eq!(c.mul(&fix.d1, k), *d2);
                }
                let mut accept = true;
                match off.as_slice() {
                    [] => {
                        cycle_sum += 1;
                        cycle_count += 1;
                    }
                    &[(u, _)] => {
                        if known[u] {
                            cycle_sum += depth[u] + 1;
                            cycle_count += 1;
                        } else {
                            known[u] = true;
                            depth[u] = 1;
                            reached += 1;
                            depth_sum += 1;
                        }
                    }
                    &[(u, _), (v, _)] => match (known[u], known[v]) {
                        (false, false) => accept = false,
                        (true, true) => {
                            cycle_sum += depth[u] + depth[v] + 1;
                            cycle_count += 1;
                        }
                        _ => {
                            let (new, old) = if known[u] { (v, u) } else { (u, v) };
                            known[new] = true;
                            depth[new] = depth[old] + 1;
                            depth_sum += depth[new];
                            reached += 1;
                        }
                    },
                    _ => unreachable!(),
                }
                if accept {
                    if let Some(k) = root.feed(&off, &on, t) {
                        assert_eq!(c.mul(&fix.d1, k), *d2);
                    }
                }
            }
        }
        if t == 2048 {
            train.copy_from_slice(&classes[..3]);
        }
        if [512, 2048, 8192, 32768].contains(&t) {
            assert_eq!(
                r,
                c.add(&c.mul(&fix.d1, a), &c.mul(d2, b)),
                "public label gate"
            );
            assert!(reached <= l as u64);
            assert_eq!(root.rows, cycle_count);
            assert!(depth_sum <= reached * t);
            snapshots.push(Snap {
                t,
                known: reached,
                depth: depth_sum,
                root_rows: root.rows,
                root_empty: root.empty,
                root_unique: root.unique,
                root_entries: root.entries,
                root_rank: root.echelon.rank,
                root_at: root.solved_at,
                full_rows: full.rows,
                full_unique: full.unique,
                full_entries: full.entries,
                full_rank: full.echelon.rank,
                full_at: full.solved_at,
                cycle_sum,
                cycle_count,
            });
        }
    }
    let rates = train.map(|n| n as f64 / 2048.0);
    let mut state = ode::State::default();
    let mut from = 0;
    for s in snapshots {
        ode::integrate(
            &mut state, from, s.t, l as f64, rates[0], rates[1], rates[2],
        );
        from = s.t;
        let pk = l as f64 * state.u;
        let pd = l as f64 * state.v;
        let holdout = s.t > 2048;
        let ok = fits(s.known as f64, pk)
            && fits(s.root_rows as f64, state.f)
            && fits(s.depth as f64, pd);
        println!("SNAP {{\"g\":{},\"p\":{},\"seed\":{seed},\"t\":{},\"h\":{h},\"l\":{l},\"holdout\":{holdout},\"known\":{},\"depth\":{},\"root_rows\":{},\"root_empty\":{},\"root_unique\":{},\"root_entries\":{},\"root_rank\":{},\"root_solved_at\":{},\"full_rows\":{},\"full_unique\":{},\"full_entries\":{},\"full_rank\":{},\"full_solved_at\":{},\"cycle_sum\":{},\"cycle_count\":{},\"pred_known\":{pk},\"pred_rows\":{},\"pred_depth\":{pd},\"M2\":{ok},\"train_a\":{},\"train_b\":{},\"train_c\":{}}}",c.g,c.fp.p,s.t,s.known,s.depth,s.root_rows,s.root_empty,s.root_unique,s.root_entries,s.root_rank,s.root_at,s.full_rows,s.full_unique,s.full_entries,s.full_rank,s.full_at,s.cycle_sum,s.cycle_count,state.f,rates[0],rates[1],rates[2]);
    }
    println!("TARGET g={} p={} seed={seed} trials=32768 raw_checks={checks} classes={classes:?} repeated={repeated} ramified={ramified} full_k={:?} root_k={:?}",c.g,c.fp.p,full.k,root.k);
    (full.k, root.k)
}
fn main() {
    ode::gate();
    if std::env::args().nth(1).as_deref() == Some("--ode-gate") {
        return;
    }
    let root = std::env::args().nth(1).expect("fixture root");
    let (mut full, mut rooted) = (0, 0);
    for (g, p) in [
        (2, 1009),
        (2, 16381),
        (3, 127),
        (3, 1009),
        (4, 31),
        (4, 127),
    ] {
        let fix = Fixture::parse(
            &fs::read_to_string(Path::new(&root).join(format!("select-g{g}-p{p}-f0.out"))).unwrap(),
        );
        for seed in [14, 15, 16] {
            let (expected, d2) = fix.target(seed);
            let (a, b) = run(&fix, &d2, seed);
            if let Some(k) = a {
                assert_eq!(k, expected);
                full += 1;
            }
            if let Some(k) = b {
                assert_eq!(k, expected);
                rooted += 1;
            }
        }
    }
    println!(
        "TERMINAL targets=18 trials=589824 full_recovered={full} rooted_recovered={rooted} M1=true"
    );
}
