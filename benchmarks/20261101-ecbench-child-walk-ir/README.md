# ecbench: the alias-table parse and the registry scan out of every child, the tuned rho walk's bookkeeping (2026-11-01)

Copies for browsing. The study, with the data tables, the exported child
inputs, the figure source and the reproduction commands, is
[`research/ecbench_child_walk_ir_20261101/`](../../research/ecbench_child_walk_ir_20261101/README.md);
the run report is its `reports/run-20261101T0130Z/`.

- `report.pdf`: the run report;
- `child_and_walk.svg`, `child_and_walk.png` (150 dpi): Callgrind
  instructions before the solve marker per 20-bit child, instructions per
  GAE inside the solve (20- and 32-bit), the 20-bit child wall medians, and
  the whole-process 32-bit counts, for the baseline (`main` 8a208fa570) and
  the three steps A, B, C.

Headline (Callgrind Ir, deterministic): a 32-bit `rho.negation` child
274.8M → 32.8M instructions (−88%), a 32-bit `ic.pipeline` child 1,147.9M →
907.4M (−21%), a 20-bit rho child 208.2M → 3.7M; the `rho.negation` solve
itself 928.9 → 881.0 Ir/GAE at 32 bits (−5.1%). Every record of both
scratch sessions replays identically under every binary, so no counted
cost, `S` or leaderboard row changes. Engineering.

**No row in `benchmarks/README.md`**: the validation gate admits a row only
with a replay certificate, a validator report and a matched rho reference
(`docs/VALIDATION_GATE.md`); the 2026-10-24 child-overhead entry is the
precedent for a harness-cost measurement kept here without a row.
