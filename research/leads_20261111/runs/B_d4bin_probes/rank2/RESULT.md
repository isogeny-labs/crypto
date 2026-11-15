# Rank-2 elimination variant of the char-2 D4 (k = 4) Frobenius-form solve

Date 2026-11-12. Task `d4bin-rank2-elimination`. Lead: the follow-up the skeptic of
LEAD-ICB-d4bin-constant-quadratic-elimination proposed (`../VERIFY.md` caveat 1, `../RESULT.md`
"Next step" 1). Preregistration: `PREREG.md`, frozen 20:46 UTC, before any code for this variant.
Every result statement is **[pcc:unverified]**. No pcc claim has been filed.
Local only, no RunPod, no AWS. Base commit `2003c4bc85a6db91e74bfe19522a517b44995ff4` (HEAD of the
shared checkout when the run was made) plus one new untracked file, listed below.

## Verdict (short)

| Arm (root finding) | Current solve, muls/trial | Rank-2 elimination, muls/trial | Paired ratio elim/current [95% bootstrap] | Cheaper on | Prereg central [80% interval] | Verdict |
|---|---|---|---|---|---|---|
| frob_roots (SqMod) | 191,046 | 131,645 | **0.6891 [0.6885, 0.6896]** | 300/300 | 1.15 [0.8, 1.7] | **success under the frozen rule** [pcc:unverified] |
| baseline roots | 223,575 | 225,123 | 1.0069 [1.0061, 1.0077] | 47/300 | 1.5 [1.0, 2.2] | no gain (slightly worse) [pcc:unverified] |

m = 12, curve `find_bin_curve(12, 1, 16)`, the same 300 targets as `d4_char2_profile --m 12 --seed 1
--trials 300`. The rational solution sets are **identical on 300 of 300** targets (324 solutions in
total, none with s4 = 0). The elimination path raised no structural error on any target.

The frozen rule said: success means identical solutions on 300/300 and a frob-arm paired 95%
interval wholly below 1.0. Both conditions hold. The prediction was wrong in the favourable
direction: the ratio of 0.689 lies below the 80% interval I froze. I had put P(ratio < 1) at 0.25.
The cost unit is counted F_q multiplications (inversions count 1). No wall time was measured, and
this is not a benchmark row.

## What the variant does (`examples/d4bin_rank2_elim_probe.rs`, new, opt-in)

1. It takes the same `system_frobenius` components, so the Weil cost is the same.
2. It row-reduces the (S2^2, S3^2) coefficients, which have rank 2 on every target. That gives
   Q1, Q2 (with the squares) and L1, L2, which are linear in (S2, S3):
   `L_i = S2 A_i + S3 B_i + C_i` with A, B, C in F_q[s4, P].
3. Cramer's rule: `D = A1B2 + A2B1`, `N2 = B1C2 + B2C1`, `N3 = A1C2 + A2C1`. Substituting gives
   `F_j = D^2 Q_j(N2/D, N3/D, s4, P)`.
4. It computes `R(s4) = Res_P(F1, F2)`: evaluation at deg-bound + 1 = 121 points, a formal-degree
   Euclidean resultant at each point, then Newton interpolation. The factor s4^k is stripped.
5. It finds the rational roots of R with the library `roots_with`, in either the baseline or the
   SqMod arm.
6. Back-substitution: for each root it computes `gcd(F1(s4,.), F2(s4,.))`, takes the rational P
   roots, then gets S2, S3 by Cramer's rule. Where D = 0 it takes the rank-1 line intersected with
   Q1, Q2. The line s4 = 0, on which D and N3 vanish identically, is solved directly.
7. Every candidate is checked on all four original components.

The structural assumptions (rank 2, only the (S2,S3) patterns 00/10/01/20/02, constant square
coefficients) are checked on every target. Any failure returns `Err` and is counted (0 at m = 12 and
at m = 8).

Measured shape [pcc:unverified]: deg (s4, P, total) of F1 = (10, 12, 12), of F2 = (10, 10, 10) after
an s4^2 factor is stripped. deg R = 120 on 300/300 targets. The current solve has a 64-dimensional
quotient, so about 56 degrees of R are extraneous (base points where D = N2 = N3 = 0). R has 0 to 8
rational nonzero roots per target (median 2).

## Component table (m = 12, 300 targets, mean F_q muls per trial) [pcc:unverified]

| Current (`solve_bin`, as `solve_d4_bin` calls it) | baseline | frob_roots | | Rank-2 elimination | baseline | frob_roots |
|---|---|---|---|---|---|---|
| weil | 1,431 | 1,431 | | weil | 1,431 | 1,431 |
| triang | 128 | 128 | | reduce | 131 | 131 |
| echelon | 54,809 | 54,809 | | build (D, N2, N3, F_j) | 1,428 | 1,428 |
| nf | 21,913 | 21,913 | | resultant (121 evaluations) | 40,161 | 40,161 |
| charpoly | 87,728 | 87,728 | | interpolation | 14,520 | 14,520 |
| roots | 55,505 | 22,977 | | roots of R (deg 120) | 165,979 | 72,640 |
| eigvec | 1,837 | 1,837 | | back-substitution | 872 | 871 |
| verify | 224 | 224 | | s4 = 0 line | 364 | 225 |
| | | | | verify | 238 | 238 |
| **total** | **223,575** | **191,046** | | **total** | **225,123** | **131,645** |

The split phase is excluded from both arms. It is identical when the solution sets are identical:
621 (baseline) / 389 (frob) in the profile.

**Control (the current arm reproduces the committed lever record).** The current-arm components
equal `control_profile_m12.txt` and `research/ic_d4_char2_lever_20261023/REPORT.md` line 52 exactly:
224,196 - 621 = 223,575 and 191,435 - 389 = 191,046. Every other component is identical too.

## Correctness checks [pcc:unverified]

- m = 12, 300 targets: the solution sets are identical on 300/300 (`rank2_m12.txt`).
- m = 8, 300 targets (`rank2_m8.txt`): identical on 297/300. On the other 3 targets (trials 20, 131,
  153) the elimination path returns one **extra** solution, and each extra solution has s4 = 0. An
  exhaustive search over all (S2, S3, s4, P) in F_256^4 (`brute` in the probe, uncounted, independent
  of both solvers) confirms that each extra solution is genuine. So the **current solver misses
  rational solutions with s4 = 0** on those 3 targets (`brute_m8_list.txt`). They cannot produce
  relations: s4 = w1w2w3w4 = 0 forces some t_i in {0, 1}, which the split rejects. On the s4 != 0
  solutions both paths agree on 300/300. Exhaustive check on the first 20 m = 8 targets: brute ==
  current == elim on 20/20 (`brute_m8_first20.txt`).
- m = 8 ratios: frob 0.5599 [0.5593, 0.5606], baseline 0.5800 [0.5794, 0.5806], cheaper on 300/300
  in both arms. Root finding on R costs fewer squarings at m = 8, so the m = 12 baseline arm loses.

## Default path unchanged

No library source was edited for this task. `git status` shows only the new untracked file
`examples/d4bin_rank2_elim_probe.rs`, and `src/cryptanalysis/gaudry_quartic_bin.rs` has no diff
against HEAD. The current arm reproduces the committed profile numbers bit for bit (above).
`cargo test --release --lib gaudry_quartic_bin`: see "Tests" below.

## Tests

`CARGO_INCREMENTAL=0 cargo test --release --lib gaudry_quartic_bin`: **8 passed, 0 failed** (11 min 17 s including the build; `tests_gaudry_quartic_bin.txt`). The library was not modified, so these tests check the unchanged default.

## Commands (single-threaded, release)

```sh
CARGO_INCREMENTAL=0 cargo build --release --example d4bin_rank2_elim_probe
RAYON_NUM_THREADS=1 target/release/examples/d4bin_rank2_elim_probe 12 300            # rank2_m12.txt (2 m 36 s)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_rank2_elim_probe 8 300             # rank2_m8.txt  (0.5 s)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_rank2_elim_probe 8 300 20          # brute_m8_first20.txt (~5 min)
RAYON_NUM_THREADS=1 target/release/examples/d4bin_rank2_elim_probe 8 300 list:20,131,153,5,6,7  # brute_m8_list.txt
CARGO_INCREMENTAL=0 cargo test --release --lib gaudry_quartic_bin
```

Probe binary sha256 `7e5823934ee76c975a05243bfed33cd129857ca2b88ece1f232d5cf92ebc3fa7`, source sha256
`fe990609f0b9acdf959fdff98eb050a58e401b4e2335abe71cbb4d980233e963` (`binary.sha256`). The
`brute_m8_list.txt` run used an earlier build of the same file; the only difference is that it
lacked the "agree on s4 != 0" counter line. Formatting was done with `rustfmt --edition 2021` on that
file only.

## Caveats (the reader should weigh these)

1. **Counted unit only.** Inversions count 1 and Newton divisions count 1 (the node-difference
   inverses are fixed constants), as the library's `newton_interpolate` charges them. No wall-time
   comparison was made, so the win is in the counted metric only.
2. The win holds only in the SqMod (frob_roots) arm. With generic squaring the degree-120 root
   finding (166k) eats the whole saving. The root finding on R is 55% of the variant's cost.
3. One curve at m = 12 (seed 1) and one at m = 8. The structure is the symbolic support, which was
   the same on 300 instances in `../coset_m*.txt`, so I expect the shape to carry over. That has not
   been run.
4. The variant needs q > 121 evaluation points (m >= 7) and the structural assumptions above. A
   solver integration would need a fallback to the current path on `Err`.
5. This is a probe example. It has not been wired into `solve_d4_bin` or the relation collector, and
   no relation-rate or end-to-end DLP measurement was made. No leaderboard row, no bench.record and
   no VALIDATION.json exist (out of scope here).
6. Side finding: the current Macaulay/eigen path loses s4 = 0 solutions on 3 of 300 m = 8 targets.
   This has no relation consequence, but it should not be cited as "identical solutions"
   unqualified at m = 8.

## What is blocked

Nothing for this probe. The steps the repository rules need before a claim (a pcc claim, a
bench.record, an Iso Lab run, visuals, a validation gate) were out of the task's scope and were not
done.

## Next steps (not run)

1. Remove the extraneous factor of R. Its degree is 120, but at most 64 roots are genuine; the
   extra roots come from base points D = N2 = N3 = 0. For example, compute the base-point factor
   from Res_P(D, N2) and gcd it out, or work with the F_q[s4]-module of the pencil. Root finding
   scales about quadratically in degree, so a degree of about 64 would cut roots from about 73k to
   about 21k (frob). That would put the projected ratio near 0.42 [pcc:unverified projection].
2. A subresultant computed once over F_q[s4], or fewer evaluation points from the true degree 120
   (the bound was 120 as well, so no gain there), to reduce the 40k resultant phase.
3. Integrate the variant as an opt-in `BinOptions` field in `solve_d4_bin`, with a fallback on
   `Err` and relation-set parity on the profile targets. Then run a recorded paired measurement
   (wall time and counted muls) through the repository's benchmark route.
4. File a pcc claim for the counted 0.689 ratio once it has been independently re-run.
