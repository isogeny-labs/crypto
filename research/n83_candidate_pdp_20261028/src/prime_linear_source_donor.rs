//! Exact-source validator copied from prime_terminal.rs at 3d38b608a576.
#![allow(dead_code, unused_imports)]
#[allow(dead_code)]
#[path = "prime_first_fall_donor.rs"]
mod donor;
#[path = "prime_terminal_math.rs"]
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
pub fn source(input: &Value, text: &str, arm: &str) -> Value {
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
