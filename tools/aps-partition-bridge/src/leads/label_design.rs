//! Fixed, bounded label-law comparison. Candidate point sets are freshly generated.
use crate::{quartic_cells, quartic_oracle, sampler, valuation};
use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::aps_single_relation::{self as aps, LabeledBase, LabeledPoint};
use std::collections::BTreeMap;

const DRAW_CAP: usize = 1000;
const WEIGHTS: [[u64; 4]; 2] = [[1, 2, 4, 8], [1, 3, 9, 27]];

mod models;
pub use models::Report;
use models::{Arm, Draw, Generation, Profile, TupleRow};

fn signs(labels: &[u64], order: u64) -> Result<Vec<u64>, String> {
    if labels.len() != 4 || order < 3 || labels.iter().any(|&b| b >= order) {
        return Err("four canonical label residues and order>=3 required".into());
    }
    Ok((0..16)
        .map(|mask| {
            labels.iter().enumerate().fold(0, |sum, (i, &b)| {
                (sum + if mask & (1 << i) == 0 {
                    b
                } else {
                    (order - b) % order
                }) % order
            })
        })
        .collect())
}

fn fresh_base(
    f: &sampler::Fixture,
    seed: u64,
    weights: [u64; 4],
    cap: usize,
) -> Result<Generation, String> {
    let order = f.curve.n;
    if weights.iter().any(|&v| v == 0 || v >= order) {
        return Err("candidate weights must be nonzero subgroup residues".into());
    }
    let mut base = LabeledBase {
        parts: vec![Vec::new(); 4],
        points: BTreeMap::new(),
        draws: 0,
    };
    let mut draws = Vec::new();
    for (slot, &v) in weights.iter().enumerate() {
        while base.parts[slot].len() < 2 {
            if base.draws == cap {
                return Ok(Generation {
                    base,
                    draws,
                    error: Some("label-law candidate draw cap exhausted".into()),
                });
            }
            // A public deterministic full cycle modulo107; no target logarithm.
            // The same raw stream is used by both laws, but rejection can differ.
            let u = (seed % order + 53 * base.draws as u64 % order) % order;
            base.draws += 1;
            let point = f
                .curve
                .add(f.curve.mul(Some(f.g), u), f.curve.mul(Some(f.q), v));
            let Some(point) = point else {
                draws.push(Draw {
                    index: base.draws,
                    slot,
                    u,
                    v,
                    point_x: None,
                    outcome: "infinity",
                });
                continue;
            };
            let (point, flipped) = f.curve.canonical_sign(point);
            let (x, y) = f.curve.canonical(point);
            if base.points.contains_key(&x) {
                draws.push(Draw {
                    index: base.draws,
                    slot,
                    u,
                    v,
                    point_x: Some(x),
                    outcome: "duplicate_x",
                });
                continue;
            }
            let entry = LabeledPoint {
                x,
                y,
                u: if flipped { (order - u) % order } else { u },
                v: if flipped { (order - v) % order } else { v },
            };
            base.parts[slot].push(x);
            base.points.insert(x, entry);
            draws.push(Draw {
                index: base.draws,
                slot,
                u,
                v,
                point_x: Some(x),
                outcome: "accepted",
            });
        }
    }
    let error = aps::validate_base(&f.curve, f.g, f.q, &base).err();
    Ok(Generation { base, draws, error })
}

fn profile(seed: u64, weights: [u64; 4]) -> Result<Profile, String> {
    let mut f = sampler::even_fixture(seed)?;
    let fixture_setup_draws = f.base.draws;
    let generated = fresh_base(&f, seed, weights, DRAW_CAP)?;
    if generated.error.is_some() {
        return Ok(Profile::incomplete(
            seed,
            weights,
            fixture_setup_draws,
            generated,
        ));
    }
    let draws = generated.draws;
    f.base = generated.base;
    let mut correct = true;
    let mut tuples = Vec::new();
    let mut roots = Vec::new();
    let mut valuation_safe = 0;
    let mut exact_safe = 0;
    let mut informative = 0;
    let mut zero = 0;
    for coordinates in quartic_cells::universe(&f) {
        let labels: Vec<_> = coordinates.iter().map(|x| f.base.points[x].v).collect();
        let residues = signs(&labels, f.curve.n)?;
        let safe = !residues.contains(&0);
        let valuation = valuation::guaranteed_nonzero(&labels, f.curve.n);
        let (good, bad) = quartic_oracle::incidents(&f.base, &coordinates)?;
        correct &= !valuation || safe;
        correct &= !safe || bad == 0;
        valuation_safe += usize::from(valuation);
        exact_safe += usize::from(safe);
        informative += good;
        zero += bad;
        if good + bad > 0 {
            roots.push(coordinates.clone());
        }
        tuples.push(TupleRow {
            coordinates,
            labels,
            signed_b_residues: residues,
            valuation_safe: valuation,
            exact_sign_safe: safe,
            informative_group_incidents: good,
            zero_b_group_incidents: bad,
        });
    }
    correct &= tuples.len() == 70 && f.base.points.len() == 8;
    let (recovery, recovery_error) = match aps::recover(&f.curve, f.g, f.q, &f.base, &roots) {
        Ok(r) => (r, None),
        Err(e) => {
            correct = false;
            (None, Some(e))
        }
    };
    correct &= recovery.is_some() == (informative > 0);
    if let Some(r) = &recovery {
        let curve = Curve::new(101, 59, 6)?;
        let (gx, gy) = f.curve.canonical(f.g);
        let (qx, qy) = f.curve.canonical(f.q);
        correct &= r.b != 0
            && f.curve.mul(Some(f.g), r.logarithm) == Some(f.q)
            && curve.mul(Some(Point { x: gx, y: gy }), r.logarithm) == Some(Point { x: qx, y: qy });
    }
    let mut poisoned = f.base.clone();
    let p = poisoned
        .points
        .values_mut()
        .next()
        .ok_or("empty generated base")?;
    p.u = (p.u + 1) % f.curve.n;
    let corrupted_label_refused = aps::validate_base(&f.curve, f.g, f.q, &poisoned).is_err();
    correct &= corrupted_label_refused;
    Ok(Profile {
        seed,
        weights,
        fixture_setup_draws,
        generation_error: None,
        base: f.base,
        draws,
        tuples,
        valuation_safe_tuples: valuation_safe,
        exact_sign_safe_tuples: exact_safe,
        informative_group_incidents: informative,
        zero_b_group_incidents: zero,
        recovered_logarithm: recovery.map(|r| r.logarithm),
        recovery_error,
        corrupted_label_refused,
        complete_and_equal: correct,
    })
}

pub fn run() -> Result<Report, String> {
    let modular_wrap_control_refused =
        signs(&[1, 2, 2, 2], 7)?.contains(&0) && !valuation::guaranteed_nonzero(&[1, 2, 2, 2], 7);
    let mut correct = modular_wrap_control_refused;
    let mut arms = Vec::new();
    for seed in sampler::SEEDS {
        for weights in WEIGHTS {
            let (report, error) = match profile(seed, weights) {
                Ok(r) => {
                    correct &= r.complete_and_equal;
                    (Some(r), None)
                }
                Err(e) => {
                    correct = false;
                    (None, Some(e))
                }
            };
            arms.push(Arm {
                seed,
                weights,
                report,
                error,
            });
        }
    }
    Ok(Report {
        candidate_u_stream: "u_i=(seed+53*i) mod107; matched raw stream, rejection may differ",
        coordinate_sets_matched: false,
        polynomial_solver_tested: false,
        performance_trial: false,
        planned_arms: 4,
        attempted_arms: arms.len(),
        arms,
        modular_wrap_control_refused,
        complete_and_equal: correct,
    })
}
