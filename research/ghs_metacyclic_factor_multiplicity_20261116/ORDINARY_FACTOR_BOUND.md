# Ordinary elliptic factors must repeat in the six-branch example

16 November 2026. This is a conditional factor-multiplicity theorem
for the exact `F4` six-branch quotient in
[`../ghs_metacyclic_six_branch_20261116/FACTOR_SCREEN.md`](../ghs_metacyclic_six_branch_20261116/FACTOR_SCREEN.md).
It narrows the pending Frobenius factor screen; it does not identify
an elliptic factor or construct a transfer. The argument is on paper,
not Lean-checked. Claim standing: [pcc:unverified].

## Statement

Let `Q/F4` be the genus-20 quotient of

`Y: y^31=H0(x)*H1(x)^5*H2(x)^25`,

where `H0=x^2+x+zeta`, `H1=x^2+zeta*x+1`,
`H2=x^2+zeta^2*x+zeta^2` and `zeta^2+zeta+1=0`.
Write its Weil polynomial as `P_Q(X)=R(X^5)` with `deg R=8`.
If an ordinary elliptic Weil polynomial

`g_t(Z)=Z^2-tZ+1024`, with odd `t`,

divides `R(Z)`, then its multiplicity in `R` is **even**. In
particular, a squarefree `R` would exclude every ordinary elliptic
restriction-of-scalars factor. This is a necessary condition, not an
assertion that `R` is squarefree. [pcc:unverified]

## Endomorphism and multiplicity proof

Over `F1024`, the cyclic deck group `C31` is defined because
`31 | 4^5-1`; together with the explicit order-three lift it forms
`G=C31 semidirect C3`. Let `H=C3`. The rational double-coset
algebra for this metacyclic pair has the nontrivial field component

`M=Q(zeta_31)^(C3)`, with `[M:Q]=10`.

The group-algebra correspondence acts faithfully through `M` on
`J(Q)` over `F1024`: in the tame `G`-representation on `H^1(Y)`,
each nontrivial `C31` character occurs four times for the six
branch points, and the `H`-fixed part is four copies of the
nontrivial double-coset representation. Thus `M` embeds in
`End^0_(F1024)(J(Q))`. This is the same representation mechanism
used in [Carocca--Lange--Rodriguez, Section 2 and equation (3.2)](https://arxiv.org/abs/0906.4185),
without claiming that this genus-20 Jacobian has CM.
[pcc:unverified]

Base change from `F4` to `F1024` raises each Frobenius root to its
fifth power, hence the characteristic polynomial becomes `R(Z)^5`.
If `g_t` has multiplicity `e` in `R`, the corresponding ordinary
elliptic isotypic component is `E^(5e)` over `F1024`. Its rational
endomorphism algebra is `M_(5e)(L)`, where
`L=Q(sqrt(t^2-4096))` is imaginary quadratic. The isotypic
component is stable under every endomorphism, so the field `M`
embeds in `M_(5e)(L)`. [pcc:unverified]

If `L` is not a subfield of `M`, then `ML/L` has degree ten.
Any `L`-linear representation of this field has dimension divisible
by ten, so `10 | 5e` and `e` is even. If `e` were odd, `L` would
therefore have to be the unique quadratic subfield of `M`, namely
`Q(sqrt(-31))`. [pcc:unverified]

For an ordinary `E/F1024` with this CM field, the Frobenius
discriminant has the form

`t^2-4096=-31*u^2` for an integer `u`.

The Hasse bound is `|t|<=64`; reducing modulo 31 gives
`t=+/-2,+/-29,+/-33,+/-60,+/-64` as the only possibilities.
Substitution gives a square `u^2` only for `t=+/-60`
(`u^2=16`) or `t=+/-64` (`u=0`). Every one of these traces is
even, whereas ordinary elliptic curves in characteristic two have
odd trace. Therefore an ordinary quadratic `g_t` cannot occur
with odd multiplicity in `R`. [pcc:unverified]

## Prior work, limits and next test

The metacyclic group-algebra field is established in
[Carocca--Lange--Rodriguez](https://arxiv.org/abs/0906.4185).
[Waterhouse's elliptic classification](https://numdam.org/item/ASENS_1969_4_2_4_521_0/)
supports the ordinary trace test; the field-embedding and norm
arithmetic above are deductions for the exact `F4` model. No
published statement of this exact multiplicity test was found in
the targeted search; that does not establish global priority.

The shared KB was checked at revision
`9141e386254340a9bdba3036f2bb706a`, last refreshed
14 November 2026. Sources checked (KB):
`source:d83cb619b3afdb7b9d999946161057c7`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`)
records the abstract route, not this factor-multiplicity result.
The KB predates this calculation. [pcc:unverified]

The decisive next step is still to compute `R`. A squarefree result
would close the ordinary elliptic-source route for this exact
six-branch curve. A repeated admissible quadratic would require an
explicit subgroup map and a measured complete attack. The shared
`docs/session-logs/2026-11-16.md` remains reserved by Conductor
T-6720, so this note records the proof and next test while new
experiments are deferred. [pcc:unverified]
