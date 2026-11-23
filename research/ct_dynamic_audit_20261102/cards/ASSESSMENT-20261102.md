# Assessment, CARD-ct-dynamic-audit-20261102-1 (2026-11-02)

Machine-readable form: `CARD-ct-dynamic-audit-20261102-1.assessment.yaml`. Interpretation: not assessed.

- Outcome: `executor_deviated`; relay_verbatim false; recommendation `escalate`. The only discrepancy is an ordering artifact (below); the main session settles it.
- check-relay JSON (tool function called directly, because the CLI re-lints and the lint now fails, see last bullet):
  `{"applicable": true, "missing_lines": ["output .../results/rows.jsonl: 2a5c921a...c5d4", "output .../results/summary.json: 7eb2fa01...d71f"], "receipt": ".../CARD-ct-dynamic-audit-20261102-1.receipt.json", "relay_verbatim": false, "status": "completed", "verify_findings": []}`
- Cause: the executor lists the four command-3 outputs in the card's order (ENV, rows, summary, fig). The check recomputes from the receipt JSON, which is stored sorted (ENV, fig, rows, summary). With the outputs in card order the recomputed brief has `missing_lines []`. All digests are identical; no number differs.
- Receipt: completed, commit a1530967, card sha ae18c737, four commands exit 0, one attempt each, seconds 0.28 / 0.325 / 2318.055 / 0.003; receipt sha fd4fee55...351d.
- Digests on disk equal receipt and report: binary ebdf722a (= bound value; build no-op, 0.19 s), rows.jsonl 2a5c921a, summary.json 7eb2fa01, fig_ir_spread.svg 309e8af7, ENV.txt 03169a9a (valgrind-3.22.0, commit a1530967).
- Acceptance: summary.json lists 11 targets; 11 `memcheck-<target>.xml`; per target 81 callgrind files = process-exit file + dumps .1 to .80 (80 per-call dumps; 880 total); rows.jsonl 2290 lines; analyzer printed `rows: 2290 lines; summary: 11 targets`.
- raw/stderr.log (6144 bytes): no fatal, Assertion, unhandled, Killed, abort or error text; Callgrind headers and `I refs` lines only.
- Write scope: only the receipt and the executor report show in `git status`; the excluded raw/, results/ and cards/*.stdout|stderr are inside scope. HEAD moved to 2d9a44841 after the run (adds only the card).
- Not checked: executor transcript commands beyond the card (only the receipt is evidence).
- Card note: `lint` and `check-relay` now print CARD FAIL because command 4's argv names output files that exist after the run and are not bound; they did not exist at pre-run lint. Not a run deviation.
