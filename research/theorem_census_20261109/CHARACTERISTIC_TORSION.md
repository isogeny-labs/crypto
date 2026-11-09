# Characteristic torsion: set inclusion loses the rank hypothesis

Lean checks a second boundary of the unqualified set-inclusion wording in
the request, on the actual
nonsingular curve E/F2:y²+xy=x³+1. The rational point group has4elements.
Its actual geometric2-torsion over the algebraic closure has exactly2points,
both rational, while2does not divide2^1−1. Thus the statement
E[r]⊂E(F_(q^k)) iff r divides q^k−1 fails here, even though r is prime,
r divides #E and r does not divide q−1. [pcc:unverified]

The primary BK1998 Theorem1 uses a different left side: the extension
contains r²torsion points. Its proof of sufficiency first observes that
divisibility forces r not to divide q. At characteristic torsion, replacing
the point-count condition by set inclusion changes the statement: geometric
E[2] has2points, rather than4. This example does not refute the original
BK theorem. The prime-to-characteristic hypothesis or a rank-two condition
must be retained when translating it into the set-inclusion formulation.
The actual corpus node already includes gcd(r,q)=1, so this is a faithful
excluded-case negative control, not a disproof of that qualified node.
The original r not dividing q−1 exclusion is independently essential by
the earlier actual F7/r3witness. [pcc:unverified]

Lean classifies every actual geometric point killed by2 as either infinity
or(0,1). Negation forces x=0, and the curve equation plus characteristic2
forces y=1. Both points are in the range of Mathlib's actual base-change
homomorphism from E(F2). This is not a custom finite operation table or an
assumed model of Frobenius. [pcc:unverified]

The same actual group supplies a pointwise pairing obstruction. The type
MuTwo={z in F2bar | z²=1} has cardinality1. The geometric2-torsion subtype
has cardinality2 and a proved nonzero point. For every function
e:E[2]×E[2]→MuTwo, Lean proves that left nondegeneracy is false, even
without assuming bilinearity. All values equal1, including the entire
row of the nonzero point. A universal pointwise nondegeneracy statement
that omits coprimality fails on this actual elliptic group. [pcc:unverified]

This does not construct the Weil or Tate pairing on their proper domains,
and does not refute the finite-flat group-scheme version of Weil duality.
The statement is explicitly about geometric points and root-of-unity
values. Full actual Weil/Tate/MOV/FR calculations and proofs remain open.
[pcc:unverified]

Key declarations in
[CharacteristicTorsionBoundary.lean](lean/CharacteristicTorsionBoundary.lean):

- `rational_curve_cardinality`: actual point group has4elements.
- `geometric_two_torsion_classification` and
  `geometric_two_torsion_cardinality`: actual geometric torsion has2points.
- `characteristic_torsion_field_counterexample`: exact hypotheses and
  failure of the set-inclusion restatement atq=r=2,k=1.
- `muTwo_cardinality`: actual root-of-unity value type has1element.
- `no_pointwise_nondegenerate_pairing`: every map on this actual group
  fails the explicit left-nondegeneracy predicate.
- `characteristic_boundary_certificate`: combined checked statement.

The actual algebraic closure, base change and nonsingular elliptic group
are Mathlib objects. All build diagnostics and the full transitive gate
receipt are retained. No new native experiment, paid compute, source-status
promotion or change to frozen population/protocol/data. The earlier116gate
snapshot and all prior mathematical statements remain intact.
[pcc:unverified]

Final expanded gate:134declarations,0failing,0trusted-native exceptions.
PCC-20261109-92d64d binds the combined actual-object certificate and remains
unsigned. Previous116gate is its earlier binary-proof snapshot.
[pcc:unverified]

Source: BK1998, Theorem1/preceding Conditions1–2 and proof, pp142–143,
DOI10.1007/s001459900040. Primary PDF SHA256
ac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499;
extracted text SHA256
3514497fbbeae88aba654b0e4b965db0b8f126de930b0cc63c67a0747370e04b.
The narrow source fallback is retained because the KB service is unavailable.
See [visual proof report](reports/proof-20261109-characteristic/report.pdf)
and the [all-fourteen status map](README.md#original-fourteen-requirements).
[pcc:unverified]
