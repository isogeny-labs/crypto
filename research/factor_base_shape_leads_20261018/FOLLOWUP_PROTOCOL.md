# Follow-up protocol: fresh probes and matched representations

Frozen after inspecting the first panel, before these follow-up runs.
The original PROTOCOL.md, plans and native attempts remain unchanged.

## Observations motivating this round

- Native replay checks all48 attempts:47 complete and1 failed construction.
  The failed trace-zero/Frobenius dimension3 seed29 inventory is empty.
  All completed cases pass their native point/system/relation checks.
- The initial degree13 trace-zero GP dimension5 cases have15 columns each:
  holdout witnesses4/32 and2/32, training rank2 and3. The coordinate control
  has16 columns, holdout1/32, training rank0. These are screening observations,
  not a matched-cardinality cost ranking or a significance result.
- Degree31 coordinate reparameterizations contain exactly the same signed
  point set, but different ordering can choose a different positive control.
  No matched random probe succeeded there, so comparing its positive-control
  work alone would confound the representation with the chosen target.
- Ordered inventory hashes are not an unordered point-set identity:
  compare exact sorted (x,y) sets on the identical curve model. Keep all
  original FB1/FBM1 and ordered commitments.

## Fresh frozen questions

Confirmation (plans/confirmation.json): same degree13 model
icv1-f2m13-t181-515ee569, dimension5, 64training/128holdout probes, fresh target
seed2026101802. Six cases: one coordinate control, random seeds17/29,
trace-zero GP seeds17/29, plus previously untested GP constructor seed43.
Do not select another constructor after viewing this holdout. A follow-up
shape lead must produce at least4 verified held-out witnesses and nonzero
training rank on each of the three GP seeds, with no correctness failure.
Even then, cardinality differences and missing full-cost counters forbid a
speed winner or degree-of-regularity conclusion. Keep distinct-target counts.

Representation (plans/representation.json): coordinate/natural, reversed and
triangular bases on icv1-f2m7-tm13-ac10a42c dimension4 and
icv1-f2m13-t181-515ee569 dimension5; 32training/64holdout probes each.
Verify exact signed point-set equality within each curve, and compare matched
training/holdout target truth and outcomes. Positive-control counts are
excluded from work comparisons even if two controls happen to match.
The strict per-success component-improvement filter in PROTOCOL.md is
unchanged; show each arm's recorded native F4 reduction work and successful
procedural degree. A lower aggregate alone is not an all-target improvement.

Both plans retain degree8, 20seconds/candidate, 2million observed matrix
cells, 2048MiB, one thread and64 maximum abscissae. New requests:12cases,
1740probes if all complete, maximum nominal candidate budget240seconds.
No retry or larger substitute. Native executable/source/hash are unchanged.

Run confirmation then representation into new directories under the existing
scratch runs root, invoke native verify for each, retain all outcomes, and
freeze the final native analysis source and outputs. This is exploratory
conditioning after a screen; the fresh holdout limits direct reuse of the
first sample but supplies no formal multiple-testing correction.

The first native analysis attempt refused the failed candidate's missing
trial data; that failure is retained. Its repair preserves the failed row
with null phase measurements and its exact native fatal reason, rather than
treating it as zero-yield completion. Four analysis unit tests passed before
this repair; rerun them and add failure-row coverage before publication.

No root-library/solver source changes, complete native pricing, external
reference, isolated runtime, rho comparison, certified regularity claim or
funded campaign is introduced by this follow-up.

