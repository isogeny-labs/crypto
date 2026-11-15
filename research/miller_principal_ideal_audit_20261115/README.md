# Actual affine ideals and evaluation for Miller lines

Lean proves the actual coordinate-ring chord identity
`I(P) * I(Q) * I(-P-Q) = (line(P,Q))` for every field, general Weierstrass model and nonsingular affine points with the nonvertical guard. For an actual affine point satisfying `3 • P = 0`, the tangent principal ideal equals `I(P)^3`. This supplies the affine ideal/function/evaluation part of the remaining Miller bridge, beyond earlier polynomial intersection certificates. The projective infinity valuation and canonical divisor/pairing identification remain open. [pcc:unverified]

[Main certificate](lean/MillerIdealCertificate.lean) · [Proof map](proof-map.json) · [One-page report](reports/report.pdf) · [Pinned source plan](sources.json)

## Mathematical content

[LineIdeals](lean/LineIdeals.lean) uses Mathlib’s actual `CoordinateRing`, point ideals and group law. The chord-product and negative-point-product identities yield the claimed principal ideal after cancellation of a nonzero principal X-ideal factor. Cancellation is proved by explicit ideal membership and integral-domain cancellation. Actual three-torsion derives both the nonvertical tangent guard and `2P = -P`; the resulting coordinate equalities specialize the product to the cube. No rank, pairing or infinity-valuation hypothesis is assumed. [pcc:unverified]

The actual regular line class `Y - line(X)` is nonzero, as is its image in Mathlib’s actual `FunctionField`. [LineEvaluation](lean/LineEvaluation.lean) defines actual affine point evaluation using `AdjoinRoot.evalEval`, a ring homomorphism. Evaluation of the X and Y classes gives `yr - y - slope * (xr - x)`. On a short model, the tangent slope specializes to `(3*x^2 + a4)/(2*y)`, connecting the regular function to the numerical Miller tangent expression. This does not assign a regular value at infinity. [pcc:unverified]

[GuardControls](lean/GuardControls.lean) uses the actual nonsingular curve `y² = x³ + 1` over F5. The point `(0,1)` is killed by 3 and satisfies the cube identity. The point `(2,2)` is nonsingular, has a nonvertical tangent, and is not killed by 3. Its cube identity is false: the tangent vanishes at `(0,1)`, whereas `(X-2)^3` evaluates to 2 there. Thus removing the three-torsion guard is disproved in Lean, while the correctly qualified identity is proved. [pcc:unverified]

## Checks and reproducibility

The whole tracked project gate audits **25 theorems, 0 failures and 0 trusted native proofs**. Only `propext`, `Classical.choice` and `Quot.sound` occur. Final build 08 and the project build logs have no warnings or errors. The frozen audit tool emits one existing `String.mk` deprecation warning; it is retained and is not a proof-source warning. Four new modules import Mathlib and one another, with no earlier custom proof modules copied or imported. Lean 4.35.0 and Mathlib `56a005855ea17e0b0958f73ec801ae140b0a1679` are pinned; default limits are retained. [pcc:unverified]

The complete 1470-character written declaration has SHA256 `d2b0fd734afc3313003d0373976189f696193cbf25807d2b3b20610645ef40ce`. The separate native source-processing check reports 1 complete match, 0 truncated prefixes and 0 unresolved cases. This is procedural header processing; it does not prove the parser, compiler, hash implementation or physical-host execution correct. [pcc:unverified]

Eight snapshots retain five failed build states and successful builds 04, 07 and 08. Attempt 03 used incorrect relative edit/snapshot paths and rebuilt unchanged code; the actual effects were inspected and that state captured before further editing. Other failures involved power simplification, evaluation transparency, ring-hom kernel notation and a missing local F5 prime instance. Build 07’s unused premise names were replaced with anonymous premises. All **100 provenance files / 44 unique objects** roundtrip through the **63732-byte** lossless deduplicated archive, alongside five gate files. The initial report draft is retained and superseded: its audit-warning claim missed the inherited deprecation diagnostic. No failure is hidden or treated as a mathematical counterexample. [pcc:unverified]

```sh
cd research/miller_principal_ideal_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/miller_principal_ideal_audit_20261115/lean
python3 research/miller_principal_ideal_audit_20261115/restore_attempts.py /tmp/miller-ideal-attempts
python3 research/miller_principal_ideal_audit_20261115/export_binding_input.py --out /tmp/miller-ideal-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/miller-ideal-binding/manifest.json
```

## Sources and limits

Galbraith chapter 26, extended corrected version 2.0, is the primary source: 222784 bytes, 16 PDF pages, SHA256 `c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e`. Printed pages 574–577 / PDF pages 2–5 were freshly read in full as text and visually inspected in the preceding work; not all 16 pages were read. Definition 26.3.4 and equations 26.1–26.2 motivate the divisor/line bridge. Copyrighted primary files remain local. [pcc:unverified]

Earlier `MillerCurve`, `MillerExpressions` and `TangentCertificate` were fully read through read-only Git and byte-checked against their frozen source plan. Their actual arithmetic, numerical expression and polynomial identity arguments are credited. The official Mathlib coordinate-ring/ideal/evaluation APIs were read before implementation. Original files remain unchanged. Fresh KB/Conductor calls returned 127; explicit prior/primary/official-source fallback and broad prior gap searches were recorded. No absence or novelty claim follows. See [source review](evidence/source-review.json). [pcc:unverified]

This does not prove the projective infinity pole/valuation, canonical principal divisor, translation pullback, Weil reciprocity, isogeny/Lang/Cartier/Jacobian bridge or canonical Weil/Tate/MOV/Frey–Rück theorem. It does not complete all fourteen items. The same prover, Mathlib and host are used; this is not independent implementation or physical-host validation. No canonical corpus promotion or signed-standing claim is made. [pcc:unverified]

Exact local **PCC-20261115-357b60** replay PASS at 2026-11-15T04:16:36Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `1d97f3f602ed8088993c84da0ae471b838a26d54` and local receipt `f934d24d30275b298913f23b23319a351beecb77` share tree `c0916904c71dba9574bbcfeb9c3f1d4ad447caa0`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. This publication metadata is outside the frozen source plan. [pcc:unverified]
