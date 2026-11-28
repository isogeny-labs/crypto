//! Tests the live production helper, so it cannot drift from the pending copy.
use crypto_lib::cryptanalysis::f4_fp::{normalise, Ordering, Poly};
use crypto_lib::cryptanalysis::groebner_f4::cmp_monomial;
#[path = "../../../../src/cryptanalysis/f4_fp/canonical_row.rs"]
mod canonical_row;

#[test]
fn live_production_scan_matches_reference_in_both_accounting_modes() {
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
                                let expected =
                                    !g.is_empty() && g[0].1 == 1 && g == normalise(&g, p, order);
                                for accounted in [false, true] {
                                    assert_eq!(
                                        canonical_row::is_canonical_monic(&g, p, order, accounted),
                                        expected
                                    );
                                    checked += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("live_production_predicates_checked={checked}; both accounting flags; no clocks");
    assert!(checked > 30000);
}
