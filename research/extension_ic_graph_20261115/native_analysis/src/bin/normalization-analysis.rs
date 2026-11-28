//! A separate straightforward u128 RREF oracle checks every replay state.
#[allow(dead_code)]
#[path = "normalization-bench.rs"]
mod driver;
use driver::{
    decode,
    full::{
        self,
        shared::{price, Field, Widths},
    },
    Matrix,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

fn oracle(m: &Matrix) -> Result<(Vec<Value>, u64), String> {
    let mut basis = BTreeMap::<usize, (Vec<u64>, u64)>::new();
    let mut order = Vec::new();
    let mut states = Vec::new();
    let mut zeros = 0;
    for (i, (input, b)) in m.rows.iter().enumerate() {
        let (mut row, mut rhs) = (input.clone(), *b);
        for (&column, (pivot, pivot_rhs)) in &basis {
            let factor = row[column];
            // This oracle deliberately performs every coefficient operation,
            // without Shoup factors or the production support-list updates.
            for (a, &v) in row.iter_mut().zip(pivot) {
                *a = full::shared::add(*a, m.l - full::shared::mul(factor, v, m.l), m.l);
            }
            rhs = full::shared::add(rhs, m.l - full::shared::mul(factor, *pivot_rhs, m.l), m.l);
        }
        let independent = if let Some(column) = row.iter().position(|&v| v != 0) {
            zeros += row
                .iter()
                .chain(std::iter::once(&rhs))
                .filter(|&&v| v == 0)
                .count() as u64;
            let inv = full::shared::pow(row[column], m.l - 2, m.l);
            for v in &mut row {
                *v = full::shared::mul(*v, inv, m.l);
            }
            rhs = full::shared::mul(rhs, inv, m.l);
            for (pivot, pivot_rhs) in basis.values_mut() {
                let f = pivot[column];
                for (a, &v) in pivot.iter_mut().zip(&row) {
                    *a = full::shared::add(*a, m.l - full::shared::mul(f, v, m.l), m.l);
                }
                *pivot_rhs =
                    full::shared::add(*pivot_rhs, m.l - full::shared::mul(f, rhs, m.l), m.l);
            }
            basis.insert(column, (row, rhs));
            order.push(column);
            true
        } else {
            if rhs != 0 {
                return Err("inconsistent affine system".into());
            }
            false
        };
        let answer = basis.get(&m.target).and_then(|(row, rhs)| {
            row.iter()
                .enumerate()
                .all(|(c, &v)| c == m.target || v == 0)
                .then_some(*rhs)
        });
        let mut h = blake3::Hasher::new();
        h.update(&m.l.to_le_bytes());
        h.update(&(m.columns as u64).to_le_bytes());
        for &column in &order {
            let (row, rhs) = &basis[&column];
            h.update(&(column as u64).to_le_bytes());
            for v in row {
                h.update(&v.to_le_bytes());
            }
            h.update(&rhs.to_le_bytes());
        }
        states.push(json!({"row":i,"rank":basis.len(),"independent":independent,"state_blake3":h.finalize().to_hex().to_string(),"target":answer}));
        if answer.is_some() {
            break;
        }
    }
    Ok((states, zeros))
}
fn qualify(result: &Value, original: &Value, input: &[u8], plan: &Value) -> Result<Value, String> {
    let m = decode(original)?;
    if plan["driver_kind"] != "recorded-zero-normalization-validation"
        || plan["backend"] != "runpod_user_authorized"
        || plan["runpod_compute_type"] != "CPU"
        || plan["cpu_affinity_rule"] != "first_allowed_cpu_bound_before_call"
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|v| v.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["allow_comparative_claims"] != false
        || plan["timeout_seconds"] != 180
        || plan["worker_timeout_seconds"] != 60
        || plan["runpod_lifetime_seconds"] != 900
        || plan["original_source_revision"] != m.original_source
        || plan["expected_scalar"] != m.original_scalar
        || plan["expected_legacy_products"] != m.original_products
        || plan["expected_rank"] != m.original_rank
        || plan["expected_columns"] != m.columns
        || result["schema"] != "cover.rref-normalization-replay/v1"
        || result["source_revision"] != plan["source_revision"]
        || result["original_source_revision"] != m.original_source
        || result["original_raw_blake3"] != blake3::hash(input).to_hex().to_string()
        || result["p"] != m.p
        || result["l"] != m.l
        || result["columns"] != m.columns
        || result["target_column"] != m.target
        || result["rows_available"] != m.rows.len()
        || result["complete_cost"] != false
        || !result["total_bitops"].is_null()
        || result["comparative_eligibility"] != false
        || result["new_full_dlp_trial"] != false
        || result["rho_parity"] != false
        || !result["cost_ratios"].is_null()
    {
        return Err("recorded input/protocol/scope".into());
    }
    let arms = result["arms"].as_array().ok_or("arms")?;
    if arms.len() != 2 {
        return Err("two arms".into());
    }
    let (states, zeros) = oracle(&m)?;
    let rank = states.last().ok_or("no oracle state")?["rank"]
        .as_u64()
        .ok_or("rank")?;
    let sparse: Vec<_> = m
        .rows
        .iter()
        .map(|(row, rhs)| {
            (
                row.iter()
                    .enumerate()
                    .filter(|&(_, &v)| v != 0)
                    .map(|(c, &v)| (c as u64, v))
                    .collect(),
                *rhs,
            )
        })
        .collect();
    let (_, batch_answer) = full::target_solution(&sparse, m.columns, m.l)?;
    if batch_answer != Some(m.original_scalar) || rank != m.original_rank {
        return Err("independent target/rank".into());
    }
    let normalization = rank * (m.columns as u64 + 1);
    let mut outcomes = Vec::new();
    for (i, arm) in arms.iter().enumerate() {
        let c: driver::kernel::Counts =
            serde_json::from_value(arm["counts"].clone()).map_err(|_| "counters")?;
        let products = m.original_products - if i == 1 { zeros } else { 0 };
        let tests = if i == 1 { normalization } else { 0 };
        if arm["mode"]
            != if i == 0 {
                "baseline"
            } else {
                "skip-zero-normalization"
            }
            || arm["states"] != json!(states)
            || arm["rank"] != rank
            || arm["actual_scalar"] != m.original_scalar
            || arm["rows_used"] != states.len()
            || c.products != products
            || c.subtractions != m.original_products - normalization
            || c.inversions != rank
            || c.normalization_products != normalization - if i == 1 { zeros } else { 0 }
            || c.zero_normalizations_observed != zeros
            || c.normalization_tests != tests
            || arm["complete_cost"] != false
            || !arm["total_bitops"].is_null()
        {
            return Err("state/counter conservation".into());
        }
        let primitive = driver::primitives(&c);
        if arm["primitives"] != json!(primitive) {
            return Err("primitive binding".into());
        }
        let widths = Widths {
            field: Field::Prime {
                bits: 64 - m.p.leading_zeros(),
            },
            r_bits: 64 - m.l.leading_zeros(),
        };
        let subtotal = price(&primitive, &widths)?;
        if arm["observed_primitive_subtotal_bitops"] != subtotal.to_string() {
            return Err("repricing".into());
        }
        outcomes.push(json!({"mode":arm["mode"],"counts":c,"primitives":primitive,"observed_primitive_subtotal_bitops":subtotal.to_string(),"recorded_inner_wall_ns_practicality_only":arm["recorded_inner_wall_ns_practicality_only"]}));
    }
    for key in [
        "forward_coefficient_scans",
        "backward_support_entries",
        "pivot_selection_scans",
        "solved_coefficient_scans",
        "factor_constructions",
    ] {
        if arms[0]["counts"][key] != arms[1]["counts"][key] {
            return Err("unexpected work-profile change".into());
        }
    }
    Ok(
        json!({"schema":"cover.rref-normalization-analysis/v1","source_revision":plan["source_revision"],"original_source_revision":m.original_source,
        "original_raw_blake3":result["original_raw_blake3"],"p":m.p,"l":m.l,"columns":m.columns,"rank":rank,"actual_scalar":m.original_scalar,
        "rows_used":states.len(),"rows_available":m.rows.len(),"zero_products_avoided":zeros,"new_zero_tests":normalization,
        "every_insert_state_independently_replayed":true,"independent_batch_target_agrees":true,"outcomes":outcomes,
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null,"new_full_dlp_trial":false,"rho_parity":false,
        "independent_signed_verdict":false,"no_end_to_end_composition":true,"flow":{"planned":1,"attempted":1,"completed":1,"excluded":0,"automatic_retries":0}}),
    )
}
fn read(p: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_reader(File::open(p)?)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: normalization-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in ["exit-code.txt", "replay.exit-code.txt"] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
            return Err("failed/censored replay".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let result = read(&run.join("raw.json"))?;
    let input = fs::read(run.join("input.raw.json"))?;
    if input.len() > 32 * 1024 * 1024 {
        return Err("input bound".into());
    }
    let original: Value = serde_json::from_slice(&input)?;
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 2
        || flow[0]["record_type"] != "execution_start"
        || flow[1]["record_type"] != "execution"
        || flow[1]["exit_code"] != 0
        || flow
            .iter()
            .any(|v| v["source_revision"] != plan["source_revision"])
    {
        return Err("flow".into());
    }
    let summary = qualify(&result, &original, &input, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    let export = json!({"schema":"cover.rref-normalization.raw-export/v1","canonical_schema_conformance_claimed":false,"protocol":plan,"original":original,"result":result,"analysis":summary,"flow":flow});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run.join("raw-export.json"))?;
    serde_json::to_writer(&mut out, &export)?;
    writeln!(out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_oracle_and_conservation_reject_forged_counts_and_states() {
        let original = json!({"schema":"cover.two-large-prime.dlp-worker/v1","source_revision":"1".repeat(40),"method":"two-lp","public":{"p":103,"l":101},"verified":true,"capped":false,"actual_scalar":3,
            "work":{"last_rank":3,"solver_modular_muls":12,"projection_solve_attempts":[{}],"final_projection":{"target_column":2,"certificates":[{"small":[[0,2]],"rhs":0},{"small":[[1,3]],"rhs":6},{"small":[[2,4]],"rhs":12}]}}});
        let input = serde_json::to_vec(&original).unwrap();
        let m = decode(&original).unwrap();
        let plan = json!({"driver_kind":"recorded-zero-normalization-validation","backend":"runpod_user_authorized","runpod_compute_type":"CPU","cpu_affinity_rule":"first_allowed_cpu_bound_before_call","pinned_cpu":"0","runpod_allocation":{"provider":"runpod","compute_type":"CPU","vcpuCount":4,"memoryInGb":8},"rounds":1,"warmup_rounds":0,"automatic_retries":0,"allow_comparative_claims":false,"timeout_seconds":180,"worker_timeout_seconds":60,"runpod_lifetime_seconds":900,"original_source_revision":"1".repeat(40),"expected_scalar":3,"expected_legacy_products":12,"expected_rank":3,"expected_columns":3,"source_revision":"2".repeat(40)});
        let raw = json!({"schema":"cover.rref-normalization-replay/v1","source_revision":plan["source_revision"],"original_source_revision":m.original_source,"original_raw_blake3":blake3::hash(&input).to_hex().to_string(),"p":m.p,"l":m.l,"columns":m.columns,"target_column":m.target,"rows_available":3,"arms":[driver::replay::<false>(m.p,m.l,m.columns,m.target,&m.rows).unwrap(),driver::replay::<true>(m.p,m.l,m.columns,m.target,&m.rows).unwrap()],"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"new_full_dlp_trial":false,"rho_parity":false,"cost_ratios":null});
        assert_eq!(
            qualify(&raw, &original, &input, &plan).unwrap()["zero_products_avoided"],
            7
        );
        for key in [
            "products",
            "subtractions",
            "inversions",
            "normalization_tests",
        ] {
            let mut bad = raw.clone();
            let value = bad["arms"][1]["counts"][key].as_u64().unwrap();
            bad["arms"][1]["counts"][key] = json!(value + 1);
            assert!(qualify(&bad, &original, &input, &plan).is_err());
        }
        let mut bad = raw.clone();
        bad["arms"][1]["states"][0]["state_blake3"] = json!("forged");
        assert!(qualify(&bad, &original, &input, &plan).is_err());
        let mut bad = raw;
        bad["complete_cost"] = json!(true);
        assert!(qualify(&bad, &original, &input, &plan).is_err());
    }
    #[test]
    fn oracle_preserves_state_when_pivots_arrive_out_of_column_order() {
        let m = Matrix {
            p: 103,
            l: 101,
            columns: 4,
            target: 3,
            rows: vec![
                (vec![0, 0, 2, 3], 47),
                (vec![4, 1, 0, 2], 39),
                (vec![0, 3, 1, 1], 33),
                (vec![1, 2, 3, 4], 78),
            ],
            original_source: "1".repeat(40),
            original_scalar: 11,
            original_products: 0,
            original_rank: 4,
        };
        let (states, zeros) = oracle(&m).unwrap();
        for raw in [
            driver::replay::<false>(m.p, m.l, m.columns, m.target, &m.rows).unwrap(),
            driver::replay::<true>(m.p, m.l, m.columns, m.target, &m.rows).unwrap(),
        ] {
            assert_eq!(raw["states"], json!(states));
            assert_eq!(raw["actual_scalar"], 11);
            assert_eq!(raw["counts"]["zero_normalizations_observed"], zeros);
        }
    }
}
