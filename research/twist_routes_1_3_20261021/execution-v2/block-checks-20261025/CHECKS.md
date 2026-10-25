# Checks for the bounded E26 wave

- Full root library: `root-lib.exit` is 0; `root-lib.stdout` records 4,492
  passed, 119 existing ignored. Network-enabled sandbox access permits the
  repository's local socket tests. No root library source was changed.
- Experiment package: `runner-tests-batch.stdout` records 13 + 13 + 13 +
  113 + 16 passing tests, 168 total, zero failures. These cover the final
  shared single/batch worker source at `497e54e7edf787a8743f216cd6967100f569ea61`.
- The two independently frozen Rust auditors verify raw hashes, complete-basis
  receipts, tuple/coefficient-row distinctions, counter coverage and raw Ir
  aggregates. Their output directories and actual exits are retained. The
  batch auditor emits byte-identical rank-replay input to the first audit;
  the existing accepted PARI replay checks all 90 ranks over source orders.
- `checker-controls.stdout` retains a passing positive basis, rejected
  deliberately truncated basis and expected vdim. Its unmarked-basis warning
  is raw stdout; it is not erased or treated as a new experimental failure.
- `backlog-validate.exit` is 0; all 16 twist cards validate. They remain
  blocked and unfunded. No allocation, lease or paid host was created.
- `packing.stderr` preserves the first synchronous extraction transport
  error. `packing-v2.exit` is 0: all member hashes and byte-identical
  extraction/repacking pass. The corrected packer uses asynchronous child
  processes and does no scientific computation.
- Both the local archive and the separately downloaded immutable S3 object
  pass complete member-list/byte/hash checks, restoring 39 loose records.
  The user-confirmed destination is `setec-astronomy-artifacts`.
- Final PDF: all four pages inspected; no overfull boxes or clipped tables.
  Editable graphs, recorded-value sources and matching benchmark copies are
  retained. The benchmark index gets an unadmitted diagnostic paragraph,
  with every admitted table row unchanged.
- STATIC_CHECK_V2.json checks 546 local Markdown links, Git sizes, credential
  patterns, PDF/copy hashes and identical rank-replay input. The first static
  check falsely rejected off-disk directory links because Git tracks files;
  its full rejected list is retained. V2 recognizes tracked directory prefixes.
- The first whitespace check flags byte-preserved Callgrind preflight lines.
  Two exact-path attributes now preserve those raw files; strict source and
  documentation checks remain enabled and the second check passes.

Binary versions/hashes, CAS receipts, raw stdout/stderr and the exact scripts
are adjacent. `TRANSPORT_NOTE.md` separates yielded tool statuses from saved
actual child exits; `BATCH_POST_RUN_RECEIPT.json` explicitly records the missing
launch timestamps. No missing field is replaced with a fabricated value.

GitHub CLI access later recovered and resolves the existing PR to
isogeny-labs/crypto#2380; the connector old path still returns 404.
DELIVERY.json records the actual pushed head and current-head CI separately.
The existing twelve oversized historical blobs
remain a separate full-PR merge hold. Neither is a scientific impossibility.

The evidence/index commit is now pushed and the PR title/body updated through
the canonical connector. The evidence head has a queued review and conflicts
with main; there is no all-green CI claim. Connector combined-status and
pull-request-workflow responses were empty, so the CLI check-run rollup was
read as well. The first CLI query used an unsupported `baseRefOid` field;
its empty stdout is retained, and the corrected query has an actual exit 0.
