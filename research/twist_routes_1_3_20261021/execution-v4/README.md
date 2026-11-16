# Twist-derived progression bases, 2026-10-27

The fixed geometry and cost diagnostic is executed and independently checked.
[Results and inspected PDFs](../reports/progression-20261027/RESULTS.md).
All 880 cost cells were attempted: **751 accepted**, **105 cooperative-budget
exits**, **24 underfilled bases**. Geometry verifies 240 construction records
and 7,680 query records; 214 bases fill. The separate 60-call preflight supplies
operational sizing only. These are correlated toy tasks, not complete IC costs.

Within one base, ordinary group MITM costs less than every admitted polynomial
comparison. Helpers/direct median Ir is 0.8937482746077081 on 124 pairs, but
helpers/group median is 2.3585326491871. Companion/direct median is
1.352375384073076 on 38 pairs. No companion arity-three case completes within
the fixed budget. Different constructors produce different answer sets; their
raw cost ratios are descriptive, not equal-output speedups. No twist-specific
or NIST-size gain is established.

## Evidence and limits

* [Frozen source and binaries](FROZEN.json), original [protocol](PROTOCOL_V1.md),
  [incoming-rule amendment](AMENDMENT_V2.md), [local-fallback and phase
  amendment](AMENDMENT_V3.md), and three frozen `*.protocol.json` records.
* [Independent geometry audit](geometry-audit-v1/AUDIT.json),
  [cost audit](cost-audit-v1/AUDIT.json), [final strict PARI
  admission](cost-audit-v1/ADMISSION.json), and [stack-only replay
  amendment](PARI_REPLAY_AMENDMENT_V2.md). The audit's original
  PARI_replay_admitted=false is immutable; the later admission is authoritative.
* [Masked analysis](blind-v1/MASKED_ANALYSIS.json), prior
  [inspection](blind-v1/INSPECTION.json), exact [unmasking](blind-v1/UNMASKING.json).
  Numeric signatures can reveal methods; this is not an independent analyst.
* Native `export-v3/{geometry,preflight,cost}.export.json` and
  `export-v3/STRATIFIED.json`; hydrate large exports through their manifests.
  [Structured binding gap](EXPORT_LIMITS.md): standalone schema/file checks pass,
  but the canonical factorial-protocol binding rejects panel/control metadata.
  Original protocols and earlier failed exports are preserved, not rewritten.
* [Host](HOST.json), [root checks](ROOT_CHECKS.md), and `checks/` hold actual
  commands, exits, failures, environment and build provenance. Six unchanged
  incoming root regressions remain. A legacy Hilbert operation-count threshold
  intermittently fails the parallel package test; isolated and serial checks
  are recorded without changing its threshold or the frozen solver.

The standardized export counts 775 conclusive process completions, including
24 unsupported failures; this is distinct from 751 accepted scientific outputs.
No native performance time, RSS, GAE, fastest-external claim, matched-rho cost,
full relation algebra or DLP recovery is admitted. No VALIDATION.json or
leaderboard row is promoted. Original full-campaign criteria remain open.

## Artifacts and replay

The user-confirmed bucket is **setec-astronomy-artifacts**. The maintained
`.artifacts.json` files bind every moved path to its immutable content-addressed
object under `s3://setec-astronomy-artifacts/store/sha256/`. No session writes
the workflow-owned reports prefix.

The new raw tree is one deterministic `runs.tar.xz` (4,639,704 bytes), SHA256
`225c00f390768d6a3bae8b89e3635fc51101b09be4452991dec54a9c59fe9f56`.
[Fresh-download verification](RAW_VERIFICATION.json) checks all 5,867 member
hashes against RAW_MEMBERS.json and identical deterministic repacking. Earlier
committed trees are not repacked. Individual raw paths resolve after extraction;
the unified exports reference the archive/member manifest rather than thousands
of loose files.

From the repository root, with authorized artifact credentials:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v4
cd research/twist_routes_1_3_20261021/execution-v4
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Rebuild the scientific worker and harness from source revision
`d01d7bb63cc577a4299200ae3c560767b6d68650` with Rust 1.90.0, the pinned Cargo.lock,
`--release --offline --locked`; the two local F4/hash snapshots are intentional
baseline inputs. Verify binary/source hashes against FROZEN.json before replay.
Use fresh output directories. Example native commands (paths relative to root):

```sh
harness audit-geometry V/PLAN.json V/runs/geometry-v1 FRESH_GEOMETRY_AUDIT
harness audit V/PLAN.json V/runs/cost-v1 V/runs/geometry-v1 FRESH_COST_AUDIT
```

Here `V=research/twist_routes_1_3_20261021/execution-v4`, and `harness` is the
frozen `twist-progression` binary. The original full argv/env/exit receipts
live in each extracted TRANSPORT/RECORDS file. Run the generated verify.gp using
the pinned GP with initial stack 64,000,000 bytes, unchanged 4 GiB stack ceiling,
actual exit/output hashes, then the frozen harness `admit AUDIT RECEIPT`. The
post-data `twist-pari-transport` only records that transport; it changes no
scientific equation, dataset, analysis or stderr acceptance rule.

Repacking uses tar sorted names, GNU format, mtime 2026-10-27 00:00:00Z,
owner/group zero, mode u+rwX,go+rX,go-w, and xz -6 -T1; exact commands are in
support/package-evidence.mjs. Geometry/preflight/main are separate populations;
do not pool them or open final E28 labels 32–63.
