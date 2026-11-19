# H4 cost interpretation amendment — 2026-11-19

## Corrected result

The recorded nine-field first-fall census remains valid as a degree
diagnostic: the measured value is seven for the 62 reported systems. The
solving-degree result remains one completed point at `n=11` and censored
profiles at `n≥13`. The frozen H4 decision rule therefore remains
**inconclusive**.

The `2^87.4` number is a **conditional dense Macaulay sizing model**, not a
lower bound on the bit cost of a decomposition or on every algorithm. For
`N=132` Boolean variables, the vector space of squarefree monomials of
degree at most seven has

`M_7 = sum_{j=0}^7 binom(132,j)` columns, with `log2(M_7)≈36.9`.

Charging a dense square linear-algebra operation at exponent `ω=2.37`
gives `ω log2(M_7)≈87.4` bits. This multiplication model does not establish
that a solver must form a full matrix, that it has square dimensions or full
rank, or that sparse elimination, structural reductions, guessing, or a
different decomposition cannot use less work. In particular, subtracting
`21` or `65` from this model yields **scenario gaps** of `66.4` and `22.4`
bits, not proven gaps to the relation gate or rho reference. The earlier
phrases “lower bound on any solving cost,” “Win excluded by a cost bound,”
and “cost fails by at least 66 bits” are superseded by this amendment.

What the measured deep profile supports is narrower. For the recorded
`n=11`, 12-variable systems, the Macaulay quotient dimension reached the
known solution count only at degree 12. For the recorded `n=13` planted
profile, degree 9 completed with quotient dimension 20,336 versus six
solutions; degree 10 was censored by a 9,000-row cap. Conditional on the
recorded projected-rank method, that profile gives `d_solv≥10` for this
specific system. It does not determine the n=13 solving degree or predict
the n=131 solver cost.

## Evidence custody

The original protocol, runner, `out/result.json`, and initial results text
are retained. The first-fall counts, row-cap statuses, controls, host
deviation, and unmeasured FGHR arm are unchanged. This amendment revises
only the interpretation of the extrapolated cost. New work should measure
solving degree or a complete solver cost on frozen planted and ordinary
queries, with exact correctness checks and explicit timeout and memory
statuses. A component cost must be charged into its full relation and
single-target accounting before an end-to-end comparison.
