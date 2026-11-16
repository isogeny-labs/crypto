# Actual local residue and guarded divisor evaluation

Lean constructs evaluation on actual regular-function rings at actual elliptic points, derives the local-unit/disjoint-support domain, and proves unit-valued divisor evaluation laws. Actual coordinate fractions have the expected numerical value. Equal geometric principal divisors give equal values on a disjoint degreezero divisor under the actual geometric hypotheses. Actual pole/zero/overlapping-support controls disprove deleting those guards. [pcc:unverified]

The main theorem is [`FinitePointEvaluationAudit.finite_point_evaluation_certificate`](lean/EvaluationCertificate.lean). See the [complete statement/source pins](sources.json), [proof map](proof-map.json), [proof report](reports/report.pdf), [source review](evidence/source-review.json) and [pre-edit protocol with dated refinements](PROTOCOL.md). [pcc:unverified]

The existing coordinate point evaluation lifts through the actual localization at its kernel. Constants, surjectivity, kernel=maximal ideal, the actual residueField≃K and local unit iff nonzero value are proved. The finite-point residue lift adapts the credited earlier infinity construction in new files only. For an actual nonsingular finite point, apply Mathlib's existing DVR integer-lift theorem to the previously proved actual point DVR and actual full function field. This gives the actual bijective comparison and RingEquiv with the official normalized adic valuation subring, including the intended field inclusion. No comparison or classification is assumed, and no new generic DVR method is claimed. [pcc:unverified]

Combine finite evaluation with the credited actual infinity-chart comparison/residue map. At every actual point, a regular function's value is nonzero iff its valuation is1. For a nonzero actual function, local order0 iff valuation1. Constants map identically. If the actual coordinate denominator has nonzero point value, its actual local image is a unit: its inverse constructs an actual regular representative of a/b with full-field coercion aFF/bFF and value pointEval(a)/pointEval(b). Actual point-unit evaluation agrees with this numerical formula. [pcc:unverified]

The official valuation-unit subgroup equivalence and Units.map of the actual residue map produce values in Kˣ. Their intersection over the support of D is the actual unit-at-support subgroup. Lean proves membership iff the support is disjoint from the actual principal-divisor support. Thus the source disjointness condition, rather than a supplied global evaluation premise, yields the domain for the finite product of actual point values raised to integer coefficients. The product is multiplicative in functions and additive in divisors on a common support domain. The common-support helper returns1 outside that set; interpreting it as actual divisor evaluation requires the explicit support-cover condition. The canonical evaluator uses exactly D.support. [pcc:unverified]

Actual constant values yield the factor c^degree(D), with the actual integer coefficient-sum degree. Degreezero gives scalar invariance. The prior actual geometric same-divisor scalar criterion then proves choice invariance for actual equal geometric principal divisors and disjoint degreezero D, under IsAlgClosed K and W.Δ≠0. This is the elliptic geometric forward direction of source Exercise26.1.1. Its converse, its all-curve version and Weil reciprocity are separate obligations. [pcc:unverified]

The skeptical controls use actual functions and local rings. Nonzero X−x would map0 under any total coefficient-fixing FunctionField→K ring homomorphism sending X to a finite point's x-coordinate, contradicting injectivity. Its inverse has no actual local representative and official finite adic value>1, while the actual local value of X−x is0. Singleton divisors with coefficients1 and−1 cancel; evaluating their zero value with totalized field negative powers makes the product0, whereas the zero divisor has value1. The deliberately totalized helper assigns1 outside the tested singleton; the control claims no arbitrary global evaluation there. [pcc:unverified]

On the credited actual nonsingular curve E/F3:y²=x³+x+1, P=(0,1) is nonsingular. The actual vertical has order1 at P and−2 at infinity. Its actual principal support and the singleton support are unequal but overlap. Chapter26's running text says “distinct”; Exercise26.1.1 explicitly says “disjoint.” The control refutes the merely-unequal reading or a deleted zero/pole guard, without refuting the intended qualification. [pcc:unverified]

Reproduce the source, every tracked theorem's transitive axioms, the complete written statement and historical attempts:

```sh
cd research/finite_point_evaluation_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/finite_point_evaluation_audit_20261115/lean
python3 research/finite_point_evaluation_audit_20261115/export_binding_input.py --out /tmp/point-evaluation-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/point-evaluation-binding/manifest.json
python3 research/finite_point_evaluation_audit_20261115/restore_attempts.py /tmp/point-evaluation-attempts
```

The native summary must say1complete/0prefix/0unresolved with the exact3887-character header/hash and empty omitted suffix. Exit0 alone is insufficient. SingletonAddFailure is a transparent positional abbreviation of the complete actual singleton counterexample, pinned with its source. Every logical guard remains; four main binders were made anonymous without waivers or flags. [pcc:unverified]

The76modules reuse67 modules byte-identically from the [actual basechange packet](../miller_basechange_audit_20261115/README.md), after all87 previous pins matched. Setup exit0 and the initial config were captured before formal edits. Every build snapshots every new source and actual config while checking old copies. Fourteen states retain failed02/07/08/09/11 and successful01/03/04/05/06/10/12/13/14. Failed compiler placeholders are diagnostics, not accepted proofs. All advisories remain, including06's deprecated name and13's four named guards; final14/project logs are clean. The frozen auditor's inherited String.mk deprecation is reported separately. [pcc:unverified]

Primary reading: [Galbraith corrected ch26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf), new full text+visual PDF1/printed573, with cumulative text+visual1–6, not all16pages. [Ch7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf) partial text lines1–72/PDF1–2/printed123–124 was reread within prior cumulative fulltext1–2,7–10,13–16 and visual2,8,9,13,15,16; not all21pages. Official DVR/unit-group/Finsupp APIs and prior actual local geometry/scalar criterion are credited. Five correct KB wrapper calls and two Conductor calls failed127 before processing/edits; bounded local/official/primary fallback is explicit, with no absence or novelty claim. Original protocol text remains in the append-only protocol alongside pre-code API/formula refinements; the budget stayed50states/12:00Z/local$0. [pcc:unverified]

Full Mathlib scheme/localRing/Divisor interoperability, finite-point divisor basechange/tensor/Galois descent, translation/Weil reciprocity/isogeny/Lang/Cartier/Jacobian/canonical Weil–Tate/MOV–FR and binary runtime remain separate. Same prover/Mathlib/host; statement processing is procedural, lean4checker is absent, signed standing unverified. No new independent implementation, canonical corpus promotion, paid compute or uploads. [pcc:unverified]

The first report draft spilled109characters onto a second page. Its original PDF, renderer and layout are retained in the [lossless draft archive](evidence/report-drafts.tar.xz) and [inventory](evidence/report-drafts.json). The final one-page report uses adjusted typography; no formal source or gate changed. [pcc:unverified]

Visual review also caught a missing “nonzero regular function” qualification in the second draft’s order-zero diagram. The prose and Lean theorem already retained it; the final diagram now matches that hypothesis. The second draft is preserved in the same lossless archive. No formal source or gate changed. [pcc:unverified]

Exact local **PCC-20261115-1a1570** replay PASS at 2026-11-15T11:01:06Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `44ba944398b9480b48d8eec905bc7976c0c37f7b` and local receipt `9ca1f86bd5c6039be5d4aa3b46da64af3d2a38a1` share tree `b720a248f30eb6aa507cb76d52a5bb5c0981cfea`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
