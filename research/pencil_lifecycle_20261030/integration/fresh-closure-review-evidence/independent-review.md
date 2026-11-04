# Independent source review: fresh full-journal closure v1

Scope: read-only review of the frozen patch, base and proposed source files, source manifest and retained software-test logs. No builds, mathematical workloads, cloud actions or shared source edits were performed by this reviewer.

## Identity verification

Patch SHA-256: `9e3c4da79d66e888954f47098823e6b17274c0b62e89c5e0174c0213302450da`.
Source manifest SHA-256: `aaea393734867356fe1b5db0bb27dbd33d8d00f9c03a32b15c3658582a0b1deb`.
All three base hashes and all three proposed hashes match the manifest. The two reused canonical/hash source files also match their recorded hashes.

## Findings

No blocking defect was found in this scoped change by source inspection.

The fresh and recovery entrypoints call the same `close_enclosing_owner` implementation. That implementation first calls `close_on_external_exit(journal_bytes, None)`, which decodes the exact journal, checks its schema, reconstructs its tail sequence through `append_tail`, and rejects invalid identities, interval ordering and sequence continuity. Fresh intake therefore does not replace journal validation with hash comparison alone. The consistently rehashed malformed-journal test specifically covers that distinction.

Fresh admission checks the independently supplied exact configuration digest, request, binary, image, Pod and Job pins; the receipt's complete binding value must equal the parsed pinned configuration. Configuration phase must be run, receipt exit must be zero, and replay cannot close this session. The common validation binds exact preparation and journal bytes, preparation-derived clock ID, post-exec binary/PID, clock namespace and monotonic ordering, publication intervals, and owner sync offsets. Fresh starttime has one producer observation, not a second independent assertion; the code explicitly documents that boundary.

The original recovery admission predicate remains equivalent: recovery schema, request digest and attempt zero are still required, and the original payload starttime comparison remains in that branch. The refactor does not convert recovery failures to fresh admission or reinterpret the older ExternalExit API.

The CLI requires an externally pinned controller-check digest; the controller record must contain exactly one run record and associate its exact owner digest and configuration digest with the requested Pod/Job/request. It then passes the original owner byte slice to the shared validator. The output owner receipt digest is SHA-256 of those original bytes, rather than a converted or reserialized recovery receipt. The controller change durably retains the standalone pretty-serialized check under its content digest before the later L2 gate checks. Existing enclosing timing remains a different endpoint from exact runner exit.

## Retained validation evidence

The supplied test-v3 log records 27 library tests and 4 CLI tests passing. The controller log records 39 tests passing. The supplied clippy-v4 and controller-clippy-v1 logs report successful completion. These are producer-retained software results, inspected but not independently rerun. Earlier logs remain present. This review does not establish an exact current shared-worktree build or CI result.

## Limits and follow-up boundaries

- External authenticity of pins and custody is explicitly a caller obligation. The CLI verifies byte/semantic associations, not an external signature or provenance service.
- This is trusted same-UID instrumentation. It does not establish a root-private observation boundary.
- The journal intake validates all serialized tail semantics and digest associations; it does not load each separately published ResourcePrefix file and independently revalidate its content. Prefix-file audit remains a separate artifact obligation. Empty journals can close a valid enclosing interval and are not evidence of completed scientific executions.
- The standalone check is retained only after maintained verification and the fresh owner check return successfully. Earlier failure capture/collection remains the responsibility of existing controller paths; this addition alone is not a general failed-job custody guarantee.
- Successful exit, valid receipt and enclosing wall interval do not establish a complete cohort, mathematical correctness, timing qualification or a cost ratio. The output leaves exact exit and active-worker time unknown and keeps the owner receipt publication tail pending.
- No fresh container qualification, cumulative allocated-worker accounting, protocol admission, replay equivalence or cloud deployment is authorized or demonstrated by this review.

Recommendation: suitable for integration with the stated narrow evidence semantics and subsequent exact-source software validation; not a scientific execution approval.
