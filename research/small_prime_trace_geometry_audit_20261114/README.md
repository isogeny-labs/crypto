# All-model F2/F3 trace, geometry and modular-order audit

Lean checks every general Weierstrass coefficient model over F2 and F3
and connects the computed equations to Mathlib's actual point groups.
It proves the source trace predicate is equivalent to trivial geometric
p-torsion for every nonsingular model over these two fields, proves the
actual Hasse inequality there and supplies the trace-square classification
as a conclusion. No Hasse or trace-classification premise is supplied.
[pcc:unverified]

| Proved coefficient-model count | F2 | F3 |
|:--|--:|--:|
| All models | 32 | 243 |
| Nonsingular models | 16 | 162 |
| Models satisfying the source supersingularity predicate | 8 | 54 |
| Supersingular actual point counts | 1, 3, 5 | 1, 4, 7 |
| Modular order for every actual point-count divisor | divides4 | divides6 |

These are exhaustive counts of coefficient models, including isomorphic
duplicates, rather than samples or isomorphism-class counts. The older
native census's 16+162 nonsingular-model baseline is credited. The new
proofs use `decide +kernel` and actual group semantics; they do not promote
the older native execution's unsigned standing. [pcc:unverified]

The generic semantics theorem works over every finite field: the actual
nonsingular point group has cardinality equal to the number of literal
equation pairs plus the identity. It uses an actual point equivalence,
under nonzero discriminant. The source trace is therefore exactly the
counted integer trace. A coefficient-five-tuple equivalence proves that
the finite model enumeration covers every general model. Closed kernel
tables then prove the F2/F3 cases. [pcc:unverified]

The source predicate is `gcd(q+1−#E(Fq),q)≠1`. Over F2 it is equivalent
to a1=0; over F3 it is equivalent to b2=0. Combine the preceding
[actual geometric j criterion](../small_characteristic_j_audit_20261114/README.md)
to identify this predicate with the actual geometric subgroup killed by
p having cardinality one. The Hasse inequalities and trace-square cases
are proved by the closed finite tables, rather than assumed from the
general theorem. The preceding
[conditional arithmetic theorem](../theorem_census_20261109/lean/SupersingularTraceClasses.lean)
is credited; these new bounded results supply its missing classification
only for F2 and F3. [pcc:unverified]

Every divisor r of the actual supersingular point count, including
composite divisors, satisfies gcd(r,q)=1 and has modular order dividing
four over F2 or six over F3. This follows from the actual point-count
cases and verified root-of-unity powers. It does not assert that every
count divisor occurs as a cyclic point order. The general field-degree
identification and full torsion-field constructions remain separate.
[pcc:unverified]

Explicit positive controls realize the bounds:

- Over F2, y²+y=x³+x has five actual rational points. P=(0,0) has
  exact order5, geometric two-torsion has one point and orderOf(2mod5)=4.
- Over F3, y²=x³+2x+1 has seven actual rational points. P=(0,1) has
  exact order7, geometric three-torsion has one point and orderOf(3mod7)=6.

Both models have nonzero discriminant and satisfy the source trace
predicate. Their point orders follow from actual prime group cardinalities
and Lagrange. These examples lie outside the source's p≥5 prime-field
clause and are not counterexamples to that qualified clause.
[pcc:unverified]

The main declaration is
`SmallPrimeTraceGeometryAudit.small_prime_trace_geometry_certificate`, in
[SmallPrimeTraceCertificate.lean](lean/SmallPrimeTraceCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact statements and scope.
Exact local **PCC-20261114-6fec58** replay PASS at 2026-11-14T06:12:46Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/small_prime_trace_geometry_audit_20261114/lean
python3 research/small_prime_trace_geometry_audit_20261114/restore_attempts.py /tmp/small-trace-builds
```

Primary review: FST2010, pp230/237, Definition2.1, Remark2.2 and Section3,
[publisher PDF](https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf);
Milne, *Elliptic Curves*, selected IV9 point-count discussion and
Theorem9.4 proof, printedp150,
[author PDF](https://www.jmilne.org/math/Books/EC2.pdf).
FST PDFpages7/14 and Milne PDF155 were rendered and inspected. Selected
sections read, no full-paper/book reading claim. KB and Conductor
unavailable; bounded repository/primary/Mathlib fallback recorded before
implementation. Raw primary books remain in the local checkpoint.
[pcc:unverified]

General prime fields, extension fields, Hasse/endomorphism-rank/geometric
identifications beyond this bounded cohort, other BK primes, the full
all-k p<500 census and the remaining fourteen-item obligations stay open.
All failed source/configuration/log/exit snapshots remain in the lossless
archive. No qualified-source refutation, novelty, corpus promotion,
native experiment, paid compute, upload, holes, custom axioms, native
evaluator, raised limits, new global elaboration flags or waivers.
[pcc:unverified]
