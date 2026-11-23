//! Paired arithmetic replay of one immutable affine relation matrix.
#[allow(dead_code)]
#[path = "two-lp-dlp-analysis.rs"]
pub(crate) mod full;
#[path = "../normalization_rref.rs"]
pub(crate) mod kernel;
use full::shared::{canonical, n, price, prime, Field, Widths, KINDS};
use kernel::{Counts, Rref};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    time::Instant,
};

pub(crate) struct Matrix {
    pub p: u64,
    pub l: u64,
    pub columns: usize,
    pub target: usize,
    pub rows: Vec<(Vec<u64>, u64)>,
    pub original_source: String,
    pub original_scalar: u64,
    pub original_products: u64,
    pub original_rank: u64,
}
pub(crate) fn decode(v: &Value) -> Result<Matrix, String> {
    let p = n(&v["public"], "p")?;
    let l = n(&v["public"], "l")?;
    let target = n(&v["work"]["final_projection"], "target_column")? as usize;
    let source = v["source_revision"].as_str().ok_or("original source")?;
    let attempts = v["work"]["projection_solve_attempts"]
        .as_array()
        .ok_or("attempts")?;
    if v["schema"] != "cover.two-large-prime.dlp-worker/v1"
        || v["method"] != "two-lp"
        || v["verified"] != true
        || v["capped"] != false
        || !(101..=1511).contains(&p)
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || target > 4095
        || source.len() != 40
        || !source.bytes().all(|b| b.is_ascii_hexdigit())
        || attempts.len() != 1
    {
        return Err("bounded original scalar-recovery matrix required".into());
    }
    let columns = target + 1;
    let certs = v["work"]["final_projection"]["certificates"]
        .as_array()
        .ok_or("certificates")?;
    if certs.is_empty() || certs.len() > 4096 {
        return Err("matrix row bound".into());
    }
    let mut rows = Vec::new();
    for row in certs {
        let mut dense = vec![0; columns];
        for (c, value) in canonical(&row["small"], columns as u64, l)? {
            dense[c as usize] = value;
        }
        let rhs = n(row, "rhs")?;
        if rhs >= l {
            return Err("RHS not reduced".into());
        }
        rows.push((dense, rhs));
    }
    Ok(Matrix {
        p,
        l,
        columns,
        target,
        rows,
        original_source: source.into(),
        original_scalar: n(v, "actual_scalar")?,
        original_products: n(&v["work"], "solver_modular_muls")?,
        original_rank: n(&v["work"], "last_rank")?,
    })
}
pub(crate) fn primitives(c: &Counts) -> BTreeMap<String, u64> {
    let mut p: BTreeMap<String, u64> = KINDS.into_iter().map(|k| (k.into(), 0)).collect();
    p.insert("modr_mul".into(), c.products);
    p.insert("modr_add".into(), c.subtractions);
    p.insert("modr_inv".into(), c.inversions);
    p.insert("word_op".into(), c.normalization_tests);
    p
}
pub(crate) fn replay<const SKIP: bool>(
    p: u64,
    l: u64,
    columns: usize,
    target: usize,
    rows: &[(Vec<u64>, u64)],
) -> Result<Value, String> {
    let mut rref = Rref::<SKIP>::new(l, columns);
    let mut states = Vec::new();
    let started = Instant::now();
    for (i, (row, rhs)) in rows.iter().enumerate() {
        let independent = rref.insert(row.clone(), *rhs)?;
        let answer = rref.solved(target);
        states.push(json!({"row":i,"rank":rref.rank(),"independent":independent,"state_blake3":rref.state_digest(),"target":answer}));
        if answer.is_some() {
            break;
        }
    }
    let actual = rref.solved(target);
    let elapsed = started.elapsed().as_nanos();
    let prim = primitives(&rref.counts);
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    Ok(
        json!({"mode":if SKIP{"skip-zero-normalization"}else{"baseline"},"rank":rref.rank(),"rows_used":states.len(),"actual_scalar":actual,
        "counts":rref.counts,"primitives":prim,"observed_primitive_subtotal_bitops":price(&prim,&widths)?.to_string(),"states":states,
        "recorded_inner_wall_ns_practicality_only":elapsed,"complete_cost":false,"total_bitops":null}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: normalization-bench INPUT.json NEW-RAW.json SOURCE-SHA".into());
    }
    if args[3].len() != 40 || !args[3].bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("source SHA required".into());
    }
    let bytes = fs::read(&args[1])?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("input bound".into());
    }
    let original: Value = serde_json::from_slice(&bytes)?;
    let matrix = decode(&original)?;
    // Solvers receive rows, modulus and target column, never the recorded answer.
    let baseline = replay::<false>(
        matrix.p,
        matrix.l,
        matrix.columns,
        matrix.target,
        &matrix.rows,
    )?;
    let candidate = replay::<true>(
        matrix.p,
        matrix.l,
        matrix.columns,
        matrix.target,
        &matrix.rows,
    )?;
    let result = json!({"schema":"cover.rref-normalization-replay/v1","source_revision":args[3],"original_source_revision":matrix.original_source,
        "original_raw_blake3":blake3::hash(&bytes).to_hex().to_string(),"p":matrix.p,"l":matrix.l,"columns":matrix.columns,"target_column":matrix.target,
        "rows_available":matrix.rows.len(),"recorded_reference_after_solving":{"scalar":matrix.original_scalar,"rank":matrix.original_rank,"products":matrix.original_products},"arms":[baseline,candidate],"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,
        "new_full_dlp_trial":false,"rho_parity":false,"cost_ratios":null,
        "pricing_scope":"logical modr products/subtractions/inverses plus one word event per candidate normalization zero test",
        "known_uncounted":["native loop/scalar/container operations","fixed-factor quotient and Euclidean internals","diagnostic per-insert state hashing and zero observation","I/O and serialization"],
        "wall_scope":"includes diagnostic state hashing/observer instrumentation; practicality only"});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer(&mut file, &result)?;
    writeln!(file)?;
    println!(
        "{}",
        json!({"schema":result["schema"],"out":args[2],"new_full_dlp_trial":false})
    );
    Ok(())
}
