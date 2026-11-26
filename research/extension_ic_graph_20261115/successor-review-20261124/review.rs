//! Public model applicability and sanitized saved metadata; no DLP or solve.
use crypto_lib::cryptanalysis::jv_cover::{self as cover, Fld, E2, E6};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const CURVE_SHA: &str = "b3ae4f93974456bdd6a772caf7daa0208d64c17f7341f70ee4187d327fd6462e";
const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn scalar(f: &cover::Fq3, v: u64) -> E6 {
    E6([E2([v % f.f.p, 0]), E2::ZERO, E2::ZERO])
}
fn wire(e: E6) -> [[u64; 2]; 3] {
    e.0.map(|v| v.0)
}
fn invariants(f: &cover::Fq3, a2: E6, a4: E6) -> Option<(E6, E6, E6, E6)> {
    let a2sq = f.sq(&a2);
    let c4 = f.mul(&scalar(f, 16), &f.sub(&a2sq, &f.mul(&scalar(f, 3), &a4)));
    let c6 = f.add(
        &f.neg(&f.mul(&scalar(f, 64), &f.mul(&a2sq, &a2))),
        &f.mul(&scalar(f, 288), &f.mul(&a2, &a4)),
    );
    let delta = f.mul(
        &scalar(f, 16),
        &f.mul(&f.sq(&a4), &f.sub(&a2sq, &f.mul(&scalar(f, 4), &a4))),
    );
    if delta == E6::ZERO {
        return None;
    }
    let c4cube = f.mul(&f.sq(&c4), &c4);
    assert_eq!(f.sub(&c4cube, &f.sq(&c6)), f.mul(&scalar(f, 1728), &delta));
    Some((c4, c6, delta, f.mul(&c4cube, &f.inv(&delta))))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Point {
    x: [[u64; 2]; 3],
    y: [[u64; 2]; 3],
    inf: bool,
}
impl Point {
    fn native(&self) -> cover::PtE6 {
        cover::PtE6 {
            x: E6(self.x.map(E2)),
            y: E6(self.y.map(E2)),
            inf: self.inf,
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Public {
    p: u64,
    l: u64,
    alpha: [[u64; 2]; 3],
    g: Point,
    q: Point,
}
fn public_model(bytes: &[u8]) -> Result<Value> {
    ensure(sha(bytes) == CURVE_SHA, "pinned public input")?;
    let c: Public = serde_json::from_slice(bytes)?;
    ensure(
        c.p == 1009 && c.l == 263807419614139553,
        "fixed public domain",
    )?;
    let f = cover::Fq3::new(c.p);
    let ec = cover::EllE::new(&f, &E6(c.alpha.map(E2)));
    ensure(
        ec.on_curve(&c.g.native()) && ec.on_curve(&c.q.native()),
        "original public points",
    )?;
    let (c4, c6, delta, j) = invariants(&f, ec.a2, ec.a4).ok_or("singular public model")?;
    let periods:Vec<_>=[1u32,2,3,6].into_iter().map(|k| {
        let exponent=(c.p as u128).pow(k);
        json!({"k":k,"j_fixed":f.pow(&j,exponent)==j,"a2_fixed":f.pow(&ec.a2,exponent)==ec.a2,"a4_fixed":f.pow(&ec.a4,exponent)==ec.a4})
    }).collect();
    Ok(
        json!({"p":c.p,"l":c.l,"curve_sha256":CURVE_SHA,"model":"y^2=x^3+a2*x^2+a4*x","a2":wire(ec.a2),"a4":wire(ec.a4),"c4":wire(c4),"c6":wire(c6),"discriminant":wire(delta),"j":wire(j),"j_zero":j==E6::ZERO,"j_1728":j==scalar(&f,1728),"frobenius_tests":periods,"endomorphism_search_complete":false}),
    )
}
fn safe_checkpoint(c: &Value) -> Value {
    // Explicit numeric allowlist: never copy a nested result or kernel.
    json!({"b_count":c["b_count"].as_u64(),"rows":c["rows"].as_u64(),"rank_lower_bound":c["rank"].as_u64(),"projections":c["projections"].as_u64(),"sources":c["sources"].as_u64(),"lines":c["lines"].as_u64()})
}
fn collection(bytes: &[u8]) -> Result<Value> {
    ensure(sha(bytes) == RAW_SHA, "pinned raw input")?;
    let raw: Value = serde_json::from_slice(bytes)?;
    let work = &raw["work"];
    let c = &work["collection"];
    let keys: Vec<_> = c.as_object().ok_or("collection object")?.keys().collect();
    let checkpoints: Vec<_> = c["checkpoints"]
        .as_array()
        .map_or_else(Vec::new, |v| v.iter().map(safe_checkpoint).collect());
    let rank_checks: Vec<_> = c["rank_checks"]
        .as_array()
        .map_or_else(Vec::new, |v| v.iter().map(safe_checkpoint).collect());
    Ok(
        json!({"raw_sha256":RAW_SHA,"collection_keys":keys,"checkpoints":checkpoints,"rank_checks":rank_checks,"kernel_columns":work["initial_basis"]["kernel"].as_array().map(Vec::len),"affine_rows":work["affine_rows"].as_array().map(Vec::len),"projected_rows":work["projected_rows"].as_array().map(Vec::len),"collection_ready":work["collection_rank_ready"].as_bool(),"unadmitted_rank_zero_is_not_exact_nullity":true}),
    )
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(
        a.len() == 6,
        "PUBLIC_CURVE RAW DERIVED H2_RAW_SUMMARY NEW_OUTPUT",
    )?;
    ensure(!Path::new(&a[5]).exists(), "exclusive metadata output")?;
    let curve = public_model(&fs::read(&a[1])?)?;
    let coll = collection(&fs::read(&a[2])?)?;
    let derived_bytes = fs::read(&a[3])?;
    let derived: Value = serde_json::from_slice(&derived_bytes)?;
    ensure(
        derived["schema"] == "nagao.template-cost.analysis/v1" && derived["all_intervals"] == 18,
        "bound derived schema",
    )?;
    let traced = derived["rows"]
        .as_array()
        .ok_or("rows")?
        .iter()
        .find(|r| r["arm"] == "traced")
        .ok_or("traced row")?;
    let total = traced["partial_nagao_field_products_min"]
        .as_u64()
        .ok_or("partial total")?;
    let summary_bytes = fs::read(&a[4])?;
    let summary: Value = serde_json::from_slice(&summary_bytes)?;
    ensure(
        summary["public_sha256"] == derived["public_sha256"]
            && summary["attempted_intervals"] == 18,
        "raw/derived public binding",
    )?;
    let totals: Vec<u64> = summary["samples"]
        .as_array()
        .ok_or("samples")?
        .iter()
        .filter(|s| s["phase"] == "panel" && s["arm"] == "traced")
        .map(|s| {
            s["cases"]
                .as_array()
                .ok_or("case array")?
                .iter()
                .try_fold(0u64, |n, c| {
                    n.checked_add(c["cost"]["lin_muls"].as_u64().ok_or("root count")?)
                        .ok_or("root count overflow")
                })
        })
        .collect::<std::result::Result<_, _>>()?;
    ensure(
        totals.len() == 5 && totals.iter().all(|&n| n == totals[0]),
        "five matching root counts",
    )?;
    let roots = totals[0];
    let floor = total.checked_sub(roots).ok_or("ceiling subtraction")?;
    ensure(floor > 0, "positive unchanged work")?;
    let result = json!({"schema":"graph-ic.successor-review/v1","curve":curve,"collection":coll,"derived_sha256":sha(&derived_bytes),"h2_raw_summary_sha256":sha(&summary_bytes),"conditional_root_only_ceiling":{"partial_pair_products":total,"root_products_assumed_eliminated":roots,"unchanged_partial_products":floor,"maximum_factor_if_all_root_work_free":total as f64/floor as f64,"is_attack_prediction":false},"signed_verification":false,"new_dlp_or_f4_calls":0});
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[5])?;
    serde_json::to_writer_pretty(&mut out, &result)?;
    writeln!(out)?;
    out.sync_all()?;
    Ok(())
}
#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn known_special_j_and_singular_refusal() {
        let f = cover::Fq3::new(101);
        assert_eq!(
            invariants(&f, scalar(&f, 0), scalar(&f, 1)).unwrap().3,
            scalar(&f, 1728)
        );
        assert_eq!(
            invariants(&f, scalar(&f, 3), scalar(&f, 3)).unwrap().3,
            E6::ZERO
        );
        assert!(invariants(&f, scalar(&f, 2), scalar(&f, 1)).is_none());
        assert!(invariants(&f, scalar(&f, 0), scalar(&f, 0)).is_none());
    }
    #[test]
    fn fixed_space_is_not_the_rational_point_set() {
        // A=F3[t]/(t^2+1)=F9. Cubing fixes three elements (dimension1),
        // although t^2+1 has no F3 zeros. x=1 in F9 also demonstrates
        // that x^3-x alone need not force another coordinate t into F3.
        let mul =
            |(a, b): (u64, u64), (c, d): (u64, u64)| ((a * c + 2 * b * d) % 3, (a * d + b * c) % 3);
        let mut fixed = 0;
        for a in 0..3 {
            for b in 0..3 {
                let z = (a, b);
                if mul(mul(z, z), z) == z {
                    fixed += 1;
                }
            }
        }
        assert_eq!(fixed, 3);
        assert_eq!((0u64..3).filter(|x| (x * x + 1) % 3 == 0).count(), 0);
        assert_eq!(mul(mul((1, 0), (1, 0)), (1, 0)), (1, 0));
        assert_ne!(mul(mul((0, 1), (0, 1)), (0, 1)), (0, 1));
    }
    #[test]
    fn metadata_allowlist_rejects_embedded_answers() {
        let c = json!({"b_count":1024,"rows":8,"rank":7,"kernel":[17,91],"actual_scalar":12345,"nested":{"private":42}});
        let s = safe_checkpoint(&c);
        assert_eq!(s["b_count"], 1024);
        assert!(
            s.get("actual_scalar").is_none()
                && s.get("kernel").is_none()
                && s.get("nested").is_none()
        );
        assert!(serde_json::from_value::<Point>(
            json!({"x":vec![[0,0];3],"y":vec![[0,0];3],"inf":false,"d":7})
        )
        .is_err());
    }
}
