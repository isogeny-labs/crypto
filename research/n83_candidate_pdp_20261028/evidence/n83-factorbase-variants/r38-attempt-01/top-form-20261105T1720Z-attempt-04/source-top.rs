//! Replayed graded quotient certificates for exact paired baseline/spike40 inputs.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
fn index(s: &str, n: usize) -> usize {
    let i: usize = s
        .strip_prefix("v(")
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .parse()
        .unwrap();
    assert!((1..=n).contains(&i));
    i - 1
}
fn parse_binary(text: &str, n: usize) -> Vec<(usize, Vec<Vec<usize>>)> {
    let mut polys = vec![];
    let mut fields = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for l in text.lines().filter(|l| l.starts_with("I[")) {
        let (a, b) = l.split_once('=').unwrap();
        let id = a[2..a.len() - 1].parse().unwrap();
        assert!(ids.insert(id), "duplicate equation id");
        let b = b.strip_suffix(';').unwrap();
        if let Some((x, y)) = b.split_once("^2-") {
            assert_eq!(x, y);
            assert!(fields.insert(index(x, n)), "duplicate field equation");
            polys.push((id, vec![vec![index(x, n), index(x, n)]]));
            continue;
        }
        let mut terms = BTreeSet::new();
        for t in b.split('+').filter(|t| *t != "0") {
            let mut mon = if t == "1" {
                vec![]
            } else {
                t.split('*').map(|v| index(v, n)).collect::<Vec<_>>()
            };
            mon.sort_unstable();
            assert!(mon.windows(2).all(|w| w[0] != w[1]));
            if !terms.remove(&mon) {
                terms.insert(mon);
            }
        }
        let degree = terms.iter().map(Vec::len).max();
        polys.push((
            id,
            terms
                .into_iter()
                .filter(|m| Some(m.len()) == degree)
                .collect(),
        ));
    }
    assert_eq!(fields, (0..n).collect());
    polys
}
fn projection(polys: &[(usize, Vec<Vec<usize>>)], keep: &BTreeSet<usize>) -> Value {
    let mut equations = vec![];
    let mut ok = true;
    for (id, terms) in polys {
        let surviving: Vec<_> = terms
            .iter()
            .filter(|m| m.iter().all(|i| keep.contains(i)) && m.windows(2).all(|w| w[0] != w[1]))
            .collect();
        ok &= surviving.is_empty();
        equations.push(json!({"equation_id":id,"top_degree":terms.first().map(Vec::len),"top_terms":terms.len(),"top_terms_sha256":sha(&serde_json::to_vec(terms).unwrap()),"surviving_terms":surviving}));
    }
    json!({"retained_variables_zero_based":keep,"target_quotient":"F2[retained]/(each retained variable squared)","projection_zero_for_every_generator":ok,"quotient_socle_degree":keep.len(),"candidate_dreg_lower_bound":if ok{Some(keep.len()+1)}else{None},"equations":equations})
}
fn s3(x: &str, y: &str, z: &str, a: &str, b: &str) -> String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+{a})+2*{b})*({z})+(({x})*({y})-{a})^2-4*{b}*(({x})+({y}))")
}
fn positions(input: &Value) -> Vec<usize> {
    let p: Vec<_> = input["coordinate_bit_positions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect();
    let arm = input["arm"].as_str().unwrap();
    let last = if arm == "baseline" {
        26
    } else {
        arm.strip_prefix("spike")
            .and_then(|bit| bit.parse::<usize>().ok())
            .filter(|bit| (27..=40).contains(bit))
            .expect("candidate arm must be spike27 through spike40")
    };
    let expected: Vec<_> = (0..27).map(|j| if j == 26 { last } else { j }).collect();
    assert_eq!(p, expected);
    p
}
fn prime(input: &Value, text: &str) -> Value {
    let pos = positions(input);
    let m = input["m"].as_u64().unwrap() as usize;
    let ell = 27;
    let nb = m * ell;
    let nv = nb + m - 2;
    let lines: Vec<_> = text.lines().collect();
    assert_eq!(
        lines[0],
        format!(
            "ring r=(integer,{}),(v(1..{nv})),dp;",
            input["field_modulus"].as_str().unwrap()
        )
    );
    assert_eq!(input["n_vars"], nv);
    assert_eq!(input["input_equations"], m - 1 + nb);
    for i in 0..m {
        let expected = format!(
            "poly x{i}={};",
            (0..ell)
                .map(|j| format!("{}*v({})", 1u128 << pos[j], i * ell + j + 1))
                .collect::<Vec<_>>()
                .join("+")
        );
        assert_eq!(lines.iter().filter(|l| **l == expected).count(), 1);
    }
    let mut proof = vec![];
    for k in 0..m - 1 {
        let target = input["target_x"].as_str().unwrap();
        let (x, y, z) = if m == 2 {
            ("x0".into(), "x1".into(), target.into())
        } else if k == 0 {
            ("x0".into(), "x1".into(), format!("v({})", nb + 1))
        } else if k == m - 2 {
            (
                format!("v({})", nb + k),
                format!("x{}", k + 1),
                target.into(),
            )
        } else {
            (
                format!("v({})", nb + k),
                format!("x{}", k + 1),
                format!("v({})", nb + k + 1),
            )
        };
        let expected = format!(
            "I[{}]={};",
            k + 1,
            s3(
                &x,
                &y,
                &z,
                input["curve_a"].as_str().unwrap(),
                input["curve_b"].as_str().unwrap()
            )
        );
        assert_eq!(lines.iter().filter(|l| **l == expected).count(), 1);
        // x and y are nonzero linear forms in disjoint variable blocks. The
        // coefficient of x_first^2*y_first^2 is 1 in their coordinates, so degree4.
        let xfirst = if k == 0 { 0 } else { nb + k - 1 };
        let yfirst = (k + 1) * ell;
        proof.push(json!({"equation_id":k+1,"source_line_sha256":sha(expected.as_bytes()),"degree":4,"nonzero_top_monomial_zero_based":[xfirst,xfirst,yfirst,yfirst],"coefficient":"1","top_coordinate_formula":if k==m-2{"x^2*y^2"}else{"x^2*y^2+x^2*z^2+y^2*z^2-2*x^2*y*z-2*x*y^2*z-2*x*y*z^2"},"projection_reason":if m==2{"x1 retains one selector only, so its square is zero in the target quotient"}else{"all base selectors and all but u0 map to zero; every top term uses at least two distinct coordinate blocks"}}));
    }
    for i in 0..nb {
        let expected = format!("I[{}]=v({})^2-v({});", m + i, i + 1, i + 1);
        assert_eq!(lines.iter().filter(|l| **l == expected).count(), 1);
    }
    assert_eq!(
        lines.iter().filter(|l| l.starts_with("I[")).count(),
        nb + m - 1
    );
    assert_eq!(lines.iter().filter(|l| l.starts_with("poly ")).count(), m);
    if m == 2 {
        let mut keep: Vec<_> = (0..ell).collect();
        keep.push(ell);
        json!({"status":"finite_interval","dreg_lower_bound":ell+2,"dreg_upper_bound":nv+1,"top_quotient_dimension_lower_bound":0,"retained_variables_zero_based":keep,"target_quotient":"Fp[28 retained selectors]/(each retained selector squared)","quotient_socle_degree":ell+1,"equations":proof,"argument":"Every degree4 top S3 term contains x1 squared; retained x1 is one selector, so zero modulo its square. Original selector top equations bound dreg above by nvars+1."})
    } else {
        json!({"status":"positive_dimensional_top_ideal","dreg_lower_bound":null,"dreg_upper_bound":null,"top_quotient_dimension_lower_bound":1,"retained_variables_zero_based":[nb],"target_quotient":"Fp[u0], no field equation on u0 in the saved presentation","degree_of_regularity_convention":"No finite first degree at which the graded quotient vanishes; this is not a claim about Castelnuovo-Mumford regularity","equations":proof,"argument":"Graded quotient surjects onto the polynomial ring Fp[u0]; every top generator maps to zero. All powers of u0 survive in that quotient."})
    }
}
fn certify(input: &Value, text: &str) -> Value {
    positions(input);
    assert_eq!(input["ell"], 27);
    assert_eq!(input["n"], 83);
    assert_eq!(input["source_sha256"], sha(text.as_bytes()));
    let nv = input["n_vars"].as_u64().unwrap() as usize;
    let m = input["m"].as_u64().unwrap() as usize;
    let mut r = if input["family"] == "prime" {
        prime(input, text)
    } else {
        assert_eq!(
            text.lines().next().unwrap(),
            format!("ring r=2,(v(1..{nv})),dp;")
        );
        let polys = parse_binary(text, nv);
        assert_eq!(
            polys.len(),
            input["input_equations"].as_u64().unwrap() as usize
        );
        assert!(
            polys.iter().all(|(_, ts)| !ts.iter().any(Vec::is_empty)),
            "constant top generator needs separate zero-quotient certificate"
        );
        let mut maps: Vec<(&str, BTreeSet<usize>)> = vec![
            ("first_block27", (0..27).collect()),
            ("first_block26", (0..26).collect()),
            ("all_base_selectors", (0..m * 27).collect()),
            (
                "low26_each_block",
                (0..m * 27).filter(|v| v % 27 < 26).collect(),
            ),
        ];
        maps.push(("greedy_ascending", greedy(&polys, nv, false)));
        maps.push(("greedy_descending", greedy(&polys, nv, true)));
        let candidates: Vec<_> = maps
            .iter()
            .map(|(name, keep)| {
                let mut p = projection(&polys, keep);
                p["name"] = json!(name);
                p
            })
            .collect();
        let lower = candidates
            .iter()
            .filter_map(|c| c["candidate_dreg_lower_bound"].as_u64())
            .max();
        let linear = linear_upper(&polys, nv);
        let upper = linear["dreg_upper_bound"].as_u64().unwrap();
        if let Some(lo) = lower {
            assert!(lo <= upper);
        }
        json!({"status":if lower.is_some(){"finite_interval"}else{"unresolved"},"dreg_lower_bound":lower,"dreg_upper_bound":upper,"top_quotient_dimension_lower_bound":0,"candidates":candidates,"linear_top_certificate":linear,"argument":"Each successful projection gives a graded surjection onto a square-zero ring. Original field tops and r independent original linear tops generate a subideal whose quotient vanishes above degree nvars-r. Bounds describe original top ideal, with no added affine consequences."})
    };
    r["schema"] = json!("n83.top-quotient-certificate/v1");
    r["input"] = input.clone();
    r["source_sha256"] = json!(sha(text.as_bytes()));
    r
}
fn greedy(polys: &[(usize, Vec<Vec<usize>>)], n: usize, reverse: bool) -> BTreeSet<usize> {
    let edges: BTreeSet<Vec<usize>> = polys
        .iter()
        .flat_map(|(_, t)| t.iter())
        .filter(|m| m.windows(2).all(|w| w[0] != w[1]))
        .cloned()
        .collect();
    let edges: Vec<_> = edges.into_iter().collect();
    let mut adjacent = vec![vec![]; n];
    for (j, edge) in edges.iter().enumerate() {
        for v in edge {
            adjacent[*v].push(j);
        }
    }
    let mut keep = BTreeSet::new();
    let mut order: Vec<_> = (0..n).collect();
    if reverse {
        order.reverse();
    }
    for v in order {
        if !adjacent[v]
            .iter()
            .any(|j| edges[*j].iter().all(|u| *u == v || keep.contains(u)))
        {
            keep.insert(v);
        }
    }
    keep
}
fn linear_upper(polys: &[(usize, Vec<Vec<usize>>)], n: usize) -> Value {
    let mut basis: BTreeMap<usize, (BTreeSet<usize>, BTreeSet<usize>)> = BTreeMap::new();
    for (id, terms) in polys
        .iter()
        .filter(|(_, t)| !t.is_empty() && t[0].len() == 1)
    {
        let mut row: BTreeSet<_> = terms
            .iter()
            .map(|t| {
                assert_eq!(t.len(), 1);
                t[0]
            })
            .collect();
        let mut combination: BTreeSet<_> = [*id].into();
        while let Some(p) = row.iter().next_back().copied() {
            if let Some((old, ids)) = basis.get(&p) {
                row = row.symmetric_difference(old).copied().collect();
                combination = combination.symmetric_difference(ids).copied().collect();
            } else {
                basis.insert(p, (row, combination));
                break;
            }
        }
    }
    json!({"rank":basis.len(),"dreg_upper_bound":n-basis.len()+1,"basis":basis.into_iter().map(|(pivot,(variables,equation_ids))|json!({"pivot_zero_based":pivot,"variables_zero_based":variables,"original_equation_ids_xor":equation_ids})).collect::<Vec<_>>(),"argument":"All variable squares are original field tops. In characteristic2 any linear coordinate change preserves their ideal. Quotient by r independent linear tops plus squares is a square-zero polynomial ring on n-r variables; the original top quotient is its graded quotient."})
}
fn prime_isomorphism(a: &Value, b: &Value) -> Value {
    let pa = positions(a);
    let pb = positions(b);
    let p = 4835703278761981908090881u128;
    assert_eq!(a["field_modulus"], p.to_string());
    assert_eq!(b["field_modulus"], p.to_string());
    for key in ["m", "n_vars", "curve_a", "curve_b", "target_x"] {
        assert_eq!(a[key], b[key]);
    }
    let m = a["m"].as_u64().unwrap() as usize;
    let nv = a["n_vars"].as_u64().unwrap() as usize;
    let mut map = vec![];
    for v in 0..nv {
        let scale = if v < m * 27 {
            assert!(pb[v % 27] >= pa[v % 27]);
            1u128 << (pb[v % 27] - pa[v % 27])
        } else {
            1
        };
        let inverse = (0..scale)
            .find_map(|k| {
                let x = k * p + 1;
                if x % scale == 0 {
                    Some(x / scale)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(scale * inverse % p, 1);
        if v < m * 27 {
            assert_eq!((1u128 << pa[v % 27]) * scale % p, (1u128 << pb[v % 27]) % p);
        }
        map.push(json!({"variable_zero_based":v,"scale":scale.to_string(),"inverse":inverse.to_string(),"field_top_multiplier":if v<m*27{Some((scale*scale%p).to_string())}else{None}}));
    }
    json!({"schema":"n83.prime-top-isomorphism/v1","id":a["id"],"baseline_source_sha256":a["source_sha256"],"spike40_source_sha256":b["source_sha256"],"field_modulus":p.to_string(),"map_direction":"Substitute baseline variable v_i by scale_i times spike40 variable v_i","variables":map,"verified_graded_quotient_isomorphism":true,"argument":"Both complete source presentations have separately passed prime() validation. Each selector linear form maps exactly to its counterpart; intermediate coordinates are unchanged. Every original S3 highest form therefore maps to the matching highest form. Each selector square maps to its recorded nonzero scalar multiple. Invertible diagonal substitution maps the two homogeneous ideals onto each other; graded Hilbert functions and graded vanishing degrees coincide. This is not an isomorphism claim for the affine Boolean selector ideals."})
}
fn paired(root: &Path, out: &Path, replay: bool) {
    let indices = [
        read(&root.join("baseline/inputs/inputs.json")),
        read(&root.join("spike40/inputs/inputs.json")),
    ];
    for i in &indices {
        assert_eq!(i.as_array().unwrap().len(), 48);
    }
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let mut rows = vec![];
    let mut replay_rows = vec![];
    let mut prime_isomorphisms = vec![];
    for j in 0..48 {
        let a = &indices[0][j];
        let b = &indices[1][j];
        for field in [
            "id",
            "family",
            "m",
            "target_kind",
            "target_x",
            "target_y",
            "points",
        ] {
            assert_eq!(a[field], b[field], "paired field {field}");
        }
        for (arm, input) in [("baseline", a), ("spike40", b)] {
            let id = input["id"].as_str().unwrap();
            let key = format!("{arm}-{id}");
            let path = out.join(format!("{key}.json"));
            let computed = std::panic::catch_unwind(|| {
                let text =
                    fs::read_to_string(root.join(arm).join("inputs").join(id).join("system.sing"))
                        .unwrap();
                let certificate = certify(input, &text);
                if replay {
                    assert_eq!(read(&path), certificate);
                } else {
                    save(&path, &certificate);
                }
                certificate
            });
            match computed {
                Ok(c) => {
                    rows.push(json!({"key":key,"arm":arm,"id":id,"family":input["family"],"m":input["m"],"target_kind":input["target_kind"],"ell":27,"n_vars":input["n_vars"],"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"status":c["status"],"dreg_lower_bound":c["dreg_lower_bound"],"dreg_upper_bound":c["dreg_upper_bound"],"linear_top_rank":c["linear_top_certificate"]["rank"],"top_quotient_dimension_lower_bound":c["top_quotient_dimension_lower_bound"],"source_sha256":c["source_sha256"],"certificate":format!("{key}.json")}));
                    replay_rows.push(
                        json!({"key":key,"verified":true,"source_sha256":c["source_sha256"]}),
                    );
                }
                Err(e) => {
                    let error = e
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|x| x.to_string()))
                        .unwrap_or_else(|| "panic".into());
                    rows.push(json!({"key":key,"arm":arm,"id":id,"family":input["family"],"m":input["m"],"target_kind":input["target_kind"],"status":"error","error":error}));
                    replay_rows.push(json!({"key":key,"verified":false,"error":error}));
                }
            }
            println!(
                "{}",
                json!({"key":key,"replay":replay,"verified":replay_rows.last().unwrap()["verified"]})
            );
            if !replay {
                save(&out.join("partial-summary.json"), &json!({"rows":rows}));
            }
        }
        if a["family"] == "prime"
            && replay_rows[replay_rows.len() - 2..]
                .iter()
                .all(|r| r["verified"] == true)
        {
            let proof = prime_isomorphism(a, b);
            let file = format!("isomorphism-{}.json", a["id"].as_str().unwrap());
            if replay {
                assert_eq!(read(&out.join(&file)), proof);
            } else {
                save(&out.join(&file), &proof);
            }
            prime_isomorphisms.push(json!({"id":a["id"],"certificate":file,"verified":true}));
        }
    }
    if replay {
        save(
            &out.parent().unwrap().join("replay.json"),
            &json!({"schema":"n83.paired-top-replay/v1","rows":replay_rows,"prime_isomorphisms":prime_isomorphisms,"all_verified":replay_rows.iter().all(|r|r["verified"]==true),"independent_review":false}),
        );
    } else {
        save(
            &out.join("summary.json"),
            &json!({"schema":"n83.paired-top-summary/v1","rows":rows,"prime_isomorphisms":prime_isomorphisms,"scope":"Original saved top-form ideals; intervals are not exact regularity or LFD/solving-degree values. Greedy maps are not claimed optimal."}),
        );
    }
}
fn single(root: &Path, out: &Path, replay: bool) {
    let inputs: Vec<Value> =
        serde_json::from_slice(&fs::read(root.join("inputs.json")).unwrap()).unwrap();
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let mut rows = Vec::new();
    for input in inputs {
        let id = input["id"].as_str().unwrap();
        let source = fs::read_to_string(root.join(id).join("system.sing")).unwrap();
        let certificate = certify(&input, &source);
        let certificate_path = out.join(format!("{id}.json"));
        if replay {
            assert_eq!(
                read(&certificate_path),
                certificate,
                "certificate replay mismatch for {id}"
            );
        } else {
            save(&certificate_path, &certificate);
        }
        rows.push(json!({"id":id,"family":input["family"],"m":input["m"],"target_kind":input["target_kind"],"arm":input["arm"],"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"status":certificate["status"],"dreg_lower_bound":certificate["dreg_lower_bound"],"dreg_upper_bound":certificate["dreg_upper_bound"],"source_sha256":certificate["source_sha256"],"certificate":format!("{id}.json")}));
    }
    if replay {
        save(
            &out.parent().unwrap().join("replay-summary.json"),
            &json!({"schema":"n83.single-top-replay/v1","all_verified":true,"rows":rows}),
        );
    } else {
        save(
            &out.join("summary.json"),
            &json!({"schema":"n83.single-top-summary/v1","rows":rows,"scope":"Original highest-homogeneous-form quotient certificates for one candidate base. These are top-form graded-quotient measurements, not affine solving degree or a solver-cost result."}),
        );
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "run" => paired(Path::new(&a[2]), Path::new(&a[3]), false),
        "replay" => paired(Path::new(&a[2]), Path::new(&a[3]), true),
        "single" => single(Path::new(&a[2]), Path::new(&a[3]), false),
        "single-replay" => single(Path::new(&a[2]), Path::new(&a[3]), true),
        _ => panic!("run/replay PAIRED_RAW CERT_DIR | single/single-replay INPUT_ROOT CERT_DIR"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn prime_fixture(m: usize) -> (Value, String) {
        let nb = 27 * m;
        let nv = nb + m - 2;
        let input = json!({"m":m,"n_vars":nv,"input_equations":nb+m-1,"field_modulus":"4835703278761981908090881","curve_a":"2","curve_b":"3","target_x":"5","arm":"baseline","coordinate_bit_positions":(0..27).collect::<Vec<_>>()});
        let mut t =
            format!("ring r=(integer,4835703278761981908090881),(v(1..{nv})),dp;\nideal I;\n");
        for i in 0..m {
            t += &format!(
                "poly x{i}={};\n",
                (0..27)
                    .map(|j| format!("{}*v({})", 1u128 << j, i * 27 + j + 1))
                    .collect::<Vec<_>>()
                    .join("+")
            );
        }
        for k in 0..m - 1 {
            let (x, y, z) = if m == 2 {
                ("x0".to_string(), "x1".to_string(), "5".to_string())
            } else if k == 0 {
                ("x0".to_string(), "x1".to_string(), format!("v({})", nb + 1))
            } else if k == m - 2 {
                (
                    format!("v({})", nb + k),
                    format!("x{}", k + 1),
                    "5".to_string(),
                )
            } else {
                (
                    format!("v({})", nb + k),
                    format!("x{}", k + 1),
                    format!("v({})", nb + k + 1),
                )
            };
            t += &format!("I[{}]={};\n", k + 1, s3(&x, &y, &z, "2", "3"));
        }
        for i in 0..nb {
            t += &format!("I[{}]=v({})^2-v({});\n", m + i, i + 1, i + 1);
        }
        (input, t)
    }
    #[test]
    fn prime_structural_certificates() {
        let (i, t) = prime_fixture(2);
        assert_eq!(prime(&i, &t)["dreg_lower_bound"], 29);
        let (i, t) = prime_fixture(3);
        assert_eq!(prime(&i, &t)["top_quotient_dimension_lower_bound"], 1);
        assert!(
            std::panic::catch_unwind(|| prime(&i, &t.replace("poly x0=1*", "poly x0=7*"))).is_err()
        );
    }
    #[test]
    fn projection_positive_and_negative() {
        let p = parse_binary("I[1]=v(1)*v(2);\nI[2]=v(1)^2-v(1);\nI[3]=v(2)^2-v(2);", 2);
        assert_eq!(projection(&p, &[0].into())["candidate_dreg_lower_bound"], 2);
        assert_eq!(
            projection(&p, &[0, 1].into())["projection_zero_for_every_generator"],
            false
        );
    }
    #[test]
    fn linear_top_must_not_be_dropped() {
        let p = parse_binary("I[1]=v(1)+1;\nI[2]=v(1)^2-v(1);", 1);
        assert_eq!(
            projection(&p, &[0].into())["projection_zero_for_every_generator"],
            false
        );
    }
    #[test]
    #[should_panic]
    fn missing_field_rejected() {
        parse_binary("I[1]=v(1)*v(2);\nI[2]=v(1)^2-v(1);", 2);
    }
    #[test]
    fn cancellation_precedes_top_extraction() {
        let p = parse_binary(
            "I[1]=v(1)*v(2)+v(1)*v(2)+v(1);\nI[2]=v(1)^2-v(1);\nI[3]=v(2)^2-v(2);",
            2,
        );
        assert_eq!(p[0].1, vec![vec![0]]);
    }
    #[test]
    fn spike_prime_weights_checked() {
        for m in [2, 3] {
            let (mut i, t) = prime_fixture(m);
            i["arm"] = json!("spike40");
            i["coordinate_bit_positions"][26] = json!(40);
            let changed = t.replace("67108864*", "1099511627776*");
            assert!(std::panic::catch_unwind(|| prime(&i, &t)).is_err());
            assert!(prime(&i, &changed)["status"].is_string());
        }
    }
    #[test]
    fn linear_upper_has_original_equation_witness() {
        let p=parse_binary("I[1]=v(1)+v(2);\nI[2]=v(2)+v(3);\nI[3]=v(1)+v(3);\nI[4]=v(1)^2-v(1);\nI[5]=v(2)^2-v(2);\nI[6]=v(3)^2-v(3);",3);
        let c = linear_upper(&p, 3);
        assert_eq!(c["rank"], 2);
        assert_eq!(c["dreg_upper_bound"], 2);
        for row in c["basis"].as_array().unwrap() {
            let mut actual = BTreeSet::new();
            for id in row["original_equation_ids_xor"].as_array().unwrap() {
                for term in &p
                    .iter()
                    .find(|(i, _)| *i == id.as_u64().unwrap() as usize)
                    .unwrap()
                    .1
                {
                    if !actual.remove(&term[0]) {
                        actual.insert(term[0]);
                    }
                }
            }
            assert_eq!(json!(actual), row["variables_zero_based"]);
        }
    }
    #[test]
    fn greedy_maps_recheck_every_generator() {
        let p=parse_binary("I[1]=v(1)*v(2)+v(2)*v(3);\nI[2]=v(1);\nI[3]=v(1)^2-v(1);\nI[4]=v(2)^2-v(2);\nI[5]=v(3)^2-v(3);",3);
        for reverse in [false, true] {
            let k = greedy(&p, 3, reverse);
            assert!(!k.contains(&0));
            assert_eq!(
                projection(&p, &k)["projection_zero_for_every_generator"],
                true
            );
        }
    }
    #[test]
    #[should_panic]
    fn duplicate_equation_rejected() {
        parse_binary("I[1]=v(1);\nI[1]=v(1)^2-v(1);", 1);
    }
    #[test]
    fn prime_diagonal_map_invertible_and_weight_matched() {
        for m in 2..=9 {
            let (a, t) = prime_fixture(m);
            prime(&a, &t);
            for r in 27..=40 {
                let mut b = a.clone();
                b["arm"] = json!(format!("spike{r}"));
                b["coordinate_bit_positions"][26] = json!(r);
                let candidate_weight = (1u128 << r).to_string();
                let candidate_text = t.replace("67108864*", &format!("{candidate_weight}*"));
                prime(&b, &candidate_text);
                let proof = prime_isomorphism(&a, &b);
                assert_eq!(
                    proof["variables"][26]["scale"],
                    (1u128 << (r - 26)).to_string()
                );
                assert_eq!(proof["verified_graded_quotient_isomorphism"], true);
            }
            let mut bad = a.clone();
            bad["arm"] = json!("spike41");
            bad["coordinate_bit_positions"][26] = json!(41);
            assert!(std::panic::catch_unwind(|| prime_isomorphism(&a, &bad)).is_err());
        }
    }
}
