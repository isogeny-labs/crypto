# All finite characteristic-two trace and geometric torsion

Lean proves the source trace criterion and geometric two-torsion criterion
agree over every finite field of characteristic two, for every nonsingular
general Weierstrass model. No bound on field size or extension degree is
imposed. The exact source predicate `gcd(q+1−#E(Fq),q)≠1` is equivalent
to each of: a1=0, j=0, odd actual point count, actual rational two-torsion
cardinality one, and actual geometric two-torsion cardinality one.
[pcc:unverified]

The rational two-torsion subgroup has cardinality one if j=0 and two
otherwise. Its actual induced ZMod2 module has rank zero or one respectively.
Its cardinality equals that of the geometric two-torsion subgroup over every
algebraically closed scalar extension. The source trace predicate is invariant
under every finite field extension. The preceding geometric j criterion,
actual point group and module construction are credited and reused.
[pcc:unverified]

The proof derives the missing finite-field bridge. If a1≠0, a nonzero
point killed by two has x=−a3/a1. Its remaining equation is a square-root
equation. The official finite reduced-ring theorem `isSquare_of_charTwo'`
supplies the ordinate. Actual Cauchy and Lagrange then show that the actual
group cardinality is even iff a1≠0. `FiniteField.card` gives q=2^n with
n>0, and a proved coprimality/parity lemma connects the source gcd criterion
to odd point count. No Hasse, trace-square classification, endomorphism
rank, geometric equivalence or rational-torsion existence premise is assumed.
[pcc:unverified]

Two negative controls check the hypotheses:

- Over F2(X), the actual curve y²+xy=x³+X has nonzero discriminant X,
  a1=1 and nonzero j. Its rational two-torsion has cardinality one,
  while its geometric two-torsion has cardinality two. X is not a square:
  a rational-function square has even integer degree, whereas X has degree
  one. Lean directly refutes ordinary rational two-torsion descent after
  deleting finiteness. This control concerns an imperfect field.
- The earlier F3 curve y²=x³+2x²+1 has five actual rational points and
  rational three-torsion cardinality one, yet its source trace predicate
  is false. Lean directly refutes changing characteristic two to three
  in the rational-torsion/source criterion. The model, point count and
  torsion classification are credited to the preceding packet.

These are hypothesis controls, not refutations of a qualified finite-field
source theorem. The general characteristic-three trace/geometric bridge
remains open beyond previously checked fields.
[pcc:unverified]

The main declaration is
`FiniteCharTwoTraceAudit.finite_char_two_trace_certificate`, in
[FiniteCharTwoCertificate.lean](lean/FiniteCharTwoCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact statements and scope.
Exact local **PCC-20261114-8d2918** replay PASS at 2026-11-14T06:48:42Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/finite_char_two_trace_audit_20261114/lean
python3 research/finite_char_two_trace_audit_20261114/restore_attempts.py /tmp/finite-char2-builds
```

Sources checked: FST2010, selected pp230/237, Definition2.1/Remark2.2/Section3,
[publisher PDF](https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf);
Milne, selected Remark3.16/context, printedpp98–99 and p75 endomorphism
notes, [author PDF](https://www.jmilne.org/math/Books/EC2.pdf), with the
preceding PDF104/80 inspections credited. Official Mathlib finite-field
cardinality, finite reduced-ring square theorem, rational-function degree
and j identities were read; preceding actual two-torsion geometry, Cauchy,
module and geometric j proofs were reused. No full-paper/book reading,
corpus promotion or novelty claim. KB and Conductor were unavailable;
the bounded fallback preceded implementation.
[pcc:unverified]

Failed source/configuration/log/exit states are retained, including an
incompatible modern-module import attempt. One unsuppressed compatibility
advisory remains for the legacy file importing modern Mathlib RatFunc.Degree;
frozen legacy dependencies were preserved. No linter waiver or elaboration
flag was added. General characteristic-three/other-prime trace-geometric
and endomorphism-rank identifications, Hasse/embedding bounds and the
remaining fourteen-item obligations stay open. No holes, custom axioms,
native evaluator, raised limits, new native experiment, paid compute or
agent upload.
[pcc:unverified]
