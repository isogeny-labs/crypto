//! Graded quotient certificates for the exact fixed-base input presentations.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
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
    for l in text.lines().filter(|l| l.starts_with("I[")) {
        let (a, b) = l.split_once('=').unwrap();
        let id = a[2..a.len() - 1].parse().unwrap();
        let b = b.strip_suffix(';').unwrap();
        if let Some((x, y)) = b.split_once("^2-") {
            assert_eq!(x, y);
            fields.insert(index(x, n));
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
fn prime(input: &Value, text: &str) -> Value {
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
                .map(|j| format!("{}*v({})", 1u128 << j, i * ell + j + 1))
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
        let candidates: Vec<_> = [27, m * 27]
            .iter()
            .map(|k| projection(&polys, &(0..*k).collect()))
            .collect();
        let lower = candidates
            .iter()
            .filter_map(|c| c["candidate_dreg_lower_bound"].as_u64())
            .max();
        json!({"status":if lower.is_some(){"finite_interval"}else{"unresolved"},"dreg_lower_bound":lower,"dreg_upper_bound":nv+1,"top_quotient_dimension_lower_bound":0,"candidates":candidates,"argument":"A verified projection gives a surjection onto a square-zero polynomial ring on k retained variables, whose degree-k squarefree monomial survives. All nvars original top field equations are variable squares, so all degrees above nvars vanish."})
    };
    r["schema"] = json!("n83.top-quotient-certificate/v1");
    r["input"] = input.clone();
    r["source_sha256"] = json!(sha(text.as_bytes()));
    r
}
fn run(root: &Path, out: &Path, replay: bool) {
    let inputs = read(&root.join("inputs.json"));
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let mut rows = vec![];
    for input in inputs.as_array().unwrap() {
        let id = input["id"].as_str().unwrap();
        let t = fs::read_to_string(root.join(id).join("system.sing")).unwrap();
        let c = certify(input, &t);
        if replay {
            assert_eq!(read(&out.join(format!("{id}.json"))), c);
        } else {
            save(&out.join(format!("{id}.json")), &c);
        }
        let row = json!({"id":id,"family":input["family"],"m":input["m"],"target_kind":input["target_kind"],"ell":27,"n_vars":input["n_vars"],"status":c["status"],"dreg_lower_bound":c["dreg_lower_bound"],"dreg_upper_bound":c["dreg_upper_bound"],"top_quotient_dimension_lower_bound":c["top_quotient_dimension_lower_bound"],"source_sha256":c["source_sha256"],"certificate":format!("{id}.json")});
        println!("{}", json!({"id":id,"status":c["status"],"replay":replay}));
        rows.push(row);
    }
    let summary = json!({"schema":"n83.top-quotient-summary/v1","rows":rows,"scope":"Top-form graded quotient of the exact saved input list. Bounds do not measure solving degree, LFD, performance or a change in factor-base quality."});
    if replay {
        assert_eq!(read(&out.join("summary.json")), summary)
    } else {
        save(&out.join("summary.json"), &summary)
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "run" => run(Path::new(&a[2]), Path::new(&a[3]), false),
        "replay" => run(Path::new(&a[2]), Path::new(&a[3]), true),
        _ => panic!("run/replay INPUTS CERT_DIR"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn prime_fixture(m: usize) -> (Value, String) {
        let nb = 27 * m;
        let nv = nb + m - 2;
        let input = json!({"m":m,"n_vars":nv,"input_equations":nb+m-1,"field_modulus":"4835703278761981908090881","curve_a":"2","curve_b":"3","target_x":"5"});
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
        if m == 2 {
            t += &format!("I[1]={};\n", s3("x0", "x1", "5", "2", "3"));
        } else {
            assert_eq!(m, 3);
            t += &format!(
                "I[1]={};\nI[2]={};\n",
                s3("x0", "x1", "v(82)", "2", "3"),
                s3("v(82)", "x2", "5", "2", "3")
            );
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
}
