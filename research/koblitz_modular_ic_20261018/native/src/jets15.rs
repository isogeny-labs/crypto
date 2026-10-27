//! First-order obstruction on actual boundary models. This is not saturation.
use crate::round04::read;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::path::Path;
#[derive(Clone)]
struct Row {
    a: BigUint,
    b: BigUint,
    proof: BigUint,
}
fn support(a: &BigUint, n: usize) -> Vec<usize> {
    (0..n).filter(|i| a.bit(*i as u64)).collect()
}
pub fn run(dir: &Path) {
    let all = read(&dir.join("q17-expanded-product-quadrics.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound15\nimport KoblitzRound14Boundary\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound15Jets\n");
    let mut complete = true;
    for graph in 0..4 {
        let input = read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p = &input["program"];
        let nodes = p["nodes"].as_array().unwrap();
        let n = nodes.len();
        let model = all["records"][graph]["after_all_Q17"]["model"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u8)
            .collect::<Vec<_>>();
        assert_eq!(model.len(), n);
        assert!(model.iter().all(|v| *v <= 1));
        assert!(model.contains(&1));
        let mut rows = vec![];
        let mut origins = vec![];
        let mut omitted = vec![];
        let mut push = |s: Vec<usize>, b: BigUint, origin: Value| {
            let mut a = BigUint::zero();
            for i in s {
                a ^= BigUint::one() << i;
            }
            let index = rows.len();
            rows.push(Row {
                a,
                b,
                proof: BigUint::one() << index,
            });
            origins.push(origin);
        };
        for (i, node) in nodes.iter().enumerate() {
            let args = &node["args"];
            match node["op"].as_str().unwrap() {
                "Input" => {}
                "Constant" => {
                    assert_eq!(model[i], 0);
                    push(
                        vec![i],
                        BigUint::parse_bytes(args.as_str().unwrap().as_bytes(), 16).unwrap(),
                        json!({"node":i,"kind":"constant"}),
                    );
                }
                "Add" => {
                    let a = args[0].as_u64().unwrap() as usize;
                    let b = args[1].as_u64().unwrap() as usize;
                    assert_eq!(model[i], model[a] ^ model[b]);
                    push(
                        vec![i, a, b],
                        BigUint::zero(),
                        json!({"node":i,"kind":"add"}),
                    );
                }
                "Mul" => {
                    let a = args[0].as_u64().unwrap() as usize;
                    let b = args[1].as_u64().unwrap() as usize;
                    assert_eq!(model[a] * model[b], 0);
                    let mut s = vec![];
                    if model[a] != 0 {
                        s.push(b);
                    }
                    if model[b] != 0 {
                        s.push(a);
                    }
                    push(
                        s,
                        BigUint::from(model[i]),
                        json!({"node":i,"kind":"multiply","inputs":[a,b],"base_values":[model[a],model[b],model[i]]}),
                    );
                }
                "Inverse" => {
                    let a = args.as_u64().unwrap() as usize;
                    assert_eq!(model[a] * model[i], 0);
                    let mut s = vec![];
                    if model[a] != 0 {
                        s.push(i);
                    }
                    if model[i] != 0 {
                        s.push(a);
                    }
                    push(s, BigUint::zero(), json!({"node":i,"kind":"inverse"}));
                }
                "CoefficientFrobenius" => {
                    let a = args.as_u64().unwrap() as usize;
                    assert_eq!(model[i], model[a]);
                    omitted.push(i);
                }
                _ => panic!("unsupported operation"),
            }
        }
        for value in p["residuals"].as_array().unwrap() {
            let i = value.as_u64().unwrap() as usize;
            assert_eq!(model[i], 0);
            push(
                vec![i],
                BigUint::zero(),
                json!({"node":i,"kind":"residual"}),
            );
        }
        assert!(n * 360 <= 500000 && rows.len() * 360 <= 500000);
        let mut basis: Vec<Option<Row>> = vec![None; n];
        let mut contradiction = None;
        let mut reductions = 0;
        for original in &rows {
            let mut r = original.clone();
            while !r.a.is_zero() {
                let pivot = r.a.trailing_zeros().unwrap() as usize;
                if let Some(b) = &basis[pivot] {
                    r.a ^= &b.a;
                    r.b ^= &b.b;
                    r.proof ^= &b.proof;
                    reductions += 1;
                    assert!(reductions <= 2_000_000);
                } else {
                    basis[pivot] = Some(r.clone());
                    break;
                }
            }
            if r.a.is_zero() && !r.b.is_zero() {
                contradiction = Some(r);
                break;
            }
        }
        let mut rec = json!({"graph":graph,"original_K_nodes":n,"full_F2_unknowns":n*360,"relaxed_K_rows":rows.len(),"omitted_coefficient_Frobenius_nodes":omitted,"row_reductions":reductions,"original_boundary_model_checked":true,"full_h_saturation":false,"boundary_point_excluded":false,"h_equals_t2_or_higher_arcs_tested":false});
        if let Some(w) = contradiction {
            let ids = support(&w.proof, rows.len());
            let mut a = BigUint::zero();
            let mut b = BigUint::zero();
            for i in &ids {
                a ^= &rows[*i].a;
                b ^= &rows[*i].b;
            }
            assert!(a.is_zero());
            assert_eq!(b, w.b);
            let bit = b.trailing_zeros().unwrap();
            let cert=ids.iter().map(|i|json!({"row":i,"origin":origins[*i],"support":support(&rows[*i].a,n),"rhs_bit":rows[*i].b.bit(bit)})).collect::<Vec<_>>();
            let mut parity = false;
            for i in &ids {
                parity ^= rows[*i].b.bit(bit);
            }
            assert!(parity);
            rec["first_jet_h_equals_t"]=json!("obstructed by exact original-row combination;valid also with all Frobenius rows imposed");
            rec["witness_coefficient_bit"] = json!(bit);
            rec["witness_rows"] = json!(cert);
            let expressions = ids
                .iter()
                .map(|i| {
                    let mut terms = support(&rows[*i].a, n)
                        .into_iter()
                        .map(|j| format!("x {j}"))
                        .collect::<Vec<_>>();
                    if rows[*i].b.bit(bit) {
                        terms.push("1".into());
                    }
                    if terms.is_empty() {
                        "0".into()
                    } else {
                        format!("({})", terms.join(" + "))
                    }
                })
                .collect::<Vec<_>>();
            lean.push_str(&format!("def residuals{graph} (x : Nat → ZMod 2) : List (ZMod 2) := [{}]\ntheorem actual_first_jet_obstruction{graph} (x : Nat → ZMod 2) : (residuals{graph} x).sum=1 := by\n simp only [residuals{graph},List.sum_cons,List.sum_nil]\n ring\ntheorem actual_first_jet_nonzero{graph} (x : Nat → ZMod 2) : (residuals{graph} x).sum ≠ 0 := by\n rw [actual_first_jet_obstruction{graph}]\n decide\n",expressions.join(",")));
            assert_eq!(ids.len(), 1);
            assert_eq!(origins[ids[0]]["node"], json!(12));
            assert_eq!(origins[ids[0]]["inputs"], json!([1, 1]));
            assert_eq!(model[1], 0);
            assert_eq!(model[12], 1);
            lean.push_str(&format!("theorem actual_gate{graph} : KoblitzRound12Boundary.program{graph}[12]? = some (.mul 1 1) := by decide +kernel\ntheorem actual_base{graph} : (KoblitzRound14Boundary.point{graph}.contains 1)=false ∧ (KoblitzRound14Boundary.point{graph}.contains 12)=true := by decide +kernel\ntheorem actual_gate_jet{graph} {{R : Type*}} [CommRing R] [Nontrivial R] (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := KoblitzRound15.square_unramified_obstruction x1\n"));
            println!("Q18:graph{graph},original boundary model verified,no h=t first jet;{} original rows,coefficient bit{bit}",ids.len());
        } else {
            complete = false;
            rec["first_jet_h_equals_t"] =
                json!("K-linear relaxation consistent;full semilinear expansion still required");
            println!("Q18:graph{graph},relaxation consistent;full first-jet status open");
        }
        records.push(rec);
    }
    lean.push_str("end KoblitzRound15Jets\n");
    for graph in 0..4 {
        for name in [
            "actual_first_jet_obstruction",
            "actual_first_jet_nonzero",
            "actual_gate",
            "actual_base",
            "actual_gate_jet",
        ] {
            lean.push_str(&format!("#print axioms KoblitzRound15Jets.{name}{graph}\n"));
        }
    }
    if complete {
        assert!(!dir.join("KoblitzRound15Jets.lean").exists());
        std::fs::write(dir.join("KoblitzRound15Jets.lean"), lean).unwrap();
    }
    crate::round02::save(
        dir,
        "q18-original-boundary-first-jets.json",
        &json!({"schema":"koblitz.round15/q18-v1","records":records,"all_four_unramified_first_jets_obstructed":complete,"scope":"linearization of original full coefficient program;Frobenius rows omitted only in a sound obstruction relaxation","ramified_control":"h=x^2 has no h=t first jet at origin but admits h=t^2,x=t;verified by parent Lean identities","full_saturation":false}),
    );
}
