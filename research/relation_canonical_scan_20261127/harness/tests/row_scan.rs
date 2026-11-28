use crypto_lib::cryptanalysis::f4_fp::{Ordering, Poly};
use crypto_lib::cryptanalysis::groebner_f4::cmp_monomial;
use std::cell::Cell;
thread_local! { static NORMALIZATIONS: Cell<usize> = const { Cell::new(0) }; }
fn normalise(g: &Poly, p: u64, order: Ordering) -> Poly {
    NORMALIZATIONS.with(|c| c.set(c.get() + 1));
    crypto_lib::cryptanalysis::f4_fp::normalise(g, p, order)
}
#[path = "../../pending-source/canonical_row.rs"]
mod canonical_row;
fn reference(g: &Poly, p: u64, order: Ordering) -> bool {
    !g.is_empty() && g[0].1 == 1 && *g == crypto_lib::cryptanalysis::f4_fp::normalise(g, p, order)
}
#[test]
fn scan_matches_actual_normalization_on_fixed_field_coefficient_and_order_boundaries() {
    let mut checked = 0;
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            for order in [Ordering::Lex, Ordering::Grlex, Ordering::Grevlex] {
                let mut monos = vec![vec![0; n]];
                for i in 0..n {
                    let mut m = vec![0; n];
                    m[i] = 1;
                    monos.push(m);
                }
                let mut edge = vec![0; n];
                edge[n - 1] = 64;
                monos.push(edge);
                for a in &monos {
                    for b in &monos {
                        for c in [0, 1, 2, p - 1, p, p + 1, u64::MAX] {
                            for g in [
                                vec![(a.clone(), 1), (b.clone(), c)],
                                vec![(a.clone(), c), (b.clone(), 1)],
                                vec![(a.clone(), 1), (b.clone(), c), (a.clone(), p - 1)],
                            ] {
                                assert_eq!(
                                    canonical_row::is_canonical_monic(&g, p, order, false),
                                    reference(&g, p, order)
                                );
                                checked += 1;
                            }
                        }
                    }
                }
                for g in [vec![], vec![(vec![0; n], 0)], vec![(vec![0; n], 1)]] {
                    assert_eq!(
                        canonical_row::is_canonical_monic(&g, p, order, false),
                        reference(&g, p, order)
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 10000);
    println!("differential_rows_checked={checked}; no timing or degree claim");
}
#[test]
fn supported_scan_avoids_normalization_and_accounted_calls_keep_it() {
    for p in [2147483647, 9007199254740881] {
        let g = vec![(vec![1, 0, 0], 1), (vec![0, 0, 0], p - 1)];
        NORMALIZATIONS.with(|c| c.set(0));
        assert!(canonical_row::is_canonical_monic(
            &g,
            p,
            Ordering::Grevlex,
            false
        ));
        assert_eq!(NORMALIZATIONS.with(Cell::get), 0);
        assert!(canonical_row::is_canonical_monic(
            &g,
            p,
            Ordering::Grevlex,
            true
        ));
        assert_eq!(NORMALIZATIONS.with(Cell::get), 1);
    }
}
#[test]
fn unsupported_shapes_keep_the_original_path() {
    for (p, g) in [
        (7, vec![(vec![1, 0, 0], 1)]),
        (2147483647, vec![(vec![1], 1)]),
        (2147483647, vec![(vec![65, 0, 0], 1)]),
    ] {
        NORMALIZATIONS.with(|c| c.set(0));
        assert_eq!(
            canonical_row::is_canonical_monic(&g, p, Ordering::Grevlex, false),
            reference(&g, p, Ordering::Grevlex)
        );
        assert_eq!(NORMALIZATIONS.with(Cell::get), 1);
    }
}
