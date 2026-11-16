# Characteristic-three torsion and the original BK nine-point criterion

Lean proves that every nonsingular general Weierstrass model over
**every field of characteristic three** has exactly **one or three**
actual points killed by three. The actual subgroup is finite and its
induced ZMod3 module has rank zero or one. No algebraic-closure, torsion-rank
or normal-form premise is assumed. [pcc:unverified]

It also proves the original Balasubramanian–Koblitz nine-point criterion
at prime three, without adding a coprimality premise: for every finite
base field, every nonsingular model with 3 dividing its actual rational
point-group cardinality and 3 not dividing q−1, every chosen compatible
finite extension of cardinality q^k has exactly nine points killed by
three iff 3 divides q^k−1. The identity is included, as in the source's
group interpretation. The qualified source survives this check.
[pcc:unverified]

The distinction matters: in characteristic three, the original nine-point
condition and cyclotomic divisibility are both false. Full inclusion of
a smaller geometric torsion subgroup can still hold. Thus the
coprimality-deleted inclusion counterexample from the earlier
[character packet](../three_torsion_character_audit_20261114/README.md)
remains valid and does not refute the original count formulation.
[pcc:unverified]

If a nonzero actual point killed by three exists, the preceding
[actual tangent normalization](../bk_three_normalization_audit_20261114/README.md)
and its point-group isomorphism reduce the problem to
y²+a*x*y+b*y=x³. In characteristic three, its discriminant is b³a³
and its literal three-division polynomial evaluates to a²x³.
Nonsingularity forces a,b nonzero; the polynomial forces x=0 and
the curve equation gives y=0 or −b. An explicit actual torsion-subgroup
bijection with Option Bool proves the count three. If no nonzero
killed-three point exists, the actual subgroup is trivial and has
count one. Actual finiteness and module rank follow from those counts.
[pcc:unverified]

For the source criterion, outside characteristic three, finite additive
Cauchy and the field's absence of zero divisors derive gcd(3,q)=1
from 3≠0, allowing the preceding qualified all-model theorem.
In characteristic three, the new classification excludes nine points;
the additive order of 1 gives 3 dividing q, so it cannot divide q^k−1
for k>0. A chosen extension field of cardinality q^k rules out k=0.
The selected source theorem therefore needs no extra characteristic
or coprimality hypothesis. [pcc:unverified]

Credited actual algebraic-closure controls realize both branches:
y²=x³−x has nonzero discriminant and one point killed by three;
y²+xy+y=x³ has discriminant1, three such points and module rank1.
The older short-curve and a=b=1 proofs are baselines, not new results.
This packet proves the general arbitrary-model classification and
original-source bridge. [pcc:unverified]

The main declaration is
`ThreeTorsionCharThreeAudit.original_three_source_certificate`, in
[SourceThreeCriterion.lean](lean/SourceThreeCriterion.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact hypotheses, controls
and provenance. Exact local **PCC-20261114-72d986** replay PASS at 2026-11-14T05:02:48Z; signed standing
remains unverified. See [verification receipt](verification.json). [pcc:unverified]

```sh
tools/lean/proof-gate.sh research/three_torsion_char3_audit_20261114/lean
python3 research/three_torsion_char3_audit_20261114/restore_attempts.py /tmp/three-char3-builds
```

All attempted sources, logs and exit codes are retained. The initial
NormalFormCharThree01 configuration snapshot lacks the manifest because
the dependency update was still finishing; that gap is recorded rather
than repaired retrospectively. Both commands exited0; subsequent changed
source builds retain complete configurations. No incomplete snapshot is
presented as a full replay receipt. [pcc:unverified]

Primary review: BK1998 Theorem1/proof/Remarks pp142–143,
[publisher PDF](https://link.springer.com/content/pdf/10.1007/s001459900040.pdf),
DOI10.1007/s001459900040; Dujella section15.3/printedp494,
[author excerpt](https://web.math.pmf.unizg.hr/~duje/pdf/section15-3.pdf),
normalization discussed over Q. Existing general rank/point-criterion,
normalization, character and characteristic-three controls are credited.
Selected sections read, not full-paper or full-book reading. KB and
Conductor unavailable; bounded repository/primary/Mathlib fallback
recorded before implementation. [pcc:unverified]

Other primes≥5, canonical Weil identification, the full all-k p<500
census and remaining fourteen-item obligations stay open. This does not
identify the two characteristic-three branches with a formal definition
of supersingularity or prove the Hasse/trace classification. No novelty,
qualified-source refutation, corpus promotion, native experiment, paid
compute, upload, holes, custom axioms, native evaluator, raised limits,
new global elaboration flags or waivers. [pcc:unverified]
