//! `rho`: protocol-matched reference, the library's negation-map parallel
//! distinguished-point rho (`prime_fast::rho_parallel`) on the same hidden instance.
use crate::cell::Cell;
use crate::util::*;
use crypto_lib::cryptanalysis::prime_fast::{rho_parallel, FastCurve, RhoConfig};
use serde_json::json;
use std::path::Path;
use std::time::Instant;

pub fn run(args: &[String]) -> Result<(), String> {
    let cell = Cell::load(Path::new(&crate::arg_req(args, "--cell")?))?;
    let tdir = crate::arg_req(args, "--targets")?;
    let seed: u64 = crate::arg_req(args, "--seed")?.parse().map_err(|_| "seed")?;
    let out = crate::arg_req(args, "--out")?;
    let nseeds: u64 = crate::arg_opt(args, "--seeds").map_or(Ok(3), |s| s.parse()).map_err(|_| "seeds")?;
    let threads: usize = crate::arg_opt(args, "--threads").map_or(Ok(1), |s| s.parse()).map_err(|_| "threads")?;
    let tj = read_json(&Path::new(&tdir).join("targets.json"))?;
    if !check_seal(&tj) || tj["cell_sha256"].as_str() != Some(&cell.sha256) {
        return Err("targets.json missing, unsealed or for another cell".into());
    }
    let (qx, qy) = (u64_of(&tj["Q"], "x")?, u64_of(&tj["Q"], "y")?);
    let fc = FastCurve::new(&cell.slug, cell.p, cell.a, cell.b, cell.r);
    let g = fc.point(cell.px, cell.py);
    let q = fc.point(qx, qy);
    if !fc.is_on_curve(g) || !fc.is_on_curve(q) {
        return Err("P or Q not on the curve".into());
    }
    let bits = 64 - cell.r.leading_zeros();
    let mut runs = Vec::new();
    for i in 0..nseeds {
        let mut cfg = RhoConfig::for_bits(bits, true);
        cfg.threads = threads;
        cfg.seed = seed.wrapping_add(i);
        let (c0, t0) = (cpu_secs(), Instant::now());
        let res = rho_parallel(&fc, g, q, &cfg);
        let (cpu, wall) = (cpu_secs() - c0, t0.elapsed().as_secs_f64());
        match res {
            Some(r) => {
                let ok = fc.mul(Some(g), r.log) == Some(q);
                let setup_combines = (1u64 << cfg.table_bits) + (cfg.walkers * cfg.threads) as u64;
                runs.push(json!({
                    "seed": cfg.seed, "solved": true, "log_verified_xP_eq_Q": ok,
                    "walk_steps_group_additions": r.total_steps,
                    "distinguished_points": r.distinguished_points,
                    "setup_combines_lower_bound": setup_combines,
                    "setup_note": "each combine is one Straus double-scalar multiplication (about 1.5*bits Jacobian operations); restarts after fruitless cycles are not counted: group additions are a lower bound",
                    "cpu_secs": cpu, "wall_secs": wall,
                    "threads": r.threads, "walkers_per_thread": r.walkers_per_thread, "dp_bits": r.dp_bits,
                }));
            }
            None => runs.push(json!({"seed": cfg.seed, "solved": false, "cpu_secs": cpu, "wall_secs": wall})),
        }
    }
    let adds: Vec<f64> = runs
        .iter()
        .filter_map(|r| r["walk_steps_group_additions"].as_u64().map(|x| x as f64))
        .collect();
    let mean = if adds.is_empty() { None } else { Some(adds.iter().sum::<f64>() / adds.len() as f64) };
    let rec = json!({
        "schema": "pkm.m8-panel.rho/v1",
        "cell_sha256": cell.sha256, "targets_sha256": tj["sha256"],
        "instance": "log_P(Q) of the cell's hidden instance (log never read; checked by xP = Q)",
        "r": cell.r, "bits": bits, "method": "prime_fast::rho_parallel, negation map, r-adding table, DP table",
        "runs": runs, "mean_group_additions": mean,
        "predicted_walk_steps": (std::f64::consts::PI * cell.r as f64 / 4.0).sqrt(),
        "reference_only": "decides nothing; unit is one affine group addition (GAE) per walk step",
        "vm_hwm_kb": vm_hwm_kb(),
        "tool": crate::tool_info(),
    });
    write_json(Path::new(&out), &seal(rec))?;
    println!("rho: {} run(s), mean group additions {:?}", runs.len(), mean);
    Ok(())
}
