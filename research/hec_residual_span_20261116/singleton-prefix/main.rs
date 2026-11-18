#[path = "../batched/inverse.rs"]
mod inverse;
#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split, OracleStats},
    poly::Poly,
    rng::Rng,
};
use std::collections::BTreeSet;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Relation {
    a: u64,
    t: u64,
    points: Vec<(u64, u64)>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    relations: Vec<Relation>,
    queries: usize,
    nonsplit: usize,
    duplicates: usize,
    skips: usize,
    zero: usize,
}
fn prefix(c: &Curve, pt: (u64, u64), d: &Div, target: &Div, sm: u64, cap: u64) -> Report {
    assert!(c.is_valid(d) && c.is_valid(target));
    assert!(!c.is_zero(d));
    let base = c.place(pt.0, pt.1);
    assert!(c.is_valid(&base));
    assert_eq!(c.mul(&base, sm), *d);
    let f = &c.fp;
    let z = root::power_map(f, 5).0;
    let mut source = vec![];
    let mut x = pt.0;
    for _ in 0..5 {
        source.push((x, pt.1));
        x = f.mul(x, z);
    }
    let mut anchor = *target;
    let mut seen = BTreeSet::new();
    let mut r = Report {
        relations: vec![],
        queries: 0,
        nonsplit: 0,
        duplicates: 0,
        skips: 0,
        zero: 0,
    };
    for ai in 1..=cap {
        anchor = c.add_place(&anchor, pt.0, pt.1);
        if anchor.u.deg() != 2 {
            continue;
        }
        let mut entries = vec![];
        let mut den = vec![];
        for &(x, y) in &source {
            let u = anchor.u.eval(x, f);
            if u == 0 {
                r.skips += 2;
                continue;
            }
            entries.push((x, y, anchor.v.eval(x, f)));
            den.push(u);
        }
        let iv = inverse::invert(&den, f);
        for (&(x, y, v), iv) in entries.iter().zip(iv) {
            for yy in [y, f.neg(y)] {
                r.queries += 1;
                let t = f.mul(f.sub(yy, v), iv);
                if !seen.insert((ai, t)) {
                    r.duplicates += 1;
                    continue;
                }
                let tt = f.mul(t, t);
                let h2 = f.neg(f.add(tt, anchor.u.c[1]));
                let w1 = f.add(anchor.v.c[1], f.mul(t, anchor.u.c[1]));
                let h1 = f.neg(f.add(
                    f.add(f.mul(f.mul(2, t), w1), f.mul(anchor.u.c[1], h2)),
                    anchor.u.c[0],
                ));
                let q1 = f.add(h2, x);
                let q0 = f.add(h1, f.mul(x, q1));
                let q = Poly::from_coeffs(&[q0, q1, 1]);
                let Some(roots) = split(&q, f, &mut OracleStats::default()) else {
                    r.nonsplit += 1;
                    continue;
                };
                let w = anchor.v.add(&anchor.u.scale(t, f), f);
                let mut points = vec![(x, yy)];
                for &(xx, m) in roots.as_slice() {
                    for _ in 0..m {
                        points.push((xx, w.eval(xx, f)));
                    }
                }
                if points.iter().any(|p| p.0 == 0) {
                    r.zero += 1;
                    continue;
                }
                r.relations.push(Relation { a: ai, t, points });
            }
        }
    }
    r
}
fn main() {
    counted::disable();
    let expected = include_str!("EXPECTED.tsv");
    let baselines = include_str!("BASELINES.tsv");
    let (mut cells, mut exceeds) = (0, 0);
    for (p, cc, ell) in [(101, 2, 7481), (251, 18, 67931), (601, 7, 411491)] {
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&[cc, 0, 0, 0, 0, 1]));
        let pt = factor_base(&c)[0];
        let base = c.place(pt.0, pt.1);
        let mut rng = Rng::new(2026113000 ^ p);
        for sm in [1, 17] {
            let d = c.mul(&base, sm);
            for case in 0..2 {
                let k = 1 + rng.below(ell - 1);
                let target = c.mul(&d, k);
                let key = format!("{p}\t{sm}\t{case}\t");
                let fields: Vec<_> = expected
                    .lines()
                    .find(|l| l.starts_with(&key))
                    .unwrap()
                    .splitn(5, '\t')
                    .collect();
                let cap = fields[3].parse().unwrap();
                let mut old = None;
                for rep in 0..2 {
                    counted::reset();
                    let report = prefix(&c, pt, &d, &target, sm, cap);
                    let counts = counted::snapshot();
                    counted::disable();
                    assert_eq!(format!("{:?}", report.relations), fields[4]);
                    for rel in &report.relations {
                        let mut anchor = target;
                        for _ in 0..rel.a {
                            anchor = c.add(&anchor, &base);
                        }
                        let w = anchor.v.add(&anchor.u.scale(rel.t, &c.fp), &c.fp);
                        let h =
                            c.f.sub(&w.mul(&w, &c.fp), &c.fp)
                                .div_exact(&anchor.u, &c.fp);
                        let mut product = Poly::one();
                        let mut group = anchor;
                        for &(x, y) in &rel.points {
                            assert_eq!(w.eval(x, &c.fp), y);
                            assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
                            product = product.mul(&Poly::x_minus(x, &c.fp), &c.fp);
                            group = c.add_place(&group, x, y);
                        }
                        assert_eq!(h, product);
                        assert!(c.is_zero(&group));
                    }
                    let signature = (report.clone(), counts.clone());
                    if let Some(ref v) = old {
                        assert_eq!(v, &signature)
                    } else {
                        old = Some(signature)
                    }
                    let prefix_cost = counts.get(&p).unwrap()[8];
                    for (m, v) in &counts {
                        assert_eq!(*m, p);
                        assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                        println!("COST\tp={p}\tsm={sm}\tcase={case}\trep={rep}\tcap={cap}\tqueries={}\trows={}\tcounts={v:?}",report.queries,report.relations.len());
                    }
                    if rep == 0 {
                        for l in baselines.lines().filter(|l| l.starts_with(&key)) {
                            let a: Vec<_> = l.split('\t').collect();
                            let b: u64 = a[4].parse().unwrap();
                            let failed = 10 * prefix_cost > b;
                            println!("BOUND\tp={p}\tsm={sm}\tcase={case}\tcomparator={}\tprefix_REDC={prefix_cost}\tcomplete_REDC={b}\texceeds_tenth={failed}",a[3]);
                            if a[3] == "derived_candidate_total" && failed {
                                exceeds += 1
                            }
                        }
                    }
                }
                cells += 1;
            }
        }
    }
    assert_eq!(cells, 12);
    eprintln!("PASS cells=12 paid_prefix_calls=24 exact_prior_rows=true exact_principal_native_checks=true repeated_vectors=true exceeds_core_step_tenth={exceeds} oracle_caps_from_prior=true stopping_LA_certificate_free=true full_solver=false independent_verification=false");
}
