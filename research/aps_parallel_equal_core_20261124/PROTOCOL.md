# APS versus rho with the same target and core budget

Status: **design draft, not frozen or admitted for a performance run**. This
file fixes the comparison question and the evidence needed to answer it. It
contains no parallel timing result or expenditure authorization.

## Source and prior evidence

The published APS one-relation construction samples labelled public points
and searches for a signed relation (Amadori, Pintore and Sala, *Finite Fields
and Their Applications* 2018, DOI 10.1016/j.ffa.2018.01.009; author
[preprint](https://eprint.iacr.org/2017/609.pdf)). The 31-bit repository
implementation is the specialized `m=3` S3 pair-search example
[`native/examples/prime31_aps_one_relation.rs`](../prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs).
Its own source describes that path as generic pair meet-in-the-middle
equivalent. The bounded queue merged in PR #5092 is in
`src/cryptanalysis/aps_single_relation.rs` and caps the field at 65,521;
its worker result does not measure the 31-bit implementation. The separate
small-field `frontier_parts` planner is not a partition of the native S3
pair loop.

The archived [48-target receipts](../prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json)
and [independent arithmetic audit](../prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_INDEPENDENT_ARITHMETIC_AUDIT_20261116.md)
form a **development baseline** on one unisolated host, not the proposed
parallel experiment. All 48 APS calls succeeded on their first attempt.
Their total cold wall was 109,734,280 ns versus 99,473,873 ns for the
unfolded 64-lane rho arm. Summing attempt rows gives 33,855,309 ns of source
setup and 63,267,152 ns of S3 query time. The latter is 57.65% of the cold
total. A perfect N-way split of only that phase, with zero overhead, gives
an aggregate APS floor of 78.10, 62.28 and 54.38 ms at N=2,4,8. These are
derived bounds, not predictions; the rho arm must receive the same cores.
The existing rho `lanes` field counts batched walks, not CPU cores.
The S3 fraction varies by target: the median case fraction is 48.75%.
Under the same idealized S3-only split, APS would win 32, 40 and 43 of
the 48 pairs at N=2,4,8 **against the archived serial rho**, versus 23
actual serial APS wins. Those hypothetical pair counts are descriptive
ceilings and do not answer the equal-core question, because rho would also
receive N cores.

`python3 research/aps_parallel_equal_core_20261124/analyze_baseline.py`
recomputes these totals from the committed JSON, checks every arm's case
sum against its published summary, and emits the raw-file SHA-256 plus the
idealized bounds. In a sparse checkout, pipe `git show
origin/main:research/prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json`
to the script with `-` as its input argument. This script reads receipts;
it is neither a solver nor a replay of point arithmetic.

Sources checked (KB, revision `750797a540fb4c38837c67b12df84d5b`,
refreshed 2026-11-25T03:19:48Z):
`source:91b6bbe9fb73267bbeb5d7c97f493b26` (`crypto:docs/ecbench/README.md`)
records APS accounting gaps and the matched rho family;
`source:b9dda86f2baa2f4b18955b8e1ee126f1`
(`crypto:papers/ecdlp-index-calculus-program/LITERATURE.md`) locates the
APS and prior group-sum literature. The committed native receipt and source
above control the numeric and implementation statements. KB search scores
were not treated as evidence of novelty or performance.

## Question and arms

For a *single* public target Q on the same curve, does an N-core APS run
finish at lower complete cold cost than the strongest reproducible N-core
rho run? The primary core count is N=4; N=1,2,8 are scaling diagnostics.
The host must expose at least eight physical cores for the N=8 diagnostic.
If the final host or target population changes, amend this draft before
observing either arm.

The primary APS candidate is a parallel version of the **same 31-bit
one-relation method and source law** as the serial control. Record whether
parallel work is independent attempts, disjoint work within an attempt, or
both. Neither a different factor-base size nor a different point-label law
may be hidden inside the thread-count contrast. Include serial APS as an
implementation parity control. The rho side includes the strongest
source-reproducible optimized negation-map implementation available for
this exact curve and an unfolded diagnostic arm. Freeze the rho version,
jump policy, restart policy and tuning search before touching held-out Q.
Every arm gets the same physical-core allocation and memory ceiling. A
single-threaded rho baseline is not the N-core denominator.

The archived 48 Q are development and replay controls only. Before the
performance run, register a separate held-out public-target panel with
exact curve, subgroup, generator and target encodings, seed law and a
caller-only answer file. Use the identical ordered Q in every arm. Preserve
all target selection failures. An independent checker must check the curve,
subgroup, public Q and each recovered scalar; the answer is never passed to
a solver. A second registered curve is a separate stratum, not a pooled
replacement for a failed first stratum.

## Execution and accounting

One cold call begins before public-target admission, table/jump creation,
labelled-point generation and worker startup, and ends after all workers
have stopped and the candidate scalar has been checked. The clock includes
queueing, coordinator work, cancellation, joining, failed attempts and
speculative work after another worker found a candidate. No cross-target
warm table enters the primary comparison. Any amortized reuse is reported
separately with its exact reuse count and full one-time setup cost.

Each call records wall time, summed process CPU time (including all worker
threads), peak resident memory, allocated cores, worker count, queue depth,
and per-worker start/end/cancellation state. Count generated, admitted,
queued, started, completed, cancelled and never-started work units, with
their seed or deterministic work identifier. Record source group operations,
S3 pair work, roots, candidates, final checks, and rho starts, steps,
collisions, restarts and table work. Missing native work counters are null
with a reason, not zero. Retain every timeout, crash and censored call.
All clocks and counters must specify whether they cover the entire process
or only a worker; nested phase values are not added as if exclusive.

Run A/A controls for deterministic outputs and measurement drift, positive
controls that recover a checked public fixture, and negative controls for
uninformative relations and incomplete or cancelled work. Recheck that the
parallel APS work covers the same declared search region as its serial
control when both finish; compare coordinate/candidate sets or a complete
independent oracle on bounded validation cases. A cancelled traversal does
not certify complete coverage. Preserve the source, compiler, flags,
binary hashes, host placement, isolation evidence, seeds and raw records.

After an admitted isolated-host pilot, freeze the held-out panel, finite
per-call budgets, sample size, chronological arm rotation, analysis script
digest, and stop rule before the main run. Keep pilot and held-out targets
disjoint. Do not replace a failed or slow held-out call with a new target.
The repository's EKS Spot, checkpoint, source custody and validation gates
apply before launch or publication of a speed ranking.

## Analysis and decision

Pair by exact curve and Q. Report the complete flow population and success
fraction by arm and core count. For each admitted paired stratum, report the
ratio of summed cold wall, ratio of summed CPU, peak-memory distribution and
paired target wins. The primary comparison is four-core APS against the
strongest four-core rho on the held-out panel. A crossover claim requires
both methods to complete the declared panel, valid controls and independent
replay, plus a prospectively fixed paired uncertainty interval for the wall
ratio whose upper bound is below 1. CPU and memory cannot be omitted or
called free; a wall win with higher total CPU is explicitly a resource
tradeoff. The N=1,2,8 diagnostics do not replace the primary decision.

If coverage parity fails, source identity changes between arms, any required
work is unaccounted, or a success-only subset is needed for a favorable
ratio, the comparison is inadmissible. If the admitted APS wall ratio is at
least 1, report a loss. If its interval includes 1, report an unresolved
ordering. No exponent or general prime-field claim follows from this panel.
