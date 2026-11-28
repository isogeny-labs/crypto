# Test-only authentic durable publication observer proposal

This packet supplies a concrete source patch, not an activated or compiled change. Candidate 3517ae554787690472e3ed9801623de27730e20d remains untouched. No Cargo/build, standalone compilation or scientific execution was performed.

The proposed cfg(test) child module exposes one thread-local function-pointer observer with an RAII guard that is not Send/Sync. Installation refuses replacement on the same thread. A borrowed event contains only numeric indices/lengths and a borrowed actual CommittedEventPosition. There is no mutable budget, owner, sink, source identity or verdict access. The safe callback cannot retain a borrowed acknowledgement past the callback. Normal non-test builds have no observer code or registration.

Publication has two observation points:

- ChunkAcknowledged follows successful CaptureEmitter payload/head installation, installed acknowledgement and exact event-count/binding checks. The existing original-budget postcheck still follows the callback. A receipt here is custody evidence and may precede a subsequent deadline refusal.
- SnapshotComplete follows encoder completion, exact chunk acknowledgement/byte-total checks, the original budget check and assignment of snapshot.complete. This proves complete wire publication only. A cfg(test) budget postcheck after callback prevents a slow returned observer from turning an expired original deadline into successful publication.

The observer does not select a favorable checkpoint or manufacture rows. Its callback may write bounded metadata through a preopened external control channel and park a dedicated test child for supervisor termination. Notification overhead must be labeled test instrumentation, outside any scientific timing/instruction comparison. A callback panic remains a test failure; it cannot manufacture an admitted interrupted outcome.

## Exact child/supervisor wiring still required

A new in-crate test module under solvers must instantiate the authentic frozen fixture described in ../owned-authentic-historical-fixture-plan-20261107/DESIGN.md. Existing verified_worker_original_tests::Fixture private builder hardcodes m=3 and synthetic code hashes; it needs a new authentic builder for proposed n23,m2 and actual binary/source identity. Do not mutate bindings after setup. Existing authentic admission test in sat_original_owner.rs provides the actual factory path.

Run that test child with a fixed exact test name and one test thread. The callback must be installed in the SAME thread that calls the actual producer. A parent may use current_exe for the built libtest child, but must set the test's ordinary finite execution context and record its actual binary digest. Start one child only; finite parent watchdog and process-group cleanup are required. A private control pipe/path must be separate from cell artifact data and opened before owner execution. The child may NOT consume an environment-supplied serialized original/token or substitute a fake sink.

Before starting, freeze the desired boundary and total notification allowance. For complete checkpoint, use an actual native-batch checkpoint rather than the initial pre-Matrix snapshot; selected nonempty rows still require central mathematics. For partial tail, the chosen chunk cut must include complete identity and remain before wire completion. Original map size means first-chunk/789-byte assumptions are invalid. A calibration dependency may remain; retain a nonqualifying run rather than choose a different sample silently. The observer API deliberately does not decide these scientific/admission questions.

When the desired boundary arrives, the parent verifies reported indices against freshly read durable head metadata, terminates the parked actual child within the original launch deadline, then uses production terminal_outcome, requested_channel_disposition, collect_cell_artifacts and CellManifest sealing. The packet does not invent a fake supervisor envelope or claim this wiring exists. Central verify_historical and same-owner revalidation/projection run only on the sealed genuine artifacts under a separate explicit finite replay policy. Native owner limits are never renewed.

## Read-only validation and remaining dependency

Observer patch is generated from exact hashed publication source. The original source bytes were rechecked unchanged. Patch text adds only cfg(test) registration/notifications and the new test-only module; it leaves production publication logic in place. No compile or runtime result is claimed. Borrow/type checking, observer guard tests, child factory implementation, authentic cell generation, exact partial-tail cut qualification and real central replay remain outstanding.

The minimal independent deliverable is this observer patch. A complete runnable child cannot be honestly implemented using existing fixture APIs alone: m/source identities are hardcoded, deterministic launch/termination orchestration is absent, and authentic producer execution cannot be replaced with the earlier standalone supplied-Original harness. Those dependencies are explicit implementation work, not new authorization.

Prior source fallback: read the saved authentic-fixture DESIGN.md, actual owned_search_capture_publication.rs Bridge/checkpoint_parts, capture.rs committed-head installation, sat_original_owner.rs authentic admission tests, and solvers.rs fixture constructors. Parent reports KB tool unavailable and sparse research-kb helper absent; exact local source inspection is the fallback. No literature/novelty claim is made.
