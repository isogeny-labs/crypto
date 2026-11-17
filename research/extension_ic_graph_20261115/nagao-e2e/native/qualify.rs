//! Retain the byte-exact stage41 independent collection/pipeline checker.
#![allow(dead_code)]
#[path = "qualifier/extension_pipeline_check.rs"]
pub mod check;
#[path = "qualifier/extension_pipeline_collection_check.rs"]
pub mod collection_check;
#[path = "qualifier/extension_remainder_driver.rs"]
pub mod driver;
#[path = "qualifier/extension_remainder.rs"]
pub mod remainder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
pub fn endpoint(w: &Value, public: &Value, answer: u64) -> Result<(), String> {
    if w["public"] != *public
        || w["actual_scalar"] != answer
        || w["verified"] != true
        || w["capped"] != false
        || w["solver_answer_placeholder"] != 0
    {
        return Err("public scalar endpoint or worker status".into());
    }
    let p: driver::PublicCurve =
        serde_json::from_value(public.clone()).map_err(|e| e.to_string())?;
    collection_check::validate_public(&p)?;
    use crypto_lib::cryptanalysis::jv_cover::{EllE, Fq3, E2, E6};
    let f = Fq3::new(p.p);
    let ec = EllE::new(&f, &E6(p.alpha.map(E2)));
    if ec.mul(&p.g.native(), answer) != p.q.native() {
        return Err("independent public elliptic replay".into());
    }
    Ok(())
}
pub fn qualify(w: &Value, public: &Value, answer: u64, input_sha: &str) -> Result<Value, String> {
    endpoint(w, public, answer)?;
    if w["method"] == "rho" {
        check::rho_identity(
            w,
            false,
            16,
            public["p"].as_u64().ok_or("p")?,
            public["l"].as_u64().ok_or("l")?,
            401,
        )?;
        if w["work_native"]["foreign_hook_calls"] != 0 {
            return Err("foreign rho hooks".into());
        }
        return Ok(json!({"endpoint":true,"registered_rho":true}));
    }
    if w["work_native"]["foreign_hook_calls"] != 0 {
        return Err("foreign IC hooks".into());
    }
    let c = check::collection(w, input_sha)?;
    let collection = collection_check::qualify(&c, 401, input_sha)?;
    let pipeline = check::pipeline(w, &c, &collection, answer, 8192, 32)?;
    if w["work"]["descent_timeouts"] != 0
        || w["work"]["descent_incomplete"] != 0
        || w["work"]["invalid_descents"] != 0
    {
        return Err("incomplete or invalid descent".into());
    }
    Ok(json!({"endpoint":true,"collection":collection,"pipeline":pipeline}))
}
pub fn signature(w: &Value) -> Value {
    if w["method"] == "rho" {
        return json!({"public":w["public"],"scalar":w["actual_scalar"],"signature":w["signature"],"rho_options":w["rho_options"],"primitives":w["work_native"]["observed_primitives"]});
    }
    let r = &w["work"];
    json!({"public":w["public"],"scalar":w["actual_scalar"],"collection":r["collection"],"initial_basis":r["initial_basis"],"descents":r["descents"],"affine_rows":r["affine_rows"],"projected_rows":r["projected_rows"],"residuals":r["descent_residuals"],"advances":r["descent_stream_advances"],"final_extra_counts":r["final_extra_counts"],"rank":r["last_rank"],"primitives":w["work_native"]["observed_primitives"]})
}
pub fn pair(a: &Value, b: &Value) -> Result<(), String> {
    // The old paired checker only permitted collection changes. This separate
    // contract permits descent arithmetic changes, preserving all geometry.
    for field in [
        "collection",
        "initial_basis",
        "descents",
        "affine_rows",
        "projected_rows",
        "descent_residuals",
        "descent_stream_advances",
        "descent_timeouts",
        "descent_incomplete",
        "invalid_descents",
        "final_extra_counts",
        "last_rank",
        "actual_scalar",
    ] {
        if a["work"][field] != b["work"][field] {
            return Err(format!("IC pair differs at {field}"));
        }
    }
    for x in a["work_native"]["phases"].as_array().ok_or("phases")? {
        let name = x["phase"].as_str().ok_or("phase")?;
        if name == "descent" {
            continue;
        }
        let y = b["work_native"]["phases"]
            .as_array()
            .ok_or("phases")?
            .iter()
            .find(|y| y["phase"] == name)
            .ok_or("missing phase")?;
        if x["primitives"] != y["primitives"] {
            return Err(format!("non-descent primitives differ: {name}"));
        }
    }
    Ok(())
}
