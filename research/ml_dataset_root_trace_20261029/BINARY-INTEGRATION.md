# Retained binary root integration design

Design only, 2026-10-29. This inventory performed no builds, tests, cloud jobs,
IAM changes or implementation edits. Historical validation below is not current
combined-source validation. Root authorized this single design file under T803.

## Source identity and minimum integration

Donor worktree: `/home/ec2-user/crypto-ml-binary-root-trace-20261028`, HEAD
`c14d4552e513f6e28bda0e0e045f5e64beb492db`. Destination is the current
`crypto-ml-root-trace-20261029` worktree (runtime 6542d87, later documentation).

1. Copy `src/cryptanalysis/binary_root_trace.rs` unchanged. SHA-256:
   `d77bfc85556b51d938ffa8cc82a402f7e0b3cbaf4c05b81707f5154e5a01ad0e`.
   Register `pub mod binary_root_trace` in `src/cryptanalysis/mod.rs`.
2. Copy `src/ml_dataset/binary_roots.rs`. Donor SHA-256:
   `493527494482e45f657a0f248dc68d7442b73a8837672b7a5326c92cedcd09c2`.
   One required test-only compatibility adjustment: remove
   `factor_base_policy: None` from its CaseConfig fixture, because destination
   CaseConfig has no such field. Do not import the whole policy schema merely
   to compile this fixture. Record the resulting adapter hash separately.
3. In current `src/ml_dataset/solvers.rs`, add the path module registration and
   `BINARY_ROOT_CERTIFICATE_SCOPE` alias; pass `case` from `solve` into
   `root_solve`; retain the prime branch, replace only the old binary loop with
   `binary_roots::solve(config, case, group, domain, target)`; add wrapper
   `verify_binary_root_artifact`. Remove the now-unused binary_semaev imports
   from solvers.rs (the adapter has its own). Update only the root capability
   description. No Cargo dependency change is indicated by this inventory.
4. In current `src/ml_dataset/mod.rs`, add donor
   `verify_binary_root_outcome` while retaining the stronger current prime
   outcome counter recomputation. Extend certificate collection to both roles,
   require the unique role matching family, and dispatch binary replay through
   the wrapper. Extend current reserved filename/role and surviving-native-
   artifact checks to `binary-root-certificates.json`,
   `native-binary-root-call`, and `native-binary-root-intermediate-state`.
   Preserve current startup, budget, prime-chain and prime certificate gates.
5. Port/extend central sealed-censoring, orphan/role-removal and label-tampering
   regression cases across prime/binary/koblitz. Preserve the stronger current
   joint role-plus-outcome removal control rather than replacing it with the
   weaker historical donor gate.
6. Exporter coordination is separate: donor export.rs includes both certificate
   roles in solution_certificates. The exporter was separately assigned to T804; fresh Conductor lookup
   reports ready with no published commit listed. Do not overwrite its retained
   work or infer delivery from that status. Until the references are integrated, describe
   certificates as retained restricted artifacts, not fully wired dedicated
   solution-certificate labels.

## Compatibility risks

Do not replace whole donor solvers.rs or mod.rs. The donor lacks destination's
prime m=3..15 affine-chain dispatch, mathematical replay and live-persistence
handling. It also contains an independent bound CaptureEmitter migration and
factor-base policy schema that are outside this narrow integration. Replacing
whole files would regress current startup/chain/budget/prime-root behavior.

Binary adapter production code uses existing Group, Element, domain, save,
live_event, set_witness and CaseData interfaces. Its v2 unsupported-base check
is inert for legacy bases. Its full canonical CaseConfig hash changes naturally
with the actual destination schema; never rewrite old donor certificates to
pretend they were generated with the new schema. Historical donor v2 fixtures
are not automatically replay-compatible with current legacy CaseConfig parsing.

The sink test correctly filters native events before checking the final native
event: an overall solver_return error may follow a native event whose selected
payload write failed. Preserve that distinction. The runtime event stream is
not a restorable native solver checkpoint.

## Existing tests to retain and validation to perform

Native module has nine tests. `requested_degrees_have_constructed_and_trace_one_controls`
iterates every degree 23..31 and 47..61 (including 23,31,47,61), checks two
constructed roots and trace-one no-root controls, and checks odd half-trace or
even matrix events. This is field-root coverage, not full curve dataset coverage.

`exhaustive_small_odd_even_quadratics_match_enumeration`,
`linear_pure_square_repeated_and_zero_cases_are_explicit`, and
`legacy_quadratic_roots_and_inverse_primitive_counts_match` cover small-field
and degenerate semantics. `exact_work_and_event_caps_preserve_censored_prefixes`
checks every prefix cap through exact completion; `sink_failure_stops_at_each_attempted_event_including_finish`
injects sink failure at every event. `rejects_noncanonical_fields_coefficients_limits_and_record_shapes`
and `certificate_and_event_tampering_is_rejected` cover malformed/tampered
records. `root_completeness_does_not_certify_complete_execution_or_total_cost`
preserves the explicit weaker claim boundary.

Adapter has five tests:

- `odd_even_binary_families_and_twists_bind_and_lift_without_private_witness`:
  binary and koblitz, n=11/12, both twists; planted public targets with private
  witness erased, group replay and certificate verification.
- `aggregate_caps_preserve_replayable_prefix_and_exact_completion_boundary`:
  koblitz n=11/12, setup/partial/exact-work boundary caps.
- `exact_identity_native_events_and_aggregate_tampering_are_rejected`:
  binary n=12 twist; object/config/domain bindings, event and byte aggregates,
  event omission, query omission, modulus and unsupported claim tampering.
- `live_states_and_event_prefix_survive_payload_sink_failure`: koblitz n=12;
  real selected-payload collision, preserved sentinel/native prefix, absence of
  success certificate, then successful fresh-directory replay.
- `higher_arity_remains_explicitly_unsupported`: m=3 rejection. Root integration
  remains m=2; it does not implement requested higher-m elimination.

After source integration, rerun focused native/adapter/central checks, maintained
strict Clippy and required full release library suite. Freeze new bounded actual
CLI fixtures for both binary families, odd/even degrees and both twists; retain
generate/verify/export outputs, restricted certificates and S3-restored replay.
Do not call these scientific corpus cells. Parent's currently running prime-only
full suite cannot validate a later binary source change.

## Prior evidence and limitations

Donor `research/ml_dataset_binary_root_20261028/VALIDATION.md` records runtime
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`: full library 5183 passed/0 failed/
123 ignored; native module nine passed; dataset 158 passed/0 failed/one ignored;
eight CLI checks and strict scoped Clippy passed. These records were read, not
re-executed or independently downloaded in this inventory. Historical archive:
`s3://setec-astronomy-artifacts/ml-dataset/20261028/software/binary-root-final-gate/8d1bac39e3909edd281b7feea3f5a17a6f4a7ac181b90d87dc04f2ac4f82d238/validation.tar.gz`.
The donor records a separate 51-fixture/30-completed-root-set consumer audit;
unsuccessful rows remain failures. This does not prove main integration.

Native scope is polynomial-basis GF(2^n), 3<=n<=61, degree<=2. Portable
gcd(f,X^(2^n)+X) checks distinct-root completeness for each completed polynomial;
it does not prove full execution, multiplicities, complete PDP search, UNSAT,
scalar recovery, first/last-fall degree, or complete instruction cost. Counters
mix selected field calls, matrix bit tests/word XORs and adapter lift charges.
Both execution_trace_complete_verified and total_operation_count_verified stay
false. Roots/traces/certificates remain restricted labels/provenance.

Source review consulted donor DESIGN.md's existing literature review (Shoup
sections 19.4/20.4.1, SEC1 v2 2.3.9), binary_semaev and retained verifier source.
No fresh external literature access or novelty/performance claim was made by
this read-only integration inventory. Destination AGENTS.md classifies rootfind
as generic decomposition; this integration is trace/replay functionality, not
an algebraic index-calculus advance.
