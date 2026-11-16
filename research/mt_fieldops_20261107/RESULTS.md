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
  cells) and at T = 16384 for every I/M; dearer at T = 1024 except 22 b at I = 3M (unranked); unranked
  otherwise. (Amended after validation, see below: `rho2` paid +1M per step for the sibling's y.) IC4's remaining edge over
  rho2 is that its target trials need no y-coordinate and its progression W_k is shared by all targets.
- vs `bsgs2` (sum-set baby table, paired giants): **IC4 dearer in all 18 combinations, 1.18x to 1.80x.**
  BSGS tiles the group, so a target needs about half the trials of any random-coverage method (IC or rho).

## What changed relative to the closed work stream

In group additions (`research/ic_e222_m221_20261106/RESULTS_multi_target.md`) IC v3 was 1.03 to 1.07 times the
anchored batch rho at large T (price 0) and 1.40 to 1.53 times a multi-target BSGS. The anchored rho here is the
same algorithm and lands at the same cost (142.6 GAE/target at 24 b, T = 4096 here against 142 counted additions
there), so the cross-instrument comparison is sound in that cell: IC4 is about 0.69x IC v3 (104 vs 151). The
saving is a real reduction of field work (one inversion per two x-classes, no y on the table or the trials,
shared progression), not a relabelling. A form of the lever was offered to both generic arms and every
operation is counted by the same layer for all four, but `rho2` received a costlier form than IC uses (it
computes the sibling's y); see the validator amendments below.

## Distance remaining (success criterion)

- To the matched (anchored) rho: IC4 / rho 0.62x to 0.83x at h <= 0.1 and up to 1.006 at h = 1; ranked
  cheaper in 17 of 18 (cell, I/M); 14 of 18 against the validator's sequential-database rho control.
- To a rho with the same arithmetic: 0.86x to 1.19x across the band; ranked cheaper only at large T
  (only T = 16384 against the validator's x-only `rho2x` control).
- To the strongest generic arm measured (paired BSGS): **1.18x to 1.80x remaining** with the shared
  verification cost included, **1.50x to 2.05x** with it excluded. This is the honest
  bottom line: the IC here is a generic algorithm (it uses no property of the curve beyond the group law), so
  it cannot beat the best generic multi-target method by more than constants, and in these runs it does not.

## Limits

Toy groups (2^22 to 2^26), T >= 1024; u64 field so I, M, S are counted, not timed (the I/M band 3 to 30 spans
batched to unbatched inversion); memory unbounded for every arm; one rho family (anchored r-adding; a
distinguished-point rho was not built); the IC tune grid's g = 2.0 is at its upper edge; the holdout instances
are the ones the earlier IC versions were developed on (disclosed); producer-written counters with no
independent validator yet; no isolab-eks / S3 / ecdb / Conductor / research-kb in this container.


## Prediction vs outcome (pre-registered in PROTOCOL.md)

- IC4 / rho predicted 0.55 to 0.75 at I = 3M: observed 0.70 to 0.81 at h = 0 (above the predicted range at
  T = 1024; the verification constant, 33 to 38 GAE/target in every arm, pulls every ratio toward 1).
- IC4 / rho2 predicted 0.85 to 1.0: observed 0.86 to 1.02 at h = 0 (within, except 24 b T = 1024).
- IC4 / bsgs2 predicted 1.3 to 1.5: observed 1.20 to 1.71 at h = 0 (wider than predicted on both sides).

## Validator amendments (2026-11-07, `results/holdout/VALIDATOR_REPORT.md`; accepted)

The independent validator replayed all 1320 holdout and 135 tune rows (0 mismatches; binary SHA matches; a
scratch build of the frozen source also reproduces all 1455), checked 172,032 recovered logs with independent
code (0 wrong; a corrupted log lowers `ok` by exactly 1), confirmed the freeze order and the shared-inversion
formulas (0 mismatches over about 15,000 random pairs against its own Jacobian arithmetic), and reproduced
every table, median and section-5a verdict exactly. Dispositions: freeze order, replays, correctness and
tables **validated**; counting fairness **validated with amendments**. Accepted amendments:

1. **Verification constant (undisclosed before).** The counted verification is the same 33 to 38 GAE per
   target in every arm, 15% to 64% of each reported cost; it compresses every ratio toward 1. Without it the
   gap to `bsgs2` is 1.50x to 2.05x.
2. **`rho2` was handicapped.** It computed the sibling's y (+1M per step), which IC never needs. With the
   validator's x-only control `rho2x` (396 runs, all verified; scratch, not pre-registered) IC4 is cheaper
   than rho-with-the-lever only at T = 16384, unranked at T = 4096 and dearer at T = 1024, at every I/M.
3. **The matched rho is a weak rho.** The earlier campaign's validator already found the anchored rho 8% to
   14% above a sequential-database rho. With the validator's `rhoseq` control (5% to 12% cheaper than `rho`;
   scratch, not pre-registered) IC4 is ranked cheaper in **14 of 18** (cell, I/M), and stays cheaper at
   every T >= 4096 cell at every I/M.
4. Wording corrected above ("dearer at T = 1024"; the rho2 range covers the whole band; no pass/fail word for
   the rho distance; "same lever ... counted the same way"). The commit message "17/18 cells" means 17 of 18
   (cell, I/M) combinations. REPORT.pdf previously truncated the Limits section; it now renders the full text.
5. Binding gaps: rows carry no dirty-tree flag, toolchain or host; the raw rows were first committed to git
   and then moved to the artifact store by the bot (hashes identical).

**Net statement after validation.** IC4 is a measured, independently replayed reduction of the multi-target
IC's field work (about 0.69x IC v3 in the comparable cell) and is cheaper than the repository's anchored batch
rho in 17 of 18 (cell, I/M) combinations, 14 of 18 against a sequential-database rho, robustly at T >= 4096.
It is not cheaper than a rho given the x-only form of the same lever except at T = 16384, and it is 1.18x to
2.05x the paired BSGS. Beating the strongest generic multi-target attack remains open; this IC is generic, so
only constants are in play. [pcc:unverified]
