# Open case O3, rerun with the transferred order bound to p: protocol (2026-11-14)

Frozen by `frozen.sha256` before the first measured run. Before freezing, only
the unit tests (`cargo test --release --lib chained_transfer::full_jacobian`)
and `full_jacobian_ic --select-only` (cover selection only: no arm, no rho, no
timing) were run; the selection table below is that output. Every prediction
and every number named here is `[pcc:unverified]`.

## Why a rerun

The 2026-11-13 run (`research/full_jacobian_ic_20261113/`, record
`benchmarks/records/20261113T1722Z-full-jacobian-ic.json`) found every whole-J
IC cell above the matched negation rho on `E1`, but its frozen rule returned
"unresolved": the 95% lower ends of the `w = 0.1` ratio slopes were −0.07 (A)
and −0.65 (B). The cause named there: the selection did not bind the
transferred prime order `r` to `p` (`r` from 71 to 3191, `h = #J / r` from
452 to 5.1e6), and rho on `E1` costs about `sqrt(r)`, so the denominator of
the ratio moved independently of `p`. This rerun changes **only the cover
selection**; algorithm, arms, seeds, references, pricing and the decision
rule are those of `research/full_jacobian_ic_20261113/PROTOCOL.md`.

## Selection (the one change)

`full_jacobian_ic --min-r-frac 0.4` (`cover_select_rp` in
`src/cryptanalysis/chained_transfer/full_jacobian_ic.rs`): for the prime with
list index `i`, the same 4000 candidates `candidate_coeffs(p, s)`, `s = 0,
1, ...`, with the same cover seed `0xC0FFEE + 101 i + s`, in the same order,
and the first one that passes every check of `negation_remeasure::cover_select`
(bielliptic, `r >= 50`, `r` does not divide `#E2`, `r^2` does not divide `#J`,
an order-`r` point on `E1`, an isogeny hop and a pull-back hop that pass `Chain::verify`)
**and** `r >= ceil(0.4 p)`. Here `r` is the largest prime with
`r || #E1(F_p)`, so `#E1 / r <= 2.5 (p + 1 + 2 sqrt p) / p`; each
`E1` selected below has a rational 2-torsion point, so here `#E1 = 2r`,
`r ~ p / 2` and `h = #J / r ~ 2 #E2 ~ 2p`. Why 0.4 and not 1/8: with 1/8 the
selected `r / p` ranged from 0.126 to 0.517 (cofactors 2, 4, 8), a factor
about 2 in the rho denominator; with 0.25 it ranged 0.27–0.52; 0.4 gives
0.494–0.542 at all ten primes (selection only, no arm run).

**Fallback, frozen:** if no candidate of the 4000 at `p` passes, the next prime
above `p` (in increasing order, same index `i`) is tried, at most 20 times
(`--max-prime-hops 20`); the prime used is recorded as `p` and the primes
passed over are listed (`primes_passed_over`); the fit uses the prime used.
If no prime within 20 hops has a candidate, that prime is reported as not run
and is not replaced.

Selection output before freezing (`--select-only`; `p` is the prime used):

| stage | p requested | p used | r | min_r | r / p | #E1 / r | #E2 | h = #J / r | candidate | skipped before it |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 211 | 211 | 109 | 85 | 0.517 | 2 | 226 | 452 | 9 | bielliptic_rejected 6, r_below_50 3 |
| 1 | 419 | 419 | 227 | 168 | 0.542 | 2 | 438 | 876 | 51 | bielliptic_rejected 34, r_below_50 15, r_below_min_r 2 |
| 1 | 821 | 821 | 421 | 329 | 0.513 | 2 | 782 | 1564 | 26 | bielliptic_rejected 14, r_below_50 10, r_below_min_r 2 |
| 1 | 1619 | 1619 | 811 | 648 | 0.501 | 2 | 1646 | 3292 | 2 | bielliptic_rejected 1, r_below_min_r 1 |
| 1 | 3203 | 3203 | 1583 | 1282 | 0.494 | 2 | 3278 | 6556 | 21 | bielliptic_rejected 15, r_below_50 2, r_below_min_r 4 |
| 1 | 6421 | 6421 | 3191 | 2569 | 0.497 | 2 | 6322 | 12644 | 2 | bielliptic_rejected 1, r_below_50 1 |
| 1 | 12841 | 12841 | 6379 | 5137 | 0.497 | 2 | 13034 | 26068 | 4 | bielliptic_rejected 2, r_below_50 1, r_below_min_r 1 |
| 2 | 25601 | 25601 | 12941 | 10241 | 0.505 | 2 | 25514 | 51028 | 41 | bielliptic_rejected 21, r_below_50 3, r_below_min_r 17 |
| 2 | 51203 | 51203 | 25717 | 20482 | 0.502 | 2 | 51366 | 102732 | 76 | bielliptic_rejected 49, r_below_50 4, r_below_min_r 23 |
| 2 | 102407 | 102407 | 51427 | 40963 | 0.502 | 2 | 102150 | 204300 | 66 | bielliptic_rejected 43, r_below_50 7, r_below_min_r 16 |

No prime needs the fallback. For comparison, the default selection (`--min-r-frac 0`, the
2026-11-13 run) still selects `r = 109, 113, 71, 401, 313, 3191, 3169, 127, 1831`
at the same primes, so that run replays unchanged.

## Instances, arms, seeds (unchanged from 2026-11-13)

Stage 1: requested primes `211, 419, 821, 1619, 3203, 6421, 12841`; stage 2:
`25601, 51203, 102407`, run in that order and not started after the wall-clock
budget of 1500 s (`--budget-s 1500`) has passed; a stage-2 prime that is not
started or not finished is reported as not run, never extrapolated. Stage 1:
3 DLP instances per prime (`seed = seed0 xor (k 0x1000)`, `x = (seed *
2246822519 + 7) mod r`), 4 IC seeds (`700 + s + 10k`) per instance for A and
B, 4 control seeds (`500 + s + 10k`) for A_S and B_S, 16 rho seeds per
instance on `E1` (negation primary, plain context), 4 rho seeds on the
`r`-subgroup of `J_C` (context). Stage 2: 1 instance, 2 IC seeds.

- **A**: whole-J IC, every degree-1 place up to sign in the base, full
  relations only.
- **B**: whole-J IC, double large primes, `ceil(2 sqrt p)` base columns.
- **A_S, B_S**: the same IC with the walk inside the transferred subgroup
  (`ic_on_rung` arms Full and DoubleLp), the T5–T7 regime.
- **Matched reference**: negation-map rho on `E1` in the same process
  (`rho_full_ec`), set-up separated; plain rho and rho on `J_C`'s `r`-part
  for context.

Every log certified by `[t]G' == T'` on the word Jacobian, `[t]phi^*G ==
phi^*T` on `J_C` (library arithmetic) and `[t]G == T` on `E1`.

## Pricing and calibrated reading (unchanged)

1 GAE = one group operation of the arm's own group (genus-2 counts over
elliptic counts understate the gap and are labelled so). Native counters of
every arm, rho included, priced at 0, 0.1 and 1 GAE per operation
(AGENTS.md §5a): IC `rel.mul + la.mul`; rho `canonicalisations +
partition_hashes + dp_lookups + dp_inserts + word_ops`. Calibrated reading:
`w_cal(class) = time(native op) / time(one group addition of the arm's own
group)`, best of 25 batches of 20,000 on this shared unisolated VM, each class
at 0, 1 and 10 `w_cal`. Order classes `above` / `below` / `unresolved` by
`negation_remeasure::classify`. Linear-algebra additions are a listed
sensitivity. The one-time transfer is in no per-instance total.

## Run

```
python3 tools/benchrec.py run <record> -- bash -c 'set -o pipefail; \
  ./target/release/full_jacobian_ic --threads 2 --min-r-frac 0.4 \
  --max-prime-hops 20 --stage2 25601,51203,102407 --budget-s 1500 \
  --out-dir research/full_jacobian_ic_rp_20261114 \
  --protocol research/full_jacobian_ic_rp_20261114/PROTOCOL.md \
  2>&1 | tee research/full_jacobian_ic_rp_20261114/run.log'
```

One run. If it is killed before stage 1 completes, the rerun is the same
command under a new record, and both are reported.

## Decision rule (the 2026-11-13 rule, unchanged)

For arms A and B separately, over the stage-1 primes (stage-2 primes are
added to a second fit, reported separately, and do not enter the verdict):

- **O3 closed by data (loss grows)**: every cell `above` at the band and at
  the calibrated reading, **and** the lower end of the 95% `t` interval of
  the OLS slope of `ln(arm total / negation-rho total at w = 0.1)` on `ln p`
  is above 0.2.
- **O3 open: advantage seen**: any cell `below` at all three band prices (an
  accounting change is not an advantage: the cell is re-read with every
  native counter listed before the word is used).
- **Otherwise unresolved**, reported with the cells or slopes that block the
  rule.

Both arms must meet the first condition for "closed"; a mixed result is
"unresolved". The verdict is the binary's `verdict` field, computed by the
code frozen here; RESULTS.md quotes it exactly. The gap to the matched rho
is reported for every row as the distance remaining, not as a pass/fail gate
for the success criterion; no arm here is called an improvement over the
index-calculus baseline unless its total is below the subgroup control's at
all three prices `[pcc:unverified]`.

## Frozen predictions

G1. Every stage-1 cell (A and B) is `above` at the band and at the calibrated
reading. P = 0.97.

G2. A's `w = 0.1` ratio slope (stage 1) lies in `[0.3, 0.8]` (P = 0.7), and
its 95% lower end is above 0.2 (P = 0.65).

G3. B's `w = 0.1` ratio slope (stage 1) lies in `[0.15, 0.7]` (P = 0.6), and
its 95% lower end is above 0.2 (P = 0.35). Reason for the low P: the
2026-11-13 secondary fit of `ln(B group ops / sqrt r)` on `ln p` was 0.29.

G4. Verdict "O3 closed by data" (G1, G2-lower, G3-lower together): P = 0.3;
"unresolved": P = 0.68; "advantage seen": P = 0.02.

G5. The negation-rho group-operation slope on `ln p` (stage 1) lies in
`[0.35, 0.6]` (P = 0.75): with `r ~ p / 2` it now tracks `sqrt p`, up to
the set-up and distinguished-point overhead that flattened it at these sizes.

G6. The slope of `ln(A group ops)` on `ln p` lies in `[0.7, 1.25]` (P = 0.6);
B in `[0.3, 1.0]` (P = 0.7).

G7. Every stage-1 prime runs to completion (P = 0.97). (No prime needs the
fallback: that is known from the selection output above, so it is not a
prediction.)

G8. Every log of every arm and reference is certified and no instance is
skipped at stage 1 (P = 0.9).

G9. Stage 2 completes 25601 (P = 0.85), 51203 (P = 0.7) and 102407 (P = 0.3)
within the budget rule.

## Amendment 1 (2026-11-15, before any data)

Frozen by the new `frozen.sha256` before the first measured run. No data has
been taken under this protocol: `full_jacobian.json` and `run.log` do not exist
in this directory (checked 2026-11-15), and the 2026-11-13 run is a different
protocol. Between the first freeze and this amendment only the unit tests,
`--select-only`, `--field-microbench` (timing of single field operations, no
cover, no arm) and two tiny smokes, at `p = 211` and at `p = 211, 419, 821`,
with output outside this directory, were run. Every number here is `[pcc:unverified]`.

### Why

Validator finding 7 (`research/validation_20261114/REPORT.md`; erratum 7 in
`research/full_jacobian_ic_20261113/ERRATA.md`): the unit "one group operation
of the arm's own group" counted one Cantor step on `J_C` as one `E1` addition,
although a Cantor step costs about 4.3-5.7 `E1` additions in time, so every
IC / rho ratio was understated. The IC arms also left work uncounted (the
partition hash, the large-prime map, the RNG draws, the one-time `[h]Z`) while
carrying `lower_bound: false`.

### Changes

1. **Primary unit: one `F_p` multiplication or squaring.** Both sides are
   priced in it. A Cantor step is tallied by `full_jacobian_ic::CantorTally`
   (`H64Curve::cantor` step for step: same extended gcds, products, divisions
   and reduction loop; same reduced divisor, unit-tested), an `E1` addition by
   `full_jacobian_ic::ec_add_tally` (affine formula on `F_p`-rational points:
   addition 2M + 1S + 1I, doubling 4M + 2S + 1I, identity operand or `P + (-P)`
   nothing). Additions, subtractions and negations are not counted. The
   `F_{p^2}` container of `ec.rs` costs extra products in time that are not
   charged (this favours the rho side being cheaper, i.e. it is conservative
   for "IC above rho"). The Cantor count is the generic algorithm, not
   explicit genus-2 formulae.
2. **Per-arm base, in this unit.** IC arms A, B: Cantor `mul + sqr` +
   `w_inv` x Cantor `inv` + `rel_mul` + `la_mul`. Rho on `E1`: `E1` `mul + sqr`
   + `w_inv` x `inv`. Rho on `J_C`: its Cantor tally. Every native counter of
   the arm (IC: none further, since `rel_mul` and `la_mul` are already in the
   base; rho: `canonicalisations + partition_hashes + dp_lookups + dp_inserts
   + word_ops`) is priced at 0, 0.1 and 1 unit per operation (AGENTS.md 5a)
   and, calibrated, at `w_cal = time(class) / time(one F_p multiplication)`
   at 0, 1 and 10 `w_cal`; classification by `negation_remeasure::classify`
   as before. The report is the arm's `field_unit` object.
3. **Frozen inversion weight `w_inv = 10`** multiplications (`--inv-weight`
   default). Justification, from `full_jacobian_ic --field-microbench` on this
   machine (Intel Xeon @ 2.30 GHz, 4 vCPU, shared VM, best of 25 batches of
   20,000, throughput timing, operands through `black_box`):

   | p | ns mul | ns inv (ext. gcd, the Cantor code) | inv / mul | ns inv (Fermat, `ec.rs`) | inv / mul |
   |---|---|---|---|---|---|
   | 211 | 1.54 | 10.83 | 7.0 | 21.84 | 14.2 |
   | 1619 | 1.54 | 15.10 | 9.8 | 31.92 | 20.7 |
   | 12841 | 1.54 | 19.36 | 12.6 | 43.31 | 28.1 |
   | 51203 | 1.54 | 22.48 | 14.6 | 46.87 | 30.5 |
   | 102407 | 1.87 | 22.82 | 12.2 | 54.02 | 28.9 |

   The ratio grows with `log p` and differs by code path. One weight is frozen
   for both sides so the unit is the same; `w_inv = 10` is the middle of the
   extended-gcd range 7.0-14.6 (the code the Cantor tally mirrors). The
   rho side's `ec.rs` uses Fermat inversion (14-30), so `w_inv = 10` is
   favourable to the rho side's cost being *under*-stated, i.e. it is
   conservative for "IC above rho" there. The sensitivity `w_inv = 0` and
   `w_inv = 30` is read from the stored `field_mul_sqr` and `field_inv` of each
   row by `render.py` (base only, native work at 0); it does not enter the
   verdict. A throughput microbench on a shared VM is a rough weight, not a
   calibration of the whole code.
4. **Decision rule, slopes and verdict in the field unit.** The rule of the
   section "Decision rule" is applied unchanged but to the `field_unit`
   reports: cells are classified `above` / `below` / `unresolved` from the
   `field_unit` band and calibrated ratios, and the slope is the OLS slope of
   `ln(arm field total / negation-rho field total at w = 0.1)` on `ln p`
   (`A_field_ratio_to_rho_E_w0.1`, `B_field_ratio_to_rho_E_w0.1` in
   `fits_stage1`). The binary's `verdict` field is this reading. The old
   group-operation reading is kept as `verdict_group_ops` (secondary, as the
   2026-11-13 rule) and every group-operation field of the output is
   unchanged. RESULTS.md quotes both, labelled.
5. **IC arms are lower bounds.** Every IC arm row carries `lower_bound: true`
   and `lower_bound_note` naming the uncounted work: the partition (jump) hash
   of every walk step (`key_of` + `mix64`), the large-prime map (`HashMap`) and
   union-find, the RNG draws (`StdRng`), the one-time `[h]Z` and `ord(Z)` of
   the instance (uncounted Cantor), polynomial additions and subtractions in
   Cantor and the relation phase, and linear-algebra additions. The subgroup
   controls A_S and B_S additionally run through `ic_on_rung`, whose Cantor
   steps are not field-tallied (`instrument.rs` is outside the tallied code):
   their `field_mul_sqr` and `field_inv` are null and their `field_unit` base
   holds only `rel_mul + la_mul`; they are a lower bound of a lower bound, and
   no comparison of A_S or B_S against A or B in the field unit is claimed.
6. **New row fields.** Per arm row: `field_mul_sqr`, `field_mul`, `field_sqr`,
   `field_inv`, `lower_bound_note`, `field_unit`; per prime: `rho_E.field`
   (negation, plain, bands), `inv_weight`; top level: `verdict_group_ops`,
   `inv_weight`, `field_unit` (definitions). The 2026-11-13 counters replay
   unchanged: `old_counters_replay_the_2026_11_13_p211_row` (p = 211, old
   selection) checks the `group_ops`, `setup_ops`, `walk_ops`, `rel`, `la` and
   `trials` sums of arms Full and DoubleLp and the negation-rho operation total
   against the 2026-11-13 row.

### Predictions G1-G9 under the amendment

G1-G4 and G6 are about the ratio slopes and the arm group operations. The
verdict and slopes in G1-G4 now refer to the **field unit** (primary); the
numbers P are unchanged, because the unit change raises the Cantor side by
the factor of field work per step (about 5 on this machine), which moves the
ratios up but the slopes only through the inversion share (a cost that grows
slowly with `log p`). So G2 and G3 hold as stated for `A_field_ratio_to_rho_E_w0.1`
and `B_field_ratio_to_rho_E_w0.1`, and G1 holds for the `field_unit` classes.
G4 is for `verdict`; the secondary `verdict_group_ops` is not predicted
separately. G5 and G6 are about group-operation counts and are unchanged (the
group-operation slopes stay in `fits_stage1`). G7-G9 do not refer to the unit.

New, frozen with the amendment:

G10. The mean Cantor step costs between 6 and 16 times the mean `E1` addition
in field-multiplication units (`w_inv = 10`; `field_mul_sqr + 10 field_inv` per
group operation, `J_C` over `E1`) at every stage-1 prime. P = 0.75. The band
is wider than the validator's 4.3-5.7 *time* ratio because the generic Cantor
code does about 110 products and 4 inversions per step at `p = 211`, while the
`E1` addition does 3 products and 1 inversion in this count; the `E1` time is
inflated by the `F_{p^2}` container that the count does not charge, so the two
ratios need not agree and the field-unit ratio is the primary one. Disclosure:
the band was set after the `p = 211` smoke run (arm A: 109 products and 4.3
inversions per step, `E1` negation rho 4.3 and 0.97, ratio 10.8), so G10 is not
an independent prediction at `p = 211`; it is for the other primes.

The smoke run (3 primes, outside this directory) also showed the unit's
sensitivity to `w_inv`: the Cantor-over-`E1` ratio per group operation was
about 25 at `w_inv = 0`, 10.5 at 10 and 6.9 at 30. The frozen weight is 10;
the other two are reported as sensitivity only.
