# S3 symmetry and absolute irreducibility in Lean

Lean proves that the actual three-variable short-Weierstrass S3 polynomial
is symmetric and absolutely irreducible over every field with 2≠0,
for arbitrary coefficients a,b. The absolute statement is explicit:
after every field homomorphism K→L, the mapped polynomial is irreducible.
Evaluation is proved equal to the existing `SemaevS3.S3` definition.
[pcc:unverified]

The whole-project gate passes 64 declarations with zero failures and
zero trusted-native exceptions. [pcc:unverified]

This checks the n=3 algebraic part of Semaev's original Theorem1.
The source assumes characteristic other than2/3 and a nonsingular short
Weierstrass curve. The new algebraic proof requires only 2≠0; it does
not assert an elliptic-curve group interpretation for singular inputs.
Higher-arity absolute irreducibility, full permutation invariance and
arbitrary-split normalization remain separate obligations.
[pcc:unverified]

The literal polynomial is

```text
(x−y)²z² − 2((x+y)(xy+a)+2b)z + (xy−a)² − 4b(x+y).
```

For e1=x+y+z, e2=xy+xz+yz and e3=xyz, Lean proves its exact form.
[pcc:unverified]

```text
e2² − 4 e1 e3 − 2 a e2 − 4 b e1 + a².
```

Mathlib's elementary-symmetric-polynomial invariance then proves actual
`MvPolynomial.IsSymmetric` under every permutation of the three variables.
[pcc:unverified]

| File | Structural proof and principal declarations |
|:--|:--|
| [OddDegree](lean/OddDegree.lean) | `odd_degree_not_square`: an odd-degree polynomial times a nonzero scalar cannot be a square in the actual rational-function field. Numerator/denominator degree accounts for all rational functions. |
| [S3FunctionField](lean/S3FunctionField.lean) | Exact discriminant16f(x)f(y), no rational-function root, degree2 and `actual_function_field_quadratic_irreducible` over K(x)(y). |
| [PrimitiveBridge](lean/PrimitiveBridge.lean) | Actual injective coefficient embedding K[x,y]→K(x)(y), literal polynomial-map identity, proved primitive condition and `actual_nested_source_irreducible` via Gauss's lemma. |
| [ActualPolynomial](lean/ActualPolynomial.lean) | Actual algebra equivalences to `MvPolynomial(Fin3)K`; exact source evaluation; structural symmetry; `sourceMv_irreducible_after_field_map`; characteristic2 negative control; complete closed certificate. |

All new declarations have namespace `SemaevIrreducibilityAudit`.
The main closed declaration is
`actual_S3_absolute_irreducibility_with_control`. [pcc:unverified]

The primitive condition is a proved bridge. The leading coefficient is
(x−y)². The prime diagonal y−x does not divide the middle coefficient:
evaluation at y=x gives −4(x³+ax+b), a nonzero polynomial. Thus every
common divisor of the leading and middle coefficients is a unit.
Gauss's injective-map theorem transfers irreducibility from K(x)(y)
back to the actual nested polynomial ring. Explicit algebra equivalences
then reach the literal three-variable polynomial; no irreducibility,
isomorphism or primitive condition is assumed as a premise.
[pcc:unverified]

## Characteristic2 negative control

Lean proves the direct negation of irreducibility for the literal short
formula over F2 at a=b=0. It is the following square. [pcc:unverified]

```text
((x+y)z+xy)²,
```

and its factor is not a unit, as evaluation at the origin gives zero.
This demonstrates that the characteristic restriction cannot simply be
dropped from this algebraic statement. It does not refute the original
qualified theorem or the distinct ordinary characteristic2 elliptic model.
The earlier [literal binary S4 display audit](../semaev_source_audit_20261110/README.md)
retains its separate actual F4 counterexample and additive repair.
[pcc:unverified]

## Source and assurance

The original six-page author manuscript is dated5February2004, source
ID `S-SEMAEV-2004`, SHA256
`1cab0ae675996cdba6393b1e369a22113fd2e74bcf1223595743dc140631c7d6`.
Theorem1 and its page3 irreducibility paragraph were re-read; the page
and enlarged paragraph were visually inspected. Its field is the
algebraic closure of K(X3): the overbar covers the entire function field.
Plain-text extraction omits that distinction. No field-of-definition
counterexample is asserted. Exact URL/read scopes are recorded in the
source-review receipt. KB unavailable (MCP absent/wrapper127); bounded
primary/repository/Mathlib fallback. No novelty claim. [pcc:unverified]

```sh
cd research/semaev_irreducibility_audit_20261113/lean
lake build
cd ../../..
tools/lean/proof-gate.sh --out /tmp/semaev-irred-gate \
  research/semaev_irreducibility_audit_20261113/lean
```

Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
Only propext, Classical.choice and Quot.sound are used. No holes, custom
axioms, native_decide, raised elaboration limits or waived checks.
Failed sources/logs/exits and the complete gate are retained. Local
checks and independently signed standing remain distinct. No new
native experiment, paid compute or agent storage upload. [pcc:unverified]

Exact local replay for **PCC-20261113-5ce98d** passes at
2026-11-13T16:33:29Z. All11 formal/config source pins and11 artifact
pins match. Complete closed binding
`c4e043d69cf67dc96e73cf41fbbf473ac0de192b56b1e39fb869595dd90f30ce`.
Signed standing remains unverified. [verification.json](verification.json)
retains the actual verdict and inspected report hash; [sources.json](sources.json)
records immutable source/gate/attempt pins. [pcc:unverified]
