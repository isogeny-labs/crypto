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
