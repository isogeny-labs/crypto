use crate::curve::{Curve, Pt};
use crate::gf::{field, Rng};
use crate::powersum::{self, R};
use crate::systems::{build, holds, plant, Rep};

const NS: [u32; 7] = [11, 13, 17, 19, 23, 29, 31];

#[test]
fn moduli_are_irreducible() {
    for n in NS {
        assert!(field(n).is_irreducible(), "n = {n}");
    }
}

#[test]
fn field_inverse_and_half_trace() {
    for n in NS {
        let f = field(n);
        let mut rng = Rng(n as u64);
        for _ in 0..200 {
            let a = rng.nonzero(&f);
            assert_eq!(f.mul(a, f.inv(a)), 1);
            let c = rng.elem(&f);
            if f.trace(c) == 0 {
                let z = f.half_trace(c);
                assert_eq!(f.sq(z) ^ z, c);
            }
        }
    }
}

#[test]
fn group_law_is_closed_and_associative() {
    let f = field(17);
    let c = Curve { f, a: 0, b: 1 };
    let mut rng = Rng(7);
    for _ in 0..200 {
        let (p, q, r) = (
            c.random_point(&mut rng),
            c.random_point(&mut rng),
            c.random_point(&mut rng),
        );
        assert!(c.on(c.add(p, q)));
        assert_eq!(c.add(c.add(p, q), r), c.add(p, c.add(q, r)));
        assert_eq!(c.add(p, c.neg(p)), Pt::Inf);
    }
}

#[test]
fn s3_vanishes_on_sums_and_line_identities_hold() {
    let f = field(19);
    for a in [0, 1] {
        let c = Curve { f, a, b: 1 };
        let mut rng = Rng(11 + a);
        for _ in 0..500 {
            let (p, q) = (c.random_point(&mut rng), c.random_point(&mut rng));
            let (Pt::A(x1, y1), Pt::A(x2, y2)) = (p, q) else {
                continue;
            };
            if x1 == x2 {
                continue;
            }
            let Pt::A(x3, _) = c.add(p, q) else { continue };
            assert_eq!(c.s3(x1, x2, x3), 0);
            let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
            let mu = y1 ^ f.mul(lam, x1);
            assert_eq!(x1 ^ x2 ^ x3, f.sq(lam) ^ lam ^ a, "e1");
            assert_eq!(f.mul(x1, x2) ^ f.mul(x1, x3) ^ f.mul(x2, x3), mu, "e2");
            assert_eq!(f.mul(f.mul(x1, x2), x3), f.sq(mu) ^ 1, "e3");
        }
    }
}

#[test]
fn every_representation_holds_at_a_planted_relation() {
    let f = field(13);
    for a in [0, 1] {
        let c = Curve { f, a, b: 1 };
        for m in 2..=4 {
            let mut rng = Rng(100 + m as u64 + a);
            let p = plant(&c, 4, m, &mut rng);
            for rep in Rep::ALL.into_iter().chain([Rep::ChainTrace]) {
                let b = build(&c, rep, &p);
                assert!(holds(&b), "{rep:?} m = {m} a = {a}");
            }
        }
    }
}

#[test]
fn quadratic_arms_are_quadratic_and_chain_is_cubic() {
    let f = field(11);
    let c = Curve { f, a: 0, b: 1 };
    let mut rng = Rng(5);
    let p = plant(&c, 3, 4, &mut rng);
    let deg = |rep| crate::systems::system_stats(&build(&c, rep, &p)).max_degree;
    assert_eq!(deg(Rep::Chain), 3);
    assert_eq!(deg(Rep::QuadGeneric), 2);
    assert_eq!(deg(Rep::E2Aux), 2);
    assert_eq!(deg(Rep::Line), 2);
}

#[test]
fn s4_vanishes_on_three_term_relations() {
    let f = field(13);
    let c = Curve { f, a: 1, b: 5 };
    let mut rng = Rng(9);
    for _ in 0..100 {
        let ps: Vec<Pt> = (0..3).map(|_| c.random_point(&mut rng)).collect();
        let s = c.add(c.add(ps[0], ps[1]), ps[2]);
        let xs: Vec<u64> = ps
            .iter()
            .map(|p| match p {
                Pt::A(x, _) => *x,
                _ => 0,
            })
            .collect();
        let Pt::A(xr, _) = s else { continue };
        let r = R { f: &f, k: 3 };
        let s4 = powersum::summation(&r, c.b, xr);
        assert_eq!(r.eval(&s4, &xs), 0);
    }
}

#[test]
fn power_sum_method_controls() {
    let f = field(13);
    let r = R { f: &f, k: 2 };
    // p1 * e2 = p3 + p1^3 in two variables (characteristic 2): j = 1 suffices for e2
    let e2 = r.elem_sym(2);
    assert_eq!(powersum::min_j(&r, &e2, 3), Some(1));
    assert!(powersum::in_elem_span(&r, &e2));
    // a non-symmetric polynomial is in neither span
    assert_eq!(powersum::min_j(&r, &r.x(0), 2), None);
}

#[test]
fn power_sum_positive_control_in_three_variables() {
    let f = field(13);
    let r = R { f: &f, k: 3 };
    let (p1, p3, p5) = (r.power_sum(1), r.power_sum(3), r.power_sum(5));
    // a known element of GF(2^n)[p1, p3, p5] of degree 12, like S_4
    let t = r.add(
        &r.add(&r.mul(&r.pow(&p3, 2), &r.pow(&p1, 6)), &r.mul(&p5, &p1)),
        &r.c(7),
    );
    assert_eq!(powersum::min_j(&r, &t, 2), Some(0));
    // and one that needs a p1 denominator: p1^-1 (p3 + p1^3) * p1^2 = p1 (p3 + p1^3)
    let u = r.mul(&p1, &r.add(&p3, &r.pow(&p1, 3)));
    assert_eq!(powersum::min_j(&r, &u, 2), Some(0));
}

#[test]
fn decomposability_is_exact_on_planted_and_detects_absence() {
    use crate::systems::{decomposable, factor_base_points};
    let f = field(11);
    let c = Curve { f, a: 0, b: 1 };
    let mut rng = Rng(21);
    for _ in 0..20 {
        let p = plant(&c, 3, 3, &mut rng);
        assert!(decomposable(&c, &p.basis, 3, *p.sums.last().unwrap()));
    }
    // T = (0, sqrt b) is on the curve and in every factor base
    let fb = factor_base_points(&c, &[1, 2, 4]);
    assert!(c.on(fb[0]) && matches!(fb[0], Pt::A(0, _)));
    // with m = 1 only factor-base points themselves are decomposable
    let mut seen_no = 0;
    for _ in 0..200 {
        let r = c.random_point(&mut rng);
        let d = decomposable(&c, &[1, 2, 4], 1, r);
        assert_eq!(d, fb.contains(&r));
        seen_no += (!d) as u32;
    }
    assert!(seen_no > 0);
}

#[test]
fn trace_equation_holds_on_planted_chains_and_is_linear() {
    let f = field(13);
    for a in [0, 1] {
        let c = Curve { f, a, b: 1 };
        let mut rng = Rng(31 + a);
        let p = plant(&c, 4, 4, &mut rng);
        let b = build(&c, Rep::ChainTrace, &p);
        assert!(holds(&b));
        assert_eq!(b.bool_eqs.len(), 3);
        assert!(b.bool_eqs.iter().all(|e| e.degree() <= 1));
    }
}
