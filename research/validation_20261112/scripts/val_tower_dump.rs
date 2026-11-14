// VALIDATOR TOOL (not producer code): dumps tower_large instances built by the
// committed hyper64::build64 with the run's seed formula, for independent checks.
use crypto_lib::cryptanalysis::chained_transfer::hyper64::build64;
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let primes: Vec<u64> = args[1].split(',').map(|t| t.trim().parse().unwrap()).collect();
    let instances: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);
    for &p in &primes {
        for inst in 0..instances {
            let seed = 1000 + 17 * inst as u64 + p % 997;
            let Some(t) = build64(p, seed, p / 4, 64) else {
                eprintln!("p={p} inst {inst}: none");
                continue;
            };
            let pt = |q: &crypto_lib::cryptanalysis::chained_transfer::ec::Pt| match q {
                None => serde_json::Value::Null,
                Some((x, y)) => json!([x.a, y.a]),
            };
            let rungs: Vec<_> = t
                .rungs
                .iter()
                .map(|r| {
                    json!({"genus": r.cover.genus, "f": r.cover.curve.f_coeffs(), "jac_order": r.jac_order,
                           "d": {"u": r.d.u_coeffs(), "v": r.d.v_coeffs()},
                           "t": {"u": r.t.u_coeffs(), "v": r.t.v_coeffs()}, "a": r.cover.a, "c": r.cover.c, "sigma": r.sigma})
                })
                .collect();
            println!(
                "{}",
                json!({"p": p, "instance": inst, "seed": seed, "e": [t.e.a2, t.e.a4, t.e.a6], "n_e": t.e.order(),
                       "r": t.r, "x": t.x, "gen": pt(&t.gen), "target": pt(&t.target), "rungs": rungs})
            );
        }
    }
}
