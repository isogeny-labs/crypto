# Generator-orbit IC levers at K1 n29: lean base, sibling pair table, tau-adic check (2026-11-18) `[pcc:unverified]`

Engineering levers on the validated generator-orbit IC arm `gen-c1` (`research/ecbench_generator_orbit_20261022`) on the
K1 a=1 n=29 row (log2 r 15.37), the row where that arm is closest to rho. Every number is `[pcc:unverified]`: counted GAE
from one local ecbench session, L0, **no independent validation yet**. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen at
0ae90a4812 (hashes in `REGISTRATION.sha256`, `registered-at.txt`) before any data; code at 33dccd579d. Report:
[`REPORT.pdf`](REPORT.pdf); figures `figures/fig1-phases-band.{svg,png}`, `figures/fig2-ratios.{svg,png}`.

## Result (P1, the one primary test)

| ratio (paired, 48 targets x 3 rounds) | w = 0 | w = 0.1 | w = 1 | reading |
|:--|:--|:--|:--|:--|
| **lv-all / gen-c1** | **0.713 [0.679, 0.749]** | **0.614 [0.580, 0.649]** | **0.487 [0.462, 0.514]** | improvement at all three weights |

The model prediction frozen in the protocol was ~0.71 / ~0.61 / ~0.49. Attribution (secondary, descriptive):

| lever alone / gen-c1 | w = 0 | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| lv-lean (`lean_layout=1`) | 1.000 (by construction) | 0.762 [0.741, 0.783] | 0.459 [0.432, 0.487] |
| lv-sib (`sib=1`) | 0.834 [0.815, 0.854] | 0.907 [0.898, 0.915] | 0.998 [0.998, 0.999] |
| lv-tau (`tau_verify=1`) | 0.879 [0.863, 0.895] | 0.945 [0.939, 0.952] | 1.030 [1.028, 1.032] (worse) |

At w = 1 the tau-adic check is slightly worse (its ~30 recoding steps, conservatively reported as unpriced native work,
outweigh the ~11 GAE it saves), and lv-lean alone (1.426) beats lv-all (1.515) in band S; lean + sib without tau was not
an arm (open thread).

## Distance to rho, before and after (same session; paired ratios, 95% CI)

| arm / reference | w = 0 | w = 0.1 | w = 1 | band order |
|:--|:--|:--|:--|:--|
| gen-c1 / rho-v5 (before) | 1.231 [1.081, 1.401] | 1.819 [1.641, 2.017] | 4.644 [4.291, 5.026] | rho-v5 < gen-c1 |
| **lv-all / rho-v5 (after)** | **0.877 [0.736, 1.046]** | **1.116 [0.956, 1.304]** | **2.262 [2.002, 2.555]** | unresolved inside the band |
| gen-c1 / rho-strong (before) | 0.098 [0.087, 0.110] | 0.153 [0.140, 0.167] | 0.560 [0.530, 0.591] | gen-c1 < rho-strong |
| lv-all / rho-strong (after) | 0.070 [0.059, 0.082] | 0.094 [0.081, 0.109] | 0.273 [0.245, 0.303] | lv-all < rho-strong |

Against the strongest measured rho (v5) the arm moves from above at every weight to: point estimate below at w = 0 but the
interval contains 1, parity at w = 0.1 (interval contains 1), still 2.3x above at w = 1. The band does not rank lv-all
against v5. Nothing here is "below rho": the w = 0 interval crosses 1 and the order flips across the band. Against
rho-strong (the leaderboard's reference, dear because of its 32-scalar-multiplication set-up) the arm was already below
and moves further below. Band S (w = 0 / 0.1 / 1): gen-c1 0.439 / 0.706 / 3.110; lv-all 0.313 / 0.433 / 1.515; rho-v5
0.357 / 0.388 / 0.670; rho-strong 4.505 / 4.611 / 5.557.

## Phase split (mean per solve)

| arm | factor base | pair table | relations | verify | total GAE | factor-base unpriced native ops | other priced native |
|:--|--:|--:|--:|--:|--:|--:|:--|
| gen-c1 | 0.99 | 32.07 (30 adds) | 43.43 (40.2 trials) | 20.33 (13.7 dbl + 6.6 add) | 96.87 | 377 | |
| lv-all | 0.99 | 17.07 (15 adds) | 43.43 (same trials) | 9.35 (~9.4 adds, 0 dbl) | 70.89 | 30 | +14 sibling points, +30.2 recode steps |
| rho-v5 | setup 22.65 | | search 43.78 | 7.12 | 73.56 | | |

Every candidate did the same `targets_tried` as gen-c1 on all 144 (round, target) pairs and recovered the same `d`: the
levers change no walk, table entry or relation, only the work spent around them.

## What the levers are (cost chain)

1. `lean_layout=1` (`compact-orbit-scan`): the all-known-log base without its hash index (58 inserts, 174 probes,
   58 negative-key negations), the 29 per-image on-curve checks and the single column's 28 orbit-key squarings; the
   orbit-layout oracle names points by index and never read the index. Same points (`points_sha256`), columns,
   coefficients, negation map. Refused by every other oracle.
2. `sib=1` (`mitm-frobenius-layout-counted`): `x(P + X)` and `x(P - X)` from one inversion (on a binary curve `-X`
   changes the slope by `x_X / (x_P + x_X)`), charged one addition plus one `sibling_second_points` (the repository's
   `wwalk`/`ksum` convention, priced at `w`); `P + (-P)` not formed. Same entries, keys, insert order and probes
   (unit test probes 3,000 walk points against both tables).
3. `tau_verify=1`: the recovery check computes `[d]G` as the tau-adic NAF of the integer `d` summed over the stored
   orbit `{+-tau^t G}` (`tau^n = 1`), then compares the full point with `Q`. It uses only `tau^2 - mu tau + 2 = 0` on
   the curve, no eigenvalue or logarithm, so it stays an independent, fully charged check (a wrong `lambda` still fails
   it). The verify phase has zero doublings and ~9.4 additions instead of 13.7 doublings and 6.6 additions.

Named so nobody mistakes it: `gen-c1` is a Frobenius- and negation-folded baby-step giant-step under the IC harness
(gen-orbit README; VAL-20261118-4ae594 section 1) and stays one. These are constant-factor savings on that arm; the
asymptotic ceiling (`~2 sqrt(r/2n)` against `sqrt(pi r/4n)` for folded rho, ~1.6x above) is unchanged. T0 only. Not a
relabelling: the boundary, unit, targets, oracle, walk and pricing are the same; removed work is performed work (hash-map
operations, field inversions, group operations). The band's own rule prices the added sibling points and recode steps.

## Wall time (descriptive, L0, microseconds; not the unit)

Mean per solve, gen-c1 -> lv-all: factor base 40.6 -> 26.9, pair table 64.4 -> 66.6, verify 3.4 -> 5.1, solve 166 ->
160. Only the lean base shows in the clock. The sibling build does not (canonicalisation and table inserts dominate the
set-up's wall time here), and the tau-adic check is *slower* in wall time than the double-and-add (lv-tau verify 6.7 vs
3.4): the GAE unit charges an affine addition and a doubling alike, and the recoding and its i128 loop are not free.
The counted GAE saving of `sib` and `tau_verify` is real in the unit and in field inversions, but this session does not
show it in wall time at n29.

## Controls

- Default off: gen-c1-off (`lean_layout=0,sib=0,tau_verify=0`) equals gen-c1 on all 144 records in cost, every phase's
  counts and native maps, counters, outcome, seed and base points; ratio 1.000 [1.000, 1.000] at all weights. The frozen
  `analyze.py` identity check printed 0/144 because it also compared the echoed parameter strings (`factor_base.params`,
  `fb_id`, `detail.label`), which differ by construction; with those three fields excluded, 144/144 (disclosed deviation).
- A/A gen-c1-aa identical on 144/144. All 1728 records verified, `matches_target` true;
  `verify --replay-all`: 1296 replays reproduced, 0 problems (`audits/audit-n29.json.gz`).
- Cross-binary: the pinned binary replays all 864 measured runs of the A0 n29 session of
  `koblitz_known_log_columns_20261118` identically, 0 problems (`audits/xbin-a0-n29.json.gz`): the incumbent and both
  rho references are unchanged since VAL-20261118-4ae594.
- Unit test `generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work` (n23, n29, n37): tau-NAF digits
  evaluate to `d` in `Z[tau]` for both `mu`; `tau_orbit_mul = [d]G` on ~1000 scalars per curve; lean base equal to the
  indexed one; sibling table equal probes; end-to-end same `d` and trials. Related existing tests: 99 passed.

## Standing and deviations

- **Validation: not done.** No `VALIDATION.json`, so no leaderboard row or section and no `benchmarks/README.md` row
  (gate; artifact store also down). Benchmark record `benchmarks/records/20261118T1512Z-koblitz-n29-orbit-levers.json`.
- No EKS/Iso Lab and no S3: local L0, `--cpus none --allow-busy`. Raw session (`records.jsonl`, sha256 in
  `summary.json`) gzipped in the scratchpad (`n29lv-sessions/`), not in git; compact table `tables/records-n29.csv.gz`;
  full compare output `results.json.gz`.
- Binary: built from 33dccd579d in a clean sparse scratch worktree, own `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=1`,
  `--locked`; sha256 `43f44fec44b902f22753c2b44fd10d0567ad327cf73c9298c3558de3ff5c3e65` (`logs/binary.sha256`);
  `strings` shows `lean_layout`, `tau_verify`, `tau_naf_recode_steps_uncharged`; target dir deleted after.
- No conductor and no `research-kb` server in this container; no novelty claimed (siblings, tau-adic NAF and stored-point
  checks are textbook and already used here for `wwalk`/`ksum` and rho v5's ladder check).
- CPU: the session took 12.9 s wall (6.0 s user); builds and tests about 2.5 CPU-hours in total.
- v5 leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1): the v5 distances
  above are slightly optimistic for rho at w > 0.

## For the validator

Recompute lv-all / gen-c1 and the four gap rows from `tables/records-n29.csv.gz` or the scratchpad session with
`ecbench compare --weight w --resamples 4000 --seed 20261002`; replay with `ecbench verify --replay-all`; check that the
three parameters absent / =0 leave the arm unchanged (gen-c1-off, unit test), that the sibling abscissae equal
`x(P - X)` (unit test probes), and that the tau-adic check computes `[d]G` without `lambda` (`tau_naf`, `tau_orbit_mul`
in `src/cryptanalysis/ic_boundary.rs`).
