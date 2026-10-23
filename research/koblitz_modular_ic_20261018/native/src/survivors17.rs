//! General source-DAG normalization on one input; no Frobenius valuation assumption.
use crate::{
    large_field::{E, F},
    round04::read,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone)]
struct Form {
    d: usize,
    k: usize,
    p: BTreeMap<(usize, usize), E>,
}
fn insert(f: &F, p: &mut BTreeMap<(usize, usize), E>, m: (usize, usize), v: E) {
    let v = p.get(&m).map(|u| u.add(&v)).unwrap_or(v);
    if v == f.z() {
        p.remove(&m);
    } else {
        p.insert(m, v);
    }
}
fn form(
    f: &F,
    p: &Value,
    i: usize,
    input: usize,
    memo: &mut BTreeMap<usize, Option<Form>>,
) -> Option<Form> {
    if let Some(x) = memo.get(&i) {
        return x.clone();
    }
    let n = &p["nodes"][i];
    let a = &n["args"];
    let mut z = Form {
        d: 1,
        k: 0,
        p: BTreeMap::new(),
    };
    match n["op"].as_str().unwrap() {
        "Input" => {
            if a.as_u64().unwrap() as usize != input {
                memo.insert(i, None);
                return None;
            }
            z.p.insert((0, 1), f.o());
        }
        "Constant" => {
            let c = E::from_biguint(
                &BigUint::parse_bytes(a.as_str().unwrap().as_bytes(), 16).unwrap(),
                360,
            );
            if c != f.z() {
                z.p.insert((1, 0), c);
            }
        }
        "Add" | "Mul" => {
            let x = form(f, p, a[0].as_u64().unwrap() as usize, input, memo);
            let y = form(f, p, a[1].as_u64().unwrap() as usize, input, memo);
            let (Some(x), Some(y)) = (x, y) else {
                memo.insert(i, None);
                return None;
            };
            z.k = x.k.max(y.k);
            if n["op"] == "Add" {
                z.d = x.d.max(y.d);
                for v in [&x, &y] {
                    for ((h, x), c) in &v.p {
                        insert(f, &mut z.p, (*h + z.d - v.d, *x), c.clone());
                    }
                }
            } else {
                z.d = x.d + y.d;
                for ((h, x), c) in &x.p {
                    for ((hh, xx), cc) in &y.p {
                        insert(f, &mut z.p, (h + hh, x + xx), f.mul(c, cc));
                    }
                }
            }
            let r =
                z.p.keys()
                    .map(|m| m.0)
                    .min()
                    .unwrap_or(z.d - 1)
                    .min(z.d - 1);
            if r > 0 {
                z.d -= r;
                z.k += r;
                z.p = z.p.into_iter().map(|((h, x), c)| ((h - r, x), c)).collect();
            }
        }
        _ => {
            memo.insert(i, None);
            return None;
        }
    }
    assert!(z.d <= 32 && z.p.len() <= 4096 && memo.len() < 100000);
    memo.insert(i, Some(z.clone()));
    Some(z)
}
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let old = read(&dir.join("q22-after-quartic-boundary-repair.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound16Boundary\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound17Survivors\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let model = &old["records"][g]["model"];
        let mut family = vec![];
        for input in [0usize, 2, 4, 6] {
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
            assert_eq!((cu.k, qu.k), (2, 2));
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
                lean.push_str(&format!("theorem actual_Q22_unit{g} : (KoblitzRound16Boundary.point{g}.contains {u})=true := by decide +kernel\n"));
                let args = &p["nodes"][v]["args"];
                assert_eq!(args[0], input);
                lean.push_str(&format!("theorem actual_quartic_gate{g} : KoblitzRound12Boundary.program{g}[{v}]?=some (.mul {input} {}) := by decide +kernel\n",args[1]));
            }
            family.push(json!({"input":input,"cube_node":u,"quartic_node":v,"cube_h_depth":cu.k,"quartic_h_depth":qu.k,"separator_h_depth":6,"coefficients_c0_c1_c2_c3":c.iter().map(|v|v.to_biguint().to_str_radix(16)).collect::<Vec<_>>(),"selected_bit":bit,"selected_F2_quartic":chosen,"actual_Q22_unit":model[u],"rejects_Q22_model":reject,"source_certificate":memo.iter().filter_map(|(i,v)|v.as_ref().map(|v|json!({"node":i,"gate":p["nodes"][*i],"degree":v.d,"h_depth":v.k,"terms":v.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>()}));
        }
        assert!(family.iter().any(|r| r["rejects_Q22_model"] == true));
        println!("Q23:graph{g},four actual depth6 quartics;Q22 survivor excluded");
        records.push(
            json!({"graph":g,"family":family,"Q22_model_excluded":true,"full_h_saturation":false}),
        );
    }
    lean.push_str("end KoblitzRound17Survivors\n");
    for g in 0..4 {
        for a in 0..4 {
            lean.push_str(&format!(
                "#print axioms KoblitzRound17Survivors.sep{g}_{a}_boundary\n"
            ));
        }
        for n in ["actual_Q22_unit", "actual_quartic_gate"] {
            lean.push_str(&format!("#print axioms KoblitzRound17Survivors.{n}{g}\n"));
        }
    }
    let out = dir.join("KoblitzRound17Survivors.lean");
    assert!(!out.exists());
    std::fs::write(out, lean).unwrap();
    crate::round02::save(
        dir,
        "q23-survivor-source-separators.json",
        &json!({"schema":"koblitz.round17/q23-v1","records":records,"scope":"actual source-bound h6 membership via Round16 universal residual;four Q22 exclusions,not full saturation"}),
    );
}
