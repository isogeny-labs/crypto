//! Independent frozen-cover/certificate replay with the existing flat field
//! and polynomial engines. No producer interpolation/Richelot/Balanced import.
use isogeny_algos::curve::{self, Curve, Pt};
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::poly;
use std::collections::HashSet;
const SOURCE: [u64; 7] = [31, 75, 74, 54, 71, 55, 1];
const POINTS: &str = include_str!("../fixtures/quartic/t0-e109-points.tsv");

fn polynomial(f: &FpR, a: &[u64]) -> Vec<ER> {
    a.iter().map(|c| f.from_u64(*c)).collect()
}

fn identity(f: &FpR, n: &Vec<ER>, q: &Vec<ER>, m: &Vec<ER>, e: &Curve<ER>) -> bool {
    let q2 = poly::mul(f, q, q);
    let q4 = poly::mul(f, &q2, &q2);
    let q6 = poly::mul(f, &q4, &q2);
    let rhs = poly::add(
        f,
        &poly::add(
            f,
            &poly::mul(f, &poly::mul(f, n, n), n),
            &poly::scale(f, &poly::mul(f, n, &q4), e.a),
        ),
        &poly::scale(f, &q6, e.b),
    );
    rhs == poly::mul(f, &polynomial(f, &SOURCE), &poly::mul(f, m, m))
}

fn points() -> Vec<[u64; 4]> {
    let mut lines = POINTS.lines();
    assert_eq!(
        lines.next().unwrap(),
        "CT28 T0 E109 F101 x y X Y centred minus q=(14,73)"
    );
    let out: Vec<_> = lines
        .map(|line| {
            let row = line
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(row.len(), 4);
            assert!(row.iter().all(|c| *c < 101));
            <[u64; 4]>::try_from(row).unwrap()
        })
        .collect();
    assert_eq!(out.len(), 118);
    out
}

#[test]
fn independent_first_quartic_identity_certificate_and_geometric_poles() {
    // Constructor performs an existing Rabin irreducibility check on the
    // actual source sextic; this also supplies a different field representation.
    let f = FpR::from_modulus(101, &SOURCE);
    let n = polynomial(&f, &[14, 0, 88, 97, 14]);
    let q = polynomial(&f, &[63, 0, 1]);
    let m = polynomial(&f, &[41, 40, 69, 73]);
    let source = polynomial(&f, &SOURCE);
    let e = Curve::new(f.one(), f.from_u64(46));
    assert!(identity(&f, &n, &q, &m, &e));
    assert!(curve::is_smooth(&f, &e));
    assert_eq!(
        poly::gcd(&f, &source, &poly::derivative(&f, &source)),
        vec![f.one()]
    );
    assert_eq!(poly::gcd(&f, &n, &q), vec![f.one()]);
    assert_eq!(poly::gcd(&f, &q, &poly::derivative(&f, &q)), vec![f.one()]);
    let denominator = poly::mul(&f, &q, &q);
    assert_eq!(poly::deg(&f, &n).max(poly::deg(&f, &denominator)), 4);
    let derivative = poly::sub(
        &f,
        &poly::mul(&f, &poly::derivative(&f, &n), &denominator),
        &poly::mul(&f, &n, &poly::derivative(&f, &denominator)),
    );
    assert!(poly::deg(&f, &derivative) >= 0);
    let mut domain = HashSet::new();
    for [x, y, xe, ye] in points() {
        assert!(domain.insert((x, y)), "distinct source certificate points");
        let (x, y, xe, ye) = (f.from_u64(x), f.from_u64(y), f.from_u64(xe), f.from_u64(ye));
        assert_eq!(f.sq(y), poly::eval(&f, &source, x));
        assert!(curve::on_curve(&f, &e, &Pt::Aff(xe, ye)));
        let qx = poly::eval(&f, &q, x);
        assert_ne!(qx, f.zero());
        assert_eq!(f.div(poly::eval(&f, &n, x), f.sq(qx)), xe);
        assert_eq!(f.div(f.mul(y, poly::eval(&f, &m, x)), f.pow(qx, 3)), ye);
    }
    let mut complete = HashSet::new();
    for x in 0..101 {
        for y in 0..101 {
            if f.sq(f.from_u64(y)) == poly::eval(&f, &source, f.from_u64(x)) {
                complete.insert((x, y));
            }
        }
    }
    assert_eq!(domain, complete);
    assert!(domain.len() > 16, "degree-bound global uniqueness premise");
    let plus = Pt::Aff(n[4], m[3]);
    assert_eq!(plus, Pt::Aff(f.from_u64(14), f.from_u64(73)));
    assert!(curve::on_curve(&f, &e, &plus));
    assert!(curve::on_curve(&f, &e, &curve::neg(&f, &plus)));
    let mut rng = Rng::new(0xC728);
    let roots = poly::roots(&f, &q, &mut rng);
    assert_eq!(roots.len(), 2);
    let mut poles = HashSet::new();
    for x in roots {
        assert_eq!(poly::eval(&f, &q, x), f.zero());
        assert_ne!(poly::eval(&f, &poly::derivative(&f, &q), x), f.zero());
        assert_ne!(poly::eval(&f, &n, x), f.zero());
        assert_ne!(poly::eval(&f, &m, x), f.zero());
        let fx = poly::eval(&f, &source, x);
        assert_ne!(fx, f.zero());
        let y = f
            .sqrt(fx)
            .expect("fixed geometric poles lie in the declared F101^6");
        assert_eq!(f.sq(y), fx);
        assert!(poles.insert((x, y)));
        assert!(poles.insert((x, f.neg(y))));
    }
    assert_eq!(poles.len(), 4);
    println!("PASS independent flat-field exact quartic identity/gcd/smoothness/separablexdegree4,all118 distinct complete rational certificate points,finite infinity±(14,±73),four distinct simple geometric pole branches overdeclaredFp6. Old H expected-value provenance and global degree argument are separately written6afa9c;no independent Richelot evaluation or timing claim");
}

#[test]
fn quartic_negative_controls_separate_identity_from_global_identification() {
    let f = FpR::from_modulus(101, &SOURCE);
    let n = polynomial(&f, &[14, 0, 88, 97, 14]);
    let q = polynomial(&f, &[63, 0, 1]);
    let m = polynomial(&f, &[41, 40, 69, 73]);
    let e = Curve::new(f.one(), f.from_u64(46));
    let mut wrong = n.clone();
    wrong[0] = f.add(wrong[0], f.one());
    assert!(!identity(&f, &wrong, &q, &m, &e));
    let wrong_target = Curve::new(e.a, f.add(e.b, f.one()));
    assert!(!identity(&f, &n, &q, &m, &wrong_target));
    let opposite = poly::scale(&f, &m, f.neg(f.one()));
    assert!(
        identity(&f, &n, &q, &opposite, &e),
        "negative map still has the exact curve identity"
    );
    let mut disagree = 0;
    for [x, y, _, ye] in points() {
        let x = f.from_u64(x);
        let qx = poly::eval(&f, &q, x);
        let actual = f.div(
            f.mul(f.from_u64(y), poly::eval(&f, &opposite, x)),
            f.pow(qx, 3),
        );
        if actual != f.from_u64(ye) {
            disagree += 1;
        }
    }
    assert!(disagree > 16);
    println!("PASS mutated numerator/wrong codomain rejected;negative ordinate still satisfies the exact curve equation but disagrees on{disagree} certificate points,so equation alone does not identify the old H");
}
