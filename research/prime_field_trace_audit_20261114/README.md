# Actual prime-field trace and composite-order audit

Lean proves the Hasse inequalities and source trace-zero criteria for every
nonsingular general Weierstrass coefficient model over F5 and F7. The
source predicate is exactly `gcd(q+1−#E(Fq),q)≠1`. Its actual point count
is six over F5 and eight over F7. These are complete all-model results,
obtained by a proved normalization followed by closed kernel tables over
all short coefficient pairs; they are not random samples.
[pcc:unverified]

The explicit coordinate change uses s=−a1/2,
r=−(a2−s·a1−s²)/3, t=−(a3+r·a1)/2. Over any field where two and three
are nonzero, the transformed a1,a2,a3 vanish. Discriminants agree.
The preceding [actual group equivalence](../bk_three_normalization_audit_20261114/lean/ShiftPointGroup.lean)
is reused: it preserves actual elliptic point addition, including the
identity, inverse, secant and tangent cases. Actual point cardinality,
trace and source-predicate transport follow. The official Mathlib short
normal form and Milne II2/Theorem2.1 are credited.
[pcc:unverified]

The closed tables cover all 25 short coefficient pairs over F5 and all 49
over F7. Under nonzero discriminant they prove actual Hasse and the
source predicate iff trace zero iff point count q+1. The generic
normalization transports these results to every general coefficient model;
no general Hasse or supersingular classification theorem is assumed.
Earlier native all-model censuses and conditional trace-zero/modular-order
lemmas remain credited and retain their own unsigned standing.
[pcc:unverified]

Every divisor r of a source-supersingular actual point count is coprime
to q and has modular order dividing two. If r>2, its modular order is
exactly two, including composite r. The generic modular proof needs only
r>2 and r dividing q+1. It does not assert that every point-count divisor
occurs as a cyclic point order or identify the full torsion-field degree.
[pcc:unverified]

The composite positive controls use actual named points:

- Over F5, y²=x³+1 has six actual points and P=(2,2) has exact order six.
  Its modular order modulo six is two; modulo two it is one. This reuses
  the same F5 curve as the earlier S3/r=2 controls and credits that model.
- Over F7, y²=x³+x has eight actual points and P=(3,3) has exact order eight.
  Its modular order modulo eight is two; modulo two it is one.

All proper positive multiples and the final zero multiple are checked
in the Lean kernel. Composite point orders are proved, rather than
inferred from group cardinality alone. These controls preserve the
source's p≥5 qualification; they are not qualified-source refutations.
[pcc:unverified]

A negative control checks the nonsingularity requirement. The F5 cusp
y²=x³ has discriminant zero, raw equation count plus identity six,
but actual nonsingular point-group cardinality five. Its actual source
trace predicate is false while the raw counted predicate is true.
Lean directly negates the universally quantified counting bridge after
deleting nonsingularity. This is a control on a hypothesis, not a
counterexample to an elliptic-curve theorem.
[pcc:unverified]

The main declaration is `PrimeFieldTraceAudit.prime_field_trace_certificate`
in [PrimeFieldTraceCertificate.lean](lean/PrimeFieldTraceCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain the exact statements and scope.
Exact local **PCC-20261114-e6d254** replay PASS at 2026-11-14T06:28:46Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/prime_field_trace_audit_20261114/lean
python3 research/prime_field_trace_audit_20261114/restore_attempts.py /tmp/prime-trace-builds
```

Primary review: FST2010, selected pp230/237, Definition2.1, Remark2.2 and
Section3, [publisher PDF](https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf);
Milne, *Elliptic Curves*, selected II1.5/1.6 and II2/Theorem2.1,
printedp50/PDF55, [author PDF](https://www.jmilne.org/math/Books/EC2.pdf).
Milne PDF55 was rendered and fully inspected. No full-paper/book reading
or corpus promotion is claimed. KB and Conductor were unavailable;
the bounded repository/primary/Mathlib fallback preceded implementation.
[pcc:unverified]

General prime/extension-field Hasse, geometric supersingularity/trace/
endomorphism-rank identifications beyond preceding bounded cohorts,
canonical field-degree/full-torsion-field bridges, other BK primes and
the other original fourteen-item gaps remain open. Failed builds and
their source/configuration/log/exit states are retained. The initial
dependency-setup path error is recorded separately. No holes, custom
axioms, native evaluator, raised limits, new global elaboration flags,
waivers, new native experiment, paid compute or agent upload.
[pcc:unverified]
