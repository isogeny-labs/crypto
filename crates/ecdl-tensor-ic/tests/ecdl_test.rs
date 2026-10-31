//! Port of the Go `ecdl_test.go` (package `ecdl`): tests for the curve, the
//! generic contraction solvers, linear algebra, the relation network and the
//! end-to-end solver.

use std::collections::HashMap;
use std::rc::Rc;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::field::{add_mod, mul_mod, rand_below};
use ecdl_tensor_ic::relation::new_factor_bases;
use ecdl_tensor_ic::{
    find_witness, is_prime, left_kernel_vector, random_prime_order_curve, relation_network,
    BruteForceCounter, Counter, FuncVertex, Network, Point, PredVertex, Solver, SparseContractor,
    SupportVertex, Vertex, INFINITY,
};

// Go: TestIsPrime
#[test]
fn is_prime_table() {
    let primes: [(u64, bool); 8] = [
        (2, true),
        (3, true),
        (4, false),
        (97, true),
        (561, false),
        (65537, true),
        ((1 << 31) - 1, true),
        (3215031751, false),
    ];
    for (n, want) in primes {
        assert_eq!(
            is_prime(n),
            want,
            "IsPrime({}) = {}, want {}",
            n,
            !want,
            want
        );
    }
}

// Go: TestCurveGroupLaw
#[test]
fn curve_group_law() {
    let mut rng = StdRng::seed_from_u64(1);
    for _ in 0..5 {
        let c = random_prime_order_curve(&mut rng, 12).unwrap();
        assert!(c.on_curve(c.g) && !c.g.inf, "bad generator");
        let got = c.mul(c.order, c.g);
        assert!(got.inf, "[r]G = {:?}, want O", got);
        let (a, b) = (c.mul(17, c.g), c.mul(40, c.g));
        assert!(
            c.add(a, b) == c.mul(57, c.g) && c.add(a, b) == c.add(b, a),
            "addition is not consistent with scalar multiplication"
        );
        assert!(
            c.add(a, a) == c.mul(34, c.g) && c.add(a, c.neg(a)).inf,
            "doubling or negation is wrong"
        );
        for pt in [INFINITY, a, b] {
            let got = c.decode_point(&c.encode_point(pt));
            assert!(got == Some(pt), "encoding round trip failed for {:?}", pt);
        }
    }
}

/// randomNetwork builds a small network mixing all three vertex shapes.
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

// Go: TestSparseContractorMatchesBruteForce
#[test]
fn sparse_contractor_matches_brute_force() {
    let mut rng = StdRng::seed_from_u64(2);
    for i in 0..200 {
        let n = random_network(&mut rng);
        let mut fixed: HashMap<usize, bool> = HashMap::new();
        for e in 0..n.num_edges {
            if rng.gen_range(0..4) == 0 {
                fixed.insert(e, rng.gen_range(0..2) == 0);
            }
        }
        let want = BruteForceCounter.count(&n, &fixed).unwrap();
        let got = SparseContractor::default().count(&n, &fixed).unwrap();
        assert_eq!(
            got, want,
            "network {}: sparse count {}, brute force {}",
            i, got, want
        );
    }
}

// Go: TestFindWitness
#[test]
fn find_witness_test() {
    let mut rng = StdRng::seed_from_u64(3);
    for _ in 0..100 {
        let n = random_network(&mut rng);
        let all: Vec<usize> = (0..n.num_edges).collect();
        let w = find_witness(&mut SparseContractor::default(), &n, &all).unwrap();
        let total = BruteForceCounter.count(&n, &HashMap::new()).unwrap();
        assert_eq!(
            w.is_none(),
            total == 0,
            "witness presence {} but count {}",
            w.is_some(),
            total
        );
        if let Some(w) = w {
            let k = BruteForceCounter.count(&n, &w).unwrap();
            assert_eq!(k, 1, "witness is not a satisfying assignment");
        }
    }
}

// Go: TestLeftKernelVector
#[test]
fn left_kernel_vector_test() {
    let mut rng = StdRng::seed_from_u64(4);
    const R: u64 = 10007;
    for _ in 0..20 {
        let ncols = 1 + rng.gen_range(0..15usize);
        let mut rows: Vec<HashMap<usize, u64>> = vec![HashMap::new(); ncols + 1];
        for j in 0..rows.len() {
            for _ in 0..3 {
                let col = rng.gen_range(0..ncols);
                let v = rng.gen_range(0..R);
                rows[j].insert(col, v);
            }
        }
        let c = left_kernel_vector(&rows, ncols, R).expect("no dependency found");
        let mut nonzero = false;
        let mut sum = vec![0u64; ncols];
        for (j, &cj) in c.iter().enumerate() {
            nonzero = nonzero || cj != 0;
            for (&col, &v) in &rows[j] {
                sum[col] = add_mod(sum[col], mul_mod(cj, v, R), R);
            }
        }
        assert!(nonzero, "zero combination");
        for (col, &s) in sum.iter().enumerate() {
            assert_eq!(s, 0, "column {} sums to {}", col, s);
        }
    }
}

// Go: TestRelationNetworkIsGraph
#[test]
fn relation_network_is_graph() {
    let mut rng = StdRng::seed_from_u64(5);
    let c = random_prime_order_curve(&mut rng, 10).unwrap();
    for m in 2..=4usize {
        let fb = new_factor_bases(&c, m, &mut rng);
        let (net, _) = relation_network(&c, &fb, c.g);
        let st = net.stats();
        let w = c.point_width();
        assert!(
            st.is_graph && st.edges == (2 * m - 1) * w && st.max_degree == 3 * w,
            "m={}: unexpected network shape {:?}",
            m,
            st
        );
    }
}

// Go: TestSolve
#[test]
fn solve() {
    let mut rng = StdRng::seed_from_u64(6);
    // (bits, m, curves)
    let cases: [(usize, usize, usize); 4] = [(10, 2, 6), (12, 2, 3), (10, 3, 3), (12, 3, 2)];
    for (bits, m, curves) in cases {
        for _ in 0..curves {
            let c = random_prime_order_curve(&mut rng, bits).unwrap();
            let xi = rand_below(&mut rng, c.order);
            let q: Point = c.mul(xi, c.g);
            let mut s = Solver::new(c, m, Box::new(SparseContractor::default()), rng);
            let res = s.solve(q);
            // Go shares one rng between the test and the solver.
            rng = s.rng;
            let (got, st) = res.unwrap_or_else(|e| panic!("b={} m={} p={}: {}", bits, m, c.p, e));
            assert_eq!(
                got, xi,
                "b={} m={} p={}: got {}, want {}",
                bits, m, c.p, got, xi
            );
            println!(
                "b={} m={} p={} r={} N={} relations={} attempts={} {:?}",
                bits, m, c.p, c.order, st.factor_base_size, st.relations, st.attempts, st.elapsed
            );
        }
    }
}
