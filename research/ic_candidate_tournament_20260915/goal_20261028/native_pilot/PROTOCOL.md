# Native IC candidate tournament: first bounded pilot

Status: registered protocol; measurement pending. This protocol is frozen
before dispatch. It depends on the schema-1 native tournament runner in
PR #2753, at source commit `060cbccff0b326afde03fa7dadd252a25b361da3`.
The new round identifier is `round-0026`.
The measured workflow checks out the exact dispatch ref and the runner freezes
its own executable, worker source, compiler, candidates, public fixtures,
configuration, host and profiler into the round seal.

## Question and panel

On the same supplied public ECDLP points, does any of the six default
configuration changes reduce complete cold IC cost against the unchanged
`incumbent`? The runner's source-bound default registry is frozen as the
round's `candidates.json` by `prepare`. It contains `batch16`, `batch32`,
`batch128`, `dense`, `excess8` and `window8`. These vary the collection/rank
schedule, relation linear algebra or collection window. The matched rho arm
is measured on every point as a reference; this panel does not qualify that
rho implementation as globally strongest.

Use `--profile pilot`, seed `2026102801`, development cells `13a0,17a1,19a0,23a0`,
and holdout cell `19a1`, with one public target per complete cold job, three
repetitions, and the runner's fixed-support policy. Development has three
cases per cell; confirmation and replay use twelve cases per cell, including
the holdout. The A/A, smoke and selection stages retain the native runner's
own frozen allocation. No old seed, target or closed registration is reused.

## Cost and decision

The primary unit is Valgrind 3.22.0 amd64 user-space instruction reads (`Ir`)
from process start through setup, factor base, relation attempts, PDP,
verification/filtering, scalar-field linear algebra, descent, scalar recovery
and reporting. Complete cold process wall time is secondary. The runner also
reports `S = total Ir / sqrt(r)` and ratios to the matched rho and the
rank/collection floor. For `K` independent factor-base columns, the collector
needs at least `K` relation-producing trials, so the deliberately weak floor
is `Ir_total >= K`, or `S_floor = K / sqrt(r)`. `Ir` is not converted to curve
additions. Counted
`2^x` group-addition equivalents remain unset without a measured conversion.

The falsification target is the runner's incumbent gate: a candidate/incumbent
cost ratio at most 0.80 in both Ir and native cold time, upper paired 95%
limits below 1.0, no cell above 1.10, verified completion and the same
factor-base support. The same gate applies at confirmation and replay.
The candidate must also pass the independent point, relation, rank, descent
and scalar checks. The rho comparison is reported separately; an IC/rho ratio
below 1.0 is not a speed claim if the matched rho quality or operation-unit
bridge is missing. If no candidate passes, retain the incumbent and every
failed/regressing arm. A failed A/A, timeout, OOM, invalid certificate or
missing stage is recorded as such and stops promotion.

The exposure cap is 1,800 profiled jobs with a 60 s timeout and 8 GiB address
space per child on Ubuntu 24.04 amd64 with Rust 1.94.1 and Valgrind 3.22.0.
The workflow's total cap is 180 minutes. The same seed is never redispatched
after a measured failure. Preserve the complete round, including raw profiles,
native reports, source snapshot, receipts, summaries and decision. Independent
native `verify` must pass before reporting a comparison. The resulting
archive, source hash, exact host and workflow run identity belong in the
evidence PR, with the scoreboard, leaderboard and browser updated in the same
PR for any source-backed comparison.

This is a small-curve implementation tournament. It does not test the m=83
confidence gate or provide a scaling result. The current runner's schema-2
scientific admission and broader candidate families remain outside this panel.
