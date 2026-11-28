use crate::algebra::{self, Ring, A};
use crate::elliptic::{self, Point};
use crate::field::{F, ONE, ZERO};
use crate::linear;
use crate::polynomial::{self, Polynomial};

const R: [[i32; 3]; 3] = [[1, 5, 2], [0, 5, 0], [2, 1, 1]];
const RINV: [[i32; 3]; 3] = [[1, 5, 5], [0, 5, 0], [5, 6, 1]];

fn apply(matrix: [[i32; 3]; 3], point: [A; 3]) -> [A; 3] {
    matrix.map(|row| {
        row.into_iter()
            .zip(point)
            .fold(point[0].ring.scalar(ZERO), |s, (a, b)| {
                s.plus(b.scale(F::c(a)))
            })
    })
}

fn direct_quartic([x, y, z]: [A; 3]) -> A {
    let g = x
        .power(2)
        .scale(F::c(6))
        .plus(x.times(y).scale(F::c(3)))
        .plus(y.power(2))
        .plus(z.power(2));
    let d = y.times(
        x.power(3)
            .plus(x.times(y.power(2)).scale(F::c(2)))
            .plus(y.power(3).scale(F::c(6))),
    );
    g.power(2).minus(d)
}

fn checked_curve(point: [A; 3], curve: &Polynomial) {
    let zero = point[0].ring.scalar(ZERO);
    assert_eq!(direct_quartic(point), zero);
    assert_eq!(polynomial::evaluate(curve, point), zero);
}

fn checked_deck(point: [A; 3], target: Point) {
    let (x, y) = target.expect("finite3-torsion");
    let [a, b, c] = point;
    assert_eq!(a.minus(b.scale(x)), a.ring.scalar(ZERO));
    let ordinate = c
        .power(2)
        .plus(a.power(2).scale(F::c(6)))
        .plus(a.times(b).scale(F::c(3)))
        .plus(b.power(2));
    assert_eq!(ordinate.minus(b.power(2).scale(y)), a.ring.scalar(ZERO));
    assert!(algebra::unit(b));
}

fn fibre0(target: Point, curve: &Polynomial) -> [A; 3] {
    let (x, y) = target.expect("nonzero3-torsion target");
    let g = F::c(6).times(x.power(2)).plus(F::c(3).times(x)).plus(ONE);
    let ring = Ring::new(2, y.minus(g));
    let point = [ring.scalar(x), ring.scalar(ONE), ring.variable()];
    checked_curve(point, curve);
    checked_deck(point, target);
    point
}

fn fibre3(target: Point, curve: &Polynomial) -> [A; 3] {
    let (x, y) = target.expect("nonzero3-torsion quotient");
    let delta = x.minus(ONE);
    assert_ne!(delta, ZERO, "x=1 is not a3-division root");
    let u = F::c(3).times(y.minus(F::c(4)).over(delta).minus(F::c(4)));
    let w = delta.minus(
        F::c(2)
            .times(u.power(2))
            .plus(F::c(6).times(u))
            .plus(F::c(3)),
    );
    let h = ONE.plus(F::c(3).times(u)).plus(F::c(2).times(u.power(2)));
    let v = w.minus(h).over(F::c(4));
    let (ring, yy, zz) = if v == ZERO {
        assert_eq!(u, ZERO);
        let ring = Ring::new(3, F::c(3));
        (ring, ring.scalar(ZERO), ring.variable())
    } else {
        let ring = Ring::new(3, v);
        let yy = ring.variable();
        (ring, yy, yy.power(2).scale(u.over(v)))
    };
    let uu = yy.times(zz);
    let vv = yy.power(3);
    assert_eq!(uu, ring.scalar(u));
    assert_eq!(vv, ring.scalar(v));
    let ww = vv
        .scale(F::c(4))
        .plus(ring.scalar(ONE))
        .plus(uu.scale(F::c(3)))
        .plus(uu.power(2).scale(F::c(2)));
    let dd = ww
        .plus(uu.power(2).scale(F::c(2)))
        .plus(uu.scale(F::c(6)))
        .plus(ring.scalar(F::c(3)));
    let image_x = dd.plus(ring.scalar(ONE));
    let image_y = uu
        .scale(F::c(5))
        .plus(ring.scalar(F::c(4)))
        .times(dd)
        .plus(ring.scalar(F::c(4)));
    assert_eq!(image_x, ring.scalar(x));
    assert_eq!(image_y, ring.scalar(y));
    let point = [
        ring.scalar(ONE).plus(yy).plus(zz),
        ring.scalar(F::c(5)),
        yy.minus(zz),
    ];
    checked_curve(point, curve);
    point
}

fn infinity_jet(curve: &Polynomial) -> [A; 3] {
    let ring = Ring::new(3, ZERO);
    let t = ring.variable();
    let point = [
        ring.scalar(ONE).plus(t).minus(t.power(2)),
        t.power(2).scale(F::c(2)),
        t.minus(ring.scalar(ONE)),
    ];
    checked_curve(point, curve);
    point
}

fn vanishes(rows: &[Vec<F>], conic: [F; 6]) -> bool {
    rows.iter().all(|row| {
        row.iter()
            .copied()
            .zip(conic)
            .fold(ZERO, |s, (a, b)| s.plus(a.times(b)))
            == ZERO
    })
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub point: Point,
    pub sign: i32,
    pub plus: Option<[F; 6]>,
    pub minus: Option<[F; 6]>,
    pub certified: bool,
}

pub fn controls(curve: &Polynomial) {
    let jet = infinity_jet(curve);
    let y_squared = polynomial::conic([ZERO, ZERO, ZERO, ONE, ZERO, ZERO]);
    assert_eq!(
        polynomial::evaluate(&y_squared, jet),
        jet[0].ring.scalar(ZERO)
    );
    let altered = polynomial::conic([ONE, ZERO, ZERO, ONE, ZERO, ZERO]);
    assert_ne!(
        polynomial::evaluate(&altered, jet),
        jet[0].ring.scalar(ZERO)
    );
    assert_eq!(
        polynomial::evaluate(&y_squared, jet),
        jet[0].ring.scalar(ZERO)
    );
    let t = [[1, 0, 0], [0, 1, 0], [0, 0, 6]];
    let multiply = |a: [[i32; 3]; 3], b: [[i32; 3]; 3]| -> [[i32; 3]; 3] {
        std::array::from_fn(|i| {
            std::array::from_fn(|j| (0..3).map(|k| a[i][k] * b[k][j]).sum::<i32>().rem_euclid(7))
        })
    };
    assert_eq!(multiply(t, multiply(R, t)), RINV);
    assert_eq!(multiply(R, RINV), [[4, 0, 0], [0, 4, 0], [0, 0, 4]]);
    println!("GEOMETRIC_CONTROLS\tactual_quartic_jet=0\tY2_jet_positive\tX2_added_jet_rejects\tpristine_restored\tTRT=Rinv_projectively\tPASS");
}

pub fn cell(point: Point, sign: i32, curve: &Polynomial) -> Cell {
    assert!(point.is_some() && (sign == 1 || sign == -1));
    let target = if sign == 1 {
        elliptic::frobenius(point)
    } else {
        elliptic::negative(elliptic::frobenius(point))
    };
    let f0 = fibre0(point, curve);
    let f1 = apply(RINV, f0);
    checked_curve(f1, curve);
    checked_deck(apply(R, f1), point);
    let f3 = fibre3(target, curve);
    let jet = infinity_jet(curve);
    let mut positive = polynomial::constraints(f0);
    positive.extend(polynomial::constraints(f3));
    let mut negative = polynomial::constraints(f1);
    negative.extend(polynomial::constraints(jet));
    assert_eq!(positive.len(), 5);
    assert_eq!(negative.len(), 5);
    let (positive_rank, plus) = linear::unique_conic(positive.clone());
    let (negative_rank, minus) = linear::unique_conic(negative.clone());
    println!("CONIC_CELL\tP={:?}\tsign={}\tf3_target={:?}\tpositive_rank={}\tnegative_rank={}\tqplus={:?}\tqminus={:?}\tfibre0_constant={:?}\tfibre3_constant={:?}", point, sign, target, positive_rank, negative_rank, plus, minus, f0[0].ring.constant, f3[0].ring.constant);
    let mut result = Cell {
        point,
        sign,
        plus,
        minus,
        certified: false,
    };
    let (Some(qp), Some(qm)) = (plus, minus) else {
        println!(
            "CELL_INCONCLUSIVE\tP={:?}\tsign={}\tnonunique_conic\tretained",
            point, sign
        );
        return result;
    };
    assert!(vanishes(&positive, qp) && vanishes(&negative, qm));
    let mutation = (0..6)
        .find(|&i| negative.iter().any(|r| r[i] != ZERO))
        .expect("nonzero constraint");
    let mut changed = qm;
    changed[mutation] = changed[mutation].plus(ONE);
    assert!(!vanishes(&negative, changed));
    assert!(vanishes(&negative, qm));
    let plus_poly = polynomial::conic(qp);
    let minus_poly = polynomial::conic(qm);
    let at_origin = [
        f0[0].ring.scalar(ONE),
        f0[0].ring.scalar(ZERO),
        f0[0].ring.scalar(F::c(6)),
    ];
    let checks = [
        algebra::unit(polynomial::evaluate(&minus_poly, f0)),
        algebra::unit(polynomial::evaluate(&minus_poly, f3)),
        algebra::unit(polynomial::evaluate(&plus_poly, f1)),
        algebra::unit(polynomial::evaluate(&plus_poly, at_origin)),
    ];
    let (coprime, common) = polynomial::common_divisor(&plus_poly, &minus_poly, curve);
    let degree = common.as_ref().map(|c| 28 - c.pivots.len());
    result.certified = checks.into_iter().all(|v| v) && degree == Some(3);
    println!("DIVISOR_CHECKS\tP={:?}\tsign={}\topposite_units={:?}\tcommon_degree={:?}\tconic_coefficient_mutation={}\tpristine_restored\tcertified={}", point, sign, checks, degree, mutation, result.certified);
    println!("COPRIME_RANK_CERT\tP={:?}\tsign={}\tpivots={:?}\toriginal_rows={:?}\tnonzero_minor={:?}\tnullspace={:?}", point, sign, coprime.pivots, coprime.original_rows, coprime.nonzero_minor, coprime.nullspace);
    if let Some(certificate) = common {
        println!("MACAULAY_RANK_CERT\tP={:?}\tsign={}\tpivots={:?}\toriginal_rows={:?}\tnonzero_minor={:?}\tnullspace={:?}", point, sign, certificate.pivots, certificate.original_rows, certificate.nonzero_minor, certificate.nullspace);
    }
    result
}
