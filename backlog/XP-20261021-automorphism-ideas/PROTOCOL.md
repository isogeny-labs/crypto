# DRAFT protocol: automorphism idea follow-ups

Protocol state: **not frozen**. Card:
`XP-20261021-automorphism-ideas`. Date: 2026-10-21.
This plan supplies no runnable experiment or performance result.

## S0: activation gates

Select exact curve/subgroup registry IDs, generator, known-answer target
construction, full point-set digests, implementation commit, seed stream and
independent checker. Use public synthetic fixtures. Reuse native Rust/C
arithmetic; no Python cryptanalytic harness. Define resume/checkpoint behavior
and a result schema preserving all failures. Freeze resources after design,
not from the illustrative card envelope.

For Koblitz conclusions preserve the required m83 fidelity gate. Toy m31
or N13 checks test correctness only. Do not silently replace a full-width
request with a toy control. P-192 and P-224 are parameter-audit targets, not
a proposed brute-force DLP workload.

## S1: DGM-01 controlled ablation

Arms:
A. Unfolded equations over the same full admissible point set (correctness control).
B. Current folded baseline, with all existing symmetry retained.
C. Proposed incremental folding/transport change, if one is identified.

Freeze the target stream and relation witnesses for equation/rank replay.
Retain point-set membership and orbit-length evidence. Compare B and C online
on matched independently generated targets only after their algorithms are
fully specified; label fixed-witness replay as stage evidence.

Acceptance: equal accepted decomposition sets for unchanged admissibility;
independent replay of each transported relation; correct target recovery.
Reject inconsistent coefficients, missing orbit members or unverified rank.
Record nonzeros, useful rank, memory, map/canonicalization time, matrix work,
relation search, descent, verification and full online wall interval.
A missing C mechanism is a design blocker, not a successful experiment.

## S2: DGM-02 map search

Bound the map family, coefficient field, degree and search budget before run.
Store exact maps/inverses and exceptional-point handling. Verify curve identity,
Jacobian action and stable subgroup scalar action independently. Record divisor
orbits separately from the image on the target subgroup. Emit required native
certificates before adding a route or effective symmetry to a benchmark.
Bounded failure means only no candidate found within that recorded search.

## S3: DGM-03 walk audit

Read the full modern negation literature, define update alphabet and short-word
bound, then enumerate short products with native exact arithmetic.
Check quotient representative consistency, invariant partition/DP predicate,
both exponent coefficients, deterministic cycle escape and collision validity.
Use baseline, naive multiplicative and audited arms on matched known answers.
Count failed recoveries, fruitless cycles and cross-start useful collisions.
A short-product pass does not prove random-walk mixing.

## S4: DGM-04 parameter exclusions

Read exact standard parameters from the cited sources and encode full fixture
identities. Independently reproduce j, ordinarity, trace and discriminant
square tests with native integers. A certificate must state the ordinary,
same-prime-field hypotheses and special-j scope. Do not claim that this excludes
other endomorphisms or higher-dimensional transfers.

## S5: DGM-05 lifting gate

Read the full Xedni proposal and analysis, plus any specific rank-one/Wieferich
source a variant relies on. Write an explicit input contract and algorithm
explaining the changed dependence/height obstruction. Require a public worked
known-answer example and independent native checker before timing. Cheon-style
auxiliary powers must be declared as additional inputs with a matched comparator.

## Accounting and stopping

Primary: matched single-target ONLINE wall ratio to the current rho baseline.
Supplementary: reusable setup and full cold cost. No batch-average substitution.
Predeclare repetitions, paired seeds, timeout, memory cap and uncertainty method
when freezing; these values are intentionally not invented in this draft.

Stop a correctness arm on an independently reproducible violation, preserve its
receipt, and fix or explicitly reject it before timing. Preserve timeouts/OOMs
and negative results. New map, identity or result claims require native evidence
and the repository's proof/certificate gates. No step is scheduled by this PR.
