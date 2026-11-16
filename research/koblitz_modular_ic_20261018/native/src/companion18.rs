//! Companion source-bound separators for the four actual Q24 points.
use super::*;
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let old = read(&dir.join("q24-after-all-ordinary-cube-repairs.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound17Boundary\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound18Companion\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let model = &old["records"][g]["model"];
        let mut family = vec![];
        for input in [8usize] {
            let mut memo = BTreeMap::new();
            let mut cube = None;
            let mut quartic = None;
            for i in 0..p["nodes"].as_array().unwrap().len() {
                if let Some(v) = form(&f, p, i, input, &mut memo) {
                    if cube.is_none()
                        && v.d == 3
                        && v.p.len() == 1
                        && v.p.get(&(0, 3)) == Some(&f.o())
                    {
                        cube = Some((i, v.clone()));
                    }
                    if quartic.is_none()
                        && v.d == 4
                        && v.p.contains_key(&(0, 4))
                        && v.p.keys().all(|m| m.1 > 0)
                    {
                        quartic = Some((i, v));
                    }
                    if cube.is_some() && quartic.is_some() {
                        break;
                    }
                }
            }
            let (u, cu) = cube.expect("source cube");
            let (v, qu) = quartic.expect("source quartic");
            let depth = 4 + cu.k.max(qu.k);
            let c = (0..4)
                .map(|k| qu.p.get(&(3 - k, k + 1)).cloned().unwrap_or_else(|| f.z()))
                .collect::<Vec<_>>();
            assert_ne!(c[3], f.z());
            let mul = |a: &E, b: &E| f.mul(a, b);
            let cb = |a: &E| mul(&f.sq(a), a);
            let coeff = vec![
                ([0, 4, 0], cb(&c[3])),
                (
                    [1, 3, 0],
                    mul(&f.sq(&c[3]), &c[0])
                        .add(&cb(&c[2]))
                        .add(&mul(&mul(&c[1], &c[3]), &c[2])),
                ),
                (
                    [2, 2, 0],
                    mul(&c[3], &f.sq(&c[0]))
                        .add(&cb(&c[1]))
                        .add(&mul(&mul(&c[1], &c[0]), &c[2])),
                ),
                ([3, 1, 0], cb(&c[0])),
                ([1, 2, 1], f.sq(&c[2]).add(&mul(&c[1], &c[3]))),
                ([1, 1, 2], c[2].clone()),
                ([1, 0, 3], f.o()),
                ([2, 1, 1], mul(&c[1], &c[0])),
            ];
            let bit = (0..360).find(|b| coeff[0].1.to_biguint().bit(*b)).unwrap();
            let chosen = coeff
                .iter()
                .filter(|(_, c)| c.to_biguint().bit(bit))
                .map(|(m, _)| *m)
                .collect::<Vec<_>>();
            let reject = model[u] == 1;
            let a = input / 2;
            let expression = chosen
                .iter()
                .map(|m| format!("h^{}*u^{}*v^{}", m[0], m[1], m[2]))
                .collect::<Vec<_>>()
                .join(" + ");
            lean.push_str(&format!("def sep{g}_{a} (h u v:ZMod 2) : ZMod 2 := {expression}\ntheorem sep{g}_{a}_boundary (v:ZMod 2) : sep{g}_{a} 0 1 v=1 := by simp [sep{g}_{a}]\n"));
            if reject {
                lean.push_str(&format!("theorem actual_Q24_unit{g} : (KoblitzRound17Boundary.point{g}.contains {u})=true := by decide +kernel\n"));
                let args = &p["nodes"][v]["args"];
                assert_eq!(args[0], input);
                lean.push_str(&format!("theorem actual_quartic_gate{g} : KoblitzRound12Boundary.program{g}[{v}]?=some (.mul {input} {}) := by decide +kernel\n",args[1]));
            }
            family.push(json!({"input":input,"cube_node":u,"quartic_node":v,"cube_h_depth":cu.k,"quartic_h_depth":qu.k,"separator_h_depth":depth,"coefficients_c0_c1_c2_c3":c.iter().map(|v|v.to_biguint().to_str_radix(16)).collect::<Vec<_>>(),"selected_bit":bit,"selected_F2_quartic":chosen,"actual_Q24_unit":model[u],"rejects_Q24_model":reject,"source_certificate":memo.iter().filter_map(|(i,v)|v.as_ref().map(|v|json!({"node":i,"gate":p["nodes"][*i],"degree":v.d,"h_depth":v.k,"terms":v.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>()}));
        }
        assert!(family.iter().any(|r| r["rejects_Q24_model"] == true));
        println!("Q25:graph{g},actual companion quartic excludes Q24");
        records.push(
            json!({"graph":g,"family":family,"Q24_model_excluded":true,"full_h_saturation":false}),
        );
    }
    lean.push_str("end KoblitzRound18Companion\n");
    for g in 0..4 {
        for a in [4] {
            lean.push_str(&format!(
                "#print axioms KoblitzRound18Companion.sep{g}_{a}_boundary\n"
            ));
        }
        for n in ["actual_Q24_unit", "actual_quartic_gate"] {
            lean.push_str(&format!("#print axioms KoblitzRound18Companion.{n}{g}\n"));
        }
    }
    let out = dir.join("KoblitzRound18Companion.lean");
    assert!(!out.exists());
    std::fs::write(out, lean).unwrap();
    crate::round02::save(
        dir,
        "q25-companion-source-separators.json",
        &json!({"schema":"koblitz.round18/q25-v1","records":records,"scope":"actual companion source-bound membership at depth4+max(source depths) via frozen universal residual;four Q24 exclusions,not full saturation"}),
    );
}
