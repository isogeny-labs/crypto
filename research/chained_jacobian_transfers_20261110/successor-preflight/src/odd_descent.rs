//! A bounded even-degree counterexample to dropping the odd coefficient-field
//! hypothesis. Only existing prime/FpR/elliptic engines and a scaling adapter.
use isogeny_algos::curve::{self, Curve, Pt};
use isogeny_algos::field::{Field, Zp};
use isogeny_algos::fpr::FpR;

#[test]
fn quadratic_twist_sign_is_a_real_descent_obstruction() {
    let p = Zp::new(101);
    let source = Curve { a: 1, b: 46 };
    let target = Curve { a: 4, b: 65 };
    let points = |c: &Curve<u64>| {
        let mut out = vec![Pt::Inf];
        for x in 0..101 {
            for y in 0..101 {
                let q = Pt::Aff(x, y);
                if curve::on_curve(&p, c, &q) {
                    out.push(q);
                }
            }
        }
        out
    };
    let all = points(&source);
    assert_eq!(all.len(), 109);
    assert_eq!(points(&target).len(), 95);
    let f = FpR::from_modulus(101, &[99, 0, 1]);
    let s = f.gen();
    assert_eq!(f.sq(s), f.from_u64(2));
    assert_eq!(f.frobenius(s), f.neg(s));
    let target2 = Curve {
        a: f.from_u64(4),
        b: f.from_u64(65),
    };
    let map = |q: &Pt<u64>| match q {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(
            f.mul(f.from_u64(2), f.from_u64(*x)),
            f.mul(f.mul(f.from_u64(2), s), f.from_u64(*y)),
        ),
    };
    let frob = |q| match q {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(f.frobenius(x), f.frobenius(y)),
    };
    let mut images = vec![];
    for q in &all {
        let image = map(q);
        assert!(curve::on_curve(&f, &target2, &image));
        assert_eq!(frob(image), curve::neg(&f, &image));
        assert_eq!(frob(frob(image)), image);
        assert!(!images.contains(&image));
        if *q != Pt::Inf {
            assert_ne!(frob(image), image);
        }
        images.push(image);
    }
    for (i, a) in all.iter().enumerate() {
        for (j, b) in all.iter().enumerate() {
            let sum = curve::padd(&p, &source, a, b);
            assert_eq!(map(&sum), curve::padd(&f, &target2, &images[i], &images[j]));
        }
    }
    println!("PASS exact even-degree counterexample:F101 E(1,46)order109 and quadratic twist(4,65)order95;explicit degree1 scaling overF101² preserves all109² additions,but ALL108 nonzero rational-source images have negative Frobenius,none rational;Frob² fixes every image. Stable trivial kernel/Aut±1 do not remove the sign over an EVEN coefficient extension. This does not contradict cubic-field descent or assert a new DLP advantage.");
}
