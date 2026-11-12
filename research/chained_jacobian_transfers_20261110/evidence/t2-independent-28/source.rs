//! Independent flat-field replay of both actual T1 elliptic covers,including
//! rational poles. No producer interpolation/Balanced/Richelot imports.
use isogeny_algos::curve::{self, Curve, Pt};
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::poly;
use std::collections::HashSet;
const F: [u64; 7] = [95, 26, 19, 27, 94, 91, 1];
fn polynomial(f: &FpR, a: &[u64]) -> Vec<ER> {
    a.iter().map(|c| f.from_u64(*c)).collect()
}
struct Certificate {
    source: [u64; 7],
    finite_count: usize,
    rational_pole_count: usize,
    pole_field_degree: usize,
    header: &'static str,
    text: &'static str,
    curve: [u64; 2],
    n: [u64; 5],
    q: [u64; 3],
    m: [u64; 4],
    infinity: [u64; 2],
}
fn check(c: Certificate) {
    // Always check irreducibility of the actual sextic, independently of
    // which field contains this particular geometric pole fibre.
    let source_field = FpR::from_modulus(101, &c.source);
    let f = if c.pole_field_degree == 6 {
        source_field
    } else {
        assert_eq!(c.pole_field_degree, 4);
        FpR::new(101, 4)
    };
    let source = polynomial(&f, &c.source);
    let n = polynomial(&f, &c.n);
    let q = polynomial(&f, &c.q);
    let m = polynomial(&f, &c.m);
    let e = Curve::new(f.from_u64(c.curve[0]), f.from_u64(c.curve[1]));
    let q2 = poly::mul(&f, &q, &q);
    let q4 = poly::mul(&f, &q2, &q2);
    let q6 = poly::mul(&f, &q4, &q2);
    let rhs = poly::add(
        &f,
        &poly::add(
            &f,
            &poly::mul(&f, &poly::mul(&f, &n, &n), &n),
            &poly::scale(&f, &poly::mul(&f, &n, &q4), e.a),
        ),
        &poly::scale(&f, &q6, e.b),
    );
    let lhs = poly::mul(&f, &source, &poly::mul(&f, &m, &m));
    assert_eq!(lhs, rhs);
    assert!(curve::is_smooth(&f, &e));
    assert_eq!(
        poly::gcd(&f, &source, &poly::derivative(&f, &source)),
        vec![f.one()]
    );
    assert_eq!(poly::gcd(&f, &n, &q), vec![f.one()]);
    assert_eq!(poly::gcd(&f, &source, &q), vec![f.one()]);
    assert_eq!(poly::gcd(&f, &q, &poly::derivative(&f, &q)), vec![f.one()]);
    assert_eq!(poly::deg(&f, &n).max(poly::deg(&f, &q2)), 4);
    let derivative = poly::sub(
        &f,
        &poly::mul(&f, &poly::derivative(&f, &n), &q2),
        &poly::mul(&f, &n, &poly::derivative(&f, &q2)),
    );
    assert!(poly::deg(&f, &derivative) >= 0);
    let mut lines = c.text.lines();
    assert_eq!(lines.next().unwrap(), c.header);
    let mut domain = HashSet::new();
    let mut rational_poles = HashSet::new();
    let mut finite = 0;
    for line in lines {
        let row = line.split_whitespace().collect::<Vec<_>>();
        assert_eq!(row.len(), 4);
        let x = row[0].parse::<u64>().unwrap();
        let y = row[1].parse::<u64>().unwrap();
        assert!(x < 101 && y < 101);
        assert!(domain.insert((x, y)));
        let xf = f.from_u64(x);
        let yf = f.from_u64(y);
        assert_eq!(f.sq(yf), poly::eval(&f, &source, xf));
        let qx = poly::eval(&f, &q, xf);
        if row[2] == "O" || row[3] == "O" {
            assert_eq!(&row[2..], ["O", "O"]);
            assert_eq!(qx, f.zero());
            assert_ne!(poly::eval(&f, &n, xf), f.zero());
            assert_ne!(poly::eval(&f, &m, xf), f.zero());
            assert!(rational_poles.insert((xf, yf)));
        } else {
            let xe = row[2].parse::<u64>().unwrap();
            let ye = row[3].parse::<u64>().unwrap();
            assert!(xe < 101 && ye < 101);
            assert_ne!(qx, f.zero());
            let p = Pt::Aff(f.from_u64(xe), f.from_u64(ye));
            assert!(curve::on_curve(&f, &e, &p));
            assert_eq!(f.div(poly::eval(&f, &n, xf), f.sq(qx)), f.from_u64(xe));
            assert_eq!(
                f.div(f.mul(yf, poly::eval(&f, &m, xf)), f.pow(qx, 3)),
                f.from_u64(ye)
            );
            finite += 1;
        }
    }
    assert_eq!(finite, c.finite_count);
    assert!(finite > 16);
    assert_eq!(rational_poles.len(), c.rational_pole_count);
    assert_eq!(domain.len(), 118);
    let mut complete = HashSet::new();
    for x in 0..101 {
        for y in 0..101 {
            if f.sq(f.from_u64(y)) == poly::eval(&f, &source, f.from_u64(x)) {
                complete.insert((x, y));
            }
        }
    }
    assert_eq!(domain, complete);
    let infinity = Pt::Aff(n[4], m[3]);
    assert_eq!(
        infinity,
        Pt::Aff(f.from_u64(c.infinity[0]), f.from_u64(c.infinity[1]))
    );
    assert!(curve::on_curve(&f, &e, &infinity));
    assert!(curve::on_curve(&f, &e, &curve::neg(&f, &infinity)));
    let mut rng = Rng::new(0xC731 + c.curve[0]);
    let roots = poly::roots(&f, &q, &mut rng);
    assert_eq!(roots.len(), 2);
    let mut geometric = HashSet::new();
    let mut rational = HashSet::new();
    for x in roots {
        assert_eq!(poly::eval(&f, &q, x), f.zero());
        assert_ne!(poly::eval(&f, &poly::derivative(&f, &q), x), f.zero());
        assert_ne!(poly::eval(&f, &n, x), f.zero());
        assert_ne!(poly::eval(&f, &m, x), f.zero());
        let fx = poly::eval(&f, &source, x);
        assert_ne!(fx, f.zero());
        let y = f.sqrt(fx).expect("four pole branches in declared field");
        for y in [y, f.neg(y)] {
            assert_eq!(f.sq(y), fx);
            assert!(geometric.insert((x, y)));
            if c.pole_field_degree == 4 {
                assert_ne!(f.pow(x, 101), x);
                assert_eq!(f.pow(x, 101 * 101), x);
                assert_eq!(f.pow(y, 101 * 101), f.neg(y));
                assert_ne!(y, f.neg(y));
                assert_eq!(f.pow(y, 101 * 101 * 101 * 101), y);
            }
            if f.pow(x, 101) == x && f.pow(y, 101) == y {
                rational.insert((x, y));
            }
        }
    }
    assert_eq!(geometric.len(), 4);
    assert_eq!(rational, rational_poles);
    let wrong = poly::add(&f, &rhs, &poly::scale(&f, &q6, f.one()));
    assert_ne!(lhs, wrong, "wrong codomain b rejected");
    println!("PASS independent source{:?} target{:?} exact curve identity/irreducible source/gcd/separabledegree4,{} finite+{}origin witnesses/complete118point domain,both infinity limits/four simple geometric poles exactly{} rational,declared F101^{} modulus{:?},degree4 field requires all four pole Frobenius orbits4,wrong codomain;old H provenance/global bridge separate",c.source,c.curve,finite,c.rational_pole_count,c.rational_pole_count,c.pole_field_degree,f.modulus());
}
#[test]
fn t1_e109prime_complete_certificate_including_rational_poles() {
    check(Certificate {
        source: F,
        finite_count: 116,
        rational_pole_count: 2,
        pole_field_degree: 6,
        header: "CT28 T1 e109prime F101 x y X Y centred minus q=(84,37)",
        text: include_str!("../fixtures/quartic/t1-e109prime-points.tsv"),
        curve: [26, 58],
        n: [8, 60, 36, 0, 84],
        q: [47, 0, 1],
        m: [98, 45, 84, 37],
        infinity: [84, 37],
    });
}
#[test]
fn t1_e113prime_complete_certificate_including_rational_poles() {
    check(Certificate {
        source: F,
        finite_count: 116,
        rational_pole_count: 2,
        pole_field_degree: 6,
        header: "CT28 T1 e113prime F101 x y X Y centred minus q=(46,41)",
        text: include_str!("../fixtures/quartic/t1-e113prime-points.tsv"),
        curve: [69, 50],
        n: [3, 0, 76, 42, 46],
        q: [70, 0, 1],
        m: [4, 76, 75, 41],
        infinity: [46, 41],
    });
}

#[test]
fn t2_e109second_complete_certificate_and_all_geometric_poles() {
    check(Certificate {
        source: [95, 93, 43, 39, 92, 17, 1],
        finite_count: 118,
        rational_pole_count: 0,
        pole_field_degree: 4,
        header: "CT30 T2 e109second F101 x y X Y centred minus q=(97,63)",
        text: include_str!("../fixtures/quartic/t2-e109second-points.tsv"),
        curve: [59, 27],
        n: [97, 93, 7, 0, 97],
        q: [55, 0, 1],
        m: [71, 49, 20, 63],
        infinity: [97, 63],
    });
}

#[test]
fn t2_e113second_complete_certificate_including_rational_poles() {
    check(Certificate {
        source: [95, 93, 43, 39, 92, 17, 1],
        finite_count: 116,
        rational_pole_count: 2,
        pole_field_degree: 6,
        header: "CT30 T2 e113second F101 x y X Y centred minus q=(64,35)",
        text: include_str!("../fixtures/quartic/t2-e113second-points.tsv"),
        curve: [23, 7],
        n: [74, 0, 81, 42, 64],
        q: [23, 0, 1],
        m: [37, 9, 9, 35],
        infinity: [64, 35],
    });
}
