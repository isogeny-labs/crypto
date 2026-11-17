> **PROVISIONAL, legacy Python, NOT FOR MERGE: violates the repo's no-Python rule; kept as labelled historical evidence; matched native re-measurement required before any claim.**

# Paired fixed-budget comparison: Frobenius phases on, off

Stage diagnostic on a toy field (`E0: y^2 + xy = x^3 + 1` over `F_(2^13)`). It runs the
harness in [`..`](../README.md) twice on the same frozen targets: arm A with factor-space
representatives only, arm B with one-hot Frobenius phases. `S`, end-to-end cost and speedup are
unset and no ECDLP speedup is claimed. The code is Python, which `AGENTS.md` ("Implementation
language: no Python") does not allow for new research comparisons; the receipts below are kept as
labelled historical evidence and the harness is **not to be extended**. The SAT backend is native
CryptoMiniSat 5.16.0; construction, orchestration, verification and rank code are Python.

Read in this order: [`PROTOCOL.md`](PROTOCOL.md) (registered before any code or run, with dated
addenda), then [`RESULTS_PROVISIONAL.md`](RESULTS_PROVISIONAL.md) (status, tables, verdict by rule,
limits, and the "Scaling to m = 51, 53, 83, 131" section). Visuals: `figure_n13.svg` and
`mechanism.svg`. No `report.pdf` was built.

## Files

| file | role |
|---|---|
| `PROTOCOL.md` | hypotheses, frozen inputs, arms, metrics, stop conditions; addenda dated 2026-10-26 |
| `freeze_targets.py` | deterministic manifest builder; refuses to overwrite a frozen file |
| `manifest.json`, `manifest.sha256` | the frozen targets, basis and representatives (what the arms may read) |
| `witness_sealed.json` | construction witnesses and brute-force answers; never read by `run_paired.py` |
| `run_paired.py` | one process, one solver thread, ABBA order, 120 s solver-CPU budget, one JSONL receipt per attempt |
| `rank.py` | verified decomposition to relation over representatives with `+-lambda^k` coefficients; four independent checks; rank over Z/r |
| `verify.py` | post-run audit: coverage, order, budget, relations, UNSAT against brute force, manifest regeneration |
| `summarize.py` | tables, `figure_*.svg`, `mechanism.svg` from the receipts (its PDF path was never run) |
| `test_paired.py` | 16 tests; all passed before the run |
| `paired_common.py` | shared helpers; imports the parent `encode.py` field and curve code, no copy |
| `results/` | receipts (`paired_n13.jsonl`, 48 attempts), run log, relations, rank data, audit output |
| `SHA256SUMS` | hashes of the frozen inputs, code and receipts |

## What exists

One complete n = 13 block: 12 frozen targets x 2 arms x 2 formulations, 48 attempts, 1360.8 s of
attempt CPU, no timeouts. Not run: the optional n = 19 block, repeat trials, a PDF.

## Reproduce (for audit of the legacy evidence only)

```sh
cd research/frobenius_quotient_sat_20260930/paired_fixed_budget_20261011
sha256sum -c SHA256SUMS                               # inputs, code and receipts unchanged
python3 freeze_targets.py --check                     # manifest equals regeneration
python3 verify.py --receipts results/paired_n13.jsonl --out /tmp/verify_n13.json
```

Re-running the solver arms (`run_paired.py`) is deliberately not documented here: the next
measurement belongs in native code.
