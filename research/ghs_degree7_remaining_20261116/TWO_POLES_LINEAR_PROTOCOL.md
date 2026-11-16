# Degree-seven two-branch-pole plus linear-square screen

Frozen 16 November 2026 UTC before counted runs. Fix the same field, carrier,
and two trace-`+22` source-orbit representatives as in [PROTOCOL.md](PROTOCOL.md).
This arm covers exactly denominators `B=Q_i Q_j (x-c)²`, where `i<j` are
distinct among the five quadratic branch factors of `f`, `c∈K` is nonzero,
and `K=F243`. These are 10×242 denominator choices. It does not cover the
remaining `B=Q_iQ_jQ_k` class.

For each pole pair, put `S=Q_iQ_j` and `M=f/S`. The three unpoled quadratic
branch factors have `3³=27` Galois-compatible assignments to elliptic
2-torsion roots, and the unpoled linear branch factor has its unique rational
2-torsion image. CRT gives `h mod M`. Every possible degree-seven numerator
is `A=(B h mod M)+λM`, with `λ∈K×`: `M` is monic of degree seven.

The identity `f C²=B(A³+A²B+bB³)` implies `C=(x-c)D` and hence
`M D²=A³+A²B+bB³`. This is an exact equivalence for a polynomial `D`.
The right side divided by `M` has degree fourteen and leading coefficient
`λ³`, so `λ` must be a nonzero square in `K`. Exactly 121 such values remain,
giving `10·27·242·121=7,906,140` logical candidates per source model.

For each candidate, first evaluate the necessary square condition at ten
specified nonzero `K` elements (`1,2,a,a+1,a+2,a²,a²+1,a²+2,a³,a⁴`).
At each point `z`, test whether
`A(z)³+A(z)²B(z)+bB(z)³ ∈ M(z)·(K²)`; `M(z)≠0` because its only
K-rational root is zero. The point filters are only necessary conditions.
Every survivor receives exact polynomial division and the full square test,
followed by coprimality, equation identity, and morphism-degree checks.
Counter and source-hash receipts remain separate by target mask; any failed
run or assertion is censored. A positive hit requires independent replay,
subgroup trace, full GHS solve, matched rho, and publication-priority audit.

Sources checked: the same repository protocols and Howe primary source
listed in [PROTOCOL.md](PROTOCOL.md). The shared KB search scope and
freshness gap are also recorded there. This is a finite exact-model screen,
not a claim about every genus-five carrier or correspondence.
