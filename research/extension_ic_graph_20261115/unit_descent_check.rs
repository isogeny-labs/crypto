//! Independent substitution arithmetic; shared baseline constructor binds inputs.
//! No production reduction, F4 solve, or production lift is invoked here.
use crate::pair_driver as driver;
use crypto_lib::cryptanalysis::{
    jv_bitops as work,
    jv_cover::{self as cover, E2, E6},
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
type Poly = Vec<(Vec<u32>, u64)>;
fn require(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n > 0 {
        if n & 1 == 1 {
            b = mul(b, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    b
}
fn collect(terms: Poly, p: u64) -> Poly {
    let mut m = BTreeMap::<Vec<u32>, u64>::new();
    for (e, c) in terms {
        let a = m.entry(e).or_default();
        *a = (*a + c) % p;
    }
    m.into_iter().filter(|(_, c)| *c != 0).collect()
}
fn normalize(terms: Poly, p: u64) -> Poly {
    let mut a = collect(terms, p);
    a.sort_by(|(x, _), (y, _)| {
        y.iter()
            .sum::<u32>()
            .cmp(&x.iter().sum::<u32>())
            .then_with(|| x.iter().rev().cmp(y.iter().rev()))
    });
    if let Some((_, c)) = a.first() {
        let inv = power(*c, p - 2, p);
        for (_, v) in &mut a {
            *v = mul(*v, inv, p);
        }
    }
    a
}
fn eval(f: &Poly, x: &[u64], p: u64) -> u64 {
    f.iter().fold(0, |s, (e, c)| {
        (s + e
            .iter()
            .zip(x)
            .fold(*c, |t, (n, a)| mul(t, power(*a, *n as u64, p), p)))
            % p
    })
}
pub fn system(a: &Value, p: u64) -> Result<(usize, usize), String> {
    let original: Vec<Poly> =
        serde_json::from_value(a["original"].clone()).map_err(|_| "original quadrics")?;
    require(
        original.len() == 6
            && original.iter().all(|f| {
                f.len() <= 28
                    && f.iter().all(|(e, c)| {
                        e.len() == 6 && e.iter().sum::<u32>() <= 2 && *c > 0 && *c < p
                    })
            }),
        "six canonical quadrics",
    )?;
    let mut pivot = 0;
    let mut rest = vec![];
    for (e, c) in &original[5] {
        if e[5] == 0 {
            rest.push((e[..5].to_vec(), *c));
        } else {
            require(
                e[5] == 1 && e[..5].iter().all(|x| *x == 0),
                "constant-unit pivot",
            )?;
            pivot = (pivot + c) % p;
        }
    }
    require(pivot != 0, "nonzero pivot")?;
    let inv = power(pivot, p - 2, p);
    let replacement = collect(
        rest.into_iter()
            .map(|(e, c)| (e, mul(p - c, inv, p)))
            .collect(),
        p,
    );
    require(
        a["replacement"] == json!(replacement),
        "scalar-preserving replacement",
    )?;
    let mut square = vec![];
    for (x, c) in &replacement {
        for (y, d) in &replacement {
            square.push((
                x.iter().zip(y).map(|(a, b)| a + b).collect(),
                mul(*c, *d, p),
            ));
        }
    }
    let powers = [
        vec![(vec![0; 5], 1)],
        replacement.clone(),
        collect(square, p),
    ];
    let mut reduced = vec![];
    for f in &original[..5] {
        let mut terms = vec![];
        for (e, c) in f {
            for (x, d) in &powers[e[5] as usize] {
                terms.push((
                    e[..5].iter().zip(x).map(|(a, b)| a + b).collect(),
                    mul(*c, *d, p),
                ));
            }
        }
        let q = normalize(terms, p);
        require(
            q.iter().all(|(e, _)| e.iter().sum::<u32>() <= 4),
            "quartic bound",
        )?;
        if !q.is_empty() {
            reduced.push(q);
        }
    }
    let sha = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&reduced).map_err(|_| "reduced encoding")?)
    );
    require(
        a["reduced_sha256"] == sha,
        "independent reduced system digest",
    )?;
    let roots: Vec<Vec<u64>> =
        serde_json::from_value(a["lifted_roots"].clone()).map_err(|_| "lifted roots")?;
    let mut unique = BTreeSet::new();
    for x in &roots {
        require(
            x.len() == 6 && x.iter().all(|v| *v < p),
            "canonical lifted root",
        )?;
        require(unique.insert(x.clone()), "duplicate lifted root")?;
        require(
            eval(&replacement, &x[..5], p) == x[5]
                && reduced.iter().all(|q| eval(q, &x[..5], p) == 0)
                && original.iter().all(|q| eval(q, x, p) == 0),
            "independent six-equation lift",
        )?;
    }
    Ok((reduced.len(), roots.len()))
}
pub fn qualify(
    w: &Value,
    c: &driver::Record,
    baseline: &Value,
    adapted: &Value,
) -> Result<Value, String> {
    require(
        w["work"]["nagao_backend"] == "exact-unit-five"
            && adapted["work"]["nagao_backend"] == "adapted-six",
        "backend identities",
    )?;
    // Exact collection comparison; the sole removed field is a recorded timer.
    for other in [baseline, adapted] {
        let a = other["work"]["collection"]
            .as_object()
            .ok_or("collection")?;
        let b = w["work"]["collection"].as_object().ok_or("collection")?;
        require(a.keys().eq(b.keys()), "collection field identities")?;
        for (key, value) in a {
            if key != "rank_checks" {
                require(
                    Some(value) == b.get(key),
                    &format!("matched collection {key}"),
                )?;
            }
        }
        let mut a = a["rank_checks"].clone();
        let mut b = b["rank_checks"].clone();
        for v in [&mut a, &mut b] {
            for r in v.as_array_mut().ok_or("rank checks")? {
                r["native"]
                    .as_object_mut()
                    .ok_or("rank native record")?
                    .remove("inner_wall_ns");
            }
        }
        require(
            a == b,
            "all collection geometry, RNG histories and primitive phases identical",
        )?;
        for key in [
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
            require(
                other["work"][key] == w["work"][key],
                &format!("matched descent/affine {key}"),
            )?;
        }
        for phase in other["work_native"]["phases"].as_array().ok_or("phases")? {
            if phase["phase"] == "descent" {
                continue;
            }
            let q = w["work_native"]["phases"]
                .as_array()
                .ok_or("phases")?
                .iter()
                .find(|v| v["phase"] == phase["phase"])
                .ok_or("phase")?;
            require(q == phase, "all non-descent primitive phases identical")?;
        }
    }
    let curve = &c.public;
    let spec = cover::spec_from_curve(
        curve.p,
        E6(curve.alpha.map(E2)),
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
    require(
        !attempts.is_empty()
            && attempts.len() <= 8192
            && json!(attempts.len()) == w["work"]["descent_residuals"],
        "complete bounded attempt history",
    )?;
    let (mut roots, mut empty, mut degenerate, mut timeouts, mut incomplete) = (0, 0, 0, 0, 0);
    for (i, t) in attempts.iter().enumerate() {
        let encode = |x: &[E2]| x.iter().map(|v| v.0).collect::<Vec<_>>();
        require(
            t["residual_index"] == json!(i)
                && t["u"] == json!(encode(&r.u))
                && t["v"] == json!(encode(&r.v)),
            "every residual input binding",
        )?;
        if r.u.len() != 4 {
            require(t["audit"].is_null(), "degenerate refusal")?;
            degenerate += 1;
        } else {
            let mut v = r.v.clone();
            v.resize(3, E2::ZERO);
            let input =
                cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &r.u, &v).polys(&ctx.reim, curve.p);
            require(
                t["audit"]["original"] == json!(input),
                "shared baseline Weil constructor input binding",
            )?;
            let (_, n) = system(&t["audit"], curve.p)?;
            roots += n;
            empty += usize::from(n == 0);
            timeouts += usize::from(t["audit"]["stats"]["timed_out"] == true);
            incomplete += usize::from(t["audit"]["stats"]["incomplete"] == true);
        }
        if i + 1 < attempts.len() {
            r = jac.add(&r, &step);
        }
    }
    require(
        w["work"]["descent_timeouts"] == json!(timeouts)
            && w["work"]["descent_incomplete"] == json!(incomplete),
        "all cap counters",
    )?;
    Ok(
        json!({"attempts":attempts.len(),"degenerate":degenerate,"empty_coordinate_sets":empty,"lifted_roots":roots,
        "timed_out":timeouts,"incomplete":incomplete,"every_input_bound":true,"independent_substitution_and_lifting":true,
        "input_constructor":"shared unchanged baseline Weil constructor; not an independent geometry implementation",
        "six_variable_adapter_control":true,"complete_cost":false,"rho_parity":false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_normalization_and_scalar_replacement_mutations() {
        let p = 7;
        let mut f = vec![vec![(vec![0, 0, 0, 0, 0, 0], 1)]; 5];
        f.push(vec![
            (vec![0, 0, 0, 0, 0, 1], 2),
            (vec![0, 0, 0, 0, 0, 0], 3),
        ]);
        let reduced: Vec<Poly> = vec![vec![(vec![0; 5], 1)]; 5];
        let sha = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&reduced).unwrap())
        );
        let a = json!({"original":f,"replacement":[[vec![0;5],2]],"reduced_sha256":sha,"lifted_roots":[]});
        assert_eq!(system(&a, p).unwrap(), (5, 0));
        for key in ["replacement", "reduced_sha256", "original"] {
            let mut bad = a.clone();
            bad[key] = Value::Null;
            assert!(system(&bad, p).is_err());
        }
        let mut bad = a.clone();
        bad["lifted_roots"] = json!([vec![0; 6]]);
        assert!(system(&bad, p).is_err());
        assert_eq!(
            normalize(vec![(vec![0, 2], 2), (vec![2, 0], 4)], 7),
            vec![(vec![2, 0], 1), (vec![0, 2], 4)]
        );
    }
}
