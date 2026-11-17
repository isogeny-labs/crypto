//! Prospective allocation and fail-closed readiness checks. This program runs no solvers.
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

type Result<T> = std::result::Result<T, String>;
const INPUT: &str = "research/conductor_gap_size_ladder_20261026/results-v2/candidates.csv";
const INPUT_SHA: &str = "1f7a8c3789c8acb46fe5407e3a23e6f227026c34d82513b42bc28951167d7f5f";
const DOMAIN: &str = "conductor-gap/pilot-allocation/v1";
const TARGETS: usize = 32;
const SLOTS: usize = 4;
const REPLICATES: usize = 2;
const FIELDS: [u32; 4] = [40, 47, 54, 61];
const FAMILIES: [u32; 4] = [11, 19, 43, 67];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Class {
    parameter_id: String,
    field_bits: u32,
    abs_d: u32,
    depth: u32,
    p: u64,
    t: i64,
    ell: u64,
    r: u64,
    cofactor: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Target {
    target_index: usize,
    scalar: u64,
    // Slot 0 is prospective S1; slots 1..=3 are prospective distinct S2 endpoints.
    solver_seeds: [[u64; REPLICATES]; SLOTS],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Allocation {
    class: Class,
    targets: Vec<Target>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
struct Block {
    parameter_id: String,
    crater_slot: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    schema: String,
    phase: String,
    input_sha256: String,
    derivation: String,
    execution_ready: bool,
    allocations: Vec<Allocation>,
    block_order: Vec<Block>,
}

fn require(ok: bool, message: impl Into<String>) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn cohort(path: &Path) -> Result<Vec<Class>> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    require(
        digest(&bytes) == INPUT_SHA,
        "frozen candidate CSV hash mismatch",
    )?;
    let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
    let mut rows = text.lines();
    require(
        rows.next() == Some("kind,field_bits,abs_d,depth,p,t,ell,r,cofactor,ell_factor_exponent,ell_witness,p_witness,r_witness"),
        "unexpected candidate CSV header",
    )?;
    let mut classes = Vec::new();
    let mut upper = 0;
    let mut cells = BTreeMap::new();
    let mut keys = BTreeSet::new();
    for row in rows {
        let c: Vec<&str> = row.split(',').collect();
        require(c.len() == 13, "malformed candidate CSV row")?;
        if c[0] == "upper" {
            upper += 1;
            continue;
        }
        require(c[0] == "native", "unknown candidate kind")?;
        let number = |i: usize| c[i].parse::<u64>().map_err(|e| format!("CSV integer: {e}"));
        let p = number(4)?;
        let t = c[5].parse::<i64>().map_err(|e| e.to_string())?;
        let class = Class {
            parameter_id: format!("parameters/p{p}/t{t}"),
            field_bits: u32::try_from(number(1)?).map_err(|e| e.to_string())?,
            abs_d: u32::try_from(number(2)?).map_err(|e| e.to_string())?,
            depth: u32::try_from(number(3)?).map_err(|e| e.to_string())?,
            p,
            t,
            ell: number(6)?,
            r: number(7)?,
            cofactor: number(8)?,
        };
        require(keys.insert((p, t)), "duplicate parameter class")?;
        require(
            FIELDS.contains(&class.field_bits)
                && FAMILIES.contains(&class.abs_d)
                && class.depth == 4
                && class.p.ilog2() + 1 == class.field_bits
                && class.r.ilog2() + 1 >= class.field_bits - 6
                && class.cofactor <= 64,
            "class outside the frozen cohort",
        )?;
        require(
            i128::from(p) + 1 - i128::from(t) == i128::from(class.r) * i128::from(class.cofactor),
            "parameter order/cofactor mismatch",
        )?;
        *cells.entry((class.field_bits, class.abs_d)).or_insert(0) += 1;
        classes.push(class);
    }
    require(
        classes.len() == 128 && upper == 6,
        "incomplete candidate cohort",
    )?;
    require(
        cells.len() == 16 && cells.values().all(|&n| n == 8),
        "wrong family/rung allocation",
    )?;
    classes.sort_by_key(|c| (c.field_bits, c.abs_d, c.p, c.t));
    Ok(classes)
}

// Length-prefix strings and fixed-width counters make the domain encoding unambiguous.
fn draw(purpose: &str, class_id: &str, counters: &[u64]) -> [u8; 32] {
    let mut h = Sha256::new();
    for s in [DOMAIN, INPUT_SHA, purpose, class_id] {
        h.update((s.len() as u64).to_le_bytes());
        h.update(s.as_bytes());
    }
    h.update((counters.len() as u64).to_le_bytes());
    for n in counters {
        h.update(n.to_le_bytes());
    }
    h.finalize().into()
}

fn word(purpose: &str, class: &Class, counters: &[u64]) -> u64 {
    u64::from_le_bytes(
        draw(purpose, &class.parameter_id, counters)[..8]
            .try_into()
            .unwrap(),
    )
}

fn target_scalar(class: &Class, index: usize, used: &BTreeSet<u64>) -> Result<u64> {
    let bound = class.r - 1;
    let space = 1_u128 << 64;
    let limit = space - space % u128::from(bound);
    for attempt in 0..1_000_000 {
        let x = word("pilot-target", class, &[index as u64, attempt]);
        if u128::from(x) < limit {
            let scalar = 1 + x % bound;
            if !used.contains(&scalar) {
                return Ok(scalar);
            }
        }
    }
    Err("target derivation budget exhausted".into())
}

fn solver_seed(
    class: &Class,
    index: usize,
    slot: usize,
    replicate: usize,
    used: &BTreeSet<u64>,
) -> Result<u64> {
    for attempt in 0..1_000_000 {
        let seed = word(
            "pilot-solver",
            class,
            &[index as u64, slot as u64, replicate as u64, attempt],
        );
        if seed != 0 && !used.contains(&seed) {
            return Ok(seed);
        }
    }
    Err("solver-seed derivation budget exhausted".into())
}

fn block_order(classes: &[Class]) -> Vec<Block> {
    let mut blocks: Vec<_> = classes
        .iter()
        .flat_map(|class| {
            (0..SLOTS).map(move |slot| Block {
                parameter_id: class.parameter_id.clone(),
                crater_slot: slot,
            })
        })
        .collect();
    blocks.sort_by_key(|b| {
        (
            draw(
                "pilot-block-order",
                &b.parameter_id,
                &[b.crater_slot as u64],
            ),
            b.clone(),
        )
    });
    blocks
}

fn plan(classes: &[Class]) -> Result<Plan> {
    let mut seeds = BTreeSet::new();
    let mut allocations = Vec::new();
    for class in classes {
        let mut scalars = BTreeSet::new();
        let mut targets = Vec::new();
        for index in 0..TARGETS {
            let scalar = target_scalar(class, index, &scalars)?;
            scalars.insert(scalar);
            let mut solver_seeds = [[0; REPLICATES]; SLOTS];
            for (slot, pair) in solver_seeds.iter_mut().enumerate() {
                for (replicate, seed) in pair.iter_mut().enumerate() {
                    *seed = solver_seed(class, index, slot, replicate, &seeds)?;
                    seeds.insert(*seed);
                }
            }
            targets.push(Target {
                target_index: index,
                scalar,
                solver_seeds,
            });
        }
        allocations.push(Allocation {
            class: class.clone(),
            targets,
        });
    }
    Ok(Plan {
        schema: "conductor-gap.pilot-allocation/v1".into(),
        phase: "excluded-pilot-proposal".into(),
        input_sha256: INPUT_SHA.into(),
        derivation: DOMAIN.into(),
        execution_ready: false,
        allocations,
        block_order: block_order(classes),
    })
}

fn audit(plan: &Plan, classes: &[Class]) -> Result<serde_json::Value> {
    require(
        plan.schema == "conductor-gap.pilot-allocation/v1"
            && plan.phase == "excluded-pilot-proposal"
            && plan.input_sha256 == INPUT_SHA
            && plan.derivation == DOMAIN
            && !plan.execution_ready,
        "invalid provenance or unsupported execution claim",
    )?;
    require(
        plan.allocations.len() == classes.len(),
        "missing or extra classes",
    )?;
    let mut seed_set = BTreeSet::new();
    let mut target_count = 0;
    let mut invocation_count = 0;
    for (allocation, expected_class) in plan.allocations.iter().zip(classes) {
        require(
            allocation.class == *expected_class,
            "class identity/cohort order mismatch",
        )?;
        require(
            allocation.targets.len() == TARGETS,
            "wrong target count in class",
        )?;
        let mut scalar_set = BTreeSet::new();
        for (index, target) in allocation.targets.iter().enumerate() {
            require(
                target.target_index == index,
                "target indices must be contiguous",
            )?;
            require(
                target.scalar > 0 && target.scalar < expected_class.r,
                "target scalar outside the prime subgroup",
            )?;
            require(!scalar_set.contains(&target.scalar), "reused target scalar")?;
            require(
                target.scalar == target_scalar(expected_class, index, &scalar_set)?,
                "target does not match frozen derivation",
            )?;
            scalar_set.insert(target.scalar);
            target_count += 1;
            for slot in 0..SLOTS {
                for replicate in 0..REPLICATES {
                    let seed = target.solver_seeds[slot][replicate];
                    require(
                        seed != 0 && !seed_set.contains(&seed),
                        "zero or reused solver seed",
                    )?;
                    require(
                        seed == solver_seed(expected_class, index, slot, replicate, &seed_set)?,
                        "solver seed does not match frozen derivation",
                    )?;
                    seed_set.insert(seed);
                    invocation_count += 1;
                }
            }
        }
    }
    let expected_blocks: BTreeSet<_> = classes
        .iter()
        .flat_map(|c| {
            (0..SLOTS).map(move |slot| Block {
                parameter_id: c.parameter_id.clone(),
                crater_slot: slot,
            })
        })
        .collect();
    let actual_blocks: BTreeSet<_> = plan.block_order.iter().cloned().collect();
    require(
        plan.block_order.len() == expected_blocks.len() && actual_blocks == expected_blocks,
        "missing or duplicate class/crater block",
    )?;
    require(
        plan.block_order == block_order(classes),
        "block order was not frozen derivation",
    )?;
    Ok(json!({
        "schema":"conductor-gap.allocation-audit/v1", "allocation_valid":true,
        "execution_ready":false, "input_sha256":INPUT_SHA,
        "parameter_classes":classes.len(), "distinct_class_target_pairs":target_count,
        "crater_slots_per_class":SLOTS, "solver_seeds_per_target_slot":REPLICATES,
        "prospective_primary_invocations":invocation_count,
        "prospective_full_replay_invocations":invocation_count,
        "class_crater_blocks":expected_blocks.len(),
        "solver_invocations_attempted":0, "controls_allocated":false,
        "confirmation_count_frozen":false, "measured_equivalence":"not_established"
    }))
}

fn readiness(audit: serde_json::Value) -> serde_json::Value {
    json!({
        "schema":"conductor-gap.preflight/v1", "status":"blocked", "allocation":audit,
        "execution_ready":false, "solver_invocations_attempted":0,
        "missing":[
            "Exact S1/three distinct S2 models, orders, strata and ICV1 identities",
            "Certified map/dual and common G/Q transport with independent scalar checks",
            "Resumable native measurement executor and independent complete replay",
            "Frozen same-instrument A/A, coordinate and plain-rho controls",
            "Pinned protocol/export, analysis masking and cost-accounting boundaries",
            "Authorized resource feasibility and backend/isolation admission",
            "Disjoint pilot, coverage/variance/joint-power calibration and final confirmation count"
        ]
    })
}

fn save_new(path: &Path, plan: &Plan) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(plan).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("create new {}: {e}", path.display()))?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

fn run(args: &[String]) -> Result<i32> {
    if args.is_empty() || args == ["--help"] || args == ["help"] {
        println!(
            "Native prospective allocation only; no solver execution.\n\
            plan [--input CSV] --out NEW_JSON\n\
            audit [--input CSV] --plan JSON\n\
            preflight [--input CSV] --plan JSON   (exit 3: execution blocked)"
        );
        return Ok(0);
    }
    let command = args[0].as_str();
    require(
        ["plan", "audit", "preflight"].contains(&command),
        "unknown command; no execution mode exists",
    )?;
    let mut options = BTreeMap::new();
    let mut tail = args[1..].chunks_exact(2);
    for pair in &mut tail {
        require(
            ["--input", "--out", "--plan"].contains(&pair[0].as_str()),
            "unknown option",
        )?;
        require(
            options.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate option",
        )?;
    }
    require(tail.remainder().is_empty(), "option needs a value")?;
    require(
        if command == "plan" {
            !options.contains_key("--plan")
        } else {
            !options.contains_key("--out")
        },
        "option not applicable to this command",
    )?;
    let classes = cohort(Path::new(options.get("--input").copied().unwrap_or(INPUT)))?;
    if command == "plan" {
        let out = options.get("--out").ok_or("plan requires --out")?;
        let plan = plan(&classes)?;
        let mut receipt = audit(&plan, &classes)?;
        let bytes = save_new(Path::new(out), &plan)?;
        receipt["allocation_sha256"] = digest(&bytes).into();
        receipt["allocation_bytes"] = bytes.len().into();
        println!(
            "{}",
            serde_json::to_string_pretty(&receipt).map_err(|e| e.to_string())?
        );
    } else {
        let path = options
            .get("--plan")
            .ok_or("audit/preflight requires --plan")?;
        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        let parsed: Plan = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        let mut receipt = audit(&parsed, &classes)?;
        receipt["allocation_sha256"] = digest(&bytes).into();
        receipt["allocation_bytes"] = bytes.len().into();
        if command == "preflight" {
            println!(
                "{}",
                serde_json::to_string_pretty(&readiness(receipt)).map_err(|e| e.to_string())?
            );
            return Ok(3);
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&receipt).map_err(|e| e.to_string())?
        );
    }
    Ok(0)
}

fn main() {
    match run(&std::env::args().skip(1).collect::<Vec<_>>()) {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("conductor-gap preflight: {error}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn fixture() -> (&'static Vec<Class>, &'static Plan) {
        static FIXTURE: OnceLock<(Vec<Class>, Plan)> = OnceLock::new();
        let (classes, plan) = FIXTURE.get_or_init(|| {
            let classes = cohort(Path::new(INPUT)).unwrap();
            let allocation = plan(&classes).unwrap();
            (classes, allocation)
        });
        (classes, plan)
    }

    #[test]
    fn full_cohort_allocation_is_not_an_executed_pilot() {
        let (classes, plan) = fixture();
        let receipt = audit(plan, classes).unwrap();
        assert_eq!(receipt["parameter_classes"], 128);
        assert_eq!(receipt["distinct_class_target_pairs"], 4096);
        assert_eq!(receipt["prospective_primary_invocations"], 32768);
        assert_eq!(receipt["class_crater_blocks"], 512);
        assert_eq!(receipt["solver_invocations_attempted"], 0);
        assert_eq!(receipt["execution_ready"], false);
        assert_eq!(readiness(receipt)["status"], "blocked");
    }

    #[test]
    fn replaying_a_scalar_cannot_add_a_target() {
        let (classes, original) = fixture();
        let mut bad = original.clone();
        bad.allocations[0].targets[1].scalar = bad.allocations[0].targets[0].scalar;
        assert!(audit(&bad, classes).unwrap_err().contains("reused target"));
    }

    #[test]
    fn out_of_range_and_post_selected_targets_are_rejected() {
        let (classes, original) = fixture();
        for scalar in [
            0,
            classes[0].r,
            original.allocations[0].targets[0].scalar + 1,
        ] {
            let mut bad = original.clone();
            bad.allocations[0].targets[0].scalar = scalar;
            assert!(audit(&bad, classes).is_err());
        }
    }

    #[test]
    fn seeds_cannot_be_shared_between_curves_or_replicates() {
        let (classes, original) = fixture();
        for (slot, replicate) in [(0, 1), (1, 0)] {
            let mut bad = original.clone();
            bad.allocations[0].targets[0].solver_seeds[slot][replicate] =
                bad.allocations[0].targets[0].solver_seeds[0][0];
            assert!(audit(&bad, classes)
                .unwrap_err()
                .contains("reused solver seed"));
        }
    }

    #[test]
    fn zero_post_selected_and_cross_class_seeds_are_rejected() {
        let (classes, original) = fixture();
        for seed in [0, original.allocations[0].targets[0].solver_seeds[0][0] ^ 1] {
            let mut bad = original.clone();
            bad.allocations[0].targets[0].solver_seeds[0][0] = seed;
            assert!(audit(&bad, classes).is_err());
        }
        let mut bad = original.clone();
        bad.allocations[1].targets[0].solver_seeds[0][0] =
            bad.allocations[0].targets[0].solver_seeds[0][0];
        assert!(audit(&bad, classes).is_err());
    }

    #[test]
    fn incomplete_cohort_target_arrays_and_indices_are_rejected() {
        let (classes, original) = fixture();
        let mut bad = original.clone();
        bad.allocations.pop();
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.allocations[0].targets.pop();
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.allocations[0].targets[0].target_index = 1;
        assert!(audit(&bad, classes).is_err());
    }

    #[test]
    fn cohort_identity_tampering_is_rejected() {
        let (classes, original) = fixture();
        let mut bad = original.clone();
        bad.allocations[0].class.p += 2;
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.allocations[0] = bad.allocations[1].clone();
        assert!(audit(&bad, classes).is_err());
    }

    #[test]
    fn missing_duplicate_or_reordered_blocks_are_rejected() {
        let (classes, original) = fixture();
        let mut bad = original.clone();
        bad.block_order.pop();
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.block_order[0] = bad.block_order[1].clone();
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.block_order.swap(0, 1);
        assert!(audit(&bad, classes).unwrap_err().contains("block order"));
    }

    #[test]
    fn unsupported_readiness_and_confirmation_claims_are_rejected() {
        let (classes, original) = fixture();
        let mut bad = original.clone();
        bad.execution_ready = true;
        assert!(audit(&bad, classes).is_err());
        let mut bad = original.clone();
        bad.phase = "confirmation".into();
        assert!(audit(&bad, classes).is_err());
    }

    #[test]
    fn parser_rejects_unequal_seed_counts_and_unrecognized_fields() {
        let (_, plan) = fixture();
        let mut bad = serde_json::to_value(plan).unwrap();
        bad["allocations"][0]["targets"][0]["solver_seeds"][0] = json!([1]);
        assert!(serde_json::from_value::<Plan>(bad).is_err());
        let mut bad = serde_json::to_value(plan).unwrap();
        bad["equivalence_passed"] = true.into();
        assert!(serde_json::from_value::<Plan>(bad).is_err());
    }

    #[test]
    fn existing_allocations_are_not_overwritten() {
        let (_, plan) = fixture();
        let path =
            std::env::temp_dir().join(format!("conductor-allocation-{}.json", std::process::id()));
        let bytes = save_new(&path, plan).unwrap();
        assert!(save_new(&path, plan).is_err());
        assert_eq!(fs::read(&path).unwrap(), bytes);
        let roundtrip: Plan = serde_json::from_slice(&bytes).unwrap();
        assert!(audit(&roundtrip, fixture().0).is_ok());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn execute_and_malformed_options_fail_closed() {
        assert!(run(&["run".into()]).is_err());
        assert!(run(&["plan".into(), "--phase".into(), "confirmation".into()]).is_err());
        assert!(run(&["audit".into(), "--plan".into()]).is_err());
    }
}
