# Primary N83 Gray 5+1 wide-XOR natural access: capped

The [frozen protocol](PROTOCOL.md) and [machine-readable pins](protocol.json)
were committed and pushed at `8d482f165ce` before this one public trial-zero
run. The primary `a=0` curve, first-pattern `gray_5_1.xcnf`, K600 sequential
base and disjoint K600 Gray residual pool came from the
[complete 11-object S3 input bundle](../linux-xor-complete-inputs-20261025/RESULT.md).
The runner checked all source, formula, archive, worker and solver hashes
against that bundle before starting a single CryptoMiniSat 5.14.7 process.
The formula was a natural query with no planted relation.

The solver returned `INDETERMINATE` at **5,000,000 conflicts** after
**1,264.671 seconds** of L0 process wall on a shared Darwin arm64 host. Its
manifest-bound process receipt reports 687,267 stdout bytes, zero stderr bytes,
SHA-256 `a3e6d97d4ca9720861fa91d7ffc38fc17c743cdda1ffb65bccb509f44f9a272e`
for stdout, and a peak-child-RSS upper bound of 1,039,515,648 bytes. The
solver reached the specified conflict cap before the 1,500-second solver-time
and 1,650-second external watchdog limits. Host load snapshots are retained;
the shared host cannot support a controlled cross-arm wall-time comparison.
The raw log records admission of 10 XOR matrices recovered from 3,486 XORs
near the start of search; that is an admission diagnostic, not a relation.
No complete assignment was emitted, so the exact group verifier was not
invoked and relation, closure, rank and descent measurements were not
available. This arm is a censored access observation, not a factor-base
ranking. `selected_best_total_runtime` remains `null`.

The [process receipt](evidence-01/process.json), [summary](evidence-01/summary.json),
[compressed raw stdout](evidence-01/solver.stdout.gz), [stderr](evidence-01/solver.stderr.gz),
[host-before](evidence-01/host-before.txt) and [host-after](evidence-01/host-after.txt)
snapshots are committed under [the artifact manifest](evidence-01/artifact_manifest.json).
The [independent auditor](audit.py) replayed the compressed package against
the freshly fetched S3 inputs and pinned binaries and returned
[`PASS_BOUNDED_REPLAY`](evidence-01/replay.json), preserving the terminal
status and exact conflict count. A scratch copy with a forged appended
`s SATISFIABLE` line was rejected at the process receipt/raw-stream gate;
the [negative check](evidence-01/tamper-check.json) records
`PASS_FORGED_SUCCESS_REJECTED`. The scratch copy was not retained as evidence.

The prior default-matrix Gray 5+1 access call reached 2,000,002 conflicts
and returned `INDETERMINATE` in 119.21 L0 seconds. The matrix limit and
host conditions differ, so this trial establishes only that wide-matrix
admission did not yield a verified natural relation by 5,000,000 conflicts
on the first pattern. The one-target cold objective still charges factor-base
construction, S3 transfer, failed relations, large-prime graph closure, rank,
linear algebra, unseen-target descent, verification, artifact I/O and matched
rho. A full IC attempt on an isolated Linux host, with independent replay and
matched controls, is still required before selecting the minimum-total-time
factor base.
