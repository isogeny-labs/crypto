# DEVIATIONS: E0-separation-lemma

The preregistered protocol (PREREGISTRATION.md, sha256 unchanged) and the frozen scripts (SCRIPTS.sha256, all OK) were not modified. Deviations 1-10 are in RESULTS.md ("Deviations and clarifications").
Finalisation pass (2026-10-26), all additive and outside the frozen scripts:

11. No tier was recomputed: RESULTS.md was already complete and every number traces to `out/*.txt` through `aggregate.awk`; `replicate/` holds an earlier full independent re-run whose `results.json` is byte-identical to the original (checked with cmp before the ops keys were added).
12. Added `opcounts.awk` (operation counts derived from recorded rows) and one line in `aggregate.sh` appending its output to `out/results_kv.txt`, hence to `results.json`. The `ops.*` keys are new; all other keys are unchanged. Counts are formula-derived from recorded fields and the frozen loop structure, not instrumented; field multiplications are not counted.
13. `run.sh` is unchanged and still the single driver (it calls `aggregate.sh`).
