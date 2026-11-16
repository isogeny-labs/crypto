# Actual elliptic function-field and divisor translations

Lean constructs the actual coefficient-fixing function-field automorphism
induced by translation through each actual point on every nonsingular general
Weierstrass model over an algebraically closed coefficient field. Every
characteristic, including two and three, is covered. Composition, the identity,
the explicit inverse through `−Q`, faithfulness and compatibility with the
actual translated generic point are derived. No transcendence, injective
coordinate substitution, inverse or abstract action premise is supplied.
[pcc:unverified]

The actual generic point has coordinates in the actual full elliptic function
field. Its x-coordinate is not constant, so the point and every constant
translate lie outside the actual constant-point image. Over algebraically
closed coefficients, an algebraic x-coordinate would force both coordinates
constant by the actual quadratic fiber equation. This derives transcendence
of every translated generic abscissa. The official actual norm-product
identity then proves the coordinate substitution injective, allowing the
actual fraction-field lift. [pcc:unverified]

The lift fixes actual coefficients and sends actual x and y to those of the
actual translated generic point. An explicit domain adapter has literally the
official `Point.map` as its body; constructor computation gives coordinate
compatibility. Actual point addition and actual coefficient/x/y/basis/fraction
generation derive the field composition and inverse laws, construct the actual
`RingEquiv`, and prove distinct points give distinct field maps. Canonical
localization algebra and scalar structures are retained. A transparent actual
field-ring projection and explicit point-map/addition interfaces resolve
implicit instance mismatches without changing global elaboration options.
[pcc:unverified]

Separately, actual point translation permutes finite integer point divisors.
Its pushforward preserves degree and changes the point sum by `degree(D) • Q`.
The proved actual elliptic principality criterion therefore gives preservation
of principal divisors. The actual induced map on the actual geometric Picard
quotient is the identity. A shifted principal divisor has an actual function,
but this existence proof does not identify that function with the constructed
field pullback. Pushforward through `Q` would correspond to field pullback
through `−Q`. Agreement with every local valuation and residue remains open.
[pcc:unverified]

The credited source example provides concrete negative and positive controls:

- On `y²=x³+x`, `Q=(0,0)`, the actual constructed field automorphism has the
  source formula `x ↦ 1/x`, `y ↦ −y/x²`. The actual reciprocal x has infinity
  order two, whereas a nonzero affine coordinate-ring function has infinity
  order equal to minus its norm degree. Lean proves reciprocal x is outside the actual affine
  coordinate-ring image. Over the actual algebraic closure of F5, with actual
  characteristic five and discriminant nonzero, the actual translation
  therefore fails to preserve that ring. A negation coordinate-ring
  automorphism cannot be generalized unchanged to every translation.
- On the actual F5 curve, the raw affine formula at Q produces a point different
  from the actual `Q+Q=O`. The ordinary input `P=(2,0)` has nonzero denominator,
  actual translate `(3,0)`, and the raw formula agrees there. The source
  explicitly excludes Q and O from that affine formula. Its qualified
  statement survives; the controls target deleted guards.

These are checks of the source's known example and established methods, without
a new population, novelty or qualified-source refutation claim. General
base-field descent, all-local-order/residue compatibility, projective Scheme
and line-bundle APIs, reciprocity, isogeny/Lang/Jacobian bridges and canonical
Weil–Tate/MOV–Frey–Rück remain separate. The full fourteen-item request remains
unfinished. [pcc:unverified]

Primary scope freshly checked: Galbraith corrected chapter9 PDF3–4/36,
printed167–168, Definition9.2.2 and Example9.2.4, full text and individual visual
inspection. Chapter7 PDF8–9, printed130–131, was read in full text for local-
valuation context; no new visual inspection there is claimed. No whole-chapter
reading. Copyright sources remain local and are excluded from the archive.
Official norm/minimal-polynomial/point-map/fraction-field APIs and the prior
actual Picard/divisor methods are credited. See [source review](evidence/source-review.json).
[pcc:unverified]

See the [closed certificate](lean/TranslationCertificate.lean),
[proof map](proof-map.json), [source pins](sources.json),
[protocol](PROTOCOL.md) and [report](reports/report.pdf).
Twelve new concrete modules reuse84 frozen modules through one local dependency,
with zero copies or edits. All107 predecessor pins match before and after24
states and the gate. Fourteen failures and ten successes are preserved,
including the duplicate scalar structure and imported implicit-ring projection
failures, never accepted as proofs. Final24 has zero source warnings.
[pcc:unverified]

The whole tracked/transitive gate passes82 theorems with zero failures or
trusted-native exemptions; the prior35-theorem gate is credited separately.
Only standard foundations are used. The complete1,910-character statement
matches with one complete result, zero prefixes, zero unresolved cases and an
empty omitted suffix, after the completed gate. Lossless provenance preserves
480 files in136 objects and40,900 bytes, exact roundtrip, plus five gate files.
One frozen auditor deprecation remains explicit. The statement check shares
the prover and host; independent parser/compiler or physical-host assurance
is not established. [pcc:unverified]

```sh
cd research/elliptic_translation_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/elliptic_translation_audit_20261115/lean
python3 research/elliptic_translation_audit_20261115/export_binding_input.py --out /tmp/translation-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/translation-binding/manifest.json
python3 research/elliptic_translation_audit_20261115/restore_attempts.py /tmp/translation-attempts
```

Four correct KB wrapper calls and Conductor returned127 before processing/code;
bounded exact prior/official/primary fallback was used without an absence
claim. Default limits, no holes, custom axioms, native proof evaluation, new
global flags, linter waivers or frozen edits. Python handles documents and
provenance only. No new native cryptographic experiment, paid compute or agent
uploads. Signed standing remains unverified. [pcc:unverified]

Exact local **PCC-20261115-2e952c** replay PASS at 2026-11-15T20:30:05Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `8a42665091a5020238a6c0b9a812c089ae848f51` and local receipt `5eadc716e1d0f9fc3aba40d4f9bf09edcd33bba8` share tree `6bf96ce8d8bc66058d2de347b7a924eead5a093c`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
