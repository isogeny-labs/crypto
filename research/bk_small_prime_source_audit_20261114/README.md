# Direct original BK count criteria at primes two and three

Lean proves the original Balasubramanian–Koblitz finite-field count
criterion at **r=2 and r=3** for arbitrary nonsingular general Weierstrass
models. Only the actual finite fields F,K, their algebra structure and
the extension cardinality q^k are supplied. The statement needs no
auxiliary algebraic closure, embedding into it, scalar tower, coprimality,
torsion rank, determinant, pairing or normal-form premise.
[pcc:unverified]

Keep the original hypotheses: r divides the actual rational point-group
cardinality, and r does not divide q−1. Then K has exactly r² actual
points killed by r, including the identity, iff r divides q^k−1.
The qualified source survives these small-prime checks. This is the
original count formulation; the inclusion paraphrase keeps its separate
coprimality condition. [pcc:unverified]

At three, construct Mathlib's canonical algebraic closure of the chosen
field K inside the proof. Its canonical F-algebra and scalar-tower
instances allow the preceding
[original prime-three theorem](../three_torsion_char3_audit_20261114/README.md)
to apply. That earlier proof establishes the general characteristic-three
torsion classification and handles characteristic three by showing both
sides false. The public finite-field statement now quantifies only F,K.
[pcc:unverified]

At two, the source condition 2 not dividing q−1 forces 2 dividing q.
Finite additive Cauchy and the field's absence of zero divisors imply
characteristic two. Reusing the preceding
[actual two-torsion geometry](../two_torsion_rank_audit_20261113/README.md),
the actual subgroup over every characteristic-two field has exactly one
or two points, is finite and cannot have four. If a nonzero point exists,
the existing uniqueness lemma supplies the two-point classification;
otherwise the subgroup is trivial. No algebraic-closure premise is
needed for this alternative. Cyclotomic divisibility is also false
for every positive extension degree; the chosen field cardinality
rules out degree zero. [pcc:unverified]

The rational point-count divisibility premise is retained in the public
prime-two theorem to match the source, although this case's proof does
not need it. Credited actual characteristic-two models with
discriminant1 realize both torsion counts. The ordinary model has an
actual point of order two, so its rational group cardinality is even
over every finite characteristic-two field. The source case is inhabited.
[pcc:unverified]

The main declaration is
`BKSmallPrimeSourceAudit.direct_small_prime_source_certificate`, in
[DirectSource.lean](lean/DirectSource.lean). The
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact statements and scope.
Exact local **PCC-20261114-e0dd93** replay PASS at 2026-11-14T05:13:24Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/bk_small_prime_source_audit_20261114/lean
python3 research/bk_small_prime_source_audit_20261114/restore_attempts.py /tmp/bk-small-prime-builds
```

All attempted sources/configurations/logs/exits remain in the lossless
provenance archive. The first closure-bridge attempt introduced a redundant
composed algebra that conflicted with canonical scalar instances; the
successful proof reuses the existing Mathlib instances. Frozen earlier
sources and artifacts are unchanged. [pcc:unverified]

Primary review: BK1998 Theorem1/proof/Remarks pp142–143,
[publisher PDF](https://link.springer.com/content/pdf/10.1007/s001459900040.pdf),
DOI10.1007/s001459900040. Prior two-/three-torsion geometry, actual
normalization, characteristic-three classification and source criterion
are credited. Mathlib canonical algebraic-closure and tower instances
were checked. KB and Conductor unavailable; bounded primary/repository/
library fallback recorded before implementation. Selected sections read,
not a full-paper/book reading claim. [pcc:unverified]

Other primes≥5, canonical Weil identification, full all-k p<500 census,
the formal supersingularity/trace classification and remaining fourteen-item
obligations stay open. No novelty, qualified-source refutation, corpus
promotion, native experiment, paid compute, upload, holes, custom axioms,
native evaluator, raised limits, new global elaboration flags or waivers.
[pcc:unverified]
