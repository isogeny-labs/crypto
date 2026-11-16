# Actual all-n Miller functions

Lean constructs nonzero actual rational functions `f(n,P)` for every natural `n` and actual nonsingular curve point `P`, proves their principal point divisors `n(P)−([n]P)−(n−1)(O)`, and derives source infinity normalization. No divisor or normalization premise is supplied for these constructed functions. [pcc:unverified]

The actual step `g(P,Q)` is one when either point is zero, the actual vertical function when the points are negatives, and the actual chord divided by the vertical through their sum otherwise. Lean proves its exact divisor `(P)+(Q)−(P+Q)−(O)` in every branch. The recursion is `f0=1`, `f(n+1)=fn*g([n]P,P)`. Integer subtraction in the coefficient `n−1` gives an explicit consistent `n0` convention; `f1=1` and the torsion specialization `div(fn)=n(P)−n(O)` are proved. [pcc:unverified]

The divisor is a finite-support `Finsupp` of the **actual normalized local orders** proved in the [preceding packet](../miller_finite_point_audit_20261115/README.md). Over arbitrary `K` it is on nonsingular `K`-points; for an elliptic model over algebraically closed `K`, these are all geometric points. Separate interoperability with Mathlib’s projective `Scheme.localRing` or `Divisor` API is not asserted. [pcc:unverified]

[LocalOrderAlgebra](lean/LocalOrderAlgebra.lean) derives actual order and point-divisor multiplication/division from the official local valuations with nonzero-function guards. [MillerStep](lean/MillerStep.lean) constructs the actual every-branch step and proves its nonzero/divisor property. [AllMiller](lean/AllMiller.lean) constructs all functions and proves finite support, principal point divisors, the torsion specialization and the addition invariant. [pcc:unverified]

[InfinityMonic](lean/InfinityMonic.lean) proves that scaling an actual function by `t^(−ord_O(f))`, with source `t=X/Y`, gives a leading unit of residue one. Actual near-one valuation algebra, nonzero parameter powers and the proved line/vertical normalizations give closure under multiplication/division, normalization for every step and constructed function, and the normalized source addition invariant. This says the **leading residue** is one; it does not say the entire power series is identically one. [pcc:unverified]

Any valid normalized `i,j` functions yield a valid normalized `i+j` function `fi*fj*g([i]P,[j]P)`. The proof does not infer equality of differently constructed functions from a common divisor, nor claim a binary-program/runtime or complexity refinement. The supplied recursion is the explicitly defined successor recursion. [pcc:unverified]

[RecursionControls](lean/RecursionControls.lean) proves that the actual F5 point `T=(4,0)` on `y²=x³+1` is nonzero and satisfies `T+T=O`. The correct vertical step has infinity order `−2`. Treating it as an ordinary nonvertical chord gives order `−3`, and Lean disproves the resulting actual divisor assertion. The source explicitly asks for exceptional-case simplifications, so its qualified statements survive. [pcc:unverified]

The main declaration is `MillerRecursionAudit.miller_recursion_certificate` in [RecursionCertificate](lean/RecursionCertificate.lean). The [proof map](proof-map.json) names all covered constructions and controls. Thirty-three modules comprise 27 exact credited earlier modules and six new ones. Seven snapshots retain failures 02 and 05 and successful builds 01, 03, 04, 06 and 07. Final07/project logs have no warnings/errors; the frozen auditor emits one inherited `String.mk` deprecation. No resource limit, global elaboration flag or linter waiver was added. [pcc:unverified]

See the [source plan](sources.json), [preflight](preflight.json), [verification receipt](verification.json), [source review](evidence/source-review.json) and [one-page report](reports/report.pdf). The complete written-header check is procedural source processing, not a kernel theorem about parser/hash/compiler execution or physical-host assurance. Signed standing remains unverified. [pcc:unverified]

```sh
cd research/miller_recursion_audit_20261115/lean
lake build
```

```sh
python3 research/miller_recursion_audit_20261115/restore_attempts.py /tmp/miller-recursion-restored
```

The primary source is Galbraith chapter 26, <https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf>, 222784 bytes / 16 PDF pages, SHA256 `c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e`. PDF pages 2–6 / printed 574–578 were fully text-read; pages 2–6 were visually inspected across prior and fresh readings. Not all 16 pages were read. Targets are Definition 26.3.4, Exercise 26.3.5, Algorithm 29 and Exercise 26.3.6. Prior chapter 7 local-order/divisor definitions, exact official APIs and earlier proofs are credited in the source review. KB/Conductor returned 127; explicit fallback was used. Copyrighted primary copies remain local. No absence or novelty claim follows. [pcc:unverified]

General all-function divisor finiteness, translation pullback, Weil reciprocity, isogeny/Lang/Cartier/Jacobian, canonical Weil/Tate/MOV/Frey–Rück and full original fourteen-item completion remain open. The same host, prover and Mathlib are used; no new independent implementation or physical-host assurance, canonical corpus promotion or signed-standing claim is made. [pcc:unverified]

Exact local **PCC-20261115-529612** replay PASS at 2026-11-15T06:52:24Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `0a8d9dff3cac788a9a04e632dd6719a01a98ee86` and local receipt `344a957c81c7dcd6e4ba4083ac5673514225837e` share tree `21aa83f876a3dc293997b59d88113944b1758c7b`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
