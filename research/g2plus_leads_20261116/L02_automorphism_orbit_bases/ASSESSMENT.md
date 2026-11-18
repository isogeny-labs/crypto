# L02 assessment: automorphism-orbit factor bases on `y^2 = x^q + a`

Assessor: an independent session that did not produce the data. Date: 2026-11-16.
Every number here is `[pcc:unverified]`. This is not a benchmark: no leaderboard
row, no `benchmarks/` row, no speed claim.

## Verdict

| hypothesis | frozen rule (PROTOCOL 6) | pooled N, primary (10, g^2) [95% CI] | (1,1) | (30,g^2) | band-dependent |
|---|---|---|---|---|---|
| H1, F5 genus 2, k = 12..14 | **SUPPORTED** (CI lower bound >= 2.0) | 2.464 [2.225, 2.750] | 2.531 [2.285, 2.819] | 2.384 [2.152, 2.657] | no |
| H2, F7 genus 3, k = 9..10 | **KILLED** (CI upper bound < 2.0) | 1.096 [0.905, 1.343] | 1.068 [0.882, 1.306] | 1.043 [0.861, 1.275] | no |

The frozen rule was applied as written. None of the 10 items in `DEVIATIONS.md`
changes an arm, a threshold, a seed rule, a grid cell or the decision rule, so
none of them invalidates the rule. **H1 passes narrowly, and the margin depends
on how this instrument charges fixed set-up costs** (concern 1 below). Read the
SUPPORTED verdict as "at p = 2^12 to 2^14, in this Python instrument, with rho
walk starts charged as two scalar multiplications". It does not show that the
orbit base narrows the gap to rho by 2x at larger sizes.

## 1. What was checked

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK. (The command
  exited 1 only because a later `wc` in the same shell line hit a directory.)
- Read PROTOCOL.md, SPEC.md, DEVIATIONS.md, RESULTS.md, `replay/REPLAY.md`
  (summary), `code/{ff,aut,ic,rho}.py`, `raw/selftest.txt`,
  `raw/gates_instances.jsonl` and `raw/analysis.json`.
- **Headline numbers re-derived from raw jsonl** with a separate script that
  does not import `code/`. It computes the arm mean as the mean over curves of
  the per-curve seed means, FPM as in PROTOCOL 5, and the geometric mean over
  the decision cells. It reproduces every point estimate in the RESULTS tables:
  pooled F5 D1 / rho_gain / N = 3.9774 / 1.6141 / 2.4642 at (10, 4), and F7
  2.8234 / 2.5749 / 1.0965 at (10, 9); per-cell values match to the printed
  digits. I did not re-run the bootstrap. The replay reproduced
  `analysis.json` byte for byte.
- **Gates re-checked independently from raw:** G4 (the A0, A and B digest
  prefixes agree, and trial counts are nested B <= A <= A0) has 0 failures in
  72 CM (curve, seed) pairs. G5 (`ncols_B q + delta0 = ncols_A`,
  `ncols_A0 = 2 ncols_A`) has 0 failures. G6 (control B = A and Raut = Rneg
  in every field except `arm`, `run_id` and `wall_s`) has 0 failures. All 48
  rows of `gates_instances.jsonl` have `pass = true`. `selftest.txt` shows
  15/15 PASS. Every IC and rho row is `certified = true`.
- **Independent mathematics** (stdlib, no project code). For F5:1031:196,
  F5:2081:3 and F5:4111:2172, I brute-forced N1 and N2 (over all of
  F_{p^2}) and rebuilt #J from the L-polynomial: 1003805, 4065821 and
  16434896 all match. r is prime, r = 1 (mod 5), Phi_5(lambda) = 0 (mod r),
  and nA, delta0 and nB = (nA - delta0)/5 match the stored values.
- **Relations are verified indirectly but strongly.** Each run's solution is
  certified against the planted t (`smul_rl(t, G) == T` and `t == t_planted`)
  inside `uncounted()`. A wrong sign or a wrong lambda^k coefficient in arm B
  would give a wrong t, and that would show up as uncertified runs. There are
  none. Individual rows are not re-checked one by one.

## 2. Instrument audit

**Counters.** `fmul`/`finv` count as SPEC 3 says, and `rmul`/`rinv` are kept
separate. `orbit_rep_point` charges q-1 `fmul` per point to arm B, which is
fair. `rho_apply` also charges one `fmul` for each zero `v_i` position via
`cnt_mul` (as SPEC requires). That slightly overcharges R_aut, by at most a
couple of multiplications against about 160 FPM per group operation, which is
under 2%. Rho mod-r bookkeeping is excluded from FPM, as the protocol says.

**Arms are paired and fair on the IC side.** Every IC arm runs on the same
curves and seeds, with the same trial stream (B is a prefix of A, and A of A0),
the same split test, the same elimination and the same unit. rel_ratio matches
(nA+10)/(nB+10) to within 0.1% in every CM cell. G7 (A0/A from 1.92 to 2.00)
shows that the instrument sees a known 2x fold.

**Concern 1, the main one: the set-up cost dominates rho.** In `rho.py`, each
of the 32 jumps and every new walk costs two full scalar multiplications plus
an addition. With `k_dp = floor(log2(E)/2)`, a walk is only about sqrt(E)
steps long, so starting a walk costs more than the walk. From raw, accepted and
look-ahead steps are only this share of all group operations:

| cell | Rneg | Raut | IC A (trials / grp_ops) | IC B |
|---|---|---|---|---|
| F5 k=12 | 0.24 | 0.13 | 0.80 | 0.46 |
| F5 k=13 | 0.41 | 0.20 | 0.87 | 0.60 |
| F5 k=14 | 0.40 | 0.20 | 0.92 | 0.74 |
| F7 k=10 | 0.62 | 0.37 | 0.70 | 0.28 |

So 80% of R_aut at F5 k=14 is walk-start and jump set-up. Class size does not
reduce that cost. It pushes the measured rho_gain (1.61 pooled) far below the
step ratio. The IC side has the same kind of fixed cost (16 jumps), which pulls
D1 down (3.98 measured against about 4.85 per trial). The two biases point in
opposite directions, and the rho side wins. The protocol fixed this accounting
("all set-up" charged), so this is not a deviation. But it is a needlessly
expensive reference, and it is what lifts N above the protocol's own
prediction of sqrt(5) = 2.24.

**Concern 2: abandoned fruitless cycles inflate R_neg.** In F5 k=13 and k=14,
27% of Rneg steps belong to walks abandoned at `20 * 2^k_dp` steps. That
matches the known rate of fruitless 4-cycles after look-ahead, about 1/(4*32^2)
per step. Raut abandons about 1%, because a 2-cycle needs the automorphism to
equal -1. A cycle-escape routine would recover these steps. This inflates
rho_gain and works against H1.

**Post-hoc sensitivity, not decisive and not a re-analysis under the rule.**
I subtracted set-up group operations, priced at each curve's mean FPM per Rneg
group operation (about 158 at genus 2 and 346 at genus 3), from every arm:

| family | setup-free D1 | setup-free rho_gain | N | rho_gain without abandoned steps | N |
|---|---|---|---|---|---|
| F5 pooled 12..14 | 4.85 | 3.08 | 1.58 | 2.42 | 2.00 |
| F7 pooled 9..10 | 6.27 | 3.94 | 1.59 | 2.74 | 2.29 |

Per F5 cell (k=12, 13, 14), setup-free N without abandonment is 1.91, 2.16
and 1.95. So once fixed costs are removed, F5 sits at the threshold, not
clearly above it. The asymptotic expectation is about q / (sqrt q x per-step
canonicalisation overhead), roughly 2.1 to 2.2. **The frozen SUPPORTED verdict
is therefore an artefact of how small these sizes are and of how the
instrument starts walks. It is not a robust 2x effect.** These figures come
from a cost approximation I chose after seeing the data. They must not
replace the frozen verdict, and they cannot rescue H2.

**Could a bug produce the effect?** I found no correctness bug. The effect
comes from accounting design, not from a coding error. The certified solves,
the exact control degeneration (G6) and the paired digests rule out a broken
arm B or a broken R_aut.

**Other items.**
- The logged selftest command in `raw/COMMANDS.log` does not run as written
  (replay note). `selftest.txt` was regenerated with the intended command and
  matched.
- DEVIATIONS item 1 (SPEC 12 test 3 contradicts SPEC 5 for two test curves) is
  handled conservatively.
- The prediction check "rho_gain in [0.75 sqrt q, 1.15 sqrt q]" fails at F5
  k=10 to 13 and F7 k=8. That symptom has the same cause as concern 1.

## 3. Effect sizes (primary reading, 3 curves x 3 IC seeds; 24 rho seeds per curve at genus 2, 12 at genus 3)

- F5 per-cell N, k = 10..14: 2.10 [1.91, 2.34], 2.24 [1.79, 2.80],
  2.28 [1.87, 2.72], 2.55 [2.13, 3.16], 2.58 [2.20, 3.07]. N rises with k
  because the fixed set-up fraction falls faster on the rho side.
- F5 D1 is 3.56, 4.01 and 4.40 at k = 12, 13, 14, approaching q = 5 as IC
  set-up amortises. The CIs are tight (about ±2%) because the trial streams are
  paired.
- F7 pooled D1 is 2.82 and rho_gain is 2.57, so N is about 1.1. The orbit base
  cuts relations by 5.8 to 6.3x, but at these sizes IC_B is dominated by fixed
  costs (trials are 21 to 28% of IC_B group operations), so D1 stays far below
  q = 7.
- Rho-side variance dominates the width of the N intervals (rho CV is about
  0.5 per run).

## 4. Distance to matched rho

The gap is reported as a distance, not as a pass/fail test. It is
`gap_B = IC_B / R_aut`, primary reading, in this Python instrument only:

- F5 (genus 2), k = 12, 13, 14: 0.68, 0.50, 1.06. IC_B is within 2x of the
  automorphism rho, in either direction. The gap does not shrink with k.
  Full-base IC and rho both scale as about p at genus 2, so no exponent gain
  is visible. The baseline gap_A = IC_A / R_aut is 2.41, 2.00 and 4.67.
- F7 (genus 3), k = 9, 10: gap_B = 0.18 and 0.10. IC is already below rho
  here (genus 3 IC is about p, rho about p^1.5). The orbit base adds only a
  constant factor of about 1.1 net.
- Neither number transfers to the lab's Rust explicit-formula instrument, to
  the 2LP arm (the lab's best genus-2 arm, `[pcc:unverified]` IC/rho 2.82 at
  p = 1031), or to larger p. At larger p the rho set-up share falls, and the
  sensitivity analysis says N should settle near 2 at genus 2.

## 5. Single most valuable next step

Re-run the F5 decision cells (and k = 15, 16 if the budget allows) with the
fixed costs taken out of both sides. Use a cheap walk start (start from the
previous distinguished point plus a hashed jump combination, or use a
precomputed table for aG + bT), add a fruitless-cycle escape in Rneg instead of
abandonment at 20 * 2^k_dp, and use the same jump precomputation in IC. Freeze
this as a new protocol with the same threshold of 2.0. If N stays at or above
2.0 there, apply the orbit fold to the 2LP genus-2 arm, which is where an
improvement over the lab's best IC row could come from.
