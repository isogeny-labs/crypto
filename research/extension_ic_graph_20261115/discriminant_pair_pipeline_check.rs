//! Independent full-matrix target recovery and exact post-collection charges.
use crate::{pair_collection_check as component, pair_driver as driver};
#[allow(dead_code)]
#[path = "native_analysis/src/bin/two-lp-dlp-analysis.rs"]
pub(crate) mod full;
#[allow(dead_code)]
#[path = "native_analysis/src/bin/rho-native-price-analysis.rs"]
pub(crate) mod rho;
use component::accounting::{add, counts, mul, n, KINDS};
use crypto_lib::cryptanalysis::{
    jv_bitops as work,
    jv_cover::{self as cover, BaseEl, Div, Fld, E2, E6},
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeMap;
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
pub fn collection(worker: &Value, input_sha: &str) -> Result<driver::Record, String> {
    let mut c = worker["work"]["collection"].clone();
    ensure(
        (c["schema"] == "cover.quadratic-pair.collection/v1"
            && c["mode"] == "grounded-quadratic-pairs")
            || (c["schema"] == "cover.discriminant-pair.collection/v1"
                && c["mode"] == "discriminant-quadratic-pairs"),
        "production collection schema",
    )?;
    c["schema"] = json!("cover.extension-pipeline.collection-projection/v1");
    c["source_revision"] = worker["source_revision"].clone();
    c["input_sha256"] = json!(input_sha);
    c["public"] = worker["public"].clone();
    c["inner_wall_ns"] = json!(0);
    serde_json::from_value(c).map_err(|e| e.to_string())
}
fn phase<'a>(w: &'a Value, name: &str) -> Result<&'a Value, String> {
    w["phases"]
        .as_array()
        .ok_or("phase list")?
        .iter()
        .find(|r| r["phase"] == name)
        .map(|r| &r["primitives"])
        .ok_or("missing phase".into())
}
fn increment(c: &mut BTreeMap<String, u64>, key: &str, v: u64) -> Result<(), String> {
    let slot = c.get_mut(key).ok_or("counter")?;
    *slot = slot.checked_add(v).ok_or("overflow")?;
    Ok(())
}
fn project(dense: &[u64], h: &[u64], l: u64) -> Result<([u64; 2], u64), String> {
    ensure(
        dense.len() == h.len() + 1 && dense.iter().chain(h).all(|&x| x < l),
        "independent projection shape/canonicality",
    )?;
    let (mut dot, mut nonzero) = (0, 0);
    for (&x, &y) in dense[..h.len()].iter().zip(h) {
        dot = add(dot, mul(x, y, l), l);
        if x != 0 && y != 0 {
            nonzero += 1;
        }
    }
    Ok(([dot, dense[h.len()]], nonzero))
}

pub fn pipeline(
    w: &Value,
    c: &driver::Record,
    collection_proof: &Value,
    answer: u64,
    max_residuals: u64,
    max_descents: usize,
) -> Result<Value, String> {
    let report = &w["work"];
    let p = c.public.p;
    let l = c.public.l;
    let columns = c.base.len();
    let sparse = (report["schema"] == "cover.quadratic-pair.dlp/v1"
        && c.mode == "grounded-quadratic-pairs")
        || (report["schema"] == "cover.discriminant-pair.dlp/v1"
            && c.mode == "discriminant-quadratic-pairs");
    ensure(
        sparse
            && report["p"] == p
            && report["l"] == l
            && report["seed"] == c.seed
            && report["collection_rank_ready"] == true,
        "pipeline registration",
    )?;
    ensure(
        report["fallback_solves"] == 0
            && report["resource_refusal"].is_null()
            && report["actual_scalar"] == answer
            && report["verified"] == true
            && report["capped"] == false,
        "actual recovery endpoint",
    )?;
    ensure(
        ["grounded-quadratic-pairs", "discriminant-quadratic-pairs"].contains(&c.mode.as_str())
            && report["options"]["b_count"] == json!((p as u128) * (p as u128))
            && report["options"]["max_descent_residuals"] == max_residuals
            && report["options"]["max_accepted_descents"] == max_descents,
        "runtime bounds",
    )?;
    ensure(
        report["options"]["max_split_records"] == 524288
            && report["options"]["graph"] == c.graph_options
            && report["options"]["split"] == c.split_options
            && report["options"]["sparse_rank"] == w["work"]["collection"]["rank_options"],
        "exact pair option binding",
    )?;
    ensure(
        report["rank_scope"]
            .as_str()
            .is_some_and(|s| s.contains("protected-target"))
            && report["solver_product_scope"]
                .as_str()
                .is_some_and(|s| s.contains("two-column")),
        "explicit rank/product scope",
    )?;
    let accounting = full::counter_check(w, p, l)?;
    let mut homogeneous = component::Rank::default();
    let mut full_rows = Vec::new();
    for row in &c.projections {
        let mut dense = vec![0; columns];
        for &(i, v) in &row.terms {
            dense[i] = v;
        }
        homogeneous.insert(dense, l)?;
        full_rows.push((
            row.terms
                .iter()
                .map(|&(i, v)| (i as u64, v))
                .collect::<Vec<_>>(),
            0,
        ));
    }
    ensure(
        homogeneous.basis.len() + 1 == columns,
        "homogeneous codimension one",
    )?;
    let free = (0..columns)
        .find(|i| !homogeneous.basis.contains_key(i))
        .ok_or("free coordinate")?;
    let mut h = vec![0; columns];
    h[free] = 1;
    let mut negations = 0;
    for (&i, row) in &homogeneous.basis {
        h[i] = if row[free] == 0 {
            0
        } else {
            negations += 1;
            l - row[free]
        };
    }
    let rank = homogeneous.basis.len() as u64;
    let cols = columns as u64;
    let initial_words = if sparse {
        ensure(free + 1 == columns, "sparse normalized free coordinate")?;
        negations = 0;
        cols + 4
    } else {
        4 * cols + 1 + 8 * rank + 4 * rank * cols
    };
    let initial = json!({"free_column":free,"kernel":h,"modulus":l,"counts":{"products":0,"additions":negations,"word_events":initial_words}});
    ensure(
        report["initial_basis"] == initial,
        "unique normalized kernel and initialization events",
    )?;
    let curve = &c.public;
    let spec = cover::spec_from_curve(
        p,
        E6(curve.alpha.map(E2)),
        l,
        curve.g.native(),
        curve.q.native(),
        0,
    )
    .ok_or("public transfer")?;
    let ctx = cover::Ctx::new(&spec);
    let jac = ctx.jac();
    let f = &ctx.f.f;
    let base: Vec<_> = c
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
    let descents = report["descents"].as_array().ok_or("descents")?;
    let affine = report["affine_rows"].as_array().ok_or("affine rows")?;
    let projected = report["projected_rows"]
        .as_array()
        .ok_or("projected rows")?;
    let residuals = n(report, "descent_residuals")?;
    ensure(
        !descents.is_empty()
            && descents.len() <= max_descents
            && descents.len() == affine.len()
            && descents.len() == projected.len()
            && residuals > 0
            && residuals <= max_residuals
            && n(report, "descent_stream_advances")? + 1 == residuals,
        "descent flow",
    )?;
    ensure(
        n(report, "descent_timeouts")? <= residuals
            && n(report, "descent_incomplete")? <= residuals,
        "descent outcome counters",
    )?;
    let _invalid_descents = n(report, "invalid_descents")?;
    let mut rng = StdRng::seed_from_u64(c.seed ^ 0x51E7E);
    let da = work::uniform_mod_u64(&mut rng, 1, l);
    let db = work::uniform_mod_u64(&mut rng, 1, l);
    let a0 = work::uniform_mod_u64(&mut rng, 0, l);
    let b0 = work::uniform_mod_u64(&mut rng, 1, l);
    let mut affine_rank = component::Rank::default();
    let mut affine_prefix = Vec::new();
    let (mut dots, mut previous_index) = (0u64, None);
    for (i, ((d, a), q)) in descents.iter().zip(affine).zip(projected).enumerate() {
        let index = n(d, "residual_index")?;
        let aa = n(d, "a")?;
        let bb = n(d, "b")?;
        ensure(
            index < residuals
                && previous_index.is_none_or(|old| old < index)
                && aa < l
                && bb > 0
                && bb < l
                && d["verified"] == true,
            "descent labels",
        )?;
        previous_index = Some(index);
        ensure(
            aa == add(a0, mul(index % l, da, l), l) && bb == add(b0, mul(index % l, db, l), l),
            "independent residual-stream coefficient binding",
        )?;
        let terms: Vec<(usize, i8)> = serde_json::from_value(d["decomposition"]["terms"].clone())
            .map_err(|_| "descent terms")?;
        ensure(
            terms.len() == 6
                && terms.windows(2).all(|w| w[0].0 < w[1].0)
                && terms
                    .iter()
                    .all(|&(i, s)| i < columns && [-1, 1].contains(&s)),
            "descent support",
        )?;
        let mut sum = jac.identity();
        let mut dense = vec![0; columns + 1];
        for &(j, sign) in &terms {
            let divisor = if sign == 1 {
                base[j].d.clone()
            } else {
                jac.neg(&base[j].d)
            };
            sum = jac.add(&sum, &divisor);
            dense[j] = driver::signed(sign, l);
        }
        let residual = jac.add(
            &jac.mul(&spec.gj, aa as u128),
            &jac.mul(&spec.qj, bb as u128),
        );
        ensure(sum == residual, "independent affine Cantor equation")?;
        dense[columns] = l - bb;
        ensure(
            a["coefficients"] == json!(dense) && a["rhs"] == aa,
            "protected target coefficient and affine RHS",
        )?;
        let (coefficients, row_dots) = project(&dense, &h, l)?;
        dots += row_dots;
        ensure(
            q["coefficients"] == json!(coefficients)
                && q["rhs"] == aa
                && q["projection_extra_counts"]
                    == json!({"products":row_dots,"additions":row_dots,"word_events":5*cols+4}),
            "actual projected row and known projection events",
        )?;
        affine_rank.insert(coefficients.to_vec(), l)?;
        let sparse = coefficients
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v != 0)
            .map(|(i, &v)| (i as u64, v))
            .collect::<Vec<_>>();
        affine_prefix.push((sparse, aa));
        let (actual_rank, target) = full::target_solution(&affine_prefix, 2, l)?;
        ensure(
            q["rank"] == actual_rank
                && actual_rank == affine_rank.basis.len()
                && q["products"] == affine_rank.products
                && q["normalization_products"] == (3 * actual_rank as u64)
                && q["subtractions"] == (affine_rank.products - 3 * actual_rank as u64)
                && q["candidate_scalar"] == json!(target),
            "every affine rank/product/target prefix",
        )?;
        if i + 1 < descents.len() {
            ensure(target.is_none(), "continued after determinate target")?;
        } else {
            ensure(
                target == Some(answer) && index + 1 == residuals,
                "actual final target",
            )?;
        }
        full_rows.push((
            dense
                .iter()
                .enumerate()
                .filter(|&(_, v)| *v != 0)
                .map(|(i, &v)| (i as u64, v))
                .collect(),
            aa,
        ));
    }
    let (full_rank, target) = full::target_solution(&full_rows, columns + 1, l)?;
    ensure(
        target == Some(answer)
            && report["last_rank"] == affine_rank.basis.len()
            && report["solver_modular_muls"] == affine_rank.products,
        "independent unreduced full-matrix target",
    )?;
    let extra = json!({"products":dots,"additions":negations+dots,"word_events":initial_words+(5*cols+4)*descents.len() as u64});
    ensure(
        report["final_extra_counts"] == extra,
        "cumulative kernel/projection events",
    )?;
    let mut expected = counts(phase(&c.work_native, "linear_algebra")?)?;
    increment(&mut expected, "modr_mul", dots + affine_rank.products)?;
    increment(
        &mut expected,
        "modr_add",
        negations + dots + descents.len() as u64 + affine_rank.products
            - 3 * affine_rank.basis.len() as u64,
    )?;
    increment(&mut expected, "modr_inv", affine_rank.basis.len() as u64)?;
    increment(
        &mut expected,
        "word_op",
        initial_words + (6 * cols + 20) * descents.len() as u64,
    )?;
    ensure(
        counts(phase(&w["work_native"], "linear_algebra")?)? == expected,
        "complete observed LA primitive conservation",
    )?;
    let ec = cover::EllE::new(&ctx.f, &spec.alpha);
    ensure(
        ec.mul(&spec.g, answer) == spec.q,
        "independent elliptic target replay",
    )?;
    Ok(
        json!({"mode":w["method"],"actual_scalar":answer,"collection":collection_proof,"accounting":accounting,"descents_checked":descents.len(),"residual_stream_checked":true,"unreduced_full_affine_rank":full_rank,"quotient_rank":affine_rank.basis.len(),"every_projected_prefix_checked":true,"kernel_and_projection_events":extra,"all_LA_primitives_conserved":true,"complete_cost":false,"total_bitops":null,"cost_ratios":null}),
    )
}

pub fn paired_pipeline(a: &Value, b: &Value) -> Result<Value, String> {
    for field in [
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
    ] {
        ensure(
            a["work"][field] == b["work"][field],
            &format!("paired pipeline identity {field}"),
        )?;
    }
    for name in component::accounting::PHASES {
        let ca = counts(phase(&a["work_native"], name)?)?;
        let cb = counts(phase(&b["work_native"], name)?)?;
        if name == "relations" {
            continue;
        }
        if name == "linear_algebra" {
            let old = counts(phase(&a["work"]["collection"]["work_native"], name)?)?;
            let new = counts(phase(&b["work"]["collection"]["work_native"], name)?)?;
            for key in KINDS {
                let left = ca[key]
                    .checked_sub(old[key])
                    .ok_or("baseline LA phase underflow")?;
                let right = cb[key]
                    .checked_sub(new[key])
                    .ok_or("candidate LA phase underflow")?;
                ensure(left == right, "paired post-collection LA equality")?;
            }
        } else {
            ensure(ca == cb, &format!("paired non-collection phase {name}"))?;
        }
    }
    Ok(
        json!({"descents_and_affine_rows_identical":true,"normalized_kernel_identical":true,"all_other_primitive_phases_identical":true,"post_collection_LA_primitives_identical":true}),
    )
}

pub fn rho_identity(
    w: &Value,
    negation: bool,
    batch: u64,
    p: u64,
    l: u64,
    seed: u64,
) -> Result<(), String> {
    let options = json!({"negation":negation,"batch":batch,"r":256,"dp_bits":16.min((l.ilog2()/2).saturating_sub(4))});
    let r = &w["work"]["run"];
    ensure(
        w["rho_options"] == options
            && w["recovered_scalar"] == w["actual_scalar"]
            && w["work"]["verified"] == true
            && r["correct"] == false
            && r["p"] == p
            && r["l"] == l
            && r["seed"] == seed
            && r["run"] == 0
            && r["capped"] == false
            && n(r, "steps")? > 0,
        "rho configuration/endpoint",
    )?;
    for key in ["negation", "batch", "r", "dp_bits"] {
        ensure(r[key] == options[key], "executed rho option")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_projection_and_affine_target_control() {
        let h = vec![99, 1];
        let mut rows = Vec::new();
        for (dense, rhs, expected) in [
            (vec![1, 1, 98], 2, [100, 98]),
            (vec![1, 0, 96], 11, [99, 96]),
        ] {
            let (row, events) = project(&dense, &h, 101).unwrap();
            assert_eq!(row, expected);
            assert_eq!(events, u64::from(dense[0] != 0) + u64::from(dense[1] != 0));
            rows.push((
                row.iter()
                    .enumerate()
                    .map(|(i, &v)| (i as u64, v))
                    .collect(),
                rhs,
            ));
        }
        assert_eq!(full::target_solution(&rows, 2, 101).unwrap(), (2, Some(7)));
        assert!(project(&[1, 2], &h, 101).is_err());
        assert!(project(&[1, 101, 0], &h, 101).is_err());
        assert!(project(&[1, 0, 0], &[101, 1], 101).is_err());
    }
    fn paired() -> (Value, Value) {
        let zero: Value = json!(KINDS
            .into_iter()
            .map(|k| (k, 0u64))
            .collect::<BTreeMap<_, _>>());
        let phases: Vec<_> = component::accounting::PHASES
            .iter()
            .map(|name| json!({"phase":name,"primitives":zero}))
            .collect();
        let a = json!({"work":{"descents":[],"collection":{"work_native":{"phases":phases}}},"work_native":{"phases":phases}});
        (a.clone(), a)
    }
    #[test]
    fn paired_conservation_refuses_extra_work_and_underflow() {
        let (a, b) = paired();
        assert!(paired_pipeline(&a, &b).is_ok());
        for name in component::accounting::PHASES
            .into_iter()
            .filter(|&x| x != "relations")
        {
            let mut bad = b.clone();
            let row = bad["work_native"]["phases"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|r| r["phase"] == name)
                .unwrap();
            row["primitives"]["word_op"] = json!(1);
            assert!(paired_pipeline(&a, &bad).is_err());
        }
        let (mut a, mut b) = paired();
        for w in [&mut a, &mut b] {
            w["work"]["collection"]["work_native"]["phases"][7]["primitives"]["modr_mul"] =
                json!(1);
        }
        assert!(paired_pipeline(&a, &b).is_err());
    }
    #[test]
    fn rho_registered_options_are_bound_to_execution() {
        for (negation, batch) in [(false, 1), (true, 1), (false, 16), (true, 16)] {
            let options = json!({"negation":negation,"batch":batch,"r":256,"dp_bits":0});
            let mut r = options.clone();
            for (k, v) in [
                ("correct", json!(false)),
                ("p", json!(101)),
                ("l", json!(101)),
                ("seed", json!(7)),
                ("run", json!(0)),
                ("capped", json!(false)),
                ("steps", json!(1)),
            ] {
                r[k] = v;
            }
            let w = json!({"rho_options":options,"actual_scalar":1,"recovered_scalar":1,"work":{"verified":true,"run":r}});
            assert!(rho_identity(&w, negation, batch, 101, 101, 7).is_ok());
            let mut bad = w;
            bad["work"]["run"]["batch"] = json!(2);
            assert!(rho_identity(&bad, negation, batch, 101, 101, 7).is_err());
        }
    }
}
