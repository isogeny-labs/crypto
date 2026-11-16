# Actual Weierstrass negation pullback

Lean constructs pullback by actual Weierstrass point negation on the actual coordinate ring, full function field and actual point localizations. It derives the official normalized valuation formula

`v_P(iota* f) = v_{−P}(f)`

and the resulting actual principal-divisor permutation. An actual nonzero tangent function on the nonsingular curve `y²=x³+1` over `F5` has order3 at `P=(0,1)` and order0 at `−P`. This disproves deleting function pullback from the statement; qualified Galbraith Lemma7.7.12 survives. [pcc:unverified]

The main certificate is [`NegationPullbackAudit.negation_pullback_certificate`](lean/NegationCertificate.lean). The [proof map](proof-map.json), [source plan and complete statement](sources.json), [one-page report](reports/report.pdf), [source review](evidence/source-review.json) and [pre-edit protocol](PROTOCOL.md) record the exact scope. [pcc:unverified]

For `Y²+H(X)Y=F(X)`, the actual `AdjoinRoot.lift` sends `Y` to `−Y−H(X)` and fixes coefficient polynomials. The actual root equation proves the map well-defined; extensionality proves its square is identity. This asserts involutivity, without asserting exact order2 for every model and characteristic. The actual rank-two basis proves norm preservation and the evaluation identity. Official fraction-ring extension gives the full actual function-field automorphism. [pcc:unverified]

Evaluation identifies allowed denominators, giving the actual point-localization isomorphism. Its actual field inclusion commutes with pullback; maximal ideals and uniformizer generators transport. Earlier nonsingularity-derived DVR and full-fraction-field proofs give normalized value `exp(−1)` for the transported parameter and value1 for transported units. Every nonzero actual field element is a unit times an integer power of a parameter, so the valuation identity follows for every function. Zero is handled separately. Infinity follows from actual algebra-norm preservation and the earlier identification of the actual infinity-chart adic with the norm valuation. All actual point orders and general finite-support principal divisors therefore transport. No valuation-invariance hypothesis is supplied. [pcc:unverified]

The official actual-coordinate `coe_norm_smul_basis` identity also gives the product of a function and its actual conjugate as its actual algebra norm; Lean extends this to full function-field fractions. For the control, the actual three-torsion tangent divisor is `3(P)−3(O)`. Since `P≠−P`, the same nonzero function has different orders at these two points. The correctly pulled-back function has order0 at `P`. [pcc:unverified]

Reproduce the complete project and every tracked theorem's axiom check:

```sh
cd research/negation_pullback_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/negation_pullback_audit_20261115/lean
python3 research/negation_pullback_audit_20261115/export_binding_input.py --out /tmp/negation-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/negation-binding/manifest.json
python3 research/negation_pullback_audit_20261115/restore_attempts.py /tmp/negation-attempts
```

The47modules reuse39 modules byte-identically from the [general-function packet](../function_divisor_finiteness_audit_20261115/README.md) and add8. All56 prior pins and39 copy equalities were checked before new formal edits; the42 formal/config pins were checked before copying, and all56 after copying. Eleven retained build states include six failures01/02/04/06/08/10 and five successes03/05/07/09/11. Final11 project logs contain no warnings/errors; the frozen axiom auditor's one inherited `String.mk` deprecation is retained. Ordinary kernel proofs use only standard Lean foundations, default limits, no holes/custom claim axioms/native proof evaluation/global elaboration flags/waivers. [pcc:unverified]

The original build01 lakefile was not snapshotted before its default-target correction. Its initial contents are reconstructed from the retained02 config and the single recorded target-name edit, explicitly labeled reconstructed; build01 source/log/exit are actual retained files. Setup exit0 and pin-check outputs were observed in the execution tool; their summary receipt is recorded afterward, not presented as a contemporaneous raw capture. [pcc:unverified]

Primary source: [Galbraith corrected chapter7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf), Lemma7.7.12, PDF16/printed138; its full lemma/proof and adjacent elliptic degreezero proof context were freshly text-read. Cumulative chapter coverage is PDF1–2,7–10,13–16 text and PDF2,8,9,13,15,16 visual, credited to prior and fresh readings. The chapter has21 pages; whole-chapter reading is not claimed. Official AdjoinRoot/localization/DVR/norm APIs and existing QuadraticAlgebra conjugation are credited established methods. KB wrappers and Conductor returned127, with explicit local-source/primary fallback. No absence or novelty claim. [pcc:unverified]

Over arbitrary fields, these are actual nonsingular rational-point divisors; for an elliptic model over an algebraically closed field they include every geometric point. Separate Mathlib projective `Scheme.localRing`/`Divisor` API comparison, geometric degreezero/constant kernel/uniqueness, algebraic-closure base change, translations/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical Weil–Tate/MOV–FR and completion of the original14 remain open. The full-statement check is procedural; it does not establish compiler/parser or physical-host independence. Signed standing remains unverified; no canonical corpus promotion, new independent implementation, cloud spend or uploads. [pcc:unverified]

Exact local **PCC-20261115-421c18** replay PASS at 2026-11-15T08:05:52Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `258d85a21ec7758ca466ac494258c21182444349` and local receipt `824a024cc79a4bfcd978c50aa523d454714f4b2e` share tree `313f8cf79b64eb187e608e42349eb6543d87c288`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
