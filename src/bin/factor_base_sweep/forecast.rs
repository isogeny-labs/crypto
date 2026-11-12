//! Degree-of-regularity forecast glue.  The live worker and the offline
//! `triage` command build the forecaster from the sweep's own records
//! (`*.system.json`, `*.result.json`, `base.json`, `request.json`) through
//! the functions here, so both compute the same forecast from the same
//! bytes.  Nothing here changes what a sweep does: the forecast is written
//! beside the measurement and the candidate keeps its frozen budget.
use super::model::{read, Request, Result};
use crypto_lib::cryptanalysis::{
    dreg_forecast::{Advice, Forecast, Forecaster, Outcome, SolveObservation, SystemShape},
    dreg_prior_table,
    ecbench::{methods::FactorBaseDump, workload::CurveSpec},
    f4_fp::Poly,
    ic_boundary::{generic_floor_ops, IncrementalGauss},
};
use serde_json::{json, Value};
use std::path::Path;

/// Order the engine runs the phases in; records are replayed in it.
pub const PHASES: [&str; 3] = ["positive_control", "train", "holdout"];

fn is_field_equation(f: &Poly) -> bool {
    if f.len() != 2 {
        return false;
    }
    let support = |m: &[u32]| {
        let nz: Vec<(usize, u32)> = m
            .iter()
            .enumerate()
            .filter(|(_, e)| **e > 0)
            .map(|(i, e)| (i, *e))
            .collect();
        (nz.len() == 1).then(|| nz[0])
    };
    match (support(&f[0].0), support(&f[1].0)) {
        (Some((i, a)), Some((j, b))) => i == j && ((a, b) == (2, 1) || (a, b) == (1, 2)),
        _ => false,
    }
}

/// The shape of one system: variables, equation total degrees without the
/// Boolean field equations, and whether the ring is Boolean (`p = 2`).
pub fn shape(equations: &[Poly], variables: usize, modulus: u64) -> SystemShape {
    let boolean = modulus == 2;
    SystemShape {
        variables,
        equation_degrees: equations
            .iter()
            .filter(|f| !(boolean && is_field_equation(f)))
            .filter_map(|f| f.iter().map(|(m, _)| m.iter().sum::<u32>()).max())
            .filter(|d| *d > 0)
            .collect(),
        boolean,
    }
}

fn shape_of_system(system: &Value) -> Option<SystemShape> {
    let equations: Vec<Poly> = serde_json::from_value(system["equations"].clone()).ok()?;
    Some(shape(
        &equations,
        system["variables"].as_u64()? as usize,
        system["coefficient_modulus"].as_u64()?,
    ))
}

/// Automorphism fold of the strongest generic reference for the curve:
/// negation and the Frobenius orbit on a Koblitz curve, negation elsewhere.
pub fn automorphisms(curve: &CurveSpec) -> f64 {
    match curve {
        CurveSpec::Koblitz { n, .. } => 2.0 * f64::from(*n),
        _ => 2.0,
    }
}

/// A forecaster for one candidate, before any solve.
pub fn forecaster(request: &Request, dump: &FactorBaseDump, r: u64, cofactor: u64) -> Forecaster {
    let mut f = Forecaster::new(request.plan.max_degree)
        .with_work_unit("F4 row-reduction field multiplications (f4_fp field_ops)");
    f.set_relation_state(dump.factor_base.columns as usize, 0);
    f.set_linear_algebra_unit(
        "relation-matrix multiply-subtracts mod r (IncrementalGauss row_ops)",
    );
    f.set_rho_floor_group_additions(generic_floor_ops(
        r as f64,
        automorphisms(&request.candidate.curve),
    ));
    f.set_inventory(dump.points.len(), r, cofactor);
    // Calibrated prior point for the cells the table covers (Boolean, m = 2,
    // binary subspace shapes); everywhere else the semi-regular prior stays.
    if let (Some(shape), Some(n)) = (
        request.candidate.shape.as_deref(),
        binary_degree(&request.candidate.curve),
    ) {
        if let Some(d) =
            dreg_prior_table::calibrated_prior_degree(n, request.candidate.dimension, shape)
        {
            f.set_prior_degree(d, dreg_prior_table::SOURCE);
        }
    }
    f
}

/// The extension degree of a binary curve, or `None` for a prime-field curve.
fn binary_degree(curve: &CurveSpec) -> Option<u32> {
    match curve {
        CurveSpec::Koblitz { n, .. } | CurveSpec::BinaryRandom { n, .. } => Some(*n),
        _ => None,
    }
}

/// One trial record as a forecaster observation.  The positive control is
/// a planted target and is excluded; `no_point_decomposition` (algebraic
/// roots, no point witness) is a decided solve without a relation and is
/// filed with the refutations.
pub fn observation(record: &Value) -> Option<SolveObservation> {
    let phase = record["phase"].as_str()?;
    if phase == "positive_control" {
        return None;
    }
    let outcome = match record["status"].as_str()? {
        "verified_decomposition" => Outcome::Decomposed,
        "refuted" | "no_point_decomposition" => Outcome::Refuted,
        "exhausted" => Outcome::Exhausted,
        _ => return None,
    };
    let degree_reached = record["degree_processed"].as_u64().unwrap_or(0) as u32;
    Some(SolveObservation {
        outcome,
        solving_degree: record["observed_solving_degree"].as_u64().map(|d| d as u32),
        censored_above: SolveObservation::censor_point(
            degree_reached,
            record["timed_out"].as_bool().unwrap_or(false),
        ),
        work: record["field_reduction_multiplications"]
            .as_u64()
            .unwrap_or(0),
        relation_phase: phase == "train",
        relation: record["relation"]["rank_gain"].as_u64().unwrap_or(0) > 0,
    })
}

/// The few numbers a reader triages on; the full forecast is in the result.
pub fn compact(f: &Forecast) -> Value {
    let hope: Vec<Value> = f
        .cost
        .hope_ratio_band
        .iter()
        .map(|h| json!({"multiplications_per_group_addition":h[0],"rho_floor_over_projected_relation_phase":h[1]}))
        .collect();
    json!({
        "schema": f.schema,
        "advice": f.advice,
        "advice_reasons": f.advice_reasons,
        "solves": f.solves,
        "decided": f.decided,
        "censored": f.censored,
        "relations": f.relations,
        "degree_bound": f.degree_bound,
        "semi_regular_degree": f.semi_regular_degree,
        "predicted_degree_of_regularity": f.all_targets.predicted_degree_label,
        "band_95": f.all_targets.band,
        "predicted_degree_decomposable_targets": f.decomposable_targets.predicted_degree_label,
        "p_exceeds_bound": f.all_targets.p_exceeds_bound,
        "excess_over_semi_regular": f.excess_over_semi_regular,
        "structural_gain": f.structural_gain,
        "relation_yield": f.cost.relation_yield,
        "expected_work_per_solve": f.cost.expected_work_per_solve,
        "remaining_relations": f.cost.remaining_relations,
        "projected_remaining_work": f.cost.projected_remaining_work,
        "projected_relation_phase_work": f.cost.projected_relation_phase_work,
        "work_unit": f.cost.unit,
        "hope_ratio_band": hope,
        "linear_algebra_observed_work": f.cost.linear_algebra.as_ref().map(|la| la.observed_work),
        "linear_algebra_unit": f.cost.linear_algebra.as_ref().map(|la| la.unit.clone()),
        "projected_linear_algebra_work": f.cost.linear_algebra.as_ref().and_then(|la| la.projected_phase_work),
        "projected_relation_phase_work_with_linear_algebra": f.cost.projected_relation_phase_work_with_linear_algebra,
        "hope_ratio_band_with_linear_algebra": f.cost.hope_ratio_band_with_linear_algebra.iter().map(|h| json!({"multiplications_per_group_addition":h[0],"rho_floor_over_projected_relation_phase":h[1]})).collect::<Vec<_>>(),
        "relation_degree_pearson_r": f.relation_trend.pearson_r,
        "status": f.status,
        "advisory": f.advisory,
        "certified_degree_of_regularity": Value::Null,
    })
}

/// Rank for the triage table: promising first, hopeless last.
pub fn advice_rank(a: Advice) -> u8 {
    match a {
        Advice::Promising => 0,
        Advice::Undecided => 1,
        Advice::Doubtful => 2,
        Advice::HopelessAtBound => 3,
    }
}

/// The forecast of one attempt directory, complete or interrupted,
/// recomputed from its records exactly as the worker computed it.  Returns
/// `Ok(None)` when the attempt has no inventory yet.
pub fn from_attempt(dir: &Path) -> Result<Option<(Forecast, Value)>> {
    let request: Request = read(&dir.join("request.json"))?;
    if !dir.join("base.json").is_file() {
        return Ok(None);
    }
    let dump: FactorBaseDump = read(&dir.join("base.json"))?;
    let (r, cofactor) = match request.candidate.curve.build()? {
        crypto_lib::cryptanalysis::ecbench::workload::Instance::Prime(i) => (i.r, i.cofactor),
        crypto_lib::cryptanalysis::ecbench::workload::Instance::Binary(i) => (i.r, i.cofactor),
        _ => return Err("forecast replay supports native prime/binary instances only".into()),
    };
    let mut f = forecaster(&request, &dump, r, cofactor);
    let mut rows = IncrementalGauss::new(dump.factor_base.columns as usize, r);
    let mut records = Vec::new();
    let mut admitted_rows = 0usize;
    let mut unreadable = 0usize;
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some(stem) = name.strip_suffix(".result.json") else {
            continue;
        };
        let Some((phase, index)) = stem.rsplit_once('-') else {
            continue;
        };
        let Some(order) = PHASES.iter().position(|p| *p == phase) else {
            continue;
        };
        let Ok(index) = index.parse::<u64>() else {
            continue;
        };
        match read::<Value>(&path) {
            Ok(record) => records.push((order, index, record)),
            // A record being written by a live worker is skipped, never guessed.
            Err(_) => unreadable += 1,
        }
    }
    records.sort_by_key(|(order, index, _)| (*order, *index));
    for (order, index, record) in &records {
        let system = dir.join(format!("{}-{index:04}.system.json", PHASES[*order]));
        if let Some(shape) = read::<Value>(&system)
            .ok()
            .as_ref()
            .and_then(shape_of_system)
        {
            f.set_shape(shape);
        }
        if let Some(relation) = record.get("relation").filter(|v| !v.is_null()) {
            let row: Vec<u64> =
                serde_json::from_value(relation["row"].clone()).map_err(|e| e.to_string())?;
            let rhs = relation["rhs"].as_u64().ok_or("relation without rhs")?;
            rows.add_row(row, rhs);
            admitted_rows += 1;
            f.observe_linear_algebra(admitted_rows, rows.row_ops);
        }
        f.set_relation_state(rows.columns(), rows.rank());
        if let Some(observation) = observation(record) {
            f.observe(observation);
        }
    }
    let forecast = f.forecast();
    let mut view = compact(&forecast);
    view["records_read"] = json!(records.len());
    view["records_unreadable"] = json!(unreadable);
    view["training_rank"] = json!(rows.rank());
    view["columns"] = json!(rows.columns());
    Ok(Some((forecast, view)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn poly(terms: &[(&[u32], u64)]) -> Poly {
        terms.iter().map(|(m, c)| (m.to_vec(), *c)).collect()
    }

    #[test]
    fn boolean_field_equations_are_left_out_of_the_shape() {
        let eqs = vec![
            poly(&[(&[1, 1, 0, 0], 1), (&[1, 0, 0, 0], 1), (&[0, 0, 0, 0], 1)]),
            poly(&[(&[0, 1, 1, 0], 1), (&[0, 0, 0, 1], 1)]),
            poly(&[(&[2, 0, 0, 0], 1), (&[1, 0, 0, 0], 1)]),
            poly(&[(&[0, 0, 0, 1], 1), (&[0, 0, 0, 2], 1)]),
        ];
        let s = shape(&eqs, 4, 2);
        assert_eq!(s.equation_degrees, vec![2, 2]);
        assert!(s.boolean);
        // Over a prime field the same two-term polynomial is an equation.
        let s = shape(&eqs, 4, 1009);
        assert_eq!(s.equation_degrees, vec![2, 2, 2, 2]);
        assert!(!s.boolean);
        // A membership polynomial keeps its full degree.
        let s = shape(&[poly(&[(&[5, 0], 1), (&[0, 0], 3)])], 2, 127);
        assert_eq!(s.equation_degrees, vec![5]);
    }

    #[test]
    fn records_map_to_observations_and_the_control_is_excluded() {
        let verified = json!({"phase":"train","status":"verified_decomposition","observed_solving_degree":3,"degree_processed":3,"timed_out":false,"field_reduction_multiplications":420,"relation":{"rank_gain":1}});
        let o = observation(&verified).unwrap();
        assert_eq!(o.outcome, Outcome::Decomposed);
        assert_eq!(o.solving_degree, Some(3));
        assert_eq!(o.work, 420);
        assert!(o.relation_phase && o.relation);
        let dependent = json!({"phase":"train","status":"verified_decomposition","observed_solving_degree":3,"degree_processed":3,"relation":{"rank_gain":0}});
        assert!(!observation(&dependent).unwrap().relation);
        let exhausted = json!({"phase":"holdout","status":"exhausted","observed_solving_degree":null,"degree_processed":8,"timed_out":true,"field_reduction_multiplications":9});
        let o = observation(&exhausted).unwrap();
        assert_eq!(o.outcome, Outcome::Exhausted);
        assert_eq!(o.censored_above, 7);
        assert!(!o.relation_phase);
        let no_point = json!({"phase":"holdout","status":"no_point_decomposition","observed_solving_degree":4,"degree_processed":4});
        assert_eq!(observation(&no_point).unwrap().outcome, Outcome::Refuted);
        let control = json!({"phase":"positive_control","status":"verified_decomposition","observed_solving_degree":3,"degree_processed":3});
        assert!(observation(&control).is_none());
    }

    #[test]
    fn koblitz_reference_folds_frobenius_and_negation() {
        assert_eq!(automorphisms(&CurveSpec::Koblitz { a: 1, n: 7 }), 14.0);
        assert_eq!(automorphisms(&CurveSpec::PrimeRoster { bits: 16 }), 2.0);
        assert!(advice_rank(Advice::Promising) < advice_rank(Advice::HopelessAtBound));
    }
}
