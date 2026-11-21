//! Native development qualifier; no wall ranking or complete-cost assertion.
#![allow(dead_code, clippy::too_many_arguments, clippy::type_complexity)]
#[path = "../anchored_descent.rs"]
mod anchored;
#[path = "../anchored_nagao.rs"]
mod nagao;
#[path = "../extension_remainder.rs"]
mod remainder;
mod unit_elimination;

use crypto_lib::cryptanalysis::{
    f4_fp,
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, E2},
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

fn qualify(file: &mut (impl Write + Send)) -> Result<Value, String> {
    let mut completed = 0;
    for (p, seed) in [(53, 2), (101, 409), (1009, 389)] {
        work::enable(false);
        let spec = cover::generate_spec(p, seed);
        let ctx = cover::Ctx::new(&spec);
        let jac = ctx.jac();
        let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
        let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        let mut planted = jac.identity();
        for b in base.iter().take(6) {
            planted = jac.add(&planted, &b.d);
        }
        if base.len() < 6 {
            return Err("planted fixture lacks base".into());
        }
        let opts = cover::opts_for_mode(24, 30.0, cover::StopMode::Staircase(64));
        for (label, r) in [("generator", spec.gj.clone()), ("planted-six", planted)] {
            if r.u.len() != 4 {
                return Err(format!("degenerate fixture {p}/{label}"));
            }
            let mut v = r.v.clone();
            v.resize(3, E2::ZERO);
            let forms = cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &r.u, &v);
            let polys = forms.polys(&ctx.reim, p);
            let mut expected_roots = None;
            let mut expected_rows = None;
            let mut controls: HashMap<&str, Value> = HashMap::new();
            for (arm, repeat) in [("six", 0), ("unit", 0), ("six", 1), ("unit", 1)] {
                work::claim();
                work::set_moduli(p, spec.l);
                work::enable(true);
                let before = work::snapshot();
                let _phase = work::enter(Phase::Descent);
                let mut rng = StdRng::seed_from_u64(947);
                let (mut roots, stats) = if arm == "six" {
                    cover::solve_zero_dim(&polys, 6, p, &opts, &mut rng)
                } else {
                    unit_elimination::solve(&polys, p, &opts, &mut rng).map_err(str::to_string)?
                };
                roots.sort();
                roots.dedup();
                if stats.timed_out || stats.incomplete || stats.nonseparating != 0 {
                    return Err(format!(
                        "capped/incomplete coordinate fixture {p}/{label}/{arm}: {stats:?}"
                    ));
                }
                if roots
                    .iter()
                    .any(|x| x.len() != 6 || polys.iter().any(|f| f4_fp::eval(f, x, p) != 0))
                {
                    return Err("coordinate replay".into());
                }
                let coordinate = work::snapshot().since(&before).total();
                let coordinate_count = work::price(&coordinate, &work::widths(p, spec.l))
                    .map_err(|e| format!("{e:?}"))?;
                work::enable(false);
                if let Some(expected) = &expected_roots {
                    if expected != &roots {
                        return Err(format!("different coordinate sets {p}/{label}/{arm}"));
                    }
                } else {
                    expected_roots = Some(roots.clone());
                }

                // Start a second clean solve so changed extraction randomness is
                // included, then compare full cofactors and group-checked rows.
                work::enable(true);
                let before = work::snapshot();
                let mut rng = StdRng::seed_from_u64(947);
                let out = nagao::decompose_using(
                    &ctx,
                    &base,
                    &by_x,
                    &r,
                    &opts,
                    &mut rng,
                    true,
                    |polys, p, opts, rng| {
                        if arm == "six" {
                            Ok(cover::solve_zero_dim(polys, 6, p, opts, rng))
                        } else {
                            unit_elimination::solve(polys, p, opts, rng)
                        }
                    },
                )
                .map_err(str::to_string)?;
                if out.stats.timed_out || out.stats.incomplete || out.stats.nonseparating != 0 {
                    return Err("capped/incomplete materializer".into());
                }
                if out
                    .full
                    .iter()
                    .any(|d| !cover::verify_dec(&jac, &base, &r, d))
                    || out
                        .partial
                        .iter()
                        .any(|d| !anchored::replay(&jac, &base, &r, d.certificate()))
                {
                    return Err("group replay".into());
                }
                let tally = work::snapshot().since(&before).total();
                let subtotal =
                    work::price(&tally, &work::widths(p, spec.l)).map_err(|e| format!("{e:?}"))?;
                let foreign = work::foreign();
                work::enable(false);
                if foreign != 0 {
                    return Err("foreign-thread accounting".into());
                }
                if label == "planted-six" && out.full.is_empty() {
                    return Err("lost planted decomposition".into());
                }
                let mut geometry: Vec<Value> = out
                    .cofactors
                    .iter()
                    .map(|x| serde_json::to_value(x).unwrap())
                    .collect();
                geometry.sort_by_key(Value::to_string);
                let rows = json!({"full":out.full,"cofactors":geometry});
                if let Some(expected) = &expected_rows {
                    if expected != &rows {
                        return Err(format!("different materialized rows {p}/{label}/{arm}"));
                    }
                } else {
                    expected_rows = Some(rows.clone());
                }
                let stable = json!({"coordinate":coordinate_count.to_string(),"materializer":subtotal.to_string(),"counts":work::prims_json(&tally)});
                if let Some(old) = controls.get(arm) {
                    if old != &stable {
                        return Err("A/A deterministic counter mismatch".into());
                    }
                } else {
                    controls.insert(arm, stable.clone());
                }
                let record = json!({"type":"case","p":p,"seed":seed,"residual":label,"arm":arm,"repeat":repeat,
                    "input":{"u":remainder::encode(&r.u),"v":remainder::encode(&r.v)},
                    "roots":roots,"rows":rows,"work":stable,"foreign_thread_hooks":foreign,
                    "f4":{"degree":stats.degree_reached,"max_rows":stats.max_rows,"max_cols":stats.max_cols,"delta":stats.delta,"muls":stats.f4_muls,"linear_muls":stats.lin_muls}});
                serde_json::to_writer(&mut *file, &record).map_err(|e| e.to_string())?;
                file.write_all(b"\n").map_err(|e| e.to_string())?;
                file.flush().map_err(|e| e.to_string())?;
                completed += 1;
            }
        }
    }
    Ok(
        json!({"type":"summary","completed":completed,"complete_cost":false,"ratios":null,"wall_ranking":null,"known_uncounted":["container comparisons/allocation","some existing solver/control arithmetic","serialization and PRNG internals"],"fresh_unknown_log":false}),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: QUALIFIER NEW_OUTPUT_JSON")?;
    // Reserve before computing, preserving failures and preventing overwrite.
    let mut file = BufWriter::new(OpenOptions::new().create_new(true).write(true).open(path)?);
    serde_json::to_writer(
        &mut file,
        &json!({"type":"header","schema":"nagao.unit-elimination.development/v1","source_revision":std::env::var("IC_SOURCE_REVISION").ok(),"library_revision":"232925556a6e39d4589932c60e75a1bf24f9e049"}),
    )?;
    file.write_all(b"\n")?;
    file.flush()?;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    let result = pool.install(|| qualify(&mut file));
    match result {
        Ok(value) => {
            serde_json::to_writer(&mut file, &value)?;
            file.write_all(b"\n")?;
            file.flush()?;
        }
        Err(error) => {
            serde_json::to_writer(&mut file, &json!({"failed":error}))?;
            file.write_all(b"\n")?;
            file.flush()?;
            return Err(error.into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_rayon_hooks_are_owned_and_counted() {
        use rayon::prelude::*;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        pool.install(|| {
            work::claim();
            work::set_moduli(101, 103);
            work::enable(true);
            let before = work::snapshot();
            (0..17).into_par_iter().for_each(|_| {
                work::hook_mul(101);
                work::hook_add(103);
            });
            let counts = work::prims_json(&work::snapshot().since(&before).total());
            work::enable(false);
            assert_eq!(counts["field_mul"], 17);
            assert_eq!(counts["modr_add"], 17);
            assert_eq!(work::foreign(), 0);
        });
    }
    #[test]
    fn backend_refusals_and_malformed_roots_remain_errors() {
        let spec = cover::generate_spec(101, 409);
        let ctx = cover::Ctx::new(&spec);
        let opts = cover::jv_options(24, 30.0);
        let mut rng = StdRng::seed_from_u64(13);
        let base = cover::factor_base(&ctx, &mut rng);
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        assert!(nagao::decompose_using(
            &ctx,
            &base,
            &by_x,
            &spec.gj,
            &opts,
            &mut rng,
            true,
            |_, _, _, _| Err("explicit refusal")
        )
        .is_err());
        assert!(nagao::decompose_using(
            &ctx,
            &base,
            &by_x,
            &spec.gj,
            &opts,
            &mut rng,
            true,
            |_, _, _, _| Ok((vec![vec![0; 5]], cover::ZeroDimStats::default()))
        )
        .is_err());
    }
    #[test]
    fn reduced_backend_preserves_a_positive_quadratic_partial() {
        let spec = cover::generate_spec(101, 409);
        let ctx = cover::Ctx::new(&spec);
        let opts = cover::jv_options(24, 30.0);
        let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        // Bounded correctness search only: not part of the frozen cost cohort.
        // Stop at the first baseline partial, without conditioning on speed.
        for k in 1..=128 {
            let r = ctx.jac().mul(&spec.gj, k);
            let old = nagao::decompose(
                &ctx,
                &base,
                &by_x,
                &r,
                &opts,
                &mut StdRng::seed_from_u64(947),
                true,
            );
            assert!(!old.stats.timed_out && !old.stats.incomplete);
            if old.partial.is_empty() {
                continue;
            }
            let new = nagao::decompose_using(
                &ctx,
                &base,
                &by_x,
                &r,
                &opts,
                &mut StdRng::seed_from_u64(947),
                true,
                unit_elimination::solve,
            )
            .unwrap();
            assert!(!new.stats.timed_out && !new.stats.incomplete);
            let mut a: Vec<_> = old
                .cofactors
                .iter()
                .map(|x| serde_json::to_string(x).unwrap())
                .collect();
            let mut b: Vec<_> = new
                .cofactors
                .iter()
                .map(|x| serde_json::to_string(x).unwrap())
                .collect();
            a.sort();
            b.sort();
            assert_eq!(a, b);
            assert_eq!(old.full, new.full);
            assert_eq!(old.partial.len(), new.partial.len());
            for row in new.partial {
                assert!(anchored::replay(&ctx.jac(), &base, &r, row.certificate()));
            }
            println!("first positive quadratic partial at [k]G with k={k}");
            return;
        }
        panic!("no positive partial in registered correctness-search cap");
    }
}
