# Recovered execution provenance for the paired Kummer cohort

The [recovery receipt](paired-run-recovery-20261022.json) is an additive record
made after the Spot host migration. It does not change the original
`results/paired/` files or their [SHA-256 manifest](../SHA256SUMS).

The migrated Codex transcript records the full `paired_uncached` command,
its working directory, zero exit status, 1,507.197-second duration, and
combined output reporting 1,260 candidate solves and 1,260 paired rho
results. The retained executable has SHA-256
`87d724fd89fa8d22c225eb0fae742c64d0c4d9378b7d4721752d0be9eb137b92`.
Building the saved source again with the locked offline release command at
the same worktree path produced a byte-identical executable. The transcript
shows that a post-build edit to `paired_uncached.rs` was reverted before the
final source was saved; the rebuild checks the net result rather than relying
on file timestamps.

The separate [matched report](../MATCHED_REPORT.md),
[graph](../collector-cost-paired.svg), and [PDF](../KUMMER_PAIRED.pdf) describe
the recorded operation-cost comparison. An isolated analyzer rebuild and two
passes over copied raw inputs reproduced its five files under
`results/paired/analysis/` and the top-level `VALIDATION.json` byte for
byte. All 69 entries in the study's SHA-256 manifest and all 31 raw-input
pins in its replay certificate matched. A separate read-only curve-law check
confirmed `kP=Q`, the declared generator order, and curve membership for all
1,260 paired workload IDs across 21 distinct `(p, r)` groups; the paired rho
M/S/I counts also matched the archived same-workload controls for all 1,260.
Those latter read-only checks supplement the checked-in Rust analyzer; this
receipt's durable source-to-executable claim rests on its explicit hashes and
the byte-identical rebuild.

A later [isolated full replay](../reports/run-20261022T1354Z/RUN.md) ran the
retained executable on copied inputs with separate stdout, stderr,
environment, and resource capture. It exited 0 after 1,260 candidate and
1,260 paired rho solves. All 14 generated raw files and five independently
recomputed analysis files are byte-identical to the archive. Its first
copy-only launch failed before any target because the theta-model input was
missing; the failed attempt is retained in that run report.

The original run did not capture separate stdout and stderr, a standalone
environment snapshot, CPU time, or an isolation level. The 1,507-second
process duration is execution context, **not** a benchmark measurement. The
cost ratio in the matched report is a modeled operation-cost ratio, not wall
time. Neither the transcript recovery nor the analyzer replay establishes the
repository's full execution-proof-chain contract or a Lean proof of the
collector.
