# Four-summand algebraic extraction: development protocol

Status: frozen before the first new native solver invocation. This is a bounded
correctness qualification, not a performance experiment. Parent tracking PR: #4938.

## Objective and source reuse

Implement the degree-five Riemann–Roch specialization of the existing RR9
construction. Extract four distinct factor-base abscissae from a public target
using polynomial remainder equations and the existing F4/root-finding code.
Recover signed points, verify their sum, and feed only genuine public labelled
relations into the original authentic-RHS linear algebra contract.

The principal curve remains PRIME31 in
`research/prime31_ic_20261115/native/lib.rs`. Its original Arithmetic is reused.
The existing production-source integration qualifier supplies F4 and finite-field
root factoring; it does not qualify the full repository library. No changes to
the merged #4901 or #4924 source/evidence are part of this development.

## Hypotheses and falsifiers

1. The RR5 remainder system has exactly the signed, distinct-x four-leaf
   decompositions for finite Q outside the factor base. Check exact point sums
   and a separate small-field signed-tuple oracle in test-only code.
2. Original F4 can extract positive roots or refute empty B=4 cases without
   factor-base tuple enumeration. Deny every non-univariate branch before child
   traversal. No available univariate means inconclusive, never empty.
3. Higher B can reuse the same interface. B=8 is the next correctness control;
   a degree/time/size limit is a retained incomplete outcome. It is not a
   statement of mathematical impossibility or evidence at B=677.
4. Signed relation insertion must distinguish duplicate, dependent and new-rank
   rows. A recovered target scalar is accepted only after original-curve dG=Q.

## Fixed development controls

- PRIME31 B=4 points chosen from a deterministic public x scan. Public targets
  include a positive sum with a nontrivial sign, its negative, and an ordinary
  point checked against the independent B4 oracle. Witnesses stay test-only.
- Ordinary small-field control: p=11, A=1, B=6, n=13, G=(2,7), and base
  [(2,4),(3,5),(5,2),(7,2)]. Compare all 12 finite public targets to an
  independently implemented integer group law and test-only signed-four oracle.
  Exactly (8,3),(8,8),(10,2),(10,9) are outside the base; the other eight
  finite points and the identity must have the explicit chart status.
- B=8 bounded positive and empty cases, if the B4 extraction gate passes.
- Reject duplicate abscissae, zero-y base points, invalid points, identity Q and
  target-in-base with explicit chart status. Preserve repeated-x scope as open.
- Stop before any forbidden full-field/finite-domain fallback. Exercise the
  fail-closed path deliberately. Preserve budget/interruption outcomes.
- Relation mutations, sign changes, false RHS, duplicate/scalar-dependent rows
  and exact target recovery. No base-point secret logs enter the adapter.

## Resources and evidence

Local development only: at most two compiler workers and one test worker;
per solver call deadline <= 5 seconds, initial degree cap 5 for B4; later
bounded controls may use degree <= 10, 30 seconds and a documented matrix cap.
Limit each test process to 180 seconds and retain nonzero exits. Use an immutable
copied test executable with a pre-call SHA-256. Keep build, pre-call and post-call
manifests distinct; never overwrite a prior attempt. Capture raw stdout/stderr.
No benchmark clock result, speed ratio, GAE calibration or fitted exponent is
derived from these tests. All failures and caps remain part of the record.

## Original completion gate, still required

After an extractor passes the public correctness/rank gate at the requested
factor-base size, preregister an admitted cold single-target panel on at least
three eligible ordinary curves. Compare the strongest reproducible applicable
algebraic IC incumbent and optimized negation-map rho on identical public
targets/resources. Include base/selector construction, symbolic setup, all
successful and unsuccessful queries, roots, signs/exceptions, useful rank,
linear algebra, target recovery, replay, peak memory, failures and interruptions.
Report amortized work separately. Parameterize growing sizes before interpreting
an asymptotic exponent. The existing generic pair tables are controls only.

## Sources checked

KB unavailable: no loaded research-KB tools; `scripts/research-kb status`
returned exit 127. Fallback: current AGENTS, ACTIVE, November 22/23 logs, merged
PRs #4901/#4924, run69/70B/71/72, trace-skew and monic-root sources, and
`research/prime31_ic_20261115/rr9/RR9_PROJECTIVE_D9_THEOREM_20261117.md`.
Semaev 2004/031 and Faugere–Huot–Joux–Renault–Vitse EUROCRYPT 2014 were located;
direct IACR PDF retrieval returned 403 in this runtime. Existing internal
source-linked discussion supplies the scoped prior-art context. No novelty
claim follows from this incomplete literature access.

## Coordination

Conductor is unavailable on PATH. Use a new isolated checkout and disjoint
new source paths, as documented by `docs/CONDUCTOR.md`. Root owns extraction,
protocol/evidence and publication. A separate worker owns the signed-row adapter;
a mathematical reviewer owns the written equivalence and source review.
