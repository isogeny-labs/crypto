# Reproducible point-table pilot software

The first point-hashing study now has a fixed input recipe, complete schedule,
fresh-process executor, retained failure flow and paired diagnostic analysis.
The production study has not run. The [protocol](PROTOCOL.md) and
[canonical JSON](protocol.json) remain draft/unfrozen.

## Prepared pilot

- Registered 12/16-bit curves, thresholds 32/128, eight base seeds per cell:
  **32 contexts**, each with 256 public query points shared across all arms.
- The [compact recipe](pilot-plan.json) binds parameters, query/control hashes,
  expanded inputs and schedule; its raw file occupies 15,743 bytes.
- **2,368 planned child attempts**: 192 context controls, 64 forced-collision
  controls, 192 counted batches, 384 warmup and 1,536 measured batches.
- Seeded context/arm shuffles with paired reverse orders; a new child process
  per batch. Controls precede the timed schedule.
- Original absolute deadline, durable start/finish records, no overwrite,
  retries or automatic resumes. Analysis preserves failures and in-flight rows.
- Paired diagnostic ratios use one counted record per independent base seed;
  timing repeats and queries do not become independent sample units.

These describe the prepared software/manifest, not measured algorithm results.
[pcc:unverified]

## Development qualification

The final source reports **15 passing controls**, strict release lint, release
build and changed-file format checks. The canonical protocol validator reports
zero findings **as a draft**. The rebuilt native backlog validator accepts the
blocked card. The real CLI development profile finishes **76/76 counts-only
F17 children**, with zero failed/in-flight/unattempted rows and no native batch
clocks. These are unregistered producer observations; no signed PCC standing or
independent audit is asserted. [pcc:unverified]

That development profile has two positive-control contexts with full pair
sumsets. Their negative controls are mathematically unavailable and are
explicitly reported as such. Separate unit controls exercise empty bases and
complete negatives. No 12/16-bit decomposition oracle batch or timing study was
executed; input/known-control generation is pre-data preparation.

Exact commands, source/binary/recipe hashes, development flow and control
coverage are in [qualification.json](qualification.json). All 661 raw
development artifacts—including the initial compiler failure, its exact source
snapshots, validation logs and both real CLI controls—are retained losslessly in
[orchestration-development.tar.xz](orchestration-development.tar.xz). Extract
into a fresh directory, then inside `orchestration-development` run
`sha256sum -c SHA256SUMS`. The qualification also pins the archive's SHA-256.
The archive was created before its first commit; earlier evidence is untouched.

The [current source manifest](source.sha256) identifies the new example/module,
named native/word/statistics dependencies, compact recipe and draft protocol.
Remaining tracked library inputs are unchanged from the qualification's base
commit. Older batch-only receipts remain tied to their own source hashes.

## Still required for the experiment

Complete native field/hash/allocation counts and calibrated prices, external
references, masked canonical export, and native L2/host/rusage/image integration
remain pending. The analyzer currently refuses timing comparisons and marks
count intervals descriptive. Its outputs cannot promote a runtime gain,
equivalence, fastest-tool result or complete ECDLP price.
Root/add/meter diagnostics do not charge key, probe or allocation work; equal
ordered/hash counts therefore cannot establish equal runtime or table cost.

Finish test-specific power/runtime calibration and freeze the completed
canonical protocol/analysis before affected data. Execution still requires
current account/lane headroom, fresh exact-source S3 write/readback and an
authenticated native worker/image. This software qualification submits no job.
The remaining four component studies and fresh-target ECBench confirmation
stay in the [original backlog](../../../backlog/XP-20261105-native-point-hashing/README.md).
