# Matched encoding and tau-presentation comparisons

Authorized2026-11-15: execute steps2/3 following the original frozen panel.
Two separately frozen descriptive finite-panel comparisons share one native
binary, the same33 serialized original instances and complete known answers.
No primary rows have been collected for these two comparisons at filing.

## Step2: encodings

Five arms: existing certified finite-domain S3; the same with constant
specialization; implicit algebraic subspace membership; complete finite-base
full-point chain; a repeated constant-specialized A/A arm. All original bases
and targets are identical. The implicit arm receives only the d-dimensional
basis, never the point table; the negative signed-base control uses the
one-dimensional span of its actual x, whose nonzero lifts are exactly +/-G.
Every returned point is checked against the independently retained actual
finite base before acceptance. Full-point chains retain all5 finite-target
or2 identity-target branch paths; S3 retains its two intermediate masks.
Both are complete at m3 and share the same total query budget.

33 cases ×5 arms ×2 repetitions =330 primary rows. Rotate arm order by
case index + repetition. Seeds, exact points and known answers are the
unchanged original workload. No favorable subset or adaptive seed selection.

## Step3: tau presentation

Four arms: direct constant-specialized S3, delta=tau-1 applied to every base
point and target, delta-inverse applied to every base point and target, and
direct A/A. These are bijective presentations of the same relation problem.
All base signs and indices are preserved. Delta does not enlarge the base.
Its image is supplied as the actual finite point domain, not assumed to be a
linear x-subspace. This comparison tests transported finite-domain equations;
a free-witness lifted delta map or geometric defining ideal is not supplied.

33 ×4 ×2 =264 primary rows. Same rotation rule. Every successful mapped sum
is independently replayed, transported back and checked against the original
base/target. Known-answer truth is original independent pair-table truth;
none of it is supplied to candidate solvers.

## Resources, guards and counts

Each comparison uses one CPU,2GiB address space,600 seconds total including
its engineering pilot, final freeze and primary run; no retries or paid
resources. Each query has2000 cumulative SAT conflicts and750ms safety fuse,
with500000 variables,1million domain clauses,2million polynomial terms and
1024 models. Preprocessing reduces the available fuse; admission inside the
production oracle has its own existing safety/deadline semantics. The600second
outer process timeout bounds the complete execution. These safety clocks are
not timing measurements.

Each panel first makes one call per arm at n5/7/9/83/131, targeting3*base[0].
Every pilot row must be independently valid and all n5 arms must return Found
with mapped and original replay. Otherwise stop before primary rows and file
a pre-data correction; preserve the failed pilot. Wide UNKNOWN is retained,
not treated as a detected positive or a negative result. Runtime is recorded
only for engineering feasibility and final metadata, before primary freeze.

Primary metric: valid decided queries. Secondary: SAT conflicts, peak variables,
and native counted group additions+doublings including transport, admission,
solving, lifting and final replay. Stage snapshots retain preprocessing and
through-solver counts. Field/ANF construction, field inversions outside group
operations, SAT propagation and Frobenius coordinate squaring are explicitly
uncharged by that group-operation unit. No cost metric claims a complete
ECDLP operation ledger or runtime improvement. Setup is repeated per row,
not silently amortized over a batch of targets.

Any invalid answer stops the panel and preserves the prefix. UNKNOWN remains
in the denominator. All expected rows not attempted before an interruption
are explicitly accounted for, with no retries. Deterministic Rust analysis
is hash-bound before primary data. Tabulate exact per-arm/per-field counts,
A/A disagreements and all caps; repeated targets are not independent samples.
No population test, p-value, isolated timing, scaling or challenge recovery.
Labels remain unmasked for this predeclared deterministic descriptive analysis.

## Sources and custody

Reuse production arithmetic/SAT, exact inverse/half-trace validation and
guarded addition; prior reports cite Kosters–Yeo1503.08001 and US7257709.
Tau norm identity and inverse action were checked on complete small groups
and public n131 G/Q in the previous study. This comparison supplies no new
mathematical novelty claim. KB tools absent; wrapper exits127; existing
primary-source copies and scoped reports are the fallback.

Canonical protocol.json files in encoding/ and tau/ bind instrument commit,
binary hash, inputs, budgets, analysis and all comparison rules before data.
The user's explicit local EKS/S3 exception applies; no credential or gate
configuration is changed. The original264-row executable remains unchanged.
