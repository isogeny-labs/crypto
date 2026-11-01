# EXP-PFDR-2d36f9-R2: rerun of the break-even growth ladder (result: O-SUB-BE)

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit 534ff08faa before any data; two design corrections, see `implementation.patch`). Report [REPORT.pdf](REPORT.pdf).
Reproduce: `scripts/run_all.sh` (about 17 minutes on 4 workers: 19,818 census jobs), `scripts/make_fig.py`. Provenance of the code: `PROVENANCE.sha256`.

Reading `O-SUB-BE`: all six structured lines valid (smallest rung mean coincidence count 3023 >= 2500), selftest and all blocking controls pass (C3 plants, C4 Z/N, C5 A/A, C6 z_cal: n = 10, mean 0.19, sd 0.83),
no ALIVE line, no failed job. Structured five-rung slopes of log2 kappa on log2 N: -0.004 to +0.002, largest upper 95% limit 0.0101 against the break-even rate 1/3.
Planted slopes 1/6, 1/3, 1/2 are recovered (0.158, 0.322-0.331, 0.489-0.499) and Z/N reads 0.699 / 0.647, so the instrument sees growth.
Meaning: no growth of the pair-sum coincidence excess of window / rank-d label-GAP symmetric bases over 16-32 bits; the original ladder's valid lines (SUB-BE) are confirmed with valid controls.
Not covered: FAM-1 and bbc53e-base ladders, other families, sizes above 32 bits. Class: structural-lead measurement; prices no end-to-end index calculus.
Raw data: `results/main/main-jobs.jsonl` and `results/pilot/pilot-jobs.jsonl` follow the artifact-store policy (manifest `.artifacts.json`); `results/analysis-main.json` is the full analysis.
