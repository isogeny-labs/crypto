# The qualified BK criterion at three for arbitrary Weierstrass models

Lean proves the qualified Balasubramanian–Koblitz criterion at prime three
for **every nonsingular general Weierstrass model over every finite field**,
including characteristic two. [pcc:unverified]
Let q be the base-field cardinality. Assuming
gcd(3,q)=1, 3 divides the actual rational point-group cardinality, and
3 does not divide q−1, the following are equivalent for every chosen
finite extension of cardinality q^k with compatible embeddings:

- All actual geometric points killed by three lie in the extension image.
- Exactly nine extension points are killed by three, including the identity.
- 3 divides q^k−1.

No normal-form, determinant, torsion-rank, pairing or abstract rationality
premise is supplied. The qualified source statement survives this check
at three. Exact local **PCC-20261114-3f6943** replay PASS at 2026-11-14T04:47:08Z; signed standing
remains unverified. See [verification receipt](verification.json). [pcc:unverified]

The proof derives a rational nonzero point P killed by three from the actual
group cardinality using additive Cauchy. With P=(r,t) and its actual tangent
slope s, the change x_old=x+r, y_old=y+s*x+t gives coefficients
a2=a4=a6=0. The curve equation gives a6=0, the slope gives a4=0,
and the literal three-division polynomial gives a2=0. Nonsingularity
proves the tangent denominator nonzero. These are field proofs and do
not divide by two. [pcc:unverified]

The coordinate change is an actual additive equivalence of Mathlib curve
point groups, with an explicit inverse. Its proof handles identities,
inverse pairs, secants and tangents. It preserves the discriminant and
commutes with actual scalar-field maps, tower inclusions and point
Frobenius. For every integer n, it preserves killed-point cardinalities
and the geometric torsion-containment condition in the chosen extension.
These proved bridges transport the preceding
[normal-form character criterion](../three_torsion_character_audit_20261114/README.md).
Characteristic other than three is derived from gcd(3,q)=1 through
the additive order of 1, rather than introduced as a hidden premise.
[pcc:unverified]

The nine-point bridge uses the actual injective map on killed-three
subgroups and the preceding [general geometric rank/count](../three_torsion_rank_audit_20261114/README.md).
Its surjectivity is exactly actual containment; bijectivity is equivalent
to equal cardinality. This distinguishes the original source's count
from an inclusion paraphrase in characteristic three. [pcc:unverified]

The closed certificate retains credited controls: the nonsingular F7
curve has 3² dividing its rational group cardinality and 3 dividing q−1,
yet an actual geometric three-torsion point is not fixed by Frobenius.
The genuine ordinary characteristic-three curve y²+xy+y=x³ has
discriminant 1 and only three geometric three-torsion points; deleting
coprimality refutes the unrestricted inclusion paraphrase, while the
original nine-point condition stays false there. These controls do not
refute the qualified source theorem. [pcc:unverified]

The main declaration is
`BKThreeNormalizationAudit.actual_general_three_bk_certificate` in
[GeneralThreeBKCertificate.lean](lean/GeneralThreeBKCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) record exact hypotheses and scope.
All failed source snapshots, logs and exit codes are retained losslessly.

```sh
tools/lean/proof-gate.sh research/bk_three_normalization_audit_20261114/lean
python3 research/bk_three_normalization_audit_20261114/restore_attempts.py /tmp/bk-normalization-builds
```

Primary review: BK1998 Theorem 1, proof and Remarks, pp142–143,
[publisher PDF](https://link.springer.com/content/pdf/10.1007/s001459900040.pdf),
DOI10.1007/s001459900040; Dujella section15.3, printed p494,
[author excerpt](https://web.math.pmf.unizg.hr/~duje/pdf/section15-3.pdf).
The latter discusses normalization over Q; its characteristic extension
and actual point-group transport are proved here. Selected sections
read, not a full-book or full-paper reading claim. Existing short-curve
scaling, F5/F7/F25 cases, general rank, actual Frobenius/descent and cubic
character proofs are credited. Pinned Mathlib variable-change and actual
affine-point APIs were read before implementation. KB and Conductor were
unavailable; bounded repository/primary/library fallback is recorded.
[pcc:unverified]

Other primes ≥5, canonical Weil-pairing identification, the complete all-k
p<500 census and the remaining fourteen-item obligations stay open.
This packet does not prove determinant q for every arbitrary model without
a rational three-torsion point. No novelty, full-source-read, corpus
promotion or whole-audit completion claim is made. No native experiment,
paid compute, upload, holes, custom axioms, native evaluator, raised
limits, new global elaboration flags or waivers. [pcc:unverified]
