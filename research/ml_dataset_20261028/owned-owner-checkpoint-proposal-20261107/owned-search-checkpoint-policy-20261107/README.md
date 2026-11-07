# Explicit within-search checkpoint policy proposal

Adds conservative_affine_envelope_checkpoint_v1 as an explicit choice under the existing exclusive v5 request. Existing conservative_affine_envelope_v1 and all older channels preserve their behavior. The worker maps the new variant to the private owner policy, which selects one callback after each committed batch with at most 16 batch-hook attempts shared across the entire continuation. Existing initial, post-exclusion and final publications are separate and still governed by global capture limits. Hitting the hook count ceiling stops the attempt under the frozen kernel contract; it does not silently continue without capture.

Arithmetic limits, original cumulative work/memory/trace allowances and deadline are unchanged. Checkpoint work and storage consume those same allowances. This policy is a source proposal, not authorization to launch a dataset or extend a deadline.

Six focused tests pass on the extracted actual policy factory and choice function. The harness uses saved Budget and kernel type dependencies with exact SOURCE_BINDINGS; the policy harness excludes actual owner selection. A separate serde/selection harness passes 16 tests using actual proposed types and extracted worker selection/profile: legacy and new policy round trips, mixed/null requests, unsupported profiles, both twists and pattern boundaries. This exercises eligibility only, not curve generation or solving. Full assembled source validation and CLI runtime remain outstanding. Owner callsite proposal is separately bound in owned-search-owner-checkpoint-20261107; this packet requires its Caps.checkpoint field.

Reviewed references: existing original_boolean_search_policy.rs, types.rs v5 opt-in validation, owned_boolean_search_worker.rs selection, frozen batch-hook contract, and original owner capture publication. No mathematical claims are inferred from these software checks.

Replay: rustc +1.98 --edition=2021 --test checks/harness.rs -o checks/tests; checks/tests --test-threads=1 original::tests. The prepare script records extraction but expects a new packet directory and should not be run over immutable evidence.
