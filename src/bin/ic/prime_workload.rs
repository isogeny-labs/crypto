//! Immutable prime-field known-answer workloads. No solver or curve search runs here.
use factor_base::arithmetic::{n, parse, Curve, CurveSpec, FieldSpec, PointSpec};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::Path,
};

pub const SCHEMA: &str = "ic.prime-workload/v1";
pub const MAX_BYTES: u64 = 1024 * 1024;
pub const MAX_TARGETS: usize = 4096;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub scalar: u64,
    pub point: PointSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub schema: String,
    pub curve: CurveSpec,
    /// Descriptive provenance only; never used as a certificate of independence.
    pub sampling: String,
    pub targets: Vec<Target>,
}

pub struct Validated {
    pub document: Workload,
    pub sha256: String,
    pub curve: Curve,
}

fn validated_curve(spec: &CurveSpec) -> Result<Curve, String> {
    let FieldSpec::Prime { modulus } = &spec.field else {
        return Err("prime workload requires a prime field".into());
    };
    // Width checks precede primality and scalar arithmetic.
    let word = |value: &str| -> Result<u64, String> {
        if value.len() > 32 {
            return Err("oversized workload integer".into());
        }
        parse(value)
            .map_err(|e| e.to_string())?
            .to_u64()
            .ok_or_else(|| "workload integer exceeds u64".into())
    };
    let p = word(modulus)?;
    if !(5..(1u64 << 32)).contains(&p) {
        return Err("prime workload requires 5 <= p < 2^32".into());
    }
    for value in [
        &spec.a,
        &spec.b,
        &spec.group_order,
        &spec.subgroup_order,
        &spec.cofactor,
        &spec.generator.x,
        &spec.generator.y,
    ] {
        word(value)?;
    }
    if word(&spec.subgroup_order)? <= 2 {
        return Err("prime workload requires odd prime subgroup order".into());
    }
    Curve::new(spec.clone()).map_err(|e| e.to_string())
}

impl Workload {
    pub fn validate(&self) -> Result<Curve, String> {
        if self.schema != SCHEMA {
            return Err("unsupported prime workload schema".into());
        }
        if self.sampling.is_empty() || self.sampling.len() > 1024 {
            return Err("workload sampling provenance must have 1..1024 bytes".into());
        }
        if self.targets.is_empty() || self.targets.len() > MAX_TARGETS {
            return Err(format!("workload requires 1..{MAX_TARGETS} targets"));
        }
        let curve = validated_curve(&self.curve)?;
        let r = curve.r.to_u64().ok_or("subgroup order exceeds u64")?;
        let mut seen = BTreeSet::new();
        for (index, target) in self.targets.iter().enumerate() {
            if target.scalar == 0 || target.scalar >= r || !seen.insert(target.scalar) {
                return Err(format!("target {index}: scalar must be unique and in 1..r"));
            }
            let expected = curve
                .scale(Some(curve.g.clone()), &n(target.scalar))
                .ok_or_else(|| format!("target {index}: identity target"))?;
            // Require canonical coordinates as well as equality to [k]P.
            if expected.spec() != target.point {
                return Err(format!("target {index}: point differs from canonical [k]P"));
            }
        }
        Ok(curve)
    }

    /// A counter-based SHA-256 stream with rejection sampling, without replacement.
    /// This generates public known-answer fixtures, not secret keys.
    #[allow(dead_code)] // Used by the standalone native fixture generator.
    pub fn generate(
        spec: CurveSpec,
        count: usize,
        seed: u64,
        domain: &str,
    ) -> Result<Self, String> {
        let curve = validated_curve(&spec)?;
        let r = curve.r.to_u64().ok_or("subgroup order exceeds u64")?;
        if count == 0 || count > MAX_TARGETS || count as u64 >= r {
            return Err("target count exceeds unique nonzero scalar population or limit".into());
        }
        if domain.is_empty() || domain.len() > 256 {
            return Err("sampling domain must have 1..256 bytes".into());
        }
        let bound = r - 1;
        let threshold = 0u64.wrapping_sub(bound) % bound;
        let mut selected = BTreeSet::new();
        let mut targets = Vec::with_capacity(count);
        let mut counter = 0u64;
        while targets.len() < count {
            let mut h = Sha256::new();
            h.update(b"ic.prime-workload/v1/sha256-counter/rejection-v1\0");
            h.update((domain.len() as u64).to_le_bytes());
            h.update(domain.as_bytes());
            h.update(seed.to_le_bytes());
            h.update(counter.to_le_bytes());
            counter = counter.checked_add(1).ok_or("sampling counter exhausted")?;
            let bytes = h.finalize();
            let draw = u64::from_le_bytes(bytes[..8].try_into().expect("eight bytes"));
            if draw < threshold {
                continue;
            }
            let scalar = 1 + draw % bound;
            if !selected.insert(scalar) {
                continue;
            }
            let point = curve
                .scale(Some(curve.g.clone()), &n(scalar))
                .ok_or("sampled identity target")?
                .spec();
            targets.push(Target { scalar, point });
        }
        Ok(Self {
            schema: SCHEMA.into(),
            curve: curve.spec,
            sampling: format!(
                "sha256-counter/rejection-v1; domain={domain}; seed={seed}; count={count}"
            ),
            targets,
        })
    }

    #[allow(dead_code)] // Used by the standalone native fixture generator.
    pub fn write_new(&self, path: &Path) -> Result<String, String> {
        self.validate()?;
        let mut bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        if bytes.len() as u64 > MAX_BYTES {
            return Err("workload exceeds byte limit".into());
        }
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| format!("cannot create {}: {e}", path.display()))?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        Ok(format!("{:x}", Sha256::digest(&bytes)))
    }
}

pub fn load(path: &Path) -> Result<Validated, String> {
    let file =
        std::fs::File::open(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("workload exceeds byte limit".into());
    }
    let document: Workload = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let curve = document.validate()?;
    Ok(Validated {
        document,
        curve,
        sha256: format!("{:x}", Sha256::digest(&bytes)),
    })
}

/// Both the IC and rho adapters call this exact target-selection function.
#[allow(dead_code)] // Used by the IC/rho consumer; generator binaries only validate.
pub fn scalar_for(frozen: &[u64], seed: u64, repeat: usize, r: u64) -> u64 {
    if frozen.is_empty() {
        1 + seed.wrapping_add((repeat as u64).wrapping_mul(0x9E37)) % (r - 1)
    } else {
        frozen[repeat]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn spec() -> CurveSpec {
        serde_json::from_str(r#"{"field":{"kind":"prime","modulus":"17"},"a":"2","b":"2","group_order":"19","subgroup_order":"19","cofactor":"1","generator":{"x":"5","y":"1"}}"#).unwrap()
    }
    #[test]
    fn deterministic_unique_complete_population() {
        let a = Workload::generate(spec(), 18, 123, "holdout").unwrap();
        a.validate().unwrap();
        let b = Workload::generate(spec(), 18, 123, "holdout").unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        assert_eq!(
            a.targets
                .iter()
                .map(|t| t.scalar)
                .collect::<BTreeSet<_>>()
                .len(),
            18
        );
        assert!(Workload::generate(spec(), 19, 123, "holdout").is_err());
    }
    #[test]
    fn seed_changes_targets_without_changing_curve() {
        let a = Workload::generate(spec(), 8, 1, "held-out").unwrap();
        let b = Workload::generate(spec(), 8, 2, "held-out").unwrap();
        assert_eq!(a.curve, b.curve);
        assert_ne!(
            a.targets.iter().map(|t| t.scalar).collect::<Vec<_>>(),
            b.targets.iter().map(|t| t.scalar).collect::<Vec<_>>()
        );
    }
    #[test]
    fn reject_changed_point_duplicate_and_out_of_range_scalar() {
        let original = Workload::generate(spec(), 3, 9, "mutation").unwrap();
        let mut bad = original.clone();
        bad.targets[0].point.x = "0".into();
        assert!(bad.validate().is_err());
        let mut bad = original.clone();
        bad.targets[1] = bad.targets[0].clone();
        assert!(bad.validate().is_err());
        for scalar in [0, 19, u64::MAX] {
            let mut bad = original.clone();
            bad.targets[0].scalar = scalar;
            assert!(bad.validate().is_err());
        }
    }
    #[test]
    fn reject_invalid_curve_and_unknown_json_fields() {
        let original = Workload::generate(spec(), 3, 2, "mutation").unwrap();
        for value in ["15", "0", "4294967296"] {
            let mut bad = original.clone();
            bad.curve.field = FieldSpec::Prime {
                modulus: value.into(),
            };
            assert!(bad.validate().is_err());
        }
        let mut bad = original.clone();
        bad.curve.group_order = "20".into();
        assert!(bad.validate().is_err());
        let mut value = serde_json::to_value(original).unwrap();
        value["ignored"] = true.into();
        assert!(serde_json::from_value::<Workload>(value).is_err());
    }
    #[test]
    fn frozen_targets_ignore_solver_seed_and_preserve_legacy_formula() {
        for rep in 0..3 {
            assert_eq!(
                scalar_for(&[7, 3, 11], 0, rep, 19),
                scalar_for(&[7, 3, 11], 999, rep, 19)
            );
            assert_eq!(
                scalar_for(&[], 42, rep, 19),
                1 + (42 + rep as u64 * 0x9E37) % 18
            );
        }
    }
    #[test]
    fn create_new_and_byte_digest_are_enforced() {
        let dir = std::env::temp_dir().join(format!("pkm-workload-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("fixture.json");
        let a = Workload::generate(spec(), 3, 123, "file").unwrap();
        let hash = a.write_new(&path).unwrap();
        assert_eq!(load(&path).unwrap().sha256, hash);
        assert!(a.write_new(&path).is_err());
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.push(b' ');
        std::fs::write(&path, bytes).unwrap();
        assert_ne!(load(&path).unwrap().sha256, hash);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
