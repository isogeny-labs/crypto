//! Validate a single graph IC worker. Never retain a cohort of raw JSON trees.
#![allow(clippy::duplicate_mod)]
#[cfg(test)]
#[allow(dead_code, unused_imports)] // Standalone example's reexport when nested for unit fixtures.
#[path = "jv_sieve_quadratic_pair_relation_controls.rs"]
mod admission_controls;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_collection_check.rs"]
mod collection_check;
#[path = "../research/extension_ic_graph_20261115/deferred_pair_graph_check.rs"]
mod deferred_graph_math_check;
#[cfg(test)]
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_deferred_pair_graph.rs"]
mod deferred_pair_graph;
#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[path = "jv_sieve_quartic_discriminant_controls.rs"]
mod discriminant_controls;
#[path = "../research/extension_ic_graph_20261115/quartic_discriminant_check.rs"]
mod discriminant_math_check;
#[allow(dead_code, clippy::type_complexity)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[path = "../research/extension_ic_graph_20261115/discriminant_pair_enumeration_check.rs"]
mod enumeration_math_check;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_graph_check.rs"]
mod graph_math_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/deferred_pair_pipeline_check.rs"]
mod pair_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/deferred_pair_collection_check.rs"]
mod pair_collection_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_driver.rs"]
mod pair_driver;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_certificate_check.rs"]
mod pair_math_check;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_math_check.rs"]
mod sparse_math_check;
// The production wire type is deserialized only. Independent arithmetic and
// graph replay do not invoke its constructors, splitter or replay helpers.
pub use crypto_lib::cryptanalysis;
#[cfg(test)]
#[path = "../src/cryptanalysis/jv_nagao_unit.rs"]
#[allow(dead_code)]
mod nagao_unit;
#[cfg(test)]
#[path = "../src/cryptanalysis/jv_nagao_unit_elimination.rs"]
mod nagao_unit_elimination;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
mod principal_relation;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
mod quadratic_pair_relation;
#[path = "../research/extension_ic_graph_20261115/unit_descent_check.rs"]
#[allow(dead_code)] // Reuse independent substitution arithmetic, not the legacy cohort driver.
mod unit_descent_check;

use crypto_lib::cryptanalysis::{jv_bitops as work, jv_cover as cover};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write};

fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn digest(value: &Value) -> Result<String, serde_json::Error> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(value)?)))
}
fn backend(method: &str) -> Option<(&'static str, bool, bool)> {
    Some(match method {
        "unit-deferred-pairs" => ("exact-unit-five", true, false),
        "fast-unit-deferred-pairs" => ("exact-unit-five-fast", true, false),
        "traced-unit-deferred-pairs" => ("exact-unit-five-fast-traced", true, true),
        "traced-six-deferred-pairs" => ("original-six-traced", false, true),
        _ => return None,
    })
}
fn coordinate_attempts(w: &Value, c: &pair_driver::Record) -> Result<Value, String> {
    let (tag, unit, traced) =
        backend(w["method"].as_str().ok_or("method")?).ok_or("unsupported method")?;
    ensure(w["work"]["nagao_backend"] == tag, "backend identity")?;
    if tag != "exact-unit-five" {
        ensure(
            w["work"]["coordinate_trace_reuse"] == traced,
            "trace policy",
        )?;
    }
    let curve = &c.public;
    let spec = cover::spec_from_curve(
        curve.p,
        cover::E6(curve.alpha.map(cover::E2)),
        curve.l,
        curve.g.native(),
        curve.q.native(),
        0,
    )
    .ok_or("public transfer")?;
    let ctx = cover::Ctx::new(&spec);
    let jac = ctx.jac();
    let mut rng = StdRng::seed_from_u64(c.seed ^ 0x51E7E);
    let da = work::uniform_mod_u64(&mut rng, 1, curve.l);
    let db = work::uniform_mod_u64(&mut rng, 1, curve.l);
    let a0 = work::uniform_mod_u64(&mut rng, 0, curve.l);
    let b0 = work::uniform_mod_u64(&mut rng, 1, curve.l);
    let step = jac.add(
        &jac.mul(&spec.gj, da as u128),
        &jac.mul(&spec.qj, db as u128),
    );
    let mut r = jac.add(
        &jac.mul(&spec.gj, a0 as u128),
        &jac.mul(&spec.qj, b0 as u128),
    );
    let attempts = w["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    ensure(
        !attempts.is_empty()
            && attempts.len() <= 8192
            && w["work"]["descent_residuals"] == json!(attempts.len()),
        "complete bounded attempts",
    )?;
    let (
        mut roots,
        mut empty,
        mut degenerate,
        mut replayed,
        mut fallbacks,
        mut timeouts,
        mut incomplete,
    ) = (0usize, 0, 0, 0, 0, 0, 0);
    for (i, t) in attempts.iter().enumerate() {
        let encode = |v: &[cover::E2]| v.iter().map(|a| a.0).collect::<Vec<_>>();
        ensure(
            t["residual_index"] == json!(i)
                && t["u"] == json!(encode(&r.u))
                && t["v"] == json!(encode(&r.v)),
            "every residual input binding",
        )?;
        if r.u.len() != 4 {
            if unit {
                ensure(t["audit"].is_null(), "degenerate audit refusal")?;
            }
            degenerate += 1;
        } else {
            let stats = if unit {
                let mut v = r.v.clone();
                v.resize(3, cover::E2::ZERO);
                let input =
                    cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &r.u, &v).polys(&ctx.reim, curve.p);
                ensure(
                    t["audit"]["original"] == json!(input),
                    "shared baseline Weil input binding",
                )?;
                let (_, n) = unit_descent_check::system(&t["audit"], curve.p)?;
                roots += n;
                empty += usize::from(n == 0);
                &t["audit"]["stats"]
            } else {
                &t["six_coordinate_cost"]
            };
            // Missing flags must not become false through as_bool().unwrap_or(false).
            for key in ["replayed", "trace_mismatch", "timed_out", "incomplete"] {
                ensure(stats[key].is_boolean(), "explicit solver status")?;
            }
            replayed += usize::from(stats["replayed"] == true);
            fallbacks += usize::from(stats["trace_mismatch"] == true);
            timeouts += usize::from(stats["timed_out"] == true);
            incomplete += usize::from(stats["incomplete"] == true);
            if !traced {
                ensure(
                    stats["replayed"] == false && stats["trace_mismatch"] == false,
                    "untraced control flags",
                )?;
            }
        }
        if i + 1 < attempts.len() {
            r = jac.add(&r, &step);
        }
    }
    ensure(
        w["work"]["descent_timeouts"] == json!(timeouts)
            && w["work"]["descent_incomplete"] == json!(incomplete),
        "all cap counters",
    )?;
    if tag != "exact-unit-five" {
        ensure(
            w["work"]["coordinate_trace_replayed"] == json!(replayed)
                && w["work"]["coordinate_trace_shape_fallbacks"] == json!(fallbacks),
            "all replay and fallback counters",
        )?;
    }
    Ok(
        json!({"attempts":attempts.len(),"degenerate":degenerate,"unit_reduction":unit,
        "lifted_roots":if unit {Some(roots)} else {None},"empty_coordinate_sets":if unit {Some(empty)} else {None},
        "replayed":replayed,"shape_fallbacks":fallbacks,"timed_out":timeouts,"incomplete":incomplete,
        "every_input_bound":true,"independent_substitution_and_lifting":unit,
        "input_constructor":"shared unchanged baseline Weil constructor; no independent geometry claim",
        "six_coordinate_root_completeness":"not independently solved here; separate captured full-cohort root replay plus accepted-row group replay"}),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("usage: ARM_CHECK RAW PUBLIC_JSON EXPECTED_SOURCE NEW_PROOF".into());
    }
    ensure(
        args[2].len() == 40 && args[2].bytes().all(|b| b.is_ascii_hexdigit()),
        "source revision",
    )?;
    ensure(
        fs::metadata(&args[0])?.len() <= 512 * 1024 * 1024,
        "bounded raw input",
    )?;
    let bytes = fs::read(&args[0])?;
    let raw_sha = format!("{:x}", Sha256::digest(&bytes));
    let w: Value = serde_json::from_slice(&bytes)?;
    drop(bytes);
    let public_bytes = fs::read(&args[1])?;
    let public_sha = format!("{:x}", Sha256::digest(&public_bytes));
    let public: Value = serde_json::from_slice(&public_bytes)?;
    ensure(
        w["schema"] == "cover.two-large-prime.dlp-worker/v1"
            && w["source_revision"] == args[2]
            && w["public"] == public
            && w["solver_answer_placeholder"] == 0
            && w["verified"] == true
            && w["capped"] == false,
        "worker source/public/endpoint identity",
    )?;
    let seed = w["seed"].as_u64().ok_or("seed")?;
    let scalar = w["actual_scalar"].as_u64().ok_or("actual scalar")?;
    let c = pair_check::collection(&w, &public_sha)?;
    ensure(
        scalar > 0 && scalar < c.public.l,
        "canonical recovered scalar",
    )?;
    let cp =
        pair_collection_check::qualify_sparse(&c, seed, &public_sha, &w["work"]["collection"])?;
    let pipeline = pair_check::pipeline(&w, &c, &cp, scalar, 8192, 32)?;
    let coordinate = coordinate_attempts(&w, &c)?;
    drop(c);
    // Temporary canonical views for comparison; the original raw bytes stay untouched.
    let mut collection = w["work"]["collection"].clone();
    for check in collection["rank_checks"]
        .as_array_mut()
        .ok_or("rank checks")?
    {
        check["native"]
            .as_object_mut()
            .ok_or("rank native")?
            .remove("inner_wall_ns");
    }
    let collection_sha = digest(&collection)?;
    drop(collection);
    let geometry: serde_json::Map<String, Value> = [
        "descents",
        "affine_rows",
        "projected_rows",
        "descent_residuals",
        "descent_stream_advances",
        "descent_timeouts",
        "descent_incomplete",
        "invalid_descents",
        "initial_basis",
        "final_extra_counts",
        "last_rank",
        "actual_scalar",
    ]
    .into_iter()
    .map(|key| (key.into(), w["work"][key].clone()))
    .collect();
    let non_descent: Vec<_> = w["work_native"]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .filter(|p| p["phase"] != "descent")
        .collect();
    let proof = json!({"schema":"cover.unit-trace.single-arm-correctness/v1","raw_sha256":raw_sha,
        "public_sha256":public_sha,"source_revision":args[2],"method":w["method"],"seed":seed,"actual_scalar":scalar,
        "pipeline":pipeline,"coordinate":coordinate,"collection_without_rank_wall_sha256":collection_sha,
        "descent_affine_geometry_sha256":digest(&Value::Object(geometry))?,"non_descent_primitives_sha256":digest(&json!(non_descent))?,
        "raw_workers_resident":1,"math_replay_only":true,"independent_signed_verdict":false,
        "complete_cost":false,"total_bitops":null,"ratios":null,"rho_parity":false});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    serde_json::to_writer_pretty(&mut file, &proof)?;
    file.write_all(b"\n")?;
    println!("One-worker mathematical replay complete; complete-cost and parity remain unset.");
    Ok(())
}
