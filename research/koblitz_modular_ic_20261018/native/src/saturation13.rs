//! Actual-circuit colon relations and countermodels after the constant repair.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
#[path = "square13.rs"]
pub(crate) mod square13;
type Quad = [usize; 4];
fn qvalue(q: Quad, z: &[u8]) -> u8 {
    (z[q[0]] & z[q[1]]) ^ (z[q[2]] & z[q[3]])
}
fn search_all(
    l: Lin,
    products: &[(usize, usize)],
    quads: &[Quad],
    visited: &mut usize,
) -> Option<Vec<u8>> {
    *visited += 1;
    if *visited > 100000 {
        return None;
    }
    let z = l.model();
    if let Some((a, b)) = products.iter().find(|(a, b)| z[*a] & z[*b] != 0) {
        for i in [a, b] {
            let mut ll = l.clone();
            if ll.equation(&[*i], 0) {
                if let Some(z) = search_all(ll, products, quads, visited) {
                    return Some(z);
                }
            }
        }
        return None;
    }
    if let Some(q) = quads.iter().find(|q| qvalue(**q, &z) != 0) {
        let vs: Vec<_> = q
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        for mask in 0..1usize << vs.len() {
            let val = |i| ((mask >> vs.iter().position(|v| *v == i).unwrap()) & 1) as u8;
            if (val(q[0]) & val(q[1])) ^ (val(q[2]) & val(q[3])) != 0 {
                continue;
            }
            let mut ll = l.clone();
            if vs
                .iter()
                .enumerate()
                .all(|(j, i)| ll.equation(&[*i], ((mask >> j) & 1) as u8))
            {
                if let Some(z) = search_all(ll, products, quads, visited) {
                    return Some(z);
                }
            }
        }
        return None;
    }
    Some(z)
}
fn query_quad(
    l: &Lin,
    products: &[(usize, usize)],
    qs: &[Quad],
    q: Quad,
) -> (Option<Vec<u8>>, usize) {
    let vs: Vec<_> = q
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut visited = 0;
    for mask in 0..1usize << vs.len() {
        let val = |i| ((mask >> vs.iter().position(|v| *v == i).unwrap()) & 1) as u8;
        if (val(q[0]) & val(q[1])) ^ (val(q[2]) & val(q[3])) != 1 {
            continue;
        }
        let mut ll = l.clone();
        if vs
            .iter()
            .enumerate()
            .all(|(j, i)| ll.equation(&[*i], ((mask >> j) & 1) as u8))
        {
            if let Some(z) = search_all(ll, products, qs, &mut visited) {
                return (Some(z), visited);
            }
        }
        if visited > 100000 {
            break;
        }
    }
    (None, visited)
}
fn checked(
    p: &Program,
    z: &[u8],
    zeros: &[usize],
    equalities: &[[usize; 2]],
    products: &[(usize, usize)],
    quads: &[Quad],
) -> usize {
    assert_eq!(z.len(), p.nodes.len());
    let mut checked = 0;
    for (i, op) in p.nodes.iter().enumerate() {
        let r = match op {
            Op::Input(_) => continue,
            Op::Constant(_) => z[i],
            Op::Add(a, b) => z[i] ^ z[*a] ^ z[*b],
            Op::Mul(a, b) => z[*a] & z[*b],
            Op::CoefficientFrobenius(a) => z[i] ^ z[*a],
            Op::Inverse(a) => z[*a] & z[i],
        };
        for j in 0..360 {
            assert_eq!(if j == 0 { r } else { 0 }, 0);
            checked += 1;
        }
    }
    for r in &p.residuals {
        assert_eq!(z[*r], 0);
        checked += 360;
    }
    for i in zeros {
        assert_eq!(z[*i], 0);
    }
    for [i, j] in equalities {
        assert_eq!(z[*i], z[*j]);
    }
    for (a, b) in products {
        assert_eq!(z[*a] & z[*b], 0);
    }
    for q in quads {
        assert_eq!(qvalue(*q, z), 0);
    }
    checked
}
pub fn run(dir: &Path) {
    let sat = crate::round04::read(&dir.join("q10c-constant-gate-saturation-step.json"));
    let mut records = vec![];
    for graph in 0..4 {
        let inp = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(inp["program"].clone()).unwrap();
        let mut l = Lin::new(p.nodes.len());
        let mut products = vec![];
        let mut zeros = BTreeSet::new();
        let mut eqs = BTreeSet::new();
        let mut inverses = BTreeMap::new();
        for (i, op) in p.nodes.iter().enumerate() {
            match op {
                Op::Input(_) => {}
                Op::Constant(_) => {
                    zeros.insert(i);
                }
                Op::Add(a, b) => assert!(l.equation(&[i, *a, *b], 0)),
                Op::CoefficientFrobenius(a) => {
                    eqs.insert([i, *a]);
                }
                Op::Mul(a, b) => products.push((*a, *b)),
                Op::Inverse(a) => {
                    products.push((*a, i));
                    inverses.insert(*a, i);
                }
            }
        }
        for r in &p.residuals {
            zeros.insert(*r);
        }
        for c in sat["records"][graph]["evaluated_constant_nodes"]
            .as_array()
            .unwrap()
        {
            zeros.insert(c["node"].as_u64().unwrap() as usize);
        }
        for r in sat["records"][graph]["new_linear_relations"]
            .as_array()
            .unwrap()
        {
            let out = r["out"].as_u64().unwrap() as usize;
            if r["kind"] == "scale" {
                let x = r["input"].as_u64().unwrap() as usize;
                match r["scalar"].as_str().unwrap() {
                    "0" => {
                        zeros.insert(out);
                    }
                    "1" => {
                        eqs.insert([out, x]);
                    }
                    _ => {
                        zeros.insert(out);
                        zeros.insert(x);
                    }
                }
            } else {
                zeros.insert(out);
            }
        }
        for (a, b) in &products {
            if a == b {
                zeros.insert(*a);
            }
        }
        let zero_node = *zeros.first().unwrap();
        for z in &zeros {
            assert!(l.equation(&[*z], 0));
        }
        for [i, j] in &eqs {
            assert!(l.equation(&[*i, *j], 0));
        }
        let zeros: Vec<_> = zeros.into_iter().collect();
        let eqs: Vec<_> = eqs.into_iter().collect();
        let mut invrels = vec![];
        let mut shared: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
        let mut trees: BTreeMap<Vec<usize>, Vec<(usize, usize, usize, usize, usize)>> =
            BTreeMap::new();
        for (i, op) in p.nodes.iter().enumerate() {
            if let Op::Mul(a, b) = op {
                for (a, b) in [(*a, *b), (*b, *a)] {
                    shared.entry(a).or_default().push((i, b));
                    if let Some(z) = inverses.get(&a) {
                        invrels.push(json!({"out":i,"inverse":z,"base":a,"other":b,"quadratic_at_h0":[i,z],"h_membership_power":1}));
                    }
                    if let Op::Mul(c, d) = p.nodes[a] {
                        let mut key = vec![c, d, b];
                        key.sort();
                        trees.entry(key).or_default().push((i, a, c, d, b));
                    }
                }
            }
        }
        let consts: BTreeSet<_> = sat["records"][graph]["evaluated_constant_nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["node"].as_u64().unwrap() as usize)
            .collect();
        let mut assoc = vec![];
        let mut seen = BTreeSet::new();
        for (a, v) in shared {
            if consts.contains(&a) {
                continue;
            }
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let (w, b) = v[i];
                    let (z, c) = v[j];
                    if b == c {
                        continue;
                    }
                    let mut mon = [[w, c], [z, b]];
                    for v in &mut mon {
                        v.sort();
                    }
                    mon.sort();
                    if seen.insert(mon) {
                        assoc.push(json!({"out_left":w,"out_right":z,"common":a,"other_left":b,"other_right":c,"quadratic_at_h0":[w,c,z,b],"h_membership_power":1}));
                    }
                }
            }
        }
        let mut linear = vec![];
        for (leaves, v) in trees {
            let mut outs = BTreeSet::new();
            let mut reps = vec![];
            for t in v {
                if outs.insert(t.0) {
                    reps.push(t);
                }
            }
            for w in reps.windows(2) {
                linear.push(json!({"out_left":w[0].0,"out_right":w[1].0,"tree_left":[w[0].1,w[0].2,w[0].3,w[0].4],"tree_right":[w[1].1,w[1].2,w[1].3,w[1].4],"common_leaf_multiset":leaves,"h_membership_power":2}));
            }
        }
        let mut stages = vec![];
        let mut failed = vec![];
        let mut aanswer = None;
        for (j, r) in invrels.iter().enumerate() {
            let i = r["out"].as_u64().unwrap() as usize;
            let z = r["inverse"].as_u64().unwrap() as usize;
            let mut ll = l.clone();
            let mut visited = 0;
            if ll.equation(&[i], 1) && ll.equation(&[z], 1) {
                if let Some(model) = search_all(ll, &products, &[], &mut visited) {
                    let n = checked(&p, &model, &zeros, &eqs, &products, &[]);
                    aanswer = Some(
                        json!({"model":model,"violated_relation_index":j,"violated_quadratic":[i,z,zero_node,zero_node],"checked_original_scalar_equations":n,"search_nodes":visited}),
                    );
                    break;
                }
            }
            failed.push(json!([j, visited]));
        }
        stages.push(json!({"repair":"all round12 constant relations","counterpoint":aanswer,"failed_queries":failed}));
        let mut ip = products.clone();
        for r in &invrels {
            ip.push((
                r["out"].as_u64().unwrap() as usize,
                r["inverse"].as_u64().unwrap() as usize,
            ));
        }
        let mut banswer = None;
        let mut failed = vec![];
        for (j, r) in assoc.iter().enumerate() {
            let q: Quad = serde_json::from_value(r["quadratic_at_h0"].clone()).unwrap();
            let (model, visited) = query_quad(&l, &ip, &[], q);
            if let Some(model) = model {
                let n = checked(&p, &model, &zeros, &eqs, &ip, &[]);
                banswer = Some(
                    json!({"model":model,"violated_relation_index":j,"violated_quadratic":q,"checked_original_scalar_equations":n,"search_nodes":visited}),
                );
                break;
            }
            failed.push(json!([j, visited]));
        }
        stages.push(json!({"repair":"constants plus all generated inverse cancellations","counterpoint":banswer,"failed_queries":failed}));
        let aq: Vec<Quad> = assoc
            .iter()
            .map(|r| serde_json::from_value(r["quadratic_at_h0"].clone()).unwrap())
            .collect();
        let mut canswer = None;
        let mut failed = vec![];
        for (j, r) in linear.iter().enumerate() {
            let a = r["out_left"].as_u64().unwrap() as usize;
            let b = r["out_right"].as_u64().unwrap() as usize;
            let mut ll = l.clone();
            let mut visited = 0;
            if ll.equation(&[a, b], 1) {
                if let Some(model) = search_all(ll, &ip, &aq, &mut visited) {
                    let n = checked(&p, &model, &zeros, &eqs, &ip, &aq);
                    canswer = Some(
                        json!({"model":model,"violated_relation_index":j,"violated_linear":[a,b],"checked_original_scalar_equations":n,"search_nodes":visited}),
                    );
                    break;
                }
            }
            failed.push(json!([j, visited]));
        }
        stages.push(json!({"repair":"constants,inverse cancellation and all generated shared-factor quadrics","counterpoint":canswer,"failed_queries":failed}));
        println!("Q13:graph{graph},{} inverse,{} shared-factor,{} depth2 linear relations;counterpoints {:?}",invrels.len(),assoc.len(),linear.len(),stages.iter().map(|s|!s["counterpoint"].is_null()).collect::<Vec<_>>());
        records.push(json!({"graph":graph,"boundary_zero_nodes":zeros,"boundary_equalities":eqs,"original_zero_product_pairs":products,"inverse_relations":invrels,"shared_factor_relations":assoc,"associative_linear_relations":linear,"stages":stages,"full_h_saturation":false,"scope":"exhaustive generated local inverse/shared-factor/three-leaf DAG colon instances,not the whole degree3 Macaulay space"}));
    }
    round02::save(
        dir,
        "q13-inverse-associative-saturation.json",
        &json!({"schema":"koblitz.round13/q13-v1","records":records,"coefficient_field":"F2^360,scalar restrictions interpreted by the original tensor","Boolean_equations_added_to_original_scheme":false,"full_h_saturation":false,"proper_quotient_embedding":false}),
    );
}
pub fn pairs(dir: &Path) {
    let old = crate::round04::read(&dir.join("q13-inverse-associative-saturation.json"));
    let constants = crate::round04::read(&dir.join("q10c-constant-gate-saturation-step.json"));
    let mut records = vec![];
    for graph in 0..4 {
        let inp = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(inp["program"].clone()).unwrap();
        let r = &old["records"][graph];
        let zeros: Vec<usize> = serde_json::from_value(r["boundary_zero_nodes"].clone()).unwrap();
        let eqs: Vec<[usize; 2]> =
            serde_json::from_value(r["boundary_equalities"].clone()).unwrap();
        let mut l = Lin::new(p.nodes.len());
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
            serde_json::from_value(r["original_zero_product_pairs"].clone()).unwrap();
        for q in r["inverse_relations"].as_array().unwrap() {
            products.push((
                q["out"].as_u64().unwrap() as usize,
                q["inverse"].as_u64().unwrap() as usize,
            ));
        }
        let quads: Vec<Quad> = r["shared_factor_relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| serde_json::from_value(q["quadratic_at_h0"].clone()).unwrap())
            .collect();
        let consts: BTreeSet<_> = constants["records"][graph]["evaluated_constant_nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| q["node"].as_u64().unwrap() as usize)
            .collect();
        let ms: Vec<_> = p
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, op)| match op {
                Op::Mul(a, b) if !consts.contains(a) && !consts.contains(b) => Some((i, *a, *b)),
                _ => None,
            })
            .collect();
        let mut groups: BTreeMap<Vec<usize>, Vec<[usize; 2]>> = BTreeMap::new();
        let mut tried = 0;
        let mut capped = false;
        'pairs: for (i, a) in ms.iter().enumerate() {
            for b in &ms[i..] {
                if tried == 250000 {
                    capped = true;
                    break 'pairs;
                }
                tried += 1;
                let mut key = vec![a.1, a.2, b.1, b.2];
                key.sort();
                groups.entry(key).or_default().push([a.0, b.0]);
            }
        }
        let mut rels = vec![];
        for (leaves, v) in groups {
            if v.len() < 2 {
                continue;
            }
            for other in &v[1..] {
                rels.push(json!({"outputs":[v[0][0],v[0][1],other[0],other[1]],"common_leaf_multiset":leaves,"h_membership_power":2}));
            }
        }
        let mut answer = None;
        let mut failed = vec![];
        for (j, q) in rels.iter().enumerate() {
            let qv: Quad = serde_json::from_value(q["outputs"].clone()).unwrap();
            let (model, visited) = query_quad(&l, &products, &quads, qv);
            if let Some(model) = model {
                let n = checked(&p, &model, &zeros, &eqs, &products, &quads);
                answer = Some(
                    json!({"model":model,"violated_relation_index":j,"violated_quadratic":qv,"checked_original_scalar_equations":n,"search_nodes":visited}),
                );
                break;
            }
            failed.push(json!([j, visited]));
        }
        println!(
            "Q14:graph{graph},{tried} output pairs,{} degree4 colon instances,counterpoint {}",
            rels.len(),
            answer.is_some()
        );
        records.push(json!({"graph":graph,"pair_candidates_examined":tried,"pair_cap_reached":capped,"relations":rels,"counterpoint":answer,"failed_queries":failed,"empty_family":rels.is_empty(),"full_h_saturation":false}));
    }
    round02::save(
        dir,
        "q14-two-product-colon.json",
        &json!({"schema":"koblitz.round13/q14-v1","records":records,"full_h_saturation":false,"scope":"actual nonconstant four-leaf multiplication pairs;not a complete degree4 Macaulay or saturation computation"}),
    );
}
pub fn certify(dir: &Path) {
    let data = crate::round04::read(&dir.join("q13-inverse-associative-saturation.json"));
    let pairs = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let mut s=String::from("import KoblitzRound12Boundary\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound13Boundary\nopen KoblitzRound12Boundary\ndef sparseEval (g : Gate) (i : Nat) (z : List Nat) : Bool :=\n match g with\n | .input => true\n | .constant => !(z.contains i)\n | .add a b => z.contains i == ((z.contains a).xor (z.contains b))\n | .mul a b => !((z.contains a) && (z.contains b))\n | .frob a => z.contains i == z.contains a\n | .inv a => !((z.contains a) && (z.contains i))\ndef sparseCheck (p : Array Gate) (z : List Nat) (r : Array Nat) : Bool :=\n (p.toList.zipIdx.all fun gi => sparseEval gi.1 gi.2 z) && (r.toList.all fun i => !(z.contains i))\ndef quad (z : List Nat) (q : Nat × Nat × Nat × Nat) : Bool := ((z.contains q.1) && (z.contains q.2.1)).xor ((z.contains q.2.2.1) && (z.contains q.2.2.2))\ndef extra (z : List Nat) (zeros : Array Nat) (eqs ps : Array (Nat × Nat)) (qs : List (Nat × Nat × Nat × Nat)) : Bool :=\n (zeros.toList.all fun i => !(z.contains i)) && (eqs.toList.all fun p => z.contains p.1 == z.contains p.2) && (ps.toList.all fun p => !((z.contains p.1) && (z.contains p.2))) && (qs.all fun q => !(quad z q))\n");
    let mut names = vec![];
    for graph in 0..4 {
        let r = &data["records"][graph];
        let zeros: Vec<usize> = serde_json::from_value(r["boundary_zero_nodes"].clone()).unwrap();
        let eqs: Vec<[usize; 2]> =
            serde_json::from_value(r["boundary_equalities"].clone()).unwrap();
        let mut inv: Vec<[usize; 2]> = vec![];
        for q in r["inverse_relations"].as_array().unwrap() {
            inv.push([
                q["out"].as_u64().unwrap() as usize,
                q["inverse"].as_u64().unwrap() as usize,
            ]);
        }
        let qs: Vec<Quad> = r["shared_factor_relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| serde_json::from_value(q["quadratic_at_h0"].clone()).unwrap())
            .collect();
        let ints = |v: &Vec<usize>| {
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        let tuples = |v: &Vec<[usize; 2]>| {
            v.iter()
                .map(|q| format!("({},{})", q[0], q[1]))
                .collect::<Vec<_>>()
                .join(",")
        };
        s.push_str(&format!("def zeros{graph} : Array Nat := #[{}]\ndef eqs{graph} : Array (Nat × Nat) := #[{}]\ndef inv{graph} : Array (Nat × Nat) := #[{}]\ndef qs{graph} : List (Nat × Nat × Nat × Nat) := [{}]\n",ints(&zeros),tuples(&eqs),tuples(&inv),qs.iter().map(|q|format!("({},{},{},{})",q[0],q[1],q[2],q[3])).collect::<Vec<_>>().join(",")));
        let candidates = [
            &r["stages"][0]["counterpoint"],
            &r["stages"][1]["counterpoint"],
            &pairs["records"][graph]["counterpoint"],
        ];
        for (stage, c) in candidates.into_iter().enumerate() {
            if c.is_null() {
                continue;
            }
            let model: Vec<u8> = serde_json::from_value(c["model"].clone()).unwrap();
            let q: Quad = serde_json::from_value(c["violated_quadratic"].clone()).unwrap();
            let tag = format!("graph{graph}_stage{stage}");
            s.push_str(&format!(
                "def {tag}_model : List Nat := [{}]\n",
                model
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| **x == 1)
                    .map(|(i, _)| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            let ip = if stage == 0 {
                "#[]".into()
            } else {
                format!("inv{graph}")
            };
            let qq = if stage < 2 {
                "[]".into()
            } else {
                format!("qs{graph}")
            };
            s.push_str(&format!("theorem {tag}_passes : (sparseCheck program{graph} {tag}_model residual{graph} && extra {tag}_model zeros{graph} eqs{graph} {ip} {qq})=true := by decide +kernel\ntheorem {tag}_missing : quad {tag}_model ({},{},{},{})=true := by decide +kernel\n",q[0],q[1],q[2],q[3]));
            names.extend([format!("{tag}_passes"), format!("{tag}_missing")]);
        }
    }
    s.push_str("end KoblitzRound13Boundary\n");
    for n in &names {
        s.push_str(&format!("#print axioms KoblitzRound13Boundary.{n}\n"));
    }
    let path = dir.join("KoblitzRound13Boundary.lean");
    assert!(!path.exists());
    std::fs::write(path, s).unwrap();
    println!(
        "Q13/Q14:{} actual finite boundary Lean certificates emitted",
        names.len()
    );
}
