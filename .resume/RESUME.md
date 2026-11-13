# Resume: crypto @ claude/relation-finding-no-groebner-vrg7ec

## Goal
Owner (2026-11-13): "Relation finding that avoids Gröbner bases altogether (probabilistic, SAT or
lattice-flavoured decomposition) is thinly explored and poorly benchmarked. This needs more exploration."
Answered with (a) a map of what the repository already measured, and (b) one matched experiment: the
table-free linear 2+1 decomposition (lin21, lin21-ht) on the frozen 2026-10-14 m=3 cells and targets.

## Spec
research/groebner_free_matched_20261113/PROTOCOL.md, frozen in commit c90b1d9d54 before any claim run.
Instrument: examples/ic_algebraic_floor (arms lin21, lin21-ht; subcommands linpool, linsummary).
Runner: research/groebner_free_matched_20261113/run_grid.sh (one cell per process, one job at a time).

## State
2026-11-13: pilot (seed 777) done, 0 disagreements with the floor. Claim grid launching; raw records go
to research/groebner_free_matched_20261113/runs/. S3 checkpoint unavailable in this container (no AWS
identity); the branch push is the durable copy.

## Next
run_grid.sh; `ic_algebraic_floor analyze --dir runs --out ANALYSIS.json`; `linsummary --dir runs --out
SUMMARY.json`; figures, REPORT.typ/pdf, README; benchrec close; session log; PR; red-team/validator pass.
