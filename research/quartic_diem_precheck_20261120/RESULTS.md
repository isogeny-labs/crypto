# Results: pre-check for lead #1 (Diem line IC on prime-order plane quartics)

Every number here is `[pcc:unverified]`. It comes from one session on a shared host
and has no replay certificate and no VALIDATION.json.

Protocol: `PROTOCOL.md`, frozen before any data (`frozen.sha256`:
`sha256(PROTOCOL.md)`). Binary: `binary.sha256`. Raw data: `raw/run.jsonl` (analysed)
and `raw/pilot.jsonl` (wall-time pilot only, not analysed). Their hashes are in
`data.sha256`. Analysis: `analyze.py` produces `analysis.json`. Figure: `plot.py`
produces `fig_scaling.svg` and `fig_scaling.png`.

## Verdict

**STOP under the frozen rule (K2 fires). The cause is in the instrument, not in LA.**

| rule | statistic (arm `Q_diem_2lp`) | value | fires? |
|---|---|---|---|
| K1 | LA-count (`la_mul`) exponent, point estimate > 1.40 | **1.201** (boot95 [1.050, 1.366]) | no |
| K2 | total (FPM mid) exponent, upper boot95 >= 1.40 | **1.301** (boot95 [1.199, **1.416**]) | **yes** |

The lead's specific worry was that sparse LA scales worse than `q^1.4`. That did not
happen: LA scales at about `p^1.2`. The total-cost interval crosses 1.40 because of
the relation phase. In this instrument, that phase holds the descent, and the
descent is implemented as a linear scan over all `~p` rational points for every
decomposition trial (see "Diagnosis").

## Grid as run

- Primes 1031, 2053, 4099, 8209, 16411, 32771. 4 prime-order curves per prime
  (`h = 1` at every prime; `--max-h 1`), 2 solver seeds per curve, giving 48 runs per
  arm. 48/48 runs per arm were certified (`[t]G = T`, `ok = true`). No prime was
  dropped.
- Prime-order admission needed 290, 105, 257, 165, 366 and 221 curve seeds for 4
  curves at the six primes. The searches took 133, 91, 528, 679, 1976 and 2241 s of
  wall time. Admitted seeds are in `raw/run.jsonl` (`kind: instance`). The search is
  not charged to any arm.
- Budget: the pilot (8.2 min wall) estimated the C=4, S=2 grid at about 45 min. The
  actual main run was 67.5 min wall and 111.5 CPU-min on `--threads 2`. The curve
  search at large p was slower than the estimate. Total wall was about 76 min
  including the pilot, within the 90 min budget. Total CPU was about 120 core-min,
  on 2 of the host's 4 cores. Nothing was rerun. `research/genus_ladder_scaling_20261119`
  was not touched.

## Fitted exponents (OLS slope of ln metric on ln p; curve level, 24 points)

Interval columns: boot95 is the cluster bootstrap over curves within each prime, 4000
replicates, seed 20261120 (the primary interval). t95 is OLS on the 6 per-prime
means, df = 4.

Arm `Q_diem_2lp` (the lead's algorithm, `nb = ceil(3 sqrt p)`):

| metric | slope | boot95 | t95 | local slopes 1031→…→32771 |
|---|--:|---|---|---|
| **la_mul (K1)** | **1.201** | [1.050, 1.366] | [1.002, 1.401] | 1.33, 0.94, 0.87, 2.00, 0.82 |
| la_mul + la_add | 1.401 | [1.246, 1.567] | [1.209, 1.594] | 1.51, 1.12, 1.12, 2.16, 1.04 |
| **FPM mid (K2)** | **1.301** | [1.199, 1.416] | [1.091, 1.512] | 1.27, 0.95, 1.04, 2.02, 1.22 |
| FPM low (I=1, kappa=1) | 1.345 | [1.251, 1.453] | [1.126, 1.564] | |
| FPM high (I=30, kappa=9) | 1.281 | [1.177, 1.398] | [1.066, 1.496] | |
| GAE w=0 (group ops) | 0.637 | [0.530, 0.764] | [0.452, 0.823] | |
| GAE w=0.1 | 1.549 | [1.464, 1.654] | [1.392, 1.706] | |
| GAE w=1 | 1.550 | [1.464, 1.657] | [1.393, 1.706] | |
| rel_mul | 1.574 | [1.491, 1.677] | [1.421, 1.726] | 1.67, 1.29, 1.38, 2.13, 1.40 |

Per-prime geometric means for `Q_diem_2lp`:

| p | la_mul | FPM mid | rel_mul | LA share of FPM mid | fb cols | core rows x cols | relations | max attempts |
|--:|--:|--:|--:|--:|--:|---|--:|--:|
| 1031 | 1.24e5 | 3.61e6 | 1.03e6 | 0.31 | 98 | 104x86 | 871 | 2 |
| 2053 | 3.10e5 | 8.64e6 | 3.24e6 | 0.33 | 137 | 145x112 | 1582 | 3 |
| 4099 | 5.93e5 | 1.67e7 | 7.88e6 | 0.33 | 194 | 197x152 | 3044 | 3 |
| 8209 | 1.08e6 | 3.44e7 | 2.05e7 | 0.29 | 273 | 277x210 | 5898 | 4 |
| 16411 | 4.34e6 | 1.39e8 | 8.97e7 | 0.28 | 386 | 394x282 | 11276 | 6 |
| 32771 | 7.62e6 | 3.23e8 | 2.37e8 | 0.22 | 545 | 560x395 | 21838 | 9 |

Secondary arm `Q_diem_full` (factor base = all points), reported only: la_mul
1.964 [1.915, 2.000]; FPM mid 1.956 [1.906, 1.991]. LA is 97 to 99.9% of its cost
(a Krylov solve on a `~p x p` core). This is the expected `p^2` and serves as a
sanity check on the LA counter.

## Diagnosis (post hoc, exploratory; not part of the frozen verdict)

1. **The descent is a linear scan.** `Ctx::decompose`
   (`src/cryptanalysis/quartic_ladder.rs`, around line 616) finds the base locus of
   the conic net by evaluating the net at every one of the `~p` rational points. That
   is about 2.9 counted multiplications per point per trial. A least-squares
   decomposition of `rel_mul` over all 48 runs,
   `rel_mul ≈ 323·trials + 2.88·decomp_trials·p + 22·p ln p`, gives R² = 0.99998.
   The scan's share of `rel_mul` grows from about 0.47 at p = 1031 to **about 0.92
   at p = 32771**. Diem's descent computes the base points algebraically (intersect
   two conics with the quartic: a resultant and root finding, `polylog p` per
   trial), so a per-trial `O(p)` scan is an artefact of this implementation.
2. **Decomposition trials grow as about p^0.75 and LA attempts as about p^0.29.**
   The solve fails and is retried more often at larger p: the mean number of attempts
   is 1.2, 1.6, 1.8, 1.8, 3.8 and 3.9, with up to 9 at 32771. Each retry adds 4
   inhomogeneous rows (`want_inhom += 4`) and a new Wiedemann solve with a dense
   projection. This inflates both `decomp_trials` and `la_mul`, and the jump in the
   local slope at 8209 → 16411 (about 2.0 in every metric) lines up with the jump in
   attempts. Why the solves fail is not diagnosed here.
3. **Counterfactual (exploratory, post hoc, not a verdict).** Subtracting the fitted
   scan term (`2.877·decomp_trials·p`) from FPM mid gives an exponent of **1.03**
   (curve-level OLS, no interval computed). With an algebraic descent the total would
   probably clear K2, but this run does not establish that.

## Caveats

- **Instrument, not algorithm.** The stop applies to the lab's current
  `quartic_ladder` Diem instrument: linear-scan descent, scalar Wiedemann with
  retries, `nb = 3 sqrt p` not tuned. It is not evidence that Diem's algorithm on
  prime-order quartics scales at `p^1.3` or more. This is the L08 failure mode
  (instrument choices bend the slope) that the referee warned about.
- **LA solver.** The LA is sparse scalar Wiedemann on `AᵀDA` after peeling
  (`chained_transfer::ic_control::wiedemann`). It is NOT the lane's block Lanczos,
  which is not wired into `genus_ladder`. Repeated failed attempts are charged. The
  report's "validated block Lanczos" premise for this binary is false.
- **Sensitivity close to K1.** `la_mul + la_add` has slope 1.401, which would exceed
  1.40 if it were the primary LA metric. `la_add` is dominated by graph and DSU
  additions and AᵀDA additions, so the frozen primary is `la_mul`. The point estimate
  sits on the boundary, so the K1 reading is fragile.
- **Curve population.** Random smooth plane quartics with a rational point
  (`next_smooth_quartic_with_point`), filtered to prime `#Pic^0`, 4 per prime. This is
  a small sample, though cost spread between curves was small next to the trend.
  Prime-order admission was about 1.1 to 4% of seeds.
- **Cofactor.** `h = 1` everywhere (no cofactor).
- **No rho comparison.** Rho was skipped (`--rho-cap 1000`). The L10 correction (the
  lab's quartic negation rho is overpriced about 1.73x, and the `<rho>` class walk is
  about 0.447 of plain negation; see
  `research/g2plus_leads_20261116/L10_c34_picard_matched_rho/ASSESSMENT.md` and
  `problems/quartic-matched-rho-overpricing.md`) is NOT applied. Nothing here is an
  IC/rho statement.
- **Small prime range.** 1031 to 32771 spans a factor of about 32. The local slopes
  vary between 0.8 and 2.0. Set-up terms still matter at the low end:
  `genus_ladder_20261114` found the quartic 2LP slope to be 0.69 on smaller p.
- **Binary not rebuilt.** The existing `target/release/genus_ladder` was copied
  (sha256 `493ee294…6921`, identical to the earlier `genus_ladder.po` copy). It was
  built at 2026-11-18 16:45 UTC, one minute after the last commit touching its
  sources (`ca72267899`). The tree at HEAD `c03241fae8` is clean. A rebuild was not
  done to verify byte identity.
- **Pilot curve reused.** The pilot's p = 32771 curve (seed 32) and its p = 1031
  curve (seed 54) are also the first curves admitted in the main grid, because seeds
  are scanned in the same order. Pilot rows are not in the fit.
- Wall time is not used for any verdict.

## Recommendation to the orchestrator

Under the frozen rule, lead #1 as instrumented is STOPPED. Before the full ladder
is reconsidered, two cheap instrument fixes would decide whether the stop is real:

1. Replace the linear-scan `decompose` with algebraic base-locus extraction and
   count it honestly.
2. Diagnose the growth in Wiedemann attempts, or use the coordinate or block-Lanczos
   solve.

Then rerun this frozen pre-check unchanged under a new protocol id. The LA premise
itself (`la_mul` about `p^1.2`) did not fail.
