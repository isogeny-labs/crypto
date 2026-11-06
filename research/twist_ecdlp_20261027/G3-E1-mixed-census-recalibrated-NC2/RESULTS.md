# RESULTS G3-E1-mixed-census-recalibrated-NC2

Every number below is copied from a file in this directory produced by running code: `verdict.txt`, `results.json`, `summary.tsv`, `pooled.tsv`,
`controls_summary.tsv`, `nc2_cells.tsv`, `f4_table.tsv`, `ops.tsv`/`ops.json`, `raw/*.out`. Seed 20261026 (bootstrap B = 2000, same seed family). PARI/GP 2.15.4 only;
Python only drew the three figures from the TSVs. No ECDLP cost claim, no rho arm: this is a structural census of Semaev solutions. Preregistration unchanged
(sha256 e8fc1a52...73fd, `PREREGISTRATION.sha256`). Deviations and disclosures: `DEVIATIONS.md` (B1-B5).

## Verdict (outcome rule of PREREGISTRATION section 9, computed by g3_aggregate.gp)

**DECISIVE_SUPPORT** for "no non-degenerate mixed solutions when n_T = 0 (any m), none for m = 2, and for m = 3 mixed solutions equal the structural predictor S exactly".
Gates: G0 PASS, G1 PASS, G2 PASS, G3 PASS (32/32 cells), G4 PASS, G5 PASS (21/21 re-run jobs byte-identical TGT lines, `raw_rep/g5.txt`). F1, F2, F3, F4 did not fire.
R20 RESOLVED; **R24 UNRESOLVED** (explicit limitation: the rate claim Rt is not tested at 24 bits; support is stated for q up to 2^20).
**Rate hypothesis H3 / F4 category: C by the operational rule (verbatim: "16-bit inside=1 excludes=0; 20-bit inside=0 excludes=0 ; interval overlaps the band: H3 not confirmed, not refuted")**.
So the exact claims (H1/Z) are decisively supported; the 20-bit rate constant is consistent with c_ref but too imprecise to confirm [0.8,1.25] (see F4 below).

## Run size (results.json, verdict.txt)

3312 grid configs (1248 jobs, 13 primes x 12 curves x 2 m x 4 FB), 508,800 targets, jobs_not_run 0, configs_incomplete 0, configs_truncated_or_nonok 0;
NC2: 624 jobs, missing 0, truncated 0; resolution blocks: 36 jobs, 960,000 targets (R20 240,000 over 12 world-FB cells; R24 720,000), missing 0, truncated 0.

## Controls and gates (controls_summary.tsv)

- G0: V0 640 rows, 0 bad; toy V = F_q (q = 101, 103) 320 rows, 0 bad; 16 resultant cross-checks, 0 bad.
- G1 positive controls: PC1 800/800 found and ok, PC2 600/600, PC3 1200/1200 (sensitivity 100%).
- G2 NC1: 480/480 oracle-checked lines ok and 840/840 detector-only lines ok, mixed = 0.
- G4 NC3 (targets on the twist): 72 lines, 0 bad, 0 target mismatches, NC3 m = 2 "mixed" = 0.
- G3 NC2-calibrated: 32/32 cells pass; the largest absolute interval end of D over all cells is 0.01443246 (equivalence margin 0.05); mean null mixed share pi_t about 0.75 (m = 2)
  and 0.875 (m = 3) (power condition >= 0.3 met in every cell). Example cells: 12-bit farey m = 2 D = -0.005413 [-0.014432, 0.003356]; 20-bit random m = 3 D = -0.0001004 [-0.000928, 0.000794];
  24-bit random m = 3 D = -0.001746 [-0.005656, 0.001616] (all 32 in `nc2_cells.tsv`; figure `fig_nc2_D.svg/png`).

## Exact claims Z / H1 (pooled.tsv)

- m = 2, all n_T: M_nd = 0 and M_deg = 0 in all 12 (n_T, FB) strata of 148 configs, 29,600 targets each; E_nd about 1.3e6 - 2.8e6 per stratum; rule-of-three upper bound on f at most 1.88e-6 per stratum.
- m = 3, n_T = 0: **N_E_nd = 179,694,796** (>= 1e7 required), M_nd = 0 in all 4 FB strata (128 configs, 12,800 targets each), M_deg = S_deg exactly (151,104 / 160,257 / 161,876 / 160,640); f <= 8.28e-8 (CP0 bound, farey).
- m = 3, n_T in {1, 3}: **target mismatches N_M != S = 0** in grid and in resolution (F3 total 0). Pooled M_nd = S_nd, e.g. n_T = 3: farey 31,461, interval 32,110, random 30,386, twistmult 11,842;
  n_T = 1: farey 9,933, interval 11,117, random 9,714, twistmult 952. f (Wilson) n_T = 3: random 6.53e-4 [6.46e-4, 6.60e-4], interval 7.17e-4, farey 7.14e-4, twistmult 2.15e-4.
- E1-literal test (Wilson lower bound of f > 5/q) fires in 44 m = 3 configs (11 random + 13 interval + 16 farey + 4 twistmult, all n_T = 3); reported per preregistration, it is not a falsifier.

## Rate claim Rt / F4 (f4_table.tsv; ratio = pooled c_hat / c_ref with hierarchical 95% interval)

| stratum (random+interval+farey, m = 3, n_T > 0) | observed mixed M_nd | expected S_ref | ratio c_hat/c_ref | 95% interval | vs. naive 1.5 n_T/q |
|---|---|---|---|---|---|
| 12 bits (grid) | 116,362 | 117,907.6 | 0.9965 | [0.9458, 1.0476] | 1.0024 |
| 16 bits (grid) | 8,328 | 8,417.6 | 0.9935 | [0.8302, 1.1412] | 0.9923 [0.8396, 1.1493] |
| 20 bits (grid, used for category) | 31 | 36.79 | 0.8465 | [0.2934, 1.4702] | 0.8618 |
| R20 random | 486 | 486 (S_nd) | 0.9220 | [0.0617, 1.6438] | 0.9864 |
| R20 farey | 699 | 699 | 1.3318 | [0.4257, 2.0556] | 1.3991 |
| R20 random+farey | 1185 | 1185 | 1.1265 | [0.3288, 1.8424] | 1.1942 |
| 24 bits (grid) and R24 (720,000 targets) | 0 | 0.061 / unresolved | n/a | n/a | UNRESOLVED |

Reading: at 12 bits the pooled constant equals the refined c_ref (ratio 0.9965, interval inside [0.8,1.25]); E1's apparent 12-bit excess (about 3.6 versus 1.5) is therefore explained by the realised composition
c_ref = 1.5 (nW/nE)^2 (#E/q), not by a larger true constant. At 16 bits the interval is inside the band. At 20 bits both the grid and the R20 intervals contain 1 and the band but are wide
(few primes and curves as resampling units), so H3 at 20 bits is neither confirmed nor refuted; Rt (factor-2 window [0.5, 2]) is not refuted anywhere (F4 fired strata: none). The twistmult stratum is excluded from categories
(DEVIATIONS B3). The 24-bit cells are UNRESOLVED (grid S_ref expectation 0.061; R24 observed 0 mixed and 0 structural over 720,000 targets, below the preregistered >= 10).

## Operation counts (primary cost metric; ops.tsv, ops.json, formulas verified in raw/opcount_verify.out)

Unit: S_3 root-set evaluations ("pkeys" calls; each a fixed small number of field multiplications and one inversion). Detector A cost per config: m = 3 table build n(n+1)/2, plus n per target (m = 2 and 3).
Instrumented check: counted = formula in 5/5 test configs (e.g. n = 231, m = 3, 20 targets: table 26,796, targets 4,620). Oracle S work (the comparator) is not counted.

| stage | bits | m | configs | targets | table pkeys | target pkeys | total pkeys |
|---|---|---|---|---|---|---|---|
| grid | 12 | 2 | 576 | 115,200 | 0 | 30,151,000 | 30,151,000 |
| grid | 12 | 3 | 576 | 57,600 | 34,822,668 | 15,075,500 | 49,898,168 |
| grid | 16 | 2 | 576 | 115,200 | 0 | 286,844,400 | 286,844,400 |
| grid | 16 | 3 | 576 | 57,600 | 357,963,478 | 56,106,000 | 414,069,478 |
| grid | 20 | 2 | 432 | 86,400 | 0 | 662,133,600 | 662,133,600 |
| grid | 20 | 3 | 288 | 28,800 | 206,877,492 | 32,941,200 | 239,818,692 |
| grid | 24 | 2 | 192 | 38,400 | 0 | 380,481,600 | 380,481,600 |
| grid | 24 | 3 | 96 | 9,600 | 108,072,000 | 14,400,000 | 122,472,000 |
| nc2 (all bits, per-bit rows in ops.tsv) | 12-24 | 2, 3 | 624 | 93,600 | 205,705,783 | 298,547,400 | 504,253,183 |
| res | 20 | 3 | 24 | 240,000 | 27,018,000 | 360,000,000 | 387,018,000 |
| res | 24 | 3 | 12 | 720,000 | 13,509,000 | 1,080,000,000 | 1,093,509,000 |
| **all stages** | | | 3,972 | 1,562,400 | 953,968,421 | 3,216,680,700 | **4,170,649,121** |

Secondary, contention-affected wall seconds summed over jobs (2 workers on a loaded 4-core box):
table build 8,806 s, target loops 30,871.5 s. Whole pipeline about 6.1 h wall.
Candidate-tuple counts are not recorded by the frozen detector; the recorded solution counts (E_nd, E_deg, M_nd, M_deg per stratum above) are the post-deduplication solution tuples.

## Reproduction

`/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G3-E1-mixed-census-recalibrated-NC2/run.sh` is the single resumable driver for everything (full run: about 6 h wall on the shared box;
skips finished jobs, so re-invoking it on this directory only redoes the aggregation (about 12 min), plots and operation counts). Reduced replication, measured 4 min 0.7 s wall (user 4 min 54 s):
`replicate_subset.sh` re-runs stage 0, the 12-bit positive controls, 8 grid jobs (12 and 16 bits, m = 2 and 3, all FB types) and 4 NC2 jobs and compares the deterministic lines byte-for-byte with `raw/`:
13/13 SUBSET_OK plus the stage-0/PC checks (`raw/replicate_subset.log`). The 21 preregistered G5 re-runs (`raw_rep/g5.txt`) are a larger such check.
Figures (recorded values only): `fig_mixed_vs_predictor.*`, `fig_nc2_D.*`, `fig_f4_ratio.*`.

## Limitations

- Rt is not tested at 24 bits (R24 unresolved); 20-bit rate intervals are wide, so the [0.8, 1.25] band is neither confirmed nor excluded there.
- Structural zero evidence concerns this detector on these parameters (q up to 2^24, FB sizes at most 1500 for m = 3); it does not make an ECDLP statement either way.
