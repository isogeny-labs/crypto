//! No F4, root finder or DLP call. Answers are excluded from the fixture.
use super::forced_divisor::Family;
use crypto_lib::cryptanalysis::{
    jv_cover::{Div, Fld, Fq, Hyp, E2},
    jv_sieve::two_large_prime::CountedRelationSystem,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn write_new(path: &str, v: &impl Serialize) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer_pretty(&mut f, v)?;
    writeln!(f)?;
    f.sync_all()?;
    Ok(())
}
fn native(a: &[[u64; 2]]) -> Vec<E2> {
    a.iter().copied().map(E2).collect()
}
fn encode(a: &[E2]) -> Vec<[u64; 2]> {
    a.iter().map(|e| e.0).collect()
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Small {
    original_index: usize,
    x: u64,
    y: [u64; 2],
    sign: i8,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Large {
    key: [u64; 6],
    coefficient: i8,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    schema: String,
    raw_sha256: String,
    p: u64,
    source_id: usize,
    h: Vec<[u64; 2]>,
    a: Vec<[u64; 2]>,
    c: Vec<[u64; 2]>,
    norm: Vec<u64>,
    small: Vec<Small>,
    remainders: Vec<Large>,
}
fn prepare(path: &str, out: &str) -> Result<()> {
    ensure(!Path::new(out).exists(), "exclusive fixture")?;
    let bytes = fs::read(path)?;
    ensure(sha(&bytes) == RAW_SHA, "raw source digest")?;
    let raw: Value = serde_json::from_slice(&bytes)?;
    let c = &raw["work"]["collection"];
    let sources = c["sources"].as_array().ok_or("source array")?;
    let (source_id, source) = sources
        .iter()
        .enumerate()
        .find(|(_, s)| {
            s["certificate"]["remainders"]
                .as_array()
                .is_some_and(|r| r.len() == 2)
        })
        .ok_or("first two-quadratic source")?;
    let row = &source["certificate"];
    let terms: Vec<(usize, i8)> = serde_json::from_value(row["terms"].clone())?;
    let base = c["base"].as_array().ok_or("public base")?;
    let small = terms
        .iter()
        .map(|&(i, sign)| {
            let b = base.get(i).ok_or("base index")?;
            Ok(Small {
                original_index: i,
                x: b["x"].as_u64().ok_or("base abscissa")?,
                y: serde_json::from_value(b["y"].clone())?,
                sign,
            })
        })
        .collect::<Result<_>>()?;
    let fixture = Fixture {
        schema: "graph-ic.forced-divisor.fixture/v1".into(),
        raw_sha256: RAW_SHA.into(),
        p: c["p"].as_u64().ok_or("prime")?,
        source_id,
        h: serde_json::from_value(c["h"].clone())?,
        a: serde_json::from_value(row["a"].clone())?,
        c: serde_json::from_value(row["c"].clone())?,
        norm: serde_json::from_value(row["norm"].clone())?,
        small,
        remainders: serde_json::from_value(row["remainders"].clone())?,
    };
    write_new(out, &fixture)
}
fn divisor(f: &Fq, r: &Large) -> Result<Div<E2>> {
    ensure(
        [-1, 1].contains(&r.coefficient) && r.key.iter().all(|&a| a < f.p),
        "canonical signed quadratic",
    )?;
    let u = vec![f.from_fp(r.key[0]), f.from_fp(r.key[1]), E2::ONE];
    let mut v = vec![E2([r.key[2], r.key[3]]), E2([r.key[4], r.key[5]])];
    if r.coefficient == -1 {
        for c in &mut v {
            *c = f.neg(c);
        }
    }
    while v.last() == Some(&E2::ZERO) {
        v.pop();
    }
    Ok(Div { u, v })
}
fn peval(f: &Fq, a: &[E2], x: u64) -> E2 {
    a.iter()
        .rev()
        .fold(E2::ZERO, |v, c| f.add(&f.scale(&v, x), c))
}
fn rank(p: u64, rows: &[Vec<u64>]) -> usize {
    let mut system = CountedRelationSystem::new(p, rows[0].len());
    for row in rows {
        system.insert(row.clone(), 0);
    }
    system.rank()
}
fn minors(p: u64, j: &[Vec<u64>]) -> Vec<usize> {
    (0..8)
        .map(|col| {
            let rows: Vec<_> = j
                .iter()
                .map(|r| {
                    r.iter()
                        .enumerate()
                        .filter_map(|(k, &v)| (k != col).then_some(v))
                        .collect()
                })
                .collect();
            rank(p, &rows)
        })
        .collect()
}
fn geometry(path: &str, out: &str) -> Result<()> {
    ensure(!Path::new(out).exists(), "exclusive geometry output")?;
    let bytes = fs::read(path)?;
    let input: Fixture = serde_json::from_slice(&bytes)?;
    ensure(
        input.schema == "graph-ic.forced-divisor.fixture/v1"
            && input.raw_sha256 == RAW_SHA
            && input.p == 1009,
        "frozen fixture domain",
    )?;
    ensure(
        input.small.len() == 5 && input.remainders.len() == 2,
        "degree-nine support",
    )?;
    ensure(
        input.norm.len() == 10 && input.norm[9] != 0 && input.norm.iter().all(|&x| x < input.p),
        "canonical degree-nine norm",
    )?;
    ensure(
        input.remainders[0].key[..2] != input.remainders[1].key[..2],
        "distinct quadratic support",
    )?;
    let f = Fq::new(input.p);
    let h = native(&input.h);
    let a = native(&input.a);
    let c = native(&input.c);
    let q = divisor(&f, &input.remainders[0])?;
    let family = Family::new(&f, &h, &q.u, &q.v)?;
    let z = family.recover_parameters(&f, &a, &c)?;
    let value = family.instantiate(&f, &z)?;
    let scale = f.inv(&f.sq(&c[1]));
    let expected_norm: Vec<_> = input
        .norm
        .iter()
        .map(|&v| f.mul(&f.from_fp(v), &scale))
        .collect();
    ensure(
        value.norm == expected_norm,
        "original source norm/gauge replay",
    )?;
    let jac = Hyp {
        f: &f,
        h: h.clone(),
        g: 3,
    };
    let mut sum = jac.identity();
    let mut previous = None;
    for b in &input.small {
        ensure(
            b.x < input.p
                && b.y.iter().all(|&v| v < input.p)
                && [-1, 1].contains(&b.sign)
                && previous.is_none_or(|x| x < b.x),
            "distinct ordered canonical small support",
        )?;
        let mut y = E2(b.y);
        if b.sign == -1 {
            y = f.neg(&y);
        }
        ensure(
            y != E2::ZERO && f.sq(&y) == peval(&f, &h, b.x),
            "original small point",
        )?;
        let cy = peval(&f, &c, b.x);
        ensure(
            cy != E2::ZERO && f.add(&peval(&f, &a, b.x), &f.mul(&cy, &y)) == E2::ZERO,
            "original function on small support",
        )?;
        let point = Div {
            u: vec![f.neg(&f.from_fp(b.x)), E2::ONE],
            v: vec![y],
        };
        sum = jac.add(&sum, &point);
        previous = Some(b.x);
    }
    for r in &input.remainders {
        let d = divisor(&f, r)?;
        let check = Family::new(&f, &h, &d.u, &d.v)?;
        check.recover_parameters(&f, &a, &c)?;
        sum = jac.add(&sum, &d);
    }
    ensure(
        jac.is_identity(&sum),
        "original Jacobian principal relation",
    )?;
    let j = family.jacobian(&z)?;
    let r = rank(input.p, &j);
    let minor_ranks = minors(input.p, &j);
    let selected = minor_ranks.iter().position(|&r| r == 7);
    let residuals = family.evaluate(&z)?;
    ensure(
        residuals.iter().all(|&a| a == 0),
        "all original coefficient constraints",
    )?;
    write_new(
        out,
        &json!({"schema":"graph-ic.forced-divisor.geometry/v1","fixture_sha256":sha(&bytes),"source_id":input.source_id,"p":input.p,"omega":f.w,"equations":family.polynomials(),"coordinates":z,"equation_values":residuals,"jacobian":j,"jacobian_rank":r,"column_deletion_ranks":minor_ranks,"selected_development_parameter":selected,"coordinate_names":["c0.re","c0.im","t0.re","t0.im","t1.re","t1.im","t2.re","t2.im"],"normalized_a":encode(&value.a),"normalized_c":encode(&value.c),"norm":encode(&value.norm),"quotient":encode(&value.quotient),"original_function_norm_replay":true,"original_support_and_group_replay":true,"full_native_cost_available":false,"new_f4_or_dlp_calls":0,"signed_verification":false}),
    )
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("prepare") if a.len() == 4 => prepare(&a[2], &a[3]),
        Some("geometry") if a.len() == 4 => geometry(&a[2], &a[3]),
        _ => Err(
            "prepare RAW NEW_FIXTURE | geometry FIXTURE NEW_OUTPUT; no fiber solver registered"
                .into(),
        ),
    }
}
#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn parameter_selection_uses_all_minors_in_fixed_order() {
        let matrix: Vec<Vec<u64>> = (0..7)
            .map(|i| (0..8).map(|j| u64::from(i == j)).collect())
            .collect();
        assert_eq!(rank(101, &matrix), 7);
        assert_eq!(minors(101, &matrix), vec![6, 6, 6, 6, 6, 6, 6, 7]);
        let mut other = matrix;
        for row in &mut other {
            row[7] = 1;
        }
        assert_eq!(minors(101, &other).iter().position(|&r| r == 7), Some(0));
    }
    #[test]
    fn fixture_types_reject_answers_and_malformed_classes() {
        assert!(serde_json::from_value::<Large>(
            json!({"key":[0,0,0,0,0,0],"coefficient":1,"actual_scalar":7})
        )
        .is_err());
        let f = Fq::new(11);
        assert!(divisor(
            &f,
            &Large {
                key: [0; 6],
                coefficient: 2
            }
        )
        .is_err());
        assert!(divisor(
            &f,
            &Large {
                key: [11; 6],
                coefficient: 1
            }
        )
        .is_err());
    }
}
