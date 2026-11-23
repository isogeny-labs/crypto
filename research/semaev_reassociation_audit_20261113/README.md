# Actual S3 recursion: two-step polynomial reassociation

The Lean modules prove equality of the actual two-step canonical resultant
polynomial and the source-ordered balanced expression with the existing
S4 polynomial. The actual-curve S4 cross-pair bridge and independent-b
negative control now build. Final whole gate60/0failures/0trusted passes;
initial51/0/0 is retained. Exact local replay of **PCC-20261113-cc3069** passes at2026-11-13T17:44:08Z. Signed standing remains unverified.
See [verification receipt](verification.json). [pcc:unverified]

Let P(U) have degree at most D. All sizes below are declared Sylvester
sizes, retained through every specialization. With the exact general
Weierstrass b-invariant S3 from the existing repository definitions,
set A(T)=Res_U(P(U),S3(z,T,U); D,2). Lean proves the following ordered
identity over every commutative integral domain and arbitrary b2,b4,b6,b8.
[pcc:unverified]

```text
Res_T(A(T), S3(x,y,T); 2D,2)
  = Res_U(P(U), S4(x,y,z,U); D,4).
```

This includes P=0, repeated coordinates and all characteristics. The
independent variables and coefficient embeddings are actual polynomial
rings and ring homomorphisms. `doubleStep_polynomial_reassociation`
proves equality of the formal polynomials, rather than agreement on
points or equality of zero sets. `canonical_polynomial_reassociation`
identifies the left side with the existing `recursivePolynomial W (n+2)`
at every canonical stage, with D=2^(n+1). [pcc:unverified]

The proof first instantiates the fixed-size interchange theorem using
literal S3, verifies its bidegrees2 and variable-swap invariance, and
identifies the inner resultant with existing `lastPolynomial` (S4).
Independent x/y variables followed by actual coefficient evaluation
cover x=y. Adjoining an independent leading coefficient, then evaluating
it at0, covers every retained D≥deg P. A final generic evaluation map is
proved the identity on the ring of three independent variables, giving
the literal polynomial equality and the canonical recursion bridge.
[pcc:unverified]

The argument order is explicit: the arbitrary-b helper yields
S4(x,y,z,U). For the actual b-invariants of a Weierstrass curve,
`actual_S4_middle_swap` and the checked cyclic rotation prove equality
with source-ordered S4(z,x,y,U). `canonical_source_order_reassociation`
therefore normalizes the source's k=2 split with an S4 block at every
arity≥5, as a literal polynomial equality. Nonsingularity is not needed
for this algebraic identity. Full arbitrary splits k≥3 and higher-arity
permutation symmetry/absolute irreducibility remain open. [pcc:unverified]

A direct rational negative control disproves cross-pair symmetry for
arbitrary independent b-coefficients: b=(0,0,0,1), inputs(0,1,2,3), with
the middle two inputs exchanged. Lean also proves that no Weierstrass
curve over ℚ has these b-coefficients, using the actual b-relation.
This is a restriction control, not a qualified source counterexample.
[pcc:unverified]

Reproduce from `lean/` with `lake build`; from repository root use
`tools/lean/proof-gate.sh research/semaev_reassociation_audit_20261113/lean`.
Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
The issued [interchange packet](../resultant_interchange_audit_20261113/README.md)
is unchanged. Failed sources, build logs and exits are retained for the
new packet. No custom axioms, native evaluation, raised limits or waivers.
[pcc:unverified]

See [proof map](proof-map.json), [exact source and artifact pins](sources.json),
[primary review](evidence/source-review.json), [PDF report](reports/report.pdf)
and editable [proof-flow diagram](reports/proof-flow.svg). Eighty-two
attempt source/log/exit files and ten initial/final gate files are retained
losslessly. The single-page report46509B was visually inspected with all
text inside the page. [pcc:unverified]

Sources reviewed: Semaev2004 Theorem1/p2 arbitrary splits;
Kosters–Yeo2015, arXiv1503.08001v3, Section2/p3 general S3 and recursion,
including visual formula inspection; actual canonical S3/S4/degree/
evaluation modules and Mathlib polynomial maps/resultants. KB MCP and
repository executable unavailable; bounded fallback, no novelty claim.
This is original item1 progress; the
[all-fourteen requirement map](../../problems/fourteen-target-audit-proof-and-execution-gaps.md)
retains all remaining obligations. No performance experiment, corpus or
leaderboard promotion; existing performance graphs stay unchanged.
Paid compute and agent uploads are zero. [pcc:unverified]
