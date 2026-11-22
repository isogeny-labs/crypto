//! Recorded checkpoint+affine component comparison, without an input answer.
#[path = "../homogeneous_quotient.rs"]
pub(crate) mod quotient;
#[allow(dead_code)]
#[path = "normalization-bench.rs"]
pub(crate) mod saved;
use quotient::Quotient;
use saved::{
    full::shared::{canonical, n, price, Field, Widths, KINDS},
    kernel::{Counts, Rref},
    Matrix,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};
pub(crate) const MODES: [&str; 6] = [
    "s75-r32", "s75-r128", "s85-r32", "s85-r128", "s95-r32", "s95-r128",
];
pub(crate) struct Input {
    pub matrix: Matrix,
    pub homogeneous: Vec<(Vec<u64>, u64)>,
    pub expected_hom_products: u64,
    pub expected_hom_rank: u64,
}
pub(crate) fn decode(v: &Value) -> Result<Input, String> {
    let matrix = saved::decode(v)?;
    let checkpoints = v["work"]["collection_rank_checks"]
        .as_array()
        .ok_or("checkpoints")?;
    if checkpoints.len() != 1 || matrix.target == 0 {
        return Err("one homogeneous checkpoint required".into());
    }
    let c = &checkpoints[0];
    if c["rank"] != matrix.target - 1 || c["ready"] != true || c["rank_target"] != matrix.target - 1
    {
        return Err("codimension-one checkpoint".into());
    }
    let certs = c["certificates"]
        .as_array()
        .ok_or("homogeneous certificates")?;
    if certs.is_empty()
        || certs.len() > 4096
        || c["complete_rows"] != certs.len()
        || c["replayed_rows"] != certs.len()
    {
        return Err("homogeneous rows".into());
    }
    let mut homogeneous = Vec::new();
    for row in certs {
        if row["rhs"] != 0 {
            return Err("homogeneous RHS".into());
        }
        let mut dense = vec![0; matrix.target];
        for (col, value) in canonical(&row["small"], matrix.target as u64, matrix.l)? {
            dense[col as usize] = value;
        }
        homogeneous.push((dense, 0));
    }
    Ok(Input {
        matrix,
        homogeneous,
        expected_hom_products: n(c, "solver_modular_muls")?,
        expected_hom_rank: n(c, "rank")?,
    })
}
pub(crate) fn primitive_counts(
    parts: &[&Counts],
    extra: &quotient::ExtraCounts,
) -> Result<BTreeMap<String, u64>, String> {
    let mut result: BTreeMap<String, u64> = KINDS.into_iter().map(|k| (k.into(), 0)).collect();
    for (key, value) in [
        ("modr_mul", extra.products),
        ("modr_add", extra.additions),
        ("word_op", extra.word_events),
    ] {
        result.insert(key.into(), value);
    }
    for c in parts {
        for (key, value) in [
            ("modr_mul", c.products),
            ("modr_add", c.subtractions),
            ("modr_inv", c.inversions),
        ] {
            let slot = result.get_mut(key).ok_or("primitive")?;
            *slot = slot.checked_add(value).ok_or("counter overflow")?;
        }
    }
    Ok(result)
}
fn homogeneous(l: u64, columns: usize, rows: &[(Vec<u64>, u64)]) -> Result<Rref<false>, String> {
    let mut system = Rref::<false>::new(l, columns);
    for (row, rhs) in rows {
        system.insert(row.clone(), *rhs)?;
    }
    Ok(system)
}
fn affine(
    l: u64,
    columns: usize,
    target: usize,
    rows: &[(Vec<u64>, u64)],
) -> Result<(Rref<false>, Vec<Value>, u64), String> {
    let mut system = Rref::<false>::new(l, columns);
    let mut states = Vec::new();
    for (i, (row, rhs)) in rows.iter().enumerate() {
        let independent = system.insert(row.clone(), *rhs)?;
        let answer = system.solved(target);
        states.push(json!({"row":i,"rank":system.rank(),"independent":independent,"state_blake3":system.state_digest(),"target":answer}));
        if let Some(answer) = answer {
            return Ok((system, states, answer));
        }
    }
    Err("target undetermined".into())
}
// Only matrix/modulus/target data enters either solver; references are emitted later.
pub(crate) fn pair(
    l: u64,
    columns: usize,
    target: usize,
    h_rows: &[(Vec<u64>, u64)],
    rows: &[(Vec<u64>, u64)],
    p: u64,
) -> Result<Value, String> {
    if columns != target + 1 || target == 0 {
        return Err("target-last matrix required".into());
    }
    let started = Instant::now();
    let old_h = homogeneous(l, target, h_rows)?;
    let (old, old_states, old_answer) = affine(l, columns, target, rows)?;
    let new_h = homogeneous(l, target, h_rows)?;
    let mut quotient = Quotient::from_rref(l, target, new_h.pivot_rows())?;
    let basis = quotient.clone();
    let mut new = Rref::<false>::new(l, 2);
    let mut new_states = Vec::new();
    let mut projected = Vec::new();
    let mut answer = None;
    for (i, (row, rhs)) in rows.iter().enumerate() {
        let (small, b) = quotient.project(row, *rhs)?;
        projected.push(json!({"row":i,"coefficients":small,"rhs":b}));
        let independent = new.insert(small.to_vec(), b)?;
        let value = new.solved(1);
        new_states.push(json!({"row":i,"rank":new.rank(),"independent":independent,"state_blake3":new.state_digest(),"target":value}));
        if value.is_some() {
            answer = value;
            break;
        }
    }
    let answer = answer.ok_or("quotient target undetermined")?;
    if answer != old_answer
        || old_h.state_digest() != new_h.state_digest()
        || old_h.counts != new_h.counts
    {
        return Err("paired target/checkpoint disagreement".into());
    }
    let old_counts = primitive_counts(&[&old_h.counts, &old.counts], &Default::default())?;
    let new_counts = primitive_counts(&[&new_h.counts, &new.counts], &quotient.counts)?;
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    Ok(
        json!({"columns":columns,"target_column":target,"p":p,"l":l,"rows_available":rows.len(),"homogeneous_rows":h_rows.len(),
        "baseline":{"homogeneous":{"rank":old_h.rank(),"counts":old_h.counts,"state_blake3":old_h.state_digest()},"affine":{"rank":old.rank(),"counts":old.counts,"states":old_states},"actual_scalar":old_answer,
            "observed_primitives":old_counts,"observed_incomplete_bitops":price(&old_counts,&widths)?.to_string()},
        "candidate":{"homogeneous":{"rank":new_h.rank(),"counts":new_h.counts,"state_blake3":new_h.state_digest()},"basis":basis,"extra_counts":quotient.counts,"projected_rows":projected,
            "affine":{"rank":new.rank(),"counts":new.counts,"states":new_states},"actual_scalar":answer,"observed_primitives":new_counts,"observed_incomplete_bitops":price(&new_counts,&widths)?.to_string()},
        "inner_wall_ns_practicality_only":started.elapsed().as_nanos().to_string(),"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: quotient-bench ORIGINALS-DIR NEW-RAW.json SOURCE-SHA".into());
    }
    if args[3].len() != 40 || !args[3].bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("source SHA".into());
    }
    let mut outcomes = Vec::new();
    for mode in MODES {
        let bytes = fs::read(Path::new(&args[1]).join(format!("{mode}.raw.json")))?;
        if bytes.len() > 32 * 1024 * 1024 {
            return Err("input bound".into());
        }
        let original: Value = serde_json::from_slice(&bytes)?;
        let input = decode(&original)?;
        let m = &input.matrix;
        let mut result = pair(m.l, m.columns, m.target, &input.homogeneous, &m.rows, m.p)?;
        if result["baseline"]["affine"]["counts"]["products"] != m.original_products
            || result["baseline"]["affine"]["rank"] != m.original_rank
            || result["baseline"]["homogeneous"]["counts"]["products"]
                != input.expected_hom_products
            || result["baseline"]["homogeneous"]["rank"] != input.expected_hom_rank
        {
            return Err("original arithmetic/rank reference mismatch".into());
        }
        result["arm"] = json!(mode);
        result["original_source_revision"] = json!(m.original_source);
        result["original_scalar_reference"] = json!(m.original_scalar);
        result["original_input_blake3"] = json!(blake3::hash(&bytes).to_hex().to_string());
        outcomes.push(result);
    }
    let raw = json!({"schema":"cover.homogeneous-quotient-replay/v1","source_revision":args[3],"outcomes":outcomes,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null,
        "known_uncounted":["native RREF scans/loop control/fixed factors/Euclidean work","other kernel/projection native control/allocation","diagnostic state hashing/serialization"],"new_dlp_sample":false,"reprices_prior_pipeline":false});
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut output, &raw)?;
    writeln!(output)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quotient_matches_full_hand_affine_target() {
        let result = pair(
            101,
            3,
            2,
            &[(vec![1, 2], 0)],
            &[(vec![1, 2, 0], 0), (vec![3, 4, 1], 17), (vec![1, 0, 2], 9)],
            103,
        )
        .unwrap();
        assert_eq!(result["baseline"]["actual_scalar"], 93);
        assert_eq!(result["candidate"]["actual_scalar"], 93);
        assert_eq!(result["candidate"]["affine"]["rank"], 2);
        assert_eq!(result["candidate"]["extra_counts"]["products"], 5);
    }
    #[test]
    fn insufficient_homogeneous_rank_and_inconsistent_affine_rejected() {
        assert!(pair(101, 3, 2, &[], &[(vec![1, 0, 1], 1)], 103).is_err());
        assert!(pair(101, 3, 2, &[(vec![1, 2], 0)], &[(vec![1, 2, 0], 1)], 103).is_err());
    }
}
