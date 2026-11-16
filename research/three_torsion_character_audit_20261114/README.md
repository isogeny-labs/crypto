# An actual cubic character proves Frobenius det=q at prime three

Lean proves the actual Frobenius determinant formula for every nonsingular
model **y²+a1xy+a3y=x³** over every finite field of characteristic other
than three, including two. It proves the qualified BK torsion-containment
criterion for every chosen finite extension and compatible embedding,
with q≠1mod3. An actual basis is constructed; no rank, determinant,
pairing or abstract rationality predicate is assumed. The whole gate passes
**77 declarations, zero failures, zero trusted native exceptions**.
Exact local **PCC-20261114-efe7ee** replay PASS at2026-11-14T04:16:34Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

For an actual point Q=(x,y) outside O,P,−P, where P=(0,0), the character is
**w(Q)=−(2y+a1x+a3)/(y+a1x+2a3)**. Literal Mathlib Ψ3, the curve equation
and actual nonsingularity prove the denominator is nonzero and the value
is a primitive cube root. Actual addition of P preserves it and negation
inverts it. Give O,P,−P the value1; these are precisely the kernel.
No division by two is used. Nonzero a3 is derived from nonzero
discriminant, not added to the selected family theorem's premises.
[pcc:unverified]

The preceding [general rank proof](../three_torsion_rank_audit_20261114/README.md)
supplies the actual geometric three-torsion module and basis. An actual
linear equivalence changes that basis to put P first. The character of
any torsion point is the second basis vector's primitive cube root raised
to its actual second coordinate. Actual point Frobenius fixes P and raises
character values to the q-th power. Primitive-root exponent injectivity
forces the second diagonal entry to qmod3; the first column is(1,0).
This proves det=q in the constructed basis, then every basis. Existing
[representation/descent](../bk_frobenius_bridge_audit_20261114/README.md)
and [qualified linear step](../bk_linear_audit_20261113/README.md) give
the all-chosen-extension criterion. Earlier F5/F7/F25 cases and short-only
three-torsion count are credited, not presented as new. [pcc:unverified]

The genuine characteristic-three model **y²+xy+y=x³** has discriminant1,
exactly three geometric three-torsion points and actual module rank1.
An explicit actual subgroup/OptionBool equivalence proves finiteness/count.
All three points already lie in every compatible extension's actual
point-map image, while3does not divide3^k−1 for positivek. Lean directly
negates the unrestricted inclusion iff after deleting coprimality.
**Original BK's nine-point condition remains unrefuted**: there are three
points here, not nine. The corpus inclusion paraphrase already retains
gcd(r,q)=1. [pcc:unverified]

Singular controls distinguish an equation point from an actual nonsingular
group point: b0 makes the origin singular; a1=3,b1 gives a nonsingular
origin and nonzero b while the curve's discriminant is zero. These are
controls of different premises. [pcc:unverified]

The selected declaration is
`ThreeTorsionCharacterAudit.actual_three_character_certificate`, in
[ThreeCharacterCertificate.lean](lean/ThreeCharacterCertificate.lean).
The [proof map](proof-map.json) and [source plan](sources.json) bind its
full1,787-character type (SHA3a07bd947154ec4b8873abb32288870bdb9c0ffb3e00c39b88a586f09d0bdf80),
all28 recursive custom modules and18 configs across six projects,
plus16 artifacts. [pcc:unverified]

```sh
tools/lean/proof-gate.sh research/three_torsion_character_audit_20261114/lean
python3 research/three_torsion_character_audit_20261114/restore_attempts.py /tmp/three-character-builds
```

All209 build-provenance files roundtrip through62 unique objects in the
64,558B archive, below the artifact policy's64KiB threshold. Rejected
sources/logs/exits and five whole-gate records remain complete. The
one-page [PDF](reports/report.pdf)/[SVG](reports/character-determinant.svg)
were fully visually inspected. Scalar **IDC1h3cd3ab88de4dfcba**, four
variables/degree4, has a universal Lean identity and the unchanged official
checker’s32+32-point acceptance. Same producer/host; it does not certify
the geometry or supply independent-host/signed verification. [pcc:unverified]

[Source review](evidence/source-review.json) retains exact primary URLs,
hashes, sections and prior-work credit: BK1998 Theorem1/proof/Remarks
pp142–143; Dujella author-hosted section15.3,p494 normal-form coordinates;
Elkies HarvardMath2232024 III.8 tangent functions/convention warning;
Snowden Michigan2013Lecture2 Weil/determinant passage. Dujella's discussion
is over Q; an exact full-torsion classification is not imported from its
shorthand. Selected sections only, no full-book/paper read or novelty claim.
[pcc:unverified]

**Remaining scope:** actual coordinate/group/Frobenius transport from an
arbitrary curve with a rational point of order three to this normal form;
generic determinant outside this family; canonical Weil identification;
other primes and the full all-k p<500census. The original qualified BK
theorem and the broader all-fourteen request are not declared complete.
Frozen corpus labels are unchanged. Pinned Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679,
only propext/Classical.choice/Quot.sound; no holes/custom axioms/native
evaluation/raised limits/new global flags/waivers. KB/Conductor unavailable,
bounded primary/repository/library fallback. No new native experiment,
paid compute or agent storage upload. [pcc:unverified]
