//! Compare the original embedded elliptic source with its direct destination.
use super::super::ct28_pullback::rational_points;
use super::{PrimeFieldPsi, Pullback};
use isogeny_algos::curve::{self, Isogeny, Pt};
use std::collections::{HashMap, HashSet};

#[test]
fn original_elliptic_sources_have_the_same_direct_endpoint() {
    let direct = PrimeFieldPsi::new();
    let old = super::super::SevenChain::new();
    for component in 0..2 {
        let source = Pullback::fixed(0, component);
        let map = &direct.alpha[component];
        let points = rational_points(&direct.fp, &map.dom);
        let order = [109_u64, 113][component];
        assert_eq!(points.len(), usize::try_from(order).unwrap());
        let mut outputs = Vec::new();
        let mut distinct = HashSet::new();
        for &p in &points {
            let embedded = source.pull(&p).unwrap().0;
            let chained = direct.forward(&embedded).unwrap();
            let image = map.eval(&direct.fp, &p);
            let endpoint = direct.pulls[component].pull(&image).unwrap().0;
            assert_eq!(
                chained, endpoint,
                "same concrete endpoint for component{component}/point{p:?}"
            );
            assert!(distinct.insert(super::key(&endpoint)));
            let coordinates = old
                .t
                .target_coordinates(&old.r1.dual_closed(&endpoint).unwrap())
                .unwrap();
            let mut expected = [Pt::Inf; 2];
            expected[component] = curve::pmul(&direct.fp, &map.cod, &image, 4);
            assert_eq!(coordinates, expected);
            let reverse = old.dual(&endpoint).unwrap();
            let recovered = old
                .t
                .source_coordinates(&old.r0.dual_closed(&reverse).unwrap())
                .unwrap();
            let mut expected = [Pt::Inf; 2];
            expected[component] = curve::pmul(&direct.fp, &map.dom, &p, 28);
            assert_eq!(recovered, expected);
            assert_eq!(
                curve::pmul(
                    &direct.fp,
                    &map.dom,
                    &recovered[component],
                    [39, 109][component]
                ),
                p
            );
            if p != Pt::Inf {
                assert_ne!(
                    direct.target.mul(&endpoint, 4).unwrap(),
                    endpoint,
                    "omitting the rational quarter is wrong"
                );
            }
            outputs.push(endpoint);
        }
        let g = points[1];
        assert_eq!(
            curve::pmul(&direct.fp, &map.dom, &g, u128::from(order)),
            Pt::Inf
        );
        let indices: HashMap<_, _> = points.iter().enumerate().map(|(i, p)| (*p, i)).collect();
        for k in 0..order {
            let p = curve::pmul(&direct.fp, &map.dom, &g, u128::from(k));
            assert_eq!(
                outputs[indices[&p]],
                direct.target.mul(&outputs[1], k).unwrap(),
                "source scalar{component}/{k}"
            );
        }
        println!("PASS all{order} original elliptic inputs/scalars:wholePsi after direct K0 equals direct K1alpha,injective same endpoints,target coordinates4alpha,fullreverse28/recovery and wrong-quarter rejection. Scope embedded source only,no whole-Jacobian collapse or timing claim");
    }
}
