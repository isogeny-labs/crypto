# Actual quadratic-twist counts and a literal isomorphism counterexample

Lean proves the actual quadratic-twist point-count identity over every finite
odd-characteristic field for every nonsingular cubic model
y²=x³+a·x²+b·x+c. For a nonsquare d, the explicitly scaled model has
coefficients (d·a,d²·b,d³·c). Their actual point counts sum to 2(q+1),
their actual traces are negatives, and their source gcd(trace,q) predicates
agree. No non-isomorphism, Frobenius trace or point-count theorem is assumed.
[pcc:unverified]

The proof supplies actual point/equation-pair/root-fiber equivalences.
Nonzero square fibers have two roots, zero has one and nonsquares have none.
The official finite-field square criterion proves that the fiber for v and
the fiber for d³v have total cardinality two. Reindexing x by d·x gives
the count identity, including both identity points. Discriminants scale by
d⁶. This proves the short-model count statement of Galbraith Exercise9.5.4
and its explicit cubic-with-x² extension; it does not rely on a classification
of twists in characteristics two or three.
[pcc:unverified]

The same source's literal Exercise9.5.3 says the nonsquare scaling is not
isomorphic over the base field. Lean supplies a counterexample with all
its stated conditions met: over F7, E:y²=x³+x is nonsingular with j=1728,
d=3 is nonsquare, and its scaled model is E':y²=x³+2x. The projective map
(X:Y:Z)↦(4X:Y:Z) has inverse (X:Y:Z)↦(2X:Y:Z). The certificate proves
the homogeneous cubic and inverse-coordinate identities over every
commutative ring of characteristic seven, an actual admissible variable
change over F7, and the explicit actual point-group additive equivalence.
Both actual groups have eight points. This exhibits a trivial nonsquare
scaling in the special-j case. The source's separately qualified generic-j
classification, Lemma9.5.7, retains its explicit j exceptions.
[pcc:unverified]

The primary is the author's version2 chapter9, fetched from the author site,
SHA256 `f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495`.
Selected Definition9.5.1, Example9.5.2, Exercises9.5.3–9.5.4 and
Theorem9.3.4 were read; printedp171/PDF7 was rendered and fully inspected.
The author's landing/errata page was also checked, including its adjacent
Exercise9.5.8 correction. No claim of novelty or previously unreported
error is made; no full-book reading or corpus promotion is asserted.
[pcc:unverified]

Two further Lean controls check the count theorem's hypotheses:

- Deleting nonsquare scaling fails on the credited F3 curve
  y²=x³+2x²+1, which has five actual points. Taking d=1 gives actual
  count sum ten, whereas 2(q+1) is eight.
- Deleting nonsingularity fails on the credited F5 cusp y²=x³ with
  nonsquare d=2. Each actual nonsingular point group has five points,
  so the sum is ten, whereas 2(q+1) is twelve.

Both controls are direct negations of universally quantified versions with
the indicated condition deleted. The F3/F5/F7 model counts are credited to
the preceding packets. The F7 additive-map check enumerates the actual
point types through a proved equation-point equivalence and uses
`decide +kernel`; no native evaluator is trusted.
[pcc:unverified]

The main declaration is `QuadraticTwistCountAudit.twist_count_trace_certificate`,
in [TwistCountCertificate.lean](lean/TwistCountCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain exact statements and scope.
Exact local **PCC-20261114-501186** replay PASS at 2026-11-14T07:21:51Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/quadratic_twist_count_audit_20261114/lean
python3 research/quadratic_twist_count_audit_20261114/restore_attempts.py /tmp/twist-count-builds
```

Sources: [Galbraith author chapter9](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf)
and [author landing/errata](https://www.math.auckland.ac.nz/~sgal018/crypto-book/crypto-book.html);
[FST2010](https://link.springer.com/content/pdf/10.1007/s00145-009-9048-z.pdf),
selected source trace criterion and Section7.3 background;
[Milne](https://www.jmilne.org/math/Books/EC2.pdf), selected IV7 pp139–140,
whose characteristic≠2,3 restriction is retained. Official Mathlib finite-field
square criteria, point semantics and variable-change definitions, and preceding
actual group/point-count controls were reviewed. KB and Conductor were
unavailable; the bounded fallback preceded implementation.
[pcc:unverified]

The general characteristic-three trace-to-geometric criterion remains the
next component; it is not assumed here. General Hasse, endomorphism-rank,
embedding/full-torsion-field and other fourteen-item gaps remain open.
All failed source/configuration/log/exit states are retained. No holes,
custom axioms, native evaluator, raised limits, new global elaboration
flags, linter waivers, new native experiment, paid compute or agent upload.
[pcc:unverified]
