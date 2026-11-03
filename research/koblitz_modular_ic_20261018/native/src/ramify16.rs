//! Original-gate ramification and an exact base-elliptic formal arc; not a full quotient arc.
use crate::round04::read;
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
const N: usize = 96;
fn mul(a: u128, b: u128) -> u128 {
    let mut c = 0;
    for i in 0..N {
        if (b >> i) & 1 == 1 {
            c ^= a << i;
        }
    }
    c & ((1u128 << N) - 1)
}
fn inv(a: u128) -> u128 {
    assert!(a & 1 == 1);
    let mut b = 1u128;
    for k in 1..N {
        let mut v = 0;
        for i in 1..=k {
            v ^= ((a >> i) & 1) & ((b >> (k - i)) & 1);
        }
        b ^= v << k;
    }
    assert_eq!(mul(a, b), 1);
    b
}
fn exponents(a: u128) -> Vec<usize> {
    (0..N).filter(|i| (a >> i) & 1 == 1).collect()
}
pub fn run(dir: &Path) {
    let all = read(&dir.join("q17-expanded-product-quadrics.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound15Jets\nnamespace KoblitzRound16Ramification\ntheorem ramification_six (e x y:ℤ) (hy:2*y=e) (hx:3*x=2*e) : ∃ k:ℤ,e=6*k ∧ x=4*k ∧ y=3*k := by\n refine ⟨e/6,?_,?_,?_⟩ <;> omega\ntheorem no_low_ramification (e x y:ℤ) (he:0<e) (he6:e<6) (hy:2*y=e) (hx:3*x=2*e) : False := by omega\n");
    for g in 0..4 {
        let raw = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &raw["program"];
        let nodes = p["nodes"].as_array().unwrap();
        let m = all["records"][g]["after_all_Q17"]["model"]
            .as_array()
            .unwrap();
        for (i, args) in [
            (12, json!([1, 1])),
            (16, json!([15, 15])),
            (17, json!([0, 16])),
            (18, json!([17, 17])),
            (19, json!([0, 18])),
        ] {
            assert_eq!(nodes[i]["op"], "Mul");
            assert_eq!(nodes[i]["args"], args);
        }
        assert_eq!(nodes[15]["op"], "Constant");
        assert_eq!(nodes[15]["args"], "1");
        assert_eq!(m[12], 1);
        assert_eq!(m[19], 1);
        let first = p["residuals"][0].as_u64().unwrap() as usize;
        let a = usize::from(g >= 2);
        let mut u = 1u128;
        for _ in 0..8 {
            u = 1 ^ 2 ^ ((a as u128) << 2) ^ ((inv(mul(u, u)) << 6) & ((1u128 << N) - 1));
        }
        assert_eq!(mul(mul(u, u), u ^ 1 ^ 2 ^ ((a as u128) << 2)), 1 << 6);
        // Evaluate the actual affine DAG as sparse Laurent polynomials, then homogenize by t^6.
        let mut vals: Vec<BTreeSet<i32>> = vec![];
        let mut reductions = 0;
        for (i, node) in nodes.iter().enumerate().take(first + 1) {
            let args = &node["args"];
            let mut z = BTreeSet::new();
            match node["op"].as_str().unwrap() {
                "Input" => {
                    if i < 2 {
                        for k in exponents(u) {
                            z.insert(k as i32 - if i == 0 { 2 } else { 3 });
                        }
                    }
                }
                "Constant" => {
                    if args.as_str().unwrap() != "0" {
                        assert_eq!(args, "1");
                        z.insert(0);
                    }
                }
                "Add" => {
                    let x = args[0].as_u64().unwrap() as usize;
                    let y = args[1].as_u64().unwrap() as usize;
                    z = vals[x].symmetric_difference(&vals[y]).copied().collect();
                }
                "Mul" => {
                    let x = args[0].as_u64().unwrap() as usize;
                    let y = args[1].as_u64().unwrap() as usize;
                    for v in &vals[x] {
                        for w in &vals[y] {
                            reductions += 1;
                            assert!(reductions <= 1000000);
                            let k = v + w;
                            if k < 102 {
                                assert!(k >= -32);
                                if !z.insert(k) {
                                    z.remove(&k);
                                }
                            }
                        }
                    }
                }
                op => panic!("unexpected base operation{op}"),
            };
            vals.push(z);
        }
        let hom = vals
            .iter()
            .map(|v| {
                v.iter()
                    .filter_map(|k| {
                        let j = k + 6;
                        assert!(j >= 0);
                        if j < 96 {
                            Some(j as usize)
                        } else {
                            None
                        }
                    })
                    .fold(0u128, |x, k| x ^ (1 << k))
            })
            .collect::<Vec<_>>();
        for (i, node) in nodes.iter().enumerate().take(first + 1) {
            let args = &node["args"];
            let r = match node["op"].as_str().unwrap() {
                "Input" => 0,
                "Constant" => hom[i] ^ if args == "0" { 0 } else { 1 << 6 },
                "Add" => {
                    hom[i]
                        ^ hom[args[0].as_u64().unwrap() as usize]
                        ^ hom[args[1].as_u64().unwrap() as usize]
                }
                "Mul" => {
                    mul(1 << 6, hom[i])
                        ^ mul(
                            hom[args[0].as_u64().unwrap() as usize],
                            hom[args[1].as_u64().unwrap() as usize],
                        )
                }
                _ => unreachable!(),
            };
            assert_eq!(r, 0, "graph{g},gate{i}");
            assert_eq!(hom[i] & 1, m[i].as_u64().unwrap() as u128);
        }
        assert_eq!(hom[first], 0);
        records.push(json!({"graph":g,"actual_y_square_gate":12,"actual_unit_cube_gate":19,"constant_one_gate":15,"necessary_valuation_equalities":["2*vy=e","3*vx=2*e"],"necessary_ramification_multiple":6,"base_arc":{"h_exponent":6,"x_exponent":4,"y_exponent":3,"unit_u_exponents":exponents(u),"precision":N,"original_nodes_checked":first+1,"original_curve_residual":first,"exact_mod_t96_checks":true,"matches_actual_Q17_base_values":true,"native_reductions":reductions},"full_quotient_arc":false,"full_saturation":false}));
        lean.push_str(&format!("theorem actual_gates{g} : KoblitzRound12Boundary.program{g}[12]?=some (.mul 1 1) ∧ KoblitzRound12Boundary.program{g}[16]?=some (.mul 15 15) ∧ KoblitzRound12Boundary.program{g}[17]?=some (.mul 0 16) ∧ KoblitzRound12Boundary.program{g}[18]?=some (.mul 17 17) ∧ KoblitzRound12Boundary.program{g}[19]?=some (.mul 0 18) := by decide +kernel\ntheorem actual_units{g} : (KoblitzRound14Boundary.point{g}.contains 12)=true ∧ (KoblitzRound14Boundary.point{g}.contains 19)=true := by decide +kernel\n"));
        println!("Q19:graph{g},necessary6|e;h=t6 base arc passes{} original nodes modt96;full quotient arc open",first+1);
    }
    lean.push_str("end KoblitzRound16Ramification\n#print axioms KoblitzRound16Ramification.ramification_six\n#print axioms KoblitzRound16Ramification.no_low_ramification\n");
    for g in 0..4 {
        for n in ["actual_gates", "actual_units"] {
            lean.push_str(&format!(
                "#print axioms KoblitzRound16Ramification.{n}{g}\n"
            ));
        }
    }
    assert!(!dir.join("KoblitzRound16Ramification.lean").exists());
    std::fs::write(dir.join("KoblitzRound16Ramification.lean"), lean).unwrap();
    crate::round02::save(
        dir,
        "q19-actual-ramification-six.json",
        &json!({"schema":"koblitz.round16/q19-v1","records":records,"scope":"necessary DVR ramification and actual base elliptic subcircuit arc only;no full quotient arc or saturation conclusion"}),
    );
}
