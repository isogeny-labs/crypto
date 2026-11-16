use crypto_lib::cryptanalysis::ecbench::{canonical::sha256_hex, workload::CurveSpec};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::OpenOptions, io::Write, path::Path};

pub type Result<T> = std::result::Result<T, String>;
pub const SCHEMA: &str = "factor_base_sweep/v1";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub label: String,
    pub curve: CurveSpec,
    pub factor_base: Option<String>,
    pub shape: Option<String>,
    pub dimension: u32,
    pub seed: u64,
}
impl Candidate {
    pub fn id(&self) -> Result<String> {
        Ok(format!(
            "FBS1h{}",
            &sha256_hex(&serde_json::to_vec(self).map_err(|e| e.to_string())?)[..20]
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub schema: String,
    pub seed: u64,
    pub train: u32,
    pub holdout: u32,
    pub max_degree: u32,
    pub candidate_seconds: u64,
    pub max_abscissae: usize,
    pub max_matrix_cells: u64,
    pub memory_mib: u64,
    pub threads: usize,
    pub candidates: Vec<Candidate>,
}
impl Plan {
    pub fn grid(which: &str, seed: u64, seed_count: u32, dimensions: &[u32]) -> Result<Self> {
        if !(1..=64).contains(&seed_count) || dimensions.iter().any(|d| !(1..=8).contains(d)) {
            return Err("grid needs 1..64 seeds and dimensions 1..8".into());
        }
        let mut plan = Self::preset(which, seed);
        let original = std::mem::take(&mut plan.candidates);
        for offset in 0..seed_count {
            let next_seed = seed
                .checked_add(offset.into())
                .ok_or("seed range overflows")?;
            for candidate in &original {
                if candidate.shape.is_some() {
                    let dimensions = if dimensions.is_empty() {
                        vec![candidate.dimension]
                    } else {
                        dimensions.to_vec()
                    };
                    for dimension in dimensions {
                        let mut c = candidate.clone();
                        c.dimension = dimension;
                        c.seed = next_seed;
                        c.label = format!("{}-d{dimension}-seed{next_seed}", c.label);
                        plan.candidates.push(c);
                    }
                } else if offset == 0
                    || candidate
                        .factor_base
                        .as_deref()
                        .is_some_and(|s| s.starts_with("prime-isogeny:"))
                {
                    let mut c = candidate.clone();
                    c.seed = next_seed;
                    if c.factor_base
                        .as_deref()
                        .is_some_and(|s| s.starts_with("prime-isogeny:"))
                    {
                        c.factor_base = Some(format!("prime-isogeny:t=2,seed={next_seed}"));
                    }
                    c.label = format!("{}-seed{next_seed}", c.label);
                    plan.candidates.push(c);
                }
            }
        }
        plan.validate()?;
        Ok(plan)
    }
    pub fn preset(which: &str, seed: u64) -> Self {
        let mut candidates = Vec::new();
        if which != "prime" {
            for shape in [
                "coordinate",
                "gp",
                "scaled",
                "symmetric",
                "small-order",
                "frobenius",
                "frobenius2",
                "mixed",
                "random",
                "tracezero",
                "tracezero-gp",
                "tracezero-frobenius",
                "subfield",
            ] {
                candidates.push(Candidate {
                    label: format!("binary-{shape}"),
                    curve: CurveSpec::Koblitz { a: 1, n: 7 },
                    factor_base: None,
                    shape: Some(shape.into()),
                    dimension: if shape == "mixed" { 4 } else { 3 },
                    seed,
                });
            }
        }
        if which != "binary" {
            for spec in [
                "prime-abscissa:size=4",
                "prime-kummer:t=2",
                "prime-isogeny:t=2,seed=1",
            ] {
                candidates.push(Candidate {
                    label: spec.into(),
                    curve: CurveSpec::PrimeRoster { bits: 16 },
                    factor_base: Some(spec.into()),
                    shape: None,
                    dimension: 0,
                    seed,
                });
            }
        }
        Self {
            schema: SCHEMA.into(),
            seed,
            train: 8,
            holdout: 4,
            max_degree: 8,
            candidate_seconds: 30,
            max_abscissae: 64,
            max_matrix_cells: 2_000_000,
            memory_mib: 2048,
            threads: 1,
            candidates,
        }
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA || self.candidates.is_empty() || self.candidates.len() > 1024 {
            return Err("unsupported schema or candidate count (1..=1024)".into());
        }
        if self.train == 0
            || self.holdout == 0
            || self.train > 4096
            || self.holdout > 4096
            || !(1..=32).contains(&self.max_degree)
            || self.candidate_seconds == 0
            || self.candidate_seconds > 86400
            || !(2..=256).contains(&self.max_abscissae)
        {
            return Err("invalid trial, degree, time or inventory budget".into());
        }
        if self.max_matrix_cells == 0
            || self.max_matrix_cells > 100_000_000
            || !(256..=65536).contains(&self.memory_mib)
            || !(1..=64).contains(&self.threads)
        {
            return Err("invalid matrix, memory or thread budget".into());
        }
        let mut ids = std::collections::BTreeSet::new();
        for c in &self.candidates {
            if matches!(c.curve, CurveSpec::PrimePublicFixture { .. }) {
                return Err("the sweep uses matched seeded public probes; a supplied public target would be ignored, so PrimePublicFixture is unsupported".into());
            }
            if c.factor_base.is_some() == c.shape.is_some() {
                return Err(
                    "candidate requires exactly one constructor spec or binary shape".into(),
                );
            }
            if c.shape.is_some() && !(1..=8).contains(&c.dimension) {
                return Err("binary shape dimension must be 1..=8".into());
            }
            if !ids.insert(c.id()?) {
                return Err("duplicate candidate configuration".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub plan: Plan,
    pub candidate: Candidate,
}

pub fn write_new(path: &Path, value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(&bytes)
        .and_then(|_| f.write_all(b"\n"))
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
pub fn read<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))
}
pub fn emit(event: &str, data: Value) -> Result<()> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, &serde_json::json!({"event":event,"data":data}))
        .map_err(|e| e.to_string())?;
    writeln!(out)
        .and_then(|_| out.flush())
        .map_err(|e| e.to_string())
}
pub fn digest(path: &Path) -> Result<String> {
    Ok(sha256_hex(&std::fs::read(path).map_err(|e| e.to_string())?))
}
