// VALIDATOR TOOL (not producer code). Rebuilds tower_large instances with the
// committed hyper64::build64 and the run's seed formula, re-runs the committed
// solvers exactly as src/bin/tower_large.rs measure() calls them (rho_reference
// seeds 100+s, cap 1<<26; ic::solve seeds 300+s, crosscheck::ic_params()), and
// prints the instance data and every solver's returned log and counted ops, so
// that the validator's Python (vlib.py) can certify each returned log with its
// own arithmetic and recompute the per-instance means of tower_large.json.
use crypto_lib::cryptanalysis::chained_transfer::hyper64::{build64, crosscheck, ic};
use crypto_lib::cryptanalysis::chained_transfer::measure::EcGroup;
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let primes: Vec<u64> = args[1].split(',').map(|t| t.trim().parse().unwrap()).collect();
    let instances: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);
    let seeds: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(8);
    let rho_seeds: u64 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(64);
    let params = crosscheck::ic_params();
    for &p in &primes {
        for inst in 0..instances {
            let seed = 1000 + 17 * inst as u64 + p % 997;
            let Some(t) = build64(p, seed, p / 4, 64) else {
                println!("{}", json!({"p": p, "instance": inst, "none": true}));
                continue;
            };
            let pt = |q: &crypto_lib::cryptanalysis::chained_transfer::ec::Pt| match q {
                None => serde_json::Value::Null,
                Some((x, y)) => json!([x.a, y.a]),
            };
            let eg = EcGroup(t.e);
            let mut rho = vec![];
            for s in 0..rho_seeds {
                let o = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 26);
                rho.push(json!({"seed": 100 + s, "k": o.recovered,
                    "ops": o.group_ops.adds + o.group_ops.doubles, "steps": o.steps, "dps": o.distinguished_points}));
            }
            let mut rungs = vec![];
            for rg in &t.rungs {
                let cur = &rg.cover.curve;
                let mut sol = vec![];
                for s in 0..seeds {
                    match ic::solve(cur, rg.d, rg.t, t.r, rg.jac_order, 300 + s, &params) {
                        Ok(o) => sol.push(json!({"seed": 300 + s, "k": o.recovered, "ops": o.group_ops(),
                            "la_mulmods": o.la_mulmods(), "trials": o.counter("trials"),
                            "relations": o.counter("relations"), "columns": o.counter("factor_base_columns")})),
                        Err(e) => sol.push(json!({"seed": 300 + s, "error": format!("{e}")})),
                    }
                }
                rungs.push(json!({"genus": rg.cover.genus, "f": cur.f_coeffs(), "jac_order": rg.jac_order,
                    "d": {"u": rg.d.u_coeffs(), "v": rg.d.v_coeffs()},
                    "t": {"u": rg.t.u_coeffs(), "v": rg.t.v_coeffs()}, "ic": sol}));
            }
            println!("{}", json!({"p": p, "instance": inst, "seed": seed, "e": [t.e.a2, t.e.a4, t.e.a6],
                "n_e": t.e.order(), "r": t.r, "x": t.x, "gen": pt(&t.gen), "target": pt(&t.target),
                "rho": rho, "rungs": rungs}));
        }
    }
}
