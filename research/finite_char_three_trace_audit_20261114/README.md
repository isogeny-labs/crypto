# Every finite characteristic-three field: trace and geometric torsion

Lean proves the exact source trace criterion agrees with the geometric
three-torsion criterion over every finite field of characteristic three,
for every nonsingular general Weierstrass model. No field-size or
extension-degree bound is imposed. The source predicate
`gcd(q+1−#E(Fq),q)≠1` is equivalent to: three dividing the actual trace,
actual point count congruent to one modulo three, b2=0, j=0, actual
geometric three-torsion cardinality one, and actual geometric ZMod3
module rank zero. The source predicate is invariant under every finite
field extension. [pcc:unverified]

The new bridge uses the preceding actual quadratic-twist count theorem.
For y²=x³+a·x²+b·x+c, if a≠0, finite-field cube-root surjectivity gives
an x-coordinate satisfying the actual third division polynomial.
Complementary square-root fibers give an ordinate on the curve or its
nonsquare-scaled partner. Thus one partner has an actual nonzero point
killed by three. Cauchy/Lagrange and source invariance under twisting
exclude the source supersingular predicate in this branch.
[pcc:unverified]

If a=0, the actual third-division-polynomial criterion excludes nonzero
three-torsion on both partners. Cauchy shows neither point count is
divisible by three. Their actual sum is 2(q+1), with q a positive power
of three, so both counts are one modulo three. Coprime-power arithmetic
then proves the source predicate. No Hasse, trace-square classification,
endomorphism rank, torsion basis or rational-torsion-existence premise is
assumed. The ordinary point is constructed on one of the actual paired
curves; it is not assumed to be rational on the original curve.
[pcc:unverified]

An explicit complete-square variable change with
r=0, s=−a1/2 and t=−a3/2 transfers this result to every general model.
It preserves discriminants, actual point groups and cardinalities,
and b2; the transformed x² coefficient equals the original b2 in
characteristic three. The preceding actual additive point equivalence
and geometric j/module criteria are credited and reused. The resulting
source-to-geometric coupling is the new unbounded finite-field result.
[pcc:unverified]

Explicit controls delimit the statement:

- The credited ordinary F3 curve y²=x³+2x²+1 has j=1, five actual points
  and rational three-torsion cardinality one. Its nonsquare d=2 partner
  has three actual points, rational three-torsion cardinality three,
  and an explicit point (1,1) of exact order three. Both source predicates
  are false. Lean negates replacing geometric torsion by rational torsion
  on the original curve.
- The credited F3 curve y²=x³+2x+1 has seven actual points and its
  partner has one. Both satisfy the source predicate; their actual
  traces are −3 and +3. Lean directly refutes the universal strengthening
  to trace zero in characteristic three. The source's p≥5 prime-field
  qualification is retained.

The older F3 models and their point counts are credited. These are
hypothesis/strengthening controls, not refutations of the qualified
embedding-degree theorem. [pcc:unverified]

The main declaration is
`FiniteCharThreeTraceAudit.finite_char_three_trace_certificate`, in
[FiniteCharThreeCertificate.lean](lean/FiniteCharThreeCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact statements and scope.
Exact local **PCC-20261114-3a8304** replay PASS at 2026-11-14T08:01:36Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/finite_char_three_trace_audit_20261114/lean
python3 research/finite_char_three_trace_audit_20261114/restore_attempts.py /tmp/finite-char3-builds
```

Sources: [FST2010](https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf),
selected pp230/237 source trace predicate and qualifications;
[Galbraith author chapter9](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf),
selected Exercise9.5.4 count context, with the preceding inspected PDF7
and actual cubic-with-x² extension credited;
[Milne](https://www.jmilne.org/math/Books/EC2.pdf), selected Remark3.16/context
pp98–99 and p75 notes distinguishing geometric p-rank from endomorphism
rank. The false literal nonsquare/non-isomorphism assertion audited in
the preceding packet is not a premise. Official finite-field Frobenius,
positive cardinality-power and coprime-power APIs, prior actual division
polynomial/group/Cauchy/geometric j proofs, and the actual twist-count
certificate were reviewed. KB and Conductor were unavailable; bounded
fallback preceded implementation. No novelty, full-book/paper reading
or corpus promotion is claimed. [pcc:unverified]

This closes the source/geometric three-torsion bridge for all finite
characteristic-three fields. General p≥5 trace/geometric identifications,
endomorphism-rank equivalences, Hasse/embedding bounds, full torsion-field
interpretations and other original fourteen-item obligations remain open.
Failed source/configuration/log/exit states are retained. No holes, custom
axioms, native evaluator, raised limits, new global elaboration flags,
linter waivers, new native experiment, paid compute or agent upload.
[pcc:unverified]
