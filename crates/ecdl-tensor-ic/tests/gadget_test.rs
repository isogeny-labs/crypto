//! Port of the Go `gadget_test.go` (package `ecdl`): exhaustive tests of the
//! arithmetic gadgets, the point gadgets and the gadget relation network.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use num_bigint::{BigInt, BigUint};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::field::rand_below;
use ecdl_tensor_ic::gadgets::{bool_int, uint_bits};
use ecdl_tensor_ic::pointgadget::bi64;
use ecdl_tensor_ic::relation::{new_affine_factor_bases, FactorBases};
use ecdl_tensor_ic::{
    gadget_relation_network, gadget_shapes, random_prime_order_curve, relation_network, Arith,
    ArithRef, Builder, Counter, Curve, DFSCounter, FieldPoint, FuncVertex, Network, NetworkStats,
    Operand, Point, PredVertex, Solver, SparseContractor, SupportVertex, Term, Var, Vertex,
    INFINITY,
};

fn bi(x: u64) -> BigUint {
    bi64(x)
}

/// countWith counts a builder's network, optionally fixing a Var to a value.
fn count_with(n: &Network, v: Option<(&Var, &BigUint)>) -> u64 {
    let mut fixed: HashMap<usize, bool> = HashMap::new();
    if let Some((v, val)) = v {
        let bs = uint_bits(val, v.width()).expect("value does not fit");
        for (i, e) in v.edges().into_iter().enumerate() {
            fixed.insert(e, bs[i]);
        }
    }
    DFSCounter.count(n, &fixed).unwrap()
}

/// randomNetwork builds a small network mixing all three vertex shapes.
/// (Local copy of the helper in ecdl_test.rs; Go shares it in-package.)
fn random_network(rng: &mut StdRng) -> Network {
    let mut n = Network {
        num_edges: 10,
        ..Default::default()
    };
    let num_edges = n.num_edges;
    let pick = |rng: &mut StdRng, k: usize| -> Vec<usize> {
        let mut perm: Vec<usize> = (0..num_edges).collect();
        perm.shuffle(rng);
        perm.truncate(k);
        perm
    };
    for _ in 0..2 {
        let e = pick(rng, 3);
        let mut rows: Vec<Vec<bool>> = Vec::new();
        for r in 0..8usize {
            if rng.gen_range(0..2) == 0 {
                rows.push(vec![r & 1 == 1, r & 2 == 2, r & 4 == 4]);
            }
        }
        n.vertices
            .push(Vertex::Support(SupportVertex::new(e, rows)));
    }
    // A function whose inputs are already bound by the support vertices.
    let mut bound = n.vertices[0].edges();
    bound.extend(n.vertices[1].edges());
    let input = vec![bound[0], bound[4]];
    let mut out: Vec<usize> = Vec::new();
    let mut perm: Vec<usize> = (0..n.num_edges).collect();
    perm.shuffle(rng);
    for e in perm {
        if e != input[0] && e != input[1] && out.len() < 2 {
            out.push(e);
        }
    }
    let o0 = out[0];
    n.vertices.push(Vertex::Func(FuncVertex {
        in_: input,
        out,
        f: Rc::new(|x: &[bool]| {
            if !x[0] || x[1] {
                Some(vec![x[0] != x[1], x[0] && x[1]])
            } else {
                None
            }
        }),
    }));
    let pe = vec![bound[1], o0];
    n.vertices.push(Vertex::Pred(PredVertex {
        e: pe,
        p: Rc::new(|v: &[bool]| v[0] || v[1]),
    }));
    n
}

fn exhaustive_builder(exhaustive: bool) -> Builder {
    // Struct-update syntax (`..Default::default()`) is not usable outside the
    // crate because `Builder` has crate-private fields.
    let mut b = Builder::new();
    b.exhaustive = exhaustive;
    b
}

// In exhaustive mode every hint enumerates all values of the Vars it
// writes, so a count of exactly 1 shows the vertices alone pin down every
// witness, and fixing the output shows that witness is the right one.
// Go: TestMulCheckExhaustive
#[test]
fn mul_check_exhaustive() {
    for p in [11u64, 13] {
        let ar = Arith::new(&bi(p));
        for a in 0..p {
            for bv in 0..p {
                for const_a in [false, true] {
                    let mut b = exhaustive_builder(true);
                    let x = b.field_var(&ar, "a");
                    let y = b.field_var(&ar, "b");
                    let c = b.field_var(&ar, "c");
                    let xo;
                    if const_a {
                        xo = Operand::Const(bi(a));
                        b.support(std::slice::from_ref(&y), &[vec![bi(bv)]]);
                    } else {
                        xo = Operand::Var(x.clone());
                        b.support(&[x.clone(), y.clone()], &[vec![bi(a), bi(bv)]]);
                    }
                    b.mul_check(&ar, &xo, &Operand::Var(y.clone()), &c, true);
                    let n = b.finalize();
                    let k = count_with(&n, None);
                    assert_eq!(
                        k, 1,
                        "p={} a={} b={} const={}: {} assignments, want 1",
                        p, a, bv, const_a, k
                    );
                    let k = count_with(&n, Some((&c, &bi(a * bv % p))));
                    assert_eq!(k, 1, "p={} a={} b={}: product is not a*b mod p", p, a, bv);
                }
            }
        }
    }
}

// At b = 7 the transform has L1 != L2 and a stage-1 vertex with no inputs,
// paths the 4-bit sweep above does not reach.
// Go: TestMulCheckExhaustiveUnbalanced
#[test]
fn mul_check_exhaustive_unbalanced() {
    const P: u64 = 101;
    let ar = Arith::new(&bi(P));
    assert!(
        !(ar.l1 == ar.l2 || ar.n > ar.l2 * (ar.l1 - 1)),
        "parameters do not exercise the unbalanced path: {:?}",
        ar
    );
    let mut rng = StdRng::seed_from_u64(16);
    for _ in 0..5 {
        let a = rng.gen_range(0..P);
        let bv = rng.gen_range(0..P);
        let mut b = exhaustive_builder(true);
        let x = b.field_var(&ar, "a");
        let y = b.field_var(&ar, "b");
        let c = b.field_var(&ar, "c");
        b.support(&[x.clone(), y.clone()], &[vec![bi(a), bi(bv)]]);
        b.mul_check(
            &ar,
            &Operand::Var(x.clone()),
            &Operand::Var(y.clone()),
            &c,
            true,
        );
        let n = b.finalize();
        let k = count_with(&n, None);
        assert_eq!(k, 1, "a={} b={}: {} assignments, want 1", a, bv, k);
        assert_eq!(
            count_with(&n, Some((&c, &bi(a * bv % P)))),
            1,
            "a={} b={}: product is not a*b mod p",
            a,
            bv
        );
    }
}

// Go: TestLinAndRangeCheckExhaustive
#[test]
fn lin_and_range_check_exhaustive() {
    const P: i64 = 13;
    let ar = Arith::new(&bi(P as u64));
    for x in 0..P as u64 {
        for y in 0..P as u64 {
            let mut b = exhaustive_builder(true);
            let xv = b.field_var(&ar, "x");
            let yv = b.field_var(&ar, "y");
            let out = b.field_var(&ar, "out");
            b.support(&[xv.clone(), yv.clone()], &[vec![bi(x), bi(y)]]);
            // out = -(2x - y + 5) mod p, and out must be nonzero.
            b.lin_check(
                &ar,
                Some(&out),
                true,
                &[
                    Term {
                        coef: 2,
                        v: xv.clone(),
                    },
                    Term {
                        coef: -1,
                        v: yv.clone(),
                    },
                ],
                &BigInt::from(5),
            );
            b.range_check(&ar, &out, &ar.p, true);
            let n = b.finalize();
            let want = ((-(2 * x as i64 - y as i64 + 5)) % P + P) % P;
            let want_count: u64 = if want == 0 { 0 } else { 1 };
            let k = count_with(&n, None);
            assert_eq!(
                k, want_count,
                "x={} y={}: {} assignments, want {}",
                x, y, k, want_count
            );
            if want_count == 1 {
                assert_eq!(
                    count_with(&n, Some((&out, &bi(want as u64)))),
                    1,
                    "x={} y={}: wrong output",
                    x,
                    y
                );
            }
        }
    }
}

// Go: TestAddGenericExhaustive
#[test]
fn add_generic_exhaustive() {
    let mut rng = StdRng::seed_from_u64(11);
    let c = random_prime_order_curve(&mut rng, 4).unwrap();
    let ar = Arith::new(&bi(c.p));
    let pts = c.points();
    for &p1 in &pts {
        for &p2 in &pts {
            // Exhaustive on every same-x pair and a sample of the rest;
            // the others still check the sum with computed witnesses.
            let mut b = exhaustive_builder(p1.x == p2.x || rng.gen_range(0..6) == 0);
            let pp1 = FieldPoint {
                x: b.field_var(&ar, "x1"),
                y: b.field_var(&ar, "y1"),
                inf: None,
            };
            let pp2 = FieldPoint {
                x: b.field_var(&ar, "x2"),
                y: b.field_var(&ar, "y2"),
                inf: None,
            };
            b.support(&[pp1.x.clone(), pp1.y.clone()], &[vec![bi(p1.x), bi(p1.y)]]);
            b.support(&[pp2.x.clone(), pp2.y.clone()], &[vec![bi(p2.x), bi(p2.y)]]);
            let p3 = b.add_generic(&ar, &pp1, &pp2);
            let n = b.finalize();
            let generic = p1.x != p2.x;
            let k = count_with(&n, None);
            assert!(
                !((generic && k != 1) || (!generic && k != 0)),
                "{:?} + {:?}: {} assignments (generic={})",
                p1,
                p2,
                k,
                generic
            );
            if generic {
                let s = c.add(p1, p2);
                assert!(
                    count_with(&n, Some((&p3.x, &bi(s.x)))) == 1
                        && count_with(&n, Some((&p3.y, &bi(s.y)))) == 1,
                    "{:?} + {:?}: wrong sum",
                    p1,
                    p2
                );
            }
        }
    }
}

// Go: TestValidityCheck
#[test]
fn validity_check() {
    let mut rng = StdRng::seed_from_u64(12);
    for bitsz in [4usize, 6] {
        let c = random_prime_order_curve(&mut rng, bitsz).unwrap();
        let ar = Arith::new(&bi(c.p));
        let mut b = exhaustive_builder(bitsz == 4);
        let pt = FieldPoint {
            x: b.field_var(&ar, "x"),
            y: b.field_var(&ar, "y"),
            inf: None,
        };
        // Every pair of b-bit values, including non-canonical ones >= p.
        let mut rows: Vec<Vec<BigUint>> = Vec::new();
        let lim = 1u64 << ar.bits;
        for x in 0..lim {
            for y in 0..lim {
                rows.push(vec![bi(x), bi(y)]);
            }
        }
        b.support(&[pt.x.clone(), pt.y.clone()], &rows);
        b.validity_check(&ar, &c, &pt);
        let k = count_with(&b.finalize(), None);
        assert_eq!(
            k,
            c.order - 1,
            "b={}: {} valid encodings, want {} affine points",
            bitsz,
            k,
            c.order - 1
        );
    }
}

/// pointVars defines a FieldPoint (with an infinity flag) holding pt.
fn point_vars(b: &mut Builder, ar: &ArithRef, pt: Point) -> FieldPoint {
    let fp = FieldPoint {
        x: b.field_var(ar, "x"),
        y: b.field_var(ar, "y"),
        inf: Some(b.new_bit("inf")),
    };
    b.support(
        &[fp.x.clone(), fp.y.clone(), fp.inf.clone().unwrap()],
        &[vec![bi(pt.x), bi(pt.y), bool_int(pt.inf)]],
    );
    fp
}

/// Checks the count of a complete-addition network and the sum it pins down.
fn check_complete_add(n: &Network, p3: &FieldPoint, p1: Point, p2: Point, want: Point) {
    let k = count_with(n, None);
    assert_eq!(k, 1, "{:?} + {:?}: {} assignments, want 1", p1, p2, k);
    assert!(
        count_with(n, Some((p3.inf.as_ref().unwrap(), &bool_int(want.inf)))) == 1
            && count_with(n, Some((&p3.x, &bi(want.x)))) == 1
            && count_with(n, Some((&p3.y, &bi(want.y)))) == 1,
        "{:?} + {:?}: wrong sum, want {:?}",
        p1,
        p2,
        want
    );
}

// Go: TestCompleteAddExhaustive
#[test]
fn complete_add_exhaustive() {
    let mut rng = StdRng::seed_from_u64(17);
    let c = random_prime_order_curve(&mut rng, 4).unwrap();
    let ar = Arith::new(&bi(c.p));
    let mut pts = c.points();
    pts.push(INFINITY);
    let mut special = 0;
    for &p1 in &pts {
        for &p2 in &pts {
            // Exhaustive on every pair with O or equal x (doubling, P + -P)
            // and a sample of generic pairs. y = 0 cannot occur on a
            // prime-order curve; see TestCompleteAddTwoTorsion.
            let sp = p1.inf || p2.inf || p1.x == p2.x;
            if sp {
                special += 1;
            }
            let mut b = exhaustive_builder(sp || rng.gen_range(0..8) == 0);
            let v1 = point_vars(&mut b, &ar, p1);
            let v2 = point_vars(&mut b, &ar, p2);
            let p3 = b.complete_add(&ar, &c, &v1, &v2);
            let n = b.finalize();
            check_complete_add(&n, &p3, p1, p2, c.add(p1, p2));
        }
    }
    println!(
        "p={}: {} pairs, {} special",
        c.p,
        pts.len() * pts.len(),
        special
    );
}

// Prime-order curves have no points with y = 0 (they would have order 2), so
// the y1 = y2 = 0 branch of CompleteAdd needs a curve with 2-torsion:
// y^2 = x^3 + x over F_13 has three such points.
// Go: TestCompleteAddTwoTorsion
#[test]
fn complete_add_two_torsion() {
    let c = Curve {
        p: 13,
        a: 1,
        b: 0,
        ..Default::default()
    };
    let ar = Arith::new(&bi(c.p));
    let two_tors: Vec<Point> = c.points().into_iter().filter(|pt| pt.y == 0).collect();
    assert!(!two_tors.is_empty(), "test curve has no points with y = 0");
    let mut pts = c.points();
    pts.push(INFINITY);
    let mut pairs = 0;
    for &p1 in &pts {
        for &p2 in &pts {
            if p1.inf || p2.inf || (p1.y != 0 && p2.y != 0) {
                continue;
            }
            pairs += 1;
            let mut b = exhaustive_builder(true);
            let v1 = point_vars(&mut b, &ar, p1);
            let v2 = point_vars(&mut b, &ar, p2);
            let p3 = b.complete_add(&ar, &c, &v1, &v2);
            let n = b.finalize();
            check_complete_add(&n, &p3, p1, p2, c.add(p1, p2));
        }
    }
    println!(
        "{} points with y = 0, {} pairs checked",
        two_tors.len(),
        pairs
    );
}

// Exhaustive over every (x, y) encoding: validity plus membership must
// accept exactly the points of F_i.
// Go: TestMembershipExhaustive
#[test]
fn membership_exhaustive() {
    let mut rng = StdRng::seed_from_u64(18);
    let c = random_prime_order_curve(&mut rng, 4).unwrap();
    let ar = Arith::new(&bi(c.p));
    let fb = new_affine_factor_bases(&c, 2, &mut rng);
    for i in 0..fb.sets.len() {
        let mut b = exhaustive_builder(true);
        let pt = FieldPoint {
            x: b.field_var(&ar, "x"),
            y: b.field_var(&ar, "y"),
            inf: None,
        };
        b.hint_nil(&[], &[pt.x.clone(), pt.y.clone()]);
        b.validity_check(&ar, &c, &pt);
        b.membership_check(&ar, &fb, i, &pt);
        let k = count_with(&b.finalize(), None);
        assert_eq!(
            k,
            fb.sets[i].len() as u64,
            "F_{}: gadget accepts {} encodings, |F_i| = {}",
            i,
            k,
            fb.sets[i].len()
        );
    }
}

/// specialTargets enumerates all decompositions and returns the targets that
/// some decomposition reaches through a non-generic addition (an O input or
/// output, or equal x), plus whether O itself is reachable.
fn special_targets(c: &Curve, fb: &FactorBases) -> Vec<Point> {
    fn walk(
        c: &Curve,
        fb: &FactorBases,
        seen: &mut HashSet<Point>,
        special: &mut Vec<Point>,
        i: usize,
        s: Point,
        sp: bool,
    ) {
        if i == fb.sets.len() {
            if sp && !seen.contains(&s) {
                seen.insert(s);
                special.push(s);
            }
            return;
        }
        for &a in &fb.sets[i] {
            let n = c.add(s, a);
            walk(
                c,
                fb,
                seen,
                special,
                i + 1,
                n,
                sp || (i > 0 && (s.inf || s.x == a.x)),
            );
        }
    }
    let mut seen = HashSet::new();
    let mut special = Vec::new();
    walk(c, fb, &mut seen, &mut special, 0, INFINITY, false);
    special
}

// The gadget network must count exactly what the whole-point network counts
// for the same factor bases, including decompositions through doubling,
// P + (-P) and O.
// Go: TestGadgetRelationCounts
#[test]
fn gadget_relation_counts() {
    let mut rng = StdRng::seed_from_u64(13);
    let c = random_prime_order_curve(&mut rng, 7).unwrap();
    let ar = Arith::new(&bi(c.p));
    for m in [2usize, 3] {
        let (fb, special) = {
            let mut tries = 0;
            loop {
                assert!(
                    tries != 50,
                    "m={}: no factor bases with special-case decompositions",
                    m
                );
                let fb = new_affine_factor_bases(&c, m, &mut rng);
                let special = special_targets(&c, &fb);
                if !special.is_empty() {
                    break (fb, special);
                }
                tries += 1;
            }
        };
        check_targets(&c, &ar, &fb, &special, m, &mut rng);
    }
}

/// Checks gadget vs whole-point counts on special targets, O and random targets.
fn check_targets(
    c: &Curve,
    ar: &ArithRef,
    fb: &FactorBases,
    special: &[Point],
    m: usize,
    rng: &mut StdRng,
) {
    let mut targets: Vec<Point> = special.to_vec();
    if targets.len() > 4 {
        targets.truncate(4);
    }
    targets.push(INFINITY);
    for _ in 0..3 {
        targets.push(c.mul(1 + rand_below(rng, c.order - 1), c.g));
    }
    let mut hit_special = 0;
    for (ti, &r) in targets.iter().enumerate() {
        let want = SparseContractor::default()
            .count(&relation_network(c, fb, r).0, &HashMap::new())
            .unwrap();
        let (n, _) = gadget_relation_network(c, ar, fb, r);
        let st = n.stats();
        assert!(st.is_graph, "gadget relation network is not a graph");
        for v in &n.vertices {
            assert!(
                !matches!(v, Vertex::Support(_)),
                "gadget relation network has a support vertex"
            );
        }
        let got = DFSCounter.count(&n, &HashMap::new()).unwrap();
        assert_eq!(
            got, want,
            "m={} R={:?}: gadget network counts {}, whole-point network {}",
            m, r, got, want
        );
        if ti < special.len() && ti < 4 && want > 0 {
            hit_special += 1;
        }
    }
    assert!(
        hit_special != 0,
        "m={}: no special-case target had a decomposition",
        m
    );
    println!(
        "m={}: {} targets agree, {} reached through special cases",
        m,
        targets.len(),
        hit_special
    );
}

// Go: TestDFSCounterMatchesSparse
#[test]
fn dfs_counter_matches_sparse() {
    let mut rng = StdRng::seed_from_u64(14);
    for i in 0..100 {
        let n = random_network(&mut rng);
        let a = SparseContractor::default()
            .count(&n, &HashMap::new())
            .unwrap();
        let b = DFSCounter.count(&n, &HashMap::new()).unwrap();
        assert_eq!(a, b, "network {}: sparse {}, dfs {}", i, a, b);
    }
}

// Go: TestSolveWithGadgets
#[test]
fn solve_with_gadgets() {
    let mut rng = StdRng::seed_from_u64(15);
    for _ in 0..2 {
        let c = random_prime_order_curve(&mut rng, 6).unwrap();
        let xi = rand_below(&mut rng, c.order);
        let mut s = Solver::new(c, 2, Box::new(DFSCounter), rng);
        s.gadgets = true;
        let res = s.solve(c.mul(xi, c.g));
        // Go shares one rng between the test and the solver.
        rng = s.rng;
        let (got, st) = res.unwrap();
        assert_eq!(got, xi, "got {}, want {}", got, xi);
        println!(
            "p={} r={} network: {:?} relations={} {:?}",
            c.p, c.order, st.network, st.relations, st.elapsed
        );
    }
}

// Go: TestGadgetScaling
#[test]
fn gadget_scaling() {
    let sizes = [64usize, 256, 1024, 4096];
    for bitsz in sizes {
        let p: BigUint = (BigUint::from(1u32) << (bitsz - 1)) + 1u32;
        let (mul, add, point) = gadget_shapes(&Arith::new(&p));
        let shapes: [(&str, NetworkStats); 3] = [("mul", mul), ("add", add), ("point", point)];
        for (name, st) in shapes {
            assert!(st.is_graph, "b={} {}: not a graph", bitsz, name);
            let per_b = st.edges as f64 / bitsz as f64;
            let deg_ratio = st.max_degree as f64 / (bitsz as f64).powf(0.75);
            println!(
                "b={:5} {}: edges={:7} edges/b={:5.1} maxdeg={:5} maxdeg/b^0.75={:4.1}",
                bitsz, name, st.edges, per_b, st.max_degree, deg_ratio
            );
            assert!(
                !(per_b > 400.0 || deg_ratio > 40.0),
                "b={} {}: shape out of bounds",
                bitsz,
                name
            );
        }
    }
}
