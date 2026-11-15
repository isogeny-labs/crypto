# Wallfix PRs on RunPod: session-level A/B with an A/A control (2026-11-14)

Owner request: "run on RunPod for now" (isolab-eks blocked by its monthly budget). Standing:
`[pcc:unverified]`, RunPod tier (shared host, below isolab L2).

Report: [`reports/run-20261114T2240Z/report.pdf`](reports/run-20261114T2240Z/report.pdf)
(source `report.typ`, graph `ab-ratios.svg`, data `reports/run-20261114T2240Z/data/`).
Benchmark entry: [`benchmarks/20261114-wallfix-runpod-abab/`](../../benchmarks/20261114-wallfix-runpod-abab/).

## Result

Outputs identical in all 18 A/B pairs (normalised records, `data/identity.txt`); 36/36 sessions
verified with `--replay-all` on the pod and again on a second host (`verify-local/`).
Session-total solve CPU time, merge over first parent, geometric mean of 3 counterbalanced rounds:

| PR | spec | B/A | 3-round range | A/A spread (run 2) | dominant arm |
|:--|:--|--:|:--|--:|:--|
| #4009 | tensor | **0.654** | 0.645–0.664 | 1.012 | ic-tensor (100%) |
| #3850 | prime | **0.905** | 0.905–0.905 | 1.001 | ic-rf-pairs (94%) 0.901–0.902 |
| #4009 | koblitz | **0.926** | 0.925–0.927 | 1.015 | da-f4 (93%) 0.919–0.923 |
| #3850 | tensor | 0.964 | 0.958–0.968 | 1.025 | ic-tensor (100%) |
| #3850 | koblitz | 0.977 | 0.973–0.981 | 1.001 | rf-pairs (8%) 0.73–0.80, da-f4 0.993–0.996 |
| #3991 | prime | **1.013** | 1.011–1.015 | 1.004 | ic-rf-pairs (94%) 1.011–1.014: a small, consistent slowdown, not yet attributed |

The PRs change wall time only: counted cost, and so the §5a band against matched rho
(`band/`), are the same for A and B; rho stays ahead of IC across the band on every curve.

## Run 1 (A/A control)

Run 1 compared byte-identical binaries (stale cargo fingerprint: `git archive` mtimes older than
the shared target dir). It is kept as an A/A control (`sessions/run1-aa/`,
`reports/run-20261114T2240Z/data/results-aa.json`). `build.sh` now touches the unpacked tree and
refuses a build that did not compile the `crypto` crate.

## Layout

- `specs/`, `mksrc.sh`, `build.sh`, `run.sh`, `tarballs.txt`: what ran, how it was built.
- `sessions/run2/`, `sessions/run1-aa/`: ecbench sessions (`records.jsonl` in the artifact store).
- `verify-local/`: independent `verify --replay-all` receipts from a second host.
- `band/`: `ecbench band` (§5a) per PR, spec and arm.
- `analysis/`: `analyze.py` (ratios), `plot.py` (graph).
- `validator/`: independent recompute of the headline numbers.
