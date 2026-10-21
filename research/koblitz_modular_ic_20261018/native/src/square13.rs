//! Characteristic-two squares of actual DAG linear forms, with explicit h-depth.
use super::*;
#[derive(Clone)]
struct Form {
    terms: BTreeMap<usize, E>,
    depth: usize,
}
fn insert(f: &F, m: &mut BTreeMap<usize, E>, i: usize, c: E) {
    let c = m.get(&i).map(|v| v.add(&c)).unwrap_or(c);
    if c == f.z() {
        m.remove(&i);
    } else {
        m.insert(i, c);
    }
}
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    let previous = crate::round04::read(&dir.join("q13-inverse-associative-saturation.json"));
    let pair = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let constants = crate::round04::read(&dir.join("q10c-constant-gate-saturation-step.json"));
    let mut records = vec![];
    for graph in 0..4 {
        let inp = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(inp["program"].clone()).unwrap();
        let h = p.nodes.len();
        let mut known = BTreeMap::new();
        for r in constants["records"][graph]["evaluated_constant_nodes"]
            .as_array()
            .unwrap()
        {
            known.insert(
                r["node"].as_u64().unwrap() as usize,
                (
                    E::from_biguint(
                        &BigUint::parse_bytes(r["scalar"].as_str().unwrap().as_bytes(), 16)
                            .unwrap(),
                        360,
                    ),
                    r["h_power_membership_depth"].as_u64().unwrap() as usize,
                ),
            );
        }
        let squares: BTreeMap<_, _> = p
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, op)| match op {
                Op::Mul(a, b) if a == b => Some((*a, i)),
                _ => None,
            })
            .collect();
        let mut forms: Vec<Form> = vec![];
        let mut capped = 0;
        for (i, op) in p.nodes.iter().enumerate() {
            let form = if let Some((c, depth)) = known.get(&i) {
                let mut terms = BTreeMap::new();
                insert(&f, &mut terms, h, c.clone());
                Form {
                    terms,
                    depth: *depth,
                }
            } else {
                match op {
                    Op::Add(a, b) => {
                        let mut z = forms[*a].clone();
                        z.depth = z.depth.max(forms[*b].depth);
                        for (j, c) in &forms[*b].terms {
                            insert(&f, &mut z.terms, *j, c.clone());
                        }
                        z
                    }
                    Op::Mul(a, b) if known.contains_key(a) || known.contains_key(b) => {
                        let (cn, x) = if known.contains_key(a) {
                            (*a, *b)
                        } else {
                            (*b, *a)
                        };
                        let (c, k) = &known[&cn];
                        let mut z = forms[x].clone();
                        z.depth = z.depth.max(*k) + 1;
                        z.terms = z
                            .terms
                            .into_iter()
                            .map(|(j, v)| (j, f.mul(c, &v)))
                            .filter(|(_, v)| *v != f.z())
                            .collect();
                        z
                    }
                    _ => Form {
                        terms: BTreeMap::from([(i, f.o())]),
                        depth: 0,
                    },
                }
            };
            forms.push(if form.terms.len() > 64 {
                capped += 1;
                Form {
                    terms: BTreeMap::from([(i, f.o())]),
                    depth: 0,
                }
            } else {
                form
            });
        }
        let mut rels = vec![];
        let mut unavailable = 0;
        let mut trivial = 0;
        for (a, out) in &squares {
            let form = &forms[*a];
            if form
                .terms
                .keys()
                .any(|j| *j != h && !squares.contains_key(j))
            {
                unavailable += 1;
                continue;
            }
            let mut terms = BTreeMap::from([(*out, f.o())]);
            for (j, c) in &form.terms {
                insert(
                    &f,
                    &mut terms,
                    if *j == h { h } else { squares[j] },
                    f.sq(c),
                );
            }
            if terms.is_empty() {
                trivial += 1;
                continue;
            }
            let terms: Vec<_> = terms
                .into_iter()
                .filter(|(j, _)| *j != h)
                .map(|(j, c)| json!([j, c.to_biguint().to_str_radix(16)]))
                .collect();
            rels.push(json!({"square_input":a,"square_output":out,"linear_form":form.terms.iter().map(|(j,c)|json!([j,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"h_node":h,"boundary_terms":terms,"h_membership_power":2*form.depth+1}));
        }
        let r = &previous["records"][graph];
        let zeros: Vec<usize> = serde_json::from_value(r["boundary_zero_nodes"].clone()).unwrap();
        let eqs: Vec<[usize; 2]> =
            serde_json::from_value(r["boundary_equalities"].clone()).unwrap();
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
            serde_json::from_value(r["original_zero_product_pairs"].clone()).unwrap();
        for q in r["inverse_relations"].as_array().unwrap() {
            products.push((
                q["out"].as_u64().unwrap() as usize,
                q["inverse"].as_u64().unwrap() as usize,
            ));
        }
        let mut qs: Vec<Quad> = r["shared_factor_relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| serde_json::from_value(q["quadratic_at_h0"].clone()).unwrap())
            .collect();
        for q in pair["records"][graph]["relations"].as_array().unwrap() {
            qs.push(serde_json::from_value(q["outputs"].clone()).unwrap());
        }
        let equations: Vec<Vec<Vec<usize>>> = rels
            .iter()
            .map(|r| {
                let ts: Vec<_> = r["boundary_terms"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|t| {
                        (
                            t[0].as_u64().unwrap() as usize,
                            BigUint::parse_bytes(t[1].as_str().unwrap().as_bytes(), 16).unwrap(),
                        )
                    })
                    .collect();
                (0..360)
                    .map(|b| {
                        ts.iter()
                            .filter(|(_, c)| c.bit(b))
                            .map(|(i, _)| *i)
                            .collect()
                    })
                    .collect()
            })
            .collect();
        let mut answer = None;
        let mut failed = vec![];
        'query: for (j, eqs) in equations.iter().enumerate() {
            for (b, e) in eqs.iter().enumerate() {
                if e.is_empty() {
                    continue;
                }
                let mut ll = l.clone();
                let mut visited = 0;
                if ll.equation(e, 1) {
                    if let Some(z) = search_all(ll, &products, &qs, &mut visited) {
                        let n = checked(&p, &z, &zeros, &eqs_from(r), &products, &qs);
                        assert_eq!(e.iter().fold(0, |a, i| a ^ z[*i]), 1);
                        answer = Some(
                            json!({"model":z,"violated_relation_index":j,"violated_coefficient":b,"checked_original_scalar_equations":n,"search_nodes":visited}),
                        );
                        break 'query;
                    }
                }
                failed.push(json!([j, b, visited, visited > 100000]));
            }
        }
        let mut consistent = true;
        for eqs in &equations {
            for e in eqs {
                if !l.equation(e, 0) {
                    consistent = false;
                }
            }
        }
        let mut visited = 0;
        let mut nonzero_queries = vec![];
        let mut model = None;
        if consistent {
            for i in 0..h {
                let mut ll = l.clone();
                let mut count = 0;
                if ll.equation(&[i], 1) {
                    model = search_all(ll, &products, &qs, &mut count);
                }
                visited += count;
                nonzero_queries.push(json!([i, count, count > 100000]));
                if model.is_some() {
                    break;
                }
            }
        }
        if let Some(z) = &model {
            checked(&p, z, &zeros, &eqs, &products, &qs);
            for es in &equations {
                for e in es {
                    assert_eq!(e.iter().fold(0, |a, i| a ^ z[*i]), 0);
                }
            }
        }
        println!("Q15:graph{graph},{} square gates,{} relations,{} trivial,{} unavailable;counterpoint{},after-repair-model{}",squares.len(),rels.len(),trivial,unavailable,answer.is_some(),model.is_some());
        records.push(json!({"graph":graph,"square_gates":squares.len(),"form_cap_hits":capped,"unavailable_atom_squares":unavailable,"trivial_relations":trivial,"relations":rels,"counterpoint_after_all_Q14":answer,"failed_queries":failed,"after_all_Q15":{"scalar_linear_consistent":consistent,"model":model,"search_nodes":visited,"nonzero_queries":nonzero_queries,"cap_reached":nonzero_queries.iter().any(|q|q[2]==true),"interpretation":"surviving restricted boundary models alone do not prove failure of saturation"},"full_h_saturation":false}));
    }
    round02::save(
        dir,
        "q15-linear-square-saturation.json",
        &json!({"schema":"koblitz.round13/q15-v1","records":records,"form_atom_cap":64,"full_h_saturation":false,"scope":"certified characteristic-two square-of-linear-form family;not whole colon or saturation"}),
    );
}
fn eqs_from(r: &Value) -> Vec<[usize; 2]> {
    serde_json::from_value(r["boundary_equalities"].clone()).unwrap()
}

pub fn certify(dir: &Path) {
    let data = crate::round04::read(&dir.join("q15-linear-square-saturation.json"));
    let pair = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let mut s=String::from("import KoblitzRound13Boundary\nimport KoblitzRound13Square\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound13SquareData\nopen KoblitzRound12Boundary KoblitzRound13Boundary\ndef parity (z : List Nat) (xs : List Nat) : Bool := xs.foldl (fun a i => a.xor (z.contains i)) false\n");
    let mut names = vec![];
    for graph in 0..4 {
        let r = &data["records"][graph];
        let c = &r["counterpoint_after_all_Q14"];
        assert!(!c.is_null());
        let model: Vec<u8> = serde_json::from_value(c["model"].clone()).unwrap();
        let rel = &r["relations"][c["violated_relation_index"].as_u64().unwrap() as usize];
        let bit = c["violated_coefficient"].as_u64().unwrap();
        let indices: Vec<_> = rel["boundary_terms"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|t| {
                BigUint::parse_bytes(t[1].as_str().unwrap().as_bytes(), 16)
                    .unwrap()
                    .bit(bit)
            })
            .map(|t| t[0].as_u64().unwrap())
            .collect();
        let qs: Vec<Quad> = pair["records"][graph]["relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| serde_json::from_value(q["outputs"].clone()).unwrap())
            .collect();
        s.push_str(&format!("def model{graph} : List Nat := [{}]\ndef pairs{graph} : List (Nat × Nat × Nat × Nat) := [{}]\n",model.iter().enumerate().filter(|(_,x)|**x==1).map(|(i,_)|i.to_string()).collect::<Vec<_>>().join(","),qs.iter().map(|q|format!("({},{},{},{})",q[0],q[1],q[2],q[3])).collect::<Vec<_>>().join(",")));
        s.push_str(&format!("theorem graph{graph}_passes_all_Q14 : (sparseCheck program{graph} model{graph} residual{graph} && extra model{graph} zeros{graph} eqs{graph} inv{graph} (qs{graph}++pairs{graph}))=true := by decide +kernel\ntheorem graph{graph}_missing_square_relation : parity model{graph} [{}]=true := by decide +kernel\n",indices.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")));
        names.extend([
            format!("graph{graph}_passes_all_Q14"),
            format!("graph{graph}_missing_square_relation"),
        ]);
    }
    s.push_str("end KoblitzRound13SquareData\n");
    for n in names {
        s.push_str(&format!("#print axioms KoblitzRound13SquareData.{n}\n"));
    }
    let path = dir.join("KoblitzRound13SquareData.lean");
    assert!(!path.exists());
    std::fs::write(path, s).unwrap();
    println!("Q15:8 actual all-Q14-passing and square-violating certificates emitted");
}
