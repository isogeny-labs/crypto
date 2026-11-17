//! Fixed inputs for the written geometric distinction argument. Existing
//! prime-field elliptic arithmetic only; no computation of an endomorphism ring.
use isogeny_algos::curve::{self, Curve, Pt};
use isogeny_algos::field::{is_prime, Zp};
use std::collections::HashSet;

#[test]
fn six_elliptic_models_have_the_declared_ordinary_traces_and_distinct_j() {
    let f = Zp::new(101);
    let models = [
        [(1, 46, 12), (26, 58, 87), (59, 27, 47)],
        [(44, 21, 58), (69, 50, 92), (23, 7, 89)],
    ];
    for (component, family) in models.iter().enumerate() {
        let order = [109_u64, 113][component];
        assert!(is_prime(order));
        let trace = 102 - i64::try_from(order).unwrap();
        assert_eq!(trace, [-7, -11][component]);
        assert_ne!(trace % 101, 0);
        let discriminant = trace * trace - 4 * 101;
        assert_eq!(discriminant, [-355, -283][component]);
        let mut invariants = HashSet::new();
        for &(a, b, j) in family {
            let e = Curve::new(a, b);
            assert!(curve::is_smooth(&f, &e));
            let mut count = 1_u64;
            for x in 0..101 {
                for y in 0..101 {
                    if curve::on_curve(&f, &e, &Pt::Aff(x, y)) {
                        count += 1;
                    }
                }
            }
            assert_eq!(count, order);
            assert_eq!(curve::jinv(&f, &e), j);
            assert!(invariants.insert(j));
        }
        assert_eq!(invariants.len(), 3);
        println!("PASS existing Rust elliptic inputs component{component}:all3 exact curve counts{order}/ordinary trace{trace}/discriminant{discriminant}/three distinct declared j-invariants;geometric End algebra and actual Jacobian embedding/uniqueness argument remain written");
    }
    assert_eq!(355 % 25, 5);
    assert_ne!(283 % 5, 0);
    println!("PASS discriminant square-class control:v5(355)=1,v5(283)=0,so355/283 is not a rational square. No endomorphism ring,geometric Hom or Jacobian isomorphism computation claimed by this input test");
}
