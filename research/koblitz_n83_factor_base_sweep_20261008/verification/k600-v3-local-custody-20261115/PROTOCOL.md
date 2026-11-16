# Primary N83 K600 v3 local-store construction control

**Frozen before execution, 2026-11-15 UTC.** This is one source-bound
construction and custody control for the native v3 producer. It does not run
an ECDLP relation search or choose a minimum-runtime base. The separate frozen
[K600 m6 search protocol](../k600-full-smooth-m6-20261114/PROTOCOL.md)
and its unused additional search allowance are unchanged.

## Exact input and hypothesis

- Curve: `icv1-f2m83-tm6151469093347-debefd74`, primary `a=0`, subgroup
  order `2417851639230796216685689`, cofactor 4.
- Construction: `public_x_hash`, seed `2026100801`, `K=600` signed-Frobenius
  columns. Expected complete base size: 99,600 subgroup points.
- Historical exact-set control: point-set BLAKE3
  `ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21`
  from the separately retained K600 input named in the linked search
  protocol. Compressed bytes may differ because v3 uses a new producer and
  manifest; point-set equality is the control.
- Hypothesis: the current native v3 producer can construct this complete
  base, independently replay its points and signed-orbit labels with the
  generic curve path, publish it to a temporary local content-addressed
  `file://` store, and fresh-pull and import the exact object.

## Frozen execution and limits

- Source: the clean commit containing this protocol; record its full SHA in
  `attempt-1/attempt.json`. No source edit or retry in the same attempt.
- Run directory: `verification/k600-v3-local-custody-20261115/attempt-1/`,
  created only by the Rust command. Raw stdout, stderr and terminal exit are
  separate write-once files in the protocol directory.
- Local remote: `file:///Volumes/SSD990-2/tmp/n83-k600-v3-local-custody-20261115/remote`.
  Its index and cache are also under that temporary directory. This control
  does not write to S3 or to the tracked `store/index.json`.
- Native command: `primary-store-v3-construct-publish` with arguments
  `attempt-1 public_x_hash 600 2026100801 600 <exact local remote>`.
- Construction watchdog: 600 seconds. External process cap: 900 seconds
  (`gtimeout`), including build/startup, local publication and fresh pull.
  A timeout, cap, error or nonzero exit is retained as its own outcome.
- Admission: at least 2 GiB free on the run filesystem and an existing
  `gtimeout`, Rust 1.99.1 compiler, and clean checkout. Record host, free
  disk, load, command, environment overrides and start/stop UTC. Host wall
  and construction wall are L0 diagnostics, not comparison measurements.
- Positive path: the same touched release example suite must pass before
  launch; its existing K2 v3 publish/fresh-pull control detects a broken
  producer. The full K600 object then gets its own generic replay, fresh
  pull and importer checks.

## Acceptance and claim boundary

Promote this **construction/custody control only** if the command exits 0;
`manifest.json` is completed; `replay.json`, `storage.json` and `import.json`
all report `PASS`; 600 representatives and 99,600 points are independently
checked; the object SHA-256 and byte count match the retained object bytes;
the fresh pull matches the publication receipt; and the point-set BLAKE3
matches the historical exact-set control. Keep a changed set, failure,
budget cap, source mismatch or incomplete receipt as a separate terminal
result, not a success. Preserve the raw files and make no automatic retry.

The temporary remote is not durable S3 custody. Actual astronomy-bucket
publication requires the owner's exact destination and writer identity,
then a separate verified write and fresh GET. Even a passing control does
not measure natural relation yield, rank, logs, descent, verification,
matched rho or cold total IC runtime; `selected_best_total_runtime` stays
null. No performance ratio or factor-base winner follows from one build.
