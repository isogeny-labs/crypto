//! Exact original-node homogeneous normalization and source-bound quartic separator.
use crate::{
    large_field::{E, F},
    round04::read,
};
use num_bigint::BigUint;
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
#[derive(Clone)]
struct Form {
    d: usize,
    k: usize,
    p: BTreeMap<(usize, usize), E>,
}
fn insert(f: &F, p: &mut BTreeMap<(usize, usize), E>, key: (usize, usize), c: E) {
    let c = p.get(&key).map(|v| v.add(&c)).unwrap_or(c);
    if c == f.z() {
        p.remove(&key);
    } else {
        p.insert(key, c);
    }
}
fn formula(f: &F, p: &serde_json::Value, i: usize, memo: &mut BTreeMap<usize, Form>) -> Form {
    if let Some(v) = memo.get(&i) {
        return v.clone();
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
            assert_eq!(i, 0);
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
            let x = formula(f, p, a[0].as_u64().unwrap() as usize, memo);
            let y = formula(f, p, a[1].as_u64().unwrap() as usize, memo);
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
                    .map(|(h, _)| *h)
                    .min()
                    .unwrap_or(z.d - 1)
                    .min(z.d - 1);
            if r > 0 {
                z.d -= r;
                z.k += r;
                z.p = z.p.into_iter().map(|((h, x), c)| ((h - r, x), c)).collect();
            }
        }
        op => panic!("unsupported dependency{op}"),
    };
    assert!(z.d <= 32 && z.p.len() <= 4096);
    memo.insert(i, z.clone());
    z
}
pub fn run(dir: &Path) {
    let f = F::new(360, 45);
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound16\nimport KoblitzRound14Boundary\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound16Separators\n");
    for g in 0..4 {
        let raw = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &raw["program"];
        let mut memo = BTreeMap::new();
        let cube = formula(&f, p, 19, &mut memo);
        let quartic = formula(&f, p, 80, &mut memo);
        assert_eq!((cube.d, cube.k), (3, 2));
        assert_eq!(cube.p.len(), 1);
        assert_eq!(cube.p[&(0, 3)], f.o());
        assert_eq!((quartic.d, quartic.k), (4, 2));
        let c = (0..4)
            .map(|k| quartic.p.get(&(3 - k, k + 1)).unwrap().clone())
            .collect::<Vec<_>>();
        assert!(c.iter().all(|c| *c != f.z()));
        let mul = |a: &E, b: &E| f.mul(a, b);
        let cube3 = |a: &E| mul(&f.sq(a), a);
        let coeffs = vec![
            ([0, 4, 0], cube3(&c[3])),
            (
                [1, 3, 0],
                mul(&f.sq(&c[3]), &c[0])
                    .add(&cube3(&c[2]))
                    .add(&mul(&mul(&c[1], &c[3]), &c[2])),
            ),
            (
                [2, 2, 0],
                mul(&c[3], &f.sq(&c[0]))
                    .add(&cube3(&c[1]))
                    .add(&mul(&mul(&c[1], &c[0]), &c[2])),
            ),
            ([3, 1, 0], cube3(&c[0])),
            ([1, 2, 1], f.sq(&c[2]).add(&mul(&c[1], &c[3]))),
            ([1, 1, 2], c[2].clone()),
            ([1, 0, 3], f.o()),
            ([2, 1, 1], mul(&c[1], &c[0])),
        ];
        let boundary = coeffs[0].1.to_biguint();
        let bit = (0..360).find(|i| boundary.bit(*i)).unwrap();
        let selected = coeffs
            .iter()
            .filter(|(_, c)| c.to_biguint().bit(bit))
            .map(|(m, _)| *m)
            .collect::<Vec<_>>();
        let expression = selected
            .iter()
            .map(|m| format!("h^{}*u^{}*v^{}", m[0], m[1], m[2]))
            .collect::<Vec<_>>()
            .join(" + ");
        let actual = (68..=80)
            .map(|i| json!({"node":i,"gate":p["nodes"][i]}))
            .collect::<Vec<_>>();
        records.push(json!({"graph":g,"cube_node":19,"quartic_node":80,"cube_membership_h_depth":cube.k,"quartic_membership_h_depth":quartic.k,"separator_membership_h_depth":6,"normalized_source_certificate":memo.iter().map(|(i,v)|json!({"node":i,"gate":p["nodes"][*i],"degree":v.d,"h_membership_depth":v.k,"homogeneous_terms":v.p.iter().map(|(m,c)|json!({"h_x_exponents":[m.0,m.1],"coefficient":c.to_biguint().to_str_radix(16)})).collect::<Vec<_>>()})).collect::<Vec<_>>(),"Horner_coefficients_c0_c1_c2_c3":c.iter().map(|c|c.to_biguint().to_str_radix(16)).collect::<Vec<_>>(),"original_Horner_gates":actual,"separator_terms":coeffs.iter().map(|(m,c)|json!({"h_U_V_exponents":m,"coefficient":c.to_biguint().to_str_radix(16)})).collect::<Vec<_>>(),"selected_F2_coefficient_bit":bit,"selected_F2_quartic":selected,"at_actual_Q17_point":1,"actual_Q17_model_excluded_under_source_semantics":true,"full_h_saturation":false}));
        lean.push_str(&format!("def scalar{g} (h u v:ZMod 2) : ZMod 2 := {expression}\ntheorem actual_boundary_separation{g} (v:ZMod 2) : scalar{g} 0 1 v=1 := by simp [scalar{g}]\ntheorem old_unit_cube{g} : (KoblitzRound14Boundary.point{g}.contains 19)=true := by decide +kernel\ntheorem actual_Horner_structure{g} : KoblitzRound12Boundary.program{g}[71]?=some (.mul 0 70) ∧ KoblitzRound12Boundary.program{g}[74]?=some (.mul 0 73) ∧ KoblitzRound12Boundary.program{g}[77]?=some (.mul 0 76) ∧ KoblitzRound12Boundary.program{g}[80]?=some (.mul 0 79) := by decide +kernel\n"));
        println!("Q21:graph{g},actual cube/quartic h-depth2,separator h-depth6,selected bit{bit} rejects actual Q17 model");
    }
    lean.push_str("end KoblitzRound16Separators\n");
    for g in 0..4 {
        for n in [
            "actual_boundary_separation",
            "old_unit_cube",
            "actual_Horner_structure",
        ] {
            lean.push_str(&format!("#print axioms KoblitzRound16Separators.{n}{g}\n"));
        }
    }
    assert!(!dir.join("KoblitzRound16Separators.lean").exists());
    std::fs::write(dir.join("KoblitzRound16Separators.lean"), lean).unwrap();
    crate::round02::save(
        dir,
        "q21-source-bound-quartic-separators.json",
        &json!({"schema":"koblitz.round16/q21-v1","records":records,"scope":"actual h6 quartic membership and scalar boundary separation,not whole-ideal saturation"}),
    );
}
