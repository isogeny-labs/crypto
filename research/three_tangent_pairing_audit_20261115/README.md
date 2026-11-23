# Actual three-torsion tangent pairing and general Frobenius determinant

Lean proves an explicit alternating, bilinear, nondegenerate pairing on the
**actual geometric three-torsion** of every nonsingular general Weierstrass
curve in characteristic other than3, including2. Its actual field-map
covariance proves **det(Frobenius)=q mod3 in every basis**, without a rational
three-torsion point or point-count divisibility premise. [pcc:unverified]

The pairing is1 if either point is infinity or their x-coordinates agree.
Otherwise its convention is

\[
  e(P,Q)=-\frac{\ell_P(Q)}{\ell_Q(P)},
\]

where each actual tangent line has coefficient1 on y. Both evaluated tangents
are proved nonzero on this outside-x domain. Total field division is not
silently treated as a proof that evaluation is defined. [pcc:unverified]

The actual coordinate and group normalization at P preserves tangent values.
In the resulting model `y²+a1xy+a3y=x³`, the ratio equals the credited character
`−(2y+a1x+a3)/(y+a1x+2a3)`. Its actual basis-coordinate formula gives the second
bilinear law; the explicit ratio's skew symmetry gives the first. The cube law,
nonzero values, alternation and both zero radicals are proved on actual point
subgroups. A normalized nontrivial basis character detects every nonzero
first argument, and skew symmetry detects every nonzero second argument.
[pcc:unverified]

Actual coefficient point maps preserve the pairing, so actual q-Frobenius
raises its values to q-th powers. On any actual torsion basis the bilinear
coordinate formula raises the primitive basis value to the determinant.
Primitive-root exponent injectivity forces the actual matrix determinant to
q mod3. No determinant, rational fixed vector or special normal-form family
is supplied as a premise. The already proved actual geometric rank-two/basis
at3 is credited. [pcc:unverified]

For every compatible chosen finite field of size `q^k`, containment of all
actual geometric three-torsion therefore forces `3 ∣ q^k−1`. This necessity
uses no rational point-count divisibility premise. The preceding qualified BK
criterion at3 and its source qualifications remain separately credited.
[pcc:unverified]

Two actual domain failures and a positive control are retained:

- In characteristic3, `y²+xy+y=x³` is nonsingular and has a nonzero actual point
  killedby3 that pairs to1 with every actual point killedby3. Its previously
  proved kernel count3 and coordinates are credited. Deleting the characteristic
  guard makes nondegeneracy false.
- Restricting this alternating pairing to the actual cyclic submodule spanned
  by any nonzero three-torsion point yields a nonzero radical. Nondegeneracy on
  the full geometric kernel does not imply nondegeneracy on that subgroup.
- The credited actual curve `y²+y=x³+1` over the algebraic closure of F2 has an
  actual nontrivial cube-root pairing value. The curve/rank/basis are reused;
  the pairing property is new formal coverage. [pcc:unverified]

This is an explicit tangent pairing with proved properties. **Its comparison
with a canonical Weil-function/divisor definition remains open.** Other
primes≥5, general canonical pairings and the complete all-k p<500 census also
remain open. Established methods and earlier proofs are credited; no novelty,
qualified-source refutation or complete fourteen-item audit is claimed.
[pcc:unverified]

Fresh primary review covers Elkies's Harvard Math2232024 selected III.8
tangent-Weil-function and inverse-convention passages, and Galbraith ch26
PDF3/16, printed575, Theorem26.2.3/Exercise26.2.4/formula26.1 and its convention
footnote, with fulltext and individual-page visual review. Formula26.1 uses
disjoint divisors and is not silently identified with the pointwise ratio.
These are selected passages, not whole-course/book readings. Exact URLs,
hashes and prior proof credit are in [source review](evidence/source-review.json).
Copyright source files remain local. [pcc:unverified]

Four fresh correct KB wrapper calls and Conductor were unavailable with
exit127 before processing/code. Bounded repository, pinned-library and primary
fallback was used without an absence claim. Ten new concrete modules reuse33
frozen custom modules through one local path dependency, with zero copies or
edits. All65 predecessor pins are checked before and after every retained build
and gate. All13 exact states and nine failures remain available. [pcc:unverified]

The whole tracked/transitive gate passes43 theorems with zero failures and
zero trusted-native exemptions. The prior50-theorem gate is credited
separately. Final new-source warnings are zero;39 inherited advisory lines and
the frozen auditor's String.mk deprecation are retained. Current default
limits and standard foundations are used, without holes, custom axioms,
native proof evaluation, new global flags or waivers. [pcc:unverified]

See the [closed certificate](lean/TangentCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[protocol](PROTOCOL.md) and [one-page report](reports/report.pdf).

```sh
python3 research/three_tangent_pairing_audit_20261115/restore_attempts.py /tmp/three-tangent-attempts
```

Python handles source/provenance/documents only. No new native cryptographic
experiment, paid compute or agent experiment upload was used. Same-prover and
procedural written-header checks do not establish parser/compiler or physical
host independence. Signed standing remains unverified. [pcc:unverified]

Exact local **PCC-20261115-8c0b5f** replay PASS at 2026-11-15T17:46:05Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `c7aaf01d6b16ee861076eda3b521a1eb81444ba4` and local receipt `7c72171e7133e5ff6bad59e3d39d4e381ed249ef` share tree `c6f73a488320c71f39d92526e59d1bbbfdfc19a8`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
