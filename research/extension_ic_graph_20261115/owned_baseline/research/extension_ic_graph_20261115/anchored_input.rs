//! Project only public curve and recorded homogeneous state into trial input.
use crate::{collection_check, driver, remainder};
use crypto_lib::cryptanalysis::jv_cover::{self as cover, BaseEl, Ctx, Div, Fld, Spec, E2, E6};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs::OpenOptions, io::Write, path::Path};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub schema: String,
    pub origin_worker_sha256: String,
    pub origin_source_revision: String,
    pub public_digest: String,
    pub collection: driver::Record,
    pub kernel: Vec<u64>,
    pub free_column: usize,
    pub scope: String,
}

pub fn no_target_data(value: &Value) -> Result<(), String> {
    match value {
        Value::Object(o) => {
            for (key, value) in o {
                if [
                    "d",
                    "d0",
                    "actual_scalar",
                    "expected_scalar",
                    "candidate_scalar",
                    "scalar",
                    "answer",
                    "descents",
                    "affine_rows",
                    "projected_rows",
                    "solver_answer_placeholder",
                ]
                .contains(&key.as_str())
                {
                    return Err(format!("forbidden target-data key {key}"));
                }
                no_target_data(value)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                no_target_data(v)?;
            }
        }
        _ => (),
    }
    Ok(())
}

pub fn parse(bytes: &[u8]) -> Result<Input, String> {
    let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    no_target_data(&value)?;
    let input: Input = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if serde_json::to_value(&input).map_err(|e| e.to_string())? != value {
        return Err("unknown nested fields or noncanonical input".into());
    }
    if input.schema!="cover.anchored-descent.recorded-input/v1"
        || input.origin_source_revision!="fcda1e22cb0c5424ca3244f783e69be8b99792f4"
        || input.collection.source_revision!=input.origin_source_revision
        || input.collection.seed!=389
        || input.collection.mode!="quadratic-remainders"
        || input.collection.public.p!=1009
        || input.collection.public.l!=263807419614139553
        || input.origin_worker_sha256.len()!=64
        || !input.origin_worker_sha256.bytes().all(|c|c.is_ascii_hexdigit())
        || input.public_digest!=driver::digest(&serde_json::to_vec(&input.collection.public).map_err(|e|e.to_string())?)
        || input.collection.input_sha256!=input.public_digest
        || input.scope!="supplied homogeneous state; excludes collection cost; new residual stream; no new DLP instance or full-pipeline comparison"
    {return Err("recorded-input registration".into());}
    Ok(input)
}

pub fn prepare(worker_path: &Path, output_path: &Path) -> Result<Value, String> {
    let bytes = std::fs::read(worker_path).map_err(|e| e.to_string())?;
    let worker: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if worker["method"] != "extension-remainders"
        || worker["source_revision"] != "fcda1e22cb0c5424ca3244f783e69be8b99792f4"
    {
        return Err("input must be the frozen trial41 remainder worker".into());
    }
    let public: driver::PublicCurve =
        serde_json::from_value(worker["public"].clone()).map_err(|e| e.to_string())?;
    let public_digest = driver::digest(&serde_json::to_vec(&public).map_err(|e| e.to_string())?);
    // An explicit allowlist copies the original homogeneous record. No target
    // data from the outer worker or from its completed descent is retained.
    let c = &worker["work"]["collection"];
    let mut projected = serde_json::Map::new();
    for key in [
        "mode",
        "seed",
        "base",
        "h",
        "sources",
        "projections",
        "checkpoints",
        "counts",
        "rank_target",
        "stop",
        "work_native",
    ] {
        projected.insert(key.into(), c[key].clone());
    }
    projected.insert(
        "schema".into(),
        json!("cover.extension-pipeline.collection-projection/v1"),
    );
    projected.insert("source_revision".into(), worker["source_revision"].clone());
    projected.insert("input_sha256".into(), json!(public_digest));
    projected.insert(
        "public".into(),
        serde_json::to_value(public).map_err(|e| e.to_string())?,
    );
    projected.insert("inner_wall_ns".into(), json!(0));
    let input=Input{
        schema:"cover.anchored-descent.recorded-input/v1".into(),
        origin_worker_sha256:driver::digest(&bytes),
        origin_source_revision:"fcda1e22cb0c5424ca3244f783e69be8b99792f4".into(),
        public_digest,
        collection:serde_json::from_value(Value::Object(projected)).map_err(|e|e.to_string())?,
        kernel:serde_json::from_value(worker["work"]["initial_basis"]["kernel"].clone()).map_err(|e|e.to_string())?,
        free_column:worker["work"]["initial_basis"]["free_column"].as_u64().ok_or("free column")? as usize,
        scope:"supplied homogeneous state; excludes collection cost; new residual stream; no new DLP instance or full-pipeline comparison".into(),
    };
    let bytes = serde_json::to_vec(&input).map_err(|e| e.to_string())?;
    parse(&bytes)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_path)
        .map_err(|e| e.to_string())?;
    out.write_all(&bytes).map_err(|e| e.to_string())?;
    Ok(
        json!({"schema":"cover.anchored-descent.input-projection/v1","input_sha256":driver::digest(&bytes),"input_bytes":bytes.len(),"origin_worker_sha256":input.origin_worker_sha256,"source_count":input.collection.sources.len(),"forbidden_target_keys_absent":true,"scientific_measurement":false,"collection_cost_included":false}),
    )
}

pub fn public_context(input: &Input) -> Result<(Spec, Ctx, Vec<BaseEl>), String> {
    let curve = &input.collection.public;
    collection_check::validate_public(curve)?;
    let spec = cover::spec_from_curve(
        curve.p,
        E6(curve.alpha.map(E2)),
        curve.l,
        curve.g.native(),
        curve.q.native(),
        0,
    )
    .ok_or("public cover transfer")?;
    let ctx = Ctx::new(&spec);
    let f = &ctx.f.f;
    let base = input
        .collection
        .base
        .iter()
        .map(|b| BaseEl {
            x: b.x,
            y: E2(b.y),
            d: Div {
                u: vec![f.neg(&f.from_fp(b.x)), E2::ONE],
                v: vec![E2(b.y)],
            },
        })
        .collect();
    if input.collection.h != remainder::encode(&ctx.cov.hx) {
        return Err("cover model binding".into());
    }
    Ok((spec, ctx, base))
}

pub fn check_homogeneous(input: &Input) -> Result<Value, String> {
    let proof = collection_check::qualify(
        &input.collection,
        input.collection.seed,
        &input.public_digest,
    )?;
    let n = input.collection.base.len();
    let l = input.collection.public.l;
    let mut rank = collection_check::Rank::default();
    for row in &input.collection.projections {
        let mut dense = vec![0; n];
        for &(i, x) in &row.terms {
            dense[i] = x;
        }
        rank.insert(dense, l)?;
    }
    if rank.basis.len() + 1 != n {
        return Err("homogeneous codimension one".into());
    }
    let free = (0..n)
        .find(|i| !rank.basis.contains_key(i))
        .ok_or("kernel free coordinate")?;
    let mut kernel = vec![0; n];
    kernel[free] = 1;
    for (&i, row) in &rank.basis {
        kernel[i] = (l - row[free]) % l;
    }
    if free != input.free_column || kernel != input.kernel {
        return Err("unique normalized kernel".into());
    }
    let anchors = input
        .collection
        .sources
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.certificate.remainder.as_ref().map(|r| (r.key, i)))
        .collect::<BTreeMap<_, _>>();
    Ok(
        json!({"collection_replay":proof,"homogeneous_rank":rank.basis.len(),"free_column":free,"exact_kernel":true,"anchored_classes":anchors.len(),"collection_cost_excluded":true}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn target_keys_are_rejected_recursively() {
        for key in [
            "scalar",
            "actual_scalar",
            "descents",
            "affine_rows",
            "projected_rows",
            "d0",
            "answer",
        ] {
            assert!(no_target_data(&json!({"homogeneous":[{key:0}]})).is_err());
        }
        assert!(
            no_target_data(&json!({"kernel":[1,2],"sources":[],"q":{"x":[1],"y":[2]}})).is_ok()
        );
    }
}
