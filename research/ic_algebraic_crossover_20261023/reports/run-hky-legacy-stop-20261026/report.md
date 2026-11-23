---
title: "HKY replay: four legacy caps and controlled stop"
date: "2026-10-26"
geometry: margin=22mm
fontsize: 10pt
colorlinks: true
---

The legacy batch stopped after four actual 900-second timeouts. Zero primary
rank/verdict outputs completed, so the independent replay remains inconclusive.
There is no independent solver-word count, verified A/A rank pair or minimum
guess count from these executions. This report describes that stopped batch;
the subsequent native continuation has a separate prospective registration.

## Fixed workload and observations

Original protocol: `protocol-hky-restart-20261026.json`, frozen before launch
at 19:50:38 UTC. It registered both real arms on all 124 fixed draw/guess/branch
tuples: 248 children, single thread, 8 GiB address-space limit, 900 seconds per
child and no timeout retries. The source/build executable, input and producer
receipt were independently pinned. Fresh processes test determinism; all clocks
are invalid for speed inference. Host: the recorded shared Linux workspace,
without admitted performance isolation. The orchestration was historical
Python/Bash around the unchanged native scientific executable.

| Actual original tuple | hky | hky-aa |
|:--|:--|:--|
| d0-k0-b0 | timeout | timeout |
| d0-k1-b0 | timeout | timeout |

The native aggregate records 248 planned, four terminated, zero verified,
four timeouts and 244 unexecuted. There are zero verified A/A rank/verdict
pairs, no observed mismatches among such pairs, and all registered-prefix
minima remain null. Zero checked pairs cannot establish agreement. Censored
metrics and independent solver words remain absent. The original producer's
one timeout and unknown cost are unchanged.

![Counts at the controlled stop; unit: registered child processes, fixed census. There are no complete comparable primary metrics, so the diagram shows execution states rather than a performance comparison. Source: flow.dot, recorded-summary.json and the immutable legacy capsule.](flow.pdf){width=155mm}

## Stop, provenance and restore

The controlled pause occurred at 21:10:27 UTC after the last completed case's
checkpoint. No fifth case had a start marker. The recorded raw manifest check
returned 0. The old batch launcher returned 143 for the intentional stop;
the pause watcher reported outer exit1 despite its successful state files.
Both observations are retained and neither is counted as an extra scientific
failure. Reviewing restore ordering found PID/PGID metadata being rewritten
before its old checksum check; the execution path also used prohibited Python
orchestration. These are implementation defects, not mathematical obstructions.

The immutable legacy capsule is 18,132 bytes, SHA256:

`2f6d57ccdaf1738a8a1a7895559505a02a9f66abb948d09a221472ade95e812c`.

Use the repository artifact pull on
`evidence/runs-hky-legacy-orchestration-20261026.tar.xz`, check the sibling
SHA256 file from that directory and extract separately. The capsule retains
the entire raw tree, original protocol/source and controlled-stop receipts.
Its genuine binding and [actual S3 GET receipt](../../pipeline-receipts/legacy-hky-partial-20261026/remote-byte-replay.json)
match the bytes. Original execution source is `e01cb6ebbf05`; the protocol's
canonical digest is:

`32fbd8c1e4d6b6d1f7dcde2cd06084a28551ad4954922621193846b862277f31`.

The unchanged scientific executable is bound to source/build `a214e78d7`.
The complete hashes are in the protocol and capsule. Its HKY/Kosters--Yeo
literature, exact fixtures and bounded-degree limitations are recorded in
PROTOCOL.md and COMPLETED_GUESS_REPLAY_PROTOCOL.md. This stop adds no degree,
cost-scaling or full IC conclusion.

## Continuation boundary

The new Rust controller verifies old manifests before PID/host writes and
replaces the relevant Python execution path. Eleven tests, strict all-target
lint and release build pass on its recorded source. A separate prospective
122-tuple, 244-child protocol froze before new data; legacy timeouts are never
retried or imported as new prospective rows. The first actual native child
started 22:35:44 UTC after quiet S3 checkpoint/archive/meta/resume/source-bundle
readback. That launch is not a completed replay; see
[native transition and launch receipts](../../NATIVE_HKY_CONTINUATION_20261026.md).

The total additional-cloud ceiling remains USD100 including replay, no extra
VM launched, and workspace billing is unmeasured. Current-head review/CI and
remaining large-memory/full-price work remain open. Canonical cost and degree
figures are unchanged: this censored result supplies no admitted new value.
