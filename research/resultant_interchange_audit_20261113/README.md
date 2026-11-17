# Resultant interchange and a degree-drop control

This component proves a fixed-size iterated-resultant identity over every
commutative integral domain, deriving both intermediate degree bounds.
It also directly disproves the unrestricted identity obtained by resetting
outer sizes to natural degrees. Whole gate46/0failures/0trusted and the
exact local replay of **PCC-20261113-97e420** pass. The complete binding is
24ccb18a96629b4469b65e1a4626eeff74ff6682d1e45f5c28d3107aecfc93d6;
signed standing remains unverified. See [receipt](verification.json).
[pcc:unverified]

Let `P(y), R(x)` have natural degrees `a,c`, and let `Q(x,y)` have
degree at most `dx` in x and `dy` in y. A resultant below lists its
two declared Sylvester sizes after the semicolon. Lean proves the following.
[pcc:unverified]

```text
Res_x(Res_y(P,Q; a,dy), R; a*dx,c)
  = Res_y(P, Res_x(Q,R; dx,c); a,dy*c).
```

The two inner natural degrees are bounded by `a*dx` and `dy*c`.
No splitting, monicity, nonzero, separability, characteristic or
degree-preservation hypothesis is required for the domain theorem.
The separate natural-size theorem explicitly assumes that both bounds
are equalities. [pcc:unverified]

The literal rational counterexample is `P=y²−y`, `Q=xy+1`, `R=2x+1`.
Lean checks both bidegrees of Q are1, the inner polynomials are
`x+1` and `y−2`, and the first has degree1 strictly below its
declared bound2. [pcc:unverified]

| Outer-size convention | Left | Right |
|:--|--:|--:|
| Recompute natural degree | −1 | 2 |
| Retain formal degree bounds | 2 | 2 |

These are exact polynomial identities, not experimental estimates.
The counterexample concerns the unqualified helper above; it does not
refute Semaev's qualified recursion or prove failure of the canonical
family, whose sharp degrees must be connected separately. [pcc:unverified]

The proof expands the two boundary resultants as products over root
multisets, retaining multiplicities and the resultant-commutation sign.
The two nested products agree by finite-product interchange. Derived
bidegree bounds justify every product formula. Injective coefficient-map
and variable-swap lemmas descend from algebraic closure to every field,
then from fraction field to every domain. [pcc:unverified]

Reproduction, from `lean/`: `lake build`. From repository root,
`tools/lean/proof-gate.sh research/resultant_interchange_audit_20261113/lean`.
See [proof map](proof-map.json), [source and artifact pins](sources.json),
[source review](evidence/source-review.json), retained attempts and gate
archives, and [PDF report](reports/report.pdf) with its editable SVG.
Toolchain Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679
are pinned. No native evaluation, custom axioms, raised elaboration limits
or linter waivers are introduced. [pcc:unverified]

This is progress on original item1. Actual arbitrary-split normalization,
higher-arity permutation symmetry and higher-arity absolute irreducibility
remain open. All fourteen requested items retain their original scope in
the [requirement map](../../problems/fourteen-target-audit-proof-and-execution-gaps.md).
No benchmark, corpus or leaderboard standing changes; canonical performance
graphs are unchanged because this component contains no timing experiment.
No paid compute or agent storage upload. [pcc:unverified]
