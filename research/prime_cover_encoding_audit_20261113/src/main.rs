//! Exact arithmetic audit of two frozen saved rows. No solver or timing run.
#[allow(dead_code)]
#[path = "../../../src/hash/sha256.rs"]
mod hash_sha256;
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs};
const PKM_SHA: &str = "88f5ad4b1d972c3d78f2d64cad5934788ff93e7f203c44528c833a185ac7ba7e";
const CUBIC_SHA: &str = "f9eeef35b50b7dd5e0f282f1c700e1d3f762f21e8cc0c31f074ae761e31bf895";
fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    r
}
fn prime32(p: u64) -> bool {
    if !(3..=u32::MAX as u64).contains(&p) || p % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= p {
        if p % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
fn number(v: &Value, label: &str) -> Result<u64, String> {
    v.as_u64()
        .ok_or_else(|| format!("missing or noninteger {label}"))
}
fn field(v: &Value, label: &str, p: u64) -> Result<u64, String> {
    let x = number(v, label)?;
    if x >= p {
        return Err(format!("noncanonical field value {label}"));
    }
    Ok(x)
}
fn rhs(a: u64, b: u64, x: u64, p: u64) -> u64 {
    (mul(mul(x, x, p), x, p) + mul(a, x, p) + b) % p
}
fn character(x: u64, p: u64) -> Result<i8, String> {
    if x == 0 {
        return Ok(0);
    }
    match pow(x, (p - 1) / 2, p) {
        1 => Ok(1),
        y if y == p - 1 => Ok(-1),
        _ => Err("Euler value outside {0,1,-1}".into()),
    }
}
fn coset(g: u64, zeta: u64, p: u64) -> Result<Vec<u64>, String> {
    if g == 0 || g >= p || zeta >= p || pow(zeta, 16, p) != 1 || pow(zeta, 8, p) == 1 {
        return Err("not a nonzero coset with a primitive16th root".into());
    }
    let points: Vec<_> = (0..16).map(|i| mul(g, pow(zeta, i, p), p)).collect();
    if points.iter().copied().collect::<BTreeSet<_>>().len() != 16 {
        return Err("coset repeats".into());
    }
    Ok(points)
}
fn digest(bytes: &[u8]) -> String {
    hash_sha256::sha256(bytes)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
fn pinned(bytes: &[u8], want: &str) -> Result<(), String> {
    if digest(bytes) != want {
        return Err("saved-input SHA-256 mismatch".into());
    }
    Ok(())
}
fn audit(path: &str, cubic: bool) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let want = if cubic { CUBIC_SHA } else { PKM_SHA };
    pinned(&bytes, want)?;
    let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
    let records: Vec<Value> = text
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    if records.len() != 2 || records[1]["summary"]["rows"] != 1 {
        return Err("expected one row plus summary".into());
    }
    let row = &records[0];
    let p = number(&row["p"], "p")?;
    if p != 2013265921 || !prime32(p) || row["t"] != 4 || row["m"] != 4 || row["kind"] != "kummer" {
        return Err("outside the frozen prime-field m4t4 audit scope".into());
    }
    let a = field(&row["curve"]["a"], "a", p)?;
    let b = field(&row["curve"]["b"], "b", p)?;
    if (mul(4, mul(mul(a, a, p), a, p), p) + mul(27, mul(b, b, p), p)) % p == 0 {
        return Err("singular curve".into());
    }
    let g = field(&row["tower"]["g"], "g", p)?;
    let zeta = field(&row["tower"]["zeta"], "zeta", p)?;
    let nodes = coset(g, zeta, p)?;
    let coefficients = if cubic {
        if row["cover"]["map"] != "cubic" || row["cover"]["den"] != json!([1]) {
            return Err("unsupported cover".into());
        }
        let list = row["cover"]["num"].as_array().ok_or("missing numerator")?;
        if list.len() != 4 || list[3] != 1 {
            return Err("not the recorded monic cubic".into());
        }
        list.iter()
            .map(|v| field(v, "cover numerator", p))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        if !row["cover"].is_null() {
            return Err("reference unexpectedly has a cover".into());
        }
        vec![0, 1]
    };
    let mut images = BTreeSet::new();
    let mut on = BTreeSet::new();
    let mut zeros = BTreeSet::new();
    let mut point_records = Vec::new();
    for u in nodes {
        let x = coefficients
            .iter()
            .rev()
            .fold(0, |v, &c| (mul(v, u, p) + c) % p);
        let value = rhs(a, b, x, p);
        let chi = character(value, p)?;
        images.insert(x);
        if chi == 1 {
            on.insert(x);
        }
        if chi == 0 {
            zeros.insert(x);
        }
        point_records.push(json!({"u":u,"x":x,"curve_rhs":value,"character":chi}));
    }
    if cubic
        && (number(&row["cover"]["base_size"], "base_size")? != images.len() as u64
            || number(&row["cover"]["base_on_curve"], "base_on_curve")? != on.len() as u64)
    {
        return Err("recorded cover cardinality mismatch".into());
    }
    let xr = field(&row["x_r"], "x_r", p)?;
    let target_chi = character(rhs(a, b, xr, p), p)?;
    if target_chi < 0 {
        return Err("target abscissa has no rational y".into());
    }
    Ok(
        json!({"role":if cubic {"cubic"} else {"reference"},"input_sha256":want,
        "p":p,"curve":{"a":a,"b":b},"g":g,"zeta":zeta,"cover_numerator":coefficients,
        "exact_image_size":images.len(),"exact_nonzero_square_base_size":on.len(),
        "excluded_zero_y_abscissas":zeros.len(),"target_x":xr,"target_character":target_chi,
        "points":point_records,"recorded_planted_ok":row["planted_ok"],
        "recorded_solver":{"max_cols":row["max_cols"],"input_degrees":row["input_degrees"],"timed_out":row["timed_out"]}}),
    )
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: cover-record-audit REFERENCE_JSONL CUBIC_JSONL OUTPUT_JSON".into());
    }
    let report = json!({"schema":"cover.saved-input-audit/v1","source_snapshot":"cc986a93b",
        "standing":"unsigned native correctness evidence",
        "scope":"two frozen rows only; no F4 replay, scalar recovery, subgroup proof, relation-yield or timing inference",
        "cases":[audit(&args[1],false)?,audit(&args[2],true)?]});
    fs::write(
        &args[3],
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    for row in report["cases"].as_array().unwrap() {
        println!(
            "{}: image={}, nonzero-square={}, excluded-zero-y={}, target-character={}",
            row["role"],
            row["exact_image_size"],
            row["exact_nonzero_square_base_size"],
            row["excluded_zero_y_abscissas"],
            row["target_character"]
        );
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn residues_match_known_curve17_points() {
        assert_eq!(character(rhs(2, 2, 0, 17), 17).unwrap(), 1);
        assert_eq!(character(rhs(2, 2, 5, 17), 17).unwrap(), 1);
        assert_eq!(character(rhs(2, 2, 1, 17), 17).unwrap(), -1);
        assert_eq!(character(0, 17).unwrap(), 0);
    }
    #[test]
    fn rejects_repeated_root_and_nonprime_scope() {
        assert!(coset(2, 1, 17).is_err());
        assert!(!prime32(15));
        assert!(!prime32(1u64 << 40));
        assert!(prime32(17));
    }
    #[test]
    fn modified_input_cannot_pass_a_digest_pin() {
        let expected = digest(b"original");
        assert!(pinned(b"original", &expected).is_ok());
        assert!(pinned(b"modified", &expected).is_err());
    }
}
