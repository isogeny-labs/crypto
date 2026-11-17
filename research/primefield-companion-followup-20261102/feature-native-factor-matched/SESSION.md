# Factored-source instrumentation handoff (T-1576/T-1579)

## What was done
Added reversible paired/factored source instrumentation, new21-slot meter and
new raw schemas.14 maps invert exactly;6 non-source modules identical.72 prebuild
locks bind native source/data and initial strict analyzer. Production/test builds
pass. Two selected native methods pass,84 filtered:8 recovery equivalence checks,
separate setup vector21/9/6/3 and stage contracts. Seven parser tests and two
single-target aggregation tests pass. No comparative rows or cloud job.

## Decisions made
Keep exponent integer primitives out of field counters. Primary analysis includes
setup+prepare+query+verify field arithmetic per single target; retain query-only
analysis and setup primitives separately. Arithmetic totals are not cold timing.
Old preflight/panels remain unchanged; new source/binary/analysis-bound draft.

## Open threads
Pinned preflight for this instrument, authoritative quota and cost-reservation
readback, freeze, panel execution and independent review. Original4563 cases
remain required for that larger cohort.

## Errors
Initial cold-analyzer creation used a repository-relative path from the component
working directory and failed before writing code. The subsequent test import
failure is retained in initial-cold-analysis-test.log; corrected paths and both
new tests pass. Native builds/tests and reverse transforms passed. Compiler
warnings are retained; no numerical experiment was restarted.
