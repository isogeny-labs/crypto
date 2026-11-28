//! Isolated signed-multiset truth grader for the common-support RR9 predicate.
//!
//! It enumerates `C(2B+7,8)` grader-side multisets.  Candidate processes may
//! receive only a public case and, after completion, its zero/nonzero label;
//! never this index or its witnesses.

use prime31_selector_sat::algebra::{AffinePoint, Curve};
use prime31_selector_sat::rr9::SignedFactor;
use prime31_selector_sat::rr9_panel::{prime31_rr9_case, Rr9Target};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

#[derive(Clone, Debug, Serialize)]
struct FibreEntry {
    multiplicity: u64,
    witness: Vec<SignedFactor>,
}

#[derive(Clone, Debug, Serialize)]
struct Label {
    case_id: String,
    residual: AffinePoint,
    multiplicity: u64,
    witness: Option<Vec<SignedFactor>>,
}

#[derive(Clone, Debug, Serialize)]
struct GraderRecord {
    schema: &'static str,
    base_size: usize,
    total_signed_multisets: usize,
    affine_signed_multisets: u64,
    distinct_affine_residuals: usize,
    natural: Vec<Label>,
    known_chart: Option<Label>,
    natural_positives: usize,
    natural_negatives: usize,
    group_additions_during_enumeration: u64,
    elapsed_ns: String,
    candidate_forbidden_transfer: &'static str,
    pcc: &'static str,
}

fn witness_from_counts(counts: &[u8]) -> Vec<SignedFactor> {
    let mut witness = Vec::new();
    for (token, multiplicity) in counts.iter().copied().enumerate() {
        if multiplicity == 0 {
            continue;
        }
        witness.push(SignedFactor {
            base_index: token / 2,
            sign: if token % 2 == 0 { 1 } else { -1 },
            multiplicity,
        });
    }
    witness
}

fn build_index(
    curve: Curve,
    base: &[AffinePoint],
) -> (HashMap<AffinePoint, FibreEntry>, usize, u64) {
    let mut tokens = Vec::with_capacity(2 * base.len());
    for point in base {
        tokens.push(*point);
        tokens.push(AffinePoint {
            x: point.x,
            y: curve.neg(point.y),
        });
    }
    let mut counts = vec![0u8; tokens.len()];
    let mut index = HashMap::<AffinePoint, FibreEntry>::new();
    let mut total = 0usize;
    let mut additions = 0u64;

    #[allow(clippy::too_many_arguments)]
    fn recurse(
        curve: Curve,
        tokens: &[AffinePoint],
        token: usize,
        remaining: u8,
        running: Option<AffinePoint>,
        counts: &mut [u8],
        index: &mut HashMap<AffinePoint, FibreEntry>,
        total: &mut usize,
        additions: &mut u64,
    ) {
        if remaining == 0 {
            *total += 1;
            if let Some(residual) = running {
                index
                    .entry(residual)
                    .and_modify(|entry| entry.multiplicity += 1)
                    .or_insert_with(|| FibreEntry {
                        multiplicity: 1,
                        witness: witness_from_counts(counts),
                    });
            }
            return;
        }
        if token == tokens.len() {
            return;
        }
        let mut next_sum = running;
        for count in 0..=remaining {
            counts[token] = count;
            recurse(
                curve,
                tokens,
                token + 1,
                remaining - count,
                next_sum,
                counts,
                index,
                total,
                additions,
            );
            if count < remaining {
                next_sum = curve.point_add(next_sum, Some(tokens[token]));
                *additions += 1;
            }
        }
        counts[token] = 0;
    }

    recurse(
        curve,
        &tokens,
        0,
        8,
        None,
        &mut counts,
        &mut index,
        &mut total,
        &mut additions,
    );
    (index, total, additions)
}

fn label(
    case_id: String,
    residual: AffinePoint,
    index: &HashMap<AffinePoint, FibreEntry>,
) -> Label {
    let entry = index.get(&residual);
    Label {
        case_id,
        residual,
        multiplicity: entry.map_or(0, |entry| entry.multiplicity),
        witness: entry.map(|entry| entry.witness.clone()),
    }
}

fn run(base_size: usize) -> Result<GraderRecord, String> {
    if !matches!(base_size, 4 | 8) {
        return Err("registered RR9 grader base size must be 4 or 8".into());
    }
    let started = Instant::now();
    let template = prime31_rr9_case(base_size, Rr9Target::Natural(0))?;
    let (index, total, additions) = build_index(template.curve, &template.base);
    let expected_total = if base_size == 4 { 6_435 } else { 490_314 };
    if total != expected_total {
        return Err(format!(
            "signed-multiset enumeration produced {total}, expected {expected_total}"
        ));
    }

    let natural = (0..64)
        .map(|trial| {
            let case = prime31_rr9_case(base_size, Rr9Target::Natural(trial))?;
            Ok(label(case.case_id, case.residual, &index))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let natural_positives = natural
        .iter()
        .filter(|trial| trial.multiplicity != 0)
        .count();
    let known_chart = if base_size == 8 {
        let case = prime31_rr9_case(base_size, Rr9Target::KnownChart)?;
        let result = label(case.case_id, case.residual, &index);
        if result.multiplicity == 0 {
            return Err("known B8 RR9 chart residual is absent from the truth index".into());
        }
        Some(result)
    } else {
        None
    };
    let affine_signed_multisets = index.values().map(|entry| entry.multiplicity).sum();
    Ok(GraderRecord {
        schema: "prime31.rr9-isolated-grader/v1",
        base_size,
        total_signed_multisets: total,
        affine_signed_multisets,
        distinct_affine_residuals: index.len(),
        natural_negatives: natural.len() - natural_positives,
        natural_positives,
        natural,
        known_chart,
        group_additions_during_enumeration: additions,
        elapsed_ns: started.elapsed().as_nanos().to_string(),
        candidate_forbidden_transfer:
            "candidate may receive only the public case and a post-completion zero/nonzero label; never this index, multiplicity, or witness",
        pcc: "unverified",
    })
}

fn main() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 {
        return Err("usage: prime31_rr9_grader <B:4|8>".into());
    }
    let base_size = arguments[0]
        .parse::<usize>()
        .map_err(|_| format!("invalid base size {:?}", arguments[0]))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&run(base_size)?).map_err(|error| error.to_string())?
    );
    Ok(())
}
