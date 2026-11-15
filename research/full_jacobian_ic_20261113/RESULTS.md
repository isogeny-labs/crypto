# Full-Jacobian index calculus on the transferred instance (open case O3): results

Status: `[pcc:unverified]`. This is one session on a shared 4-vCPU VM with no
isolation. It has no signed pcc claim and is not a row in the IC leaderboard
or in `benchmarks/README.md`. Protocol: `PROTOCOL.md`, frozen before any data
(`frozen.sha256`; `sha256sum -c` reports OK). Raw output: `full_jacobian.json`
and `run.log`. Full tables: `tables.md`. Figure: `full_jacobian.svg`.

## Question

T5–T7 in `research/chained_transfer_proof_20261112/PROOF.md` show that an
index calculus whose relations lie in the transferred subgroup `S` of the
genus-2 Jacobian degenerates to a collision search on `E`. O3 is the case
those theorems leave open: an IC that draws relations from the whole
`J_C(F_p)`, Gaudry-style. The question is whether that IC, run on the
transferred instance, gets anywhere near the matched negation-map rho on `E1`.

## Arms

| arm | what it is |
|---|---|
| A | whole-J IC, full relations: walk `aG' + bT' + kZ` with `Z` a random whole-J generator, one extra unknown `ζ = log_{G'}(hZ)` |
| B | whole-J IC, double large prime |
| A_S, B_S | the same IC restricted to `S` (the T5–T7 controls) |
| rho on E1 (negation) | the matched reference, with set-up separated |
| rho on J (negation) | control: rho in the Jacobian's `r`-part |

All arms are certified per log, with 0 exhausted runs. (Relation and walk
checks were off in this run, so no bad-relation count was measured.) Every
arm's group operations and linear-algebra multiplications are priced on the
§5a band (0 / 0.1 / 1 GAE per operation) and at the calibrated
reading (0 / w_cal / 10·w_cal). The transfer itself is not charged (trapdoor
convention).

## Result

Every cell, at every prime from 211 to 51,203 and for all four IC arms, is
`above` the negation rho on E1, both at the band and at the calibrated
reading. The distance remaining at `w = 0.1` (arm total / rho total):

| p | r | A | B | A_S | B_S |
|---|---|---|---|---|---|
| 211 | 109 | 7.0e3 | 406 | 114 | 51 |
| 419 | 113 | 2.4e4 | 1.8e3 | 246 | 133 |
| 821 | 71 | 5.6e4 | 4.3e3 | 434 | 240 |
| 1619 | 401 | 4.4e4 | 1.7e3 | 1.5e3 | 220 |
| 3203 | 313 | 3.4e4 | 1.2e3 | 3.9e3 | 556 |
| 6421 | 3191 | 2.4e4 | 626 | 1.1e4 | 649 |
| 12841 | 3169 | 1.2e5 | 881 | 2.7e4 | 920 |
| 25601 (stage 2) | 127 | 6.2e6 | 5.7e3 | 2.7e4 | 4.9e3 |
| 51203 (stage 2) | 1831 | 7.5e6 | 4.7e3 | 1.8e5 | 6.5e3 |

At `w = 0`, with group operations only, the smallest whole-J ratio is 17.96, for B at p = 211
(the S-restricted controls go lower: B_S 10.9, A_S 11.9).
Across the stage-1 primes, the cheapest whole-J arm, B, needs 18× to 54× the
rho group operations before any linear algebra is charged.

**Verdict under the frozen decision rule: unresolved.** Rule F1, "every cell
above", holds for both arms. The second condition, a lower 95% bound above
0.2 on the slope of `ln ratio` on `ln p` at `w = 0.1`, fails:

| arm | slope (stage 1, n = 7) | 95% interval |
|---|---|---|
| A | 0.421 | [−0.070, 0.912] |
| B | −0.053 | [−0.654, 0.549] |

No cell is `below` at any price, so the "advantage seen" branch is not
triggered either. The rule's blocking-cell list is empty: the result is
unresolved because of the slope, not because of any cell.

## Why the slope criterion fails

The protocol assumed rho on E1 would cost about `√p`. It costs about `√r`,
and the selected `r` does not grow with `p`: it is 71 at p = 821, 3191 at
p = 6421 and 127 at p = 25601. The fitted rho exponent on `ln p` is 0.236,
with 95% interval [0.120, 0.352]. Against `ln r` it is 0.234 over all 9 primes (0.238 over the 7 stage-1 primes), flattened at
these sizes by the set-up and the distinguished-point overhead. So the ratio's
denominator moves with `r` almost independently of `p`, and seven points
cannot fix a slope in `p`. This is a defect in the instance selection, which
did not bind `r` to `p`, and not a sign of an advantage. The secondary,
unfrozen fit of `ln(group ops / √r)` on `ln p` gives 0.420 for A and 0.292 for
B over all 9 primes, but 0.074 and −0.033 over the 7 stage-1 primes, so this
fit does not show a positive B trend either. The reading rests on the cells:
at least 17.96× rho at `w = 0` and 406× at `w = 0.1`.

The B slope is also noisy for a second reason: B's cost at `w = 0.1` is
dominated by the linear-algebra additions, which change by large factors
between primes as the number of large-prime merges changes.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| F1 | every A and B cell above at band and calibrated, 7 stage-1 primes | **held** (7/7 for each) |
| F2 | ratio slope in [0.3, 0.7] for A and B; lower 95% end > 0.2 | **failed**: A 0.421 is in range, B −0.053 is not; neither lower end clears 0.2 |
| F3 | group-op exponent A, B in [0.8, 1.25]; rho on E1 in [0.4, 0.6] | **failed**: A 0.552, B 0.444, rho 0.236 (r not bound to p) |
| F4 | B cheaper than A at w = 1, p = 12841 | **held** (1.96e6 vs 2.85e8) |
| F5 | A within [0.4, 2.5]× A_S in group ops, p = 12841 | **held** (0.71) |
| F6 | deg-2 smooth fraction in [0.4, 0.6] for every run | **failed**: 3 of 176 whole-J runs exceed 0.6, all at p = 211 (0.6031, 0.6031, 0.6034); per-prime means 0.4830–0.5398 |
| F7 | every log certified, no instance skipped at stage 1 | **held** (12/12 per arm per prime) |
| F8 | stage 2 completes p = 25601 and p = 51203 | **held**, on the rerun: the first run was killed at p = 51203 before p = 102407; the rerun used `--stage2 25601,51203` under the same open record |

## Reading

No evidence of an advantage: the whole-J arms A and B are above the matched rho by
factors of 406 to 7.5e6 at `w = 0.1`, at every prime measured. That agrees with
the best-known-algorithm comparison in PROOF.md O3 (`Õ(p^{2−2/g})` against
`√r`). But the frozen rule needed to call O3 "closed by data" is not met,
because of the instance-selection defect above, so O3 remains open in
PROOF.md. A rerun that would decide it fixes `r ≈ p`, so that `#E1 / r`
stays small, and freezes that choice in a new protocol. The current
selection does not bind `r` to `p` (h = #J / r ranges from 452 to 5.1e6).

Limits: toy size (p ≤ 51,203); one shared VM with no isolation (wall times
are indicative only); stage 2 has 2 IC seeds and 1 instance per prime.

## Errata after independent validation (2026-11-14)

`research/validation_20261114/REPORT.md` admitted this run with caveats
(`VALIDATION.json`, `[pcc:unverified]`). It recomputed 2,057 values, replayed
all 9 primes with 0 non-timing differences and re-checked every log with
independent group-law and Cantor code. It found eight reporting errors, none of
which changes a cell, a class or the verdict. Items 1–5 are corrected in the
text above; 6–8 are recorded here.

1. F6 was scored "held"; it failed on 3 of 176 runs.
2. The smallest `w = 0` ratio is B's 17.96, not A's 18.7.
3. Three secondary fits were over all 9 primes without saying so.
4. "0 bad relations" was not a measurement (the checks were off).
5. Linear-algebra additions are not on the band; they appear only in
   `total_w1_with_additions`, as `PROTOCOL.md` says.
6. The seeds in `PROTOCOL.md` (700+s, 500+s, 1000+s) differ from the code
   (700+s+10k, 500+s+10k, 1000+s+(seed mod 997)). Both are fixed formulas.
7. The IC arms are lower bounds but carry `lower_bound: false`. Uncounted work
   (partition hash, large-prime map, RNG draws, `[h]Z`) and the unit (one
   Cantor step counted as one E1 addition, though it costs about 4.3–5.7 E1
   additions in time) both understate the IC side, so every ratio is
   conservative for "above".
8. p = 102,407 is absent from `stage2_not_run`, and the killed first stage-2
   run is not in the bench record.

The validator also could not establish the freeze ordering from git (the
protocol and data entered in one commit), and the smoke run had already shown
the operation counts at p = 211 and 419 before the predictions were frozen.
