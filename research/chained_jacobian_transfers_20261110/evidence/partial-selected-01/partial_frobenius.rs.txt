//! Actual one-factor Frobenius graph transport; existing engines only.
//! Global inseparable degree/group-scheme claims require the written proof.
use super::*;

fn partial(f: &FpR, source: &Presentation, power: usize) -> Presentation {
    Presentation {
        curves: source.curves,
        graph: source
            .graph
            .iter()
            .map(|&(p, q)| (frobenius(f, p, power), q))
            .collect(),
    }
}

fn paired_history(f: &FpR, start: &Presentation) -> Vec<Presentation> {
    let mut models = start.curves.map(|e| vec![e]);
    let mut hops: [HashMap<Curve<u64>, Hop>; 2] = std::array::from_fn(|_| HashMap::new());
    let mut state = start.clone();
    let mut history = vec![];
    for _ in 0..25 {
        history.push(state.clone());
        for (i, source) in state.curves.iter().copied().enumerate() {
            if !hops[i].contains_key(&source) {
                hops[i].insert(source, construct_hop(f, source, i, &mut models[i]));
            }
        }
        let curves = std::array::from_fn(|i| hops[i][&state.curves[i]].target);
        let graph = state
            .graph
            .iter()
            .map(|&(p, q)| {
                (
                    hops[0][&state.curves[0]].eval(f, p),
                    hops[1][&state.curves[1]].eval(f, q),
                )
            })
            .collect();
        state = Presentation { curves, graph };
    }
    history
}

#[test]
fn partial_frobenius_actual_graphs_models_and_rational_shadow() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let ([eg, fg], q0) = actual_graph(&f, 0);
    let curves = CURVES[0].map(|[a, b]| Curve::new(a, b));
    let start = Presentation { curves, graph: q0 };
    let groups = [
        HashMap::from([(curves[0], eg)]),
        HashMap::from([(curves[1], fg)]),
    ];
    let states: [_; 4] = std::array::from_fn(|k| partial(&f, &start, k));
    for state in &states {
        check_graph(&f, state, &groups);
    }
    for i in 0..3 {
        for j in 0..i {
            assert!(!same_graph(&f, &states[i].graph, &states[j].graph));
            assert!(!same_graph(
                &f,
                &doubled(&f, &states[i]),
                &doubled(&f, &states[j])
            ));
        }
    }
    let exact_negative = start
        .graph
        .iter()
        .map(|&(p, q)| (curve::neg(&f, &p), q))
        .collect::<HashSet<_>>();
    assert_eq!(
        states[3].graph.iter().copied().collect::<HashSet<_>>(),
        exact_negative
    );
    assert!(same_graph(&f, &states[3].graph, &start.graph));
    let source_set = start.graph.iter().copied().collect::<HashSet<_>>();
    for (k, state) in states.iter().enumerate().take(3).skip(1) {
        let identity_mutant_missing = state
            .graph
            .iter()
            .filter(|p| !source_set.contains(p))
            .count();
        assert!(identity_mutant_missing > 0);
        println!("CONTROL partialFrobenius k{k}:geometricidentitymutant missing{identity_mutant_missing}/16 actualgraphpoints");
    }
    for &p in &groups[0][&curves[0]].points {
        for &q in &groups[0][&curves[0]].points {
            let g = &groups[0][&curves[0]];
            assert_eq!(
                pairing(&f, g, frobenius(&f, p, 1), frobenius(&f, q, 1)),
                f.pow(pairing(&f, g, p, q), 101)
            );
        }
    }
    let history = paired_history(&f, &start);
    for (power, expected) in [(1, 24), (2, 12)] {
        assert_eq!(states[power].curves, history[expected].curves);
        assert!(same_graph(
            &f,
            &states[power].graph,
            &history[expected].graph
        ));
        assert!(same_graph(
            &f,
            &doubled(&f, &states[power]),
            &doubled(&f, &history[expected])
        ));
        let model = construct_even(&f, curves, &doubled(&f, &states[power]));
        check_even(&f, &model, if power == 1 { "Cpi1" } else { "Cpi2" });
        println!("PASS partialFrobenius k{power}:actualA{expected}/Gamma{expected};16graph/256sums+Weil/Frobeniusstable;model{:?};factorstageidentityonF101 only", model.coefficients);
    }
    let rational: [_; 2] = curves.map(|e| rational_points(&base, &e));
    assert_eq!([rational[0].len(), rational[1].len()], [109, 113]);
    for k in 1..=3 {
        for points in &rational {
            for &p in points {
                let lifted = point_lift(&f, p);
                assert_eq!(frobenius(&f, lifted, k), lifted);
            }
        }
    }
    for k in 1..=2 {
        let images = rational[0]
            .iter()
            .map(|&p| frobenius(&f, point_lift(&f, p), k))
            .collect::<Vec<_>>();
        let mut pairs = 0;
        for (&p, image) in rational[0].iter().zip(images) {
            for &q in &rational[1] {
                assert_eq!(
                    (image, point_lift(&f, q)),
                    (point_lift(&f, p), point_lift(&f, q))
                );
                pairs += 1;
            }
        }
        assert_eq!(pairs, 12317);
    }
    assert_eq!((6103 % 109, 6103 % 113), (108, 1));
    assert_eq!((6214 % 109, 6214 % 113), (1, 112));
    let first = rational[0]
        .iter()
        .map(|p| curve::pmul(&base, &curves[0], p, 6103))
        .collect::<Vec<_>>();
    let second = rational[1]
        .iter()
        .map(|p| curve::pmul(&base, &curves[1], p, 6103))
        .collect::<Vec<_>>();
    let mut signed_pairs = 0;
    for (&p, &image) in rational[0].iter().zip(&first) {
        assert_eq!(image, curve::neg(&base, &p));
        for (&q, &other) in rational[1].iter().zip(&second) {
            assert_eq!(other, q);
            signed_pairs += 1;
        }
    }
    assert_eq!(signed_pairs, 12317);
    println!("PASS partialFrobenius:rational109+113fixed/k1+k2all12317productpairsidentity;signnormalizedk3all12317productpairsCRT6103/residues108,1/alternative6214. Globaldegree101^k ispurelyinseparable/group-schemekernel,writtennot101geometricpoints;no wholeMumfordmap or timing acceptance");
}
