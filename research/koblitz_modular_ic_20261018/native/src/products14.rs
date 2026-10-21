//! Certified products of the actual expanded K-linear forms.
use super::*;
fn coefficients(rel: &Value) -> Vec<Vec<usize>> {
    let mut equations = BTreeSet::new();
    for bit in 0..360 {
        let e = rel["boundary_terms"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|t| {
                BigUint::parse_bytes(t[1].as_str().unwrap().as_bytes(), 16)
                    .unwrap()
                    .bit(bit)
            })
            .map(|t| t[0].as_u64().unwrap() as usize)
            .collect::<Vec<_>>();
        if !e.is_empty() {
            equations.insert(e);
        }
    }
    equations.into_iter().collect()
}
pub fn run(dir: &Path) {
    run_all(dir, false);
}
pub fn quadrics(dir: &Path) {
    run_all(dir, true);
}
fn run_all(dir: &Path, quadratic: bool) {
    let f = F::new(360, 45);
    let prev = crate::round04::read(&dir.join("q13-inverse-associative-saturation.json"));
    let pair = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let sq = crate::round04::read(&dir.join("q15-linear-square-saturation.json"));
    let constants = crate::round04::read(&dir.join("q10c-constant-gate-saturation-step.json"));
    let mut records = vec![];
    for graph in 0..4 {
        let input = crate::round04::read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p: Program = serde_json::from_value(input["program"].clone()).unwrap();
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
        let mut muls = BTreeMap::new();
        for (i, op) in p.nodes.iter().enumerate() {
            if let Op::Mul(a, b) = op {
                muls.entry([(*a).min(*b), (*a).max(*b)]).or_insert(i);
            }
        }
        let mut rels = vec![];
        let mut unavailable = 0;
        let mut trivial = 0;
        let mut examined = 0;
        for (out, op) in p.nodes.iter().enumerate() {
            let Op::Mul(a, b) = op else { continue };
            examined += 1;
            let mut terms = BTreeMap::from([(out, f.o())]);
            let mut absent = false;
            let mut pairs = vec![];
            for (i, c) in &forms[*a].terms {
                for (j, d) in &forms[*b].terms {
                    let r = if *i == h {
                        Some(*j)
                    } else if *j == h {
                        Some(*i)
                    } else {
                        muls.get(&[(*i).min(*j), (*i).max(*j)]).copied()
                    };
                    if let Some(r) = r {
                        insert(&f, &mut terms, r, f.mul(c, d));
                        pairs.push(json!([i, j, r]));
                    } else {
                        absent = true;
                    }
                }
            }
            if absent {
                unavailable += 1;
                continue;
            }
            if terms.is_empty() {
                trivial += 1;
                continue;
            }
            rels.push(json!({"output":out,"inputs":[a,b],"input_forms":[forms[*a].terms.iter().map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),forms[*b].terms.iter().map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()],"product_representations":pairs,"h_node":h,"full_terms":terms.iter().map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"boundary_terms":terms.iter().filter(|(i,_)|**i!=h).map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"h_membership_power":2*forms[*a].depth.max(forms[*b].depth)+1}));
        }
        let old = &prev["records"][graph];
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
            pair["records"][graph]["relations"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| serde_json::from_value::<Quad>(r["outputs"].clone()).unwrap()),
        );
        let prior: Vec<Vec<usize>> = sq["records"][graph]["relations"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(coefficients)
            .collect();
        for e in &prior {
            assert!(l.equation(e, 0));
        }
        let new: Vec<Vec<Vec<usize>>> = rels.iter().map(coefficients).collect();
        if quadratic {
            for es in &new {
                for e in es {
                    assert!(l.equation(e, 0));
                }
            }
            records.push(quad_record(
                &p, &f, &forms, h, graph, l, &products, &qs, &prior, &new,
            ));
            continue;
        }
        let mut counter = None;
        let mut failed = vec![];
        'queries: for (ri, es) in new.iter().enumerate() {
            for e in es {
                let mut ll = l.clone();
                let mut visited = 0;
                let z = if ll.equation(e, 1) {
                    search_all(ll, &products, &qs, &mut visited)
                } else {
                    None
                };
                if let Some(z) = z {
                    checked(&p, &z, &zeros, &eqs, &products, &qs);
                    assert!(prior
                        .iter()
                        .all(|e| e.iter().fold(0, |a, i| a ^ z[*i]) == 0));
                    assert_eq!(e.iter().fold(0, |a, i| a ^ z[*i]), 1);
                    counter = Some(
                        json!({"model":z,"relation_index":ri,"violated_indices":e,"search_nodes":visited}),
                    );
                    break 'queries;
                }
                failed
                    .push(json!({"relation":ri,"indices":e,"nodes":visited,"cap":visited>100000}));
            }
        }
        let mut consistent = true;
        for es in &new {
            for e in es {
                consistent &= l.equation(e, 0);
            }
        }
        let mut model = None;
        let mut queries = vec![];
        if consistent {
            for i in 0..h {
                let mut ll = l.clone();
                let mut visited = 0;
                if ll.equation(&[i], 1) {
                    model = search_all(ll, &products, &qs, &mut visited);
                }
                queries.push(json!([i, visited, visited > 100000]));
                if model.is_some() {
                    break;
                }
            }
        }
        if let Some(z) = &model {
            checked(&p, z, &zeros, &eqs, &products, &qs);
            for e in prior.iter().chain(new.iter().flatten()) {
                assert_eq!(e.iter().fold(0, |a, i| a ^ z[*i]), 0);
            }
        }
        println!("Q16:graph{graph},{examined} multiplications,{} relations,{trivial} trivial,{unavailable} unavailable,counter{},survivor{}",rels.len(),counter.is_some(),model.is_some());
        records.push(json!({"graph":graph,"multiplication_gates":examined,"relations":rels,"trivial":trivial,"unavailable":unavailable,"form_cap_hits":capped,"counterpoint_after_all_Q15":counter,"failed_queries":failed,"after_all_Q16":{"linear_consistent":consistent,"model":model,"queries":queries},"full_h_saturation":false}));
    }
    if quadratic {
        round02::save(
            dir,
            "q17-expanded-product-quadrics.json",
            &json!({"schema":"koblitz.round14/q17-v1","records":records,"full_h_saturation":false}),
        );
        return;
    }
    round02::save(
        dir,
        "q16-linear-product-saturation.json",
        &json!({"schema":"koblitz.round14/q16-v1","records":records,"full_h_saturation":false}),
    );
}

type Multi = (Vec<(usize, usize)>, u8);
pub fn certify(dir: &Path) {
    let q17 = crate::round04::read(&dir.join("q17-expanded-product-quadrics.json"));
    let q16 = crate::round04::read(&dir.join("q16-linear-product-saturation.json"));
    let q15 = crate::round04::read(&dir.join("q15-linear-square-saturation.json"));
    let q14 = crate::round04::read(&dir.join("q14-two-product-colon.json"));
    let mut s=String::from("import KoblitzRound14\nimport KoblitzRound13SquareData\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound14Boundary\nopen KoblitzRound12Boundary KoblitzRound13Boundary KoblitzRound13SquareData\ndef multi (z : List Nat) (m : List (Nat × Nat)) : Bool := m.foldl (fun v ab => v.xor ((z.contains ab.1) && (z.contains ab.2))) false\n");
    for graph in 0..4 {
        let model: Vec<u8> =
            serde_json::from_value(q17["records"][graph]["after_all_Q17"]["model"].clone())
                .unwrap();
        let indices = model
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 1)
            .map(|(i, _)| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let lin: BTreeSet<Vec<usize>> = q15["records"][graph]["relations"]
            .as_array()
            .unwrap()
            .iter()
            .chain(
                q16["records"][graph]["relations"]
                    .as_array()
                    .unwrap()
                    .iter(),
            )
            .flat_map(coefficients)
            .collect();
        let polys: Vec<Vec<(usize, usize)>> =
            serde_json::from_value(q17["records"][graph]["coefficient_equations"].clone()).unwrap();
        let ps: Vec<Quad> = q14["records"][graph]["relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| serde_json::from_value(r["outputs"].clone()).unwrap())
            .collect();
        s.push_str(&format!("def point{graph} : List Nat := [{indices}]\ndef linear{graph} : List (List Nat) := [{}]\ndef expanded{graph} : List (List (Nat × Nat)) := [{}]\ndef pairs{graph} : List (Nat × Nat × Nat × Nat) := [{}]\n",lin.iter().map(|e|format!("[{}]",e.iter().map(ToString::to_string).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(","),polys.iter().map(|e|format!("[{}]",e.iter().map(|(a,b)|format!("({a},{b})")).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(","),ps.iter().map(|q|format!("({},{},{},{})",q[0],q[1],q[2],q[3])).collect::<Vec<_>>().join(",")));
        s.push_str(&format!("theorem graph{graph}_all_Q17 : (sparseCheck program{graph} point{graph} residual{graph} && extra point{graph} zeros{graph} eqs{graph} inv{graph} (qs{graph}++pairs{graph}) && (linear{graph}.all fun e => !(parity point{graph} e)) && (expanded{graph}.all fun e => !(multi point{graph} e)))=true := by decide +kernel\ntheorem graph{graph}_nonzero : point{graph}.isEmpty=false := by decide +kernel\n"));
    }
    s.push_str("end KoblitzRound14Boundary\n");
    for g in 0..4 {
        for n in ["all_Q17", "nonzero"] {
            s.push_str(&format!(
                "#print axioms KoblitzRound14Boundary.graph{g}_{n}\n"
            ));
        }
    }
    let path = dir.join("KoblitzRound14Boundary.lean");
    assert!(!path.exists());
    std::fs::write(path, s).unwrap();
    println!("Q17:8 kernel-evaluated boundary certificates emitted");
}
fn value(m: &Multi, z: &[u8]) -> u8 {
    m.0.iter().fold(0, |v, (a, b)| v ^ (z[*a] & z[*b]))
}
fn search_multi(
    l: Lin,
    products: &[(usize, usize)],
    qs: &[Quad],
    polys: &[Multi],
    visited: &mut usize,
) -> Option<Vec<u8>> {
    let z = search_all(l.clone(), products, qs, visited)?;
    let Some(bad) = polys.iter().find(|p| value(p, &z) != p.1) else {
        return Some(z);
    };
    for i in bad
        .0
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<BTreeSet<_>>()
    {
        let mut opposite = l.clone();
        if !opposite.equation(&[i], z[i] ^ 1) {
            continue;
        }
        for b in [0, 1] {
            let mut ll = l.clone();
            if ll.equation(&[i], b) {
                if let Some(z) = search_multi(ll, products, qs, polys, visited) {
                    return Some(z);
                }
            }
            if *visited > 100000 {
                return None;
            }
        }
        return None;
    }
    None
}
#[allow(clippy::too_many_arguments)]
fn quad_record(
    p: &Program,
    f: &F,
    forms: &[Form],
    h: usize,
    graph: usize,
    l: Lin,
    products: &[(usize, usize)],
    qs: &[Quad],
    prior: &[Vec<usize>],
    linear: &[Vec<Vec<usize>>],
) -> Value {
    let mut rels = vec![];
    let mut unique: BTreeSet<Vec<(usize, usize)>> = BTreeSet::new();
    for (out, op) in p.nodes.iter().enumerate() {
        let Op::Mul(a, b) = op else { continue };
        let mut terms: BTreeMap<(usize, usize), E> = BTreeMap::new();
        for (i, c) in &forms[*a].terms {
            for (j, d) in &forms[*b].terms {
                let pair = ((*i).min(*j), (*i).max(*j));
                let v = f.mul(c, d);
                let v = terms.get(&pair).map(|x| x.add(&v)).unwrap_or(v);
                if v == f.z() {
                    terms.remove(&pair);
                } else {
                    terms.insert(pair, v);
                }
            }
        }
        let mut coeffs = BTreeSet::new();
        for bit in 0..360 {
            let e = terms
                .iter()
                .filter(|((i, j), c)| *i != h && *j != h && c.to_biguint().bit(bit))
                .map(|(ij, _)| *ij)
                .collect::<Vec<_>>();
            if !e.is_empty() {
                coeffs.insert(e.clone());
                unique.insert(e);
            }
        }
        rels.push(json!({"output":out,"inputs":[a,b],"input_forms":[forms[*a].terms.iter().map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),forms[*b].terms.iter().map(|(i,c)|json!([i,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()],"boundary_terms":terms.iter().filter(|((i,j),_)|*i!=h&&*j!=h).map(|((i,j),c)|json!([i,j,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"h_membership_power":forms[*a].depth.max(forms[*b].depth),"coefficient_equations":coeffs}));
    }
    let constraints: Vec<Multi> = unique.into_iter().map(|e| (e, 0)).collect();
    let mut counter = None;
    let mut failed = vec![];
    for (i, e) in constraints.iter().enumerate() {
        let mut count = 0;
        let z = search_multi(l.clone(), products, qs, &[(e.0.clone(), 1)], &mut count);
        if let Some(z) = z {
            counter = Some(
                json!({"model":z,"violated_equation_index":i,"violated_monomials":e.0,"search_nodes":count}),
            );
            break;
        }
        failed.push(json!([i, count, count > 100000]));
    }
    let mut survivor = None;
    let mut queries = vec![];
    for i in 0..h {
        let mut ll = l.clone();
        let mut count = 0;
        if ll.equation(&[i], 1) {
            survivor = search_multi(ll, products, qs, &constraints, &mut count);
        }
        queries.push(json!([i, count, count > 100000]));
        if survivor.is_some() {
            break;
        }
    }
    for z in counter
        .as_ref()
        .map(|r| serde_json::from_value::<Vec<u8>>(r["model"].clone()).unwrap())
        .iter()
        .chain(survivor.iter())
    {
        assert!(prior
            .iter()
            .chain(linear.iter().flatten())
            .all(|e| e.iter().fold(0, |a, i| a ^ z[*i]) == 0));
        for (i, op) in p.nodes.iter().enumerate() {
            let r = match op {
                Op::Input(_) => 0,
                Op::Constant(_) => z[i],
                Op::Add(a, b) => z[i] ^ z[*a] ^ z[*b],
                Op::Mul(a, b) => z[*a] & z[*b],
                Op::CoefficientFrobenius(a) => z[i] ^ z[*a],
                Op::Inverse(a) => z[i] & z[*a],
            };
            assert_eq!(r, 0);
        }
        for i in &p.residuals {
            assert_eq!(z[*i], 0);
        }
    }
    if let Some(z) = &survivor {
        assert!(constraints.iter().all(|m| value(m, z) == 0));
    }
    println!(
        "Q17:graph{graph},{} unique coefficient quadrics,counter{},survivor{}",
        constraints.len(),
        counter.is_some(),
        survivor.is_some()
    );
    json!({"graph":graph,"relations":rels,"coefficient_equations":constraints.iter().map(|m|&m.0).collect::<Vec<_>>(),"counterpoint_after_all_Q16":counter,"failed_queries":failed,"after_all_Q17":{"model":survivor,"queries":queries},"full_h_saturation":false})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multi_solver_matches_three_bit_exhaustion() {
        let polys = vec![
            vec![(0, 1)],
            vec![(0, 1), (0, 2), (1, 2)],
            vec![(0, 0), (1, 1), (2, 2)],
            vec![(0, 1), (0, 0)],
            vec![(0, 0), (1, 2)],
        ];
        for p in &polys {
            for q in &polys {
                for a in 0..2 {
                    for b in 0..2 {
                        let constraints = vec![(p.clone(), a), (q.clone(), b)];
                        let mut linear = Lin::new(3);
                        assert!(linear.equation(&[0, 2], 1));
                        let mut visited = 0;
                        let got = search_multi(linear, &[(0, 1)], &[], &constraints, &mut visited);
                        let brute = (0..8).any(|bits| {
                            let z = (0..3).map(|i| ((bits >> i) & 1) as u8).collect::<Vec<_>>();
                            (z[0] ^ z[2]) == 1
                                && (z[0] & z[1]) == 0
                                && constraints.iter().all(|m| value(m, &z) == m.1)
                        });
                        assert_eq!(got.is_some(), brute);
                        if let Some(z) = got {
                            assert!(constraints.iter().all(|m| value(m, &z) == m.1));
                        }
                    }
                }
            }
        }
    }
}
