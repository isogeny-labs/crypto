# Actual finite-field Frobenius descent and BK's exclusion

Lean proves that the actual q-Frobenius on curve points fixes P exactly
when P is the base change of a point over the finite base field F, where
q=|F|. This holds for every finite field, every field extension L/F, and
every general Weierstrass model's nonsingular point group. Algebraic
closure, an assumed torsion basis and an assumed rationality criterion
are not premises. [pcc:unverified]

The actual additive homomorphism is Mathlib Point.map applied to
FiniteField.frobeniusAlgHom. It acts on affine coordinates by raising
them to q. Lean also proves preservation of r-torsion for every natural r
and the actual set criterion: all points killed by r lie in the rational
base-change image iff Frobenius fixes every point killed by r.
[pcc:unverified]

## Proof and actual objects

The polynomial X^q−X over F has exactly the q base-field roots and splits.
Mathlib's checked root-map lemma implies that any root in L is in the
range of algebraMap F L. Thus a scalar is fixed by q-Frobenius iff it
comes from F. An actual point is either zero or an actual nonsingular
affine pair. In the affine case, descend both coordinates and use the
actual baseChange_nonsingular equivalence to construct a point of W.
The reverse direction uses that the F-algebra homomorphism fixes every
base scalar. Torsion stability follows from the actual additive map;
the set criterion applies this point theorem to every r-torsion point.
[pcc:unverified]

No nonzero-discriminant premise is needed for this descent statement:
Mathlib's Point type already consists of nonsingular points and zero.
It applies in particular to every nonsingular elliptic curve. It is an
actual point-group statement, without a group-scheme interpretation of
characteristic torsion. [pcc:unverified]

## Exact excluded-case disproof

The existing actual curve E/F7:y²=x³+3x+2 has nonzero discriminant and
#E(F7)=9. The previously checked algebraic-closure construction supplies
an actual point P killed by3 outside the rational-point image. The new
descent theorem shows that actual 7-Frobenius does not fix P. Lean directly
negates the statement that every actual3-torsion point is fixed, although
3|7−1 and9|#E. Prime3 and coprimality with7 are explicit in the combined
certificate. [pcc:unverified]

This refutes dropping BK's exclusion, including the proposed repair that
r² divides #E. The original BK1998 Theorem1 already imposes r not dividing
q−1 and already warns that r² divisibility does not repair its removal.
The corpus node THM-XT-BK-WEIL-SUFFICES retains that exclusion. Neither
the original qualified theorem nor that qualified node is refuted.
[pcc:unverified]

## Proof map

| Declaration in FrobeniusPointDescentAudit | Exact obligation |
|:--|:--|
| finite_field_split | Actual polynomial X^q−X splits, using its complete finite-field root multiset and actual degree |
| scalar_frobenius_fixed_iff | Actual q-Frobenius-fixed scalars iff algebraMap image, every extension |
| pointFrobenius | Actual Mathlib additive map on W.baseChange L points |
| point_frobenius_fixed_iff | Actual point fixed iff actual W.Point base-change witness |
| point_frobenius_preserves_torsion | Every actual point killed by r maps to one killed by r |
| torsion_rational_iff_frobenius_identity | Actual r-torsion set lies in the base image iff Frobenius is identity there |
| bk_nonfixed_control | Nonsingular actual F7 curve,3|q−1,9|#E and actual nonfixed3-torsion |
| bk_full_torsion_fixed_false | Direct universal negation on that actual torsion set |
| frobenius_descent_audit_certificate | Fully quantified positive bridges and the actual excluded-case disproof |

The source uses only propext, Classical.choice and Quot.sound. A narrowly
scoped elaboration transparency option unfolds the actual self-base-change
identity when Lean matches point constructors. The kernel and transitive
axiom audit remain unchanged. There is no hole, custom axiom, native
decision exception or linter waiver. Failed attempts and diagnostics are
preserved losslessly. Toolchain Lean4.35.0/Mathlib56a00585 is pinned;
fresh lean4checker is unavailable. [pcc:unverified]

## Sources and remaining obligations

Re-read original BK1998 Theorem1 and proof/Remark1, printed pp142–143,
[DOI](https://doi.org/10.1007/s001459900040). The retained primary PDF has
SHA256 ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Reviewed Mathlib Finite.Basic's Frobenius/root lemmas, Polynomial.Splits's
actual root-image theorem, Affine.Basic baseChange_nonsingular, and
Affine.Point map/baseChange. The actual excluded-case construction and
triangular core were read in TorsionBoundaries/FrobeniusCore. KB and
Conductor tools remain unavailable; bounded repository/primary/Mathlib
fallback, no novelty claim or blanket unread-backlog promotion.
[pcc:unverified]

The qualified BK theorem still needs the actual geometric r-torsion
rank-two theorem, determinant q, Weil-pairing necessity, and a bound
between q-Frobenius iterates and the chosen degree-k extension. This
component closes actual descent over any given finite base field; it
does not supply those remaining bridges or the all-p<500 census.
Original requirements stay in the README acceptance map. [pcc:unverified]

Exact sources, statements, gate and actual packet standing are recorded
in frobenius-point-descent-sources.json and the separate verification
record. There is no new native experiment, timing or paid compute.
Leaderboard/benchmark/ecdb graphs have no new measured values; the new
proof-dependency diagram accompanies this result. Signed PCC standing
remains unverified until a trusted signed verdict. [pcc:unverified]

Local source commit `12cb97ae` precedes packet `PCC-20261110-f1abe2`.
Full producer-local verification passes at05:30:15UTC on762f5fc120c9;
signed standing remains unverified. Whole census gate479passes with zero
failures/zero trusted-native exceptions. See [actual bindings](frobenius-point-descent-verification.json),
[inspected two-page report](reports/proof-20261110-frobenius-descent/report.pdf),
and [editable report](reports/proof-20261110-frobenius-descent/report.md).
[pcc:unverified]
