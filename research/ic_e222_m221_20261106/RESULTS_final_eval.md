# Pair-sum table vs a hoisted sweep: frozen tune/eval (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, serial runs; no replay certificate, validator report on this panel, or matched-rho band. Not a leaderboard row.

**Why this panel.** The independent validator (`results/confirm/VALIDATOR_REPORT.md`) found that part of the earlier table gain was baseline waste: the sweep rebuilds its x-index on every relation call. This panel adds the missing control (`IC_HOIST_SWEEP=1`, the sweep with the index built once per solve; default path unchanged) and removes the selection bias: `run_final.py tune` chose each algorithm's best fb on v4 instances 0 to 9, `frozen.json` was committed and pushed at 01:55:34Z (commit 48cfa58) before `run_final.py eval` started, and the evaluation used v4 instances 10 to 29 (W 14, 15, 16 bits; 60 instances, never used by any earlier panel; generator seed `e222m221-holdout-v4`), solver seeds 1, 2, 720 runs, serial, interleaved seeded order. Both the solver's stage timer and the process wall time are recorded. Data: `results/final/` (`rows.jsonl`, `frozen.json`, `tune_rows.json`, `analysis.txt`).

Frozen: shipped default (A) fb 8; sweep extra 64 (B) fb 128; sweep extra 2 (C) fb 128; hoisted sweep extra 64 (Bh) fb 256; hoisted sweep extra 2 (Eh) fb 256; pair-sum table extra 2 (D) fb 32.

| arm | fb | solver ms | process ms | median trials |
|---|---|---|---|---|
| A shipped default | 8 | 708.5 | 733.9 | 18,818 |
| B sweep, extra 64 | 128 | 46.3 | 68.8 | 304 |
| C sweep, extra 2 | 128 | 35.6 | 60.7 | 206.5 |
| Bh hoisted sweep, extra 64 | 256 | 33.7 | 57.0 | 325 |
| Eh hoisted sweep, extra 2 | 256 | 26.9 | 50.6 | 262 |
| D pair-sum table, extra 2 | 32 | 16.7 | 40.2 | 542 |

Paired ratios, median [bootstrap 95% CI], 120 pairs:

| ratio | solver ms | process ms |
|---|---|---|
| D / Bh (table vs fair, tuned baseline) | 0.446 [0.410, 0.470] | 0.664 [0.609, 0.688] |
| D / Eh (table vs best sweep) | 0.531 [0.509, 0.574] | 0.737 [0.712, 0.763] |
| D / B (table vs the unhoisted baseline) | 0.347 [0.322, 0.370] | 0.559 [0.501, 0.595] |
| D / A (table vs shipped default) | 0.024 [0.023, 0.025] | 0.056 [0.049, 0.063] |
| Bh / B (hoisting alone) | 0.762 [0.734, 0.844] | 0.846 [0.811, 0.902] |
| Eh / Bh (relation surplus alone, hoisted) | 0.799 [0.775, 0.829] | 0.864 [0.833, 0.911] |

D beat Bh in 120 of 120 pairs on the solver timer and in 116 of 120 at process level. Per-size D/Bh (solver): 0.373, 0.458, 0.439, 0.453 for r bits 14 to 17 (no clear trend). All 720 runs recovered the planted log.

**Decomposition of the improvement over the unhoisted baseline B (solver time).** hoisting the index: 0.762; the lower relation surplus (extra 2): about 0.80 more; the table: about 0.53 more. The table is therefore the largest single factor (about 1.9x over the best sweep at its own best fb), but about a quarter of the headline gain over B is engineering of the baseline (hoisting) and about a fifth is a retune. The fair statement of the structural gain is D/Eh = 0.531 on the solver timer, 0.737 at process level.

**Costs counted.** Factor-base construction, relation collection (including the O(fb^2) table build, inside `relations_ms`), linear algebra and extraction, and verification ([x]G = Q) are all inside the solver timer; the process figure adds startup and JSON output. No lifting (prime-order group).

**Limits.** Toy sizes (p up to 2^18). The table's memory and build grow as fb^2 and the optimal fb grows with p; nothing here shows what happens at cryptographic sizes, and the estimate in the session log suggests the best case is not below p^(2/3), against rho's p^(1/2), so a constant-factor gain at toy size is not evidence of closing the gap to rho. No matched rho reference or 0 / 0.1 / 1 GAE band here. Not run on isolab-eks. The fb for each arm was chosen from a 5-point grid on 30 instances with one seed. The goal is not met.
