# Owning checked-source search and worker transport proposal

This is a concrete external source proposal that removes the borrowed-phase lifetime obstacle. It is uncompiled and unexecuted; frozen SAT recovery inputs and shared production sources remain unchanged.

## Reuse the existing owning expansion

`checked_boolean_equations.rs` already defines `OwnedExpansion` owning the authentic CheckedMap plus full sparse attempt and owner failure, and `expand_owned(CheckedMap, Limits, &mut impl Admission)`. No new duplicate expander/API is needed. The map is moved, not cloned or reconstructed from an authentication boolean. The existing type exposes the exact source, ordered equations, variable count, status and charges. This proposal calls that existing API directly.

## Concrete consuming transition

`OriginalOwner::into_boolean_search(self, actual_sink, Caps) -> OwnedAttempt` first checks its one-shot state, actual map publication acknowledgement, original attempt binding, sink directory and limits, and original deadline. Frame/reporting-control resources are prepaid through the existing original ledger before fallible sink validation, following the already implemented Boolean-row owning transport pattern.

Regardless of admission success, it moves every OriginalOwner field into OriginalParts: genuine Budget, Math (including staged signed_target), CheckedMap or failed unchecked map, source hashes, binding and launch digests, identity pattern, retained map bytes and acknowledgement, publication flags and capture limits. It also moves the actual CaptureEmitter. The verified borrowed token is consumed only to retain its fixed source hashes; no borrowed token/map/ledger survives in OwnedAttempt. No fresh owner, sink, context, cloned source, unsafe cast or lifetime erasure is used.

Expansion takes the map by value and is immediately retained even on failure. Normalization preserves ordered generators while converting only polynomial term order, under the same ledger and construction deadline. Search V2, its full node/matrix state and the independent successful-result audit are retained by value. The independent checker validates branch coverage, conditional source lists, exact Boolean models and each BQI refutation under that same original ledger/deadline. All failures return the actual ownership container. There is no separate generic-owner argument exposed to callers.

The Boolean candidate accessor requires successful search and structural/arithmetic audit, and is gated off by adapter failure. It still exposes only a gate assignment; signed group lift has not been implemented. The independent audit's source_authenticated flag remains false because the generic checker alone does not authenticate equations. Genuine source custody comes from the containing original-owner adapter, not from changing that flag.

## Actual proposed worker patch

`worker-transport.patch` adds a dedicated owning slot to WorkerResidualCustody and a WorkerSolveOutput constructor that moves OwnedAttempt into it without Box/clone/sink reopening. All explicit old custody initializers are updated. The parent's finish_retained_worker_output first adopts custody and only then returns a dedicated report-unavailable error. This preserves the owner on the existing parent/telemetry/error route without fabricating a solved or published outcome. The patch deliberately has no scientific CLI dispatch yet: publication and central original-source replay must be added before an outcome can be admitted. This is a transport integration change, not a replacement of the requested successful CLI behavior with permanent failure.

## Source registration and compilation

Install original_boolean_search.rs plus the scheduler/search/bridge/checker donor files under src/ml_dataset; add the minimal registration.patch to the final sat_original_owner. The arithmetic checker is included from the canonical ../cryptanalysis/boolean_identity_bqi.rs. Apply worker-transport.patch only to a separately prepared integration source. The proposed solvers file is provided for exact review, not as a stale source transplant.

SOURCE_BINDINGS.json pins the staged original owner, canonical owning expansion, Budget, worker parent and all mathematical donors. Scheduler differs from the reviewed donor only in its search module filename. Search V2, arithmetic bridge and structural checker are byte-identical donors. No tests for this assembled adapter have run; their standalone component tests are not relabeled as adapter validation.

Required next verification: compile against exact final-source modules; genuine original owner + sink + checked map tests covering drop of the lexical original-token scope, return through WorkerSolveOutput/parent adoption, post-reservation failures, exact same ownership_id/deadline, admitted nontrivial n23,m4 rows and both twists; independent reconstruction of checked source plus branch audit. Resource checking must include frame/custody size changes. The existing normalized-source and search scratch limitations mean this is retained in-memory ownership, not a restart checkpoint.

Next implementation remains prepaid durable search/audit capture and acknowledgement, central source reconstruction, genuine signed lift from retained Math/domain, full exceptional-pattern coverage, explicit versioned CLI dispatch and appropriate exported public/restricted records. Owned transport now has code rather than an unsupported lifetime cast, but the complete dataset CLI path is not claimed finished.

No full-library build, retry approval consumption, generation, cloud or Conductor action occurred.
