# Actual elliptic divisor classes and the point group

Lean proves the actual elliptic Abel–Jacobi group equivalence: on every
nonsingular general Weierstrass model over an algebraically closed field,
`P ↦ class((P) − (O))` bijects the actual point group with the actual
integer point divisors of degree zero modulo divisors of actual full-function-
field units. Every characteristic, including two and three, is covered.
The point group, local valuations, function field, divisors, quotient and
both directions of the correspondence are the actual defined objects.
[pcc:unverified]

It also proves the full principality criterion: a geometric point divisor is
principal exactly when its degree and its integer-weighted sum in the actual
point group are both zero. In particular, `(P) − (Q)` is principal exactly
when `P = Q`. No Picard equivalence, genus, Riemann–Roch, no-simple-pole,
normality or divisor-sum-zero premise is supplied. [pcc:unverified]

The missing injectivity step is derived in two stages. Weak Nullstellensatz
identifies every actual affine maximal ideal with the evaluation ideal of an
actual nonsingular point. An actual denominator ideal and the credited actual
finite local DVR integer lift then prove that a function regular at every
finite point belongs to the actual affine coordinate ring. Its actual infinity
order is the negative degree of its algebra norm. The official norm-degree-
not-one theorem excludes a lone simple pole at infinity and forces a principal
`(P) − (O)` to have `P = O`. This is an explicit norm argument, distinct from
the primary source's genus/differential proof. [pcc:unverified]

For addition and surjectivity, the already checked actual chord/vertical step
function has divisor `(P) + (Q) − (P+Q) − (O)`. It therefore supplies the group
homomorphism into the quotient. An actual finite integer linear combination
in the point group, with actual degree adjustment at the origin, proves
reduction of every divisor and surjectivity. The resulting `AddEquiv` is
constructed from the proved bijection. The principal-divisor criterion and
all principal point sums being zero follow from it. [pcc:unverified]

The controls check the domain. On the credited actual nonsingular curve
`y²=x³+x+1/F3`, the actual unit represented by `X²+1` has rational-point-only
divisor degree `−4`. Lean proves that the actual principal map cannot even be
restricted to degree-zero rational-point divisors with the same values.
Thus dropping geometric scope breaks the proposed quotient construction.
This reuses and credits the prior control, without claiming a new population.
Over the actual algebraic closure of F3, the credited mapped nonsingular curve
has the actual bijection and an actual nonzero divisor class. [pcc:unverified]

This closes the elliptic bridge for this geometric point-divisor quotient.
Comparison with a line-bundle or projective `Scheme` Picard API, quotient
basechange/descent, translations, Weil reciprocity, isogeny/Lang/Jacobian
bridges and canonical Weil–Tate/MOV–Frey–Rück remain separate obligations.
The original fourteen-item request remains unfinished. Qualified sources
survive; established methods and prior proofs are credited. [pcc:unverified]

Primary sources freshly checked: Galbraith corrected chapter7 PDF16–21,
printed138–143, Definition7.8.1 and Theorems7.9.7–9; full text for those pages
and individual visual inspection of PDF16,20,21. Fresh chapter8 PDF17–18/19,
printed161–162, Theorems8.6.1/8.6.4 and Corollary8.6.5, full text and individual
visual inspection. Whole-chapter reading is not claimed. Copyright sources
remain local. Official `Point.toClass` injectivity and its norm proof were
read and credited; the affine ideal class group was not silently identified
with the geometric quotient. See [source review](evidence/source-review.json).
[pcc:unverified]

The [closed certificate](lean/PicardCertificate.lean), [proof map](proof-map.json),
[source pins](sources.json), [protocol](PROTOCOL.md) and [report](reports/report.pdf)
record the exact statements and limits. Eight new concrete modules reuse76
frozen modules through one local dependency, with zero copies or edits.
All96 predecessor pins match before and after every one of13 states and the
gate. Six failures and seven successes are retained, including the default-
recursion failure corrected by exact rewriting. Final13 has zero source
warnings. The whole tracked/transitive gate passes35 theorems with zero
failures or trusted-native exemptions; the prior536-theorem gate is credited
separately. Only standard foundations are used. [pcc:unverified]

The complete1,056-character statement matches the native written-header
check with one complete result, zero prefixes, zero unresolved cases and an
empty omitted suffix, after the completed gate. The lossless archive preserves
226 files in83 objects and32,044 bytes, with exact roundtrip checks and five
gate files. A frozen auditor deprecation remains explicit. This is a procedural
statement check on the same host and prover; independent parser/compiler or
physical-host assurance is not established. [pcc:unverified]

```sh
cd research/elliptic_picard_bridge_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/elliptic_picard_bridge_audit_20261115/lean
python3 research/elliptic_picard_bridge_audit_20261115/export_binding_input.py --out /tmp/picard-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/picard-binding/manifest.json
python3 research/elliptic_picard_bridge_audit_20261115/restore_attempts.py /tmp/picard-attempts
```

Four correct KB wrapper calls and Conductor returned127 before processing/code;
bounded exact prior/official/primary fallback was used without an absence or
novelty claim. Default limits; no holes, custom axioms, native proof evaluation,
new global flags, linter waivers or frozen edits. Python handles documents and
provenance only. No new native cryptographic experiment, paid compute or agent
uploads. Signed standing remains unverified. [pcc:unverified]

Exact local **PCC-20261115-d00adb** replay PASS at 2026-11-15T19:25:02Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `4a0f66130aef98a90577beed077873f95714ca9c` and local receipt `d19dd8f140e9ca3857bbd50687f0821e9cd79fd1` share tree `c9094c7777d4e8a87b1d785a918d19c65c42ad6b`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
