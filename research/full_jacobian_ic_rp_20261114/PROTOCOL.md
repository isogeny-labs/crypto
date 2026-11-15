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
