# Affine-pencil collision screen, 2026-11-15

## First gate terminal
### What was done
Protocol992262265epreceded code. Native gate passes13577exact class equalities,
3316split principal identities,447degree drops,316repeated and136ramified
split residuals;9low-degree Dskipped,443u-root point skips. Existing native
oracle decisions/roots match exhaustive division on every candidate.
### Decisions made
The proposed residual equals -(D+P_i+P_j), the existing two-place walk class.
Treat construction as known addition/reduction, not a new mathematical10xlead.
The first Dcohort is a sum of gcanonical places and therefore often split;
general proof applies independently but implementation coverage should include
non-split public-divisor inputs before closing this gate.
### Open threads
Freeze an additional input cohort, keep first output, then publish/checkpoint.
### Errors
No mismatch. Initial input generation alone did not establish non-split-D
coverage; this limitation is being addressed explicitly, not inferred away.

## Pre-code coverage amendment
### What was done
Reviewed that gplace sums can retain split support in their degree-gu.
### Decisions made
Keep original cohort0unchanged. Add cohort1with16divisors per same curve,
each the sum of2g+3seeded places, to force reduction before the test. Count
degree-g inputs whose udoes NOTsplit, and require positive coverage per genus.
All pair enumeration, assertions and small fields remain unchanged. This is
additional correctness coverage, not a rerun for better numerical results.
### Open threads
Execute both cohorts and retain source/output hashes; no timings.
### Errors
None; first output preserved separately.

## Expanded gate terminal
### What was done
Both cohorts pass30759exact equalities with the two-add_placeclass and7319
principal checks. Non-split Dinputs25/76/85forg2/3/4respectively. Coverage:
1190degree drops,781repeated and347ramified split residuals. Native oracle
roots/decisions match exhaustive division throughout. RESULTS.md records the
proof, coverage limits and closure decision. No speed experiment was run.
### Decisions made
Class relabelling/possible arithmetic engineering. The quadratic residual in
genus2is ordinary reduction, so this supplies no new relation-yield mechanism.
Keep full10xgoal; next complete-cost screen must include rank caps and actual
stopping times, or pursue a genuinely distinct mechanism with a cost case.
### Open threads
Commit/publish/checkpoint with hashes, follow fresh CI. No scientific worker
is live after terminal output; no cap or historical record was changed.
### Errors
No equality/oracle/group failure. The initial limited split-input coverage was
explicitly amended and the first output retained; no independent replay claim.
