# Original fixture curve-membership diagnostic

Frozen before this diagnostic's output, 2026-10-25. This adds a finite
relevance control; it does not replace the original branch experiments or
their acceptance criteria.

Input: the byte-identical seven original ell=8 fixtures in
`inputs/guess-ell8.jsonl`. Use their exact sparse field representation
at n=22 and the original a=0,b=1 S3 curve equation. Independently solve
`y^2+x*y=x^3+1` at each target abscissa, verify every y root by substitution,
and compare existence with the Artin--Schreier trace criterion. Do the
same over every abscissa in each fixture's full 256-element V.

Report target curve-liftability separately on the three Boolean SAT and
four UNSAT draws, the per-draw y roots, and base membership counts. Do not
select inputs, alter guesses or infer subgroup membership from a curve
lift. A non-liftable target is not an ordinary query point on this curve;
a liftable target still needs subgroup and complete signed-decomposition
checks. Either outcome qualifies the relevance of the old fixture panel
without changing its valid finite Boolean results.

Native code: `examples/ic_fixture_group_audit.rs`; source pinned in Git
before execution. A correctness diagnostic only, no timings, crossover,
global degree or security claim. Preserve complete JSON and a descriptive
diagram/PDF with both positive and negative outcomes. Limits: existing
local 12-GiB/900-second envelope; no paid provisioning.
