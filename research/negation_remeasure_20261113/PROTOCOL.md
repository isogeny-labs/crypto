# Negation-map re-measurement of three chained-transfer targets: protocol (2026-11-13)

Owner-approved task, follow-up of `research/validation_20261112/REPORT.md`.
Frozen by `frozen.sha256` before the first run of `negation_remeasure`.
Every prediction and every number named here is `[pcc:unverified]`.

## Question

The independent validation admitted `small_de` and refused three targets for one
reason: their matched rho was plain `rho_reference` (A = 1, no negation) although
the negation map is available, and rho's native work (canonicalisations,
partition hashes, DP-table operations) was not emitted. With the strongest
available matched rho in the same process and every native counter emitted on
every arm, what is the distance of each recorded IC arm and of each improved arm
to the matched rho, and does any recorded verdict change?

## Targets

1. **v3** (`chained_transfer_bench`): `p = 211, 419, 821, 1619`; the same
   candidate search, the same instance and seed (`0xC0FFEE + 101 i + s`), the
   pulled-back pair `(phi^*(P), phi^*([x]P))` on `J(C)`, certificates.
2. **tower_large**: genus 2 and genus 3 rungs of `build64(p, 1000 + 17 i + p mod
   997, p/4, 64)`, 3 instances per prime, `p = 251, 503, 1009, 2003, 4001, 8009,
   16001`, then `32003` and `64007` in a second stage when the budget of about 60
   minutes of wall clock allows (a stage that does not finish is reported as not
   run, never extrapolated).
3. **cover_pushforward** (degree 2): the primes of the original run `211, 419,
   821, 1619, 3203, 6421, 12841`, the same curve per prime and the same seeds
   (`seed0 ^ (k 0x1000)`, `k < 3`).

## Arms

Per row, in one process:

- **Recorded IC arms, exactly as measured** (same code path, same seeds):
  v3 `genus2_index_calculus` (counters: group ops, `la_field_ops`);
  tower `hyper64::ic::solve` with `crosscheck::ic_params` (full degree-1 factor
  base, 8 algorithm seeds `300 + s`), every emitted unpriced counter;
  cover_pushforward `ic_jc` (`collect_jc` + `solve_rows`), `ic_e1_fibre_full`,
  `ic_e1_two_term`, `ic_jc_first_collision`, `ic_e1_fibre_first_collision`,
  `birthday_e1_first_collision` (the control).
- **Improved arms** (`instrument.rs`, protocol
  `research/instrument_fixes_20261112/PROTOCOL.md` plus its Amendment 1):
  `L` (v3 only: the recorded relations with the counted elimination, an
  accounting change), and `A` (every degree-1 place), `B` (single large prime),
  `C` (double large prime) via `ic_on_rung` on the word Jacobian, for v3, tower
  (both genera) and cover_pushforward `J(C)`; 8 algorithm seeds `500 + s` (v3,
  cover), 4 (tower `300 + s`).
- **Matched rho, in the same process, on the same `E`**: the tuned negation-map
  walk (`RhoWalk::negation`, primary), the tuned plain walk, and the recorded
  plain `rho_reference`/`rho_on_ec` (kept as the "old" column). 32 seeds (v3,
  cover), 64 (tower). The automorphism fold is run on every curve whose
  `j` is `0` or `1728` (the cubic or quartic walk); the run lists every curve it
  checked and what it found. A curve with such a `j` for which the fold is not
  run is reported as an implementation gap.

Rho counts every group operation (set-up, walk starts, walk, verification) and
emits, per run: `canonicalisations` (the walk's own counter), `partition_hashes`
(`key` plus mix evaluations that pick the jump and test the distinguished
bit), `dp_lookups`, `dp_inserts`, and `key_evaluations` (all `CountedGroup::key`
calls, counted by a wrapper, the exact total from which `partition_hashes` is
derived). The recorded walks are not modified.

## Pricing

Unit: one group operation (add or double) of the arm's own group, 1 GAE. A
Cantor step costs several elliptic additions; every genus-2 or genus-3 over
elliptic ratio understates the gap and is labelled so.

Native counters priced: IC `rel.mul + la.mul` (additions as a labelled
sensitivity) and, where emitted, the old arms' own counters (v3 `la_field_ops`;
tower `la_mulmods + la_inversions + oracle_fp_muls + factor_base_fp_muls +
factor_base_lookups + canonicalisations + step_draws`; cover `la_mulmods`,
`lookups`); rho `canonicalisations + partition_hashes + dp_lookups +
dp_inserts`. Every arm's total is `group_ops + w * native`.

- **Band** (AGENTS.md 5a): `w = 0, 0.1, 1` on every native counter of every arm.
  Class of an arm against the matched rho: `below` (below at all three),
  `above` (above at all three), `unresolved`.
- **Calibrated weight** (AGENTS.md 5a, bullet of 2026-11-12): `w_cal` is the time
  of one native operation divided by the time of one group addition of the same
  group on the same host, by **best-of-N timing** (N = 25 batches of 20,000
  operations, the minimum batch taken; the host is a shared, unisolated VM and
  the label says so; no `perf-ipc` here). It is measured per prime and per
  group (E, genus 2, genus 3) for the mulmod mod `p` (relation phase), the
  mulmod mod `r` (linear algebra), and for each rho native operation
  (canonicalisation, partition hash, DP-table lookup, DP-table insert), so the
  calibrated price is applied to **both** sides (a calibrated cost is never
  compared with an uncalibrated one). Order reported at `w = 0`, at `w_cal` and
  at `10 w_cal` (every native class scaled by 0, 1, 10 times its own `w_cal`);
  "ranked at calibrated price" only where the three agree. The band verdict
  still governs.
- **Arms with an unemitted counter** (the relation-phase work of the v3 and
  cover recorded collectors, hash work of the cover first-collision arms): the
  arm is a lower bound. A lower-bound arm is classified `above` only when its
  lower bound is above the matched rho at the price; otherwise
  `unresolved (lower bound)`, never `below`. The missing emission is listed as
  an implementation gap.
- A byte-valued counter is listed, never priced.
- The transfer (tower build, pull-back, cover) is one-time, in no per-instance
  total, and reported in its own column.

## Certification

Every recovered log of every arm (IC and rho, old and new) is certified by an
independent `[k]G == T` (`Curve::mul` on `E`/`E1`, `mul_rtl` on the word
Jacobian, `Genus2Curve::mul` on the v3/cover Jacobian). An uncertified log is a
failure that is listed; the binary exits 0 only when none occurred. A
**reproduction gate** runs first on each target: the recorded rows
(`results_v3.json` group ops, trials and LA counter; `tower_large.json`
per-instance `rho_ops` and `ic` counters for the instances measured;
`pushforward.json` per-seed arm group ops) must be reproduced exactly by the
in-process "old" arms, else the section is flagged `gate_failed` and its
verdicts are not reported as like-for-like.

## Old versus new

- **Old distance**: recorded IC total (its emitted native counters, LA only in
  v3, as recorded) over the recorded plain old rho, per price, as in the
  original files (`results_v3.json`, `tower_large.json` `ratio_la`,
  `pushforward.json` `ratio_w*`).
- **New distance**: the arm's total over the matched negation rho total
  (native priced on both sides), per price, plus the calibrated reading.
- A change of class (old vs new) is reported with both ratios. A fall of a
  distance that comes from a counting change (arm `L`, or the rho now priced)
  is stated as such and is not an improvement. A fall that comes from the
  algorithm doing less work (arms `A`, `B`, `C`) is a measured improvement
  **only** where the improved arm's emitted-counter total is below the recorded
  arm's lower-bound total at all three prices; the distance remaining to the
  matched rho is reported in every case.

## Frozen predictions

N1. **Verdicts.** No arm (recorded or improved, any target, any row) is below
the negation rho at all three band prices; every recorded-arm row is `above`
or `unresolved (lower bound)`. P = 0.92. Falsified by one `below` row.

N2. **Native work of rho.** Per run, `native / walk steps` is between 1.8 and
3.5 for the negation walk and between 0.9 and 1.6 for the plain walk, at every
prime of every target (mean over seeds). P = 0.8.

N3. **Cost of the walk's native work.** The negation rho total at `w = 1`
exceeds its total at `w = 0` by a factor between 1.3 and 4 at every v3 and
tower prime, and the recorded `rho_on_ec` plain total is 4 to 10 times the
negation total at `w = 0` at v3 sizes (the instrument-fixes finding, 5 to 9).
P = 0.75.

N4. **Distances move up.** The recorded arm's distance at `w = 0` to the
negation rho (v3 recorded collector, cover `ic_jc`) exceeds its recorded
distance to the old plain rho in at least 9 of the 11 rows (4 v3 primes, 7
cover primes), and the ratio new/old of the cover `ic_jc` distance at `w = 0` is
between 3 and 12 at every cover prime. P = 0.8.

N5. **Calibrated reading.** The calibrated order agrees with the band verdict
(`above` at 0, `w_cal` and `10 w_cal`) for every recorded-arm row and for arm
`C` at every row. P = 0.9. The mulmod-mod-`p` `w_cal` on the genus-2 Jacobian is
between 0.002 and 0.03 at every prime. P = 0.7.

N6. **Aut fold.** No curve of the 32 (4 v3, 21 tower, 7 cover) has `j = 0` or
`1728`. P = 0.93. Falsified by one curve; then the fold is a gap or is run.

N7. **Improvement.** For v3 at `p = 1619` and for tower genus 3 at `p = 16001`
arm `C` (or the best of `A, B, C`) is below the recorded arm's total at all
three prices, so the improvement over the baseline reproduces
(instrument-fixes 13.4 times fewer group ops at v3 `p = 1619`). P = 0.8. For
cover `ic_jc` at `p = 12841` the same, P = 0.8.

N8. **Budget.** `p = 32003` completes (one or more instances) within the
budget; `p = 64007` completes at least one instance. P = 0.5.

N9. **Growth.** The distance of arm `C` at `w = 0` increases with `p` in both
genera across the tower primes that complete (rank correlation with `p` of at
least 0.7). P = 0.7.

## Decision rule

Per (target, row, arm): class at the band and at the calibrated points against
the negation rho (against the best of negation and aut where an aut fold was
run). Report old class, new class, both ratios. A recorded verdict **changes**
when the class differs. Report the scorecard of N1 to N9 (holds / refuted by
its stated falsifier), the 5a band, the calibrated reading, and for every
target whether the evidence now has: a matched negation rho in the same process,
rho's native counters emitted, every IC arm's counters listed (with gaps), every
log certified, `w_cal` recorded with its method and host. Admission to the gate
is the validator's call, not this run's. All prose claims in RESULTS.md carry
`[pcc:unverified]`.
