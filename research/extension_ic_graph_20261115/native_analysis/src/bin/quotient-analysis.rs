//! Independent column-keyed/u128 oracle for every reused-basis component.
#[allow(dead_code)]
#[path = "quotient-bench.rs"]
pub(crate) mod driver;
use driver::saved::{
    full::shared::{self, n, price, Field, Widths, KINDS},
    kernel::Counts,
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
pub(crate) struct Oracle {
    pub(crate) basis: BTreeMap<usize, (Vec<u64>, u64)>,
    order: Vec<usize>,
    pub(crate) counts: Counts,
    pub(crate) states: Vec<Value>,
    pub(crate) answer: Option<u64>,
}
impl Oracle {
    fn digest(&self, l: u64, columns: usize) -> String {
        let mut h = blake3::Hasher::new();
        h.update(&l.to_le_bytes());
        h.update(&(columns as u64).to_le_bytes());
        for c in &self.order {
            let (row, rhs) = &self.basis[c];
            h.update(&(*c as u64).to_le_bytes());
            for v in row {
                h.update(&v.to_le_bytes());
            }
            h.update(&rhs.to_le_bytes());
        }
        h.finalize().to_hex().to_string()
    }
}
pub(crate) fn oracle(
    l: u64,
    columns: usize,
    rows: &[(Vec<u64>, u64)],
    target: Option<usize>,
) -> Result<Oracle, String> {
    let mut o = Oracle {
        basis: BTreeMap::new(),
        order: Vec::new(),
        counts: Counts::default(),
        states: Vec::new(),
        answer: None,
    };
    for (i, (input, b)) in rows.iter().enumerate() {
        if input.len() != columns || input.iter().any(|&v| v >= l) || *b >= l {
            return Err("oracle canonical row".into());
        }
        let (mut row, mut rhs) = (input.clone(), *b);
        for (&col, (pivot, pivot_rhs)) in &o.basis {
            let f = row[col];
            if f != 0 {
                o.counts.factor_constructions += 1;
                o.counts.forward_coefficient_scans += columns as u64;
                for (a, &v) in row.iter_mut().zip(pivot) {
                    *a = shared::add(*a, l - shared::mul(f, v, l), l);
                    if v != 0 {
                        o.counts.products += 1;
                        o.counts.subtractions += 1;
                    }
                }
                rhs = shared::add(rhs, l - shared::mul(f, *pivot_rhs, l), l);
                o.counts.products += 1;
                o.counts.subtractions += 1;
            }
        }
        let col = row.iter().position(|&v| v != 0);
        o.counts.pivot_selection_scans += col.map_or(columns, |c| c + 1) as u64;
        let independent = if let Some(col) = col {
            let inv = shared::pow(row[col], l - 2, l);
            o.counts.inversions += 1;
            o.counts.factor_constructions += 1;
            for v in row.iter_mut().chain(std::iter::once(&mut rhs)) {
                if *v == 0 {
                    o.counts.zero_normalizations_observed += 1;
                }
                *v = shared::mul(*v, inv, l);
                o.counts.products += 1;
                o.counts.normalization_products += 1;
            }
            for (pivot, pivot_rhs) in o.basis.values_mut() {
                let f = pivot[col];
                if f != 0 {
                    o.counts.factor_constructions += 1;
                    for (a, &v) in pivot.iter_mut().zip(&row) {
                        *a = shared::add(*a, l - shared::mul(f, v, l), l);
                        if v != 0 {
                            o.counts.products += 1;
                            o.counts.subtractions += 1;
                            o.counts.backward_support_entries += 1;
                        }
                    }
                    *pivot_rhs = shared::add(*pivot_rhs, l - shared::mul(f, rhs, l), l);
                    o.counts.products += 1;
                    o.counts.subtractions += 1;
                }
            }
            o.basis.insert(col, (row, rhs));
            o.order.push(col);
            true
        } else {
            if rhs != 0 {
                return Err("inconsistent oracle row".into());
            }
            false
        };
        if let Some(target) = target {
            o.answer = o.basis.get(&target).and_then(|(row, rhs)| {
                for (c, &v) in row.iter().enumerate() {
                    o.counts.solved_coefficient_scans += 1;
                    if c != target && v != 0 {
                        return None;
                    }
                }
                Some(*rhs)
            });
            o.states.push(json!({"row":i,"rank":o.basis.len(),"independent":independent,"state_blake3":o.digest(l,columns),"target":o.answer}));
            if o.answer.is_some() {
                break;
            }
        }
    }
    Ok(o)
}
fn check_rref(raw: &Value, o: &Oracle, l: u64, columns: usize, affine: bool) -> Result<(), String> {
    if raw["rank"] != o.basis.len()
        || raw["counts"] != serde_json::to_value(&o.counts).map_err(|_| "counts")?
    {
        return Err("independent RREF rank/arithmetic/scan counters".into());
    }
    if affine {
        if raw["states"] != json!(o.states) {
            return Err("per-insert independent affine states".into());
        }
    } else if raw["state_blake3"] != o.digest(l, columns) {
        return Err("homogeneous state".into());
    }
    Ok(())
}
fn check_math(raw: &Value, m: &Matrix, h_rows: &[(Vec<u64>, u64)]) -> Result<Value, String> {
    let (l, ncols) = (m.l, m.target);
    if raw["p"] != m.p
        || raw["l"] != l
        || raw["columns"] != m.columns
        || raw["target_column"] != m.target
        || raw["rows_available"] != m.rows.len()
        || raw["homogeneous_rows"] != h_rows.len()
        || raw["complete_cost"] != false
        || !raw["total_bitops"].is_null()
        || raw["comparative_eligibility"] != false
        || !raw["cost_ratios"].is_null()
    {
        return Err("component identity/coverage".into());
    }
    let hom = oracle(l, ncols, h_rows, None)?;
    if hom.basis.len() + 1 != ncols {
        return Err("independent homogeneous codimension".into());
    }
    for mode in ["baseline", "candidate"] {
        check_rref(&raw[mode]["homogeneous"], &hom, l, ncols, false)?;
    }
    let old = oracle(l, m.columns, &m.rows, Some(m.target))?;
    check_rref(&raw["baseline"]["affine"], &old, l, m.columns, true)?;
    if old.answer != Some(m.original_scalar)
        || old.counts.products != m.original_products
        || old.basis.len() as u64 != m.original_rank
        || raw["baseline"]["actual_scalar"] != m.original_scalar
    {
        return Err("recorded baseline target/arithmetic".into());
    }
    let free = (0..ncols)
        .find(|c| !hom.basis.contains_key(c))
        .ok_or("free coordinate")?;
    let mut h = vec![0; ncols];
    h[free] = 1;
    let mut negations = 0;
    for (&c, (row, rhs)) in &hom.basis {
        if *rhs != 0 {
            return Err("homogeneous RHS".into());
        }
        h[c] = if row[free] == 0 {
            0
        } else {
            negations += 1;
            l - row[free]
        };
    }
    for (row, rhs) in h_rows {
        let dot = row
            .iter()
            .zip(&h)
            .fold(0, |a, (&v, &b)| shared::add(a, shared::mul(v, b, l), l));
        if dot != 0 || *rhs != 0 {
            return Err("kernel equation".into());
        }
    }
    let basis = &raw["candidate"]["basis"];
    let initial_words = 4 * ncols as u64
        + 1
        + 8 * hom.basis.len() as u64
        + 4 * hom.basis.len() as u64 * ncols as u64;
    let basis_counts = json!({"products":0,"additions":negations,"word_events":initial_words});
    if basis["kernel"] != json!(h)
        || basis["free_column"] != free
        || basis["modulus"] != l
        || basis["counts"] != basis_counts
    {
        return Err("unique normalized kernel/extraction events".into());
    }
    let mut projected = Vec::new();
    let mut arithmetic = Vec::new();
    for (i, (row, rhs)) in m.rows.iter().enumerate() {
        let dot = row[..ncols]
            .iter()
            .zip(&h)
            .fold(0, |a, (&v, &b)| shared::add(a, shared::mul(v, b, l), l));
        projected.push(json!({"row":i,"coefficients":[dot,row[m.target]],"rhs":rhs}));
        arithmetic.push((vec![dot, row[m.target]], *rhs));
    }
    let new = oracle(l, 2, &arithmetic, Some(1))?;
    check_rref(&raw["candidate"]["affine"], &new, l, 2, true)?;
    if new.answer != Some(m.original_scalar)
        || raw["candidate"]["actual_scalar"] != m.original_scalar
        || raw["candidate"]["projected_rows"] != json!(&projected[..new.states.len()])
    {
        return Err("affine projection/target".into());
    }
    let mut dots = 0u64;
    for (row, _) in &m.rows[..new.states.len()] {
        dots += row[..ncols]
            .iter()
            .zip(&h)
            .filter(|&(a, b)| *a != 0 && *b != 0)
            .count() as u64;
    }
    let extra = json!({"products":dots,"additions":dots+negations,"word_events":initial_words+(5*ncols as u64+4)*new.states.len() as u64});
    if raw["candidate"]["extra_counts"] != extra {
        return Err("projection arithmetic/new event conservation".into());
    }
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - m.p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let mut prices = Vec::new();
    for (mode, affine, extra) in [
        (
            "baseline",
            &old,
            json!({"products":0,"additions":0,"word_events":0}),
        ),
        ("candidate", &new, extra),
    ] {
        let mut counts: BTreeMap<String, u64> = KINDS.into_iter().map(|k| (k.into(), 0)).collect();
        counts.insert(
            "modr_mul".into(),
            hom.counts.products + affine.counts.products + n(&extra, "products")?,
        );
        counts.insert(
            "modr_add".into(),
            hom.counts.subtractions + affine.counts.subtractions + n(&extra, "additions")?,
        );
        counts.insert(
            "modr_inv".into(),
            hom.counts.inversions + affine.counts.inversions,
        );
        counts.insert("word_op".into(), n(&extra, "word_events")?);
        let priced = price(&counts, &widths)?.to_string();
        if raw[mode]["observed_primitives"] != json!(counts)
            || raw[mode]["observed_incomplete_bitops"] != priced
        {
            return Err("independent primitive conservation/repricing".into());
        }
        prices.push(
            json!({"mode":mode,"observed_primitives":counts,"observed_incomplete_bitops":priced}),
        );
    }
    Ok(
        json!({"columns":m.columns,"homogeneous_rank":hom.basis.len(),"homogeneous_rows":h_rows.len(),"baseline_rows_used":old.states.len(),"candidate_rows_used":new.states.len(),"baseline_affine_rank":old.basis.len(),"candidate_affine_rank":new.basis.len(),"actual_scalar":m.original_scalar,
        "prices":prices,"all_affine_prefix_states_and_counters_checked":true,"independent_kernel_and_projection_checked":true,"known_new_event_conservation_checked":true,"complete_cost":false,"total_bitops":null,"cost_ratios":null}),
    )
}
fn protocol(plan: &Value) -> Result<(), String> {
    let frozen: Value = serde_json::from_str(include_str!("../../../quotient-plan.json"))
        .map_err(|_| "compiled plan")?;
    for (key, value) in frozen.as_object().ok_or("plan")? {
        if key != "source_revision" && plan[key] != *value {
            return Err(format!("frozen quotient protocol {key}"));
        }
    }
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|s| s.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
    {
        return Err("source/CPU binding".into());
    }
    Ok(())
}
fn read(path: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(path)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB".into());
    }
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn write_new(path: &Path, v: &Value, pretty: bool) -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    if pretty {
        serde_json::to_writer_pretty(&mut f, v)?;
    } else {
        serde_json::to_writer(&mut f, v)?;
    }
    writeln!(f)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: quotient-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in ["exit-code.txt", "replay.exit-code.txt"] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
            return Err("failed/censored replay retained".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    protocol(&plan)?;
    let result = read(&run.join("raw.json"))?;
    if result["schema"] != "cover.homogeneous-quotient-replay/v1"
        || result["source_revision"] != plan["source_revision"]
        || result["complete_cost"] != false
        || !result["total_bitops"].is_null()
        || result["comparative_eligibility"] != false
        || !result["cost_ratios"].is_null()
        || result["new_dlp_sample"] != false
        || result["reprices_prior_pipeline"] != false
        || result["known_uncounted"]
            .as_array()
            .is_none_or(|v| v.is_empty())
    {
        return Err("record scope/identity".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 2 {
        return Err("flow length".into());
    }
    for (i, row) in flow.iter().enumerate() {
        if row["source_revision"] != plan["source_revision"]
            || row["mode"] != "quotient-replay"
            || row["record_type"]
                != if i == 0 {
                    "execution_start"
                } else {
                    "execution"
                }
            || (i == 1 && row["exit_code"] != 0)
        {
            return Err("flow identity/status".into());
        }
    }
    let outcomes = result["outcomes"].as_array().ok_or("outcomes")?;
    if outcomes.len() != 6 {
        return Err("all six recorded arms required".into());
    }
    let mut checked = Vec::new();
    let mut inputs = Vec::new();
    for (mode, row) in driver::MODES.iter().zip(outcomes) {
        let bytes = fs::read(run.join("originals").join(format!("{mode}.raw.json")))?;
        let original: Value = serde_json::from_slice(&bytes)?;
        let input = driver::decode(&original)?;
        let m = &input.matrix;
        if row["arm"] != *mode
            || row["original_source_revision"] != plan["original_source_revision"]
            || m.original_source != plan["original_source_revision"]
            || row["original_input_blake3"] != blake3::hash(&bytes).to_hex().to_string()
            || row["original_scalar_reference"] != m.original_scalar
            || plan["expected_scalar"] != m.original_scalar
            || plan["p"] != m.p
            || plan["l"] != m.l
        {
            return Err("original input/source/reference binding".into());
        }
        let mut proof = check_math(row, m, &input.homogeneous)?;
        if row["baseline"]["homogeneous"]["counts"]["products"] != input.expected_hom_products
            || row["baseline"]["homogeneous"]["rank"] != input.expected_hom_rank
        {
            return Err("saved checkpoint arithmetic/rank".into());
        }
        proof["arm"] = json!(mode);
        checked.push(proof);
        inputs.push(original);
    }
    let summary = json!({"schema":"cover.homogeneous-quotient-analysis/v1","source_revision":plan["source_revision"],"original_source_revision":plan["original_source_revision"],"p":plan["p"],"l":plan["l"],"actual_scalar":plan["expected_scalar"],"outcomes":checked,
        "flow":{"planned_paired_replays":6,"attempted_paired_replays":6,"completed_paired_replays":6,"excluded":0,"automatic_retries":0},"new_dlp_sample":false,"reprices_prior_pipeline":false,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false});
    write_new(Path::new(&args[2]), &summary, true)?;
    let export = json!({"schema":"cover.homogeneous-quotient.raw-export/v1","canonical_schema_conformance_claimed":false,"canonical_experiment_export_schema_available":false,"protocol":plan,"originals":inputs,"worker":result,"analysis":summary,"flow":flow});
    write_new(&run.join("raw-export.json"), &export, false)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_dense_oracle_matches_all_states_and_semantic_counters() {
        let mut rng = 0xd116_0938_7ef4_a321u64;
        for l in [3, 101, (1u64 << 61) - 1, (1u64 << 63) + 29] {
            for _ in 0..24 {
                let columns = 7;
                let solution = [1, 2, 0, 1, 0, 2, 1];
                let mut rows = Vec::new();
                for i in 0..18 {
                    let row: Vec<_> = (0..columns)
                        .map(|j| {
                            rng ^= rng << 13;
                            rng ^= rng >> 7;
                            rng ^= rng << 17;
                            if i == 0 {
                                u64::from(j == 3)
                            } else if rng & 3 == 0 {
                                0
                            } else {
                                rng % l
                            }
                        })
                        .collect();
                    let rhs = row
                        .iter()
                        .zip(solution)
                        .fold(0, |a, (&v, b)| shared::add(a, shared::mul(v, b, l), l));
                    rows.push((row, rhs));
                }
                let expected = oracle(l, columns, &rows, Some(6)).unwrap();
                let mut actual = driver::saved::kernel::Rref::<false>::new(l, columns);
                let mut states = Vec::new();
                for (i, (row, rhs)) in rows.iter().enumerate() {
                    let independent = actual.insert(row.clone(), *rhs).unwrap();
                    let target = actual.solved(6);
                    states.push(json!({"row":i,"rank":actual.rank(),"independent":independent,"state_blake3":actual.state_digest(),"target":target}));
                    if target.is_some() {
                        break;
                    }
                }
                assert_eq!(actual.counts, expected.counts);
                assert_eq!(states, expected.states);
            }
        }
    }
    fn hand() -> (Value, Matrix, Vec<(Vec<u64>, u64)>) {
        let rows = vec![(vec![1, 2, 0], 0), (vec![3, 4, 1], 17), (vec![1, 0, 2], 9)];
        let hom = vec![(vec![1, 2], 0)];
        let raw = driver::pair(101, 3, 2, &hom, &rows, 103).unwrap();
        let m = Matrix {
            p: 103,
            l: 101,
            columns: 3,
            target: 2,
            rows,
            original_source: "a".repeat(40),
            original_scalar: 93,
            original_products: raw["baseline"]["affine"]["counts"]["products"]
                .as_u64()
                .unwrap(),
            original_rank: 3,
        };
        (raw, m, hom)
    }
    #[test]
    fn independent_hand_replay_and_counter_kernel_projection_mutants() {
        let (raw, m, hom) = hand();
        assert!(check_math(&raw, &m, &hom).is_ok());
        for mode in ["baseline", "candidate"] {
            for key in [
                "products",
                "subtractions",
                "inversions",
                "normalization_products",
                "zero_normalizations_observed",
                "normalization_tests",
                "forward_coefficient_scans",
                "backward_support_entries",
                "pivot_selection_scans",
                "solved_coefficient_scans",
                "factor_constructions",
            ] {
                let mut bad = raw.clone();
                let value = bad[mode]["affine"]["counts"][key].as_u64().unwrap();
                bad[mode]["affine"]["counts"][key] = json!(value + 1);
                assert!(check_math(&bad, &m, &hom).is_err());
            }
        }
        for pointer in [
            "/candidate/basis/kernel/0",
            "/candidate/basis/counts/word_events",
            "/candidate/projected_rows/0/coefficients/0",
            "/candidate/extra_counts/additions",
            "/candidate/actual_scalar",
        ] {
            let mut bad = raw.clone();
            let field = bad.pointer_mut(pointer).unwrap();
            *field = json!(field.as_u64().unwrap() + 1);
            assert!(check_math(&bad, &m, &hom).is_err());
        }
    }
    #[test]
    fn frozen_plan_does_not_accept_input_or_limit_changes() {
        let mut p: Value =
            serde_json::from_str(include_str!("../../../quotient-plan.json")).unwrap();
        p["source_revision"] = json!("a".repeat(40));
        p["pinned_cpu"] = json!("0");
        p["runpod_allocation"] =
            json!({"provider":"runpod","compute_type":"CPU","vcpuCount":4,"memoryInGb":8});
        assert!(protocol(&p).is_ok());
        for key in ["worker_timeout_seconds", "expected_scalar"] {
            let mut bad = p.clone();
            bad[key] = json!(1);
            assert!(protocol(&bad).is_err());
        }
        p["input_sha256"]["s85-r32"] = json!("b".repeat(64));
        assert!(protocol(&p).is_err());
    }
}
