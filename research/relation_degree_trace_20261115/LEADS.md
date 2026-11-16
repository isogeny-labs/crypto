# Leads after the local component confirmation

Status: development and bounded follow-up preparation. No new timing campaign
or mature-CAS advantage has been established `[pcc:unverified]`.

The first study found a native reference improvement but also a much stronger
external baseline. Keep those distinct. The next lead must charge construction
and certification, compare the best existing native route to its successor,
and retain an external full-cost comparison before any practical claim.

## Lead 1: project the original quadratics directly

Each recognized f3 pair is quadratic in the eliminated coordinate. The current
implementation instead builds two cubics and expands a Sylvester determinant
of size five. Project the original two quadratics with their size-four
resultant, using an explicit polynomial identity. This can reduce construction
and expanded membership-certificate work without a scout, a witness, or a GB.

For f=A*y²+B*y+C and g=D*y²+E*y+F, with coefficients in Fp[x], set

```
U=A*E-B*D; V=A*F-C*D; W=B*F-C*E
R=V²-U*W
R=(D*U*y+E*U-D*V)*f + (-A*U*y+A*V-B*U)*g.
```

Expanding the right side proves membership without dividing by A or D.
It is the standard quadratic resultant identity, not a novelty claim. The
identity remains valid at specializations with a zero leading coefficient;
such a specialization is not automatically an exact root-existence test.
Only append consequences; never replace the original ideal by a resultant.
Require nonzero output before monic normalization and check certificates.

Use the original inputs, so scaled equations are handled without assuming
their coefficients are normalized. Reuse the symmetry of each recognized f3
equation to project the other endpoint. Take certified GCDs at shared vertices
as before. Compare complete bases against the original ideal, including
degenerate/small-characteristic tests, rather than comparing only witnesses.

Limits: preprocessing is a small part of the final native candidate interval.
If the new monic vertex polynomials and ordered augmented input list are
unchanged, the solver has the same work; this lead alone cannot be assumed to
provide another drastic native gain. It is also worth testing only as a
full-cost external candidate, since preprocessing was previously unpaid in
the CAS diagnostic. These are hypotheses `[pcc:unverified]`.

## Leads not yet promoted to experiments

1. Use a certified univariate quotient instead of general Buchberger on the
   structured four-solution family. Starting with a certified vertex quartic
   H(t), reduce a neighboring pair of quadratics over Fp[t]/H. Their linear
   subresultant is U(t)*y+V(t). If gcd(U,H)=1, an extended-Euclidean inverse
   gives y=-V/U mod H and an exact original-input certificate. Propagate
   across the connected shared-point graph, checking every inversion and
   retaining a failure rather than dividing by a zero divisor. Generate the
   reduced grevlex basis by linear dependence in the small quotient algebra,
   with ideal equality and every final S-pair checked. This could remove the
   dominant solve/check work; no implementation or timing is claimed yet
   `[pcc:unverified]`.
2. Keep original-input provenance as a DAG and verify shared subexpressions,
   instead of repeatedly expanding coefficient maps. Preserve exact checks
   and separate a checker-engine effect from an equation-selection effect.
3. Trace projected four-variable inputs against an independent completed
   reference under strict degree/row/column limits. A faster wall result does
   not establish a smaller closure threshold, top-form d_reg or paid degree.
4. Integrate reusable point pools and P/Q labels only after demonstrating a
   useful full-cost relation-solver result; the controlled paired-f3 geometry
   does not establish relation yield or logarithm recovery.

## Execution gate and durability

Development may proceed locally under AGENTS.md. Before any new experiment,
the required `scripts/checkpoint.sh gate` must upload a complete resume state
to S3 and pass. The initial API-staged directory lacked Git metadata; a real
sparse checkout of PR #4228 now supplies it. No checkpoint token or AWS upload
transport is currently configured. Do not bypass the checkpoint gate or
launch a cloud job. Freeze QPROJECTION_PROTOCOL.md before any measurements.
The root session log and PCC ledger still have concurrent merge conflicts;
do not resolve them by rewriting evidence.
