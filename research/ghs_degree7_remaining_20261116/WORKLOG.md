# Degree-seven remaining-shapes worklog

## 2026-11-15 22:41 America/Los_Angeles

### What was done

Read the newest dated shared session logs, prior degree-seven sparse,
linear-square and quadratic-square protocols, the trace-`+22` 24-cell result,
and Howe's original Theorem 4.17. Queried the shared research KB (revision
`9141e386254340a9bdba3036f2bb706a`, refreshed 14 November 2026) for
degree-seven cubic-square covers, trace-`+22` GHS, and Howe's bound. Wrote the
frozen protocol and cubic-square Sage source in this directory. No counted run
had started at this entry.

### Decisions made

The linear branch factor cannot be an odd pole of an even-degree denominator
in the normalized degree-seven case. The remaining denominator classes are
`B=H²` with monic cubic `H`, `B=Q_iQ_j L_1²` with linear `L_1`, and
`B=Q_iQ_jQ_k`; the first uses three quadratic equations in three field variables
and is the next executable screen.

### Open threads

Commit the frozen source, run both source-orbit representatives, audit any
solutions and failures, then move to the other two denominator classes.

### Errors

Conductor holds `docs/session-logs/2026-11-15.md` under T-6448, so this scoped
worklog records this task's decisions until that journal can be reconciled.

## 2026-11-15 22:46 America/Los_Angeles

### What was done

Committed frozen protocol and source at `1417546d4915`. First b=68 run took
4.78 process CPU seconds. It completed 232 zero-dimensional systems with 355
allowed cubic parameters, all failing the square identity, but 11 systems had
positive-dimensional ideals and were censored. Added an exact finite-field
recovery for those systems; source has not yet been rerun at this entry.

### Decisions made

The partial receipt cannot support a zero-hit statement. Re-run both source
orbits after committing the positive-dimensional recovery.

### Open threads

Complete and audit b=68 and b=84; then address other denominator classes.

### Errors

Initial solver logic treated positive-dimensional ideals as censored. The
amended bounded enumeration resolves them or records a solver error.

## 2026-11-15 22:55 America/Los_Angeles

### What was done

Started b=68 and b=84 finite-field recovery runs at source `79da85c9c7e2`.
Both remained active after several minutes. A process sample of b=68 showed
the cost concentrated in repeated NTL polynomial gcds in the univariate
fallback. Sent SIGINT to both processes; both exited 130 without a final JSON
receipt. Changed the fallback to take degree-at-most-two roots from the first
nonzero polynomial and verify all remaining equations.

### Decisions made

The two interrupted runs are incomplete evidence, and the edited source must
be committed before restarting both targets.

### Open threads

Commit, rerun, audit all cells and exact identities; continue the other shapes.

### Errors

Univariate gcd was needlessly expensive. Direct root filtering is exact here
because every specialized equation is quadratic or lower in `w`.

## 2026-11-15 23:02 America/Los_Angeles

### What was done

Stopped both second-amendment runs without a final JSON receipt after the
finite-field fallback expanded many parameter triples; process sampling showed
the candidate-list construction. A separate exact diagnostic checked all 243
b=68 coefficient ideals: 232 had dimension zero, ten dimension one, and one
dimension three. In each dimension-one ideal, the reduced Gröbner basis has
a monic relation `t³+c w³+d=0` with coefficients in `K`. In the dimension-three
case the CRT branch residue `h` is constant, so `H²h` has degree six.

### Decisions made

Use inverse Frobenius to solve `t` from `w` and then roots of quadratic
equations in `s` for every `w∈K`. This is exact for the checked relation
shape and bounded by 243 univariate systems per exceptional assignment.

### Open threads

Commit third amendment; finish b=68 and b=84, independently audit receipts.

### Errors

The direct finite-field fallback was exact but not computationally bounded
enough in the exceptional ideals. The interrupted files contain no results.

## 2026-11-15 23:08 America/Los_Angeles

### What was done

Committed the Frobenius reduction at `0c362e2ba465`. Both counted v3 runs
completed. b=68: 243 systems, 232 zero-dimensional, ten Frobenius-reduced,
one constant assignment, 2,785 parameter solutions, 355 degree-seven
coprime candidates, zero full identities, zero censored, 7.636379 CPU s.
b=84: corresponding counts 243, 232, ten, one, 2,774, 340, zero, zero,
7.837593 CPU s. The source SHA256 matches between both receipts. Wrote the
bounded result note.

### Decisions made

This closes the cubic-square denominator class for the fixed carrier and
the two target representatives. Continue to two quadratic branch poles plus
a linear square, then three quadratic branch poles.

### Open threads

Independent elimination replay; remaining two degree-seven pole classes;
explicit trace-+22 transfer if any map is found.

### Errors

None in the final v3 runs. Earlier interrupted receipts remain incomplete.

## 2026-11-15 23:13 America/Los_Angeles

### What was done

Committed final cubic-square receipts and bounded result at `10c907330308`.
Derived the two-pole linear-square identity by cancelling `S=Q_iQ_j` and
`H²=(x-c)²` from the cover equation. Wrote its frozen protocol and Sage
screen. The exact candidate count is 7,906,140 per target representative
after the necessary nonzero-square leading-coefficient filter.

### Decisions made

Use ten necessary square evaluations to keep full polynomial checks small;
all survivors still receive the complete identity check. Freeze and commit
this source before any counted run.

### Open threads

Run b=68 and b=84, audit all coverage counters, then address the three-pole
class.

### Errors

None in this preparation.

## 2026-11-15 23:18 America/Los_Angeles

### What was done

Committed the two-pole linear-square protocol and source at `7804e49e1678`.
Both counted runs completed. Each checked 7,906,140 logical candidates and
65,340 denominators. b=68 had 11,076 point-filter survivors and zero full
square identities in 27.232 CPU s; b=84 had 11,195 and zero in 27.648 CPU s.
The two receipts have the same source SHA256. Wrote the bounded result note.

### Decisions made

This closes the second of the three previously unsearched normalized
degree-seven denominator classes on the fixed carrier. The last class is
`B=Q_iQ_jQ_k`; its naive candidate count is 643,043,610 per source model
after the square-leading-coefficient filter. Use exact bitset intersections
of necessary point conditions to avoid per-candidate polynomial arithmetic.

### Open threads

Freeze and run the three-pole screen, then independently audit all three new
shape results and assess the fixed-carrier degree-seven bound.

### Errors

None in the two counted runs.
