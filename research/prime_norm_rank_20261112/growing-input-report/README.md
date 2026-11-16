# Actual growing norm input budgets

For every arity `a=h+2 >=2`, the actual computed membership remainder
`R=P modMonic U` satisfies

\[
\operatorname{weightedTotalDegree}(R_j)\le 2\deg(P)-2j,
\]

using natural subtraction at zero. All monomials are taken from actual
multivariate coefficient support. Central claims PCC-20261114-253550 and
PCC-20261114-2c4a15 are unsigned [pcc:unverified].

The actual coefficient variables have weights
`w(A_j)=2(a-j)` and `w(B_j)=2a-3-2j`; an explicit bijection identifies them
with `1,...,2a-2` (PCC-20261114-cad062). The norm and computed support are
monic of degrees `2a` and `2a-1`. The target-curve premise binds the computed
support to exact norm multiplication (PCC-20261114-6c253b) [pcc:unverified].

The generic division theorem follows the literal Mathlib well-founded
algorithm: every leading-term subtraction preserves the coefficient budget,
and the computed quotient/remainder meet the stated budgets
(PCC-20261114-3a3e7b). The public polynomial P is arbitrary; no successful
support, quotient or normal form is supplied [pcc:unverified].

At h=1 the exact coefficient-ring renaming preserves the existing actual
norm, support and full remainder; the four weights are `(4,2,3,1)`.
PCC-20261114-3a3330/69b0f2. This is a polynomial-ring identity, not a
comparison only at finite-field points [pcc:unverified].

[Generic source](../lean/NormInputBudget.lean),
[actual source](../lean/NormGrowingInput.lean),
[prospective contract](../GROWING_INPUT_GRADING_PROTOCOL.md),
[declaration hashes](../growing-input-evidence/claim-pins.json),
[validation](../growing-input-evidence/validation-001.json),
[editable graph](input.dot), [SVG](input.svg),
[PDF](report.pdf), [editable report](report.tex).

Source freeze `0a12a195c66785b5b8b4b4409ab0cc02ee991f64`.
Forty authored statements plus generated `finSumFinEquiv.eq_1`;
whole-study gate435/0 extra trusted axioms and import-inclusive fresh
NormGrowingInput exit0. Gate data PCC-20261114-dc7d16, final validation
PCC-20261114-41a33c. All42 new claims are unsigned/default not_covered;
no signed execution verification is asserted [pcc:unverified].

Budget drafts001/003 and growing001-004 fail; their exact sources/logs
remain. Successful intermediate drafts and growing005's unused-simp
warning also remain. Final006 exits0 without warnings. Report001 has a
26.38254pt source-citation overflow; report002 fixes it, renders two pages
without Overfull/Underfull messages, and both final pages were inspected
[pcc:unverified].

Sources checked(KB): unavailable (no loaded tool or wrapper executable).
Primary fallback read actual pinned Mathlib support/weight lemmas and
Div.lean recursion, plus existing NormMembershipElimination,
NormPoleTransport and NormTargetFiber. The earlier weighted-GB review and
P1514/P1536 audits distinguish input bounds from regularity and available
constructors. No novelty claim for generic weighted division
[pcc:unverified].

A constructive small full-closure cutoff, public finder, base coverage,
signed output, field extraction, retries and complete expected bit cost
remain separate. No degree29 normalized-row claim or original W-jet map is
established. The frozen native fixture is still unlaunched; no solver,
paid/cloud/attack/F4/msolve or Python cryptomath campaign ran
[pcc:unverified].

Canonical performance/leaderboard/benchmark/progress graphs were checked
for scope and left unchanged: no measured performance or ecdb-imported
field is added [pcc:unverified].
