# Degree-seven three quadratic branch poles

Frozen 16 November 2026 UTC before counted runs. Fix the same `K=F243`,
trace-`+22` genus-five carrier, and source representatives as in
[PROTOCOL.md](PROTOCOL.md). This arm covers the last previously unsearched
normalized degree-seven denominator class `B=Q_iQ_jQ_k`, with three distinct
quadratic branch factors of `f` and no square factor. There are ten choices.

Put `M=f/B` (degree five). Each of its two unpoled quadratic branch factors
has three elliptic 2-torsion assignments, so there are nine CRT residues
`h mod M` per denominator. Every possible degree-seven numerator is
`A=(Bh mod M)+M(λx²+μx+ν)`, `λ∈K×`, `μ,ν∈K`. The cover equation becomes
`M C²=A³+A²B+bB³`. The quotient on the right has degree sixteen and
leading coefficient `λ³`; hence `λ` must be a nonzero square. Exactly 121
values remain, giving `10·9·121·243²=643,043,610` logical candidates
per source model.

For each of sixteen fixed nonzero field points `z`, a necessary condition is
`A(z)³+A(z)²B(z)+bB(z)³ ∈ M(z)·K²`. The source builds a 243-bit set of
allowed `ν` for each affine shift `δ=λz²+μz` and intersects these sets for
every `λ,μ`. This bitset operation represents all 243 `ν` values exactly;
it introduces no probability or heuristic exclusion. Every surviving triple
receives the exact polynomial quotient, squarehood, coprimality, and full
equation identity test. Any assertion failure is a censored run. Source SHA256,
coverage counters, survivor counts, hits, and CPU/wall seconds are retained
separately for masks 68 and 84.

The 16 points are `1,2,a,a+1,a+2,2a,2a+1,2a+2,a²,a²+1,a²+2,2a²,2a²+1,a³,a³+1,a⁴`.
They are distinct and nonzero; `M` has no K-rational root among them.

A positive hit requires independent algebraic replay, subgroup trace,
complete GHS solve, matched rho, and publication-priority check. A negative
result applies only to the fixed carrier and the exact source models (plus
their Frobenius conjugates), not other carriers or Jacobian correspondences.
The local prior-work and Howe sources are recorded in [PROTOCOL.md](PROTOCOL.md).
