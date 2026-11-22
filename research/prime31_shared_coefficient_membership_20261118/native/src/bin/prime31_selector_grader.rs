//! Isolated generic four-plus-four truth grader for selector-SAT cases.
//!
//! This binary is intentionally separate from the candidate encoder.  Its
//! tables and witness words must never be passed to a candidate solver.  It is
//! a correctness oracle, not an index-calculus advance.

use prime31_selector_sat::algebra::{AffinePoint, Curve};
use prime31_selector_sat::panel::{planted_word, prime31_weighted_relation, PanelTarget};
use prime31_selector_sat::{RelationSpec, W8_ODD_WEIGHTS};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Instant;

type Word4 = [u8; 4];

#[derive(Clone, Debug, Serialize)]
struct GraderWitness {
    indices: [usize; 8],
    signs: [bool; 8],
}

#[derive(Clone, Debug, Serialize)]
struct Trial {
    case_id: String,
    target: AffinePoint,
    multiplicity: u64,
    witness: Option<GraderWitness>,
}

#[derive(Clone, Debug, Serialize)]
struct HalfSummary {
    ordered_words: usize,
    formal_support: usize,
    group_support: usize,
}

#[derive(Clone, Debug, Serialize)]
struct PanelRecord {
    schema: &'static str,
    base_size: usize,
    weights: [u64; 8],
    left: HalfSummary,
    right: HalfSummary,
    natural: Vec<Trial>,
    planted: Vec<Trial>,
    natural_positives: usize,
    natural_negatives: usize,
    natural_multiplicity_sum: u64,
    elapsed_ns: String,
    candidate_forbidden_transfer: &'static str,
    pcc: &'static str,
}

struct Half {
    words: Vec<(Option<AffinePoint>, Word4)>,
    index: HashMap<Option<AffinePoint>, (u64, Word4)>,
    formal_support: usize,
}

fn signed_point(
    curve: Curve,
    relation: &RelationSpec,
    slot: usize,
    token: usize,
) -> Option<AffinePoint> {
    let base_size = relation.base_size();
    let point = relation.slots[slot].points[token % base_size];
    if token < base_size {
        Some(point)
    } else {
        curve.point_neg(Some(point))
    }
}

fn build_half(relation: &RelationSpec, begin: usize) -> Result<Half, String> {
    let curve = relation.curve;
    let base_size = relation.base_size();
    if base_size > 8 {
        return Err("frozen generic grader supports base sizes at most eight".into());
    }
    let alphabet = 2 * base_size;
    let capacity = alphabet.pow(4);
    let mut words = Vec::with_capacity(capacity);
    let mut index = HashMap::<Option<AffinePoint>, (u64, Word4)>::with_capacity(capacity);
    let mut formal = HashSet::<[i32; 8]>::with_capacity(capacity);
    for i in 0..alphabet {
        for j in 0..alphabet {
            let sum2 = curve.point_add(
                signed_point(curve, relation, begin, i),
                signed_point(curve, relation, begin + 1, j),
            );
            for k in 0..alphabet {
                let sum3 = curve.point_add(sum2, signed_point(curve, relation, begin + 2, k));
                for l in 0..alphabet {
                    let sum = curve.point_add(sum3, signed_point(curve, relation, begin + 3, l));
                    let word = [i as u8, j as u8, k as u8, l as u8];
                    let mut coefficients = [0i32; 8];
                    for offset in 0..4 {
                        let token = word[offset] as usize;
                        let sign = if token < base_size { 1 } else { -1 };
                        coefficients[token % base_size] +=
                            sign * W8_ODD_WEIGHTS[begin + offset] as i32;
                    }
                    formal.insert(coefficients);
                    words.push((sum, word));
                    index
                        .entry(sum)
                        .and_modify(|entry| entry.0 += 1)
                        .or_insert((1, word));
                }
            }
        }
    }
    if words.len() != capacity {
        return Err("half-word enumeration count mismatch".into());
    }
    if index.len() != formal.len() {
        return Err("unexpected EC-only collision in a four-slot half".into());
    }
    Ok(Half {
        words,
        index,
        formal_support: formal.len(),
    })
}

fn decode_word(base_size: usize, left: Word4, right: Word4) -> GraderWitness {
    let tokens = [
        left[0], left[1], left[2], left[3], right[0], right[1], right[2], right[3],
    ];
    GraderWitness {
        indices: std::array::from_fn(|slot| tokens[slot] as usize % base_size),
        signs: std::array::from_fn(|slot| tokens[slot] as usize >= base_size),
    }
}

fn query(relation: &RelationSpec, left: &Half, right: &Half) -> Result<Trial, String> {
    let curve = relation.curve;
    let mut multiplicity = 0u64;
    let mut witness = None;
    for (left_sum, left_word) in &left.words {
        let complement = curve.point_add(Some(relation.target), curve.point_neg(*left_sum));
        if let Some((right_multiplicity, right_word)) = right.index.get(&complement) {
            multiplicity = multiplicity
                .checked_add(*right_multiplicity)
                .ok_or_else(|| "relation multiplicity overflow".to_owned())?;
            if witness.is_none() {
                let candidate = decode_word(relation.base_size(), *left_word, *right_word);
                if relation.replay(&candidate.indices, &candidate.signs) != Some(relation.target) {
                    return Err("grader witness fails independent affine replay".into());
                }
                witness = Some(candidate);
            }
        }
    }
    if (multiplicity == 0) != witness.is_none() {
        return Err("multiplicity/witness presence mismatch".into());
    }
    Ok(Trial {
        case_id: relation.case_id.clone(),
        target: relation.target,
        multiplicity,
        witness,
    })
}

fn run(base_size: usize) -> Result<PanelRecord, String> {
    if !matches!(base_size, 4 | 6 | 8) {
        return Err("registered grader base size must be 4, 6, or 8".into());
    }
    let started = Instant::now();
    let template = prime31_weighted_relation(base_size, PanelTarget::Planted(0))?;
    let left = build_half(&template, 0)?;
    let right = build_half(&template, 4)?;
    let left_summary = HalfSummary {
        ordered_words: left.words.len(),
        formal_support: left.formal_support,
        group_support: left.index.len(),
    };
    let right_summary = HalfSummary {
        ordered_words: right.words.len(),
        formal_support: right.formal_support,
        group_support: right.index.len(),
    };
    if base_size == 4 && (left.formal_support != 3_921 || right.formal_support != 3_921) {
        return Err("frozen B4 half-support count mismatch".into());
    }
    if base_size == 8 && (left.formal_support != 64_737 || right.formal_support != 64_737) {
        return Err("frozen B8 half-support count mismatch".into());
    }

    let mut natural = Vec::with_capacity(64);
    for trial in 0..64 {
        let relation = prime31_weighted_relation(base_size, PanelTarget::Natural(trial))?;
        natural.push(query(&relation, &left, &right)?);
    }
    let mut planted = Vec::with_capacity(8);
    for trial in 0..8 {
        let relation = prime31_weighted_relation(base_size, PanelTarget::Planted(trial))?;
        let result = query(&relation, &left, &right)?;
        let (indices, signs) = planted_word(base_size, trial);
        if relation.replay(&indices, &signs) != Some(relation.target) || result.multiplicity == 0 {
            return Err("planted detector failed".into());
        }
        planted.push(result);
    }
    let natural_positives = natural
        .iter()
        .filter(|trial| trial.multiplicity != 0)
        .count();
    let natural_multiplicity_sum = natural.iter().map(|trial| trial.multiplicity).sum();
    if base_size == 4 && (natural_positives != 0 || natural_multiplicity_sum != 0) {
        return Err("frozen B4 natural labels disagree".into());
    }
    if base_size == 8 && (natural_positives != 46 || natural_multiplicity_sum != 115) {
        return Err("frozen B8 natural labels disagree".into());
    }
    Ok(PanelRecord {
        schema: "prime31.selector-isolated-grader/v1",
        base_size,
        weights: W8_ODD_WEIGHTS,
        left: left_summary,
        right: right_summary,
        natural_negatives: natural.len() - natural_positives,
        natural_positives,
        natural_multiplicity_sum,
        natural,
        planted,
        elapsed_ns: started.elapsed().as_nanos().to_string(),
        candidate_forbidden_transfer:
            "only case_id, public target, and zero/nonzero label may cross after candidate completion; never tables, multiplicities, or witnesses",
        pcc: "unverified",
    })
}

fn main() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 {
        return Err("usage: prime31_selector_grader <B:4|6|8>".into());
    }
    let base_size = arguments[0]
        .parse::<usize>()
        .map_err(|_| format!("invalid base size {:?}", arguments[0]))?;
    let record = run(base_size)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?
    );
    Ok(())
}
