# Actual finite-point local rings and Miller divisors

Lean derives the actual finite-point local DVRs, their full function fields and normalized adic zero orders. Combining those with the proved infinity chart pole orders yields the chord/tangent, vertical and three-torsion tangent principal point divisors. Multiplicities and regularity are derived from the actual curve equation and ideals. [pcc:unverified]

The identities are

- `div(line) = (P) + (Q) + (−P−Q) − 3(O)` for a nonvertical chord/tangent;
- `div(vertical) = (P) + (−P) − 2(O)`, including repeated two-torsion;
- `div(three-torsion tangent) = 3(P) − 3(O)`. [pcc:unverified]

Here `div` is a `Finsupp` constructed from the **actual normalized local valuations**, after proving their finite support for the specified functions. Over arbitrary `K` it is indexed by nonsingular `K`-points. For a smooth elliptic model over algebraically closed `K`, those are all geometric points. A separate comparison to Mathlib’s projective `Scheme.localRing` or `Divisor` API is not asserted. [pcc:unverified]

[PointLocalization](lean/PointLocalization.lean) proves that actual coordinate evaluation is surjective with kernel the actual point ideal `(X−xP,Y−yP)`, and identifies the maximal ideal in its actual localization. [FiniteDVR](lean/FiniteDVR.lean) proves the exact shifted relation. [pcc:unverified]

`V(V+a1 U+A) = U(U²+(3xP+a2)U+B)`,

where `U=X−xP`, `V=Y−yP`, `A=2yP+a1xP+a3` and `B=3xP²+2a2xP+a4−a1yP`. Nonsingularity gives `A≠0` or `B≠0`. The corresponding bracket evaluates to a nonzero scalar and becomes a unit after localization. Thus `U` or `V` generates the nonzero maximal ideal; the Noetherian local domain is proved to be a DVR, with no DVR premise. This works in every characteristic. [pcc:unverified]

[FiniteFractionField](lean/FiniteFractionField.lean) constructs the explicit injective map from that localization into the actual `W.FunctionField` and proves full fraction-field equality. [FiniteValuation](lean/FiniteValuation.lean) uses the official normalized height-one adic valuation: the derived parameter has value `exp(−1)`, units have value one, and an actual principal ideal equal to the nth maximal-ideal power gives exact order `n`. [pcc:unverified]

[LineMultiplicity](lean/LineMultiplicity.lean) proves that an actual point ideal maps to the maximal ideal at the same point and to the unit ideal at every other point. Localizing the previously proved actual chord and negative-pair ideal identities therefore counts all finite intersections, including repetitions. [PointDivisor](lean/PointDivisor.lean) combines these exact finite orders with the actual infinity-chart orders `−3` and `−2`, proves finite support, and establishes the displayed identities. [pcc:unverified]

[FiniteControls](lean/FiniteControls.lean) gives actual controls on `y²=x³+1` over `F5`. At `(4,0)`, the vertical function has order two and `X−4` fails to generate the actual local maximal ideal. The ordinary tangent at `(2,2)` has order two there and an additional simple zero at `(0,1)`. Its actual principal point divisor differs from `3(P)−3(O)`. These disprove deleting the two-torsion/three-torsion exceptions; the qualified source statements survive. [pcc:unverified]

The main declaration is `MillerFinitePointAudit.miller_finite_point_certificate` in [FinitePointCertificate](lean/FinitePointCertificate.lean). The [proof map](proof-map.json) also names the independently audited helper declarations and the explicit divisor counterexample. Nineteen earlier modules are copied byte-identically and credited; eight modules are new. The [infinity chart](../miller_infinity_chart_audit_20261115/README.md), [norm valuation](../miller_norm_valuation_audit_20261115/README.md) and [actual principal ideals](../miller_principal_ideal_audit_20261115/README.md) are the reused proof packets. [pcc:unverified]

Twelve build snapshots retain seven failures and successes 03, 04, 06, 11 and 12. Final12/project logs have no warnings/errors. The frozen auditor emits one inherited `String.mk` deprecation, which is recorded separately. Failures involved equation-proof/coefficient-map transparency, exact API arguments, multiline set syntax, opaque local maps, equality orientation, point-zero notation, rewrite order and concrete finite-field implications. Style advisories were resolved structurally. All failed states and diagnostics are retained; no resource limit, global elaboration flag or linter waiver was added. [pcc:unverified]

The [source plan](sources.json) pins the formal sources and evidence. The [preflight](preflight.json), [verification receipt](verification.json), [source review](evidence/source-review.json) and [one-page report](reports/report.pdf) distinguish theorem checks from source-processing and publication assurance. The native written-header check is procedural; it is not a kernel proof about the parser, hashing, compiler or physical host. Signed standing remains unverified. [pcc:unverified]

To rebuild the proof:

```sh
cd research/miller_finite_point_audit_20261115/lean
lake build
```

To restore every archived build-state byte:

```sh
python3 research/miller_finite_point_audit_20261115/restore_attempts.py /tmp/miller-finite-restored
```

Primary source: Galbraith chapter 7, extended corrected version 2.0, <https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf>, 281643 bytes / 21 PDF pages, SHA256 `c5b9a25eeb00c280f9e723b7290533d3f62ecfcc0995731e9e30e29503a6b942`. PDF pages 1–2, 7–10 and 14–15 were fully text-read; pages 2, 8, 9 and 15 were visually inspected. The finite-point part of Example 7.3.4 and full proof of Lemma 7.7.10 were freshly reread as text. Definition 7.7.2 fixes the divisor convention. Not all 21 pages were read; Theorem 7.7.11 is not fully read/proved. Chapter 26’s prior printed pages 574–577 / PDF pages 2–5 were read, not all 16 pages. Copyrighted primary copies remain local. Fresh KB status/mechanism/negative/theorem and Conductor calls returned 127; explicit primary/prior/official-source fallback is documented. No absence or novelty claim follows. [pcc:unverified]

General all-function divisor finiteness, translation pullback, Weil reciprocity, isogeny/Lang/Cartier/Jacobian, canonical Weil/Tate/MOV/Frey–Rück and the complete original fourteen-item request remain open. The same host, prover and Mathlib are used; this is not new independent implementation or physical-host assurance. No canonical corpus promotion or signed-standing claim is made. [pcc:unverified]

Exact local **PCC-20261115-bb5771** replay PASS at 2026-11-15T06:18:06Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `756c274a6b75323069fcab94a37343f44e4b1238` and local receipt `d9313b0d2e5af5d77a603065340589b8bb148f43` share tree `e94b5985dd3763904f6c08a27f6ebdd358e2783a`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
