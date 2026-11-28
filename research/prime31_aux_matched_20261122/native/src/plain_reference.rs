//! Frozen cardinality bindings for the plain-control candidate.
//!
//! Reading a binding performs no auxiliary-curve construction, isogeny
//! evaluation, coordinate lifting, or kernel enumeration. The offline auditor
//! independently reconstructs the retained fiber and checks its lift count.

use crate::kernel_fibers::{KernelChoice, KernelFactorization};
use crate::target_transport::TargetArm;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

const FIXTURE: &[u8] =
    include_bytes!("../../isogeny-choice/fixtures/kernel_correctness_fixtures.json");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainReferenceBinding {
    pub schema: String,
    pub fixture_sha256: String,
    pub kernel: KernelChoice,
    pub coset_scalar: u64,
    #[serde(default, skip_serializing_if = "KernelFactorization::is_standard")]
    pub factorization: KernelFactorization,
    pub target_arm: TargetArm,
    pub columns: usize,
}

/// Exact retained bytes, also covered by the executable's source inventory.
pub fn fixture_bytes() -> &'static [u8] {
    FIXTURE
}

/// Look up the preregistered count. This is intentionally a metadata lookup;
/// mathematical certification belongs to the independent offline auditor.
pub fn frozen_plain_reference(
    kernel: KernelChoice,
    coset_scalar: u64,
    factorization: KernelFactorization,
    target_arm: TargetArm,
) -> Result<PlainReferenceBinding, String> {
    if coset_scalar != 1 {
        return Err("the retained plain-reference fixture covers only coset scalar one".into());
    }
    if factorization != KernelFactorization::Standard && kernel != KernelChoice::AminusH2 {
        return Err("full-two-torsion-first factorization requires the H2 kernel".into());
    }
    let fixture: Value = serde_json::from_slice(FIXTURE).map_err(|e| e.to_string())?;
    if fixture["schema"] != "p31.kernel-comparison/v1"
        || fixture["status"] != "arithmetic_correctness_fixtures_no_timing"
        || fixture["p"] != 2_147_483_647u64
        || fixture["degree"] != 1024
        || fixture["source_scalars"] != serde_json::json!([1])
    {
        return Err("the retained plain-reference fixture has changed its declared scope".into());
    }
    let name = match kernel {
        KernelChoice::AplusCyclic => "Aplus_cyclic",
        KernelChoice::AminusH0 => "Aminus_H0",
        KernelChoice::AminusH1 => "Aminus_H1",
        KernelChoice::AminusH2 => "Aminus_H2",
    };
    let arms = fixture["arms"]
        .as_array()
        .ok_or("plain-reference fixture has no arm inventory")?;
    let matching: Vec<_> = arms.iter().filter(|a| a["name"] == name).collect();
    if matching.len() != 1 {
        return Err("plain-reference fixture must contain exactly one matching arm".into());
    }
    let cosets = matching[0]["cosets"]
        .as_array()
        .ok_or("plain-reference arm has no coset inventory")?;
    if cosets.len() != 1 || cosets[0]["source_scalar"] != 1 {
        return Err("plain-reference arm does not identify the frozen coset".into());
    }
    let count_key = if target_arm == TargetArm::IsogenousNative {
        "accepted_E1_x"
    } else {
        // The prime-order degree-three map transports source point columns
        // bijectively. The auditor independently checks that correspondence.
        "accepted_E_x"
    };
    let columns = cosets[0][count_key]
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .filter(|&n| n > 0 && n <= 1024)
        .ok_or("plain-reference fixture has an invalid usable-column count")?;
    Ok(PlainReferenceBinding {
        schema: "prime31-isogeny-choice.plain-reference/v1".into(),
        fixture_sha256: format!("{:x}", Sha256::digest(FIXTURE)),
        kernel,
        coset_scalar,
        factorization,
        target_arm,
        columns,
    })
}
