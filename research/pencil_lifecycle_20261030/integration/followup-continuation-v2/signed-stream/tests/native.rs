use five_leads_study::{three_leads::*, three_leads_followup::*};
use serde_json::{json, Value};
use signed_collector_stream_controls::Capture;
use std::collections::BTreeMap;

#[test]
fn captured_native_candidates_replay_in_actual_order() {
    let inst = scholten_instance(739, 23).expect("fixed software fixture");
    let target = inst.curve.mul(inst.generator, 2);
    assert!(!target.inf);
    for (projective, rank, cap) in [
        (false, false, 64),
        (true, false, 64),
        (true, true, 64),
        (false, false, 2048),
        (true, false, 2048),
        (true, true, 2048),
    ] {
        let baseline =
            solve_genus2_observed(&inst, target, 47, cap, projective, rank, None).unwrap();
        let mut capture = Capture::new(16384, 8_000_000).unwrap();
        let observed = solve_genus2_observed(
            &inst,
            target,
            47,
            cap,
            projective,
            rank,
            Some(&mut |v| capture.push(v)),
        )
        .unwrap();
        assert!(capture.complete());
        assert_eq!(
            serde_json::to_value(&baseline).unwrap(),
            serde_json::to_value(&observed).unwrap()
        );
        let mut graph = SignedGraph::new(Ring::new(inst.r));
        let mut candidates = BTreeMap::new();
        let mut first = BTreeMap::new();
        let mut dispositions = BTreeMap::new();
        let mut edges = 0;
        let mut rows: Vec<Vec<u64>> = vec![];
        let mut kinds = BTreeMap::<String, usize>::new();
        for (i, record) in capture.records.iter().enumerate() {
            assert_eq!(record["sequence"], json!(i));
            let e = &record["event"];
            *kinds.entry(e["kind"].as_str().unwrap().into()).or_default() += 1;
            let attempt = e["attempt"].as_u64();
            match e["kind"].as_str().unwrap() {
                "header" => {
                    assert_eq!(i, 0);
                    assert_eq!(e["target"], serde_json::to_value(target).unwrap());
                    assert_eq!(e["zero_node"], 0);
                    assert_eq!(
                        e["source_field"],
                        serde_json::to_value(&inst.curve.f).unwrap()
                    );
                    assert_eq!(e["source_binary"], false);
                }
                "candidate" => {
                    let a = attempt.unwrap();
                    assert_eq!(a, candidates.len() as u64 + 1);
                    candidates.insert(a, e.clone());
                }
                "node" => {
                    assert!(candidates.contains_key(&attempt.unwrap()));
                    assert_eq!(e["node"], json!(graph.node()));
                    // A new nuisance column precedes the existing target/constant.
                    for row in &mut rows {
                        row.insert(row.len() - 2, 0);
                    }
                }
                "edge" => {
                    let a = attempt.unwrap();
                    let c = &candidates[&a];
                    assert_eq!(e["a"], c["a"]);
                    assert_eq!(e["b"], c["b"]);
                    let u = e["nodes"][0].as_u64().unwrap() as usize;
                    let v = e["nodes"][1].as_u64().unwrap() as usize;
                    let su = e["signs"][0].as_i64().unwrap() as i8;
                    let sv = e["signs"][1].as_i64().unwrap() as i8;
                    if rank {
                        assert_eq!(
                            e["rank_score"],
                            json!(graph.target_rank_score(u, su, v, sv, e["b"].as_u64().unwrap()))
                        );
                    }
                    let answer = graph.edge(
                        u,
                        su,
                        v,
                        sv,
                        e["a"].as_u64().unwrap(),
                        e["b"].as_u64().unwrap(),
                    );
                    assert_eq!(e["recovered"], json!(answer));
                    let n = graph.nodes() - 1;
                    let mut row = vec![0; n + 2];
                    for (node, sign) in [(u, su), (v, sv)] {
                        if node != 0 {
                            row[node - 1] = ((row[node - 1] as u128
                                + if sign == 1 { 1 } else { (inst.r - 1) as u128 })
                                % inst.r as u128)
                                as u64;
                        }
                    }
                    let neg = |x: u64| if x == 0 { 0 } else { inst.r - x };
                    row[n] = neg(e["b"].as_u64().unwrap());
                    row[n + 1] = neg(e["a"].as_u64().unwrap());
                    rows.push(row);
                    let info = pencil_target_information::information(inst.r, n, &rows).unwrap();
                    assert!(info.consistent);
                    assert_eq!(
                        info.target_determined,
                        answer.is_some(),
                        "native return versus whole captured prefix"
                    );
                    assert_eq!(e["cycles"], json!(graph.cycles));
                    assert_eq!(e["negative_cycles"], json!(graph.negative_cycles));
                    assert_eq!(e["zero_coefficient"], json!(graph.zero_coefficient));
                    first.insert(serde_json::to_string(&e["canonical"]).unwrap(), a);
                    assert!(dispositions.insert(a, "edge").is_none());
                    edges += 1;
                }
                "duplicate" => {
                    let a = attempt.unwrap();
                    let key = serde_json::to_string(&e["canonical"]).unwrap();
                    assert_eq!(e["first_attempt"], json!(first[&key]));
                    assert!(dispositions.insert(a, "duplicate").is_none());
                }
                "identity" | "nonsplit" => {
                    let a = attempt.unwrap();
                    assert!(candidates.contains_key(&a));
                    assert!(dispositions.insert(a, "other").is_none());
                }
                "choice" => {
                    assert!(candidates.contains_key(&e["selected_attempt"].as_u64().unwrap()));
                }
                "restart" => {}
                "terminal" => {
                    assert_eq!(i + 1, capture.records.len());
                    assert_eq!(e["attempts"], json!(candidates.len()));
                    assert_eq!(e["split_relations"], json!(edges));
                    assert_eq!(e["source_verified"], false);
                }
                x => panic!("unexpected {x}"),
            }
        }
        println!(
            "SIGNED_CONTROL {}",
            json!({"header":capture.records[0]["event"],"fixture_generator_seed":23,"p":739,"seed":47,"candidate_cap":cap,"projective":projective,"rank_policy":rank,"events":kinds,"graph_prefixes_checked":edges,"native_output_and_counts_equal":true,"recovered":observed.recovered,"prefix_consistency_checked":true,"target_information_matches_native_return":true})
        );
        assert_eq!(candidates.len(), dispositions.len());
        assert!(edges > 0, "fixture exercises actual graph call");
        if cap == 2048 {
            assert_eq!(observed.recovered, Some(2), "positive software fixture");
        }
        if let Some(k) = observed.recovered {
            assert_eq!(inst.curve.mul(inst.generator, k), target);
        }
    }
}
#[test]
fn observer_failure_is_immediate_and_has_no_terminal() {
    let inst = scholten_instance(739, 23).unwrap();
    let target = inst.curve.mul(inst.generator, 2);
    for limit in [1, 2, 3] {
        let mut capture = Capture::new(limit, 1_000_000).unwrap();
        assert!(solve_genus2_observed(
            &inst,
            target,
            47,
            64,
            true,
            true,
            Some(&mut |v| capture.push(v))
        )
        .is_err());
        assert!(!capture.complete());
        assert_eq!(capture.records.len(), limit);
        assert!(capture
            .records
            .iter()
            .all(|r| r["event"]["kind"] != "terminal"));
    }
}
#[test]
fn byte_limit_and_closed_capture_reject_more_records() {
    assert!(Capture::new(0, 1).is_err());
    let mut c = Capture::new(10, 1).unwrap();
    assert!(c.push(json!({"kind":"header"})).is_err());
    assert!(c.records.is_empty());
    let mut c = Capture::new(10, 1000).unwrap();
    c.push(json!({"kind":"header"})).unwrap();
    c.push(json!({"kind":"terminal"})).unwrap();
    assert!(c.complete());
    assert!(c.push(Value::Null).is_err());
    assert!(!c.complete());
}
