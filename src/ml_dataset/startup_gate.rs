//! Fixed-path startup admission. All retained records are restricted operational
//! evidence. This gate neither submits Jobs nor issues or refunds budget grants.
//! verify_protocol establishes frozen bytes/schema. The trusted GateConfig must
//! be reviewed against the protocol's intended schedule and resources; the gate
//! does not interpret prose stopping rules or claim general schedule validation.

use super::budget::OwnerIdentity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum GateErrorKind {
    MissingInputTimeout,
    InvalidInput,
    IdentityMismatch,
    AuthorityChanged,
    UnsupportedPlatform,
    ActivationFailure,
    GenerationFailure,
    GenerationOutcomeUnknown,
    PersistenceFailure,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct GateError {
    kind: GateErrorKind,
    message: String,
}

impl GateError {
    fn new(kind: GateErrorKind, message: impl ToString) -> Self {
        // Bound diagnostics even when an underlying parser supplies context.
        let message = message.to_string().chars().take(2048).collect();
        Self { kind, message }
    }
}
impl std::fmt::Display for GateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "startup gate {:?}: {}", self.kind, self.message)
    }
}
impl std::error::Error for GateError {}
type Result<T> = std::result::Result<T, GateError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct GateOutcome {
    schema: String,
    status: String,
    owner: OwnerIdentity,
    config_sha256: String,
    accepted_bundle_sha256: String,
    generation_handler_returned_success: bool,
    kubernetes_attestation_performed: bool,
}

#[cfg(target_os = "linux")]
mod linux;

pub(super) fn start(resume: bool) -> Result<GateOutcome> {
    #[cfg(target_os = "linux")]
    {
        linux::start(resume)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = resume;
        Err(GateError::new(
            GateErrorKind::UnsupportedPlatform,
            "startup gate requires Linux openat2 and CLOCK_BOOTTIME",
        ))
    }
}

#[cfg(all(test, not(target_os = "linux")))]
mod platform_tests {
    use super::*;

    #[test]
    fn unsupported_startup_gate_refuses_before_native_admission() {
        for resume in [false, true] {
            assert_eq!(
                start(resume).unwrap_err().kind,
                GateErrorKind::UnsupportedPlatform
            );
        }
    }
}
