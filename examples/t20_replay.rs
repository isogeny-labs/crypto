//! Independent replay of an n = 73 Koblitz single-target recovery.
//!
//! Own bitwise GF(2^73) arithmetic (carry-less schoolbook multiply, shift
//! reduction, Fermat inversion), no code shared with the producer or with the
//! repository's field types.  Checks: G and Q on the curve, r*G = O, and
//! [d]G = Q for the recovered scalar d of each run row; compares d with the
//! validation-only sidecar answer.
//!
//! Usage: t20_replay <fixture.json> <ic_row.jsonl>... ; exits 1 on any failure
//! and prints one JSON receipt line per row.

use serde_json::{json, Value};
use std::fs;

const N: u32 = 73;
// x^73 + x^4 + x^3 + x^2 + 1
const MOD_LOW: u128 = (1 << 4) | (1 << 3) | (1 << 2) | 1;

fn reduce(hi: u128, lo: u128) -> u128 {
    // (hi:lo) is a 146-bit product; hi holds bits 128.., lo bits 0..128.
    // Fold bits >= 73 down using x^73 = MOD_LOW.
    let mut v = lo;
    // bits 73..127 of lo
    let mut top = v >> N;
    v &= (1u128 << N) - 1;
    // contribution of hi: bits 128.. correspond to x^128.. = x^(128-73) * x^73
    let mut fold = (hi << (128 - N)) | top; // value (bits >= 73) / x^73 as polynomial
                                            // fold is a polynomial of degree < 73 + ... ; multiply by MOD_LOW and reduce again
    while fold != 0 {
        let mut acc = 0u128;
        for b in 0..5u32 {
            if (MOD_LOW >> b) & 1 == 1 {
                acc ^= fold << b;
            }
        }
        v ^= acc & ((1u128 << N) - 1);
        top = acc >> N;
        fold = top;
    }
    v
}

fn mul(a: u128, b: u128) -> u128 {
    let (mut lo, mut hi) = (0u128, 0u128);
    for i in 0..N {
        if (b >> i) & 1 == 1 {
            lo ^= a << i;
            if i > 0 {
                hi ^= a >> (128 - i);
            }
        }
    }
    reduce(hi, lo)
}

fn inv(a: u128) -> u128 {
    // a^(2^73 - 2)
    let mut result = 1u128;
    let mut base = a;
    for _ in 0..(N - 1) {
        base = mul(base, base);
        result = mul(result, base);
    }
    result
}

type Pt = Option<(u128, u128)>;

fn on_curve(p: Pt) -> bool {
    match p {
        None => true,
        Some((x, y)) => {
            // y^2 + xy = x^3 + b, a = 0, b = 1
            let lhs = mul(y, y) ^ mul(x, y);
            let rhs = mul(mul(x, x), x) ^ 1;
            lhs == rhs
        }
    }
}

fn add(p: Pt, q: Pt) -> Pt {
    match (p, q) {
        (None, r) | (r, None) => r,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 {
                if y1 ^ y2 == x1 {
                    return None; // q = -p
                }
                // doubling
                let l = x1 ^ mul(y1, inv(x1));
                let x3 = mul(l, l) ^ l; // a = 0
                let y3 = mul(x1, x1) ^ mul(l ^ 1, x3);
                Some((x3, y3))
            } else {
                let l = mul(y1 ^ y2, inv(x1 ^ x2));
                let x3 = mul(l, l) ^ l ^ x1 ^ x2;
                let y3 = mul(l, x1 ^ x3) ^ x3 ^ y1;
                Some((x3, y3))
            }
        }
    }
}

fn smul(k: u128, p: Pt) -> Pt {
    let mut r: Pt = None;
    for i in (0..128).rev() {
        r = add(r, r);
        if (k >> i) & 1 == 1 {
            r = add(r, p);
        }
    }
    r
}

fn pt(v: &Value) -> Pt {
    let a = v.as_array().expect("point array");
    Some((
        a[0].as_str().unwrap().parse().unwrap(),
        a[1].as_str().unwrap().parse().unwrap(),
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fixture: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let g = pt(&fixture["generator"]);
    let q = pt(&fixture["public_target"]);
    let r: u128 = fixture["subgroup_order"].as_str().unwrap().parse().unwrap();
    let sidecar: u128 = fixture["fixture_scalar_validation_only"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    // known-answer self-test of the replay arithmetic
    let base_ok = on_curve(g) && on_curve(q) && smul(r, g).is_none();
    let sidecar_ok = smul(sidecar, g) == q;
    let mut failed = !(base_ok && sidecar_ok);
    println!(
        "{}",
        json!({"kind": "t20_replay_base", "g_q_on_curve_and_rG_is_O": base_ok, "sidecar_d_gives_Q": sidecar_ok})
    );
    for path in &args[2..] {
        let text = fs::read_to_string(path).unwrap();
        let row: Value =
            serde_json::from_str(text.lines().find(|l| l.starts_with('{')).unwrap()).unwrap();
        let d: u128 = row["recovered_scalar"]
            .as_u64()
            .map(|v| v as u128)
            .unwrap_or(0);
        let ok = d != 0 && smul(d, g) == q;
        let matches = d == sidecar;
        failed |= !(ok && matches);
        println!(
            "{}",
            json!({"kind": "t20_replay_row", "file": path, "recovered_scalar": d.to_string(),
            "d_times_G_equals_Q": ok, "equals_sidecar_answer": matches})
        );
    }
    if failed {
        std::process::exit(1);
    }
}
