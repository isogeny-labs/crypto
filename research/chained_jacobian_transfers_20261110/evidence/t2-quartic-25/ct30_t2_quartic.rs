//! Model-specific H2 adapter to the single existing quartic reconstruction.
use super::SuccessorChain;
use crate::nonsplit::map::seven_glue::torsion::chain::ct28_quartic::reconstruct_from;

#[test]
fn reconstruct_both_t2_elliptic_quartic_covers() {
    let c = SuccessorChain::new();
    let f = &c.s.old.fp;
    let all = c.r2.target.classes().unwrap();
    for component in 0..2 {
        let curve = &c.s.beta[component].map.cod;
        let coordinates = |d: &crate::balanced::Class| {
            c.s.target_coordinates(&c.r2.dual_closed(d).unwrap())
                .unwrap()
        };
        reconstruct_from(
            f,
            curve,
            &all,
            coordinates,
            vec![95, 93, 43, 39, 92, 17, 1],
            "2",
            component,
        );
    }
}
