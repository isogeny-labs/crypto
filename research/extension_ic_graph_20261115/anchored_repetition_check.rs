//! Compare geometry against the failed capture; never reuse its invalid costs.
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};
fn ensure(ok: bool, why: &str) -> Result<(), Box<dyn std::error::Error>> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("usage: REPLAY OLD42_RAW_DIR NEW_RUN_DIR NEW_RECEIPT".into());
    }
    let old = Path::new(&args[0]);
    let new = Path::new(&args[1]);
    let mut checked = Vec::new();
    for mode in ["original-nagao", "full-only", "anchored"] {
        let read = |d: &Path| -> Result<Value, Box<dyn std::error::Error>> {
            Ok(serde_json::from_slice(&fs::read(
                d.join(format!("{mode}.raw.json")),
            )?)?)
        };
        let a = read(old)?;
        let b = read(new)?;
        ensure(
            a["source_revision"] == "9c7ab02c1e20cd5d0fe93b3b49a53c087681a78f"
                && a["foreign_thread_hooks"] == 1
                && b["foreign_thread_hooks"] == 0,
            "old rejection and new ownership gate",
        )?;
        for key in [
            "input_sha256",
            "mode",
            "seed",
            "p",
            "l",
            "max_residuals",
            "max_rows",
            "stream",
            "rows",
            "partial_rows",
            "matched_partials",
            "unmatched_partials",
            "projection_refusals",
            "rank",
            "actual_scalar",
            "verified",
            "capped",
            "resource_refusal",
        ] {
            ensure(
                a[key] == b[key],
                &format!("unchanged geometric/endpoint field {key}"),
            )?;
        }
        let x = a["trials"].as_array().ok_or("old trials")?;
        let y = b["trials"].as_array().ok_or("new trials")?;
        ensure(x.len() == y.len(), "same completed residual prefix")?;
        for (i, (x, y)) in x.iter().zip(y).enumerate() {
            for key in [
                "index",
                "a",
                "b",
                "residual",
                "full",
                "cofactors",
                "algebraic_solutions",
                "degenerate",
                "cofactor_prefix_sha256",
                "selected_row",
            ] {
                ensure(x[key] == y[key], &format!("same residual {i} field {key}"))?;
            }
            ensure(
                x["stats"]["partial_events"] == y["stats"]["partial_events"],
                "same anchored class lookup/projection outcomes",
            )?;
        }
        checked.push(json!({"mode":mode,"equal_residual_prefixes":x.len(),"all_full_cofactor_partial_affine_endpoints_equal":true,"old_foreign_hooks":1,"new_foreign_hooks":0,"old_costs_reused":false}));
    }
    let receipt = json!({"schema":"cover.anchored-descent.ownership-repeat/v1","seed":401,"workers":checked,"old_capture_still_rejected":true,"old_costs_repriced":false,"fresh_dlp_instance":false,"scope":"Changed-source ownership correction on the identical recorded-state workload. Original copied checker must independently qualify the new capture; this comparison supplies no cost admission for the failed one."});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &receipt)?;
    out.write_all(b"\n")?;
    Ok(())
}
