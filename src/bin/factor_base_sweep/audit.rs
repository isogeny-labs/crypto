//! Independent inventory reconstruction and point-witness replay. Does not rerun F4.
use super::{
    engine,
    model::{read, Request, Result},
    polynomials, shapes,
};
use crypto_lib::cryptanalysis::{
    ecbench::{
        canonical::{derive_u64, sha256_hex},
        methods::{dump_factor_base, FactorBaseDump},
        workload::Instance,
    },
    ic_boundary::{BinaryGroup, CountedGroup, GroupOps, IncrementalGauss, PrimePoint},
    koblitz_fast::FastPoint,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn number(s: &str) -> Result<u64> {
    u64::from_str_radix(s.strip_prefix("0x").unwrap_or(s), 16).map_err(|e| e.to_string())
}
pub fn replay(dir: &Path, req: &Request) -> Result<usize> {
    let deriv: Value = read(&dir.join("derivation.json"))?;
    let spec = deriv["constructor"].as_str().ok_or("missing constructor")?;
    let inst = req.candidate.curve.build()?;
    if let Some(original) = &req.candidate.factor_base {
        if original != spec {
            return Err("constructor does not match candidate request".into());
        }
    }
    if let Some(shape) = &req.candidate.shape {
        let Instance::Binary(i) = &inst else {
            return Err("binary shape bound to a different field".into());
        };
        let b = shapes::basis(i, shape, req.candidate.dimension, req.candidate.seed)?;
        if deriv["basis_words"] != json!(b) {
            return Err("recorded derivation basis does not reproduce".into());
        }
        let expected = format!(
            "binary-subspace:dimension={},basis={}",
            b.len(),
            b.iter()
                .map(|v| format!("0x{v:x}"))
                .collect::<Vec<_>>()
                .join(";")
        );
        if expected != spec {
            return Err("recorded constructor differs from generated basis".into());
        }
    }
    let dump: FactorBaseDump = read(&dir.join("base.json"))?;
    if dump_factor_base(&req.candidate.curve, spec)? != dump {
        return Err("native factor-base reconstruction differs".into());
    }
    if deriv["measurement_identity"]
        != json!(engine::measurement_identity(
            &dump,
            spec,
            req.plan.max_degree
        )?)
    {
        return Err("measurement identity does not reproduce".into());
    }
    if deriv["order_merkle_root"] != json!(engine::root_order(&dump)?) {
        return Err("order commitment mismatch".into());
    }
    let expected: BTreeMap<_, _> = dump
        .points
        .iter()
        .map(|p| {
            (
                format!("{}:{}", p.x, p.y),
                json!({"column":p.col,"coefficient":p.coef}),
            )
        })
        .collect();
    let lookup: BTreeMap<String, Value> = read(&dir.join("lookup.json"))?;
    if expected != lookup || lookup.len() != dump.points.len() {
        return Err("signed lookup differs from inventory".into());
    }
    let record: Value = read(&dir.join("result.json"))?;
    if record["candidate"] != json!(req.candidate)
        || record["fb_id"] != json!(dump.factor_base.fb_id)
    {
        return Err("result identity differs from request/inventory".into());
    }
    if record["certified_degree_of_regularity"] != Value::Null
        || record["full_pipeline_speedup"] != Value::Null
    {
        return Err("unsupported promoted claim".into());
    }
    match inst {
        Instance::Prime(i) => {
            let pts = dump
                .points
                .iter()
                .map(|p| Ok(PrimePoint::affine(number(&p.x)?, number(&p.y)?)))
                .collect::<Result<Vec<_>>>()?;
            if pts.iter().any(|p| !i.curve.is_on_curve(*p)) {
                return Err("off-curve inventory".into());
            }
            let xs = pts
                .iter()
                .map(|p| p.x)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            trials(
                dir,
                req,
                &dump,
                &record,
                &i.curve,
                i.generator_point(),
                i.r,
                i.cofactor,
                &pts,
                |p| [p.x, p.y],
                |x| polynomials::prime(&i.curve, x, &xs),
            )
        }
        Instance::Binary(i) => {
            let pts = dump
                .points
                .iter()
                .map(|p| Ok(FastPoint::affine(number(&p.x)?, number(&p.y)?)))
                .collect::<Result<Vec<_>>>()?;
            if pts.iter().any(|p| !i.fast.is_on_curve(*p)) {
                return Err("off-curve inventory".into());
            }
            let b = shapes::recipe_basis(
                &dump.factor_base.params,
                dump.factor_base.dimension.ok_or("missing dimension")?,
            )?;
            if deriv["basis_words"] != json!(b) {
                return Err("derivation basis differs from native constructor parameters".into());
            }
            let v = shapes::subspace(&i, &b);
            let group = BinaryGroup(&i.fast);
            trials(
                dir,
                req,
                &dump,
                &record,
                &group,
                i.generator,
                i.r,
                i.cofactor,
                &pts,
                |p| [p.x, p.y],
                |x| polynomials::binary(&i, &v, x),
            )
        }
        _ => Err("unsupported replay family".into()),
    }
}
#[allow(clippy::too_many_arguments)]
fn trials<G: CountedGroup>(
    dir: &Path,
    req: &Request,
    dump: &FactorBaseDump,
    result: &Value,
    group: &G,
    generator: G::Elt,
    r: u64,
    h: u64,
    points: &[G::Elt],
    xy: impl Fn(G::Elt) -> [u64; 2],
    encode: impl Fn(u64) -> Vec<crypto_lib::cryptanalysis::f4_fp::Poly>,
) -> Result<usize> {
    let trials = result["trials"].as_array().ok_or("missing trials")?;
    if trials.len() != 1 + (req.plan.train + req.plan.holdout) as usize {
        return Err("completed candidate omitted requested probes".into());
    }
    let mut unique = BTreeSet::new();
    let mut matrix = IncrementalGauss::new(dump.factor_base.columns as usize, r);
    let mut witnesses = 0;
    let mut counts: BTreeMap<&str, [u64; 4]> = BTreeMap::new();
    let mut best_degree: Option<u64> = None;
    for v in trials {
        let phase = v["phase"].as_str().ok_or("missing phase")?;
        let j = v["trial"].as_u64().ok_or("missing trial")?;
        let cap = match phase {
            "positive_control" => 1,
            "train" => req.plan.train,
            "holdout" => req.plan.holdout,
            _ => return Err("unknown trial phase".into()),
        };
        let status = v["status"].as_str().ok_or("missing trial status")?;
        if !matches!(
            status,
            "verified_decomposition" | "refuted" | "exhausted" | "no_point_decomposition"
        ) {
            return Err("unknown trial verdict".into());
        }
        let count = counts.entry(phase).or_default();
        count[0] += 1;
        count[1] += u64::from(status == "verified_decomposition");
        count[2] += u64::from(status == "exhausted");
        count[3] += u64::from(status == "refuted");
        if phase == "train" && status == "verified_decomposition" {
            if let Some(d) = v["observed_solving_degree"].as_u64() {
                best_degree = Some(best_degree.map_or(d, |old| old.min(d)));
            }
        }
        if j >= u64::from(cap) || !unique.insert((phase, j)) {
            return Err("duplicate or out-of-range trial".into());
        }
        let saved: Value = read(&dir.join(format!("{phase}-{j:04}.result.json")))?;
        if saved != *v {
            return Err("aggregate and individual trial records differ".into());
        }
        let system: Value = read(&dir.join(format!("{phase}-{j:04}.system.json")))?;
        if v["system_sha256"]
            != json!(sha256_hex(
                &serde_json::to_vec(&system).map_err(|e| e.to_string())?
            ))
        {
            return Err("system hash mismatch".into());
        }
        let target_xy: [u64; 2] =
            serde_json::from_value(v["target"].clone()).map_err(|e| e.to_string())?;
        if system["target"] != v["target"] || system["equations"] != json!(encode(target_xy[0])) {
            return Err("polynomial system is not the declared target/base system".into());
        }
        let mut ops = GroupOps::default();
        let scalar = 1 + derive_u64(
            &format!("factor-base-sweep/{phase}/v1"),
            &[req.plan.seed, j],
        ) % (r - 1);
        let target = if phase != "positive_control" {
            let q = group.mul(&mut ops, generator, scalar);
            if xy(q) != target_xy || v["public_control_scalar"] != json!(scalar) {
                return Err("target is not the matched seeded fixture".into());
            }
            Some(q)
        } else {
            None
        };
        let mut matched = None;
        if !v["witness_inventory_indices"].is_null() {
            let indices: [usize; 2] =
                serde_json::from_value(v["witness_inventory_indices"].clone())
                    .map_err(|e| e.to_string())?;
            if indices.iter().any(|k| *k >= points.len()) {
                return Err("witness outside inventory".into());
            }
            let q = group.add(&mut ops, points[indices[0]], points[indices[1]]);
            if group.is_identity(&q) || xy(q) != target_xy {
                return Err("point witness fails independent group addition".into());
            }
            if v["witness_verified"] != true || v["status"] != "verified_decomposition" {
                return Err("inconsistent witness status".into());
            }
            matched = Some(q);
            witnesses += 1;
            if phase == "train" {
                let mut row = vec![0u64; dump.factor_base.columns as usize];
                for k in indices {
                    let pt = &dump.points[k];
                    row[pt.col as usize] = (row[pt.col as usize] + pt.coef) % r;
                }
                let rhs = (scalar as u128 * h as u128 % r as u128) as u64;
                let before = matrix.rank();
                matrix.add_row(row.clone(), rhs);
                if v["relation"]
                    != json!({"row":row,"rhs":rhs,"cofactor_projection":h,"rank_gain":matrix.rank()-before})
                    || matrix.inconsistent != 0
                {
                    return Err("relation coefficients or independence do not replay".into());
                }
            } else if !v["relation"].is_null() {
                return Err("control/holdout was counted as a training relation".into());
            }
        } else if v["witness_verified"] != false
            || !v["relation"].is_null()
            || status == "verified_decomposition"
        {
            return Err("unverified relation was promoted".into());
        }
        let q = target
            .or(matched)
            .ok_or("positive control lacks a verified point witness")?;
        let truth = points
            .iter()
            .any(|&a| points.iter().any(|&b| group.add(&mut ops, a, b) == q));
        if v["ground_truth_decomposable"] != json!(truth) || (v["status"] == "refuted" && truth) {
            return Err("independent finite inventory census contradicts trial".into());
        }
    }
    if result["training_independent_rank"] != json!(matrix.rank()) {
        return Err("reported rank differs from relation replay".into());
    }
    for (phase, count) in counts {
        let s = &result["phase_summary"][phase];
        if s["trials"] != count[0]
            || s["verified"] != count[1]
            || s["exhausted"] != count[2]
            || s["refuted"] != count[3]
        {
            return Err("phase totals do not match replayed trials".into());
        }
    }
    if result["best_observed_successful_solving_degree"] != json!(best_degree) {
        return Err("training degree selection does not match recorded successful probes".into());
    }
    Ok(witnesses)
}
