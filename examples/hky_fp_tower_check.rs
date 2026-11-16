//! Hand cases for `hky_fp_tower` (gate G1 of
//! `research/prime_fall_degree_20261027/PREREGISTRATION.md`). Each case
//! panics on a wrong value; `main` runs them all.
//!
//! `cargo run --release --example hky_fp_tower_check`
use crypto_lib::cryptanalysis::f4_fp_tower::SquareRule;
use crypto_lib::cryptanalysis::hky_fp_tower::*;

fn kummer_rules(t: usize, blocks: usize, c_top: u64) -> Vec<SquareRule> {
    let mut rules = Vec::new();
    for b in 0..blocks {
        for j in 0..t {
            let top = j + 1 == t;
            rules.push(SquareRule {
                next: (!top).then_some((b * t + j + 1) as u8),
                a: 0,
                b: if top { 0 } else { 1 },
                c: 0,
                d: if top { c_top } else { 0 },
            });
        }
    }
    rules
}

fn column_count_matches_closure() {
    // 1 in the generators: V_{F,c} = R_{<=c}, whose dimension is the count.
    let ring = HkyRing::new(97, kummer_rules(3, 2, 1), 3);
    let one = ring.from_raw(&[(vec![0; 9], 1)]);
    for c in 0..5 {
        let cl = closure(&ring, std::slice::from_ref(&one), c, &HkyLimits::default());
        assert!(cl.contains_one);
        assert_eq!(u128::from(cl.dim()), count_monomials(6, 3, c));
        assert_eq!(cl.cols, count_monomials(6, 3, c));
    }
}

fn linear_inconsistency_is_degree_one() {
    let ring = HkyRing::new(101, kummer_rules(2, 1, 1), 0);
    let f = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], 100)]);
    let g = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], 99)]);
    let fd = fall_degrees(&ring, &[f, g], 1, 4, &HkyLimits::default());
    assert_eq!(fd.last_fall, Some(1));
}

fn tower_rewrite_closes() {
    // y0 = 3 on y0^2 = y1, y1^2 = 1 mod 101 (3^4 = 81 != 1): inconsistent.
    // At c = 2: y0*(y0 - 3) = y1 - 3*y0 gives y1 - 9, then
    // y1*(y1 - 9) = 1 - 9*y1 gives 1 - 81: 1 in V_{F,2}.
    // At c = 1, V = <y0 - 3>, so the fall at 2 (y1 - 9 has degree 1) is the first.
    let ring = HkyRing::new(101, kummer_rules(2, 1, 1), 0);
    let f = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], 98)]);
    let fd = fall_degrees(&ring, &[f], 1, 4, &HkyLimits::default());
    assert_eq!(fd.last_fall, Some(2));
    assert_eq!(fd.first_fall, Some(2));
}

fn consistent_system_has_no_one() {
    // y0 = 10 with 10^2 = -1 mod 101, so 10^4 = 1: consistent, one point.
    // I = (y0 - 10, y1 + 1); dim I ∩ R_{<=c} = #monomials - 1 = 3 for c >= 2.
    let ring = HkyRing::new(101, kummer_rules(2, 1, 1), 0);
    let f = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], 91)]);
    let fd = fall_degrees(&ring, &[f], 1, 4, &HkyLimits::default());
    assert_eq!(fd.last_fall, None);
    for cl in &fd.closures[1..] {
        assert_eq!(cl.dim(), 3);
    }
}

fn free_variable_only_system() {
    // u^2 - 2 and u - 3 over F_101 (9 != 2): u*(u - 3) = u^2 - 3u
    // -> 2 - 3u, then 2 - 3u + 3(u - 3) = -7: 1 in V at c = 2, and
    // V_{F,1} = <u - 3>, so the fall at 2 is the first.
    let ring = HkyRing::new(101, Vec::new(), 1);
    let f = ring.from_raw(&[(vec![2], 1), (vec![0], 99)]);
    let g = ring.from_raw(&[(vec![1], 1), (vec![0], 98)]);
    let fd = fall_degrees(&ring, &[f, g], 1, 4, &HkyLimits::default());
    assert_eq!(fd.last_fall, Some(2));
}

fn isogeny_rule_rewrites_without_raising_degree() {
    // y0^2 = y0*y1 - 2*y1 + 2*y0 - 5 (a=1, b=-2, c=2, d=-5), y1^2 = 4 mod 101.
    let p = 101;
    let rules = vec![
        SquareRule {
            next: Some(1),
            a: 1,
            b: p - 2,
            c: 2,
            d: p - 5,
        },
        SquareRule {
            next: None,
            a: 0,
            b: 0,
            c: 0,
            d: 4,
        },
    ];
    let ring = HkyRing::new(p, rules, 0);
    let sq = ring.from_raw(&[(vec![2, 0], 1)]);
    assert_eq!(HkyRing::degree(&sq), 2);
    // evaluation agrees on a point of the tower
    for y1 in [2u64, 99] {
        for y0 in 0..p {
            // y0 on the tower: y0^2 - (y0*y1 - 2y1 + 2y0 - 5) = 0
            let lhs = y0 * y0 % p;
            let rhs = (y0 * y1 + 2 * (p - y1) + 2 * y0 + p - 5) % p;
            if lhs == rhs {
                assert_eq!(ring.eval(&sq, &[y0, y1]), lhs);
            }
        }
    }
}

fn main() {
    column_count_matches_closure();
    println!("ok column_count_matches_closure");
    linear_inconsistency_is_degree_one();
    println!("ok linear_inconsistency_is_degree_one");
    tower_rewrite_closes();
    println!("ok tower_rewrite_closes");
    consistent_system_has_no_one();
    println!("ok consistent_system_has_no_one");
    free_variable_only_system();
    println!("ok free_variable_only_system");
    isogeny_rule_rewrites_without_raising_degree();
    println!("ok isogeny_rule_rewrites_without_raising_degree");
    println!("G1: 6 cases pass");
}
