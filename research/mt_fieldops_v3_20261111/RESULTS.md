# Multi-target IC v3 results: the k-term table (ic5) is cheaper than the strongest rho in every cell

Protocol: `PROTOCOL.md` (frozen in f71b42cc4f; amendment A1 after the rho-restart fix 75dbc70b36, before any holdout
result). ic5 parameters frozen from the tune set in `frozen_ic5.json` (mu = 1.0, k = 4) before the holdout.
Holdout: 44 fresh prime-order curves (indices 1..11 at 22, 24, 28, 32 bits), seeds 21 and 22, T in {1024, 4096,
16384}: **4488 runs, 0 errors, every target of every run verified**. Data: `results/holdout/rows.jsonl` (artifact
store), `results/holdout/runs.json` (compact counters in Git), `analysis.txt`, `summary.json`, `noverify.txt`.
Every number here is [pcc:unverified] until the claim is signed.

Cost per target in GAE = [(c_I I + M + S)/(c_I + 3) + h (H + L)] / T. Each ratio is the median per-configuration
ratio (22 configurations per cell); ranges are over c_I in {3, 10, 30} and h in {0, 0.1, 1}. rhobest = per
configuration and price, the cheapest of 12 rho runs (rho, rho2, rho2x, rhoseq at kappa 0.5, 0.7, 1.0) after the
restart fix; bsgs2 = best of three mf.

## Main table

| bits | T | ic5/rhobest | ic5/bsgs2 | ic5/ic4 | ic4/rhobest | ic4 vs rhobest (c_I 3/10/30) |
|---|---|---|---|---|---|---|
| 22 | 1024 | 0.919-0.963 | 1.275-1.361 | 0.830-0.894 | 1.071-1.130 | dearer / dearer / dearer |
| 22 | 4096 | 0.931-0.972 | 1.181-1.278 | 0.927-0.970 | 0.984-1.016 | unranked / unranked / unranked |
| 22 | 16384 | 0.931-0.982 | 1.112-1.197 | 0.969-0.992 | 0.960-0.990 | cheaper / cheaper / cheaper |
| 24 | 1024 | 0.915-0.975 | 1.344-1.405 | 0.786-0.840 | 1.125-1.213 | dearer / dearer / dearer |
| 24 | 4096 | 0.912-0.962 | 1.243-1.334 | 0.902-0.944 | 0.987-1.042 | unranked / dearer / dearer |
| 24 | 16384 | 0.925-0.968 | 1.158-1.255 | 0.958-0.984 | 0.954-0.988 | cheaper / cheaper / cheaper |
| 28 | 1024 | 0.875-0.959 | 1.430-1.447 | 0.634-0.662 | 1.360-1.502 | dearer / dearer / dearer |
| 28 | 4096 | 0.870-0.946 | 1.367-1.410 | 0.831-0.863 | 1.037-1.134 | dearer / dearer / dearer |
| 28 | 16384 | 0.880-0.945 | 1.283-1.358 | 0.922-0.949 | 0.944-1.019 | cheaper / unranked / unranked |
| 32 | 1024 | 0.874-0.975 | 1.469-1.491 | 0.473-0.483 | 1.826-2.060 | dearer / dearer / dearer |
| 32 | 4096 | 0.854-0.952 | 1.423-1.432 | 0.707-0.723 | 1.190-1.338 | dearer / dearer / dearer |
| 32 | 16384 | 0.848-0.942 | 1.374-1.402 | 0.862-0.879 | 0.974-1.090 | unranked / dearer / dearer |
Section 5a decisions (bootstrap 95% interval of the median ratio at all three h):

- **ic5 vs rhobest: cheaper in all 36 (cell, c_I)**, from 22 to 32 bits and T = 1024 to 16384.
- ic5 vs ic4: cheaper in all 36. ic5 vs bsgs2: dearer in all 36 (1.11x to 1.49x).
- ic4 vs rhobest: cheaper only at T = 16384 for 22 and 24 b (and 28 b at c_I = 3); dearer at every T = 1024 cell
  and from 28 b at T = 4096; at 32 b, T = 16384 unranked or dearer.
- ic4 vs bsgs2: dearer in all 36.

Without the verification phase that every arm shares (about 30 to 50 GAE per target), at I = 10 M
(`noverify.txt`): ic5/rhobest 0.883-0.962, ic5/bsgs2 1.42-1.53, ic4/rhobest 0.92-2.04.

## Why ic5 wins where ic4 did not (counted phase split, `rows.jsonl` phases)

ic4 needs about 2m relations for m ~ sqrt(table) unknown factor-base logs. ic5 with k = 4 reaches the same table
size from m ~ table^(1/4) logs: at 32 b, T = 1024 it has m = 48 unknowns and needs about 61 relations, against
ic4's m = 1218 and 2436 relations, so its relation phase is a few per cent of the run. What is left is the table (one shared
inversion per two x-classes) and the paired target trials, and the measured trials per target match n/(2K)
(32 b, T = 1024: 1396 measured vs 1380 predicted with K = 1 555 415 classes). ic5's advantage over ic4 therefore
grows with the group and shrinks with T (0.47 at 32 b, T = 1024; 0.86-0.99 at T = 16384).

Against rho the remaining gain is constant-factor: x-only classes from shared inversions in both the table and the
target trials, against rho's walk steps. It does not change the exponent, and BSGS, which tiles its windows, stays
1.1x to 1.5x cheaper (`../mt_bsgs_gap_20261111/README.md`).

## Pre-registered predictions vs outcome

| prediction | outcome |
|---|---|
| ic5/ic4 < 1 everywhere, largest gain at T = 1024 (about 0.85) | holds; the gain is larger than predicted and grows with size (0.47 at 32 b) |
| ic5/rhobest cheaper at T = 4096 and 16384 at every size | holds |
| ic5/rhobest at T = 1024 about 0.92, cheaper or unranked | cheaper in every size (0.87-0.98) |
| ic4/rhobest at 28 and 32 b: cheaper at 16384, dearer at 1024 | dearer at 1024 holds; at 16384 only 28 b at c_I = 3 is cheaper, 32 b is unranked or dearer (wrong) |
| both dearer than bsgs2 everywhere | holds |

## Answers to the questions

1. (option 2, small T) Yes: ic5 moves the crossover against the strongest rho down to T = 1024 at every size tested.
2. (option 1, larger groups) ic4's v2 pattern does not hold at 28 and 32 b: its relation phase grows like sqrt of
   the table, and it falls behind rho except at T = 16384. ic5 holds or improves its margin with size (the largest
   gains, 0.85-0.88, are at 28 and 32 b).

## Limits

Groups of 2^22 to 2^33; costs counted in field operations, not timed; memory unbounded for every arm; one local
host (this container has no AWS identity, so not isolab-eks); the rho family is this harness's, not ecbench's, and
was fixed during this study (`../mt_rho_start_fix_20261111/`). The comparison is per target in a batch of T
targets on one curve, the regime in which multi-target methods apply; it says nothing about single-target ECDLP.
The ic5 tune set chose k = 4, the largest k allowed; k = 5 was not tried.
