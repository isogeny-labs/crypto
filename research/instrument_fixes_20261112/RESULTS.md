# Instrument fixes and reruns: results (2026-11-12)

Protocol: [`PROTOCOL.md`](PROTOCOL.md) and [`AMENDMENT1.md`](AMENDMENT1.md), both
hashed in [`frozen.sha256`](frozen.sha256) before the recorded run. Data:
[`rerun.json`](rerun.json) (new values, every run aggregated), [`run.log`](run.log),
[`rerun.svg`](rerun.svg); the old-versus-new join is `render_rerun.py`, whose full
output is [`tables.md`](tables.md) and [`summary.json`](summary.json). Code:
`src/cryptanalysis/chained_transfer/instrument.rs`, `src/bin/instrument_rerun.rs`
(plus the additive edits listed under Deviations). Bench record (left open):
`benchmarks/records/20261112T2311Z-instrument-fixes.json`. The whole run took 149 s
of wall clock [pcc:unverified]. Every number is a [pcc:unverified] assertion: no
independent validator has replayed it. Every log of the run (every IC arm,
every rho) certified by an independent `[k]G == T` (the run exited 0 only when all did), and the v3 gate reproduced the
recorded rho, group operations, trials and LA counter exactly at all four primes
[pcc:unverified]. The transfer is in no total.

## Answer

The three fixes do not change any recorded verdict except one, and they move the
distance to the matched rho the wrong way for the transfer: **the recorded gaps were
too small, not too large.** [pcc:unverified]

- **LA counter (fix 1).** The recorded genus-2 counter is exactly `(fb+2) x rows + 1`
  in all four v3 rows. Counting every multiplication of that same elimination gives
  1.001 to 1.02 times the recorded value in multiplications, plus 1.5 to 1.8 times as
  many additions as multiplications; the answer is identical [pcc:unverified]. So for
  the dense elimination the recorded number was a matrix-size count that happened to be
  within 2% of the real multiplication count. The real change is the algorithm in
  arms A to C (below), not the counter.
- **Matched rho (fix 2).** The recorded rho on E (`rho_reference`) spends about 400
  group operations on its jump-table set-up. The tuned negation walk spends 51 to 66
  (v3) and 56 to 79 (tower) and takes 10 to 118 walk operations: totals of 63 to 92 (v3)
  and 69 to 197 (tower) against recorded 357 to 847 and 416 to 1067 [pcc:unverified].
  The recorded matched rho was 5 to 9 times too expensive at v3 sizes and 6 times at
  p = 251 falling to 5.4 times at p = 16001. Every recorded IC/rho distance is divided
  by the corrected reference. The negation total is 0.88 to 0.96 of the plain tuned
  total at every prime, so negation is a small effect against set-up
  [pcc:unverified].
- **Double large primes and walk collection (fix 3).** They lower the IC cost
  substantially in group operations (v3, p = 1619: 34,521 recorded to 2,583 for arm C,
  13.4 times; 5.1 times at p = 211) and at w = 0.1 and 1 (3.0 to 4.0 times at
  p = 1619, versus recorded LA-only pricing). They do not close the gap, because rho
  fell more. [pcc:unverified]

## v3 (genus 2, pull-back instance), distance IC total / negation-map rho total

Recorded = IC / recorded plain rho (one seed). New arms: mean of 8 algorithm seeds,
negation rho mean of 32 seeds. `cal` is informational.

| p | recorded 0 / 0.1 / 1 | L (accounting only) 0 / 0.1 / 1 | A 0 / 0.1 / 1 | B 0 / 0.1 / 1 | C 0 / 0.1 / 1 | C cal | class |
|--:|--|--|--|--|--|--:|--|
| 211 | 7.99 / 11.4 / 42.2 | 54.6 / 76.5 / 233 | 13.2 / 123 / 908 | 12.2 / 77.1 / 542 | 10.7 / 51.9 / 347 | 13.0 | above |
| 419 | 23.7 / 38.3 / 169 | 131 / 205 / 727 | 27.0 / 311 / 2300 | 23.9 / 138 / 940 | 30.7 / 174 / 1173 | 38.2 | above |
| 821 | 30.7 / 75.2 / 476 | 171 / 410 / 2147 | 32.9 / 459 / 3565 | 21.2 / 168 / 1238 | 29.4 / 218 / 1594 | 39.2 | above |
| 1619 | 40.8 / 112 / 754 | 376 / 1000 / 5195 | 39.8 / 1651 / 12474 | 32.6 / 325 / 2293 | 28.1 / 247 / 1715 | 39.5 | above |

[pcc:unverified] Class is "above" for every arm, every p and every price, against the
negation rho, the plain tuned rho and the negation walk alone (class columns in
`tables.md`). The 0-GAE order is unchanged: IC above rho in all 16 arm-rows.
Row L isolates the accounting change: the recorded IC against the corrected rho is
5.5 to 9.2 times its recorded distance at 0 GAE (54.6, 131, 171, 376). The distance
that the algorithm change recovers is the gap between L and A, B, C: at p = 1619 from 376
to 28.1 at 0 GAE. Best arm at 0 GAE: C at 211 and 1619, B at 419 and 821 (the
choice of arm moves the ratio by under 30% except at 1619 where A is 1.4 times C).
Group operations of arm C: 670, 1,993, 1,953, 2,583 (recorded 3,419, 8,475, 11,353,
34,521). Set-up is 247, 250, 218, 334 of them [pcc:unverified].

## tower_large (p up to 16001), C over the negation rho on E

Recorded = `ratio_la` of `tower_large.json` (full base, LA mulmods priced, plain
rho, old walk). C numbers: mean of 3 instances x 4 seeds; rho mean of 3 x 64.

| p | g=2 recorded 0 / 0.1 / 1 | g=2 C 0 / 0.1 / 1 | g=3 recorded 0 / 0.1 / 1 | g=3 C 0 / 0.1 / 1 | C cal (g2 / g3) |
|--:|--|--|--|--|--|
| 251 | 0.67 / 8.77 / 81.7 | 11.0 / 61.7 / 422 | 1.84 / 22.8 / 211 | 12.3 / 143 / 1069 | 13.7 / 20.2 |
| 503 | 1.02 / 20.5 / 196 | 17.5 / 132 / 896 | 3.15 / 35.3 / 324 | 15.1 / 235 / 1698 | 23.7 / 27.9 |
| 1009 | 1.67 / 40.9 / 394 | 25.3 / 219 / 1509 | 5.08 / 55.4 / 508 | 22.5 / 425 / 3105 | 35.5 / 45.3 |
| 2003 | 2.84 / 75.7 / 731 | 31.4 / 348 / 2337 | 8.44 / 114 / 1065 | 30.3 / 730 / 5134 | 48.0 / 69.9 |
| 4001 | 5.62 / 147 / 1419 | 59.4 / 682 / 4559 | 16.4 / 346 / 3317 | 48.0 / 1307 / 9154 | 92.0 / 118 |
| 8009 | 8.4 / 212 / 2046 | 61.9 / 889 / 5652 | 25.5 / 993 / 9698 | 62.7 / 2110 / 13897 | 106 / 177 |
| 16001 | 15.0 / 375 / 3613 | 79.8 / 1309 / 7998 | 44.9 / 4616 / 45755 | 73.5 / 2786 / 17543 | 145 / 224 |

[pcc:unverified] The one recorded verdict that changes: **genus 2 at p = 251 was
"unresolved (order changes inside the band)"** (IC2 at 0.67 times the plain rho at
0 GAE); against the corrected rho it is above at all three prices (11.0 at 0 GAE).
The other thirteen rows stay "above"; so do all 42 arm-rows (A, B, C, both genera, seven
primes) against the negation rho [pcc:unverified]. The C ratio grows with p in both
genera (genus 3: 12.3 to 73.5 at 0 GAE), as do the recorded ones; the corrected
distance is 16 times (g = 2) and 6.7 times (g = 3) the recorded one at p = 251
and 5.3 times (g = 2) and 1.6 times (g = 3) at p = 16001, at 0 GAE.

Against the old IC (total at the same price, old = full base, LA mulmods only), C is
cheaper in total at g = 3 for p >= 2003 (1.9 to 3.3 times at 0 GAE, up to 8.4 times at
0.1 and 1 GAE at p = 16001) and for g = 2 at 0.1 and 1 GAE from p = 1009 (1.3 to 1.5
times); it is dearer for g = 2 at 0 GAE at every p (0.36 to 1.02) and for g = 3 below
p = 1009 at the priced weights [pcc:unverified]. That is the measured improvement of
the index-calculus baseline: it appears for g = 3 and grows with p; it does not reach
rho. The IC3/IC2 ratio, recorded 2.75 to 3.1, is for the best arm per price
0.94 to 1.12 at 0 GAE and 1.78 to 2.46 at 0.1 and 1 GAE, so the claim "the tower
costs a factor of about 3" holds at priced weights only [pcc:unverified].

## small_de (24- and 28-bit, 48 arm-cells)

The IC arms reproduce the recorded totals to 2.2e-16 (relative), and the best of
plain/negation/automorphism rho reproduces the recorded reference exactly
[pcc:unverified]. What changes is the split. The negation walk is 80.5% to 96.4%
of the negation total (80.5 to 91.6% at 24 bits, 91.8 to 96.4% at 28 bits), so
set-up is a minor part here, unlike v3 and tower. With the negation total as
primary reference (instead of the best-of), the minimum distance over the 48 arm-cells
is 1.63, 516, 2947 at w = 0, 0.1, 1 (recorded best-of: 2.32, 516, 4612); 0 of 48
arm-cells are below the negation rho at all three prices, 0 against the walk alone
(minimum 1.74, 576, 3135), 0 against best-of (summary.json). The D = -3, -4 arm-cells
(automorphism rho the recorded reference) move down, by as much as a factor 0.43 at 0 GAE, when the
negation rho is the reference; at w = 1 the negation rho is priced for its canonicalisations,
so the ratio falls by a median 0.68 [pcc:unverified]. Large primes are not ported to this
decomposition; its elimination was already a counted sparse one.

## Verdicts

- Changed: tower genus 2, p = 251, "unresolved" to "above" [pcc:unverified].
- Unchanged: the other 13 tower rows, 4 v3 rows ("above"), 48 small_de arm-cells
  (none below any matched rho) [pcc:unverified].
- Strengthened: the distance remaining is larger than recorded in 50 of 54 v3 and
  tower (row, price) cells at arm C (10 of 12 in v3: the exceptions are p = 1619 and
  p = 821 at 0 GAE, where C is 28.1 and 29.4 against 40.8 and 30.7; 40 of 42 in the tower)
  [pcc:unverified].
- Success criterion: at p = 1619 arm C needs 2,583 group operations against 34,521
  recorded (13.4 times fewer) and 3.0 times fewer at w = 1: a measured improvement of the
  IC baseline. It is an algorithm change (walk collection, large primes), not a
  relabelling. Its gap to the matched rho stays 28.1 / 247 / 1715 at 0, 0.1, 1 GAE
  [pcc:unverified]. The accounting change alone (arm L, rho) is not an improvement and
  made the gap larger.

## Prediction scorecard (frozen in PROTOCOL.md)

1. P1 (LA real count 2 to 60 times the recorded): **refuted.** 1.001 to 1.02 times in
   multiplications. Seen in the smoke test before the recorded run (AMENDMENT1).
2. P2 (C over negation rho at 0 GAE within 1 to 8): **refuted.** 10.7, 30.7, 29.4, 28.1.
   Group operations fell as predicted; the reference fell more.
3. P3 (IC stays above rho at all prices in v3): **holds.**
4. P4 (C at w = 1 above 10): **holds** (347 to 1715).
5. P5 (negation 0.7 to 1.3 of plain; walk share under 0.6 up to 16001): **holds**
   (0.89 to 0.95; 0.598 at 16001, just under).
6. P6 (g = 3, w = 0: A fewer group operations than C; w = 1: C below A): **refuted
   in its first half**: C needs fewer group operations at every p (845 against 1023 at
   251 and 14,457 against 24,526 at 16001); second half holds.
7. P7 (g = 2, C/A group operations 0.3 to 1.2 at p >= 4001): **holds** (0.72, 0.68, 0.64).
8. P8 (IC3 stays above; IC2 at 251 not below; IC3/IC2 for best arm 2 to 4 at 0 GAE):
   first two **hold**, third **refuted** (0.94 to 1.12).
9. P9 (small_de unchanged, no verdict change; walk-only 5 to 60% lower at 24 bits and under
   10% at 28): **holds** for the best-of reference and the walk shares (8 to 20% at 24
   bits, 3.6 to 8.2% at 28); the wording "differs by under 5% in every arm-cell" was
   written for the best-of reference and fails for the negation primary at D = -3, -4
   (flawed wording, not edited).
10. P10 (no row below the matched rho at all prices): **holds.**

## Deviations

- Edits outside `instrument.rs` and the binary: `ic_control.rs` (the `deg == 3` test
  reads `deg == genus`; `run_ic_with(inst, params, restart)` added, `run_ic` calls it with
  the old 4096), `measure.rs` (additive `genus2_collect_rows`), one line in `mod.rs`.
  The existing tests of `chained_transfer` pass; the positive control's genus-3
  behaviour is unchanged (restart default 4096).
- Amendment 1: restart every `max(4, sqrt r)` steps in arms A to C (the default 4096
  wasted 99% of trials in a subgroup of a few hundred elements). The smoke test was run
  before the amendment and its v3 numbers were seen (disclosed in AMENDMENT1).
- Native work of arms A to C includes the relation-phase smoothness tests; the recorded
  tower and v3 IC counted LA mulmods only. At w > 0 this makes the new IC dearer by
  construction and the old-versus-new comparison at those prices is not like for like.
- Tower rerun stops at p = 16001 as instructed; 32003 and 64007 not rerun.
- A group operation is 1 GAE for every group type; a Cantor step costs several elliptic
  additions, so IC/rho ratios understate the gap.
