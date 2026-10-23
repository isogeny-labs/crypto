//! Child of products14: retain all prior scalar equations and add only the proved boundary radical condition.
use super::*;
pub fn run(dir: &Path) {
    let q13 = crate::round04::read(&dir.join("q13-inverse-associative-saturation.json"));
    let q14 = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let q15 = crate::round04::read(&dir.join("q15-linear-square-saturation.json"));
    let q16 = crate::round04::read(&dir.join("q16-linear-product-saturation.json"));
    let q17 = crate::round04::read(&dir.join("q17-expanded-product-quadrics.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound17Survivors\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound17Boundary\nopen KoblitzRound12Boundary KoblitzRound13Boundary KoblitzRound13SquareData KoblitzRound14Boundary\n");
    for graph in 0..4 {
        let input = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(input["program"].clone()).unwrap();
        let h = p.nodes.len();
        let old = &q13["records"][graph];
        let zeros: Vec<usize> = serde_json::from_value(old["boundary_zero_nodes"].clone()).unwrap();
        let eqs = eqs_from(old);
        let mut l = Lin::new(h);
        for (i, op) in p.nodes.iter().enumerate() {
            if let Op::Add(a, b) = op {
                assert!(l.equation(&[i, *a, *b], 0));
            }
        }
        for i in &zeros {
            assert!(l.equation(&[*i], 0));
        }
        for [a, b] in &eqs {
            assert!(l.equation(&[*a, *b], 0));
        }
        let mut products: Vec<(usize, usize)> =
            serde_json::from_value(old["original_zero_product_pairs"].clone()).unwrap();
        for r in old["inverse_relations"].as_array().unwrap() {
            products.push((
                r["out"].as_u64().unwrap() as usize,
                r["inverse"].as_u64().unwrap() as usize,
            ));
        }
        let mut qs: Vec<Quad> = old["shared_factor_relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| serde_json::from_value(r["quadratic_at_h0"].clone()).unwrap())
            .collect();
        qs.extend(
            q14["records"][graph]["relations"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| serde_json::from_value::<Quad>(r["outputs"].clone()).unwrap()),
        );
        for doc in [&q15, &q16] {
            for r in doc["records"][graph]["relations"].as_array().unwrap() {
                for c in coefficients(r) {
                    assert!(l.equation(&c, 0));
                }
            }
        }
        let polys: Vec<Multi> = q17["records"][graph]["coefficient_equations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| (serde_json::from_value(v.clone()).unwrap(), 0))
            .collect();
        let family = crate::round04::read(&dir.join("q23-survivor-source-separators.json"));
        let nodes = family["records"][graph]["family"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["cube_node"].as_u64().unwrap() as usize)
            .collect::<Vec<_>>();
        for node in &nodes {
            assert!(l.equation(&[*node], 0));
        }
        let mut queries = vec![];
        let mut model = None;
        let mut total = 0;
        let mut cap = false;
        for i in 0..h {
            let mut ll = l.clone();
            if !ll.equation(&[i], 1) {
                queries.push(json!({"unit_node":i,"status":"linear inconsistent","visits":0}));
                continue;
            }
            let mut count = 0;
            let z = search_multi(ll, &products, &qs, &polys, &mut count);
            total += count;
            if let Some(z) = z {
                checked(&p, &z, &zeros, &eqs, &products, &qs);
                assert!(nodes.iter().all(|i| z[*i] == 0));
                assert!(polys.iter().all(|m| value(m, &z) == 0));
                model = Some(z);
                queries.push(json!({"unit_node":i,"status":"checked model","visits":count}));
                break;
            }
            if count > 100000 || total > 1000000 {
                cap = true;
                queries
                    .push(json!({"unit_node":i,"status":"inconclusive search cap","visits":count}));
                break;
            }
            queries.push(json!({"unit_node":i,"status":"exhausted scalar query","visits":count}));
        }
        if let Some(z) = &model {
            let zero_ids = nodes
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let ids = z
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == 1)
                .map(|(i, _)| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
            lean.push_str(&format!("def point{graph} : List Nat := [{ids}]\ntheorem actual_remaining_model{graph} : (sparseCheck program{graph} point{graph} residual{graph} && extra point{graph} zeros{graph} eqs{graph} inv{graph} (qs{graph}++KoblitzRound14Boundary.pairs{graph}) && (KoblitzRound14Boundary.linear{graph}.all fun e => !(parity point{graph} e)) && (KoblitzRound14Boundary.expanded{graph}.all fun e => !(multi point{graph} e)))=true := by decide +kernel\ntheorem cube_zero{graph} : ([{zero_ids}].all fun i => !(point{graph}.contains i))=true := by decide +kernel\ntheorem nonzero{graph} : point{graph}.isEmpty=false := by decide +kernel\n"));
        }
        println!(
            "Q24:graph{graph},four ordinary cube-zero repairs,model{},cap{cap},visits{total}",
            model.is_some()
        );
        records.push(json!({"graph":graph,"new_boundary_zero_nodes":nodes,"condition_kind":"radical boundary consequences of all four source quartics;not membership of individual cube nodes in saturation","queries":queries,"total_visits":total,"capped":cap,"model":model,"all_prior_original_Q13_Q17_constraints_checked":true,"full_h_saturation":false,"survivor_is_false":false}));
    }
    lean.push_str("end KoblitzRound17Boundary\n");
    for r in &records {
        if !r["model"].is_null() {
            let g = r["graph"].as_u64().unwrap();
            for n in ["actual_remaining_model", "cube_zero", "nonzero"] {
                lean.push_str(&format!("#print axioms KoblitzRound17Boundary.{n}{g}\n"));
            }
        }
    }
    let path = dir.join("KoblitzRound17Boundary.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    round02::save(
        dir,
        "q24-after-all-ordinary-cube-repairs.json",
        &json!({"schema":"koblitz.round17/q24-v1","records":records}),
    );
}
