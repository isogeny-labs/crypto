//! Fixed-cofactor extension (Rust only; there is no Go original). Each test
//! names the identity it checks; the design and its plan are in
//! research/ecdl_tensor_ic_cofactor_20261106/DESIGN.md.

use std::collections::HashMap;

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;

use ecdl_tensor_ic::field::rand_below;
use ecdl_tensor_ic::gadgets::{bool_int, uint_bits};
use ecdl_tensor_ic::pointgadget::bi64;
use ecdl_tensor_ic::{
    cofactor_factor_bases, cofactor_relation_network, cofactor_shape,
    gadget_cofactor_relation_network, is_prime, random_cofactor_curve, relation_network, Arith,
    Builder, CofactorCurve, Counter, DFSCounter, FieldPoint, Network, Point, Solver,
    SparseContractor, Var, Vertex, INFINITY,
};

fn count_with(n: &Network, fixed_vars: &[(&Var, BigUint)]) -> u64 {
    let mut fixed: HashMap<usize, bool> = HashMap::new();
    for (v, val) in fixed_vars {
        let bs = uint_bits(val, v.width()).expect("value does not fit");
        for (i, e) in v.edges().into_iter().enumerate() {
            fixed.insert(e, bs[i]);
        }
    }
    DFSCounter.count(n, &fixed).unwrap()
}

fn sparse_count(n: &Network) -> u64 {
    SparseContractor::default()
        .count(n, &HashMap::new())
        .unwrap()
}

fn targets(cc: &CofactorCurve, rng: &mut StdRng, k: usize) -> Vec<Point> {
    let c = cc.curve;
    let mut t = vec![INFINITY, c.g];
    for _ in 0..k {
        t.push(c.mul(1 + rand_below(rng, c.order - 1), c.g));
    }
    t
}

// Identity: #E = c·r with r prime, r > c; G = <g> has order r; [c·r]S = O
// for every S ∈ E.
#[test]
fn cofactor_curves_have_the_requested_order() {
    let mut rng = StdRng::seed_from_u64(1);
    for c in [2u64, 4, 8] {
        for bits in [8usize, 10, 12] {
            let cc = random_cofactor_curve(&mut rng, bits, c).unwrap();
            let pts = cc.curve.points();
            assert_eq!(pts.len() as u64 + 1, cc.group_order(), "c={c} bits={bits}");
            assert!(is_prime(cc.curve.order) && cc.curve.order > c);
            assert!(!cc.curve.g.inf && cc.in_subgroup(cc.curve.g));
            for &s in &pts {
                assert!(cc.curve.mul(cc.group_order(), s).inf);
            }
        }
    }
    assert!(random_cofactor_curve(&mut rng, 8, 1).is_err());
}

// Identity: every A ∈ G \ {O} has exactly c preimages under [c]; a point
// outside G has none.
#[test]
fn projection_has_exactly_c_preimages() {
    let mut rng = StdRng::seed_from_u64(2);
    for c in [2u64, 4] {
        let cc = random_cofactor_curve(&mut rng, 8, c).unwrap();
        let pts = cc.curve.points();
        let mut in_g = 0;
        for &a in &pts {
            let k = pts.iter().filter(|&&s| cc.project(s) == a).count() as u64;
            if cc.in_subgroup(a) {
                assert_eq!(k, c, "A={a:?}");
                in_g += 1;
            } else {
                assert_eq!(k, 0, "A={a:?} is outside G");
            }
        }
        assert_eq!(in_g + 1, cc.curve.order);
    }
}

// Identity: the free-source whole-point network counts exactly c^m times the
// subgroup network over the same factor bases, for every target.
#[test]
fn whole_point_count_identity() {
    let mut rng = StdRng::seed_from_u64(3);
    for c in [2u64, 4] {
        let cc = random_cofactor_curve(&mut rng, 9, c).unwrap();
        for m in [2usize, 3] {
            let fb = cofactor_factor_bases(&cc, m, &mut rng, false);
            for set in &fb.sets {
                assert!(set.iter().all(|&pt| cc.in_subgroup(pt) && !pt.inf));
            }
            let mut nonzero = 0;
            for r in targets(&cc, &mut rng, 4) {
                let want = sparse_count(&relation_network(&cc.curve, &fb, r).0);
                let (n, _, _) = cofactor_relation_network(&cc, &fb, r);
                assert!(n.stats().is_graph);
                let got = sparse_count(&n);
                assert_eq!(got, c.pow(m as u32) * want, "c={c} m={m} R={r:?}");
                if want > 0 {
                    nonzero += 1;
                }
            }
            assert!(nonzero > 0, "c={c} m={m}: every target had count 0");
        }
    }
}

fn source_vars(b: &mut Builder, ar: &ecdl_tensor_ic::ArithRef, s: Point) -> FieldPoint {
    let fp = FieldPoint {
        x: b.field_var(ar, "sx"),
        y: b.field_var(ar, "sy"),
        inf: None,
    };
    b.support(&[fp.x.clone(), fp.y.clone()], &[vec![bi64(s.x), bi64(s.y)]]);
    fp
}

// Exhaustive: for every affine S, [k]S has exactly one assignment, and it is
// Curve::mul(k, S) (including O when S is k-torsion).
#[test]
fn scalar_mul_const_exhaustive() {
    let mut rng = StdRng::seed_from_u64(4);
    for (c, bits) in [(2u64, 5usize), (4, 6)] {
        let cc = random_cofactor_curve(&mut rng, bits, c).unwrap();
        let ar = Arith::new(&bi64(cc.curve.p));
        for k in [2u64, 3, c] {
            for s in cc.curve.points() {
                let mut b = Builder::new();
                b.exhaustive = true;
                let sv = source_vars(&mut b, &ar, s);
                let a = b.scalar_mul_const(&ar, &cc.curve, k, &sv);
                let n = b.finalize();
                let want = cc.curve.mul(k, s);
                assert_eq!(count_with(&n, &[]), 1, "c={c} k={k} S={s:?}");
                let inf = a.inf.as_ref().expect("k >= 2 yields an infinity flag");
                assert_eq!(
                    count_with(
                        &n,
                        &[
                            (inf, bool_int(want.inf)),
                            (&a.x, bi64(want.x)),
                            (&a.y, bi64(want.y)),
                        ]
                    ),
                    1,
                    "c={c} k={k} S={s:?}: [k]S is not {want:?}"
                );
            }
        }
    }
}

// Identity: the gadget cofactor network counts exactly what the whole-point
// cofactor network counts, with no support vertex.
#[test]
fn gadget_cofactor_counts_match_whole_point() {
    let mut rng = StdRng::seed_from_u64(5);
    let cc = random_cofactor_curve(&mut rng, 6, 2).unwrap();
    let ar = Arith::new(&bi64(cc.curve.p));
    let fb = cofactor_factor_bases(&cc, 2, &mut rng, true);
    let mut nonzero = 0;
    for r in targets(&cc, &mut rng, 3) {
        let want = sparse_count(&cofactor_relation_network(&cc, &fb, r).0);
        let (n, _, _) = gadget_cofactor_relation_network(&cc, &ar, &fb, r);
        assert!(n.stats().is_graph);
        assert!(n.vertices.iter().all(|v| !matches!(v, Vertex::Support(_))));
        let got = DFSCounter.count(&n, &HashMap::new()).unwrap();
        assert_eq!(got, want, "R={r:?}");
        if want > 0 {
            nonzero += 1;
        }
    }
    assert!(nonzero > 0, "every target had count 0");
}

// End to end: the solver recovers ξ on cofactor curves (whole-point and
// meet-in-the-middle for c = 2, 4; gadgets for c = 2) and verifies it.
#[test]
fn solve_on_cofactor_curves() {
    let mut rng = StdRng::seed_from_u64(6);
    for (c, bits, gadgets, mitm) in [
        (2u64, 10usize, false, false),
        (4, 10, false, false),
        (4, 12, false, true),
        (2, 6, true, false),
    ] {
        let cc = random_cofactor_curve(&mut rng, bits, c).unwrap();
        let xi = rand_below(&mut rng, cc.curve.order);
        let q = cc.curve.mul(xi, cc.curve.g);
        let counter: Box<dyn Counter> = if gadgets {
            Box::new(DFSCounter)
        } else {
            Box::new(SparseContractor::default())
        };
        let mut s = Solver::new(cc.curve, 2, counter, StdRng::seed_from_u64(7));
        s.cofactor = c;
        s.gadgets = gadgets;
        s.mitm = mitm;
        let (got, st) = s.solve(q).unwrap();
        assert_eq!(got, xi, "c={c} bits={bits} gadgets={gadgets} mitm={mitm}");
        assert!(st.relations > 0);
    }
}

// The solver refuses targets outside G and coordinate windows with c > 1.
#[test]
fn solver_rejects_inputs_outside_the_setting() {
    let mut rng = StdRng::seed_from_u64(8);
    let cc = random_cofactor_curve(&mut rng, 9, 4).unwrap();
    let outside = cc
        .curve
        .points()
        .into_iter()
        .find(|&pt| !cc.in_subgroup(pt))
        .unwrap();
    let mut s = Solver::new(
        cc.curve,
        2,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(9),
    );
    s.cofactor = 4;
    let err = s.solve(outside).unwrap_err();
    assert!(err.0.contains("subgroup"), "{err}");
    s.window_bits = 2;
    let err = s.solve(cc.curve.g).unwrap_err();
    assert!(err.0.contains("window"), "{err}");
}

// Shape: with c fixed, the per-slot cofactor clearing keeps O(b) edges and
// max degree O(b^(3/4)), and is a graph.
#[test]
fn cofactor_shape_stays_bounded() {
    for c in [2u64, 4, 8] {
        let mut per_bit = Vec::new();
        for b in [64usize, 256, 1024] {
            let p: BigUint = (BigUint::from(1u32) << (b - 1)) + 1u32;
            let st = cofactor_shape(&Arith::new(&p), c);
            assert!(st.is_graph, "c={c} b={b}");
            let ratio = st.max_degree as f64 / (b as f64).powf(0.75);
            assert!(ratio < 25.0, "c={c} b={b}: degree / b^0.75 = {ratio:.1}");
            per_bit.push(st.edges as f64 / b as f64);
        }
        // Edges per bit level off (they fall as b grows, as for add e/b).
        assert!(per_bit[2] <= per_bit[0], "c={c}: edges/b {per_bit:?}");
        println!("c={c}: edges/b {per_bit:.1?}");
    }
}

// Optimisation guard: listing only the cached preimages of each F_i as
// sources gives exactly the count of listing all of E, for every target.
#[test]
fn cached_preimages_leave_counts_unchanged() {
    let mut rng = StdRng::seed_from_u64(10);
    for c in [2u64, 4] {
        let cc = random_cofactor_curve(&mut rng, 9, c).unwrap();
        let fb = cofactor_factor_bases(&cc, 2, &mut rng, false);
        assert_eq!(fb.preimages.len(), 2);
        for (i, pre) in fb.preimages.iter().enumerate() {
            assert_eq!(pre.len() as u64, c * fb.sets[i].len() as u64);
        }
        let mut full = fb.clone();
        full.preimages.clear();
        for r in targets(&cc, &mut rng, 4) {
            let cached = sparse_count(&cofactor_relation_network(&cc, &fb, r).0);
            let every = sparse_count(&cofactor_relation_network(&cc, &full, r).0);
            assert_eq!(cached, every, "c={c} R={r:?}");
        }
    }
}
