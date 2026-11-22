//! Exact low-degree fall certificates on the preserved n83 input grid.
//! A rank deficit alone is never reported as a degree fall.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

type Mon = Vec<u16>;
type Poly = BTreeSet<Mon>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, x: &Value) {
    fs::write(p, serde_json::to_string_pretty(x).unwrap() + "\n").unwrap();
}
fn degree(p: &Poly) -> usize {
    p.iter().map(Vec::len).max().unwrap_or(0)
}
fn toggle(p: &mut Poly, m: Mon) {
    if !p.remove(&m) {
        p.insert(m);
    }
}
fn add(p: &mut Poly, q: &Poly) {
    for m in q {
        toggle(p, m.clone());
    }
}
#[cfg(test)]
fn product(p: &Poly, var: u16) -> Poly {
    let mut out = Poly::new();
    for m in p {
        let mut t = m.clone();
        t.push(var);
        t.sort_unstable();
        t.dedup();
        toggle(&mut out, t);
    }
    out
}
fn parse_binary(text: &str, nv: usize) -> Vec<(usize, Poly)> {
    let mut out = Vec::new();
    for line in text.lines().filter(|l| l.starts_with("I[")) {
        let (left, expr) = line.split_once('=').unwrap();
        let id: usize = left[2..left.len() - 1].parse().unwrap();
        let expr = expr.strip_suffix(';').unwrap();
        if expr.contains('^') {
            let (v, tail) = expr.split_once("^2-").unwrap();
            assert_eq!(
                v, tail,
                "Only explicit Boolean field equations may be omitted"
            );
            let index: usize = v
                .strip_prefix("v(")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .parse()
                .unwrap();
            assert!((1..=nv).contains(&index));
            out.push((id, Poly::new()));
            continue;
        }
        let mut poly = Poly::new();
        for term in expr.split('+') {
            if term == "0" {
                continue;
            }
            let mut m = Vec::new();
            if term != "1" {
                for v in term.split('*') {
                    let i: u16 = v
                        .strip_prefix("v(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .parse()
                        .unwrap();
                    assert!(i > 0 && usize::from(i) <= nv);
                    m.push(i - 1);
                }
            }
            m.sort_unstable();
            m.dedup();
            toggle(&mut poly, m);
        }
        out.push((id, poly));
    }
    out
}
fn bit(row: &[u64], i: usize) -> bool {
    row[i / 64] >> (i % 64) & 1 == 1
}
fn bxor(a: &mut [u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= y;
    }
}
fn leading(row: &[u64]) -> Option<usize> {
    row.iter()
        .enumerate()
        .find(|(_, w)| **w != 0)
        .map(|(i, w)| 64 * i + w.trailing_zeros() as usize)
}
fn rank(rows: &[Vec<u64>], reverse: bool) -> usize {
    let mut pivots: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for input in rows {
        let mut row = input.clone();
        loop {
            let at = if reverse {
                row.iter()
                    .enumerate()
                    .rev()
                    .find(|(_, w)| **w != 0)
                    .map(|(i, w)| 64 * i + 63 - w.leading_zeros() as usize)
            } else {
                leading(&row)
            };
            let Some(at) = at else {
                break;
            };
            if let Some(p) = pivots.get(&at) {
                bxor(&mut row, p);
            } else {
                pivots.insert(at, row);
                break;
            }
        }
    }
    pivots.len()
}
fn row(poly: &Poly, columns: &BTreeMap<Mon, usize>, n: usize) -> Vec<u64> {
    let mut out = vec![0; n.div_ceil(64)];
    for m in poly {
        let k = columns[m];
        out[k / 64] ^= 1 << (k % 64);
    }
    out
}
fn binary(input: &Value, text: &str) -> Value {
    let nv = input["n_vars"].as_u64().unwrap() as usize;
    if !input["family"].is_null() {
        assert_eq!(
            text.lines().next().unwrap(),
            format!("ring r=2,(v(1..{nv})),dp;")
        );
    }
    let polys = parse_binary(text, nv);
    assert_eq!(
        polys.len(),
        input["input_equations"].as_u64().unwrap() as usize
    );
    let mut hist = BTreeMap::<usize, usize>::new();
    for (_, p) in &polys {
        if !p.is_empty() {
            *hist.entry(degree(p)).or_default() += 1;
        }
    }
    // In these frozen public fixtures all nonzero generators must have degree >=2.
    // This is checked, not assumed from the curve family; otherwise fail closed.
    if polys.iter().any(|(_, p)| !p.is_empty() && degree(p) < 2) {
        return json!({"status":"unsupported_lower_input_degree","first_fall_degree":null,"input_degree_histogram":hist});
    }
    let quadratic: Vec<_> = polys.iter().filter(|(_, p)| degree(p) == 2).collect();
    let mut mons: BTreeSet<Mon> = BTreeSet::new();
    for (_, p) in &quadratic {
        mons.extend(p.iter().cloned());
    }
    let mut columns: Vec<_> = mons.into_iter().collect();
    columns.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
    let ntop = columns.iter().take_while(|m| m.len() == 2).count();
    let map: BTreeMap<_, _> = columns
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, m)| (m, i))
        .collect();
    let full: Vec<_> = quadratic
        .iter()
        .map(|(_, p)| row(p, &map, columns.len()))
        .collect();
    let top: Vec<Vec<u64>> = full
        .iter()
        .map(|r| {
            let mut t = r.clone();
            for j in ntop..columns.len() {
                t[j / 64] &= !(1 << (j % 64));
            }
            t
        })
        .collect();
    let rfull = rank(&full, false);
    let rtop = rank(&top, false);
    assert_eq!(rfull, rank(&full, true));
    assert_eq!(rtop, rank(&top, true));
    let mut pivots: BTreeMap<usize, (Vec<u64>, Vec<u64>)> = BTreeMap::new();
    let mut witness = None;
    for (i, initial) in full.iter().enumerate() {
        let mut r = initial.clone();
        let mut combination = vec![0u64; quadratic.len().div_ceil(64)];
        combination[i / 64] |= 1 << (i % 64);
        loop {
            let Some(k) = leading(&r) else {
                break;
            };
            if k >= ntop {
                let chosen: Vec<_> = (0..quadratic.len())
                    .filter(|j| bit(&combination, *j))
                    .map(|j| quadratic[j].0)
                    .collect();
                let result: Vec<_> = columns
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| bit(&r, *j))
                    .map(|(_, m)| m.clone())
                    .collect();
                witness = Some(
                    json!({"equation_ids":chosen,"multiplier":1,"result_monomials_zero_based":result,"result_degree":if result.iter().any(|m|!m.is_empty()){1}else{0}}),
                );
                break;
            }
            if let Some((p, c)) = pivots.get(&k) {
                bxor(&mut r, p);
                bxor(&mut combination, c);
            } else {
                pivots.insert(k, (r, combination));
                break;
            }
        }
        if witness.is_some() {
            break;
        }
    }
    assert_eq!(witness.is_some(), rfull > rtop);
    let value = json!({"status":if witness.is_some(){"certified"}else{"no_fall_through_degree_2"},"first_fall_degree":if witness.is_some(){Some(2)}else{None},"first_fall_lower_bound":if witness.is_some(){2}else{3},"last_fall_degree":null,"input_degree_histogram":hist,"quadratic_equations":quadratic.len(),"quadratic_columns":ntop,"total_columns":columns.len(),"quadratic_top_rank":rtop,"quadratic_full_rank":rfull,"new_lower_degree_dimension":rfull-rtop,"rank_verified_two_pivot_orders":true,"lower_degree_certificate":"No nonzero generators of degree 0 or 1 in the Boolean normal form; no degree fall can occur before 2","witness":witness});
    if value["status"] == "certified" {
        replay_binary(&polys, &value);
    }
    value
}
fn replay_binary(polys: &[(usize, Poly)], cert: &Value) {
    assert!(polys.iter().all(|(_, p)| p.is_empty() || degree(p) >= 2));
    let mut result = Poly::new();
    let mut seen = BTreeSet::new();
    for id in cert["witness"]["equation_ids"].as_array().unwrap() {
        let id = id.as_u64().unwrap() as usize;
        assert!(seen.insert(id));
        let p = &polys.iter().find(|(i, _)| *i == id).unwrap().1;
        assert_eq!(degree(p), 2);
        add(&mut result, p);
    }
    let expected: Poly =
        serde_json::from_value(cert["witness"]["result_monomials_zero_based"].clone()).unwrap();
    assert_eq!(result, expected);
    assert!(!result.is_empty());
    assert!(degree(&result) < 2);
    assert_eq!(cert["first_fall_degree"], 2);
}
// The prime certificate is a minor of the degree-four coefficient matrix after
// reducing selector squares. It proves absence of a new degree fall through 4,
// not an attained first fall degree. Coefficients are extracted by multiplication.
type PP = BTreeMap<Mon, i128>;
fn padd(a: &PP, b: &PP, scale: i128) -> PP {
    let mut r = a.clone();
    for (m, c) in b {
        *r.entry(m.clone()).or_default() += scale * c;
    }
    r.retain(|_, c| *c != 0);
    r
}
fn pmul(a: &PP, b: &PP, nbits: usize) -> PP {
    let mut r = PP::new();
    for (x, c) in a {
        for (y, d) in b {
            let mut m = x.clone();
            m.extend(y);
            m.sort_unstable();
            let mut n = Vec::new();
            for v in m {
                if usize::from(v) >= nbits || n.last() != Some(&v) {
                    n.push(v);
                }
            }
            *r.entry(n).or_default() += c * d;
        }
    }
    r.retain(|_, c| *c != 0);
    r
}
fn var(i: usize) -> PP {
    [(vec![i as u16], 1)].into()
}
fn selector(i: usize, ell: usize) -> PP {
    padd(&var(i * ell), &var(i * ell + 1), 2)
}
fn top_s3(x: &PP, y: &PP, z: &PP, nbits: usize) -> PP {
    let xy = pmul(x, y, nbits);
    let xm = padd(x, y, -1);
    let xp = padd(x, y, 1);
    let a = pmul(&pmul(&xm, &xm, nbits), &pmul(z, z, nbits), nbits);
    let b = pmul(&pmul(&xp, &xy, nbits), z, nbits);
    padd(&padd(&a, &b, -2), &pmul(&xy, &xy, nbits), 1)
}
fn s3text(x: &str, y: &str, z: &str, a: &str, b: &str) -> String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+{a})+2*{b})*({z})+(({x})*({y})-{a})^2-4*{b}*(({x})+({y}))")
}
fn prime(input: &Value, text: &str) -> Value {
    let m = input["m"].as_u64().unwrap() as usize;
    let ell = input["ell"].as_u64().unwrap() as usize;
    assert!(ell >= 2);
    let nbits = m * ell;
    let positions: Vec<usize> = match input.get("coordinate_bit_positions") {
        Some(v) => v.as_array().unwrap().iter().map(|x|x.as_u64().unwrap() as usize).collect(),
        None => (0..ell).collect(),
    };
    assert_eq!(positions.len(),ell);
    assert_eq!(&positions[..2], &[0,1]);
    assert!(positions.iter().all(|j| *j < 83));
    let nv = nbits + m - 2;
    assert_eq!(input["n_vars"], nv);
    let lines: Vec<_> = text.lines().collect();
    assert_eq!(
        lines[0],
        format!(
            "ring r=(integer,{}),(v(1..{nv})),dp;",
            input["field_modulus"].as_str().unwrap()
        )
    );
    let a = input["curve_a"].as_str().unwrap();
    let b = input["curve_b"].as_str().unwrap();
    let target = input["target_x"].as_str().unwrap();
    let mut minor_mons = Vec::new();
    let mut equations = Vec::new();
    for i in 0..m {
        let definition = format!(
            "poly x{i}={};",
            (0..ell)
                .map(|j| format!("{}*v({})", 1u128 << positions[j], i * ell + j + 1))
                .collect::<Vec<_>>()
                .join("+")
        );
        assert!(lines.contains(&definition.as_str()));
    }
    for k in 0..m - 1 {
        let (xs, ys, zs) = if m == 2 {
            ("x0".into(), "x1".into(), target.into())
        } else if k == 0 {
            ("x0".into(), "x1".into(), format!("v({})", nbits + 1))
        } else if k == m - 2 {
            (
                format!("v({})", nbits + k),
                format!("x{}", k + 1),
                target.into(),
            )
        } else {
            (
                format!("v({})", nbits + k),
                format!("x{}", k + 1),
                format!("v({})", nbits + k + 1),
            )
        };
        let expected = format!("I[{}]={};", k + 1, s3text(&xs, &ys, &zs, a, b));
        assert!(lines.contains(&expected.as_str()));
        if k == 0 {
            minor_mons.push(vec![0, 1, ell as u16, ell as u16 + 1]);
        } else if k == m - 2 {
            minor_mons.push(vec![
                ((m - 1) * ell) as u16,
                ((m - 1) * ell + 1) as u16,
                (nbits + k - 1) as u16,
                (nbits + k - 1) as u16,
            ]);
        } else {
            minor_mons.push(vec![
                (nbits + k - 1) as u16,
                (nbits + k - 1) as u16,
                (nbits + k) as u16,
                (nbits + k) as u16,
            ]);
        }
        let x = if k == 0 {
            selector(0, ell)
        } else {
            var(nbits + k - 1)
        };
        let y = selector(k + 1, ell);
        let poly = if k == m - 2 {
            pmul(&pmul(&x, &x, nbits), &pmul(&y, &y, nbits), nbits)
        } else {
            top_s3(&x, &y, &var(nbits + k), nbits)
        };
        equations.push(poly);
    }
    for i in 0..nbits {
        let expected = format!("I[{}]=v({})^2-v({});", m + i, i + 1, i + 1);
        assert!(lines.contains(&expected.as_str()));
    }
    assert_eq!(
        lines.iter().filter(|l| l.starts_with("I[")).count(),
        m - 1 + nbits
    );
    // All omitted selector variables are set to zero. Since retained monomials
    // involve only retained variables, this preserves their exact coefficients.
    let matrix: Vec<Vec<i128>> = equations
        .iter()
        .map(|p| {
            minor_mons
                .iter()
                .map(|mon| *p.get(mon).unwrap_or(&0))
                .collect()
        })
        .collect();
    let mut det = 1i128;
    for i in 0..m - 1 {
        for j in 0..m - 1 {
            if i != j {
                assert_eq!(matrix[i][j], 0);
            }
        }
        assert_ne!(matrix[i][i], 0);
        det *= matrix[i][i];
    }
    let p: u128 = input["field_modulus"].as_str().unwrap().parse().unwrap();
    assert!(p > det.unsigned_abs());
    json!({"status":"certified_no_fall_through_degree_4","first_fall_degree":null,"first_fall_lower_bound":5,"last_fall_degree":null,"degree_four_rank":m-1,"minor_monomials_zero_based":minor_mons,"minor_coefficients":matrix,"minor_determinant":det.to_string(),"lower_degree_certificate":"Below degree 4 only selector equations occur. They are a monic degree-2 Groebner basis with disjoint leading variables, so their filtered ideal has no new degree falls. Modulo their top squares the S3 degree-4 parts have the displayed nonsingular minor.","input_presentation_verified":true})
}
fn control(polys: Vec<Poly>, expected: Option<usize>) -> Value {
    let mut text = String::new();
    for (i, p) in polys.iter().enumerate() {
        if p.is_empty() {
            text += &format!("I[{}]=0;\n", i + 1);
            continue;
        }
        text += &format!(
            "I[{}]={};\n",
            i + 1,
            p.iter()
                .map(|m| if m.is_empty() {
                    "1".into()
                } else {
                    m.iter()
                        .map(|v| format!("v({})", v + 1))
                        .collect::<Vec<_>>()
                        .join("*")
                })
                .collect::<Vec<_>>()
                .join("+")
        );
    }
    let result = binary(&json!({"n_vars":4,"input_equations":polys.len()}), &text);
    assert_eq!(result["first_fall_degree"], json!(expected));
    result
}
fn selfcheck() -> Value {
    let xy = vec![0, 1];
    let x = vec![0];
    let y = vec![1];
    let positive = control(
        vec![[xy.clone(), x].into(), [xy.clone(), y].into()],
        Some(2),
    );
    let duplicate = control(vec![[xy.clone()].into(), [xy.clone()].into()], None);
    let independent = control(vec![[xy].into(), [vec![2, 3]].into()], None);
    let zero = control(vec![Poly::new(), [vec![0, 1]].into()], None);
    json!({"positive":positive,"duplicate_negative":duplicate,"homogeneous_negative":independent,"zero_negative":zero})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("selfcheck") => println!("{}", selfcheck()),
        Some("run") => {
            let root = Path::new(&a[2]);
            let out = Path::new(&a[3]);
            fs::create_dir(out).unwrap();
            let inputs = read(&root.join("inputs.json"));
            let mut results = Vec::new();
            for input in inputs.as_array().unwrap() {
                let id = input["id"].as_str().unwrap();
                let raw = fs::read(root.join(id).join("system.sing")).unwrap();
                assert_eq!(sha(&raw), input["source_sha256"]);
                let text = std::str::from_utf8(&raw).unwrap();
                let mut result = if input["family"] == "prime" {
                    prime(input, text)
                } else {
                    binary(input, text)
                };
                result["input"] = input.clone();
                result["system_sha256"] = json!(sha(&raw));
                result["schema"] = json!("degree-fall.certificate/v1");
                save(&out.join(format!("{id}.json")), &result);
                if let Ok(uri) = std::env::var("ISOLAB_RESULTS_URI") {
                    let status = std::process::Command::new("aws")
                        .args(["s3", "cp"])
                        .arg(out.join(format!("{id}.json")))
                        .arg(format!(
                            "{}/certificates/{id}.json",
                            uri.trim_end_matches('/')
                        ))
                        .arg("--only-show-errors")
                        .status()
                        .unwrap();
                    assert!(status.success(), "Certificate upload failed");
                }
                println!(
                    "{}",
                    json!({"id":id,"status":result["status"],"first_fall_degree":result["first_fall_degree"],"first_fall_lower_bound":result["first_fall_lower_bound"]})
                );
                results.push(result);
            }
            save(&out.join("results.json"), &json!(results));
        }
        Some("replay") => {
            let root = Path::new(&a[2]);
            let result = read(Path::new(&a[3]));
            let input = &result["input"];
            let raw =
                fs::read(root.join(input["id"].as_str().unwrap()).join("system.sing")).unwrap();
            assert_eq!(sha(&raw), result["system_sha256"]);
            let text = std::str::from_utf8(&raw).unwrap();
            if input["family"] == "prime" {
                let fresh = prime(input, text);
                for key in [
                    "minor_coefficients",
                    "minor_determinant",
                    "first_fall_lower_bound",
                ] {
                    assert_eq!(fresh[key], result[key]);
                }
            } else {
                assert_eq!(result["status"], "certified");
                replay_binary(
                    &parse_binary(text, input["n_vars"].as_u64().unwrap() as usize),
                    &result,
                );
            }
            println!(
                "{}",
                json!({"input":input["id"],"verified":true,"kind":"explicit polynomial witness or coefficient-minor replay; not independent research review"})
            );
        }
        _ => panic!("selfcheck | run INPUTS FRESH_OUTPUT | replay INPUTS CERTIFICATE"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controls() {
        selfcheck();
    }
    #[test]
    fn product_reduces_boolean_squares() {
        assert_eq!(
            product(&[vec![0], vec![0, 1]].into(), 0),
            [vec![0], vec![0, 1]].into()
        );
    }
    #[test]
    fn malformed_field_equation_rejected() {
        assert!(std::panic::catch_unwind(|| parse_binary("I[1]=v(1)^2-v(2);", 2)).is_err());
    }
    #[test]
    fn parser_cancels_duplicate_monomials() {
        let p = parse_binary("I[1]=v(1)*v(2)+v(2)*v(1)+v(3);", 3);
        assert_eq!(p[0].1, [vec![2]].into());
    }
    #[test]
    fn raw_linear_equations_fail_closed() {
        let p = binary(&json!({"n_vars":2,"input_equations":1}), "I[1]=v(1);\n");
        assert_eq!(p["status"], "unsupported_lower_input_degree");
    }
    #[test]
    fn prime_minors_toy_all_arities() {
        for m in 2..=9 {
            let ell = 2;
            let nbits = m * ell;
            let nv = nbits + m - 2;
            let input = json!({"m":m,"ell":ell,"n_vars":nv,"field_modulus":"65537","curve_a":"2","curve_b":"3","target_x":"7"});
            let mut text = format!("ring r=(integer,65537),(v(1..{nv})),dp;\n");
            for i in 0..m {
                text += &format!("poly x{i}=1*v({})+2*v({});\n", 2 * i + 1, 2 * i + 2);
            }
            for k in 0..m - 1 {
                let (x, y, z) = if m == 2 {
                    ("x0".into(), "x1".into(), "7".into())
                } else if k == 0 {
                    ("x0".into(), "x1".into(), format!("v({})", nbits + 1))
                } else if k == m - 2 {
                    (
                        format!("v({})", nbits + k),
                        format!("x{}", k + 1),
                        "7".into(),
                    )
                } else {
                    (
                        format!("v({})", nbits + k),
                        format!("x{}", k + 1),
                        format!("v({})", nbits + k + 1),
                    )
                };
                text += &format!("I[{}]={};\n", k + 1, s3text(&x, &y, &z, "2", "3"));
            }
            for i in 0..nbits {
                text += &format!("I[{}]=v({})^2-v({});\n", m + i, i + 1, i + 1);
            }
            let c = prime(&input, &text);
            assert_eq!(c["minor_determinant"], if m == 2 { "16" } else { "64" });
            assert_eq!(c["first_fall_lower_bound"], 5);
        }
    }
    #[test]
    fn prime_spike40_encoding_and_tamper() {
        for m in 2..=9 {
            let ell = 27;
            let nbits = m * ell;
            let nv = nbits + m - 2;
            let mut input = json!({"m":m,"ell":ell,"n_vars":nv,"field_modulus":"65537","curve_a":"2","curve_b":"3","target_x":"7"});
            input["coordinate_bit_positions"] = json!((0..26).chain([40]).collect::<Vec<_>>());
            let mut text = format!("ring r=(integer,65537),(v(1..{nv})),dp;\n");
            for i in 0..m {
                let expr = (0..ell).map(|j|format!("{}*v({})",1u128 << if j==26 {40} else {j},i*ell+j+1)).collect::<Vec<_>>().join("+");
                text += &format!("poly x{i}={expr};\n");
            }
            for k in 0..m - 1 {
                let (x, y, z) = if m == 2 {
                    ("x0".into(), "x1".into(), "7".into())
                } else if k == 0 {
                    ("x0".into(), "x1".into(), format!("v({})", nbits + 1))
                } else if k == m - 2 {
                    (
                        format!("v({})", nbits + k),
                        format!("x{}", k + 1),
                        "7".into(),
                    )
                } else {
                    (
                        format!("v({})", nbits + k),
                        format!("x{}", k + 1),
                        format!("v({})", nbits + k + 1),
                    )
                };
                text += &format!("I[{}]={};\n", k + 1, s3text(&x, &y, &z, "2", "3"));
            }
            for i in 0..nbits {
                text += &format!("I[{}]=v({})^2-v({});\n", m + i, i + 1, i + 1);
            }
            let c = prime(&input, &text);
            assert_eq!(c["minor_determinant"], if m == 2 { "16" } else { "64" });
            assert_eq!(c["first_fall_lower_bound"], 5);
            let bad = text.replace("1099511627776*", "67108864*");
            assert!(std::panic::catch_unwind(|| prime(&input,&bad)).is_err());
        }
    }
    #[test]
    fn altered_witness_rejected() {
        let polys = parse_binary("I[1]=v(1)*v(2)+v(1);\nI[2]=v(1)*v(2)+v(2);", 2);
        let mut c = control(polys.iter().map(|(_, p)| p.clone()).collect(), Some(2));
        c["witness"]["result_monomials_zero_based"] = json!([[0]]);
        assert!(std::panic::catch_unwind(|| replay_binary(&polys, &c)).is_err());
    }
}
