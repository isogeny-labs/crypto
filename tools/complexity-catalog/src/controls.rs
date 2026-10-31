//! Finite controls taken from published counterexample constructions. They do
//! not independently prove unbounded degree or all-exponent asymptotic claims.

#[test]
fn three_small_degree_generators_are_product_zero_without_a_common_boolean_zero() {
    // Hexagon 2610.00327v1, Theorem 1.1: t*A, t*B, 1-t.
    for n in 1..=4usize {
        let mut nonzero = [false; 3];
        for assignment in 0usize..1usize << (2 * n + 1) {
            let t = (assignment & 1) as i64;
            let x: i64 = (0..n).map(|j| ((assignment >> (j + 1)) & 1) as i64).sum();
            let y: i64 = (0..n)
                .map(|j| ((assignment >> (n + j + 1)) & 1) as i64)
                .sum();
            let g = [t * x, t * (x + y - n as i64 - 1), 1 - t];
            assert_eq!(g.iter().product::<i64>(), 0);
            assert!(g.iter().any(|&v| v != 0));
            for i in 0..3 {
                nonzero[i] |= g[i] != 0;
            }
        }
        assert_eq!(nonzero, [true; 3]);
    }
}

#[test]
fn subspace_retention_is_not_union_retention() {
    // Hexagon 2610.00328v1, section 2. Unused ambient coordinates cancel
    // from every fraction; enumerate only the 2k distinguished coordinates.
    for k in 2..=4usize {
        let masks: Vec<_> = (0usize..1usize << (2 * k))
            .filter(|m| m.count_ones() as usize == k)
            .collect();
        let mut deleted_from_union = 0usize;
        for assignment in 0usize..1usize << (2 * k) {
            let middle = assignment.count_ones() as usize == k;
            let mut covered = false;
            let mut retained = false;
            for &mask in &masks {
                let member = assignment & mask == 0 || assignment & mask == mask;
                covered |= member;
                retained |= member && !middle;
            }
            assert!(covered);
            if !retained {
                deleted_from_union += 1;
            }
        }
        assert_eq!(deleted_from_union, masks.len());
        for &mask in &masks {
            for bit in [false, true] {
                let in_member = |a: usize| if bit { a & mask == mask } else { a & mask == 0 };
                let members = (0usize..1usize << (2 * k))
                    .filter(|&a| in_member(a))
                    .count();
                let deleted = (0usize..1usize << (2 * k))
                    .filter(|&a| in_member(a) && a.count_ones() as usize == k)
                    .count();
                assert_eq!(members, 1usize << k);
                assert_eq!(deleted, 1);
                assert!(deleted_from_union * members > deleted * (1usize << (2 * k)));
            }
        }
    }
}
