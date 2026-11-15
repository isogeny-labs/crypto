# Difference-surface decoder handoff

## What was done
Reviewed prior transport results and current shared log. KB query and primary
Pauly–Previato/Tian review completed. Derived an explicit bilinear Hessian test
and nonzero complementary-pair formula. Frozen destination-only protocol and
native implementation; exact source quotient jets remain a separate gate.

## Decisions made
Test generic split transverse chords with explicit exclusions; sum-surface and
zero-Hessian controls expose ambiguity. No inference from group-order matching.

## Open threads
Build, commit/lock/source checkpoint, run once. Then actual source jets and
Gamma00 difference-divisor specialization proof, followed by public-input decomposition.

## Errors
None so far. Shared resume untouched; scope claimed T-6227 epoch1.

## Registered execution
Sourcef476d0dedabd/checkpoint161127 preceded single release run, exit0/stderr empty.
3825 admitted pairs unique; sum controls2candidates and zero controls6candidates
on all3825. Exclusions306chart,0Fy,4125not4rationalpoints; total8256pairs.
PROOF adds reduced difference-divisor specialization argument after execution,
separate from raw destination-only gate. Next: actual translated source jets.

## Reports
One-page REPORT.pdf compiled, rendered to PNG and visually inspected. PAIRS.svg
plots exact surviving candidate counts1/2/6; no runtime inference. Proof and
results distinguish derived destination Hessians from unevaluated source jets.
