# ecbench round 2: the tuned walk's cycle window, the 32-bit inversion, the IC phase trace (2026-11-01)

Copies for browsing. The study, with the data, the exported inputs, the
figure source and the reproduction commands, is
[`research/ecbench_walk_trace_ir_20261101/`](../../research/ecbench_walk_trace_ir_20261101/README.md);
the run report is its `reports/run-20261101T2010Z/`.

- `report.pdf`: the run report;
- `round2.svg`, `round2.png` (150 dpi): 32-bit Callgrind Ir per child
  relative to the #3144 head, 20-bit solve Ir per GAE, and the interleaved
  wall rounds with an A/A arm.

Headline (Callgrind Ir, deterministic): 32-bit `rho.negation` solve
30.09M → 24.78M (−17.6%, 881.0 → 725.6 Ir/GAE); 32-bit `ic.pipeline`
child 916.3M → 776.3M (−15.3%); 20-bit IC solves −17% to −18%. Every
record identical. Engineering.

**No row in `benchmarks/README.md`**: the validation gate admits a row only
with a replay certificate, a validator report and a matched rho reference
(`docs/VALIDATION_GATE.md`).
