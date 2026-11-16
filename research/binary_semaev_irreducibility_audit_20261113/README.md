# Ordinary binary S3: actual absolute irreducibility

The literal ordinary binary polynomial
`(xy+xz+yz)²+xyz+b` is symmetric and irreducible over every
characteristic-two field, and remains irreducible after every field
scalar extension. Lean binds its evaluation to the existing ordinary
S3 formula and its zeros to actual Mathlib point sums on every
nonsingular model `y²+xy=x³+a₂x²+b` over an algebraically closed field.
Whole gate passed43declarations with zero failures and zero trusted native
exceptions. Exact local **PCC-20261113-1e2412** replay passed at2026-11-13T20:58:19Z;
signed standing remains unverified. See [receipt](verification.json).
[pcc:unverified]

The proof derives the actual Artin–Schreier change of variable. A generic
root would give `v²+v=Y+c+s/Y` after adjoining a square root of b.
The right side has actual rational-function degree1; an unequal-degree
sum argument rules out every such root. An injective coefficient map and
proved primitive Gauss bridge descend irreducibility from the algebraic
closure, so the final theorem assumes no square root, finite-field
enumeration or irreducibility premise. The multivariate/nested polynomial
identification is explicit. [pcc:unverified]

The closed certificate also proves the short expression at F2,a=b=0 is
a nonunit square and is not irreducible, and proves it differs from the
ordinary binary polynomial. This distinguishes two actual formulas.
It confirms the qualified ordinary formula; it does not refute it.
The irreducibility statement permits b=0 as polynomial algebra, while
the actual elliptic-group interpretation retains nonsingularity.
General higher-arity absolute irreducibility and splits k≥3 remain open.
[pcc:unverified]

Main declaration:
`BinarySemaevIrreducibilityAudit.actual_binary_S3_certificate` in
[BinaryCertificate.lean](lean/BinaryCertificate.lean).
Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
Only standard foundations; no holes, custom axioms, native evaluator,
raised limits, global elaboration flags or waivers. Failed sources/logs/
exits are retained losslessly. No native experiment, paid compute or
agent upload. [pcc:unverified]

Sources checked: Semaev2015section2, actual PDFpage3 formula visually
inspected; Kosters–Yeo2015section2general invariant formula/Proposition2.1
and4.2ordinary hypotheses; original Semaev2004Theorem1has characteristic
not2/3. Existing ordinary BinaryS3 group equivalence and separate short
square-control/primitive/MVtransport proofs were reviewed. Pinned Mathlib
RatFunc degree, algebraic closure, Gauss and MV APIs supply the explicit
bridges. KB unavailable; bounded primary/repository/library fallback,
no novelty claim. [pcc:unverified]

Reproduce from the repository root:
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/binary_semaev_irreducibility_audit_20261113/lean
```

All fourteen requirements and remaining obligations remain in the
[current map](../theorem_census_20261109/README.md#original-fourteen-requirements).
[pcc:unverified]
