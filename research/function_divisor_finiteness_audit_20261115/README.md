# Actual general-function divisors and rational-point scope

Lean proves that every nonzero actual Weierstrass rational function has finitely many nonzero actual normalized local orders. It constructs its general principal point divisor, product/quotient laws, the actual function-unit-group additive divisor homomorphism and principal subgroup range. No finite-support premise is supplied. [pcc:unverified]

An actual counterexample tests the scope: on `y²=x³+x+1` over `F3`, the nonzero, nonconstant function `(X²+1)/(X²+X+2)` has zero divisor on **all rational points**, including infinity, and is normalized at infinity. This disproves inferring constantness from zero **rational-point-only** divisor. The source uses geometric points over the algebraic closure; its qualified statement is not refuted. [pcc:unverified]

[NormEvaluation](lean/NormEvaluation.lean) proves that actual coordinate-ring evaluation agrees with polynomial evaluation, and that a coordinate function zero implies its actual algebra norm polynomial vanishes at the x-coordinate. The norm polynomial of a nonzero actual coordinate function is nonzero. The actual polynomial-function embedding into the actual `FunctionField` is proved injective from the actual rank-two coordinate basis. [pcc:unverified]

[FiniteZeros](lean/FiniteZeros.lean) proves finite root sets and finite monic-quadratic y-fibers, then uses actual coordinate injection and finite unions/images/preimages to derive finite affine point zero sets. The proof works in every characteristic and over every field. [pcc:unverified]

[FunctionDivisor](lean/FunctionDivisor.lean) proves that a function nonvanishing under actual point evaluation is a unit in the actual localized DVR and has official normalized adic value one. Every actual fraction `a/b` consequently has nonzero local orders only in the finite zeros of `a`, zeros of `b` and infinity. It constructs the `Finsupp` of actual normalized local orders, proves product and quotient rules, constructs the actual additive homomorphism from function units and its principal subgroup range, and proves compatibility with the [actual all-n Miller divisors](../miller_recursion_audit_20261115/README.md). [pcc:unverified]

[PolynomialValues](lean/PolynomialValues.lean) computes actual polynomial-function infinity values from actual degree. [RationalControls](lean/RationalControls.lean) proves the F3 curve discriminant is nonzero; both quadratic polynomials have no F3 root. Injective polynomial functions and coefficient comparison prove the ratio differs from every constant. Both infinity values are `exp4`, so its infinity order is zero. The difference from one has actual value `exp(−2)`, proving leading-residue-one normalization. Thus every actual rational-point coefficient is zero while the function is nonconstant and normalized. [pcc:unverified]

The main declaration is `FunctionDivisorFinitenessAudit.function_divisor_finiteness_certificate` in [FunctionDivisorCertificate](lean/FunctionDivisorCertificate.lean). The [proof map](proof-map.json) names the independently checked constructions and counterexample.39modules comprise33exact credited earlier modules and6new. Five snapshots preserve failures02/04 and successes01/03/05. Final05/project logs have no warnings/errors; the frozen auditor has one inherited `String.mk` deprecation. No raised resource limit, global elaboration flag or linter waiver was added. [pcc:unverified]

See the [source plan](sources.json), [preflight](preflight.json), [verification receipt](verification.json), [source review](evidence/source-review.json) and [one-page report](reports/report.pdf). The complete written-header check is procedural source processing, not a kernel proof of parser/hash/compiler execution or independent physical-host assurance. Signed standing remains unverified. [pcc:unverified]

```sh
cd research/function_divisor_finiteness_audit_20261115/lean
lake build
```

```sh
python3 research/function_divisor_finiteness_audit_20261115/restore_attempts.py /tmp/function-divisor-restored
```

Primary: Galbraith chapter7 extended corrected version2.0, <https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf>,281643bytes/21PDFpages/SHA256 `c5b9a25eeb00c280f9e723b7290533d3f62ecfcc0995731e9e30e29503a6b942`. PDF1–2,7–10,13–16 were text-read across prior/fresh readings; PDF2,8,9,13,15,16 visually inspected. Fresh PDF13/16 were fully text-read/visually inspected. The target is Theorem7.7.1's plane-curve finiteness argument, specialized to the actual Weierstrass norm. Full elliptic Theorem7.7.11 proof/Lemma7.7.12 and Corollaries7.7.13/7.7.14 were read as context after the prior packet issued; they are not formally proved here. Not all21pages were read. Copyrighted primary copies remain local. KB/Conductor calls returned127; explicit primary/prior/official-source fallback is recorded. No absence or novelty claim follows. [pcc:unverified]

Over arbitrary `K`, the model is the divisor on nonsingular `K`-points. For an elliptic model over algebraically closed `K` these are all geometric points. Geometric degree-zero and constant-kernel/uniqueness theorems, algebraic-closure base-change comparison, separate Mathlib `Scheme.localRing`/`Divisor` interoperability, translation/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical Weil-Tate/MOV-Frey–Rück and full original14completion remain open. Same host/prover/Mathlib; no new independent validation, canonical corpus promotion or signed-standing claim. [pcc:unverified]

Exact local **PCC-20261115-c81d29** replay PASS at 2026-11-15T07:24:51Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `ff8dab58c6d1136ca1a1a913bdc167899f7c77cb` and local receipt `9f5048ce2d858287caf7ae64ca7722ec258ee70a` share tree `6a61d30b0f23d810b4e1c3ff0a9e477db74e45d6`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
