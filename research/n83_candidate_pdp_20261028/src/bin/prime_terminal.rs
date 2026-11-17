//! Higher-m prime terminal certificates; see PRIME_TERMINAL_DESIGN.md.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
use math::*;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn s3(x: &str, y: &str, z: &str) -> String {
    format!("(({x})-({y}))^2*({z})^2-2*((({x})+({y}))*(({x})*({y})+2)+2*3)*({z})+(({x})*({y})-2)^2-4*3*(({x})+({y}))")
}
fn eval_s3(x: u128, y: u128, z: u128, p: u128) -> u128 {
    let xy = mul(x, y, p);
    let sum = add(x, y, p);
    let diff = add(x, neg(y, p), p);
    let first = mul(mul(diff, diff, p), mul(z, z, p), p);
    let second = mul(mul(2, add(mul(sum, add(xy, 2, p), p), 6 % p, p), p), z, p);
    let d = add(xy, neg(2, p), p);
    add(
        add(first, neg(second, p), p),
        add(mul(d, d, p), neg(mul(12, sum, p), p), p),
        p,
    )
}
fn source(input: &Value, text: &str, arm: &str) -> Value {
    let p = donor::field();
    let m = input["m"].as_u64().unwrap() as usize;
    assert!((3..=9).contains(&m));
    let nb = m * 27;
    let nv = nb + m - 2;
    assert_eq!(input["family"], "prime");
    assert_eq!(input["n"], 83);
    assert_eq!(input["ell"], 27);
    assert_eq!(input["n_vars"], nv);
    assert_eq!(input["input_equations"], nb + m - 1);
    assert_eq!(input["field_modulus"], p.to_string());
    assert_eq!(input["curve_a"], "2");
    assert_eq!(input["curve_b"], "3");
    assert_eq!(input["source_sha256"], sha(text.as_bytes()));
    assert!(["original", "baseline", "spike40"].contains(&arm));
    let pos: Vec<_> = (0..27)
        .map(|j| if arm == "spike40" && j == 26 { 40 } else { j })
        .collect();
    if arm != "original" {
        assert_eq!(input["arm"], arm);
        assert_eq!(input["coordinate_bit_positions"], json!(pos));
    }
    let manifest = if arm == "spike40" {
        "78f21fdb94cd84163c8054258f9a4de914df8f82c24897555cb4c15ffe5b4324"
    } else {
        "199d1b214c78650b2e2e2b539f458eae2688296b9bc51985830951b1cac5aaa5"
    };
    assert_eq!(input["factor_base_manifest_sha256"], manifest);
    let w: Vec<_> = pos.iter().map(|j| 1u128 << j).collect();
    let t = input["target_x"].as_str().unwrap();
    let target = t.parse::<u128>().unwrap();
    assert!(target < p);
    assert_eq!(t, target.to_string());
    let mut expected = format!("ring r=(integer,{p}),(v(1..{nv})),dp;\nideal I;\n");
    for i in 0..m {
        expected += &format!(
            "poly x{i}={};\n",
            (0..27)
                .map(|j| format!("{}*v({})", w[j], i * 27 + j + 1))
                .collect::<Vec<_>>()
                .join("+")
        );
    }
    for k in 0..m - 1 {
        let (x, y, z) = if k == 0 {
            ("x0".into(), "x1".into(), format!("v({})", nb + 1))
        } else if k == m - 2 {
            (format!("v({})", nb + k), format!("x{}", k + 1), t.into())
        } else {
            (
                format!("v({})", nb + k),
                format!("x{}", k + 1),
                format!("v({})", nb + k + 1),
            )
        };
        expected += &format!("I[{}]={};\n", k + 1, s3(&x, &y, &z));
    }
    for i in 0..nb {
        expected += &format!("I[{}]=v({})^2-v({});\n", m + i, i + 1, i + 1);
    }
    let planted = input["target_kind"] == "planted";
    if planted {
        let v = input["witness"].as_array().unwrap();
        assert_eq!(v.len(), nv);
        let vals: Vec<_> = v
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let s = x.as_str().unwrap();
                let a = s.parse::<u128>().unwrap();
                assert!(a < p);
                assert_eq!(s, a.to_string());
                if i < nb {
                    assert!(a <= 1);
                }
                a
            })
            .collect();
        let xs: Vec<_> = (0..m)
            .map(|i| (0..27).fold(0, |a, j| add(a, mul(w[j], vals[i * 27 + j], p), p)))
            .collect();
        for k in 0..m - 1 {
            let (x, y, z) = if k == 0 {
                (xs[0], xs[1], vals[nb])
            } else if k == m - 2 {
                (vals[nb + k - 1], xs[k + 1], target)
            } else {
                (vals[nb + k - 1], xs[k + 1], vals[nb + k])
            };
            assert_eq!(eval_s3(x, y, z, p), 0, "planted S3 relation");
        }
        expected+=&format!("map witness=r,{};\nif(size(witness(I))!=0){{print(\"WITNESS_FAILED\");exit;}} print(\"WITNESS_OK\");\n",v.iter().map(|x|x.as_str().unwrap()).collect::<Vec<_>>().join(","));
    } else {
        assert_eq!(input["target_kind"], "natural");
        assert!(input["witness"].is_null());
    }
    assert_eq!(expected, text, "full source bytes");
    json!({"full_source_template_verified":true,"planted_witness_checked":planted,"source_sha256":sha(text.as_bytes()),"m":m,"selector_count":nb,"last_block_start_zero_based":(m-1)*27,"terminal_intermediate_zero_based":nv-1,"terminal_equation_one_based":m-1,"weights":w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"projection":"all variables outside last selector block -> 0; retained selector squares -> 0","nonterminal_uses_last_block":false})
}
fn zero_mul_x(a: &Poly, n: usize, p: u128) -> Poly {
    let mut out = Poly::new();
    for (&mask, &c) in a {
        for j in 0..n {
            let bit = 1 << j;
            if mask & bit == 0 {
                accum(&mut out, mask | bit, c, p);
            }
        }
    }
    out
}
fn terminal(w: &[u128], t: u128, p: u128) -> (Value, Vec<Poly>, Vec<Vec<u128>>) {
    let n = w.len();
    assert!(n >= 3 && n <= 27 && n % 2 == 1 && p > n as u128 && t < p);
    assert!(w.iter().all(|x| *x > 0 && *x < p));
    let h = h_poly(n, p);
    assert_eq!(h.len(), 1 << ((n - 1) / 2));
    assert!(zero_mul_x(&zero_mul_x(&h, n, p), n, p).is_empty());
    let a1 = multiply_x(&h, w, p);
    let a2 = multiply_x(&a1, w, p);
    let r = (n - 1) / 2;
    assert_eq!(degree(&h), r);
    assert!(degree(&a1) <= r + 1 && degree(&a2) <= r + 1);
    let tt = mul(t, t, p);
    let nt = neg(mul(2, t, p), p);
    let b = vec![
        vec![
            add(4, neg(mul(12, t, p), p), p),
            neg(add(mul(4, t, p), 12 % p, p), p),
            tt,
        ],
        vec![
            neg(add(mul(4, t, p), 12 % p, p), p),
            neg(add(mul(2, tt, p), 4, p), p),
            nt,
        ],
        vec![tt, nt, 1],
    ];
    let aa = vec![h, a1, a2];
    let mut combined = vec![Poly::new(); 3];
    for i in 0..3 {
        for k in 0..3 {
            scaled_add(&mut combined[k], &aa[i], b[i][k], p);
        }
    }
    let actual = combined
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| k + degree(v))
        .max();
    assert!(actual.map_or(true, |d| d <= r + 3));
    (
        json!({"annihilator_degree":r,"annihilator_terms":aa[0].len(),"highest_form_annihilation_verified":true,"nontriviality_method":"last-block quotient preserves H but kills every original top generator; hence terminal coordinate is outside other-generator ideal","first_fall_upper_bound":r+4,"first_fall_degree":null,"affine_derivation_degree_upper_bound":r+4,"affine_consequence_degree":actual,"affine_degree_upper_bound":r+3,"strict_filtered_fall_verified":false,"last_fall_lower_bound":null,"last_fall_upper_bound":null,"last_fall_degree":null}),
        aa,
        b,
    )
}
// Independent controls use ordinary exponent vectors, not the squarefree codec.
type General = BTreeMap<Vec<u8>, u128>;
fn gadd(a: &mut General, b: &General, c: u128, p: u128) {
    for (m, v) in b {
        let x = add(*a.get(m).unwrap_or(&0), mul(*v, c, p), p);
        if x == 0 {
            a.remove(m);
        } else {
            a.insert(m.clone(), x);
        }
    }
}
fn gmul(a: &General, b: &General, w: &[u128], p: u128) -> General {
    let mut out = General::new();
    for (ma, ca) in a {
        for (mb, cb) in b {
            let mut v = mul(*ca, *cb, p);
            let mut m: Vec<_> = ma.iter().zip(mb).map(|(a, b)| a + b).collect();
            for i in 0..w.len() {
                while m[i] > 1 {
                    v = mul(v, w[i], p);
                    m[i] -= 1;
                }
            }
            gadd(&mut out, &[(m, v)].into(), 1, p);
        }
    }
    out
}
fn embed(a: &Poly, n: usize) -> General {
    a.iter()
        .map(|(mask, c)| {
            let mut v: Vec<_> = (0..n).map(|j| ((mask >> j) & 1) as u8).collect();
            v.push(0);
            (v, *c)
        })
        .collect()
}
fn control(n: usize, w: Vec<u128>, t: u128, p: u128) {
    let (_, aa, bb) = terminal(&w, t, p);
    let one: General = [(vec![0; n + 1], 1)].into();
    let mut um = vec![0; n + 1];
    um[n] = 1;
    let u: General = [(um, 1)].into();
    let x = embed(&linear(n), n);
    let mut d = x.clone();
    gadd(&mut d, &u, p - 1, p);
    let mut xy = gmul(&x, &u, &w, p);
    let mut sum = x.clone();
    gadd(&mut sum, &u, 1, p);
    let mut full = gmul(&d, &d, &w, p);
    for c in full.values_mut() {
        *c = mul(*c, mul(t, t, p), p);
    }
    let mut xyplus = xy.clone();
    gadd(&mut xyplus, &one, 2, p);
    let mut second = gmul(&sum, &xyplus, &w, p);
    gadd(&mut second, &one, 6 % p, p);
    gadd(&mut full, &second, neg(mul(2, t, p), p), p);
    gadd(&mut xy, &one, neg(2, p), p);
    gadd(&mut full, &gmul(&xy, &xy, &w, p), 1, p);
    gadd(&mut full, &sum, neg(12 % p, p), p);
    let expected = gmul(&embed(&aa[0], n), &full, &w, p);
    let mut compact = General::new();
    for i in 0..3 {
        for k in 0..3 {
            let mut term = embed(&aa[i], n);
            term = term
                .into_iter()
                .map(|(mut m, c)| {
                    m[n] = k as u8;
                    (m, c)
                })
                .collect();
            gadd(&mut compact, &term, bb[i][k], p);
        }
    }
    assert_eq!(expected, compact);
    assert!(!zero_mul_x(&zero_mul_x(&constant(1), n, p), n, p).is_empty());
}
fn controls() -> Value {
    let mut count = 0;
    for n in [3, 5, 7] {
        for mode in 0..2 {
            let w: Vec<_> = (0..n)
                .map(|i| {
                    if mode == 0 {
                        (i + 1) as u128
                    } else {
                        (1u128 << i) % 101
                    }
                })
                .collect();
            for t in [0, 9, 50] {
                control(n, w.clone(), t, 101);
                count += 1;
            }
        }
    }
    json!({"expanded_polynomial_comparisons":count,"negative_annihilator_controls":count,"all_passed":true,"production_cases_run":0})
}
fn check_cases(cases: &[Value]) {
    assert_eq!(cases.len(), 42);
    let mut seen = BTreeSet::new();
    for c in cases {
        let key = c["key"].as_str().unwrap();
        assert!(seen.insert(key.to_owned()));
    }
    for arm in ["original", "baseline", "spike40"] {
        for m in 3..=9 {
            for target in ["planted", "natural"] {
                assert!(seen.contains(&format!("{arm}-prime-m{m}-{target}")));
            }
        }
    }
}
fn run(root: &Path, out: &Path, replay: bool) {
    let protocol = read(&root.join("protocol-prime-terminal.json"));
    assert_eq!(protocol["freeze"]["status"], "frozen");
    let cases = protocol["extensions"]["cases"].as_array().unwrap();
    check_cases(cases);
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let primality = donor::primality();
    if replay {
        assert_eq!(read(&out.join("primality.json")), primality);
    } else {
        save(&out.join("primality.json"), &primality);
    }
    let p = donor::field();
    let mut rows = vec![];
    for c in cases {
        let key = c["key"].as_str().unwrap();
        let dir = root.join("inputs").join(key);
        let raw = fs::read(dir.join("input.json")).unwrap();
        assert_eq!(sha(&raw), c["input_sha256"]);
        let input: Value = serde_json::from_slice(&raw).unwrap();
        let text = fs::read_to_string(dir.join("system.sing")).unwrap();
        assert_eq!(sha(text.as_bytes()), c["source_sha256"]);
        let arm = c["arm"].as_str().unwrap();
        assert_eq!(
            key,
            format!(
                "{arm}-prime-m{}-{}",
                input["m"],
                input["target_kind"].as_str().unwrap()
            )
        );
        let src = source(&input, &text, arm);
        let w: Vec<_> = src["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().parse().unwrap())
            .collect();
        let t = input["target_x"].as_str().unwrap().parse().unwrap();
        let (algebra, aa, bb) = terminal(&w, t, p);
        let dest = out.join(key);
        if !replay {
            fs::create_dir(&dest).unwrap();
        }
        let mut files = vec![];
        for (i, a) in aa.iter().enumerate() {
            let name = format!("A{i}.poly");
            let data = bytes(a, 27, p);
            assert_eq!(parse(&data).2, *a);
            if replay {
                let stored = fs::read(dest.join(&name)).unwrap();
                assert_eq!(stored, data);
                assert_eq!(parse(&stored).2, *a);
            } else {
                fs::write(dest.join(&name), &data).unwrap();
            }
            files.push(json!({"path":name,"sha256":sha(&data),"bytes":data.len(),"terms":a.len(),"degree":degree(a)}));
        }
        let cert = json!({"schema":"n83.prime-terminal-certificate/v1","key":key,"input":input,"source":src,"algebra":algebra,"factor_files":files,"witness_circuit":{"operation":"sum_i_Ai_times_Bi_U","B_coefficients_low_to_high":bb.iter().map(|b|b.iter().map(|c|c.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>(),"A_variables":"normalized last selector block z0..z26","U_variable_zero_based":src["terminal_intermediate_zero_based"],"U_is_unbounded_indeterminate":true,"relation":"g=NF(H*S3(U,X,t))","strict_nonmembership_checked":false}});
        if replay {
            assert_eq!(read(&dest.join("certificate.json")), cert);
        } else {
            save(&dest.join("certificate.json"), &cert);
        }
        rows.push(json!({"key":key,"arm":arm,"m":input["m"],"target_kind":input["target_kind"],"source_sha256":input["source_sha256"],"factor_base_manifest_sha256":input["factor_base_manifest_sha256"],"first_fall_upper_bound":algebra["first_fall_upper_bound"],"first_fall_degree":null,"affine_derivation_degree_upper_bound":algebra["affine_derivation_degree_upper_bound"],"affine_consequence_degree":algebra["affine_consequence_degree"],"strict_filtered_fall_verified":false,"last_fall_lower_bound":null,"last_fall_upper_bound":null,"last_fall_degree":null,"certificate":format!("{key}/certificate.json"),"verified":true}));
        println!("{} {key}", if replay { "replayed" } else { "certified" });
    }
    let summary = json!({"schema":"n83.prime-terminal-summary/v1","cases":42,"rows":rows,"independent_scientific_review":false,"broader_goal":"active"});
    if replay {
        assert_eq!(read(&out.join("summary.json")), summary);
    } else {
        save(&out.join("summary.json"), &summary);
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("controls") => println!("{}", serde_json::to_string_pretty(&controls()).unwrap()),
        Some("run") => run(Path::new(&a[2]), Path::new(&a[3]), false),
        Some("replay") => run(Path::new(&a[2]), Path::new(&a[3]), true),
        _ => panic!("controls | run ROOT OUT | replay ROOT OUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_expansion() {
        assert_eq!(controls()["all_passed"], true);
    }
    #[test]
    fn small_primes() {
        for p in [11, 17, 101] {
            control(3, vec![1, 2, 3], 4, p);
        }
    }
    #[test]
    fn codec() {
        let a = h_poly(7, 101);
        let b = bytes(&a, 7, 101);
        assert_eq!(parse(&b).2, a);
        assert!(std::panic::catch_unwind(|| parse(&b[..b.len() - 1])).is_err());
        let mut c = b.clone();
        c[40] = 255;
        assert!(std::panic::catch_unwind(|| parse(&c)).is_err());
    }
    #[test]
    fn invalid_weights() {
        assert!(std::panic::catch_unwind(|| terminal(&[1, 0, 3], 4, 101)).is_err());
        assert!(std::panic::catch_unwind(|| terminal(&[1, 2, 3], 4, 3)).is_err());
    }
    #[test]
    fn panel_missing_rejected() {
        assert!(std::panic::catch_unwind(|| check_cases(&[])).is_err());
    }
}

#[cfg(test)]
mod source_tests {
    use super::*;
    fn fixture(m: usize, arm: &str) -> (Value, String) {
        let p = donor::field();
        let nb = 27 * m;
        let nv = nb + m - 2;
        let positions: Vec<_> = (0..27)
            .map(|j| if arm == "spike40" && j == 26 { 40 } else { j })
            .collect();
        let mut t = format!("ring r=(integer,{p}),(v(1..{nv})),dp;\nideal I;\n");
        for i in 0..m {
            t += &format!(
                "poly x{i}={};\n",
                (0..27)
                    .map(|j| format!("{}*v({})", 1u128 << positions[j], i * 27 + j + 1))
                    .collect::<Vec<_>>()
                    .join("+")
            );
        }
        let mut coords: Vec<String> = vec!["x0".into(), "x1".into()];
        for k in 0..m - 1 {
            let z = if k == m - 2 {
                "0".into()
            } else {
                format!("v({})", nb + k + 1)
            };
            t += &format!("I[{}]={};\n", k + 1, s3(&coords[0], &coords[1], &z));
            coords = vec![z, format!("x{}", k + 2)];
        }
        for i in 1..=nb {
            t += &format!("I[{}]=v({i})^2-v({i});\n", m - 1 + i);
        }
        let manifest = if arm == "spike40" {
            "78f21fdb94cd84163c8054258f9a4de914df8f82c24897555cb4c15ffe5b4324"
        } else {
            "199d1b214c78650b2e2e2b539f458eae2688296b9bc51985830951b1cac5aaa5"
        };
        (
            json!({"family":"prime","m":m,"n":83,"ell":27,"n_vars":nv,"input_equations":nb+m-1,"field_modulus":p.to_string(),"curve_a":"2","curve_b":"3","source_sha256":sha(t.as_bytes()),"arm":arm,"coordinate_bit_positions":positions,"factor_base_manifest_sha256":manifest,"target_kind":"natural","target_x":"0","witness":null}),
            t,
        )
    }
    #[test]
    fn full_chain_sources() {
        for arm in ["original", "baseline", "spike40"] {
            for m in 3..=9 {
                let (i, t) = fixture(m, arm);
                assert_eq!(source(&i, &t, arm)["full_source_template_verified"], true);
            }
        }
    }
    #[test]
    fn altered_source_rejected() {
        let (mut i, t) = fixture(4, "baseline");
        let bad = t.replace("I[1]=", "I[1]=1+");
        i["source_sha256"] = json!(sha(bad.as_bytes()));
        assert!(std::panic::catch_unwind(|| source(&i, &bad, "baseline")).is_err());
    }
    #[test]
    fn wrong_base_rejected() {
        let (mut i, t) = fixture(3, "spike40");
        i["factor_base_manifest_sha256"] = json!("wrong");
        assert!(std::panic::catch_unwind(|| source(&i, &t, "spike40")).is_err());
    }
    #[test]
    fn invalid_planted_witness_rejected() {
        let (mut i, t) = fixture(3, "baseline");
        i["target_kind"] = json!("planted");
        i["witness"] = json!(vec!["0"; 82]);
        assert!(std::panic::catch_unwind(|| source(&i, &t, "baseline")).is_err());
        i["witness"][0] = json!("2");
        assert!(std::panic::catch_unwind(|| source(&i, &t, "baseline")).is_err());
    }
    #[test]
    fn projection_negative_control() {
        let n = 3;
        let p = 101;
        let x = embed(&linear(n), n);
        let w = vec![1, 2, 3];
        let mut uvec = vec![0; n + 1];
        uvec[n] = 1;
        let u: General = [(uvec, 1)].into();
        let f = gmul(&gmul(&x, &x, &w, p), &gmul(&u, &u, &w, p), &w, p);
        assert!(!f.is_empty());
        assert!(f.keys().all(|m| m[n] == 2));
        let projected: General = f.into_iter().filter(|(m, _)| m[n] == 0).collect();
        assert!(projected.is_empty());
    }
}
