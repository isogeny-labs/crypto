# Actual finite-extension points and Frobenius iterates

For actual finite fields F⊆K and any field extension B/K with compatible
scalar towers, Lean proves that a point of W over B is the image of an
actual point over K exactly when k iterations of F-Frobenius fix it,
where k is the actual extension degree [K:F]. The same equivalence holds
for the entire actual r-torsion set, for every natural r. W is any general
Weierstrass model over F; points and maps are Mathlib's actual objects.
[pcc:unverified]

The proof also checks |K|=|F|^[K:F]. If the finite extension is instead
specified by |K|=|F|^k, it derives [K:F]=k using |F|>1 and proves the
explicit k-iterate point and torsion criteria. This is the actual
finite-extension bridge used in the F_(q^k) notation. Neither point
rationality nor a torsion basis is assumed. [pcc:unverified]

## Structural proof

First generalize the checked finite-base point descent to a model whose
coefficients come from a smaller commutative ring R. A point is fixed by
the actual K-Frobenius iff an actual W(K) point maps to it. The affine
case descends both coordinates using the previous scalar theorem, then
uses actual baseChange_nonsingular to construct the source point. The
point at zero is handled separately. [pcc:unverified]

Second prove actual Point.map respects powers of an algebra endomorphism,
using its checked composition law and induction. Independently, the
actual scalar F-Frobenius raised to [K:F] equals K-Frobenius restricted
to F: both send x to x^|K|, by the actual finite-field cardinality theorem.
Applying the point-map power lemma makes K-Frobenius on actual points
equal to the required iterate of F-Frobenius. Actual K-point descent then
gives the result; applying it to every point killed by r gives the
torsion-set criterion. [pcc:unverified]

## Proof map

| Declaration in FrobeniusFiniteExtensionAudit | Exact obligation |
|:--|:--|
| frobeniusOver / frobeniusOver_fixed_iff | Actual larger-base Frobenius and actual point-image equivalence, even for a model over a smaller coefficient ring |
| point_map_pow | Actual Point.map of a homomorphism power equals iteration of actual Point.map |
| scalar_frobenius_power | Actual finite-field scalar Frobenius power equals restricted larger-base Frobenius |
| finrank_eq_of_card | Derive actual extension degree from the given field cardinality |
| frobeniusOver_eq_iterate | Actual point Frobenius equals the correct base-field iterate |
| finite_extension_fixed_iff | Actual K-rational image iff iterate fixedness, degree form |
| finite_extension_torsion_iff | Entire actual r-torsion set criterion, degree form |
| finite_extension_fixed_iff_of_card / finite_extension_torsion_iff_of_card | Same exact criteria in the explicit |K|=|F|^k form |
| finite_extension_audit_certificate | Fully quantified actual cardinality and both point/torsion formulations |

All field embeddings and their compatibility are explicit algebra and
scalar-tower data. The theorem applies to a chosen embedded finite
extension; it does not define rationality by a Frobenius predicate or
construct a canonical subfield inside a fixed algebraic closure. The
actual point type contains nonsingular points and zero, so a nonzero
discriminant assumption is unnecessary for this descent theorem. In
particular it applies to nonsingular elliptic curves. It describes
geometric torsion points, not nonreduced group schemes. [pcc:unverified]

## Skeptical scope and sources

The earlier [actual F7 control](FROBENIUS_DESCENT.md) remains unchanged:
E:y²=x³+3x+2 has9rational points and actual nonfixed3-torsion despite
3|q−1. It disproves dropping BK's exclusion, including the proposed
r²|#E repair. The qualified original theorem and corpus node retain the
exclusion. This new component proves a valid bridge and does not assert
a new counterexample to that qualified theorem. [pcc:unverified]

Reviewed original BK1998 Theorem1/proof/Remark1, pp142–143, DOI
10.1007/s001459900040; retained PDF SHA256
ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Reviewed Mathlib Affine.Point map_map, Affine.Basic nonsingularity descent,
AlgHom powers/restrictScalars, Finite.Basic Frobenius iteration,
FieldTheory.Finiteness Module.card_eq_pow_finrank, and the cardinality/
finrank argument in Finite.Extension. KB and Conductor remain unavailable;
bounded primary/repository/Mathlib fallback. No novelty or broad unread
source-status promotion. [pcc:unverified]

The qualified BK theorem still needs actual geometric r-torsion rank two,
determinant q and Weil-pairing necessity. The requested all-p<500 census
also remains unresolved. This component closes the iterate/point-image
bridge for a chosen actual finite extension, with exact hypotheses.
Earlier issued source files and source plans are unchanged.
[pcc:unverified]

The source uses only propext, Classical.choice and Quot.sound. No hole,
custom axiom, native-decision exception or linter waiver; this module
adds no elaboration option. Failed attempts/raw diagnostics are retained
losslessly. Lean4.35.0/Mathlib56a00585 are pinned; fresh lean4checker is
unavailable. Exact statement/source/gate and actual packet standing are
recorded in frobenius-finite-extension-sources.json and the separate
verification record. No native experiment, timing or paid compute.
Leaderboard, benchmark and ecdb graphs have no new measured value;
the accompanying proof-dependency diagram records this bridge. Signed
PCC standing remains unverified until a trusted signed verdict.
[pcc:unverified]

Local source commit `3c660a40` precedes packet `PCC-20261110-a496b9`.
Full producer-local verification passes at05:53:48UTC on762f5fc120c9;
signed standing remains unverified. Whole census gate490passes with zero
failures/zero trusted-native exceptions. See [actual bindings](frobenius-finite-extension-verification.json),
[inspected two-page report](reports/proof-20261110-frobenius-extension/report.pdf),
and [editable report](reports/proof-20261110-frobenius-extension/report.md).
[pcc:unverified]
