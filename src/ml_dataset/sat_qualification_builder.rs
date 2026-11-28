//! Proposed cfg(test) helper in a child of ml_dataset::solvers::koblitz_m3.
//! The caller freezes the recipe/configuration before calling this function.
//! No Solver, native solve, seed scan or target-selection feedback is used.
use crate::ml_dataset::{
    curves,
    types::{CaseConfig, CaseData, FactorBasePolicy},
};
use serde_json::json;
use std::time::Instant;

/// Caller must serialize this owning result on every outcome, including refusal.
/// HashMap allocator overhead is not inferred from slot counts.
#[derive(Default, serde::Serialize)]
pub(super) struct SetupAttempt {
    pub case: Option<CaseData>,
    #[serde(serialize_with = "table_entries")]
    pub baby: std::collections::HashMap<Option<(u64, u64)>, u64>,
    pub stage: &'static str,
    pub first_error: Option<String>,
    pub checkpoint_error: Option<String>,
    pub width: u64,
    pub probes: u64,
    pub scalar: Option<u64>,
    pub baby_point: Option<(u64, u64)>,
    pub giant: Option<(u64, u64)>,
    pub tuple: [Option<(u64, u64)>; 3],
    pub target: Option<(u64, u64)>,
    pub first_prefix: Option<(u64, u64)>,
    pub permutation_prefixes: [Option<(u64, u64)>; 3],
    pub requested_table_slots: u64,
    pub actual_table_capacity: u64,
    pub baby_completed: u64,
    pub native_add_started: u64,
    pub native_add_completed: u64,
    pub reference_add_started: u64,
    pub reference_add_completed: u64,
    pub native_mul_started: u64,
    pub native_mul_completed: u64,
    pub reference_mul_started: u64,
    pub reference_mul_completed: u64,
    pub opaque_calls_started: u64,
    pub opaque_calls_completed: u64,
}
/// `checkpoint` must durably persist restricted state under finite byte limits.
/// Run only under the external absolute-deadline process supervisor; helper
/// calls are not claimed interruptible inside this Rust function.
pub(super) fn fixed_nonroot_positive(
    config: &CaseConfig,
    deadline: Instant,
    checkpoint: &mut impl FnMut(&SetupAttempt) -> Result<(), String>,
) -> SetupAttempt {
    let mut state = SetupAttempt::default();
    if let Err(error) = build(config, deadline, &mut state, checkpoint) {
        state.first_error = Some(error);
    }
    if let Err(error) = checkpoint(&state) {
        state.checkpoint_error = Some(error);
    }
    state
}
fn build(
    config: &CaseConfig,
    deadline: Instant,
    state: &mut SetupAttempt,
    checkpoint: &mut impl FnMut(&SetupAttempt) -> Result<(), String>,
) -> Result<(), String> {
    let tick = || {
        if Instant::now() < deadline {
            Ok(())
        } else {
            Err("original positive-control setup deadline exhausted".to_string())
        }
    };
    tick()?;
    if config.n != 23
        || config.m != 3
        || config.factor_base_size != 4
        || config.solver != "sat"
        || !matches!(config.family.as_str(), "binary" | "koblitz")
        || !matches!(
            config.factor_base_policy.as_ref(),
            Some(FactorBasePolicy::Legacy {})
        )
    {
        return Err("fixed nonroot control requires original n23/m3/LegacyFB4 SAT profile".into());
    }
    state.stage = "generate";
    state.opaque_calls_started += 1;
    checkpoint(state)?;
    state.case = Some(curves::generate(config)?);
    state.opaque_calls_completed += 1;
    state.stage = "generated";
    checkpoint(state)?;
    tick()?;
    state.stage = "arithmetic-decode";
    state.opaque_calls_started += 1;
    checkpoint(state)?;
    let arithmetic = curves::arithmetic(
        &state
            .case
            .as_ref()
            .expect("generated case retained")
            .public_instance,
    )?;
    state.opaque_calls_completed += 1;
    tick()?;
    let generator = curves::point_words(
        &state
            .case
            .as_mut()
            .expect("generated case retained")
            .public_instance["generator"],
    )?;
    let rows = state
        .case
        .as_mut()
        .expect("generated case retained")
        .factor_base["points"]
        .as_array()
        .ok_or("missing fixed factor-base rows")?;
    let p = curves::point_words(rows.first().ok_or("missing row0")?)?;
    let q = curves::point_words(rows.get(2).ok_or("missing row2")?)?;
    let (Some((px, _)), Some((qx, _))) = (p, q) else {
        return Err("fixed tuple has infinity entry".into());
    };
    if px == qx {
        return Err(
            "fixed row0/row2 do not have distinct x; preserve setup failure, no fallback".into(),
        );
    }
    state.tuple = [p, p, q];
    state.stage = "reference-prefixes";
    checkpoint(state)?;
    state.reference_add_started += 1;
    let twice = arithmetic.reference_add(p, p);
    state.reference_add_completed += 1;
    state.first_prefix = twice;
    tick()?;
    state.reference_add_started += 1;
    let target = arithmetic.reference_add(twice, q);
    state.reference_add_completed += 1;
    state.target = target;
    tick()?;
    if target.is_none() || twice.is_none() || generator.is_none() {
        return Err("fixed tuple has unsupported identity target/prefix; no fallback".into());
    }
    // Group prefixes are candidate references only. Complete-map qualification is separate.
    for (index, tuple) in [[p, p, q], [p, q, p], [q, p, p]].into_iter().enumerate() {
        tick()?;
        state.reference_add_started += 1;
        let partial = arithmetic.reference_add(tuple[0], tuple[1]);
        state.reference_add_completed += 1;
        state.permutation_prefixes[index] = partial;
        state.reference_add_started += 1;
        let total = arithmetic.reference_add(partial, tuple[2]);
        state.reference_add_completed += 1;
        if partial.is_none() || total != target {
            return Err("fixed three-permutation reference check failed".into());
        }
    }
    let order: u64 = state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["curve"]["subgroup_order"]
        .as_str()
        .ok_or("missing subgroup order")?
        .parse()
        .map_err(|_| "invalid subgroup order")?;
    state.width = 1;
    while state
        .width
        .checked_mul(state.width)
        .ok_or("width overflow")?
        < order
    {
        tick()?;
        state.width += 1;
        if state.width > 4096 {
            return Err("frozen BSGS slot cap exceeded".into());
        }
    }
    state.requested_table_slots = state.width;
    state.stage = "baby-allocation";
    checkpoint(state)?;
    state
        .baby
        .try_reserve(state.width as usize)
        .map_err(|e| e.to_string())?;
    state.actual_table_capacity = state.baby.capacity() as u64;
    state.stage = "baby-loop";
    state.baby_point = None;
    for j in 0..state.width {
        tick()?;
        if state.baby.contains_key(&state.baby_point) {
            return Err("duplicate reference baby point".into());
        }
        state.baby.insert(state.baby_point, j);
        state.native_add_started += 1;
        state.baby_point = arithmetic.add(state.baby_point, generator);
        state.native_add_completed += 1;
        state.baby_completed = j + 1;
        if (j + 1) % 64 == 0 {
            checkpoint(state)?;
        }
    }
    state.stage = "giant-step-multiply";
    checkpoint(state)?;
    state.native_mul_started += 1;
    let backwards = arithmetic.neg(arithmetic.mul(generator, state.width));
    state.native_mul_completed += 1;
    tick()?;
    state.giant = target;
    state.stage = "giant-loop";
    checkpoint(state)?;
    for i in 0..=state.width {
        tick()?;
        state.probes += 1;
        if let Some(&j) = state.baby.get(&state.giant) {
            let candidate = i
                .checked_mul(state.width)
                .and_then(|x| x.checked_add(j))
                .ok_or("reference scalar overflow")?;
            let valid = if candidate > 0 && candidate < order {
                state.reference_mul_started += 1;
                let equal = arithmetic.reference_mul(generator, candidate) == target;
                state.reference_mul_completed += 1;
                equal
            } else {
                false
            };
            if valid {
                state.scalar = Some(candidate);
                break;
            }
        }
        state.native_add_started += 1;
        state.giant = arithmetic.add(state.giant, backwards);
        state.native_add_completed += 1;
        if (i + 1) % 64 == 0 {
            checkpoint(state)?;
        }
    }
    let scalar = state
        .scalar
        .ok_or("fixed target reference logarithm unavailable within frozen setup bound")?;
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .private_witness["unconditioned_scalar_before_software_control"] = state
        .case
        .as_mut()
        .expect("generated case retained")
        .private_witness["scalar"]
        .clone();
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .private_witness["unconditioned_target_before_software_control"] = state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["target"]
        .clone();
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .private_witness["scalar"] = scalar.to_string().into();
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .private_witness["software_control"] = json!({
  "schema":"sat-fixed-nonroot-reference/v1","factor_base_row_indices":[0,0,2],
  "signed_tuple":[curves::point_json(p),curves::point_json(p),curves::point_json(q)],
  "identity_pattern":0,"group_permutations_checked":3,"complete_map_qualification":"pending","baby_slot_cap":4096,
  "baby_slots":state.width,"giant_probe_cap":state.width+1,"giant_probes":state.probes,
  "feature_access":"forbidden; restricted fixture reference only",
  "not_solver_evidence":true});
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["target"] = curves::point_json(target);
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["population"] = "conditioned-positive-pdp-software-control".into();
    // Do not place tuple indices, signs, pattern, scalar or reference log in public inputs.
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["sampling"]["algorithm"] =
        "fixed-public-target-software-control/v2".into();
    state
        .case
        .as_mut()
        .expect("generated case retained")
        .public_instance["sampling"]["scalar_domain"] =
        "conditioned software control; not uniform sampling".into();
    tick()?;
    state.stage = "known-answer-replay";
    state.opaque_calls_started += 1;
    checkpoint(state)?;
    {
        let case = state.case.as_ref().unwrap();
        curves::verify(&case.public_instance, &case.private_witness)?;
    }
    state.opaque_calls_completed += 1;
    checkpoint(state)?;
    tick()?;
    state.stage = "factor-base-replay";
    state.opaque_calls_started += 1;
    checkpoint(state)?;
    {
        let case = state.case.as_ref().unwrap();
        curves::verify_factor_base_config(config, &case.public_instance, &case.factor_base)?;
    }
    state.opaque_calls_completed += 1;
    checkpoint(state)?;
    tick()?;
    Ok(())
}

fn table_entries<S: serde::Serializer>(
    table: &std::collections::HashMap<Option<(u64, u64)>, u64>,
    s: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeSeq;
    let mut seq = s.serialize_seq(Some(table.len()))?;
    for entry in table {
        seq.serialize_element(&entry)?;
    }
    seq.end()
}
/// Restricted immutable streaming checkpoints. Partial files are never deleted.
/// Directory must be privately created by the supervisor; no stdout witness dump.
pub(super) struct CheckpointWriter {
    directory: std::path::PathBuf,
    sequence: u64,
    total: u64,
}
impl CheckpointWriter {
    pub fn journal_bytes(&self) -> u64 {
        self.total
    }
    pub fn new(directory: std::path::PathBuf) -> Self {
        Self {
            directory,
            sequence: 0,
            total: 0,
        }
    }
    pub fn save<T: serde::Serialize>(&mut self, state: &T) -> Result<(), String> {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        struct Bounded<'a> {
            file: &'a mut std::fs::File,
            bytes: u64,
            limit: u64,
        }
        impl Write for Bounded<'_> {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                if b.len() as u64 > self.limit - self.bytes {
                    return Err(std::io::Error::other("restricted checkpoint byte cap"));
                }
                let n = self.file.write(b)?;
                self.bytes += n as u64;
                Ok(n)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                self.file.flush()
            }
        }
        let remaining = (64u64 << 20)
            .checked_sub(self.total)
            .ok_or("checkpoint total exhausted")?
            .min(
                (96u64 << 20)
                    .checked_sub(retained_bytes(&self.directory)?)
                    .ok_or("cumulative original trace exhausted")?,
            );
        let path = self
            .directory
            .join(format!("checkpoint-{:06}.partial", self.sequence));
        self.sequence += 1;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .map_err(|e| e.to_string())?;
        let mut writer = Bounded {
            file: &mut file,
            bytes: 0,
            limit: remaining.min(4 << 20),
        };
        let result = serde_json::to_writer(&mut writer, state).map_err(|e| e.to_string());
        self.total += writer.bytes;
        drop(writer);
        file.sync_all().map_err(|e| e.to_string())?;
        result?;
        let final_path = path.with_extension("json");
        std::fs::rename(&path, &final_path).map_err(|e| e.to_string())?;
        std::fs::File::open(&self.directory)
            .and_then(|d| d.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub(super) fn retained_bytes(root: &std::path::Path) -> Result<u64, String> {
    let mut total = 0u64;
    for entry in std::fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        let size = if ty.is_dir() {
            retained_bytes(&entry.path())?
        } else if ty.is_file() {
            entry.metadata().map_err(|e| e.to_string())?.len()
        } else {
            return Err("unexpected reference symlink/special file".into());
        };
        total = total
            .checked_add(size)
            .ok_or("restricted trace byte overflow")?;
    }
    Ok(total)
}
