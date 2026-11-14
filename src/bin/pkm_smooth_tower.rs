//! `pkm-smooth-tower` — Petit–Kosters–Messeng factor bases with mixed
//! smooth degrees over a given prime (`crypto_lib::cryptanalysis::pkm_smooth_tower`).
//!
//! ```text
//! pkm-smooth-tower kummer  --p P --degrees 3,2,5 [--g G] [--seed S]
//! pkm-smooth-tower search  --p P --bound B --min-size N [--seed S] [--max-curves K]
//! pkm-smooth-tower isogeny --p P --bound B --min-size N [--seed S] [--max-curves K]
//!                          [--curve A,B --target-x X --m M --system OUT.ms]
//! ```
//!
//! `kummer` needs `∏ degrees | p − 1`. `search` is Method 1 only (the
//! auxiliary curve). `isogeny` searches and builds the Vélu chain; with
//! `--curve` and `--target-x` it also writes slide 17's system for `m`
//! points in msolve format. Every tower is verified (`|V| = S`, `L = c`
//! on `V`, and below 2^22 no other root) before anything is printed.
//! Output is one JSON object on stdout. Exit 0 ok, 1 failure, 2 usage.

use crypto_lib::cryptanalysis::pkm_smooth_tower::*;
use serde_json::json;
use std::collections::HashMap;

fn usage() -> ! {
    eprintln!(
        "usage: pkm-smooth-tower kummer|search|isogeny --p P [options]; see the source header"
    );
    std::process::exit(2)
}

fn tower_json(t: &SmoothTower) -> serde_json::Value {
    let kind = match &t.kind {
        TowerKind::Kummer { g } => json!({"form": "kummer", "g": g}),
        TowerKind::Isogeny {
            aux,
            aux_order,
            q,
            r,
            codomains,
            curves_tried,
        } => json!({
            "form": "isogeny",
            "aux": {"a": aux.a, "b": aux.b, "order": aux_order},
            "Q": [q.0, q.1], "R": [r.0, r.1],
            "codomains": codomains.iter().map(|c| [c.a, c.b]).collect::<Vec<_>>(),
            "curves_tried": curves_tried,
        }),
    };
    json!({
        "p": t.p, "degrees": t.degrees, "S": t.size(), "c": t.c,
        "steps": t.steps.iter().map(|s| json!({"degree": s.degree, "num": s.num, "den": s.den})).collect::<Vec<_>>(),
        "V": t.v, "kind": kind, "verified": true,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }
    let mut o: HashMap<String, String> = HashMap::new();
    let mut i = 2;
    while i + 1 < args.len() {
        o.insert(
            args[i].trim_start_matches("--").to_string(),
            args[i + 1].clone(),
        );
        i += 2;
    }
    let num = |k: &str, d: Option<u64>| -> u64 {
        o.get(k)
            .map(|v| v.parse().unwrap_or_else(|_| usage()))
            .or(d)
            .unwrap_or_else(|| usage())
    };
    let p = num("p", None);
    let seed = num("seed", Some(1));
    let res: Result<serde_json::Value, String> = (|| match args[1].as_str() {
        "kummer" => {
            let degs: Vec<u64> = o
                .get("degrees")
                .ok_or("--degrees")?
                .split(',')
                .map(|s| s.parse().map_err(|_| "bad degree"))
                .collect::<Result<_, _>>()?;
            let t = kummer_tower(p, &degs, o.get("g").map(|g| g.parse().unwrap_or(1)), seed)?;
            t.verify()?;
            Ok(tower_json(&t))
        }
        "search" | "isogeny" => {
            let f = search_aux(
                p,
                num("bound", None),
                num("min-size", None),
                seed,
                num("max-curves", Some(100_000)),
            )?;
            if args[1] == "search" {
                return Ok(
                    json!({"p": p, "aux": {"a": f.aux.a, "b": f.aux.b, "order": f.order},
                    "degrees": f.degrees, "S": f.degrees.iter().product::<u64>(), "Q": [f.q.0, f.q.1],
                    "curves_tried": f.curves_tried, "orders_failed": f.orders_failed}),
                );
            }
            let t = isogeny_tower_on(f.aux, f.order, f.q, &f.degrees, seed, f.curves_tried)?;
            t.verify()?;
            let mut out = tower_json(&t);
            if let Some(cv) = o.get("curve") {
                let ab: Vec<u64> = cv
                    .split(',')
                    .map(|s| s.parse().unwrap_or_else(|_| usage()))
                    .collect();
                let e = Curve {
                    p,
                    a: ab[0] % p,
                    b: ab[1] % p,
                };
                let xr = num("target-x", None);
                let m = num("m", Some(2)) as usize;
                let sys = relation_system_msolve(&e, &t, m, xr);
                if let Some(path) = o.get("system") {
                    std::fs::write(path, &sys).map_err(|e| e.to_string())?;
                }
                let base =
                    t.v.iter()
                        .filter(|&&x| sqrtm(e.rhs(x), p).is_some())
                        .count();
                out["factor_base_abscissae_on_E"] = json!(base);
                if m == 2 {
                    if let Some(y) = sqrtm(e.rhs(xr), p) {
                        let rels = decompose_m2(&e, &t.v, (xr, y));
                        out["m2_relations"] = json!(rels
                            .iter()
                            .map(|r| [[r.p1.0, r.p1.1], [r.p2.0, r.p2.1]])
                            .collect::<Vec<_>>());
                    }
                }
            }
            Ok(out)
        }
        _ => usage(),
    })();
    match res {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            eprintln!("pkm-smooth-tower: {e}");
            std::process::exit(1)
        }
    }
}
