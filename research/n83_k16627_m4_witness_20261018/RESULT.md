# Result: K=16,627 complete-witness control

The exact stored primary N83 base passed native group preflight and full
four-summand orbit-domain construction under the frozen one-CPU, 4 GiB
container. Its seven-block positive-control worker entered SAT at 40.267 s
and reached the 1,800 s outer wall cap without a solver return. Outcome:
`UNKNOWN_wall_cap`, exit 124, no OOM, peak 2,661,351,424 bytes. The
dependent unpinned arm was not attempted. See the [run report](RUN_REPORT.md),
[phase JSONL](positive/worker.json.phases.jsonl),
[outer outcome](positive/outcome.json), [diagram](witness-phases.svg) and
rendered [PDF](RUN_REPORT.pdf).

This localizes the remaining positive-control delay to the solver call;
initial propagation versus later conflict search remains unresolved. It
does not select a factor base or update an index-calculus cost row.
