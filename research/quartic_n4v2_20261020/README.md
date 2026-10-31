# N4 v2: plane-quartic index calculus on the GHS-weak family, q = 2^8..2^22, against plain and negation-map rho

2026-10-20 to 2026-10-25. Branch `claude/quartic-n4v2-20261020` (cut from
`origin/claude/quartic-pic-canonical-20261018`, which contains N4, #2283). Frozen protocol:
[PROTOCOL.md](PROTOCOL.md), **commit `82a9e87de3`**, committed before any pilot or grid data and not
edited. Pilot-set parameters: [PARAMS.md](PARAMS.md) (commit `93bee805c4`, with the instance list).
Pre-grid addendum: [ADDENDUM-20261025.md](ADDENDUM-20261025.md) (commit `1416ae3f92`: CPU-time
fields, build hashes, the lost first attempt). Report with graphs: [REPORT.pdf](REPORT.pdf) (source
[REPORT.typ](REPORT.typ)). Records: `results/` (raw units in `results/grid-20261025/units.tar.gz`).

Scope: the GHS-weak family `E0/F_{q^3}`, `#E0(K) = 4 ell`, special `m = 2` form, only. No
generic-curve, prime-order, deployed-curve, standard-curve or key-recovery claim. No external
implementation was compared (none installed), so there is no fastest-implementation claim.

## Answer

* **Bend (frozen rule on the high-range exponent `e_high`, k = 17..22, 95% bootstrap).** At w = 0:
  B0, S, L, D and DL1 are **between 1 and 6/5**; DL is **below 1**; no arm is "toward 1" or "toward
  6/5". At w = 1: every IC arm is between 1 and 6/5. *Interpretation (not the rule):* the
  double-large-prime arms steepen from the low range and settle at 0.99-1.02, the `O~(q)` model; the
  single-large-prime arms rise but stay below 6/5 on this range. The intervals come from resampling
  3 units per size (few distinct draws), so they are narrow: D excludes 1 by only 0.007, and the
  conversion-constant sensitivity below moves D's `e_high` to 1.048, more than any interval.
  Measured exponents of cold M-eq (w = 0):

  | arm | full k 12..22 | low k 12..16 | high k 17..22 | frozen bend rule on `e_high` (w = 0) |
  |:--|:--|:--|:--|:--|
  | B0 (frozen N4) | 1.209 [1.204, 1.214] (k <= 21) | 1.119 [1.103, 1.133] | 1.141 [1.139, 1.142] | between 1 and 6/5 |
  | S (sorted) | 1.211 [1.203, 1.218] | 1.116 [1.094, 1.131] | 1.108 [1.050, 1.153] | between 1 and 6/5 |
  | L (SGE + block Lanczos) | 1.212 [1.204, 1.220] | 1.122 [1.091, 1.145] | 1.162 [1.138, 1.184] | between 1 and 6/5 |
  | D (2 large primes) | 1.035 [1.028, 1.042] | 0.902 [0.873, 0.935] | 1.009 [1.007, 1.011] | between 1 and 6/5 (1.009) |
  | DL1 (D + SGE + scalar Lanczos) | 0.938 [0.927, 0.950] | 0.752 [0.708, 0.803] | 1.017 [1.015, 1.021] | between 1 and 6/5 (1.017) |
  | DL (D + SGE + block Lanczos, headline) | 0.983 [0.975, 0.993] | 0.835 [0.802, 0.873] | 0.988 [0.986, 0.989] | below 1 |
  | rho plain | 1.504 [1.470, 1.532] | 1.511 [1.464, 1.558] | 1.475 [1.373, 1.573] | |
  | rho-neg | 1.543 [1.494, 1.583] | 1.522 [1.474, 1.565] | 1.640 [1.493, 1.769] | |

  D, DL1 and DL steepen from the low to the high range (curvature +0.11, +0.27, +0.15, intervals
  above 0) and land at 0.99-1.02 on k = 17..22, the model `O~(q)`. B0's high-range exponent 1.141 is
  below 6/5; its full-range 1.209 is pushed up by a step in the measured `lmul/mul` ratio at
  k = 17 (1.55 -> 2.37, when `ell` passes about 2^50: the u128 remainder becomes slower on this
  host). With N4's Xeon constants (`r_inv = 15`, `r_lmul = 1.1`, sensitivity) B0 is 1.144 full,
  1.099 low, 1.179 high (still "between" by the rule), D 0.982 / 0.877 / 1.048, and DL
  0.942 / 0.811 / 1.029.
  The bootstrap intervals cover sampling of units and runs only, not the conversion constants.
* **Crossover in M-eq, against rho-neg (the pre-registered comparison; DL headline arm).** DL is
  band-resolved below rho-neg from q = 991 at every larger size; interpolated `q*` = 555 [389, 752]
  at w = 0, 276 [232, 319] at w = 1 GAE. B0: resolved from q = 16363, `q*` = 6371 [5383, 8267] at
  w = 0 (895 at w = 1). At the top sizes DL / rho-neg = 0.005 [0.004, 0.008] (q = 2097091) and
  0.004 [0.003, 0.007] (q = 4194271, using the post-hoc k = 22 rho supplement below; the frozen
  k = 22 rho runs fail the calibration rule).
* **Crossover in wall time** (frozen measure, single thread, shared host, L0): DL vs rho-neg resolved
  from q = 8191, `q*` = 5704 [1495, 7613]; B0 vs rho-neg resolved only at q = 2097091, `q*` = 2.77e5
  [2.32e5, 4.26e5]. **CPU time** (addendum deviation, `getrusage`): DL vs rho-neg resolved from 8191,
  `q*` = 1889 [1733, 4373]; B0 vs rho-neg resolved from 262111, `q*` = 1.84e5 [1.56e5, 2.30e5].
  At q = 4194271 one DL solve took 12.3 CPU-s against 616 CPU-s (rho-neg) and 650 CPU-s (plain rho)
  per run (supplement means).
* **Each change against the frozen B0** (same instances, paired bootstrap over seeds, M-eq w = 0,
  q = 2097091): S/B0 = 1.001 [1.000, 1.001] (cost-neutral: engineering, replayable, O(N) memory,
  0.72x wall); L/B0 = 0.784 [0.767, 0.816]; D/B0 = 0.081 [0.081, 0.082]; DL/B0 = 0.042
  [0.042, 0.042] (0.060 at w = 1). All band-resolved lower except S.
* **Negation map.** rho-neg / rho-plain M-eq at w = 0 = 0.715 [0.656, 0.777] (geometric mean over
  the 8 sizes k >= 12 that pass the calibration rule; theory 0.707). At w = 1 GAE rho-neg is the
  more expensive reference (it charges about three unpriced operations per step: canonicalisation,
  distinguished-point test, cycle check, against one for plain rho); rho-neg is cheaper at w = 0
  and 0.1.
* **Lead N4 stays open** (frozen rule): every unit of every arm verified, and the full-range exponent
  gap rho-neg - DL is 0.560 [0.510, 0.600] at w = 0 and 0.545 [0.496, 0.585] at w = 1.

## What ran

* Instances: q = largest prime `< 2^k`, `q = 7 mod 12`, k = 8..22 (223 ... 4194271); seeds 1..5
  (k <= 16), 1..3 (k >= 17); N4's GHS builder unchanged; 63 instances,
  `results/instances.jsonl` (sha256 `94bfd391...b6c`), identical to N4's for k <= 18 (checked at
  q = 262111 seed 1). The W2a list on `origin/codex/diem-durable-checkpoints-20261019` was not used:
  it has no frozen `instances.jsonl` / `FROZEN.sha256`, its partial-validation record lists
  `(k, i) = (20, 0..2), (21, 0..2)` missing, and its construction differs. That branch was not touched.
* Arms (PROTOCOL.md): B0 (N4 code included unmodified), S, L0, L1, L, D, DL1, DL, rho plain,
  rho-neg. Units: 501 IC units, all verified by `kG = Q` on `E0`; 2442 grid rho runs (1221 per arm),
  all verified. Six failed attempts were retried and charged: D/DL1/DL at q = 223 on all 5 seeds
  (too few relations at `c_D = 2.0`, `N = 30`), S and L0 at q = 262111 seed 2 (hold-out violation;
  L1 and L on the same matrix succeeded). `relations_bad = 0` and `holdout_bad = 0` on every
  successful unit.
* Grid: `run_grid.sh` with `params.sh` (PARAMS.md), binary sha256 in the addendum, 294 jobs,
  2026-10-26 01:58Z to 03:12Z, 4 `nice -n 10` processes on an Apple M4 Pro (load average 22-60
  at the time; L0 isolation). Calibration: `v2 calib` three times per size (PARAMS.md deviation).
* Controls: N4's `n4 check` passed (CHECK OK) at every size except k = 22, where it failed its
  homomorphism test with 3 of 8 triples; the cause is N4's check tool computing `(a + b) % ell` in
  u64 with `ell ~ 2^64` (overflow). `v2 homcheck` (same test, 128-bit sum) passes at k = 22 (8 triples,
  0 failures, `#Jac x = 0` on 10 classes) and at k = 21; all 21 k = 22 IC solves (3 instances x 7 arms) also verified.
  Replay (arm c): S/L0/L1/L/D/DL1/DL records identical on a second execution at all 12 replayed
  sizes (k = 8..18 and 20); B0 records differ at all 11 replayed sizes (`results/replay.txt`).
  rho calibration (frozen rule): plain in [1.0, 1.5] at every size except k = 22 (1.640 over 2 runs);
  rho-neg in [0.75, 1.15] except k = 19 (0.713, 12 runs), 20 (0.727, 9 runs), 22 (1.449, 2 runs).
  Those (arm, size) cells are excluded from every ratio, fit and crossover, as frozen; the
  all-size variant is `results/summary_allrho.md`.
* Post-hoc supplement (labelled, not frozen): the k = 22 rho count was cut to one seed by PARAMS.md
  because of the pilot CPU budget; the host was later much less loaded, so seeds 2 and 3 were
  run with 2 runs each (`results/supp-k22-rho.tar.gz`). With them, plain is 1.203 and rho-neg 1.031
  steps/sqrt(ell) at k = 22 (both pass); `results/summary_supp.md` uses them.
* The first grid attempt (2026-10-21) was interrupted and its uncommitted outputs were lost; nothing
  from it is used (addendum).

## Predictions (PROTOCOL.md) against the data

| | prediction | outcome |
|:--|:--|:--|
| P1 | every unit verifies; relations_bad = holdout_bad = 0 | holds (501/501 units; 6 failed attempts charged) |
| P2 | rho exponents 1.50 +- 0.10; neg/plain in [0.60, 0.85] | holds: 1.504, 1.543; 0.715 [0.656, 0.777] |
| P3 | S/L/D/DL replay; S relation mul = B0 within 1%; S relation wall < B0 at k >= 18 | replay holds; mul within 1% only for k >= 14 (1.087 at q = 223 to 1.012 at 8191: S recomputes 4-point lines from their second point; the 1.44 at q = 262111 is the charged S/L0 retry, not the method); wall 0.26-0.48 at k >= 18 (holds) |
| P4 | SGE dimension >= 2x smaller (k >= 16); L1 LA >= 2x below S; L LA in 1-3x L1 and below S | dimension factor 0.74-0.78 (**fails**); L1/S LA 0.20-0.22 (holds); L/L1 LA 3.6 (**fails**, above 3); L/S LA 0.70-0.79 (holds) |
| P5 | D below S at k >= 14; D, DL `e_high` in [0.95, 1.15]; B0, S `e_high` in [1.10, 1.30] | holds: D/S 0.07-0.26; 1.009, 0.988; 1.141, 1.108 |
| P6 | DL vs rho-neg below 2^12; B0 vs rho-neg 2^12..2^14 (M-eq); wall B0 3e4..3e5, DL < 2^15 | holds: 555; 6371; wall 2.77e5; 5704 |

Where the LA gain of L comes from (attribution on the same matrices, LA-phase M-eq relative to S at
q = 4194271): `+-1` skip alone (L0) 0.364 of S's LA (0.445 of S's whole cost), SGE + scalar
Lanczos (L1) 0.218 of S's LA, block Lanczos width 4 (L) 0.79 of S's LA: in a large prime field the
4x4 block operations cost `b^2` per vector entry, so the block arm is 3.6x the scalar one after SGE.
The best measured pipeline is **DL1** (D + SGE + scalar Lanczos): paired DL1/DL = 0.536 [0.536, 0.536]
at q = 4194271 and 0.531 at 2097091 (band-resolved, 0.818 at w = 1), DL1/B0 = 0.022 at q = 2097091;
it is reported as a post-hoc secondary, DL stays the pre-registered headline.

Phase shares at the top size: B0 (k = 21) linear algebra 87%, relations 13%, descent < 1%;
D (k = 22) linear algebra 82%, relations 18%; DL (k = 22) linear algebra 64%, relations 35%;
DL1 (k = 22) relations 65%, linear algebra 34%. The descent is negligible for every D arm (the
root component of the large-prime graph holds most rational points), 0-1% for k >= 17.

## Improvement statement (AGENTS.md success criterion)

Baseline: N4's frozen B0, same instances. Band-resolved improvements at every size k >= 12: L
(0.76-0.79x), D (0.08-0.26x), DL (0.04-0.20x), DL1 (0.02-0.15x, post hoc). S is engineering
(cost-neutral, bit-for-bit replay, O(N) instead of O(N^2) memory, which is what made k = 22
runnable for the S-family; B0 needs 4.7 GB at k = 21). New ratchet baseline: **DL1**, with DL as the
registered headline. Distance to rho-neg at the top size: DL 0.004, DL1 0.002 (k = 22, supplement).

## Deviations and limits

1. Timing ratios: N4's `ratios()` gave unusable values under contention (PARAMS.md); M-eq uses the
   robust `v2 calib` median per size. Sensitivity with N4's Xeon constants:
   `results/summary_xeonconst.md`. The k = 17 `lmul/mul` step is a property of the u128 remainder on
   this host and moves every LA-dominated exponent.
2. CPU time added (addendum); the uncharged `Pic` relation checks of the new arms are excluded from
   phase times; B0's matrix-phase wall still contains its own checks (small: about 10-20 ms).
3. B0 not run at k = 22: N4's `pack()` needs `q < 2^21` and N4's `la()` overflows for `ell > 2^63`
   (and its global hash table would need about 9 GB).
4. Mod-`ell` inversions in the new code are charged 60 lmul each; B0's Lanczos inversions are
   uncharged (N4); the effect is below 0.1% of LA.
5. rho is single-trail with one inversion per step and no simultaneous inversion; a batched
   rho would be cheaper in M-eq. rho at k = 19..22 has 2-12 runs per size, so its per-size means
   are noisy; the pooled-calibration model is given next to every measured mean in `summary.md`.
6. Exponent intervals are narrow because the IC cost varies little between instances; they do not
   include the conversion-constant uncertainty (deviation 1).
7. Analysis-side changes after the grid (grid binary unchanged): the summarizer gained the
   frozen rho-calibration exclusion, the P3/P4 table, the DL1 pairs and the CPU / fixed-constant
   modes, and `v2 homcheck`; every reported number comes from that summarizer (`v2_sum`, built
   from the committed `summ2.rs`).
8. Not done: an external-implementation comparison (no Magma/Sage/PARI installed); k > 22 (needs
   128-bit `ell` arithmetic in the builder and solver); the W2a instance list (incomplete).

## Reproduce

```bash
cd research/quartic_n4v2_20261020
rustc --edition 2021 -O v2.rs -o v2           # driver (includes N4 sources by path)
./v2 qlist 8 22; ./v2 ic 65479 1 B0,S,D 3.0 2.0 4 4; ./v2 rho 65479 1 0 2 plain,neg; ./v2 calib 65479 1
./run_grid.sh $PWD/v2 $PWD/../quartic_n4_20261019/n4 $PWD/results/grid-X $PWD/params.sh
tar xzf results/grid-20261025/units.tar.gz -C results/grid-20261025
U=results/grid-20261025/units
./v2 summarize results/summary.json $U/ic_*.jsonl $U/rho_*.jsonl $U/calib_*.jsonl > results/summary.md
N4V2_TIME=cpu ./v2 summarize ...      # CPU-time variant;  N4V2_FIXED=15,1.1 ...  Xeon constants
./v2 replaycmp $U/replay_65479_1_SD.jsonl $U/ic_65479_1_SD.jsonl
python3 reports/plot.py results/summary.json reports "caption"   # layout only
```
