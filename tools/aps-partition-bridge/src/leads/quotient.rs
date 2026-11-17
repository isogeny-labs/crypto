//! Exact finite-domain remainders; these are correctness diagnostics, not costs.
use crate::{quartic_cells, quartic_oracle, sampler};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Encoding},
    f4_fp::{self, F4Options, Ordering, Poly},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::BTreeSet, time::Duration};

mod reduction;
use reduction::remainder;

fn profile(poly: &Poly) -> Value {
    let mut degrees = vec![0; 4];
    for (e, _) in poly {
        for (d, x) in degrees.iter_mut().zip(e) {
            *d = (*d).max(*x);
        }
    }
    json!({"terms":poly.len(),"coordinate_degrees":degrees,
        "total_degree":poly.iter().map(|(e,_)| e.iter().sum::<u32>()).max(),
        "zero":poly.is_empty(),"constant":!poly.is_empty() && degrees.iter().all(|&x|x==0)})
}

#[derive(Serialize)]
struct Route {
    complete: bool,
    timed_out: bool,
    error: Option<String>,
    roots: BTreeSet<Vec<u64>>,
}

fn visit(equations: &[Poly], parts: &[Vec<u64>], p: u64, budget: Duration) -> Route {
    let options = F4Options::new(Ordering::Grevlex, 24).with_budget(budget);
    let domains: Vec<_> = parts.iter().cloned().map(Some).collect();
    let mut roots = BTreeSet::new();
    let mut callback = |point: &[u64]| {
        let mut tuple = point.to_vec();
        tuple.sort_unstable();
        roots.insert(tuple);
        false
    };
    match f4_fp::solve_visit(equations, 4, p, &options, &domains, &mut callback) {
        Ok(r) => Route {
            complete: r.complete,
            timed_out: r.timed_out,
            error: None,
            roots,
        },
        Err(e) => Route {
            complete: false,
            timed_out: false,
            error: Some(e),
            roots,
        },
    }
}

fn cell_report(f: &sampler::Fixture, cell: &quartic_cells::Cell) -> Result<(Value, bool), String> {
    let system = aps::build_system(&f.curve, &cell.parts, Encoding::Expanded4)?;
    let original = &system.equations()[0];
    let (reduced, rewrites) = remainder(original, &cell.parts, system.modulus())?;
    let tuples = quartic_cells::product(&cell.parts)?;
    let expected = quartic_oracle::roots(&f.base, &tuples)?;
    let mut values = Vec::new();
    let mut direct_roots = BTreeSet::new();
    let mut equal_values = true;
    for &a in &cell.parts[0] {
        for &b in &cell.parts[1] {
            for &c in &cell.parts[2] {
                for &d in &cell.parts[3] {
                    let point = [a, b, c, d];
                    let before = f4_fp::eval(original, &point, system.modulus());
                    let after = f4_fp::eval(&reduced, &point, system.modulus());
                    equal_values &= before == after;
                    if before == 0 {
                        let mut tuple = point.to_vec();
                        tuple.sort_unstable();
                        direct_roots.insert(tuple);
                    }
                    values.push(json!({"ordered_coordinates":point,"original_value":before,"remainder_value":after}));
                }
            }
        }
    }
    let mut equations = system.equations().to_vec();
    equations[0] = reduced.clone();
    let cold_original = visit(
        system.equations(),
        &cell.parts,
        system.modulus(),
        Duration::from_millis(2000),
    );
    let cold_reduced = visit(
        &equations,
        &cell.parts,
        system.modulus(),
        Duration::from_millis(2000),
    );
    let correct = equal_values
        && direct_roots == expected
        && [&cold_original, &cold_reduced]
            .iter()
            .all(|r| r.complete && !r.timed_out && r.error.is_none() && r.roots == expected);
    Ok((
        json!({"cell":cell,"original_polynomial":original,"remainder_polynomial":reduced,
        "original_profile":profile(original),"remainder_profile":profile(&reduced),
        "unchanged":original==&reduced,"reduction_monomial_rewrites":rewrites,
        "cartesian_tuples":values.len(),"direct_baseline_polynomial_evaluations":values.len(),
        "remainder_verification_polynomial_evaluations":values.len(),"values":values,"equal_values":equal_values,
        "direct_roots":direct_roots,"expected_affine_roots":expected,
        "original_route":cold_original,"reduced_route":cold_reduced,"complete_and_equal":correct}),
        correct,
    ))
}

#[derive(Serialize)]
pub struct Report {
    schema: &'static str,
    pub complete_and_equal: bool,
    performance_trial: bool,
    cost_complete: bool,
    flow: Value,
    seed_errors: Vec<(u64, String)>,
    samples: Vec<Value>,
}

pub fn run() -> Result<Report, String> {
    let mut samples = Vec::new();
    let mut correct = true;
    let mut seed_errors = Vec::new();
    let mut attempted_cells = 0;
    let mut completed_cells = 0;
    for seed in sampler::SEEDS {
        let f = match sampler::even_fixture(seed) {
            Ok(f) => f,
            Err(error) => {
                correct = false;
                samples.push(json!({"seed":seed,"error":error}));
                seed_errors.push((seed, error));
                continue;
            }
        };
        let cells = match quartic_cells::plan(&f) {
            Ok(cells) => cells,
            Err(error) => {
                correct = false;
                samples.push(json!({"seed":seed,"error":error}));
                seed_errors.push((seed, error));
                continue;
            }
        };
        let mut rows = Vec::new();
        for cell in cells {
            attempted_cells += 1;
            match cell_report(&f, &cell) {
                Ok((row, pass)) => {
                    correct &= pass;
                    completed_cells += usize::from(pass);
                    rows.push(row);
                }
                Err(error) => {
                    correct = false;
                    rows.push(json!({"cell":cell,"error":error,"complete_and_equal":false}));
                }
            }
        }
        samples.push(json!({"seed":seed,"base":f.base,"cells":rows}));
    }
    correct &= samples.len() == 2 && attempted_cells == 38 && seed_errors.is_empty();
    Ok(Report {
        schema: "aps.quotient-reduction/v1",
        complete_and_equal: correct,
        performance_trial: false,
        cost_complete: false,
        flow: json!({"planned_samples":2,"attempted_samples":samples.len(),
            "planned_cells":38,"attempted_cells":attempted_cells,"completed_cells":completed_cells,
            "failed_or_incomplete_cells":attempted_cells-completed_cells,
            "not_attempted_cells":38usize.saturating_sub(attempted_cells),"excluded_by_rule":0}),
        seed_errors,
        samples,
    })
}

#[cfg(test)]
mod tests;
