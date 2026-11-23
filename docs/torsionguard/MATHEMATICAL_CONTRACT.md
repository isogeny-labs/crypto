# Mathematical contract

Let

\[
H = \bigoplus_{j=1}^{d} \mathbf Z/(\ell_j^{a_j})
\]

be the finite abelian module declared by the input. Each relation matrix
defines an endomorphism of `H`; TorsionGuard first checks the quotient
well-definedness condition

\[
\ell_i^{a_i} \mid A_{ij}\ell_j^{a_j}
\]

for every matrix entry.

For each distinct prime `ell`, collect the components with base `ell` and
restrict every relation to the order-`ell` subgroup `H[ell]`. In the basis
whose `j`th vector is `ell^(a_j-1)`, the restricted coefficient from component
`j` to component `i` is

\[
\begin{cases}
A_{ij}\ell^{a_j-a_i} \bmod \ell, & a_j \ge a_i,\\
(A_{ij}/\ell^{a_i-a_j}) \bmod \ell, & a_j < a_i.
\end{cases}
\]

The division in the second line is exact because quotient-map validation has
already succeeded. The restricted matrices for all relations are stacked.

The common kernel on `H` is trivial exactly when every stacked prime-local
matrix has full column rank. One direction is immediate. For the other, every
nonzero finite `ell`-group contains an element of order `ell`; therefore a
nontrivial common kernel intersects `H[ell]` nontrivially for some `ell`.

When row reduction finds a null vector `v` over `F_ell`, TorsionGuard lifts its
`j`th local coordinate to `v_j ell^(a_j-1)` in the original component. It then
re-evaluates every unreduced, full-module relation on that lift. Analysis stops
with an internal error unless the lift is nonzero and every full evaluation is
zero.

This is a standard finite-group reduction, not a new mathematical result. The
implementation contribution is a strict machine-readable contract, replayable
rank and witness certificates, evidence gates, and an independent exhaustive
test oracle. The reduction says nothing about whether an input faithfully
models a curve, circuit, or implementation; that boundary is represented by
the six obligations in the certificate.
