# Degree-eight rational and two quadratic branch poles

Frozen 16 November 2026 UTC before counted runs. Fix the two exact carrier
representatives 32280 and 35349 and source masks 68 and 84 from the prior
protocol. This arm covers precisely

`B=L Q_iQ_j` and `B=L³ Q_iQ_j`, with `L=x` and `i<j` among five quadratic
branch factors. It does not cover nonbranch square factors or three
quadratic branch poles.

For each pole pair, put `S=LQ_iQ_j`, `M=f/S`; `deg M=6`. The three unpoled
quadratics have `3³=27` CRT assignments. For each shape and assignment,
every possible degree-eight numerator is
`A=(B h mod M)+M(λx²+μx+ν)` with `λ∈K×` and `μ,ν∈K`.
The full equation `f C²=B(A³+A²B+bB³)` reduces to
`M D²=A³+A²B+bB³`, with `C=D` for `B=S` and `C=L D` for `B=L²S`.
The quotient has degree eighteen and leading coefficient `λ³`, so `λ`
must be a nonzero square. There are 121 such values and
`2·10·27·121·243²=3,858,261,660` logical candidates per
carrier/source cell.

The source intersects exact 243-bit allowed-`ν` sets at twenty explicit
field points for each `(λ,μ)` pair, with direct-evaluation controls at every
branch assignment. Every survivor receives the complete polynomial quotient,
squarehood, coprimality and cover identity check. The bitsets are an exact
reorganization of all 243 `ν` values, not a sample. Any assertion failure
or interrupted run is censored. A positive map needs independent algebraic
replay, evaluated subgroup image, full GHS solve, matched rho, and prior-work
check before a vulnerability claim.

This is a new bounded search arm, not a claim that degree eight is covered.
The earlier primary-source and KB review is in
[DEGREE8_SPARSE_PROTOCOL.md](DEGREE8_SPARSE_PROTOCOL.md).
