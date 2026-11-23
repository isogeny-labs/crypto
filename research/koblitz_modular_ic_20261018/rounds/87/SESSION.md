# R87 project handoff, 2026-11-03 19:49 UTC

## What was done

Froze and pushed the R87 protocol/source at `e1e235997`, ran one
bounded Singular attempt, and checked the complete degree-16
relative-equation diamond pullback. The 145 source monomials
produced a 4,612-term cleared polynomial with a zero remainder and
a full ideal lift. A lambda-shift control was rejected. Compiled
four characteristic-two Lean shift lemmas; the axiom audit found
zero failing and zero trusted. Rendered and inspected the one-page report. Raw files and
hashes are committed with the result.

## Decisions made

Treat the equation compatibility as a rational-open result. The
next check is an explicit inverse upper-field formula and complete
rank-32 RR regularity lifts in both directions before promoting
normal-chart gluing or exceptional-branch geometry.

## Open threads

Check current-head CI, then derive
the inverse and 32-generator transport. The independent 132-section
finite-u RR intersection remains available.

## Errors

The first Conductor `--next` call claimed unrelated T-75; it was
released immediately without editing its files. The dated-log task
was initially claimed with an incorrect ID; no change was made to
that other task. T-1466 reserves R87, and T-1471 reserves today's
session log. No R87 mathematical run failed or was duplicated.
