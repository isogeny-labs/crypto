# Degree-seven cubic-square cover screen

Frozen 16 November 2026 UTC, before counted runs. This screen fixes
`K=F_3[a]/(a^5+2a+1)`, the trace-`+22` carrier
`Y:y²=f(x)=x^11+x^9+x^8+2x^7+2x^6+2x^4+x^3+x²+2x`, and the
two source-orbit representatives `E_b:v²=u³+u²+b`, with `b` masks 68 and 84.
The exact target group has 222 points. The source and carrier are the same
as in `../ghs_trace22_cover_screen_20261116/`.

The normalized degree-seven map has `u=A/B`, `deg(A)=7`, `deg(B)≤6` even.
This screen covers exactly `B=H²` for every monic cubic `H∈K[x]` coprime to
`f`. It includes reducible and repeated cubic `H`. Existing sparse, linear,
and quadratic-square screens do not cover all such denominators. It does not
cover `B=Q_i Q_j H_1²` or `B=Q_i Q_j Q_k`.

Write `f=L Q_1…Q_5` over `K`, with `L=x`. Every branch point is unpoled.
For each of `3^5=243` Galois-compatible assignments of the five quadratic
branch factors to the three roots of `u³+u²+b`, CRT gives a residue `h mod f`.
The unique possible numerator of degree at most seven is
`A=(H² h) mod f`. With `H=x³+s x²+t x+w`, the three coefficients
`[x^8]A,[x^9]A,[x^10]A` are quadratic polynomials in `s,t,w`.
Solve their common zero set over all `K³` via a finite-field ideal and
check each solution. A solver failure or positive-dimensional ideal is a
censored cell, never a zero-hit result. The exact final filter is
`f C²=B(A³+A²B+bB³)` with `gcd(A,B)=1`, `deg A=7`, and `C∈K[x]`.

The run output retains source SHA256, every system's status, counts, hits,
and wall/CPU timing in a separate receipt per target. Any hit requires
independent algebraic replay, morphism degree, explicit subgroup image, full
GHS solve, matched rho, and a fresh publication-priority search.

Sources checked: the repository's three degree-seven protocols and trace-`+22`
results; [Howe, Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221)
for the existence ceiling (not an executable map); the 2015 Iijima–Momose–Chao
classification audit in `../ghs_odd5_carrier_20261115/PRIOR_WORK_AUDIT_20261115.md`.
The shared KB was ready at revision `9141e386254340a9bdba3036f2bb706a`,
refreshed 14 November 2026, before this newer local screen; searches for
`degree seven cubic square denominator hyperelliptic elliptic cover`,
`trace 22 genus five 3 5 GHS`, and `Howe theorem 4.17 elliptic degree bound
hermitian` found no exact screen, which does not prove publication priority.
