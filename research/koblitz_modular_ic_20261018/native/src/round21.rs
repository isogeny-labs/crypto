//! Round21: characteristic-two tangent constraints and equivariant recovery.
use crate::{large_field::F, round02, round04::read};
use serde_json::{json, Value};
use std::{collections::BTreeSet, path::Path};

fn save(dir: &Path, name: &str, value: &Value) {
    assert!(!dir.join(name).exists());
    round02::save(dir, name, value);
}
fn lean(dir: &Path, name: &str, value: &str) {
    assert!(!dir.join(name).exists());
    std::fs::write(dir.join(name), value).unwrap();
}
fn binary_rank(mut rows: Vec<u32>) -> usize {
    let mut rank = 0;
    for bit in (0..18).rev() {
        if let Some(i) = (rank..rows.len()).find(|i| rows[*i] >> bit & 1 != 0) {
            rows.swap(rank, i);
            for j in 0..rows.len() {
                if j != rank && rows[j] >> bit & 1 != 0 {
                    rows[j] ^= rows[rank];
                }
            }
            rank += 1;
        }
    }
    rank
}

fn univariate(
    f: &F,
    nodes: &Value,
    node: usize,
    input: usize,
) -> Option<Vec<crate::large_field::E>> {
    let n = &nodes[node];
    let args = &n["args"];
    let mut r = match n["op"].as_str().unwrap() {
        "Input" => {
            if args.as_u64().unwrap() as usize == input {
                vec![f.z(), f.o()]
            } else {
                return None;
            }
        }
        "Constant" => vec![crate::large_field::E::from_biguint(
            &num_bigint::BigUint::parse_bytes(args.as_str().unwrap().as_bytes(), 16).unwrap(),
            f.m(),
        )],
        "Add" | "Mul" => {
            let a = univariate(f, nodes, args[0].as_u64().unwrap() as usize, input)?;
            let b = univariate(f, nodes, args[1].as_u64().unwrap() as usize, input)?;
            if n["op"] == "Add" {
                (0..a.len().max(b.len()))
                    .map(|i| {
                        a.get(i)
                            .cloned()
                            .unwrap_or_else(|| f.z())
                            .add(&b.get(i).cloned().unwrap_or_else(|| f.z()))
                    })
                    .collect()
            } else {
                let mut z = vec![f.z(); a.len() + b.len()];
                for (i, c) in a.iter().enumerate() {
                    for (j, d) in b.iter().enumerate() {
                        z[i + j] = z[i + j].add(&f.mul(c, d));
                    }
                }
                z
            }
        }
        _ => return None,
    };
    while r.last() == Some(&f.z()) {
        r.pop();
    }
    Some(r)
}

pub fn ordinary_profiles(dir: &Path, graph: usize) -> Vec<Value> {
    let f = F::new(360, 45);
    let doc = read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
    let nodes = &doc["program"]["nodes"];
    let mut result = vec![];
    for (input, node) in [
        (1, 123),
        (3, 183),
        (5, 243),
        (7, 303),
        (1, 429),
        (3, 485),
        (5, 541),
        (7, 597),
    ] {
        assert_eq!(nodes[node]["op"], "Mul");
        assert_eq!(nodes[node]["args"][0], input);
        let s_node = nodes[node]["args"][1].as_u64().unwrap() as usize;
        let s = univariate(&f, nodes, s_node, input - 1).unwrap();
        assert_eq!(s.len(), 9);
        assert_eq!(s[8], f.o());
        result.push(json!({"input_y":input,"node":node,"S_node":s_node,"S_coefficients":s.iter().map(|c|c.to_biguint().to_str_radix(16)).collect::<Vec<_>>(),"degree":8,"leading":"1","pole_order":19}));
    }
    result
}
pub fn tangent(dir: &Path) {
    let f = F::find(18);
    let z = f.z();
    let one = f.o();
    let exponent = ((1u64 << 18) - 1) / 19;
    let mut root = None;
    for i in 2..1024 {
        let r = f.powu(&f.e(i), exponent);
        if r != one {
            assert_eq!(f.powu(&r, 19), one);
            root = Some(r);
            break;
        }
    }
    let root = root.expect("bounded primitive-root search");
    let roots = (0..19).map(|i| f.powu(&root, i)).collect::<Vec<_>>();
    let masks = roots
        .iter()
        .map(|r| r.to_biguint().to_u32_digits()[0])
        .collect::<Vec<_>>();
    let distinct = masks.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(distinct.len(), 19);
    let mut trace = vec![];
    for r in &roots {
        let mut t = z.clone();
        let mut x = r.clone();
        for _ in 0..18 {
            t = t.add(&x);
            x = f.sq(&x);
        }
        trace.push(t.to_biguint().to_str_radix(16));
    }
    assert_eq!(trace[0], "0");
    assert!(trace[1..].iter().all(|t| t == "1"));
    let all_sum = masks.iter().fold(0, |a, b| a ^ b);
    assert_eq!(all_sum, 0);
    let rank = binary_rank(masks.clone());
    assert_eq!(rank, 18);
    let mut prefixes = 0usize;
    let mut matches = vec![];
    for a in 0..19 {
        for b in 0..19 {
            for c in 0..19 {
                for d in 0..19 {
                    prefixes += 1;
                    let e = masks[a] ^ masks[b] ^ masks[c] ^ masks[d];
                    if distinct.contains(&e) {
                        matches.push(json!([
                            a,
                            b,
                            c,
                            d,
                            masks.iter().position(|r| *r == e).unwrap()
                        ]));
                    }
                }
            }
        }
    }
    assert_eq!(prefixes, 130321);
    assert!(matches.is_empty());
    let cycle = (0..18).map(|i| (1u64 << i) % 19).collect::<Vec<_>>();
    assert_eq!(
        cycle.iter().copied().collect::<BTreeSet<_>>(),
        (1..19).collect()
    );
    let q33 = read(&dir.join("q33-coupled-quotient-resultant.json"));
    let q34 = read(&dir.join("q34-after-coupled-quotient-repair.json"));
    let mut sources = vec![];
    let mut literal =
        String::from("import KoblitzRound20Boundary\nnamespace KoblitzRound21TangentData\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let nodes = &doc["program"]["nodes"];
        assert_eq!(nodes[123], json!({"op":"Mul","args":[1,122]}));
        assert_eq!(q33["records"][g]["S"].as_array().unwrap().len(), 9);
        assert_eq!(q33["records"][g]["S"][8], "1");
        let support = q34["records"][g]["model"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| (b == 1).then_some(i))
            .collect::<Vec<_>>();
        assert_eq!(support, [123, 124, 126, 127]);
        let sigma = nodes
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                (n["op"] == "CoefficientFrobenius").then_some(json!([i, n["args"]]))
            })
            .collect::<Vec<_>>();
        assert_eq!(sigma.len(), 12);
        sources.push(json!({"graph":g,"nodes":nodes.as_array().unwrap().len(),"coefficient_Frobenius_gates":sigma,"S_degree":8,"S_leading":"1","ordinary_local_poles":{"x":2,"y":3,"yS":19},"survivor_support":support,"complete_arc_constructed":false,"geometric_arc_exclusion":"requires written DVR, split-coefficient and formal-group bridge"}));
        literal.push_str(&format!("theorem actual_support{g} : KoblitzRound20Boundary.point{g}=[123,124,126,127] := by decide +kernel\ntheorem actual_U_gate{g} : KoblitzRound12Boundary.program{g}[123]?=some (.mul 1 122) := by decide +kernel\n"));
    }
    literal.push_str("end KoblitzRound21TangentData\n");
    for g in 0..4 {
        for n in ["actual_support", "actual_U_gate"] {
            literal.push_str(&format!("#print axioms KoblitzRound21TangentData.{n}{g}\n"));
        }
    }
    lean(dir, "KoblitzRound21TangentData.lean", &literal);
    save(
        dir,
        "q35-trace-zero-tangent-obstruction.json",
        &json!({"schema":"koblitz.round21/q35-v1","field_degree":18,"modulus_low":f.irr.low_terms,"primitive_root":root.to_biguint().to_str_radix(16),"root_masks":masks,"trace18_values":trace,"binary_rank":rank,"binary_kernel_dimension":1,"binary_kernel_generator":"all19 coefficients one","doubling_exponents_mod19":cycle,"four_prefixes_exhausted":prefixes,"compatible_five_tuples":matches,"controls":{"equal_pair_sum_zero":masks[0]^masks[0]==0,"all19_sum_zero":all_sum==0},"sources":sources,"full_saturation_computed":false,"polynomial_separator_constructed":false}),
    );
    println!("Q35:130321 ordered four-root prefixes;0 compatible fifth roots;binary rank18;four actual source profiles;complete geometric bridge separate");
}

type C = (i128, i128);
type M = Vec<Vec<C>>;
fn cmul(a: C, b: C) -> C {
    (a.0 * b.0 - 2 * a.1 * b.1, a.0 * b.1 + a.1 * b.0 - a.1 * b.1)
}
fn product(a: &M, b: &M) -> M {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| {
                    (0..b.len()).fold((0, 0), |(x, y), k| {
                        let z = cmul(a[i][k], b[k][j]);
                        (x + z.0, y + z.1)
                    })
                })
                .collect()
        })
        .collect()
}
fn eye(n: usize, scale: i128) -> M {
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| (if i == j { scale } else { 0 }, 0))
                .collect()
        })
        .collect()
}
fn add(a: &mut M, b: &M) {
    for (r, s) in a.iter_mut().zip(b) {
        for (x, y) in r.iter_mut().zip(s) {
            *x = (x.0 + y.0, x.1 + y.1);
        }
    }
}
fn lit(a: &M) -> String {
    format!(
        "![{}]",
        a.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(|(a, b)| format!("({a},{b})"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn value(a: &M) -> Value {
    json!(a
        .iter()
        .map(|r| r
            .iter()
            .map(|(a, b)| [a.to_string(), b.to_string()])
            .collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
pub fn descent(dir: &Path) {
    let d = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let rows = (0..20)
        .map(|i| {
            (0..4)
                .map(|j| {
                    (
                        d["orbits"][i / 5][i % 5][j][0].as_i64().unwrap() as i128,
                        d["orbits"][i / 5][i % 5][j][1].as_i64().unwrap() as i128,
                    )
                })
                .collect()
        })
        .collect::<M>();
    let d = read(&dir.join("g28-integral-orbit-retraction.json"));
    let l = d["CM_recovery_coefficients"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|c| {
                    (
                        c[0].as_str().unwrap().parse::<i128>().unwrap(),
                        c[1].as_str().unwrap().parse::<i128>().unwrap(),
                    )
                })
                .collect()
        })
        .collect::<M>();
    let r = vec![
        vec![(-1, 0); 4],
        vec![(1, 0), (0, 0), (0, 0), (0, 0)],
        vec![(0, 0), (1, 0), (0, 0), (0, 0)],
        vec![(0, 0), (0, 0), (1, 0), (0, 0)],
    ];
    let p = (0..20)
        .map(|i| {
            (0..20)
                .map(|j| (i128::from(j == 5 * (i / 5) + (i + 1) % 5), 0))
                .collect()
        })
        .collect::<M>();
    assert_eq!(product(&p, &rows), product(&rows, &r));
    assert_eq!(product(&l, &rows), eye(4, 1));
    let mut rp = eye(4, 1);
    let mut pp = eye(20, 1);
    let mut rinv = eye(4, 1);
    for _ in 0..4 {
        rinv = product(&rinv, &r);
    }
    let mut avg = vec![vec![(0, 0); 20]; 4];
    let mut norm = eye(4, 0);
    for _ in 0..5 {
        add(&mut avg, &product(&product(&rp, &l), &pp));
        rp = product(&rinv, &rp);
        pp = product(&pp, &p);
    }
    let mut power = eye(4, 1);
    for _ in 0..5 {
        add(&mut norm, &power);
        power = product(&power, &r);
    }
    assert_eq!(power, eye(4, 1));
    assert_eq!(norm, eye(4, 0));
    assert_eq!(product(&avg, &rows), eye(4, 5));
    assert_eq!(product(&avg, &p), product(&r, &avg));
    let indivisible = avg
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter().enumerate().filter_map(move |(j, c)| {
                (c.0 % 5 != 0 || c.1 % 5 != 0).then_some(json!([
                    i,
                    j,
                    c.0.to_string(),
                    c.1.to_string()
                ]))
            })
        })
        .collect::<Vec<_>>();
    assert!(!indivisible.is_empty());
    let mut out=format!("import KoblitzRound20Retraction\nnamespace KoblitzRound21DescentData\nopen scoped BigOperators\nopen KoblitzRound19OrbitCover\ndef rotation : Fin 4→Fin 4→CM := {}\ndef averaged : Fin 4→Fin 20→CM := {}\ndef shift (i:Fin 20) : Fin 20 := ⟨5*(i.val/5)+(i.val+1)%5,by omega⟩\ntheorem actual_averaged_recovery : ∀i j:Fin 4,(∑k:Fin 20,cmul (averaged i k) (KoblitzRound20Retraction.row k j))=(if i=j then(5,0)else(0,0)) := by decide +kernel\ntheorem actual_equivariance : ∀i:Fin 4,∀j:Fin 20,averaged i (⟨5*(j.val/5)+(j.val+4)%5,by omega⟩)=(∑k:Fin 4,cmul (rotation i k) (averaged k j)) := by decide +kernel\ntheorem actual_rows_equivariance : ∀i:Fin 20,∀j:Fin 4,KoblitzRound20Retraction.row (shift i) j=(∑k:Fin 4,cmul (KoblitzRound20Retraction.row i k) (rotation k j)) := by decide +kernel\n",lit(&r),lit(&avg));
    out.push_str("end KoblitzRound21DescentData\n#print axioms KoblitzRound21DescentData.actual_averaged_recovery\n#print axioms KoblitzRound21DescentData.actual_equivariance\n#print axioms KoblitzRound21DescentData.actual_rows_equivariance\n");
    lean(dir, "KoblitzRound21DescentData.lean", &out);
    save(
        dir,
        "g31-equivariant-recovery-denominator.json",
        &json!({"schema":"koblitz.round21/g31-v1","rotation":value(&r),"target_shift":(0..20).map(|i|5*(i/5)+(i+1)%5).collect::<Vec<_>>(),"averaged_CM_recovery":value(&avg),"averaged_recovery_scalar":5,"exact_equivariance":true,"norm_rotation_zero":true,"nondivisible_by5_entries":indivisible,"minimum_equivariant_retraction_denominator":"5 under written norm/fixed-vector obstruction","source_closed_embedding_unchanged":true,"final_105_19_quotient_constructed":false}),
    );
    println!("G31:actual averaged recovery A*Phi=5I;A*Pi=R*A;denominator5 not entrywise removable;integral obstruction requires structural norm proof");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rank_controls() {
        assert_eq!(binary_rank(vec![1, 2, 4, 8, 16]), 5);
        assert_eq!(binary_rank(vec![1, 2, 3]), 2);
        assert_eq!(binary_rank(vec![0, 0]), 0);
    }
    #[test]
    fn cm_relation_control() {
        assert_eq!(cmul((0, 1), (0, 1)), (-2, -1));
    }
}
