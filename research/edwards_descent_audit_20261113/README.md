# A nonregular Edwards descent with nonzero equations

Lean checks a concrete twisted-Edwards instance of the point decomposition
problem over F25 whose two symmetrized descent equations are nonzero
quadratics and whose affine coordinate algebra has dimension two over F5,
yet their highest homogeneous parts are not a regular sequence. The whole
project gate passes 55 declarations, zero failures and zero trusted-native exceptions.
Signed standing remains unverified. [pcc:unverified]

This strengthens the prior short-Weierstrass `[f,0]` witness. The new input
uses the representation in FGHR2014, avoids a zero descended equation and
has an actual Artinian quotient. It disproves the literal universal n = 2
reading of Hypothesis 3. It does not disprove a generic-input or n ≥ 3
interpretation, the conditional Theorem 4.1, or Joux–Vitse Assumption 2.
[pcc:unverified]

## Exact input and actual points

Let K=F5[beta], beta²=2. Lean obtains its field structure from the checked
irreducibility condition and proves cardinality 25. Set a=beta, d=4 and
R=(1,1+beta) on the twisted-Edwards curve. [pcc:unverified]

\[
  ax^2+y^2=1+dx^2y^2.
\]

The required parameters a,d are nonzero and different; both a and the target
y-coordinate lie outside F5. The target is smooth. The two actual
factor-base points have Edwards coordinates
P=(-(2+3beta),2) and Q=(2+3beta,3). Their Edwards addition has nonzero
denominators and gives R. [pcc:unverified]

The standard explicit change of coordinates sends them to actual nonsingular
Mathlib points on

\[
  Y^2=X^3+(2+3\beta)X^2+(3+2\beta)X.
\]

Lean directly checks P+Q=R in that actual point group and R≠O. This is a
concrete input validation; a general Edwards/Weierstrass group-equivalence
theorem is not asserted. [pcc:unverified]

## The literal source polynomial and its descent

The printed polynomial on FGHR p604 uses a/d in its first and final terms
and 2(d−a)/d in the mixed term. The exact definition is `sourceF3` in
[EdwardsInput.lean](lean/EdwardsInput.lean). A universal algebraic identity
proves the change to e1=y1+y2, e2=y1*y2; actual coefficient extraction in
the basis (1,beta) then gives [pcc:unverified]

\[
  f_0=2(e_1^2+e_2^2)+2e_2+1,\qquad
  f_1=2(e_1^2+e_2^2)+3.
\]

Both are nonzero, with total degree 2. Their highest homogeneous parts
are the same nonzero polynomial 2(e1²+e2²). Lean directly proves failure
of Mathlib `RingTheory.Sequence.IsRegular` for those actual top parts.
This is FGHR Definition 5, rather than nonregularity of the affine sequence.
[pcc:unverified]

Exact ideal membership certificates give

\[
  (f_0,f_1)=(e_1^2,e_2-1),\qquad
  F_5[e_1,e_2]/(f_0,f_1)\simeq F_5[T]/(T^2).
\]

The algebra equivalence precedes the dimension-two and finite/Artinian
proofs. The rational zero set is exactly {(0,1)}; support size does not
replace multiplicity. In particular the generic degree 4 prediction is
not a universal all-input equality. A special finite instance does not
refute generic attainment. [pcc:unverified]

## Scope, sources and reproduction

The main closed binding is
`EdwardsDescentAudit.actual_edwards_descent_counterexample` in
[DescentCertificate.lean](lean/DescentCertificate.lean). It combines actual
input validity, source-polynomial symmetrization, actual coefficient descent,
both nonzero degree-two equations, finite-dimensional/Artinian quotient,
and the direct nonregularity statement. [pcc:unverified]

```sh
cd research/edwards_descent_audit_20261113/lean
lake build
cd ../../..
tools/lean/proof-gate.sh --out /tmp/edwards-descent-gate \
  research/edwards_descent_audit_20261113/lean
```

Lean 4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
Only propext, Classical.choice and Quot.sound occur. No holes, custom
axioms, native_decide, waived checks or raised elaboration limits are used.
Failed sources/build logs/exits and the whole gate are preserved losslessly.
No native solver run is claimed by this formal increment. [pcc:unverified]

Sources checked: FGHR2014 §§1.1–1.2/2.3.1/3/4.1.1, Hypothesis 3/Theorem 4.1
and §5 experimental scope; printed603–604 also visually inspected. The PDP
definition allows a given target point; the relation-sampling algorithm
uses a prime-order generator, whose distribution is not modeled here.
Joux–Vitse author manuscript pp4–5/7/9–10 was also read: its Assumption 2
concerns homogenized Gröbner computation for n equations in n−1 variables,
not this two-equation Edwards regularity claim. Exact source hashes/read
scopes are retained in `evidence/source-review.json`. KB unavailable
(MCP absent; wrapper127); bounded primary/repository/Mathlib fallback.
No novelty claim. [pcc:unverified]

The original all 14 requirement map and qualified-system/native/independent-host
obligations remain in
[the central tracker](../../problems/fourteen-target-audit-proof-and-execution-gaps.md).
PCC and exact source-pin receipts accompany this study; signed standing is
separate from local kernel replay. Paid compute and agent storage uploads: zero.
[pcc:unverified]

Exact local replay for **PCC-20261113-bc105f** passes at
2026-11-13T01:29:28Z. Its seven source/config pins match;
the complete statement hash is `ce76b77d337dcdced1883e0c16de67e4f85bc8672cca81392596b26b25464154`.
Signed standing remains unverified. See [verification.json](verification.json)
and [sources.json](sources.json) for the actual verdict, source plan and
lossless gate/attempt inventories. [pcc:unverified]
