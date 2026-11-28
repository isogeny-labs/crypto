# Resume: native Rust rootfinder end-to-end measurement

## Goal
Measure the complete cold native Rust PKM pipeline requested by the user,
retaining failures, matched controls, replay and source identities.

## Lead / hypothesis
The merged rootfinder implementation is PR #5283. Compare native F4, scan and
resultant-GCD on both historical degree-32 and interval bases. The archived
3.695x factor-base observation is not a performance result from this patch.

## Spec
The three prospective v2 specs and design are under
research/pkm_native_rootfinder_e2e_20261127/. The source/spec publication commit
is 62a79cf21f43657aed7ca5c20d3400ee67bdee6c on
codex/rootfinder-e2e-20261127, draft PR #5303. There is no frozen canonical
protocol yet and no experiment may launch from these draft inputs.

## State
No benchmark launched and no performance data collected. Trusted-main read-only
AWS budget run 46072715236 failed to assume the existing OIDC role; budget reads
were skipped. Its non-secret receipts are in the study's access directory.
The launcher rejects v2 specs, and the canonical protocol tool is vault-locked.
The local Rust release build and all three native plans completed; see
local-validation.json. The intended curves match, with paired seeds and 448
execution slots per curve, but all three lack registry/EC1 bindings.
An existing authenticated host/role has been requested from the user.

## Next steps
Follow README.md's resume sequence for native curve registration, authenticated
budget/capacity, v2 compatibility, canonical freeze, S3 checkpoint, production
image, admitted execution and native full replay. Retain the passing local
build/plans; repeat only when a changed instrument or new concern requires it.
Do not convert new work to legacy v1 or infer authority from elapsed time or a
failed workflow.

## Runs in flight
No experiment or build is running. The local release build completed and the
three planning commands returned zero. The binary is target/release/ecbench;
its hash and build flags are recorded in local-validation.json, with the log
in plans/build.log. Local builds and planning are not performance evidence.

## Stop and success conditions
Success needs admitted cold end-to-end measurements, full scalar verification
and replay, A/A-qualified timing and complete cost/memory evidence. No launch
with failed checkpoint/admission, unknown budget, unfrozen protocol or expired
deadline. Proposed total spend ceiling USD 10; three jobs at most 4,800 seconds
each, concurrency one, no retries, original deadline 2026-11-28T04:00:00Z.
If access remains unavailable, preserve the draft and explicit open requirement.
