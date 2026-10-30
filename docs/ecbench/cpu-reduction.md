# CPU hashed-ECDLP reduction lane

## 2026-10-29 PDT: completed implementation and matched checks

### What was done

Added the standalone [`hashed-ecdl-cpu`](../../crates/hashed-ecdl-cpu/README.md)
package. It reuses the checked shared reduction and supplies a CPU MITM oracle
with persistent tuple-sum tables, lexicographic witness selection and Montgomery
batch inversion. Cache identity uses modulus, coefficients and full prime order;
it does not compare or copy derived small-field square-root lookup tables.

The public-input CLI emits a JSON certificate and independently replays it.
`--verify` checks an archived certificate. The shared order-witness constructor
supports prime fields below 2^60; explicit table, column and trial caps remain.
The new CPU path has no CUDA dependency and needs no local root plug-in build.

Validation: 20 standalone release tests, including all-target oracle comparisons,
all-pair exceptional affine arithmetic, 36-bit arithmetic, equivalent validated
curve representations, complete certificate equality and corruption rejection.
Strict Clippy and formatting pass. Root crypto was not compiled locally, as
requested. The existing tensor crate is a dependency, unchanged by this lane.

Two matched EKS panels each completed 120 executions (108 measured, 12 warmups).
Each has exact cross-arm full certificate equality, replay, negative controls,
a frozen native analyzer and a schema-checked export. In the final panel,
16-bit complete-recovery medians were 45.993793 ms versus 3.903194 ms (3 slots), and
80.536765 ms versus 4.118365 ms (4 slots). These are shared-host L0 diagnostics on
AMD EPYC 9R45, one software thread, not certified timings or fastest-tool claims.

Sources and results:
- [Initial protocol and immutable results](../../research/hashed_ecdl_cpu_20261030/protocol.json).
- [Final matched protocol](../../research/hashed_ecdl_cpu_20261030/panel2/protocol.json).
- [Final checked export](../../research/hashed_ecdl_cpu_20261030/panel2/results/export.json).
- [Final PDF report](../../research/hashed_ecdl_cpu_20261030/reports/panel2/report.pdf).
- [Bounded public-target ladder](../../research/hashed_ecdl_cpu_20261030/scale/protocol.json).

### Decisions made

T-878 works in a separate worktree/branch from active GPU task T-877. Shared
wide-field and deferred-dependency improvements were merged only after their
source was committed upstream in that branch; this lane does not modify those
files. Compare arms within each panel: the shared reduction changed between
panels, so cross-panel differences cannot isolate the cache change.

The source includes the existing root reduction verbatim, preserving its
independent checker. Prefix tuples and equal-sum suffixes keep the original
lexicographic winner. A cap returns a limit error, never nonexistence. Standard
batch inversion follows Bernstein/Lange's affine-arithmetic/precomputation
analysis: https://www.hyperelliptic.org/EFD/precomp.pdf .

### Open threads

The separately frozen 20–40-bit CPU ladder uses public target points without
constructing their logarithms. Its job reached the 1500-second scheduling
deadline with no recovery rows or
certificates; the last observed pod was unschedulable behind another exclusive
CPU job. The terminal receipt and incomplete zero-execution export are in
`scale/results/`. No larger-field CPU recovery is claimed, and no retry was
launched. The frozen instrument remains ready for free CPU capacity.

Matched rho, fastest external implementation, strict isolation, and independent
scientific review remain absent. No global leaderboard or benchmark-index row
is promoted through the repository's validation gate. Table-byte counters are
retained capacities, not total peak RSS. This is generic decomposition, not the
paper's conditional Boolean tensor contraction or a subexponential claim.

### Errors

The initial runtime staging encountered S3 HeadObject 403 and a missing tar
executable, before any benchmark ran. Immutable protocol amendments preserve
both failures. Successful runs used per-object short-lived URLs and standard
Python archive transport; all research computation and analysis stayed native.
The larger ladder encountered an existing exclusive CPU allocation; its queued
state is tracked separately and no other workload was stopped or changed.

## 2026-10-29 22:34 PDT: further 10x iteration

### What was done

T-892 reserved the shared reduction after its previous reservation was clear.
The arithmetic candidate (Euclidean inverses, finite-difference constructor,
fixed-base scalar tables) passed 24 release tests, strict Clippy and format
checks. The full certificates still match all three archived baseline outputs.
A new frozen [screen](../../research/hashed_ecdl_cpu_20261030/round3/README.md)
completed all 120 executions on EKS and its export passed file verification.
Its report was rendered and inspected. Baseline is optimized commit 2bd4588fba60.

### Decisions made

Recorded L0 median ratios were 2.003, 2.814 and 2.554. The further 10x goal
remains active and unverified. Do not compare absolute durations across the
previous AMD host and this Intel host. Keep all setup and replay costs charged.
The next candidate extends the existing order-witness proof to automatic small
curve construction and removes division/exponentiation from residual arithmetic.
It passes the 24-test suite but is not measured by the completed screen.

### Open threads

Freeze and execute a fresh comparison for that next candidate against the same
optimized baseline. Strict-isolation confirmation, independent review and the
20–40-bit CPU ladder remain outstanding. No CPU job remains running from round3.

### Errors

The downloaded archive retains its `work/results` parent; the initial copy used
`results` and failed without changing evidence. Corrected the local copy path.
The job itself completed successfully with no missing or failed cells.
