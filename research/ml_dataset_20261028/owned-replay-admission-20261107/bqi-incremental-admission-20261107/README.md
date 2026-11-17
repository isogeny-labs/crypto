# Incremental original-owner admission for Boolean identity checking

This patch replaces the scheduler certificate bridge's full maximum-envelope reservation with original-owner admission of the BQI checker's actual resource increments. It changes no policy caps, original deadline, parent allowance, mathematical identity or source-authentication rule.

## Exact source audit

The source base is GitHub revision `ff02728b58311a34129d3472cac325334cc7b347`. The real owned-search policy caps sum to 2,491,392 modeled work and include 1,048,576 allocation bytes, while continuation work is at most 4,000,000. Consequently one full-envelope certificate may fit (with at most 1,508,608 work left for all other work), but two such checks inside one continuation deterministically exceed its work cap before other costs. A single small certificate is not universally impossible. Lower remaining work and repeated coverage/exclusion checks further restrict admission; the original global owner ceiling remains 16,777,216.

The earlier component fixture had the much larger BQI software envelope: 215,311,360 work / 67,108,864 allocation bytes, but a 20,000,000 certificate work/byte cap. It therefore failed its maximum-envelope reservation after serializing 59 bytes, before BQI execution. This was a reservation refusal, not a disproved identity.

Relevant source locations are the constants and checker factory in original_boolean_search_policy.rs, bridge::certify in dynamic_boolean_search_bqi.rs, Meter::charge and verify_untrusted in boolean_identity_bqi.rs, Refuted handling in dynamic_boolean_search_check.rs, and cumulative meters in dynamic_boolean_continuation.rs. Parent orchestration will update the policy's stale full-envelope comment; this packet deliberately contains only two functional source overlays.

## API and preserved accounting

`verify_untrusted_admitted(bytes, limits, deadline, admission)` takes an original-owner callback `FnMut(Resource, u64) -> Result<(), ()>`. Before each accepted local counter increment, including deadline probes, it requests the matching parent reservation. The native bridge maps every resource amount to its existing work convention and additionally maps AllocationBytes to bytes. No maximum reserve, refund, owner replacement or counter reset occurs.

The callback contract requires every successful return to enforce the SAME original deadline after reservation and postchecks. Actual bridge Meter::pay does so. Generic callers must satisfy that contract; the API does not wrap arbitrary callbacks in an additional unmetered clock check. Existing no-hook verify_untrusted/verify_expected behavior and all nine counters remain unchanged. Successful admitted checks have exactly the same nine counters as no-hook checks.

A refused request is preserved as `Unknown(Admission { resource, amount })`; the bridge installs the BQI audit before propagating its specific failure. Earlier successful requests remain in BQI counters. An ancestor can reserve successfully then fail its postcheck: in that case the rejected request may already be charged in the parent ledger, while the corresponding BQI operation/allocation has not started. The exact request and authoritative original ledger remain available; refusal is never labeled uncharged or refunded. Actual bridge controls exercise this case as well as a reservation that consumes the remaining deadline.

## Validation

Final `tests-reviewed.log`: 22 passing tests, consisting of ten existing BQI checks, seven new admitted-checker controls and five bridge controls. Tests cover legacy result/counter parity, refusal at every callback boundary, refused allocations, unchanged local caps, deadlines, malformed/unequal identities, retained ancestor reservations, and sharing one original budget across eight certificates.

Eight small certificates consume 1,575,408 modeled work and 2,099,744 reserved bytes in one unchanged 2,000,000-work parent ledger. Two old full-envelope reservations alone would require 4,982,784 work. These are modeled work/requested allocation counters, not retired CPU instructions or exact heap consumption.

The previous genuine Rust scheduler/search/encoder fixture now produces CandidateRetained, audit Ok and certificate Ok under its SAME 20,000,000 limits. BQI charges are `[59,111,18,4,17,8,6,786832,184]`; bridge totals are 787,379 work / 852,808 bytes. Independent decoding, all five node lineages and eight committed basis identities verify. The source identity remains synthetic and this is a component fixture, not a production CLI/dataset run.

The new fixture is 24,808 bytes, SHA-256 `e20530065820a7c47bcf99247e554ab6202197d19a0062816d7de5ed09d2c978`. Final-source re-emission is kept separately in fixture-final, with matching snapshot bytes. The historical producer BQI lacked the newer optional expected-source path; this packet uses the exact activated base with that path preserved, and the same untrusted arithmetic path as before.

## Preserved development outcomes

An initial test tried a 50,000-work parent ledger and failed because actual BQI accumulator allocations are charged as modeled work. Its source, log and binary are retained. The revised software control uses 2,000,000, below one old 2,491,392 envelope and below the existing 4,000,000 policy cap; no production allowance changed.

An intermediate 21-test run included a post-callback clock check. Review identified that it added an uncounted clock probe; that change was removed. Its log/binary is retained as superseded evidence, not a final-source result. Final tests instead assert the documented callback contract through the actual bridge's admitted reservation and postcheck. The final 22-test suite is authoritative for this patch.

## Replay and scope

Run `rustc +1.98.0 --edition=2021 --test harness.rs -o <new binary>` then `<new binary> --test-threads=1 --nocapture`. `bqi.rs` is the exact proposed Boolean checker plus a test-module registration; the bridge source is compiled directly. The bridge harness uses structural Owner/row declarations, not the complete original owning integration.

To replay the component fixture, compile producer/harness.rs with ROW_REPLAY_FIXTURE_DIR and ENCODER_FIXTURE_DIR pointing to NEW empty directories and run filter `emit_row_replay_fixture --nocapture`. This compiles existing actual continuation/search/scheduler/proof and encoding components, with structural original-owner/map metadata. Original fixture failures remain in the preceding polynomial-replay packet.

No full Cargo build, CLI link/run, cloud work, generator restart or Git mutation was performed by this packet. Applying the exact overlays and validating the whole original-owner integration remain parent tasks. No cap increase is requested.
