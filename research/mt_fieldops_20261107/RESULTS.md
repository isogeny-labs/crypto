# Multi-target IC in field operations: holdout result (2026-11-07)

Protocol: `PROTOCOL.md` (commit 449d6f14e6, 2026-11-07T06:17:38+00:00, before any data); frozen IC parameters `frozen.json`
(mu = 1.0, g = 2.0; commit 91766533a8, 06:26:10Z, before the holdout). Raw rows `results/holdout/rows.jsonl`
(1320 runs, every target of every run verified by counted scalar multiplication, 0 errors, each row carries
argv, commit and binary SHA-256); tables `results/holdout/analysis.txt`; summary `summary.json`; figure
`ratios.svg` / `ratios.png`; report `REPORT.pdf`. Standing: **[pcc:unverified]**, pre-gate (no validator
report, no replay certificate, no isolab-eks run yet).

## Headline

Cost per target in GAE (1 GAE = one affine addition I + 2M + S), median over 22 paired configurations per
cell; ratio median [bootstrap 95%]; `h` = price of a hash or solver operation in GAE.

| cell | IC4 / rho at I = 3M, h = 0 / 0.1 / 1 | IC4 / rho, I = 30M, h = 0 / 1 | IC4 / rho2, I = 3M, h = 0 / 1 | IC4 / bsgs2, I = 3M, h = 0 / 1 |
|---|---|---|---|---|
| 22 b, T = 1024 | 0.805 / 0.829 / 0.961 | 0.696 / 0.900 | 0.981 / 1.067 | 1.52 / 1.64 |
| 24 b, T = 1024 | 0.800 / 0.834 / 1.006 [0.985, 1.021] | 0.688 / 0.946 | 1.017 / 1.136 | 1.71 / 1.80 |
| 24 b, T = 4096 | 0.729 / 0.750 / 0.862 | 0.648 / 0.816 | 0.907 / 0.979 | 1.37 / 1.48 |
| 24 b, T = 16384 | 0.739 / 0.749 / 0.808 | 0.679 / 0.772 | 0.881 / 0.910 | 1.20 / 1.31 |
| 25 b, T = 4096 | 0.711 / 0.735 / 0.859 | 0.623 / 0.811 | 0.899 / 0.979 | 1.44 / 1.53 |
| 25 b, T = 16384 | 0.701 / 0.714 / 0.787 | 0.636 / 0.749 | 0.861 / 0.900 | 1.25 / 1.35 |

Section 5a ranking under the pre-registered rule (95% upper bound < 1 at h = 0, 0.1 and 1):

- **vs the anchored batch rho (`rho`, the reference the earlier campaign matched): IC4 cheaper in 17 of the
  18 (cell, I/M) combinations, unranked in one (24 b, T = 1024, I = 3M, where h = 1 gives 1.006
  [0.985, 1.021]).** The pre-registered headline "cheaper than rho in every cell at every I/M" therefore
  narrowly fails; it holds at I = 10M and 30M in all six cells and at I = 3M in five.
- vs `rho2` (rho given the same shared-inversion lever): cheaper at T = 4096 and 16384 for I = 3M (all four
  cells) and at T = 16384 for every I/M; dearer at T = 1024; unranked otherwise. IC4's remaining edge over
  rho2 is that its target trials need no y-coordinate and its progression W_k is shared by all targets.
- vs `bsgs2` (sum-set baby table, paired giants): **IC4 dearer in all 18 combinations, 1.18x to 1.80x.**
  BSGS tiles the group, so a target needs about half the trials of any random-coverage method (IC or rho).

## What changed relative to the closed work stream

In group additions (`research/ic_e222_m221_20261106/RESULTS_multi_target.md`) IC v3 was 1.03 to 1.07 times the
anchored batch rho at large T (price 0) and 1.40 to 1.53 times a multi-target BSGS. The anchored rho here is the
same algorithm and lands at the same cost (142.6 GAE/target at 24 b, T = 4096 here against 142 counted additions
there), so the cross-instrument comparison is sound in that cell: IC4 is about 0.69x IC v3 (104 vs 151). The
saving is a real reduction of field work (one inversion per two x-classes, no y on the table or the trials,
shared progression), not a relabelling: the same lever was offered to both generic arms and is counted the same
way for all four.

## Distance remaining (success criterion)

- To the matched (anchored) rho: passed, by 0.62x to 0.83x at h <= 0.1 (17/18 ranked).
- To a rho with the same arithmetic: 0.86x to 1.11x; IC wins only at large T.
- To the strongest generic arm measured (paired BSGS): **1.18x to 1.80x remaining.** This is the honest
  bottom line: the IC here is a generic algorithm (it uses no property of the curve beyond the group law), so
  it cannot beat the best generic multi-target method by more than constants, and in these runs it does not.

## Limits

Toy groups (2^22 to 2^26), T >= 1024; u64 field so I, M, S are counted, not timed (the I/M band 3 to 30 spans
batched to unbatched inversion); memory unbounded for every arm; one rho family (anchored r-adding; a
distinguished-point rho was not built); the IC tune grid's g = 2.0 is at its upper edge; the holdout instances
are the ones the earlier IC versions were developed on (disclosed); producer-written counters with no
independent validator yet; no isolab-eks / S3 / ecdb / Conductor / research-kb in this container.
