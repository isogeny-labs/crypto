use crate::{sparse_kernel as kernel, sparse_kernel_input as input};
use crypto_lib::cryptanalysis::jv_bitops as work;
use serde_json::json;
use std::{fs, path::Path, time::Instant};
pub const SEED: u64 = 443;
pub const ATTEMPTS: usize = 3;
pub fn run(
    method: &str,
    input_path: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !["counted-rref", "library-wiedemann", "randomized-lanczos"].contains(&method) {
        return Err("method".into());
    }
    let bytes = fs::read(input_path)?;
    let supplied = input::parse(&bytes)?;
    let revision = std::env::var("IC_SOURCE_REVISION")?;
    work::claim();
    work::set_moduli(1009, 263807419614139553);
    let mut records = Vec::new();
    for corpus in &supplied.corpora {
        work::enable(true);
        let before = work::snapshot();
        let ub = work::uniform_snapshot();
        let other = work::snapshot_other();
        let phase = work::enter(work::Phase::LinearAlgebra);
        let start = Instant::now();
        let result = match method {
            "counted-rref" => kernel::counted_rref(&corpus.matrix),
            "library-wiedemann" => kernel::library_wiedemann(&corpus.matrix, SEED, ATTEMPTS),
            _ => kernel::randomized_lanczos(&corpus.matrix, SEED, ATTEMPTS),
        }?;
        let wall = start.elapsed().as_nanos();
        drop(phase);
        let tally = work::snapshot().since(&before);
        let uniform = work::uniform_snapshot()
            .since(&ub)
            .ok_or("uniform underflow")?;
        work::enable(false);
        let extra: Vec<_> = work::snapshot_other()
            .iter()
            .zip(other)
            .map(|(&a, b)| a - b)
            .collect();
        let total = tally.total();
        let price = work::price(&total, &work::widths(1009, corpus.matrix.modulus))
            .map_err(|e| format!("price: {e:?}"))?;
        records.push(json!({"corpus":corpus.name,"result":result,"inner_wall_ns":wall.to_string(),"primitives":work::prims_json(&total),"uniform":work::uniform_json(&uniform.sum(&work::Phase::ALL).ok_or("uniform overflow")?),"observed_incomplete_subtotal":price.to_string(),"foreign_thread_hooks":work::foreign(),"other_modulus_events":extra,"phases":work::Phase::ALL.iter().map(|&p|(p.name().to_string(),work::prims_json(tally.phase(p)))).collect::<serde_json::Map<String,_>>() }));
    }
    input::create_json(
        output,
        &json!({"schema":"cover.sparse-kernel.component/v1","method":method,"source_revision":revision,"input_sha256":input::digest(&bytes),"seed":SEED,"max_attempts":ATTEMPTS,"complete_cost":false,"complete_total":null,"ratios":null,"known_uncounted":["integer divisions and reciprocal setup","allocator and vector construction","some scalar control, index and memory operations"],"excluded":["relation collection and graph filtering","input read/parse/admission","output serialization","independent verification"],"corpora":records}),
    )?;
    Ok(())
}
