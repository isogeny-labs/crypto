# Signed native event adapter source review

The corrected adapter preserves the first sink or limit error across later callbacks. Its distinct observed envelope and native state-machine boundaries are supported at source level. The patch remains unapplied and uncompiled.

- Existing collector emits candidate before every identity/split/duplicate/nonsplit disposition; identity and solved edge emit before early walk break. Degree-one edges are padded to two nodes. New-node IDs begin at1 and align with adapter nodes count.
- First canonical edge attempt is recorded before duplicates; callback duplicate first_attempt matches adapter table when prior callback writes succeeded.
- Callback errors propagate Result and prevent successful collector Outcome; ingest_observed emits before/after exact original ingest and propagates sink errors, but ingest may mutate before result sink failure. Failed prefix must not be retried as though no side effect occurred.
- Kummer scope is Labels::ingest only. Full advance/lift identity/degree-one/fallback paths remain uninstrumented and are not newly observed natural streams.
- Observed JSON is retained as distinct envelope and pencil consumer rejects it, preserving separation from pencil Admission/rank semantics.
- Adapter performs no field arithmetic, but allocations/maps/JSON copies/flushes and expected-header preparation incur real unpriced costs; no timing equivalence inferred.
- Structural validator is not scalar/geometric/label truth validation; external source/header custody remains necessary.

## Correction and controls

The initial first-error replacement defect is preserved in initial-review.json. Final observe returns an existing failure immediately; reject/error paths use get_or_insert. Eight prospective controls now include a Write error followed by another callback and a RecordLimit followed by terminal. These exercise the adapter and real writer faults, not actual producer integration.

## Required producer qualification

- Actual solve_genus2_observed invocation through NativeEvents, compare full algorithm output and existing arithmetic counters against absent observer on a fixed qualified fixture. Cover identity and solved-edge early exits, duplicates, nonsplit, nodes, choices/restarts; report branches not hit rather than claiming coverage.
- Actual Labels::ingest_observed inputs covering first, redundant, invariant error, candidate verified/rejected and noncanonical label. Preserve separate fixed/planted/archive/natural provenance.
- Faulting Write/flush at identity, duplicate and label-result callbacks and terminal: propagate failure, no successful Outcome/complete Trace, original error retained; retain changed Labels state as failed-attempt side effect.
- Negative first-reference, stale node, unknown kind, incomplete/truncated terminal; post-failure callback attempts retain original cause. Existing manually authored event fixtures cover adapter parsing only.

producer-validation-plan.json leaves exact native fixture bindings pending. No acceptance of actual producer coverage or timing/cost equivalence is made.
