//! CT20: independently construct four genus-two curves by gluing the CT19 degree-seven
//! elliptic image 2-torsion. Producer regression only; [pcc:unverified].
//! cargo test --manifest-path isogeny_algos/Cargo.toml --test ct20_f101_gluing
use isogeny_algos::curve::{rhs, Curve, Isogeny};
use isogeny_algos::field::{Field, Zp};
use isogeny_algos::fpr::FpR;
use isogeny_algos::genus2::{cubic_to_short, elliptic_trace, glue, lpoly, LPoly};
use isogeny_algos::kernel::kohel::kohel;

struct Pair {
    left: [u64; 4],
    right: [u64; 4],
    left_target: (u64, u64),
    right_target: (u64, u64),
    raw: [u64; 7],
    scale: Option<u64>,
}

const PAIRS: [Pair; 4] = [
    Pair {
        left: [9, 76, 20, 1],
        right: [77, 86, 71, 1],
        left_target: (5, 39),
        right_target: (13, 42),
        raw: [32, 0, 26, 0, 49, 0, 41],
        scale: None,
    },
    Pair {
        left: [9, 76, 20, 1],
        right: [59, 46, 8, 1],
        left_target: (5, 39),
        right_target: (69, 50),
        raw: [38, 0, 48, 0, 9, 0, 46],
        scale: None,
    },
    Pair {
        left: [72, 100, 35, 1],
        right: [77, 86, 71, 1],
        left_target: (26, 58),
        right_target: (13, 42),
        raw: [87, 0, 15, 0, 13, 0, 21],
        scale: Some(7),
    },
    Pair {
        left: [72, 100, 35, 1],
        right: [59, 46, 8, 1],
        left_target: (26, 58),
        right_target: (69, 50),
        raw: [19, 0, 86, 0, 34, 0, 79],
        scale: Some(21),
    },
];

#[test]
fn ct20_four_f101_gluings_descent_counts_and_component_swap() {
    let f = FpR::from_modulus(101, &[56, 13, 6, 1]);
    let fp = Zp::new(101);
    let mut roots = [f.gen(); 3];
    roots[1] = f.frobenius(roots[0]);
    roots[2] = f.frobenius(roots[1]);
    assert_eq!(f.frobenius(roots[2]), roots[0]);
    for &x in &roots {
        assert_eq!(
            f.add(f.mul(x, f.mul(x, x)), f.add(f.mul(f.from_u64(6), f.sq(x)), f.add(f.mul(f.from_u64(13), x), f.from_u64(56)))),
            f.zero()
        );
    }

    let e1 = Curve::new(f.from_u64(1), f.from_u64(46));
    let e2 = Curve::new(f.from_u64(44), f.from_u64(21));
    let source_x1 = roots.map(|x| f.add(x, f.from_u64(2)));
    let source_x2 = roots.map(|x| f.add(f.div(f.from_u64(56), x), f.from_u64(38)));
    for x in source_x1 { assert_eq!(rhs(&f, &e1, x), f.zero()); }
    for x in source_x2 { assert_eq!(rhs(&f, &e2, x), f.zero()); }

    for pair in PAIRS {
        let kernel1 = pair.left.map(|x| f.from_u64(x)).to_vec();
        let kernel2 = pair.right.map(|x| f.from_u64(x)).to_vec();
        let iso1 = kohel(&f, &e1, &kernel1, 7);
        let iso2 = kohel(&f, &e2, &kernel2, 7);
        assert_eq!((f.project(iso1.cod.a), f.project(iso1.cod.b)),
                   (Some(pair.left_target.0), Some(pair.left_target.1)));
        assert_eq!((f.project(iso2.cod.a), f.project(iso2.cod.b)),
                   (Some(pair.right_target.0), Some(pair.right_target.1)));
        let a = source_x1.map(|x| iso1.eval_x(&f, x).expect("not a 7-kernel point"));
        let b = source_x2.map(|x| iso2.eval_x(&f, x).expect("not a 7-kernel point"));
        for i in 0..3 {
            assert_eq!(rhs(&f, &iso1.cod, a[i]), f.zero());
            assert_eq!(rhs(&f, &iso2.cod, b[i]), f.zero());
            assert_eq!(f.frobenius(a[i]), a[(i + 1) % 3]);
            assert_eq!(f.frobenius(b[i]), b[(i + 1) % 3]);
        }
        let ext_sextic = glue(&f, a, b).expect("HLP gluing degeneracy");
        assert_eq!(ext_sextic.len(), 7);
        let raw: Vec<u64> = ext_sextic.iter()
            .map(|&c| f.project(c).expect("the entire sextic must descend")).collect();
        assert_eq!(raw, pair.raw);
        assert_eq!(lpoly(&fp, &raw), LPoly::product(-7, -11, 101));
        assert_eq!(lpoly(&fp, &raw).jac_order(101), 12_317);
        let leading = *raw.last().unwrap();
        assert_eq!(fp.sqrt(leading).is_some(), pair.scale.is_some());
        if let Some(u) = pair.scale {
            let monic: Vec<u64> = raw.iter().map(|&x| fp.div(x, leading)).collect();
            assert_eq!(*monic.last().unwrap(), 1);
            let b0 = monic[0];
            let a2 = monic[4];
            let a4 = monic[2];
            let direct = vec![b0, a4, a2, 1];
            let reverse = vec![fp.sq(b0), fp.mul(b0, a2), a4, 1];
            // Important: first quotient is the CT19 SECOND elliptic factor.
            let e_direct = cubic_to_short(&fp, &direct);
            let e_reverse = cubic_to_short(&fp, &reverse);
            let uu4 = fp.pow(u, 4);
            let uu6 = fp.pow(u, 6);
            assert_eq!(fp.mul(e_direct.a, uu4), pair.right_target.0);
            assert_eq!(fp.mul(e_direct.b, uu6), pair.right_target.1);
            assert_eq!(fp.mul(e_reverse.a, uu4), pair.left_target.0);
            assert_eq!(fp.mul(e_reverse.b, uu6), pair.left_target.1);
            assert_eq!(101 + 1 - elliptic_trace(&fp, &direct), 113);
            assert_eq!(101 + 1 - elliptic_trace(&fp, &reverse), 109);
        }
        println!("CT20 target {:?}, square-leading={:?}, #J=12317", raw, pair.scale.is_some());
    }
}
