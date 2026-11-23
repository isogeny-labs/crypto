# R7.2: is any toy binary elliptic curve weak under GHS, end to end, under a counted cost model?

**Date.** 2026-11-01. **Branch** `research/r7-2-ghs-end-to-end`, on `main` after PR #2542 (R7.1, merged).
**Code.** `src/cryptanalysis/ghs_ic.rs` (factor base, relation collection, linear algebra mod ell, extraction),
`MumfordDivisor::add_unchecked` and `from_unreduced_unchecked` (`src/binary_ecc/hyperelliptic.rs`),
`SmoothCover::image_alg` (`src/cryptanalysis/ghs_smooth.rs`), `examples/r7_2_ghs_end_to_end.rs` (instances, rho and BSGS on E,
pricing, statistics, `profile`, `pick`, `fit`).
**Labels.** *measured*: a count or a time printed by the example (raw JSON in `raw/`). *derived*: arithmetic on measured values
(shown). *recalled*: a literature statement not re-checked here. *extrapolated*: a fitted line read outside the measured range.
Toy fields only (N = 12 to 28). No claim about any deployed curve. Conductor is not installed in this container, so no task was
reserved (as in R7.1).

## 0. Hypothesis, falsifier, verdict

**H.** At some toy instance with N <= about 18, Gaudry-style index calculus (IC) on the descended Jacobian solves a planted ECDLP
on E(F_{q^n}) with fewer counted group operations than Pollard rho, and than the best generic method on the same E.
**Falsifier.** At every measured instance the total counted cost (relation collection with the factor base + linear algebra + final
extraction) is at least the rho cost, or IC fails to solve.

**Verdict: H is refuted. No instance is weak.** 30 applicable instances, N = 12 to 28 (the requested N <= 18 range, and larger
instances added to see the trend), 600 planted logs (20 per instance, paired seeds). IC solved and verified on E all 600; rho and
BSGS solved all 600 as well. In bitop/v1 gate equivalents, expressed in E additions of the same curve ("gae"):

* **Total cost, IC / best generic method:** from 1.91 [1.72, 2.11] (N = 28, q = 128, g = 4) to 131 [123, 141] (N = 12, q = 4, g = 8),
  all 30 intervals above 1 (derived from the 20 paired logs, 95% percentile bootstrap, 10^4 resamples). At N <= 18 the smallest
  ratio is 9.18 [8.49, 9.96] ((l,n) = (4,4), q = 16, g = 4, N = 16). Against Pollard rho specifically the smallest ratio is 1.60 [1.25, 2.12]
  (N = 28); the best generic method was BSGS on 28 instances and tuned table-rho on 2.
* **The ratio falls with N only in one family.** For genus 4 over F_{q^4} (n = 4, N = 4l, q = 2^l) the ratio falls from 9–12 at q = 16
  to 1.9 at q = 128 (slope -0.83 [-0.94, -0.75] in log2 per doubling of q). The same fit crosses 1 at l = 8.1 [7.8, 8.4], N = 32
  [31, 33] (**extrapolated**, one step beyond the last measurement; the library's irreducible-polynomial table stops at degree 31, so
  N = 32 could not be built here). For genus 4 over F_{q^3} (n = 3, N = 3l) the ratio is flat at 18 to 30 from q = 16 to q = 256
  (slope -0.12 [-0.18, -0.03]); its fitted crossing, l = 43 [31, 141], is not supported, because the linear algebra phase grows as
  q^2.1 against q^1.4 for the generic methods (section 5).
* **Counting only Jacobian additions** (one Jacobian addition = one E addition, everything else free) puts IC below the generic
  method for n = 4 from N = 20: ratio 0.83 and 0.93 (N = 20), 0.41 (N = 24), 0.21 (N = 28); for n = 3 it is 0.90 and 1.15 at N = 24
  and above 1.2 for N <= 21. This is a stage diagnostic, not a speed (AGENTS.md section 2): the library's Cantor addition costs 5.4 to
  10.4 E additions at g = 4, and the other phases are priced (section 4).
* **The (l,n) = (3,4), g = 8, q = 8 run was not hung.** R7.1's own example finishes it in 1175 s (one curve, 20000 steps). The time
  is point counting up to N_9 over F_{2^27} (about 9 min) plus a 400000-element subgroup closure with validated additions (about
  14 min); nothing loops and `ghs_smooth.rs` has no bug (section 6). IC on that instance: ratio 122 [104, 143].
* **Not weak, and not claimed otherwise:** nothing here is a sub-rho claim at a cryptographic size or a statement about a
  deployed curve.

## 1. The counted cost model

* **Unit.** `bitop/v1` (`docs/units/BITOPS.md`, `src/cryptanalysis/bitops.rs`): binary-field add `n`, mul `n^2 + (n-1)^2 + 4(n-1)`,
  squaring `4(n-1)`, inversion `2n * 5n`; E group add `I + 2M + S + 9A`, double `I + 2M + 2S + 5A`; hash probe `4n`; arithmetic mod ell
  as prime-field arithmetic at ell's width (`modr_mul = 2(w^2 + 5w(w-1)) + 13w`, `modr_add = 13w`, `modr_inv = 2w * 13w`).
  Every cost below is divided by the gates of one E addition on the same curve ("gae", E-add equivalents); `S = gae / sqrt(ell)`.
* **E side (rho, BSGS, verification).** Group operations are counted by a wrapper and priced by the formula. Check (measured):
  400 E additions run under the field-operation ledger and priced from the recorded multiplications, squarings, additions and
  inversions equal 400 x the `GroupAdd` formula, ratio 1.000 (`raw/selftest.json`).
* **Jacobian side.** Every field operation over F_q of the library's Cantor arithmetic is recorded by `OperationTally` and priced in
  the same unit; an inversion is priced as `field_inv` with the multiplications and squarings recorded inside it removed (so it is
  not charged twice). `add_unchecked` skips the operand and result re-validation of `MumfordDivisor::add` (a defensive check; `docs/units/BITOPS.md`
  puts the share of such re-checks at 39 to 59% of the counted gates for the prime-field implementation, not measured here for this binary-field one;
  the two agree on 200 random sums, tested). Phi(Q) is computed over K
  with `image_alg`, which also uses no re-validation and records every Frobenius squaring.
* **Phases of one IC solve (online, per target).** Phi(Q) over K; step set-up (doubling tables of D1 = Phi(P) and D2 = Phi(Q) and the
  8 walk steps); relation search (walk + smoothness tests); linear algebra mod ell; verification [d]P = Q on E (about 1.5 w E-operations by
  double-and-add, charged to every arm). Arithmetic mod ell is counted for the non-zero entries a sparse elimination would touch.
* **Online vs cold.** The headline is the online cost per target. Reusable setup (factor-base enumeration, Phi(P)) is priced and reported
  separately (0.4 to 95 gae and 85 to 1940 gae); the cover construction (`SmoothCover::new`) is **not priced** (the library finds the
  embedding of F_q in K by search; wall time recorded, 0 to 7 s) and is excluded from both. The cold ratio (online + both priced setups)
  is in the arms table; it differs from the online ratio by 0.4 to 6.5 percent at N >= 20.
* **Not in the unit** (AGENTS.md "what the unit leaves out"): memory (BSGS keeps sqrt(ell) points, IC keeps B rows of B+2 residues), control
  flow, allocation, the random-number generator. Wall times are secondary and contended (shared 4-core container, other jobs running).
* **Generic reference arms, same unit.** *Table rho*: r = 8 adding walk, every visited point kept (so the collision is seen at once:
  the cheapest a walk of this kind can be counted; real rho needs distinguished points and costs more), multiplier width tuned on the
  10 tuning logs, 1 hash probe per step. *BSGS*: m = ceil(sqrt(ell)), table of m points. The best generic method of an instance is the
  one with the lower holdout mean. **ecbench was not used**: a hyperelliptic-Jacobian IC arm does not exist there and adding one is
  `ecbench-extend` work not done here (I did not check whether its rho covers N = 12 to 28 binary curves); these numbers are therefore not ecbench
  sessions, carry no replay certificate and are not eligible for the scoreboard or leaderboard. The table-rho and BSGS arms are lower bounds on a distinguished-point rho in the same unit.

## 2. What was built

* **`ghs_ic::solve_dlp`.** Factor base = F_q-points of C, one column per x-coordinate (a point and its negative share a column,
  opposite signs); ramified points (2-torsion) are allowed as factors but have no column. If ell is odd and ell || #J, the ell-primary part
  of Jac(C)(k) is cyclic, generated by D1, so every exact relation projects to a relation mod ell and **no multiple of #J is needed**
  (this is why the attack never counts points of C; wrongly assumed hypotheses are caught by the final verification, and by the
  echelon form's two inconsistency tests). Two walks: *dense* (steps `[a]D1 + [b]D2 + D_c`, T ranges over <D1, D2, factor base>, dense
  rows) and *sparse* (steps `[a]D1 + [b]D2`, T stays in <D1>, rows have at most 2 + g entries, Gaudry's relations). The first echelon row
  with its pivot at mu_1 gives `d = -1/c` from `mu_1 + c mu_2 = 0` (mu_1 = 1, mu_2 = d).
  Smoothness test: trial division by the factor-base points, preceded by `X^q = X (mod u)` (l squarings mod u) when the base is large
  (`B > 2 l deg u`); a u with a repeated root fails that pre-test and is skipped.
* **Tuning (10 logs, never reused for the holdout).** For each instance the relation mode (dense or sparse) and the factor-base size
  (sizes tried from all columns downward, stop after two sizes that do not improve; a size that cannot solve 8 of 10 logs within 10 x the best mean
  number of positions is rejected: 3 of 155 configurations) were chosen by the lowest mean online cost; rho's multiplier width the same
  way. Result: **sparse in 30 of 30 instances; the whole F_q-point set as factor base in 26 of 30** (the other four have 2 or 3 columns
  available and chose 1 or 2); in the sparse scans the mean cost falls monotonically as the base grows for all 25 instances with 4 or more columns, so **the optimum sits at the boundary (all F_q-points)**: a larger base is not available; in the five instances with 2 or 3 columns the sizes tie within 1% except one (q = 4, 0x58: 1 column costs 52% more) (`fig4`).
* **Tests.** `cargo test --release --lib -- ghs_ic ghs_smooth hyperelliptic`: 96 passed, 0 failed, 2 ignored (the 2 are existing
  ignored benchmarks). New: echelon solver on a planted system; `add_unchecked` equals `add` on 200 sums; both relation modes (and a partial
  factor base) recover planted logs at N = 12, q = 16, g = 4; end-to-end recovery on the N = 12, (2,6), g = 8 instance with
  `image_alg == image`. `selftest` of the example: fast #E equals `count_e_points` on four curves; rho and BSGS recover every d in [1, 130].

## 3. Protocol

Per instance: `P` of prime order ell = the largest prime factor >= 7 of #E whose subgroup survives Phi (the (l,n,a,beta) = (3,4,0,0x21)
curve is **not applicable**: ell = 29 does not divide #J = 72, Phi kills it, and no other prime factor >= 7 exists). 10 tuning logs and
20 holdout logs `d` drawn uniformly from [1, ell-1], `Q = [d]P`; IC, rho and BSGS get the same `Q` and the same seed value (paired). Every
solved log is verified by [d]P = Q on E; unsolved or unverified runs would be recorded as failures (there are none). The tallied IC run is
repeated untallied for wall time and asserted to give identical `d` and counts. Instances: the 13 R7.1 curves (N = 12, 14), the (3,4) g = 8
curve, and curves found by `pick` (beta with magic number 3, cofactor at most 8; for n = 4 the cofactor is always 8, a 2-part forced by the
structure). For n = 3, magic number 3 and g = 4 is the generic descended curve; for n = 4 it needs beta in a 3-dimensional sigma-stable
subspace (R7.1's census at (l,n) = (3,4): 448 of 4095 beta). A deliberately selected class, not an unpublished method on honest curves.

## 4. Results

IC "online gae" is the mean over the 20 holdout logs of the sum of the five priced phases, in E additions. Intervals are 95% percentile
bootstrap over the 20 paired logs (ratio of means for ratios).

| instance (l,n,a,beta) | N | q | g | ell (cofactor) | FB cols used/avail, mode | IC online gae [95% CI] | best generic gae | IC / best generic [95% CI] | J-adds / generic E-ops | solved IC/rho/BSGS |
|--|--|--|--|--|--|--|--|--|--|--|
| (2,6,0,0xe2) | 12 | 4 | 8 | 83 (48) | 1/1, Sparse | 2502 [2088, 3012] | 23.1 (bsgs) | 108.11 [90.58, 129.32] | 8.48 | 20/20/20 |
| (2,6,0,0x28) | 12 | 4 | 8 | 131 (32) | 1/1, Sparse | 2332 [2044, 2743] | 27.8 (bsgs) | 83.95 [72.53, 99.16] | 5.87 | 20/20/20 |
| (2,6,0,0x220) | 12 | 4 | 8 | 149 (28) | 1/1, Sparse | 2835 [2519, 3159] | 30.1 (bsgs) | 94.20 [83.96, 105.23] | 7.65 | 20/20/20 |
| (2,6,0,0x221) | 12 | 4 | 8 | 67 (60) | 1/1, Sparse | 1822 [1783, 1863] | 21.5 (bsgs) | 84.66 [78.07, 92.34] | 3.88 | 20/20/20 |
| (2,6,0,0x228) | 12 | 4 | 8 | 29 (140) | 1/1, Sparse | 1852 [1763, 1960] | 14.1 (bsgs) | 131.10 [122.81, 141.08] | 5.66 | 20/20/20 |
| (2,6,0,0x229) | 12 | 4 | 8 | 1049 (4) | 1/1, Sparse | 6379 [5060, 7800] | 61.0 (bsgs) | 104.62 [82.13, 129.95] | 12.41 | 20/20/20 |
| (3,4,0,0xa2) | 12 | 8 | 2 | 7 (592) | 3/3, Sparse | 117 [115, 120] | 6.4 (bsgs) | 18.22 [16.35, 20.58] | 4.77 | 20/20/20 |
| (3,4,0,0x16) | 12 | 8 | 4 | 83 (48) | 3/3, Sparse | 524 [496, 551] | 23.1 (bsgs) | 22.63 [20.65, 24.73] | 3.73 | 20/20/20 |
| (3,4,0,0x3) | 12 | 8 | 8 | 37 (112) | 1/3, Sparse | 2139 [1862, 2451] | 17.5 (bsgs) | 122.14 [104.47, 142.82] | 5.01 | 20/20/20 |
| (3,4,0,0x8) | 12 | 8 | 4 | 37 (112) | 3/3, Sparse | 426 [414, 440] | 17.5 (bsgs) | 24.35 [22.79, 26.12] | 3.51 | 20/20/20 |
| (4,3,0,0x74b) | 12 | 16 | 4 | 1039 (4) | 9/9, Sparse | 1902 [1712, 2119] | 63.2 (bsgs) | 30.07 [26.65, 34.36] | 2.89 | 20/20/20 |
| (4,3,0,0x36f) | 12 | 16 | 4 | 1013 (4) | 9/9, Sparse | 1816 [1674, 1960] | 66.7 (bsgs) | 27.24 [24.31, 30.56] | 2.58 | 20/20/20 |
| (2,7,0,0x4c) | 14 | 4 | 8 | 157 (104) | 2/3, Sparse | 2519 [2421, 2625] | 30.9 (bsgs) | 81.62 [75.11, 89.46] | 4.25 | 20/20/20 |
| (2,7,0,0x4d) | 14 | 4 | 7 | 79 (208) | 1/2, Sparse | 2483 [2363, 2613] | 22.0 (bsgs) | 112.67 [103.69, 122.53] | 8.22 | 20/20/20 |
| (2,7,0,0x58) | 14 | 4 | 8 | 677 (24) | 2/3, Sparse | 3085 [2870, 3333] | 58.3 (bsgs) | 52.91 [47.54, 59.36] | 4.57 | 20/20/20 |
| (5,3,1,0x31a6) | 15 | 32 | 4 | 16349 (2) | 16/16, Sparse | 4298 [3774, 4799] | 229.2 (bsgs) | 18.76 [16.00, 21.59] | 1.62 | 20/20/20 |
| (5,3,1,0x1df9) | 15 | 32 | 4 | 16451 (2) | 14/14, Sparse | 5451 [4904, 6027] | 207.0 (bsgs) | 26.34 [23.29, 29.62] | 2.31 | 20/20/20 |
| (4,4,1,0x2944) | 16 | 16 | 4 | 8191 (8) | 7/7, Sparse | 1585 [1476, 1706] | 172.6 (bsgs) | 9.18 [8.49, 9.96] | 1.44 | 20/20/20 |
| (4,4,1,0x3d2c) | 16 | 16 | 4 | 8179 (8) | 7/7, Sparse | 1725 [1586, 1857] | 149.4 (bsgs) | 11.54 [10.57, 12.63] | 1.83 | 20/20/20 |
| (6,3,0,0x37711) | 18 | 64 | 4 | 32801 (8) | 39/39, Sparse | 7291 [6451, 8103] | 305.0 (bsgs) | 23.91 [20.42, 27.56] | 1.69 | 20/20/20 |
| (6,3,1,0x1e702) | 18 | 64 | 4 | 65563 (4) | 33/33, Sparse | 9652 [8731, 10490] | 408.7 (bsgs) | 23.61 [21.11, 26.40] | 1.73 | 20/20/20 |
| (5,4,0,0x48eb3) | 20 | 32 | 4 | 131023 (8) | 15/15, Sparse | 3528 [3224, 3814] | 553.9 (bsgs) | 6.37 [5.60, 7.18] | 0.93 | 20/20/20 |
| (5,4,0,0x90f2d) | 20 | 32 | 4 | 131129 (8) | 15/15, Sparse | 3382 [3168, 3612] | 585.9 (bsgs) | 5.77 [5.18, 6.50] | 0.83 | 20/20/20 |
| (7,3,0,0x11ceab) | 21 | 128 | 4 | 524221 (4) | 73/73, Sparse | 22375 [21589, 23142] | 1063.9 (rho) | 21.03 [17.54, 26.04] | 1.40 | 20/20/20 |
| (7,3,1,0x1dd060) | 21 | 128 | 4 | 1047199 (2) | 54/54, Sparse | 31714 [28471, 34345] | 1641.5 (bsgs) | 19.32 [16.70, 21.76] | 1.23 | 20/20/20 |
| (6,4,0,0x5716d4) | 24 | 64 | 4 | 2097383 (8) | 31/31, Sparse | 7064 [6693, 7432] | 2280.3 (rho) | 3.10 [2.66, 3.67] | 0.41 | 20/20/20 |
| (6,4,1,0xb1261a) | 24 | 64 | 4 | 2097803 (8) | 31/31, Sparse | 7484 [7167, 7808] | 2264.7 (bsgs) | 3.30 [3.02, 3.62] | 0.41 | 20/20/20 |
| (8,3,0,0x2a521b) | 24 | 256 | 4 | 4195349 (4) | 129/129, Sparse | 63833 [59539, 67530] | 3483.9 (bsgs) | 18.32 [16.84, 19.92] | 0.90 | 20/20/20 |
| (8,3,1,0xc136f2) | 24 | 256 | 4 | 4194973 (4) | 121/121, Sparse | 65568 [57852, 71586] | 3122.2 (bsgs) | 21.00 [18.37, 23.55] | 1.15 | 20/20/20 |
| (7,4,1,0x19e70cc) | 28 | 128 | 4 | 33553901 (8) | 63/63, Sparse | 17129 [16127, 18191] | 8969.6 (bsgs) | 1.91 [1.72, 2.11] | 0.21 | 20/20/20 |

"J-adds / generic E-ops" is the mean number of Jacobian additions (step set-up + walk) over the best generic method's mean E operations,
**a stage diagnostic** (one Jacobian addition priced as one E addition, all other phases free), not a speed.

Arms side by side (IC vs table rho vs BSGS) and cold ratio:

| instance | N | ell | BSGS gae | rho gae (tuned multiplier bits) | IC / rho [95% CI] | IC / BSGS [95% CI] | IC cold / best generic | IC wall ms | rho wall ms | BSGS wall ms |
|--|--|--|--|--|--|--|--|--|--|--|
| ghs-l2-n6-a0-b0x220 | 12 | 149 | 30 | 68.3 (3) | 41.5 [36.18, 47.36] | 94.19 [84.01, 105.38] | 139.87 | 55 | 0.06 | 0.02 |
| ghs-l2-n6-a0-b0x221 | 12 | 67 | 21.5 | 59.3 (3) | 30.72 [29.41, 32.17] | 84.65 [78.11, 92.41] | 148.82 | 16.9 | 0.45 | 0.01 |
| ghs-l2-n6-a0-b0x228 | 12 | 29 | 14.1 | 56.4 (3) | 32.79 [29.64, 35.75] | 131.1 [122.52, 140.86] | 228.84 | 15.9 | 0.05 | 0.01 |
| ghs-l2-n6-a0-b0x229 | 12 | 1049 | 60.9 | 121.3 (3) | 52.58 [41.58, 65.18] | 104.62 [82.11, 130.37] | 127.18 | 186.4 | 0.12 | 0.06 |
| ghs-l2-n6-a0-b0x28 | 12 | 131 | 27.7 | 72.4 (3) | 32.17 [27.31, 38.82] | 83.95 [72.41, 99.23] | 133.53 | 37.5 | 0.07 | 0.02 |
| ghs-l2-n6-a0-b0xe2 | 12 | 83 | 23.1 | 64 (3) | 39.06 [32.12, 48.01] | 108.1 [90.34, 129.32] | 167.24 | 56.8 | 0.06 | 0.02 |
| ghs-l3-n4-a0-b0x16 | 12 | 83 | 23.1 | 66.5 (3) | 7.86 [7.23, 8.58] | 22.62 [20.66, 24.73] | 31.43 | 3.6 | 0.06 | 0.01 |
| ghs-l3-n4-a0-b0x3 | 12 | 37 | 17.5 | 58.6 (3) | 36.46 [31.38, 42.47] | 122.14 [104.07, 142.91] | 156.56 | 9.1 | 0.06 | 0.01 |
| ghs-l3-n4-a0-b0x8 | 12 | 37 | 17.5 | 59.5 (3) | 7.16 [6.25, 7.92] | 24.35 [22.8, 26.15] | 35.6 | 6.5 | 0.66 | 0.01 |
| ghs-l3-n4-a0-b0xa2 | 12 | 7 | 6.4 | 46 (3) | 2.55 [2.47, 2.63] | 18.21 [16.35, 20.55] | 31.64 | 0.2 | 0.03 | 0 |
| ghs-l4-n3-a0-b0x36f | 12 | 1013 | 66.6 | 112.3 (3) | 16.16 [14.3, 18.36] | 27.24 [24.3, 30.58] | 30.35 | 20.9 | 0.7 | 0.06 |
| ghs-l4-n3-a0-b0x74b | 12 | 1039 | 63.2 | 118 (3) | 16.11 [13.97, 18.73] | 30.07 [26.63, 34.29] | 33.27 | 17.7 | 0.51 | 0.06 |
| ghs-l2-n7-a0-b0x4c | 14 | 157 | 30.8 | 72.2 (3) | 34.85 [32.37, 37.68] | 81.62 [75.18, 89.46] | 144.39 | 45.1 | 0.08 | 0.03 |
| ghs-l2-n7-a0-b0x4d | 14 | 79 | 22 | 61.1 (3) | 40.62 [37.8, 43.67] | 112.66 [103.8, 122.42] | 194.05 | 27.1 | 0.06 | 0.02 |
| ghs-l2-n7-a0-b0x58 | 14 | 677 | 58.3 | 105.8 (3) | 29.13 [25.58, 33.23] | 52.9 [47.5, 59.53] | 84.48 | 68.6 | 0.11 | 0.09 |
| ghs-l5-n3-a1-b0x1df9 | 15 | 16451 | 206.9 | 444.4 (3) | 12.26 [9.58, 16.18] | 26.33 [23.26, 29.63] | 27.41 | 50.4 | 0.77 | 0.17 |
| ghs-l5-n3-a1-b0x31a6 | 15 | 16349 | 229.1 | 407.9 (3) | 10.53 [8.79, 12.86] | 18.75 [16.04, 21.58] | 19.72 | 41.1 | 0.36 | 0.6 |
| ghs-l4-n4-a1-b0x2944 | 16 | 8191 | 172.5 | 254.1 (3) | 6.23 [5.28, 7.56] | 9.18 [8.48, 9.97] | 10.48 | 18.2 | 2.15 | 0.23 |
| ghs-l4-n4-a1-b0x3d2c | 16 | 8179 | 149.4 | 200.1 (5) | 8.61 [7.09, 10.39] | 11.53 [10.55, 12.63] | 13.04 | 31.7 | 0.86 | 2.41 |
| ghs-l6-n3-a0-b0x37711 | 18 | 32801 | 304.9 | 340.7 (5) | 21.39 [16.51, 27.72] | 23.9 [20.43, 27.63] | 24.66 | 78.5 | 2.23 | 2.18 |
| ghs-l6-n3-a1-b0x1e702 | 18 | 65563 | 408.7 | 565.7 (6) | 17.06 [14.26, 20.62] | 23.61 [21.12, 26.43] | 24.1 | 98.8 | 4.39 | 1.16 |
| ghs-l5-n4-a0-b0x48eb3 | 20 | 131023 | 553.8 | 761.4 (5) | 4.63 [3.99, 5.38] | 6.36 [5.6, 7.18] | 6.77 | 54.8 | 3.52 | 1.17 |
| ghs-l5-n4-a0-b0x90f2d | 20 | 131129 | 585.9 | 730.4 (4) | 4.63 [3.79, 5.69] | 5.77 [5.16, 6.49] | 6.14 | 65.8 | 2.21 | 2.64 |
| ghs-l7-n3-a0-b0x11ceab | 21 | 524221 | 1195.1 | 1063.9 (5) | 21.03 [17.54, 25.93] | 18.72 [17.44, 20.24] | 21.27 | 183.1 | 5.05 | 6.66 |
| ghs-l7-n3-a1-b0x1dd060 | 21 | 1047199 | 1641.4 | 2037.8 (4) | 15.56 [12.67, 19.53] | 19.32 [16.72, 21.8] | 19.47 | 277 | 7.2 | 8.51 |
| ghs-l6-n4-a0-b0x5716d4 | 24 | 2097383 | 2333.8 | 2280.3 (12) | 3.09 [2.66, 3.67] | 3.02 [2.76, 3.32] | 3.2 | 116.1 | 18.61 | 17.97 |
| ghs-l6-n4-a1-b0xb1261a | 24 | 2097803 | 2264.7 | 2368.4 (6) | 3.15 [2.59, 3.95] | 3.3 [3.02, 3.61] | 3.4 | 103.1 | 12.91 | 12.48 |
| ghs-l8-n3-a0-b0x2a521b | 24 | 4195349 | 3483.8 | 3816.4 (13) | 16.72 [13.94, 20.76] | 18.32 [16.8, 20.03] | 18.4 | 377.8 | 14.61 | 14.08 |
| ghs-l8-n3-a1-b0xc136f2 | 24 | 4194973 | 3122.2 | 3160.1 (13) | 20.74 [16.68, 25.61] | 21 [18.42, 23.58] | 21.09 | 420.5 | 10.65 | 12.21 |
| ghs-l7-n4-a1-b0x19e70cc | 28 | 33553901 | 8969.6 | 10718 (14) | 1.59 [1.24, 2.11] | 1.9 [1.72, 2.11] | 1.93 | 242.4 | 49.18 | 40.53 |

Phase split (mean over the 20 holdout logs, gae) for representative genus-4 instances:

| instance | N | IC total | Phi(Q) on K | step set-up | relation search | linear algebra | verify | best generic | J-adds (mean) | positions tested (mean) | relations (mean) | calibrated J-add / test (gae) |
|--|--|--|--|--|--|--|--|--|--|--|--|--|
| ghs-l4-n3-a0-b0x74b | 12 | 1901 | 197 | 825 | 803 | 64 | 11 | 63 | 179 | 78 | 6.5 | 9.2 / 0.95 |
| ghs-l4-n3-a0-b0x36f | 12 | 1815 | 198 | 830 | 722 | 51 | 12 | 66 | 169 | 70 | 6.7 | 9.4 / 0.96 |
| ghs-l4-n4-a1-b0x3d2c | 16 | 1724 | 220 | 625 | 805 | 56 | 15 | 149 | 269 | 139 | 7.4 | 5.4 / 0.49 |
| ghs-l4-n4-a1-b0x2944 | 16 | 1584 | 220 | 632 | 668 | 45 | 16 | 172 | 245 | 115 | 6.8 | 5.4 / 0.5 |
| ghs-l6-n3-a1-b0x1e702 | 18 | 9651 | 209 | 1563 | 7013 | 843 | 21 | 408 | 698 | 534 | 29.6 | 10.3 / 2.86 |
| ghs-l6-n3-a0-b0x37711 | 18 | 7290 | 206 | 1466 | 4794 | 803 | 19 | 304 | 507 | 353 | 31.6 | 10.1 / 3.27 |
| ghs-l8-n3-a0-b0x2a521b | 24 | 63832 | 208 | 2164 | 39342 | 22086 | 30 | 3483 | 3104 | 2881 | 124.4 | 10.3 / 3.23 |
| ghs-l8-n3-a1-b0xc136f2 | 24 | 65568 | 210 | 2142 | 45087 | 18097 | 29 | 3122 | 3545 | 3325 | 111.9 | 10.3 / 3.22 |
| ghs-l5-n4-a0-b0x90f2d | 20 | 3382 | 219 | 913 | 2033 | 192 | 22 | 585 | 482 | 309 | 14.6 | 5.6 / 0.87 |
| ghs-l5-n4-a0-b0x48eb3 | 20 | 3527 | 219 | 908 | 2201 | 175 | 22 | 553 | 506 | 335 | 14.6 | 5.6 / 0.87 |
| ghs-l6-n4-a0-b0x5716d4 | 24 | 7064 | 219 | 1155 | 4859 | 799 | 29 | 2280 | 863 | 652 | 31.1 | 5.8 / 1.58 |
| ghs-l6-n4-a1-b0xb1261a | 24 | 7483 | 220 | 1157 | 5291 | 785 | 28 | 2264 | 918 | 708 | 30.6 | 5.5 / 1.6 |
| ghs-l7-n4-a1-b0x19e70cc | 28 | 17129 | 221 | 1359 | 11925 | 3587 | 34 | 8969 | 1857 | 1611 | 62.2 | 5.8 / 1.52 |

Findings read off these tables (measured unless marked):

* IC needs 5.4 to 10.4 E additions per Jacobian addition in this library at g = 4 (calibration over 300 positions; 460 to 545 F_q multiplications
  and 8 to 9 inversions per Cantor addition), plus 0.5 to 4.6 per smoothness test. Relations cost about g! = 24 positions each
  (N = 28: 1611 positions for 62 relations), as expected for a split fraction near 1/g! (recalled).
* The relation search is the largest phase from N = 18 on (60 to 84% of IC at N >= 20); step set-up is 36 to 46% at q = 16, linear algebra
  grows to 28 to 35% at q = 256 (n = 3) and 21% at N = 28 (n = 4).
* Phi(Q) over K costs about 200 gae at q >= 16 (1370 to 1940 at q = 4, g = 8) and is not a bottleneck at N >= 16.
* Wall time (contended, secondary; three of this study's runs and other jobs shared 4 cores): IC 0.3 to 420 ms, rho 0.04 to 49 ms, BSGS 0.004 to 41 ms per log.

## 5. Trend, phases and the crossover

Fits are ordinary least squares of log2(mean cost) on l = log2 q over the instances of a family, bootstrap (5000 resamples of the
instances) for the intervals (`raw/fit_genus4.json`). Slopes are exponents of q.

| family | points | IC | best generic | relation search | linear algebra | ratio IC / generic (slope per doubling of q) |
|--|--|--|--|--|--|--|
| n = 3, g = 4, q = 16 ... 256 | 10 | 1.27 [1.17, 1.36] | 1.39 [1.28, 1.49] | 1.43 [1.25, 1.59] | 2.11 [1.93, 2.26] | -0.12 [-0.18, -0.03] |
| n = 4, g = 4, q = 16 ... 128 | 7 | 1.11 [1.03, 1.16] | 1.93 [1.86, 2.00] | 1.35 [1.23, 1.48] | 2.04 [1.90, 2.15] | -0.83 [-0.94, -0.75] |

* **n = 3, g = 4 has no crossover in sight.** The generic cost grows like q^1.4 (theory q^(n/2) = q^1.5), the relation phase like q^1.4
  (Gaudry's q^(2-2/g) = q^1.5 is recalled), so the phase's ratio to the generic method stays at 11 to 18; and the linear algebra (q^2.1)
  already costs 0.8 to 6.3 times the generic method and grows faster. The aggregate fit crosses 1 only at l = 43 [31, 141]: extrapolation
  of a sum of phases with different exponents, not a prediction.
* **n = 4, g = 4: the measured ratio is 11.5, 9.2 (N = 16), 6.4, 5.8 (N = 20), 3.1, 3.3 (N = 24), 1.9 (N = 28): it halves about every step of l.**
  Linear algebra stays at 0.26 to 0.39 of the generic cost across the four sizes; the relation phase falls from 5.4 to 1.3 of it. If the trend
  continues the priced total crosses the generic method at **l = 8.1 [7.8, 8.4], N = 32 [31, 33] (extrapolated)**; with the step set-up phase set to
  zero (derived) at l = 8.3 [7.9, 9.0]. Since the LA phase (q^2.04) and the generic cost (q^1.93) have nearly equal exponents, the fit
  also implies the advantage cannot grow past a constant factor (about 1 / 0.4 = 2.5, derived from the LA share) with this plain linear algebra; a
  large-prime variant would change that (not built).
* **Derived floor: Jacobian additions free.** Removing the calibrated cost of all Jacobian additions from the measured total leaves 1.43 to
  1.70 (N = 16), 1.09 to 1.22 (N = 20), 0.87 and 1.06 (N = 24) and 0.70 (N = 28) of the generic cost for n = 4 (a derived lower bound on any
  implementation that keeps these phases; not a measurement), and 3.2 to 3.8 (N = 12) rising to 9.1 and 9.3 (N = 24) for n = 3.
* **Where the crossover would lie (all extrapolated):** n = 4, g = 4: N about 32 (32 [31, 33] for the priced total; 33 [32, 36] with the step set-up phase free, derived);
  n = 3, g = 4: none supported; g = 7 or 8 over F_4, F_8: no trend measurable (two sizes, ratios 53 to 131; the factor base has 1 to 3 columns).

## 6. The (l,n) = (3,4), g = 8, q = 8 run that "never finished" in R5.2 and R7.1

Measured with `r7_2_ghs_end_to_end profile 3 4 --magic 4` (curve a = 0, beta = 0x3, first with magic number 4 and a prime factor >= 20 of #E;
#E = 4144 = 2^4 * 7 * 37) and by running R7.1's own example unchanged on that curve (`raw/r71_rerun.log`):

| phase of R7.1 `run_curve` | measured or extrapolated | time |
|--|--|--|
| `count_points_extension` k = 1 ... 8 (field F_{8^k}, 3k bits) | measured | 0.00002, 0.00014, 0.0011, 0.015, 0.11, 0.92, 6.4, 67 s (growth 7 to 14 per degree) |
| k = 9 = g + 1 (2^27 x-values; the independent genus check) | extrapolated x8 | about 540 s |
| closure of J0 capped at 400000 elements: 20 generators, 8 x 10^6 validated Cantor additions | measured 20000-element closure at 104 us per addition, scaled | about 835 s (2024 s in a second profile run with more concurrent jobs) |
| three 100000-step walks, filters | scaled | about 40 s |
| **R7.1's example, 1 curve, 20000 steps (so ~20 s less than at 100000)** | **measured** | **1175 s, finished, results written** |

* Cause isolated: the run is exponential in the extension degree (point counting) and linear in 8 x 10^6 validated Cantor additions
  (subgroup closure), and the example prints nothing between "curve 0" and the final JSON. Under the heavier contention of the second profile run the
  same phases scale to about 2700 s, so R7.1's stop after 25 minutes at 100000 steps is within the plausible run time, and its 280 s re-run was
  still inside the point counts. **No infinite loop and no bug in `ghs_smooth.rs`**; no regression test applies. `N_1 ... N_9 = 9, 65, 513, 3961, 32769, 262145, 2097153, 16807673, 134217729`; `#J = 16642304`;
  `N_9` matches the value predicted from `N_1 ... N_8`; Phi is a homomorphism on 20 of 20 pairs; 37 | #J.
* Measured fully-split rate on this curve (R7.1 method, 20000 steps): 3.0e-4 per step, 6 hits; closed-form split fraction of the whole
  Jacobian 1.42e-4 (ratio 2.1; with 6 hits the interval is wide). R7.1's "exact J0" enumeration hits its 400000 cap here (the closure is not
  complete), so its ratio to the walk (0.057) is not meaningful.
* IC on this curve: online 2139 [1862, 2451] gae against BSGS 17.5: ratio 122 [104, 143], 1 factor-base column of the 3 available.

## 7. Verification summary

| check | result |
|--|--|
| Every planted log verified on E (`[d]P == Q`) | 600 of 600 (IC), 600 of 600 (rho), 600 of 600 (BSGS) |
| IC inconsistent-row flag (`mu_1 = 0` or `d = 0` contradiction) | 0 of 600 |
| Tallied and untallied IC runs identical (d, counts) | 600 of 600 (asserted in the run) |
| Reruns with a different binary reproduce IC and BSGS counts | 10 of 10 files identical; one instance re-run with a later binary identical per log |
| Ledger-priced E additions vs `GroupAdd` formula | 1.000 on 400 additions |
| `add_unchecked == add` | 200 of 200 random sums |
| `image_alg == image` | on the N = 12 test instance |
| fast #E count vs library | equal on 4 curves; the example's `pick` uses it for N > 16 |
| Library tests | 96 passed, 0 failed, 2 ignored |
| `rustfmt --edition 2021 --check`, clippy (library and example) | clean for the changed files (one toolchain `unknown lint` warning in the library, not from these files) |

## 8. Limits and what is not claimed

* Toy sizes. The crossover estimate is an extrapolation of a power-law fit over four to five doublings of q (n = 4) from N = 16 to 28. It rests on
  the library's generic Cantor arithmetic (460 to 530 multiplications per addition at g = 4; an explicit genus-4 formula would be cheaper), on a plain dense-rows echelon
  (counted as sparse), on factor-base trial division, no large-prime variation, no symmetry beyond the sign of a point, and 8 walk steps. Only the relation mode, the
  factor-base size and rho's multiplier width were tuned. A better IC moves the crossover down; a stronger generic arm (rho with the negation map,
  about 1.4 times cheaper per the usual estimate (recalled), not built) moves it up.
* The generic reference is table-based (BSGS memory sqrt(ell) points; table rho keeps every visited point). At these sizes memory is not a
  constraint; at cryptographic sizes it would be.
* The curves are selected (a in {0, 1} embedded in k, beta of magic number 3 or taken from R7.1). The instance families are small samples: 2 curves per size.
* `a` outside the base field, odd-genus t-models beyond R7.1's single curve, and N > 31 (the library's `enumerate_irreducibles` stops at degree 31) are not covered. The
  ell-primary part is assumed cyclic (ell || #J); it is checked implicitly (inconsistent-row flag, verification), not by computing #J.
* No ecbench session, replay certificate, scoreboard or leaderboard entry (these instances are not registered curves); `docs/` dashboards are unchanged. No entry
  in `benchmarks/` (the study measures counted cost, not an algorithm's speed). Contended wall times are not evidence. Conductor was not available.
* R7.1's own example was not modified; R7.1's rate table and frozen records stand.

## 9. Requirements

| requirement | status |
|--|--|
| 1 Hypothesis H and falsifier, tested end to end under an explicit counted model | verified complete: refuted at all 30 instances |
| 2 Factor-base relation collection on the real descended Jacobian, LA mod ell, planted log recovered and verified; unit stated, wall time reported | verified complete (sections 1, 2, 4) |
| 3 Factor-base size scan per instance | verified complete (tuning scan in every record; `fig4`) |
| 3 Run all instances: N = 12 (2,6) g = 8; N = 14 (2,7); N = 12 (3,4) g = 2 and 4; try (3,4) g = 8 | verified complete; (3,4) g = 2: curve 0xa2: Phi kills the 37-part, only the order-7 subgroup is attacked (a trivial instance, 7 | #J = 56), so its full ECDLP is not solved; curve 0x21 not applicable |
| 3 Diagnose the (3,4) g = 8 q = 8 hang | verified complete: not a hang, not a bug (section 6); no regression test applies |
| 4 Compare to rho on E with paired seeds, 95% bootstrap CIs, >= 20 planted logs | verified complete (20 holdout logs per instance); ecbench's rho not reused (section 1) |
| 4 State whether any instance is weak and where the crossover would lie | verified complete: none weak; extrapolated N about 32 for n = 4, g = 4, none for n = 3 |
| 5 Report, raw JSON, SVG figures, report.typ and PDF | complete (this file, `raw/`, `fig1`-`fig6`, `report.typ`, `report.pdf`) |
| Open the PR ready for review, report, stop; no CI watching | done by the authoring session after this commit |

## 10. Files and commands

* `raw/run_*.json`: per-instance records (tuning scan, rho tuning, calibration, 20 paired holdout rows with all phase costs).
  `raw/fit_genus4.json`, `raw/selftest.json`, `raw/profile_l3_n4_m4.json` (second run) and `raw/profile_run1_less_contended.json`,
  `raw/r71_rerun_l3n4_g8.json` + `raw/r71_rerun.log`, `raw/pick_*.json`, `raw/logs/`, `raw/v1_dense_only/` (first runs of the dense relation mode only, kept
  because the dense walk's total cost at q = 256 was 2.9 to 3.1 times the tuned sparse mode's, 195529 and 192836 gae against 63832 and 65568: preserved, superseded by the tuned runs).
* `fig1_ic_vs_generic`, `fig2_ratio_vs_N`, `fig3_phase_breakdown`, `fig4_factor_base_scan`, `fig5_hang_profile`, `fig6_extrapolation` (`.svg`, `.png`),
  `figure_data.tsv`, `plot.py` (reads `raw/` only), `report.typ`, `report.pdf`, `commands.sh`, `SHA256SUMS`.
* Reproduce: `commands.sh`.

Graphs checked and left unchanged: R7.1's `fig1`/`fig2` (relation rate, a different quantity); no scoreboard, leaderboard or `benchmarks/` graph measures
this quantity.

## Note text (ready to paste into the consolidating section)

R7.2 (PR for `research/r7-2-ghs-end-to-end`, `research/isogeny_fibers_20261010/round7/r7_2_ghs_end_to_end/RESULTS.md`) ran Gaudry-style index
calculus on the real GHS-descended Jacobian of R7.1 end to end (factor base of F_q-points, a walk producing relations, linear algebra mod ell,
extraction) and compared it with table rho and baby-step giant-step on E, in bitop/v1 gate equivalents expressed in E additions, on 30
instances with N = 12 to 28 (600 planted logs, every answer verified on E, paired seeds, 95% bootstrap intervals). **No instance is weak:** the total online cost
of IC divided by the best generic method is 1.91 [1.72, 2.11] at best (N = 28, q = 128, g = 4, n = 4) and 9.18 [8.49, 9.96] at best for N <= 18; against
rho alone 1.60 [1.25, 2.12] at best. The ratio is flat at 18 to 30 for genus 4 over F_{q^3} (q = 16 to 256) and falls to half per doubling of q for genus 4 over
F_{q^4} (11.5, 6.4, 3.3, 1.9 at q = 16, 32, 64, 128); the fit of that family crosses 1 at N about 32 (31 to 33, extrapolated, one step beyond the
last measurement), a statement about a selected class of toy curves under this implementation's counts, not about any deployed curve.

Where the cost goes: the relation search is the largest phase (60 to 84% at N >= 20) at about g! = 24 positions per relation; linear algebra grows as q^2.0
against q^1.9 for the generic cost in the n = 4 family (0.26 to 0.39 of it), so with plain linear algebra the advantage tops out at a constant factor,
and for n = 3 (q^2.1 against q^1.4) the crossing is not supported. Counting only Jacobian additions (one per E addition, everything else free)
falsely suggests IC wins at N >= 20 (0.21 at N = 28, against 1.91 priced); the library's Cantor addition costs 5.4 to 10.4 E additions at g = 4 and the other phases cost as much again. The
(l,n) = (3,4), g = 8, q = 8 run of R5.2 and R7.1 was not hung: R7.1's example finishes it in 1175 s, spent on point counting to N_9 over F_{2^27}
and a 400000-element closure with validated Cantor additions; there is no bug in `ghs_smooth.rs`.

Limits: toy sizes; generic Cantor arithmetic (an explicit genus-4 formula would be cheaper); no large-prime variation; table-based generic arms without the
negation map; ecbench not used, so no replay certificate and no scoreboard entry; cover construction unpriced; curves are a selected class (beta of magic number 3).
