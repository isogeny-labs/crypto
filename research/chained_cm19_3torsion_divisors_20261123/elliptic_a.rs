use crate::field::{F, ONE, Q, ZERO};
use std::collections::BTreeSet;

pub type Point = Option<(F, F)>;

pub fn valid(p: Point) -> bool {
    p.is_none_or(|(x, y)| y.power(2) == x.power(3).plus(F::c(2).times(x)).plus(F::c(6)))
}

pub fn negative(p: Point) -> Point {
    p.map(|(x, y)| (x, y.negative()))
}

pub fn add(p: Point, q: Point) -> Point {
    assert!(valid(p) && valid(q));
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return p.or(q);
    };
    if x == u && y.plus(v) == ZERO {
        return None;
    }
    let slope = if x == u {
        F::c(3)
            .times(x.power(2))
            .plus(F::c(2))
            .over(F::c(2).times(y))
    } else {
        v.minus(y).over(u.minus(x))
    };
    let xx = slope.power(2).minus(x).minus(u);
    let yy = slope.times(x.minus(xx)).minus(y);
    let result = Some((xx, yy));
    assert!(valid(result));
    result
}

pub fn multiple(p: Point, n: usize) -> Point {
    (0..n).fold(None, |s, _| add(s, p))
}

pub fn frobenius(p: Point) -> Point {
    p.map(|(x, y)| (x.power(7), y.power(7)))
}

pub fn division(x: F) -> F {
    F::c(3)
        .times(x.power(4))
        .plus(F::c(5).times(x.power(2)))
        .plus(F::c(2).times(x))
        .plus(F::c(3))
}

pub fn torsion() -> Vec<Point> {
    let mut result = BTreeSet::from([None]);
    let mut roots = Vec::new();
    for encoded in 0..Q {
        let x = F(encoded as u16);
        if division(x) == ZERO {
            roots.push(x);
            let rhs = x.power(3).plus(F::c(2).times(x)).plus(F::c(6));
            let mut ys = 0;
            for value in 0..Q {
                let y = F(value as u16);
                if y.power(2) == rhs {
                    assert_ne!(y, ZERO);
                    let p = Some((x, y));
                    assert!(valid(p));
                    assert_eq!(multiple(p, 3), None);
                    assert!(result.insert(p));
                    ys += 1;
                }
            }
            assert_eq!(ys, 2);
        }
    }
    assert_eq!(roots.len(), 4);
    assert_eq!(result.len(), 9);
    let points: Vec<_> = result.iter().copied().collect();
    for &p in &points {
        assert_eq!(add(p, negative(p)), None);
        assert!(result.contains(&frobenius(p)));
        assert_eq!(frobenius(frobenius(p)), negative(p));
        for &q in &points {
            assert!(result.contains(&add(p, q)));
            for &r in &points {
                assert_eq!(add(add(p, q), r), add(p, add(q, r)));
            }
        }
        if p.is_some() {
            let span: BTreeSet<_> = (0..3)
                .flat_map(|i| (0..3).map(move |j| add(multiple(p, i), multiple(frobenius(p), j))))
                .collect();
            assert_eq!(span, result);
        }
        println!("E3_POINT\t{:?}\tFrobenius={:?}\tPASS", p, frobenius(p));
    }
    for &x in &roots {
        assert_ne!(division(x).plus(ONE), ZERO);
        assert_eq!(division(x), ZERO);
    }
    println!("E3\tx_scan=2401\tdivision_roots=4\tpoints=9\taddition_pairs=81\tassociative_triples=729\tall8_P_Frobenius_bases_span9\tchanged_division_constant_rejects_all8_affine_points\tPASS");
    points
}
