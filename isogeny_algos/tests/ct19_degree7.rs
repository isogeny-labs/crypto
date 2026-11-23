//! CT19: fixed F101 degree-seven kernel, exact-map, and dual controls.
//! These are development assertions, not an independently signed PCC verdict.
//! Run: cargo test --manifest-path isogeny_algos/Cargo.toml --test ct19_degree7 -- --nocapture
use isogeny_algos::curve::{on_curve, padd, pmul, rhs, Curve, Isogeny, Pt, RatIsogeny};
use isogeny_algos::field::{Field, Rng, Zp};
use isogeny_algos::find::divpoly::{division_poly, kernel_polys};
use isogeny_algos::kernel::kohel::kohel;
use isogeny_algos::poly;

#[derive(Clone, Copy)]
struct Fixture {
    source: (u64, u64),
    order: usize,
    kernel: [u64; 4], // low to high, monic
    target: (u64, u64),
    dual_kernel: [u64; 4],
}

const FIXTURES: [Fixture; 4] = [
    Fixture {
        source: (1, 46), order: 109, kernel: [9, 76, 20, 1],
        target: (5, 39), dual_kernel: [49, 69, 62, 1],
    },
    Fixture {
        source: (1, 46), order: 109, kernel: [72, 100, 35, 1],
        target: (26, 58), dual_kernel: [58, 26, 58, 1],
    },
    Fixture {
        source: (44, 21), order: 113, kernel: [77, 86, 71, 1],
        target: (13, 42), dual_kernel: [42, 13, 8, 1],
    },
    Fixture {
        source: (44, 21), order: 113, kernel: [59, 46, 8, 1],
        target: (69, 50), dual_kernel: [29, 88, 45, 1],
    },
];

// A coefficientwise cleared polynomial identity over F101,
// not a random-x or point-only identity. For f(x)=N(x)/D(x) and
// f'(x)=S(x)/D(x)^2, the elliptic curve identity is
//       (x^3+a*x+b)*S(x)^2 = D(x)*(N(x)^3+a'*N(x)*D(x)^2+b'*D(x)^3).
fn exact_curve_identity(fp: &Zp, iso: &RatIsogeny<Zp>) -> bool {
    let n = &iso.num;
    let d = &iso.den;
    let nd = poly::mul(fp, &poly::derivative(fp, n), d);
    let dn = poly::mul(fp, n, &poly::derivative(fp, d));
    let s = poly::sub(fp, &nd, &dn);
    let source_cubic = vec![iso.dom.b, iso.dom.a, 0, 1];
    let lhs = poly::mul(fp, &source_cubic, &poly::mul(fp, &s, &s));
    let d2 = poly::mul(fp, d, d);
    let d3 = poly::mul(fp, &d2, d);
    let n3 = poly::mul(fp, n, &poly::mul(fp, n, n));
    let and2 = poly::scale(fp, &poly::mul(fp, n, &d2), iso.cod.a);
    let bd3 = poly::scale(fp, &d3, iso.cod.b);
    let sum = poly::add(fp, &poly::add(fp, &n3, &and2), &bd3);
    let right = poly::mul(fp, d, &sum);
    lhs == right
}

fn all_points(fp: &Zp, e: &Curve<u64>) -> Vec<Pt<u64>> {
    let mut points = vec![Pt::Inf];
    for x in 0..fp.p {
        let r = rhs(fp, e, x);
        for y in 0..fp.p {
            if fp.mul(y, y) == r {
                points.push(Pt::Aff(x, y));
            }
        }
    }
    points
}

fn certified_map(fp: &Zp, source: &Curve<u64>, kernel: &[u64; 4]) -> RatIsogeny<Zp> {
    let h = kernel.to_vec();
    assert_eq!(h[3], 1);
    let cubic = vec![source.b, source.a, 0, 1];
    let div = division_poly(fp, source, 7);
    assert!(poly::rem(fp, &div, &h).is_empty(), "h must divide psi_7");
    assert_eq!(poly::gcd(fp, &h, &poly::derivative(fp, &h)), vec![1]);
    assert_eq!(poly::gcd(fp, &h, &cubic), vec![1]);
    let iso = kohel(fp, source, &h, 7);
    assert_eq!(iso.deg, 7);
    assert_eq!(poly::gcd(fp, &iso.num, &iso.den), vec![1]);
    assert!(exact_curve_identity(fp, &iso), "exact curve identity must hold");
    iso
}

fn scaled_dual_point(fp: &Zp, raw: Pt<u64>) -> Pt<u64> {
    let inv7 = fp.inv(7);
    let inv49 = fp.mul(inv7, inv7);
    let inv343 = fp.mul(inv49, inv7);
    match raw {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(fp.mul(inv49, x), fp.mul(inv343, y)),
    }
}

#[test]
fn ct19_four_elliptic_seven_maps_exact_and_exhaustive() {
    let fp = Zp::new(101);
    let mut rng = Rng::new(0xC719_2026);
    let inv7 = fp.inv(7);
    let inv7_2 = fp.mul(inv7, inv7);
    let inv7_4 = fp.mul(inv7_2, inv7_2);
    let inv7_6 = fp.mul(inv7_4, inv7_2);
    for fixture in FIXTURES {
        let e = Curve::new(fixture.source.0, fixture.source.1);
        let phi = certified_map(&fp, &e, &fixture.kernel);
        assert_eq!((phi.cod.a, phi.cod.b), fixture.target);
        let e_points = all_points(&fp, &e);
        let eprime_points = all_points(&fp, &phi.cod);
        assert_eq!(e_points.len(), fixture.order);
        assert_eq!(eprime_points.len(), fixture.order);
        assert_eq!(fixture.order % 7, if fixture.order == 109 { 4 } else { 1 });

        // The raw Kohel dual is normalized. Scaling by 1/7 is the actual
        // dual, and the target model equality below checks that scaling.
        let psi = certified_map(&fp, &phi.cod, &fixture.dual_kernel);
        assert_eq!(fp.mul(psi.cod.a, inv7_4), e.a);
        assert_eq!(fp.mul(psi.cod.b, inv7_6), e.b);

        for &p in &e_points {
            let q = phi.eval(&fp, &p);
            assert!(on_curve(&fp, &phi.cod, &q));
            assert_eq!(scaled_dual_point(&fp, psi.eval(&fp, &q)), pmul(&fp, &e, &p, 7));
            // All nonidentity source points have order 109 or 113 and
            // therefore remain nonzero under a degree-seven isogeny.
            if p != Pt::Inf {
                assert_ne!(q, Pt::Inf);
            }
        }
        for &q in &eprime_points {
            let p = scaled_dual_point(&fp, psi.eval(&fp, &q));
            assert!(on_curve(&fp, &e, &p));
            assert_eq!(phi.eval(&fp, &p), pmul(&fp, &phi.cod, &q, 7));
        }

        // Exhaust the *whole* rational point group, not a few random
        // samples or just the chosen public generator's scalar orbit.
        for &p in &e_points {
            for &q in &e_points {
                let sum = padd(&fp, &e, &p, &q);
                assert_eq!(
                    phi.eval(&fp, &sum),
                    padd(&fp, &phi.cod, &phi.eval(&fp, &p), &phi.eval(&fp, &q)),
                    "phi must preserve addition"
                );
            }
        }
        println!(
            "F101 source={:?} kernel={:?} target={:?}, {} points, exact and dual checks",
            fixture.source, fixture.kernel, fixture.target, fixture.order
        );
    }

    // The existing factor-search path is intentionally treated as an
    // untrusted proposer. The fixed certificates above are checked without
    // any of its four-random-x acceptance heuristic.
    for &(curve, order) in &[((1, 46), 109usize), ((44, 21), 113usize)] {
        let e = Curve::new(curve.0, curve.1);
        let mut got = kernel_polys(&fp, &e, 7, &mut rng);
        got.sort();
        let mut expected: Vec<Vec<u64>> = FIXTURES
            .iter()
            .filter(|x| x.source == curve && x.order == order)
            .map(|x| x.kernel.to_vec())
            .collect();
        expected.sort();
        assert_eq!(got, expected, "fixed F101 rational seven-kernel census");
    }
}

#[test]
fn ct19_second_quotient_short_model_bridge() {
    let fp = Zp::new(101);
    // y^2 = u^3 + 13u^2 + 33u + 5; X = u + 13/3 = u + 38.
    // The short model is y^2 = X^3 + 44X + 21.
    let translated = Curve::new(44, 21);
    for u in 0..101 {
        let x = (u + 38) % 101;
        let rhs_long = (u * u * u + 13 * u * u + 33 * u + 5) % 101;
        assert_eq!(rhs_long, rhs(&fp, &translated, x));
    }
}
