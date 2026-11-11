# Correction results: v2 IC4 vs the fixed rho reference

Protocol: `PROTOCOL.md` (frozen in 75dbc70b36 before data). Rerun: every rho-family job of the v2 holdout,
1584 runs with the fixed binary, **0 errors, every target verified**. `results/rows.jsonl` (new rho rows),
`results/merged.jsonl` (v2 `ic4` and `bsgs2` rows + new rho rows), `results/analysis.txt` (v2 `analyze.py`
unchanged), `results/summary.json`. All numbers [pcc:unverified] until the claim is signed.

## IC4 / rhobest (median per-configuration ratio, range over c_I in {3, 10, 30} and h in {0, 0.1, 1})

| bits | T | v2 as published | corrected | v2 decision (all c_I) | corrected decision (c_I = 3 / 10 / 30) |
|---|---|---|---|---|---|
| 22 | 1024 | 1.053-1.122 | 1.074-1.133 | dearer | dearer / dearer / dearer |
| 24 | 1024 | 1.094-1.188 | 1.123-1.210 | dearer | dearer / dearer / dearer |
| 24 | 4096 | 0.961-1.021 | 0.985-1.038 | unranked | unranked / unranked / dearer |
| 24 | 16384 | 0.918-0.949 | 0.951-0.984 | **cheaper** | **cheaper / cheaper / cheaper** |
| 25 | 4096 | 0.972-1.038 | 0.997-1.057 | unranked | unranked / dearer / dearer |
| 25 | 16384 | 0.908-0.944 | 0.951-0.991 | **cheaper** | **cheaper / cheaper / cheaper** |

IC4 / bsgs2 is unchanged (1.18-1.80; neither arm's code changed).

## What changed and what did not

- The defect made the rho reference 1.4% to 4.8% too expensive (rhobest cost, per cell, c_I and h); the corrected reference is cheaper in every cell.
- **The v2 headline survives**: IC4 is still cheaper than the cheapest of four rho arms at T = 16384 for every
  c_I and h, but by 1.6% to 4.9% (median ratio 0.951-0.991), not 5% to 9%. At 25 b, c_I = 30, h = 1 the margin is
  0.9%.
- At T = 4096 three (cell, c_I) decisions move from unranked to dearer.
- The pre-registered prediction (T = 16384 moves to about 1.0, unranked) was wrong in direction of size: the margin
  shrank by about half but stayed below 1 at every h.
- The shared verification constant (about 32 to 50 GAE per target, identical in every arm) still compresses every
  ratio toward 1; without it the T = 16384 margins are larger (reported in the v2 RESULTS, recomputed for the
  corrected rows by `../mt_fieldops_v2_20261111/analyze.py`).

## Consequences

- Claim PCC-20261111-fa13c8 (v2) is superseded for its rho numbers by the claim recorded with this correction;
  the leaderboard component section and `benchmarks/20261111-mt-fieldops-ic4/README.md` carry the corrected range.
- The v3 holdout (`../mt_fieldops_v3_20261111/`, amendment A1) runs entirely on the fixed binary.
