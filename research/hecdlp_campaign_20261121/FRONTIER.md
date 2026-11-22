# Frontier after campaign round 1 (2026-11-21)

Best measured index-calculus arm against the matched negation-map rho, by genus and curve kind,
with the stack each lead added. Every figure is `[pcc:unverified]`; all runs on the shared 4-vCPU
container; FPM primary reading (`cantor_mul + 10·cantor_inv + rel_mul + g²·la_mul`, the quartic arms
with `pic_*`). "Below rho" holds in the FPM band; the §5a GAE band stays unresolved (the LA price
decides). Replays by C5 reproduce the genus-3 composition rows field for field (`VALIDATION_REPORT.md`
in the two studies); everything else is a single producer's measurement.

| genus / kind | best arm | over rho | at p | change this round | source |
|---|---|--:|--:|---|---|
| 2, hyperelliptic | 2LP + prefilter | 3–5 | 2^24 | none (no lead targeted it) | `genus_ladder_scale_20261115` |
| 3, hyperelliptic | 2LP + prefilter + SGE(16,3) | 0.255 | 32771 | C1: w_max 16 (was 0.283 with SGE(8,3)); the LA drift 0.32→0.39 stops at 0.31–0.32 | `genus_ladder_sge_depth_20261121` |
| 3, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.416 [0.336, 0.678] / 0.507 [0.354, 0.823] / 0.293 [0.167, 0.623] | 65537 / 131101 / 262147 | C4: orders by BSGS from N_1 alone (order search 6.5 s instead of 204 s at 8209; genus 3 reaches p ≈ 2·10^6); the ratio stops falling; exponents over 8209…131101: SGE arm 1.418 [1.411, 1.425], rho 1.243 [1.069, 1.421] | `genus_ladder_fast_order_20261121` |
| 3, hyperelliptic | same, cc 1.5 | 0.368 | 16411 | C2: cc 1.5 gains 0.8–3.7 % over cc 2.0; not a lever | `genus_ladder_cc_sweep_20261121` |
| 3, plane quartic (random) | Diem 2LP + algebraic decomposition + SGE(8,3) | 0.0070 [0.0037, 0.0133] | 8209 | C3: decomposition by conic resultant (`rel_mul` 0.20 of the scan), FPM 0.54 of the old arm (was 0.013 of quartic rho); arm now LA-bound | `quartic_diem_alg_20261121` |
| 4, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.0198 | 4099 | C2: first SGE point at genus 4, p = 4099 (was 0.0375 without SGE); C1: w_max 16 is worse at genus 4 | `genus_ladder_cc_sweep_20261121` |
| 4, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.061 | 2053 | C1 (p = 1031: 0.068) | `genus_ladder_sge_depth_20261121` |
| 5, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.004 | 1031 | C1 (was 0.0084 without SGE) | `genus_ladder_sge_depth_20261121` |

## What the round established
- **Merge depth is not a lever beyond w_max 16** (C1). Non-zeros grow 1.3×, 2.0×, 3.0× for
  (8,3), (16,·), (32,3) at genus 3, p = 32771, and Wiedemann pays for the density; the merge's own
  work is negligible. fill_max 3 beats 10. The Wiedemann multiplication ratio tracks
  n·(generic entries + n) within 0.016 in all 30 rows (descriptive, six cells).
- **The base constant is not a lever for the SGE arm** (C2). cc = 2.0 is within a few percent of
  the optimum at genus 3 and 4; SGE moves the optimum back toward 2.0 relative to the unmerged arm
  swept on main (PR #4852: cc 1.5 at κ ≈ 2). At genus 4 the base saturates at the p/2 cap near cc 4.
- **Diem on quartics is now linear-algebra bound** (C3). At q = 8209, 9·la_mul is 58 % of FPM_mid;
  SGE(8,3) removes only 11 % of la_mul because the cores are small (about 290 × 205). Three of five
  predictions failed (ratios 0.542 and 0.0070 against 0.5 and 0.005; exponent 1.31 [0.94, 1.61]).
- **Genus 3 at p = 32771 is replayed** (C5): 36 IC rows, 18 unmerged, 18 rho rows, 9 instances with
  0 mismatches; the analyzer double-counted the unmerged arm (intervals too narrow, means unchanged;
  errata in both studies).
- **The genus-3 ratio does not keep falling** (C4). With the order search made cheap (BSGS over the
  one-dimensional Weil window from N_1; `count_points(2)` 2.3–2.5× faster by Barrett reduction,
  counts unchanged), genus 3 was extended to p = 262147: SGE arm over rho 0.416, 0.507, 0.293 at
  65537, 131101, 262147 with wide intervals (3 curves × 2 seeds), and the SGE arm's exponent
  (1.418) sits above the rho fit (1.243 [1.069, 1.421]) over 8209…131101. The campaign target
  ≤ 0.25 at 2^17 is missed (O2 falsified). At genus 4 the N_1-only BSGS is 4.4–4.5× slower than
  the N_1 + N_2 route: a negative; genus 4 reaches about 2^15.7 with the faster count only.

## Distance remaining and next levers
At genus 3 the arm sits at a quarter to a half of rho in the FPM reading, and over 8209…131101 its
exponent (1.42) is above the rho fit (1.24, wide): a constant-factor gain that shrinks with p unless
more curves per prime say otherwise. The linear algebra after the merge is now the whole
lever at genus 3 and on quartics: block Wiedemann with a wider block, or a sparse solver that
exploits the two-large-prime structure before the merge (the core's generic entries dominate the
cost, per C1's model check). The relation phase's Cantor steps are the lever at genus 2, where no
arm is below rho.

## Round 2 opening analysis (2026-11-21, main session, from C4's rows) [pcc:unverified]

**Cost shares of the genus-3 SGE(8,3) arm** (FPM_mid): the linear-algebra share rises 0.48, 0.53,
0.57, 0.60, 0.64, 0.68 from p = 8209 to 262147 while the Cantor walk falls 0.24 → 0.13 and the
smoothness test 0.22 → 0.16. Over the 32× range of p: `la_mul` scales as p^1.52, the relation
phase (`cantor_mul`) as p^1.25, `core_cols` as p^0.69 (theory 2/3), so the average weight of the
merged core grows about p^0.15. The LA, not the relation phase, caps the ratio.

**Expected-rho reference.** Replacing the six measured rho runs per prime by the expected
negation-rho cost (0.886·√r steps × the measured 219 `cantor_mul` and the measured inversions per
step) gives SGE arm / expected rho = 0.333, 0.546, 0.388, 0.434, 0.358, 0.342 at p = 8209, 16411,
32771, 65537, 131101, 262147. Measured rho ran at 1.15, 1.43, 1.37, 1.04, 0.71, 1.16 of its
expectation, which is what produced 0.290 at 8209 and 0.507 at 131101. The de-noised line is flat
at 0.33–0.43 across a 32× range of p where the asymptotic p^{−1/6} would predict a fall by 1.8×.

**Round 2 leads.** R2: 9 curves per prime at 65537–262147 to pin both exponents (with the expected
reference as a second reading). R3: relation excess with lightest-row selection before SGE, to cut
the merged core's weight (the only lever the shares point at). Not pursued: deeper merge (C1) and
the base constant (C2), both closed.

## Round 2 results (2026-11-22) [pcc:unverified]

| genus / kind | best arm | over rho | at p | change | source |
|---|---|--:|--:|---|---|
| 3, hyperelliptic | 2LP + prefilter + SGE(8,3) | 0.300 [0.223, 0.420] / 0.399 [0.288, 0.534] / 0.377 [0.263, 0.565] (expected rho: 0.330 / 0.393 / 0.324) | 65537 / 131101 / 262147 | R2: 9 curves per prime; flat, no resolved trend; exponents rho 1.360 [1.254, 1.464], SGE arm 1.422 [1.418, 1.425], unmerged 1.347; the data do not separate the exponents | `genus_ladder_g3_highp_20261121` |
| 3, hyperelliptic | + relation excess, keep lightest fraction K ≥ 0.5 | 1.8–5.2× the plain arm | 32771–131101 | R3: negative; Wiedemann pays per row (`la_wied_mul ≈ c·cols·(generic + rows)`, c 4.9–6.0), so an enlarged core costs more however light its rows | `genus_ladder_light_rows_20261121` |
| 3, hyperelliptic | + excess 0.2, keep only the lightest cols + margin rows (post hoc) | FPM 0.877 / 0.866 / 0.843 of plain; `la_mul` 0.63–0.65 | 32771 / 65537 / 131101 | R3 post-hoc lead, same 3 curves: gain grows with p; preregistered on fresh instances as R4 | `genus_ladder_light_rows_20261121` |

What round 2 established: the genus-3 arm is a constant factor 2.5–3.3 below rho across 8209–262147
with no measurable exponent gap, and the merged core's row count, not its weight, prices the LA.
Relation filtering that leaves the core at cols + margin but lighter is the one lever that moved
(about 15 % at 131101, post hoc); it is the subject of R4.

## Round 3: the light-row floor, preregistered (2026-11-22) [pcc:unverified]

R4 (`genus_ladder_light_floor_20261122`) tested R3's post-hoc variant on six curves per prime that R3
never saw (R2's curves 4 to 9), protocol frozen before data. Relation excess E, then keep only the
lightest cols + margin rows after peel, then SGE(8,3) and Wiedemann. 180 runs certified; plain rows
equal R2's.

| p | FPM_mid / plain, E = 0.2 | E = 0.3 | `la_mul` / plain | best arm over rho | plain over rho (same curves) |
|--:|--:|--:|--:|--:|--:|
| 65537 | 0.860 [0.855, 0.865] | 0.857 | 0.64 | 0.226 | 0.263 |
| 131101 | 0.848 [0.843, 0.853] | 0.841 | 0.65 | 0.303 | 0.360 |
| 262147 | 0.829 [0.825, 0.833] | 0.809 [0.805, 0.814] | 0.65 | 0.356 [0.254, 0.588] | 0.439 |

**A measured improvement over the index-calculus baseline** [pcc:unverified], confirmed out of sample: 14 to 19 % off
the genus-3 arm, growing with p. Predictions L1 to L4 held; L5 (≤ 0.33 of rho at 262147) failed at
0.356. The selection lowers the LA level, not its exponent (`la_mul` exponent 1.51 for both); the
arm's FPM exponent falls from 1.43 to 1.39 because the relation phase takes a larger share.
Disclosed deviations: the frozen command lacked `--curves 6` (a partial run on two curves was analysed
before the rerun) and the analysis script got a key-name fix; both in `AMENDMENT.md`.

### R5: the floor does not carry to genus 4 and 5 [pcc:unverified]

`genus_ladder_light_floor_g45_20261122`, same option on the earlier genus-4 and genus-5 instances, 105
runs certified, plain rows identical. Best-E FPM_mid over plain: 0.984 [0.958, 1.002] at genus 4,
p = 2053; 0.963 [0.950, 0.971] at genus 4, p = 4099; 1.072 [1.063, 1.082] at genus 5, p = 1031. G2 to
G5 failed. Cause: at these primes the plain merged core already holds only 17 to 32 rows beyond its
columns, so the floor removes 1 to 2.5 % of the rows, and the LA share is lower (0.52, 0.60, 0.23)
than at genus 3, p = 262147 (0.67). The lever scales with the core's size; it is a genus-3, large-p
effect on present evidence.
