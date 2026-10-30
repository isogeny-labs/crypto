# Independent subgroup ECDLP view source review, revision02

Reviewer: consequence_contract. Scope: exact scratch proposal02 source, no implementation changes, build, tests, native solver, dataset generation, Git mutation or cloud action. Source-only result: no blocking finding identified in the reviewed changes. Functional/integration validation remains pending.

Reviewed exact bytes:

- export.rs: 65bd78d3d50ed8991ec6b51dbfe7f6574912e9d65136ffdbdde98191c70ee2b6
- ecdlp_features.rs: fa7724e8a66beb21be93f553e9ddcbb07fb32d78133d7ae3b614d67e8217efec
- CLI mod.rs companion: 30217ab44cc0cc030f311c4334be49d6a57ddf662852210a1eb4330915367076
- combined-proposal.patch: 47ddd1a9f12fca34413921e74545b1f87500a08ca5e1dbc94ef1bd2a07fbeb04

## Findings checked

1. Interrupted sealed source handling is reached only after central verify_cell(dir,true), manifest/run/plan/index/config binding and original V2 plan-file equality. The new classifier accepts actual collector fallback unregistered-partial-artifact/native for the recognized source paths only on error, worker_failed, timeout or trace_budget_exhausted with solver_completed=false and all seven positive/exhaustive flags absent or boolean false. Duplicate roles/paths and wrong canonical roles still fail. This early path emits a restricted unavailable record with null feature/scalar/certificate, retaining source index/outcome provenance, rather than parsing or promoting incomplete scalar bytes. Correct-role complete records still undergo public and certificate verification.
2. The authored sealed-timeout test uses Fixture::source, actual collect_cell_artifacts with absent worker registry, seals the result, calls verify_cell(true), exports, checks null unavailable records/counters and exact retry, and checks completed wrong-role rejection. Separate path variants cover public/base/witness and malformed partial scalar bytes. These are substantive authored controls; they have not run. The central fixture is historical V1, so it is not evidence that coordinated V2 activation works.
3. Public inputs are explicitly reconstructed from canonical field/model coefficients, subgroup order, generator and target. Prime and binary metadata receive explicit characteristic/degree/basis checks, decimal integers must be canonical, and the allowlist omits factor-base contents, policy, arity, cost, source metadata, private scalar and certificate. Identity is domain-separated over canonical JSON, with field size derived from checked mathematics for splits. The sentinel control mutates nested future metadata and verifies identical public input identity while target changes alter identity.
4. Public group/torsion checks alone do not admit a cyclic-subgroup membership feature. Admission additionally requires checked curves::verify construction witness or checked scalar equation certificate. A construction witness produces membership-only rows with no recovered_scalar. Certificate-derived scalar labels remain restricted and explicitly disclaim proof of independent execution. BSGS's original empty factor base can be passed to the unchanged scalar verifier, which does not call domain().
5. Enabled export binds plan/run/options before staging channel files. Default export rejects both the enabled marker and orphan enabled channel files even without a manifest. Enabled retries require matching immutable option bytes; an existing default/nonempty unbound output cannot silently become enabled. Tests cover profile flip, plan/run rebind, interrupted output and preserved orphan bytes. No-flag path creates no new marker/channel artifacts and retains the historical manifest schema; the authored test compares existing PDP files and exact repeated default outputs.
6. Sealed and unsealed occurrences each receive exactly one restricted subgroup-channel record with the same source index identity used by the original index. Channel finish checks observed/expected/deduplicated record counts and feature/recovery inequalities. Sealed channel admission happens before PDP availability filtering, so empty source factor bases do not suppress a separately verified subgroup task. New counters are explicitly occurrence counts, and inherited PDP counters are scoped separately.

## Integration boundaries

The source review does not establish test success or full CLI readiness. Retain original central custody/replay and complete V2 worker/solver/capture/execution/startup/restore migration before activation. Native scalar verification establishes an equation, not unknown-scalar search provenance, a complete ECDLP pipeline, instruction totals or comparative performance. The actual dataset scope rejects zero scalar and expects affine public targets; this view follows that existing admitted population.

No further source correction requested for these revision02 bytes.
