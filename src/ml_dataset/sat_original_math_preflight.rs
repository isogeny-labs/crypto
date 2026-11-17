//! Closed, allocation-free shape plan for bounded explicit-Legacy v2 replay.
//! The caller prepays the input-length scan. Saved counters/acceptance bits are
//! never authority: actual reconstruction enforces coordinate/lift cutoffs.
use crate::ml_dataset::factor_bases::ReplayBounds;
use crate::ml_dataset::types::{CaseConfig, FactorBasePolicy};
use serde_json::Value;
use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    UnsupportedPolicy,
    UnsupportedMembership,
    UnsupportedCertificate,
    Shape,
    Overflow,
    ActiveTally,
}
pub(super) struct Plan {
    pub work: u64,
    pub bytes: u64,
    pub scalar_steps: u64,
    pub scalar_calls: u64,
    pub primality_calls: u64,
    pub generated_wire_bound: u64,
    pub generated_owner_bytes: u64,
    pub bounds: ReplayBounds,
}
fn add(a: u64, b: u64) -> Result<u64, Error> {
    a.checked_add(b).ok_or(Error::Overflow)
}
fn mul(a: u64, b: u64) -> Result<u64, Error> {
    a.checked_mul(b).ok_or(Error::Overflow)
}
fn number(v: &Value) -> Result<u64, Error> {
    let s = v.as_str().ok_or(Error::Shape)?;
    if s.len() > 20 {
        return Err(Error::Shape);
    }
    s.parse().map_err(|_| Error::Shape)
}
fn array(v: &Value) -> Result<&[Value], Error> {
    v.as_array().map(Vec::as_slice).ok_or(Error::Shape)
}
#[derive(Default)]
struct Scalars {
    steps: u64,
    calls: u64,
    factors: u64,
}
impl Scalars {
    fn scalar(&mut self, k: u64) -> Result<(), Error> {
        self.calls = add(self.calls, 1)?;
        self.steps = add(
            self.steps,
            u64::from(64 - k.leading_zeros() + k.count_ones()),
        )?;
        Ok(())
    }
}
fn certificate(v: &Value, twist: bool, s: &mut Scalars) -> Result<(), Error> {
    match v["method"].as_str() {
        Some("exact-point-order-unique-hasse-multiple-v1") => {
            let order = number(&v["point_order"])?;
            if order == 0 {
                return Err(Error::Shape);
            }
            s.scalar(order)?;
            let fs = array(&v["point_order_factorization"])?;
            if fs.len() > 64 {
                return Err(Error::Shape);
            }
            s.factors = add(s.factors, fs.len() as u64)?;
            for row in fs {
                let p = number(&row["prime"])?;
                if p < 2 {
                    return Err(Error::Shape);
                }
                s.scalar(order / p)?;
            }
            Ok(())
        }
        Some("koblitz-frobenius-trace-recurrence-v1") if !twist => Ok(()),
        Some("quadratic-twist-order-v1") if twist => certificate(&v["base_certificate"], false, s),
        _ => Err(Error::UnsupportedCertificate),
    }
}
pub(super) fn plan(
    config: &CaseConfig,
    public: &Value,
    base: &Value,
    deadline: Instant,
) -> Result<Plan, Error> {
    if !matches!(
        &config.factor_base_policy,
        Some(FactorBasePolicy::Legacy {})
    ) || base["schema"] != "ml-ecdlp-factor-base/v2"
    {
        return Err(Error::UnsupportedPolicy);
    }
    if !matches!(config.family.as_str(), "binary" | "koblitz")
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
        || !(2..=15).contains(&config.m)
    {
        return Err(Error::Shape);
    }
    if crate::binary_ecc::operation_tally::current_thread_active_or_borrowed() {
        return Err(Error::ActiveTally);
    }
    let n = config.n as u64;
    let q = 1u64 << config.n;
    if public["n"].as_u64() != Some(n) || public["twist"].as_bool() != Some(config.twist) {
        return Err(Error::Shape);
    }
    let r = number(&public["curve"]["subgroup_order"])?;
    let order = number(&public["curve"]["group_order"])?;
    let h = number(&public["curve"]["cofactor"])?;
    if r <= 2 || order == 0 || order % r != 0 || order / r != h {
        return Err(Error::Shape);
    }
    if h % r == 0 {
        return Err(Error::UnsupportedMembership);
    }
    let requested = u64::try_from(config.factor_base_size).map_err(|_| Error::Shape)?;
    if !(1..=4096).contains(&requested) || base["requested_size"].as_u64() != Some(requested) {
        return Err(Error::Shape);
    }
    let units = add(mul(requested, 32)?, 128)?
        .min(q)
        .checked_sub(1)
        .ok_or(Error::Shape)?;
    let candidates = array(&base["construction_candidates"])?;
    if candidates.len() as u64 > units {
        return Err(Error::Shape);
    }
    let (mut coordinates, mut lifts) = (0u64, 0u64);
    for unit in candidates {
        let rows = array(&unit["coordinates"])?;
        if rows.len() != 1 {
            return Err(Error::Shape);
        }
        coordinates = add(coordinates, 1)?;
        let ls = array(&rows[0]["lifts"])?;
        if ls.len() > 2 {
            return Err(Error::Shape);
        }
        lifts = add(lifts, ls.len() as u64)?;
    }
    let k = public["symmetry"]["frobenius_power"]
        .as_u64()
        .ok_or(Error::Shape)?;
    if k == 0 || k > n || n % k != 0 {
        return Err(Error::Shape);
    }
    let groups = array(&public["curve"]["group_order_factorization"])?;
    if groups.len() > 64 {
        return Err(Error::Shape);
    }
    let mut s = Scalars::default();
    s.factors = groups.len() as u64;
    certificate(&public["curve"]["order_certificate"], config.twist, &mut s)?;
    s.scalar(r)?;
    s.scalar(r)?; // original generator and target, once
    let mut clearing = order;
    let mut multiplicity = 0u64;
    while clearing != 0 && clearing % r == 0 {
        clearing /= r;
        multiplicity += 1;
    }
    // Every admitted lift has an independently bounded projection and membership
    // scalar replay; early identity return can only reduce actual operations.
    let mut per = Scalars::default();
    per.scalar(clearing)?;
    for _ in 1..multiplicity {
        per.scalar(r)?;
    }
    per.scalar(r)?;
    s.steps = add(s.steps, mul(per.steps, lifts)?)?;
    s.calls = add(s.calls, mul(per.calls, lifts)?)?;
    let n2 = mul(n, n)?;
    let n3 = mul(n2, n)?;
    // Preserve the inherited coarse 128*n^3 arithmetic-call tariff. Exact
    // scalar bit steps remain evidence; no new per-step work rate is inferred.
    let scalar_work = mul(s.calls, mul(128, n3)?)?;
    // Three independently checked field constructions and public twist trace;
    // each actual coordinate lift retains the original128*n^3 block rate.
    let setup_and_lifts = mul(add(4, coordinates)?, mul(128, n3)?)?;
    let points = lifts.min(add(requested, 1)?).min(4096);
    let orbit_steps = mul(points, n / k)?;
    let sym_work = add(
        mul(64, n3)?,
        mul(add(mul(points, 2)?, orbit_steps)?, mul(32, n2)?)?,
    )?;
    let metadata_work = add(
        mul(add(s.factors, 1)?, 8192)?,
        add(mul(units, 32)?, mul(add(coordinates, points)?, 4096)?)?,
    )?;
    let work = add(
        add(scalar_work, setup_and_lifts)?,
        add(sym_work, metadata_work)?,
    )?;
    // Inherited requested-object/word convention from reference_add_requested_bytes:
    // full inline field layouts, inversion Vec requests and fixed stack scratch.
    let field_layout = std::mem::size_of::<crate::binary_ecc::F2mElement>() as u64;
    let big_layout = std::mem::size_of::<num_bigint::BigUint>() as u64;
    let step_bytes = add(
        add(mul(64, add(field_layout, 8)?)?, 48 + 160)?,
        mul(4, add(big_layout, 16)?)?,
    )?;
    // Five growing u32 vectors: polynomial, a,b and up to two input coordinates.
    // Charge 4*n slots cumulatively per vector, including superseded capacities.
    let call_bytes = add(
        mul(80, n)?,
        add(
            (std::mem::size_of::<crate::binary_ecc::BinaryCurve>()
                + 4 * std::mem::size_of::<crate::binary_ecc::BinaryPoint>()
                + 5 * std::mem::size_of::<Vec<u32>>()) as u64,
            add(
                mul(8, field_layout)?,
                add(mul(3, add(big_layout, 32)?)?, 256)?,
            )?,
        )?,
    )?;
    // Schema-bound serialized content; all decimal words <=20 digits. Reserve
    // the existing128-byte JSON owner tariff for TWO materializations. Generated
    // point/lift/row/unit/orbit/counter trees now move into parents; the second
    // pass conservatively covers placeholders and bounded scalar-text copies.
    let generated_wire_bound = add(
        4096,
        add(
            mul(coordinates, 512)?,
            add(
                mul(lifts, 1024)?,
                add(mul(points, 128)?, mul(orbit_steps, 128)?)?,
            )?,
        )?,
    )?;
    let generated_owner_bytes = mul(generated_wire_bound, 128 * 2)?;
    // Three field parses, two portable Gf2 tables plus one possible clone.
    // Include vector and box conversion requests and all power arrays.
    let table_positions = (n - 1).div_ceil(8);
    let field_setup_bytes = add(32768, mul(table_positions, 3 * 2 * 256 * 8 + 2 * 8 * 8)?)?;
    // Even lifting: cumulative pivot/sort and bit-position owners. The full
    // field-layout envelope includes trace, basis construction and result points.
    let coordinate_bytes = add(
        mul(16, n2)?,
        add(
            mul(160, n)?,
            add(mul(add(mul(8, n)?, 128)?, field_layout)?, 8192)?,
        )?,
    )?;
    let bytes = [
        field_setup_bytes,
        mul(s.steps, step_bytes)?,
        mul(s.calls, call_bytes)?,
        mul(units, 64)?,
        mul(coordinates, coordinate_bytes)?,
        generated_owner_bytes,
        mul(points, 4096)?,
        mul(orbit_steps, 1024)?,
    ]
    .into_iter()
    .try_fold(0, add)?;
    Ok(Plan {
        work,
        bytes,
        scalar_steps: s.steps,
        scalar_calls: s.calls,
        primality_calls: add(s.factors, 1)?,
        generated_wire_bound,
        generated_owner_bytes,
        bounds: ReplayBounds {
            coordinates,
            lifts,
            units,
            deadline,
        },
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_schedule_matches_actual_binary_loop() {
        for k in [0, 1, 2, 3, 379, 22144, u64::MAX] {
            let mut s = Scalars::default();
            s.scalar(k).unwrap();
            let mut exact = 0;
            for i in (0..(64 - k.leading_zeros())).rev() {
                exact += 1;
                if (k >> i) & 1 == 1 {
                    exact += 1;
                }
            }
            assert_eq!(s.steps, exact);
        }
    }
}

#[cfg(test)]
mod actual_original_tests {
    use super::*;
    #[test]
    fn bounded_actual_legacy_originals_match_full_reconstruction_both_twists() {
        use crate::ml_dataset::{curves, solvers};
        for twist in [false, true] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, twist, "sat",
            );
            crate::ml_dataset::factor_bases::assert_move_materialization_eq(&f.config, &f.case);
            let public = &f.case.public_instance;
            let base = &f.case.factor_base;
            let p = plan(
                &f.config,
                public,
                base,
                Instant::now() + std::time::Duration::from_secs(120),
            )
            .unwrap();
            assert!(
                p.work <= 64 * 1024 * 1024,
                "math alone exceeds historical work allowance: {}",
                p.work
            );
            assert!(
                p.bytes <= 16 * 1024 * 1024,
                "math alone exceeds unchanged historical byte allowance: {}",
                p.bytes
            );
            assert!(p.generated_owner_bytes >= p.generated_wire_bound * 128);
            assert!(p.primality_calls >= 1);
            assert!(
                serde_json::to_vec(base).unwrap().len() as u64 <= p.generated_wire_bound,
                "genuine generated schema exceeds declared content envelope"
            );
            curves::verify_factor_base_config_bounded(&f.config, public, base, p.bounds)
                .expect("required genuine bounded original mathematics");
            let mut short = p.bounds;
            short.coordinates = 0;
            assert!(
                curves::verify_factor_base_config_bounded(&f.config, public, base, short).is_err()
            );
            short = p.bounds;
            short.lifts = 0;
            assert!(
                curves::verify_factor_base_config_bounded(&f.config, public, base, short).is_err()
            );
            short = p.bounds;
            short.deadline = Instant::now();
            assert!(
                curves::verify_factor_base_config_bounded(&f.config, public, base, short).is_err()
            );
            let mut changed = base.clone();
            changed["construction_counters"]["candidate_coordinates"] = serde_json::json!(999);
            assert!(
                curves::verify_factor_base_config_bounded(&f.config, public, &changed, p.bounds)
                    .is_err(),
                "saved counters are not reconstruction authority"
            );
            let mut nc = public.clone();
            let r = number(&public["curve"]["subgroup_order"]).unwrap();
            nc["curve"]["cofactor"] = serde_json::json!(r.to_string());
            nc["curve"]["group_order"] = serde_json::json!((r * r).to_string());
            assert!(matches!(
                plan(&f.config, &nc, base, p.bounds.deadline),
                Err(Error::UnsupportedMembership)
            ));
        }
    }
}
