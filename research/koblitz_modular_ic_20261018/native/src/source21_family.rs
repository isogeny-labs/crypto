use super::*;
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound21FinalBoundary\nnamespace KoblitzRound21Family\ndef topCheck (ts:List (Nat×Nat×Nat)) : Bool := decide (ts.filter (fun v => v.1=0)=[(0,9,0)])\n");
    for g in 0..4 {
        for (input, higher_node) in [(0, 95), (2, 155), (4, 215), (6, 275), (8, 335)] {
            let id = format!("{g}_{input}");
            let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
            let p = &doc["program"];
            let mut memo = BTreeMap::new();
            for i in 0..p["nodes"].as_array().unwrap().len() {
                let _ = form(&f, p, i, input, &mut memo);
            }
            let higher = uv(&f, memo[&higher_node].as_ref().unwrap());
            assert_eq!(higher.len(), 10);
            let mut unique = BTreeMap::<Vec<String>, (Vec<E>, Vec<usize>)>::new();
            for (i, z) in &memo {
                if let Some(z) = z {
                    if z.d < 9 && z.p.contains_key(&(0, z.d)) {
                        let a = uv(&f, z);
                        unique
                            .entry(coefficients(&a))
                            .or_insert_with(|| (a, vec![]))
                            .1
                            .push(*i);
                    }
                }
            }
            assert!(unique.len() <= 256);
            let mut family = vec![];
            let mut vanish: Vec<usize> = vec![];
            let mut exponentlists = vec![];
            for (_, (lower, aliases)) in unique {
                let mut q = higher
                    .iter()
                    .map(|c| {
                        if *c == f.z() {
                            Poly::new()
                        } else {
                            BTreeMap::from([((0, 0), c.clone())])
                        }
                    })
                    .collect::<Vec<_>>();
                plus(&mut q[0], &BTreeMap::from([((0, 1), f.o())]));
                let mut r = norm(&f, &lower, &q);
                assert!(r.keys().all(|(u, v)| u + v <= 9));
                assert!(r.keys().filter(|(u, v)| u + v == 9).eq([&(9, 0)]));
                let z = f.inv(&r[&(9, 0)]);
                r = scale(&f, &r, &z);
                let mut lp = vec![vec![f.o()]];
                let mut hp = lp.clone();
                for i in 1..=9 {
                    lp.push(upmul(&lp[i - 1], &lower, &f));
                    hp.push(upmul(&hp[i - 1], &higher, &f));
                }
                let mut residual = vec![];
                for ((u, v), c) in &r {
                    upadd(
                        &mut residual,
                        &upmul(&upmul(&lp[*u], &hp[*v], &f), &[c.clone()], &f),
                        &f,
                    );
                }
                assert!(residual.is_empty());
                let bitterms = r
                    .iter()
                    .filter(|(_, c)| c.to_biguint().bit(0))
                    .map(|((u, v), _)| (9 - u - v, *u, *v))
                    .collect::<Vec<_>>();
                assert_eq!(
                    bitterms
                        .iter()
                        .filter(|(h, _, _)| *h == 0)
                        .copied()
                        .collect::<Vec<_>>(),
                    vec![(0, 9, 0)]
                );
                exponentlists.push(format!(
                    "[{}]",
                    bitterms
                        .iter()
                        .map(|(h, u, v)| format!("({h},{u},{v})"))
                        .collect::<Vec<_>>()
                        .join(",")
                ));
                vanish.extend(&aliases);
                family.push(json!({"lower_nodes":aliases,"lower_coefficients":coefficients(&lower),"higher_node":higher_node,"degree":9,"normalized_homogeneous_terms":r.iter().map(|((u,v),c)|json!([9-u-v,u,v,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"selected_F2_terms":bitterms,"exact_univariate_substitution_zero":true}));
            }
            vanish.sort();
            vanish.dedup();

            lean.push_str(&format!("def family{id} : List (List (Nat×Nat×Nat)) := [{}]\ndef nodes{id} : List Nat := [{}]\ntheorem all_actual_tops{id} : family{id}.all topCheck=true := by decide +kernel\n",exponentlists.join(","),vanish.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")));
            println!(
                "Q39:g{g},input{input},{} exact relations;{} lower nodes",
                family.len(),
                vanish.len()
            );
            records.push(json!({"graph":g,"input":input,"higher_node":higher_node,"family":family,"radical_boundary_zero_nodes":vanish,"source_certificate":memo.iter().filter_map(|(i,z)|z.as_ref().map(|z|json!({"node":i,"gate":p["nodes"][*i],"degree":z.d,"h_depth":z.k,"terms":z.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>(),"full_saturation":false}));
        }
    }
    lean.push_str("end KoblitzRound21Family\n");
    for g in 0..4 {
        for input in [0, 2, 4, 6, 8] {
            lean.push_str(&format!(
                "#print axioms KoblitzRound21Family.all_actual_tops{g}_{input}\n"
            ));
        }
    }
    let path = dir.join("KoblitzRound21Family.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "q39-all-elliptic-pure-source-families.json",
        &json!({"schema":"koblitz.round21/q39-v1","records":records,"scope":"source-bound reduced boundary radical consequences from exact polynomial elimination;not full ideal saturation"}),
    );
}
