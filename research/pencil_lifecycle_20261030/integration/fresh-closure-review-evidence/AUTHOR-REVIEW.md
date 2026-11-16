# Fresh full-journal closure consumer — private source proposal

Status: implemented and software-validated in a small isolated crate containing the actual relevant root source modules. Root checkout was not edited or built. No cloud calls, images, experiment, replay workload or mathematical measurement ran. No budget/deadline was changed. The original empirical task remains incomplete.

## Exact scope and reuse

Read existing root ecbench_owner_closure.rs, diagnostic_resources.rs, canonical.rs and hash/sha256.rs, plus prior owner evidence reviews and the frozen fresh-owner producer/wiring. This proposal reuses the actual SessionResourceJournal validator. It does not manufacture a recovery-owner receipt or introduce a second journal implementation.

`close_enclosing_evidence` and the new `close_fresh_enclosing_evidence` dispatch to one private `close_enclosing_owner` body. That body still calls `close_on_external_exit(journal_bytes,None)` before checking owner/preparation associations. Thus exact JSON bytes, duplicate rejection, journal schema/identity, per-tail identity and monotonic publication intervals, scientific identity consistency, hash shapes, clock namespace/order, preparation origin, exact preparation-derived clock ID, binary/attempt identity and final observed interval constraints remain shared. Old recovery admission still requires its own schema, request and attempt0, plus its top-level PID-starttime association. No old recovery condition is silently removed.

FreshOwnerPins explicitly carries caller-authenticated exact config SHA, request SHA, binary SHA, image SHA, Pod UID and Job UID. The fresh API independently checks these against bounded duplicate-free exact config bytes and the fresh owner's matching binding/config hash. It requires phase run, successful child exit, null asserted exact exit and the existing no-findings/observed-reap conditions. Replay has no runner-session closure and is rejected even when its own config and receipt are consistently rehashed.

Fresh receipt has its observed PID starttime only inside postexec_runner_observation, unlike recovery's separate top-level copy. The common validator explicitly accepts that source field for fresh mode and still requires its nonempty starttime and PID/binary binding. It does not synthesize a top-level claim. Actual clock/preparation/journal semantic checks remain identical. Returned owner_receipt_sha256 is the original fresh receipt's exact hash.

## CLI and custody

The existing CLI retains its original invocation. Its new form is:

```
ecbench_owner_closure --fresh CUSTODY CUSTODY_SHA OWNER CONFIG CONFIG_SHA PREPARATION JOURNAL EXPECTED_REQUEST_SHA EXPECTED_BINARY_SHA EXPECTED_POD_UID EXPECTED_JOB_UID EXPECTED_IMAGE_SHA
```

Every expected argument must come from independently authenticated controller/artifact evidence, not by copying self-asserted values from an untrusted bundle. Fresh custody is an exact externally pinned `isolab.fresh-owner-controller-check/v1` record, bound to expected request/Pod/Job and one exact run receipt/config pair. The CLI rejects duplicate JSON and duplicate run entries before calling the actual full-journal API. It reads only bounded regular files and supplies no external authentication by itself.

The small controller delta saves the existing successful identity/custody check as a digest-addressed top-level JSON file before L2/AA interpretation, using existing immutable retention and EXIT upload. This makes the direct custody input available instead of requiring a caller to relabel a nested fragment. Saving an observation check before L2 acceptance does not qualify the measurement; CLI timing_qualification and mathematical_result remain null.

Output reports `closed_fresh_enclosing_evidence`, with the same distinct session-origin-to-owner-post-reap-evidence-sync wall estimand. It keeps exact_runner_exit_ns, total_active_worker_ns and worker_allocation_ns null, and keeps owner_receipt_publication_tail_pending true. Trust context explicitly states same-UID instrumentation, not root-private custody. This closes the full journal semantic intake gap for that estimand; it does not close publication-tail, replay allocation, physical teardown or campaign accounting gaps.

## Frozen artifacts and bases

Patch `fresh-full-journal-closure-v1.patch` SHA2569e3c4da79d66e888954f47098823e6b17274c0b62e89c5e0174c0213302450da changes only root diagnostic_resources.rs, root ecbench_owner_closure.rs and controller cloud.rs. `source-manifest.json` SHA256aaea393734867356fe1b5db0bb27dbd33d8d00f9c03a32b15c3658582a0b1deb records exact bases/proposed hashes and unchanged reused canonical/SHA source hashes. Root source bases are the currently reviewed enclosing API; controller base is the independently reviewed frozen fresh wiring patch890bf747a00d03af14508dda25bea72f5296a09c41fa50ff0f831e84738d40bf on campaignV2. Existing frozen native/package/wiring artifacts were not edited.

The private Cargo.toml/src/lib.rs only assemble these real source modules for validation. They are fixture infrastructure and are not proposed replacements for root Cargo.toml or lib.rs. No dependency shim or alternate SHA implementation was used. Controller validation likewise reused the frozen actual controller sources plus the standalone custody-save delta.

## Software validation and retained failures

`test-v3.log`:27library tests and4CLI tests passed. These include existing SHA/canonical/resource cases, a fresh valid nonempty publication journal, a consistently rehashed backwards journal, six independent pin mutations, missing session binding, wrong PID/starttime/clock namespace, replay rejection, exact custody/owner mismatch, duplicate run custody and direct --fresh CLI dispatch through bounded temporary files. Fake clock values are software fixtures, not timing observations. Reused old recovery closure fixtures passed unchanged.

`clippy-v4.log`: all-target strict Clippy passed using the root's existing chunks_exact_to_as_chunks=allow setting for unchanged SHA256 source. Initial `clippy-v3.log` failed because the small fixture Cargo.toml omitted this existing root lint policy; no production source or gate was weakened to address it.

`controller-test-v1.log`:39controller tests passed. `controller-clippy-v1.log`: all-target strict Clippy passed. These validate source compilation and existing software fixtures; they are not AWS/Job/runtime tests.

Initial tests v1/v2 failed because the new negative fixture incorrectly assumed that the first journal sequence must be0. Existing append_tail requires increasing later sequences and permits another initial value. The fixture was corrected to a genuinely backwards durable-publication interval while consistently rehashing journal/owner references; the shared production validator was not changed for that failure. Earlier failed logs remain retained.

Remaining: independent source review; root application after current freeze; required full root checks; pinned compilation/publication of an actual intake binary when needed; authenticated receipt acquisition; real fresh-epoch evidence and timing qualification. No claim is made that the four-lead experiment task, all-cost ratio, full1260cohort or publication is complete.
