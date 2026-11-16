# A growing family of exact reduced-pair cycles

Lean proves a uniform arithmetic family over F3: for every t≥0, put
g=12t+2, n=24t+5 and f=Xⁿ+1. The actual polynomial is monic, separable
and has degree 2g+1. Its complete finite set of valid reduced Mumford
pairs contains an n-element cycle under normalized addition by (X,1).
Every cycle pair is smooth, while the full reduced-pair set contains an
explicit nonsmooth pair. Their smoothness proportions differ for every t.
Both genus and period grow without bound. [pcc:unverified]

The step is defined by the unique output of polynomial composition and
zero or one normalized reduction, before proving its agreement with the
indexed successor. Return after j steps holds exactly when n divides j.
This is an actual polynomial arithmetic result, with no assumed group
table or assumed geometric Jacobian correspondence. [pcc:unverified]

## Objects and exact certificates

The ambient universe consists of **all** pairs (u,v) over F3 satisfying
u monic, deg(u)≤g, deg(v)<deg(u), and u dividing f−v². Finiteness follows
from an injective bounded coefficient encoding, not from declaring an
enumerated list complete. Smooth means u splits over F3. [pcc:unverified]

The cycle consists of (1,0) and (Xᵈ,±1), 1≤d≤g. These are n distinct
valid pairs, all smooth. The pair (X²+X+2,X) is valid for every t and
nonsmooth: its quadratic has no F3 root. Uniform divisibility follows
from explicit quotient identities modulo X²+X+2 and n=8(3t)+5.
The positive-degree, high-degree crossing, negative-degree, cancellation
and identity phases each have full polynomial gcd/Bézout, composition,
remainder and reduction witnesses. [pcc:unverified]

| Module / namespace | Checked obligation |
|:--|:--|
| `ReducedPairs`, `MumfordFamilyPairsAudit` | Actual universe, injective coefficient encoding, finiteness, strict smooth count from a nonsmooth witness |
| `CurveFamily`, `MumfordCurveFamilyAudit` | Actual monicity, degree, separability; valid split powers and uniform nonsplit pair |
| `OrbitFamily`, `MumfordOrbitFamilyAudit` | n distinct actual pairs, all smooth, outside witness and unequal rational fractions |
| `NormalizedCantor`, `MumfordFamilyCantorAudit` | Full normalized gcd, bounded remainder, reduction and output uniqueness |
| `FamilyTransitions`, `MumfordFamilyTransitionsAudit` | Five exact normalized addition-by-(X,1) phases |
| `Successor`, `MumfordFamilySuccessorAudit` | Unique-output definition of actual arithmetic step; agreement with candidate indices |
| `Cycle`, `MumfordFamilyCycleAudit` | Injective residue positions, iterate conjugacy, return iff n divides j, no shorter positive period |
| `FamilyCertificate`, `MumfordFamilyCertificateAudit` | `growing_family_arithmetic_certificate`; unbounded genus/order; direct `universal_cycle_ambient_equality_false` |

## Scope and literature

[Stoll 2014](https://www.mathe2.uni-bayreuth.de/stoll/teaching/ArithHypKurven-SS2014/Skript-ArithHypCurves-pub-screen.pdf),
Lemmas 4.16–4.17 and Theorem 4.18, gives the reduced-pair/Picard
correspondence and addition algorithm; Example 4.19 gives the base
quintic cycle. These sections, printed pages 23–26, were reread. The
current Lean proof checks polynomial arithmetic independently. The
Riemann–Roch/Picard identification and a full all-pair ambient group law
remain separate formal obligations. Do not promote this cycle theorem
to an unconditional geometric Jacobian subgroup theorem. [pcc:unverified]

[Jędrzejak 2016](https://www.impan.pl/shop/publication/transaction/download/product/91624),
DOI 10.4064/aa8141-3-2016, already studies the torsion family
y²=x²ᵐ⁺¹+c². Its introduction, relevant theorem/model definitions,
good-reduction discussion and odd-degree torsion proof on printed
page 111 were read; this is not a full-paper read. No novelty claim.
Cantor's original 1987 paper remains unread after ordinary AMS primary
URLs returned HTTP403. Exact primary PDF hashes and read scopes are in
[the source review](evidence/source-review.json). KB/Conductor were
unavailable; bounded primary, repository and Mathlib reads supplied
the fallback. [pcc:unverified]

This disproves universal cycle/ambient smoothness equality in the stated
polynomial model. It does not refute Gaudry's fixed-genus, q→∞ estimate,
typical-subgroup heuristics or a large-prime-order asymptotic statement.
The periods 24t+5 are not all prime; no infinite prime-order family is
claimed. The earlier complete finite Stoll Cantor-group counterexample
remains in [the parent study](../jacobian_smoothness_audit_20261109/README.md).
[pcc:unverified]

## Verification

Lean 4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are
pinned. Only propext, Classical.choice and Quot.sound are allowed. No
holes, custom axioms, native_decide, linter waivers or added elaboration
settings. Failed sources and diagnostics are retained losslessly. No
new native experiment, timing, sampling or paid compute. [pcc:unverified]

The byte-identical existing [IDC1hb7102c15384f2648](stoll-identity-certificate.json)
is a base-quintic scalar quotient/remainder supplement only. It does not
certify this characteristic, variable exponent, period, counting or
geometry. The new polynomial/arithmetic conclusions rest on Lean.
[pcc:unverified]

```sh
cd research/mumford_family_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/mumford_family_audit_20261110/lean
```

The whole-project gate passes 91 declarations, zero failures and zero trusted-native
exceptions. All 39 attempt/source files and five full-gate files have byte-equal
archive roundtrip checks. Source pins, preissue source commit, actual packet
verdict and signed standing are recorded in sources.json and
verification.json. The [report](reports/proof-20261110/report.pdf) and
[editable proof diagram](reports/proof-20261110/proof.dot) map the arithmetic
result and the remaining geometric bridge. Existing IC performance graphs,
benchmark records and corpus statuses keep their prior evidence scope.
[pcc:unverified]

Source commit `9e23de04` precedes `PCC-20261110-156a3c`. Full local
verification passes at 13:09:19Z; signed standing remains unverified.
The actual verdict and all matching source/artifact pins are in
[verification.json](verification.json). [pcc:unverified]
