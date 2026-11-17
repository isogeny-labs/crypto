#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: Phase B hold-out, 2026-10-19 22:02 – 2026-10-20 00:01 UTC

*Study* `research/bitop_ladder_20261019` (`PROTOCOL.md` Phase B).

*Commands* `run_sessions.sh` via task cards 7 (P39, P40) and 8 (P16–P38), `cards/CARD-bitop_ladder-20261019-{7,8}.yaml`, with receipts.

*Binary* sha256 `372ed2eb2f19c5fd17168188c68cae57abc581edbf4d2a07ad0e3e8a0950a21a`.

*Host* env class `ECBENV2he1389885ccd0`: Intel Xeon \@ 2.10 GHz, 4 vCPU. Isolation L0.

*Workloads*
- Curves: the 25 curves of `curves.jsonl`.
- Targets: 80 public targets a curve, target seed 20261019200.
- Measurement seed: 20261019201.
- Specs: `specs/holdout-P*.json`, generated from `params.json`.

*Arms*
- `rho-neg` (reference) and `rho-neg-aa`, which is identical by seed (Deviation 4);
- `bsgs-neg` and `kangaroo`;
- `ic-pair-rule`;
- `ic-pair-b32` and `ic-rootfind-m3-rule`, 16–28 bits only.

*Runs*
- 25 sessions and 12,080 runs, all verified and all completely priced in bitop/v1.
- Every session passes `ecbench verify --replay 2` (`verify.json` `ok`).
- No timeouts, no exhaustions, no crashes.

*Measured values*
- Each arm's mean bitop/v1 relative to rho per curve: `analysis/gap_to_rho.json`.
- Fits: `analysis/bounds/`.
- Comparisons: `analysis/compare/`, `analysis/table.txt` and `analysis/band.txt`.

#figure(image("gap_to_rho.svg", width: 95%), caption: [Mean bitop/v1 over rho-neg's, per curve, 80 targets each. Sessions `sessions/holdout-P16` … `-P40`.])

*Raw records* `sessions/holdout-P*/records.jsonl`.
