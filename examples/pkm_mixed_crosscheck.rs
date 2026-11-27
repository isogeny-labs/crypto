//! Cross-check the mixed-degree relation finder against the exhaustive scan.
//!
//! `algebraic_decompose` (F4 on the paper's system built from a mixed-degree
//! Vélu chain) must return exactly the relations `decompose_m2` finds by
//! scanning `V` — the same check the unit-test module would make, kept as a
//! runnable example because the lib-test build embeds research artifacts that
//! are absent from some checkouts.
//!
//! Usage: `cargo run --release --example pkm_mixed_crosscheck`

use crypto_lib::cryptanalysis::pkm_smooth_tower as st;
use std::time::Duration;

fn main() {
    let p = 1009u64;
    let e = st::Curve { p, a: 1, b: 3 };
    let mut rng = st::Rng(7);
    let (mut decided, mut with_rels, mut targets) = (0u64, 0u64, 0u64);

    for (s, degrees) in [
        (15u64, vec![3u64, 5]),
        (49, vec![7, 7]),
        (105, vec![3, 5, 7]),
    ] {
        let Ok(found) = st::search_aux_exact(p, s, 11, 4000) else {
            continue;
        };
        let tower =
            st::isogeny_tower_on(found.aux, found.order, found.q, &degrees, 11, found.curves_tried)
                .expect("tower");
        tower.verify().expect("verify");
        assert_eq!(tower.v.len() as u64, s, "|V| != S");
        let dmax = *degrees.iter().max().unwrap() as u32 + 8;

        for _ in 0..8 {
            let Some(r) = e.random_point(&mut rng) else {
                continue;
            };
            targets += 1;
            let dec = st::algebraic_decompose(&e, &tower, 2, r, dmax, Duration::from_secs(30));
            if dec.undetermined {
                continue;
            }
            decided += 1;
            let mut got: Vec<Vec<(u64, u64)>> = dec.relations.clone();
            let mut want: Vec<Vec<(u64, u64)>> = st::decompose_m2(&e, &tower.v, r)
                .into_iter()
                .map(|rel| {
                    let mut pts = vec![rel.p1, rel.p2];
                    pts.sort_unstable();
                    pts
                })
                .collect();
            got.sort();
            got.dedup();
            want.sort();
            want.dedup();
            assert_eq!(got, want, "S={s} degrees={degrees:?} r={r:?}");
            if !got.is_empty() {
                with_rels += 1;
            }
        }
    }

    assert!(decided >= 3, "F4 decided only {decided} of {targets}");
    assert!(with_rels >= 1, "no target had a relation: vacuous");
    println!(
        "{{\"p\":{p},\"targets\":{targets},\"decided\":{decided},\
\"with_relations\":{with_rels},\"ok\":true}}"
    );
}
