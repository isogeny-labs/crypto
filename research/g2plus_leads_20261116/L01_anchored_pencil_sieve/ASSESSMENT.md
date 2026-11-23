# L01 anchored pencil sieve: assessment (assessor, 2026-11-16)

Status: `[pcc:unverified]`. The data has one producer, a same-code replay and this
assessor's re-derivation; nothing has been validated. Not a benchmark: no leaderboard
row and no speed claim.

## Verdict

**H1 is SUPPORTED, ROBUST under the frozen rule (PROTOCOL section 8).** The margin
against the strongest baseline is about 3x to 5x, not the 30x to 70x of the headline R1.

| g=2 cell | R1 GM | R1 MIN | R2 GM | R3 GM |
|---|---|---|---|---|
| p=1031 | 68.56 | 29.42 | 12.22 | 20.37 |
| p=4099 | 31.65 | 29.91 | 5.35 | 9.36 |
| p=16411 | 38.73 | 32.80 | 6.02 | 11.40 |
| p=65537 | 33.57 | 33.18 | 5.25 | 9.86 |

How the rule applies:
- All gates G1 to G7 pass, and the positive control passes (PC min 69.5, against a threshold of 5).
- GM(p) is at least 3.0 and MIN(p) at least 2.0 at all four g=2 primes, so H1 is supported.
- R2 and R3 GM are at least 3.0 at all four primes, which makes it "robust".

Secondary results:
- **H2 (g=3, R1) is supported.** GM is 42.1, 46.2, 49.8 and 51.2; the minimum over pairs is 39.9.
- **H3 (R2) is supported.** GM is between 5.25 and 12.22.
- **H5 (2LP, FPM per cycle) is supported as specified.** The ratios are 7.12 and 8.64 at g=2, and 27.9 and 40.0 at g=3. The spread over the 3 curves is 6.90 to 9.54 at g=2. The caveat below (descent not charged) applies.
- **H4 (no threshold) goes against the pencil.** The counted LA multiplications are 42 to 58x higher for P rows at g=2 p=1031, 329 to 465x at g=2 p=4099, and 16 to 19x at g=3 p=1031. The net is positive only in the 3 cells at g=2 p=1031.

Spread over seeds (max/min of FPM per relation over 6 runs):
- P_full: 1.009 to 1.122.
- B_lab at g=2: 7.47 at p=1031, 1.28 at 4099, 1.50 at 16411 and 1.03 at 65537. The cause is the duplicates described under the fairness section.

## What I checked

1. **Frozen files.** `sha256sum -c frozen.sha256` passes for PROTOCOL.md and SPEC.md, and `raw/data.sha256` passes for all 125 files.
2. **Headline re-derived from the raw files with my own script.** The script is `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l01a/rederive.py`. I recomputed FPM from `mul`, `inv`, `cantor_*` and `rel_*`, not from the stored `fpm_per_rel`. R1, R2, R3 and PC match `raw/summary.md` to every printed digit in all 8 cells. Example: at g=2 p=65537, R1 GM is 33.571 and MIN is 33.182.
3. **Counter audit.**
   - `sieve_tuple` charges 1 counted multiplication per point (lambda), one batch inversion per tuple, interpolation, and the forward-difference initialisation.
   - The per-tuple count-formula gate is enforced in code on every tuple. The record per tuple is mul = 10 + 8 + 3(nX-1) + npts and inv = 2; for example 163783 = 18 + 3·32753 + 65506.
   - B_lab is a transcription of the lab walk: generic counted Cantor, `disc_character`, Cantor–Zassenhaus `roots_of`, and inversions counted as 1 inversion each, which favours the baseline.
   - The measured B_lab cost at g=2 is about 90 mul + 4 inv per walk step (about 131 FPM), plus about 100 FPM per trial in the relation tests.
4. **Independent mathematics.** My own sieve shares no code with `l01/`. I used random curves, random anchors, Lagrange interpolation and per-point lambdas; the script is `/tmp/claude-0/-home-user/6fd00ecc-ad17-50e3-84b9-4e7db4d013e1/scratchpad/l01a/indep.py`.
   - The split fraction against p/(g+1)! is 1.026 (g=2, p=4099), 0.932 (g=3, p=4099) and 0.997 (g=2, p=16411).
   - 480 polynomial identity checks f - a² = ∏(x - x_k) passed with 0 failures.
   - A brute-force root count of the residual at random lambdas agreed with the buckets, with 0 mismatches.
   - The rate and the relation identity therefore hold independently of the producer's code. Its G7 values are 0.99 to 1.06.
5. **Gates.**
   - V1 (exact polynomial identity) was run on every pencil relation, and `v1_walk` on every B relation. V2 (a Cantor sum) was run on all relations at p ≤ 4099 and on about 200 sampled relations above that.
   - G3 recovers the planted t at all four tiny instances, with 0 bad rows and 0 L mismatches.
   - The replay ran the same code on the same host and reproduced every count and row digest. It is not an independent implementation.
6. **Deviations.** None of the 10 deviations changes a threshold, an arm or the rule. Deviation 1 (a repeated root at a bucket abscissa is rejected rather than raised as G5) follows PROTOCOL section 1(iii) and affects only the H5 arm (15 rejects). Deviation 2 is covered by the polynomial check on all B relations.

## Fairness concerns (they qualify the result but do not change the verdict)

- **The headline R1 of 30x to 70x mostly measures how weak the baseline instrument is.** That baseline uses generic Cantor at about 131 FPM per addition and Cantor–Zassenhaus roots at about 100 FPM per trial. R2, against explicit formulas plus table roots, is the honest reading: about 5.2x to 6.0x for p ≥ 4099.
- **The setup is excluded from both arms, but only P_full needs it.** The setup is the y-per-x table (5.5p multiplications). P_full reads it on every tuple; B_lab at full base does not. Charging it to P_full gives these GM values:
  - R1: 19.8 to 43.7.
  - R2: 7.80, 3.34, 3.89 and 3.38. The minimum over pairs at p=4099 is 3.004.
  - R2 with setup and the pencil's additions at 0.1 FPM each (the §5a middle band): 2.91 to 3.37. With additions at 1 FPM: 1.33 to 1.53.

  The baseline's own additions are not charged in that last reading, so it is pessimistic for the pencil. The pencil stays ahead of B_strong in every reading I computed, but the margin is about 3x, not 30x.
- **B_lab duplicates at small p.** The 16-adding walk cycles before its 4096-step restart: up to 4130 duplicates per run at g=2, and 7237 in total over the 6 runs at p=1031. This inflates R1 at p=1031 (MAX 225.8). At p=65537 duplicates are about 1% to 3%, so the decisive prime is clean.
- **Relation usefulness.** Pencil rows have weight 2g+1 and their g anchors are dense columns, so the counted elimination costs far more (H4). The comparison also favours B: in counting mode B's rows carry no target or label column, which real walk rows do. H1 is a claim about the relation phase only and says nothing about end-to-end cost.
- **About 30% overshoot of N_target.** The relations are counted in the denominator. Per tuple the yield and cost are homogeneous, so FPM per relation is unaffected, but those relations are not needed.
- **Descent is not measured.** For the full base it is O(g!) trials, which is negligible, as the protocol says. For the 2LP base S (nb ≈ 2p^{1-1/g}) it is not negligible. My rough estimate, not measured: about 2(p/nb)²/… ≈ 2000 or more Cantor trials per descent at g=2 p=16411. That is on the order of P_2lp's whole relation phase or more. **The H5 ratio of about 8x at g=2 therefore overstates the net gain**, because homogeneous rows need descents of G and T and inhomogeneous B walk rows do not.
- **Prior art.** I did not query research-kb, because it is not installed here. The protocol's novelty argument rests on a grep (fixed anchors and genus 3, against the walk-anchored Vitse–Wallet sieve).

## Distance to matched rho

No rho was run in this lead, so all of the following is an estimate. The rho figures are the lab's unverified numbers from `genus_ladder_scale_20261115/RESULTS.md`.

- **g=2.** The lab's 2LP arm sits 3x to 5x above rho, and 80% to 92% of its cost is the relation walk.
  - Replacing the walk by P_2lp at its measured 7x to 9x relation-phase saving would bring the arm to about 0.6x to 1.5x of rho, before descent.
  - Descent of G and T into S is estimated to add back a cost comparable to the walk's.
  - A realistic distance remaining is therefore about 1x to 3x. This is unmeasured, and the gap is not shown closed.
- **g=3.** The lab's 2LP arm is already about 0.54x to 0.74x of rho, with three quarters of its cost in the LA.
  - The pencil cuts the relation phase by 28x to 40x (H5), which is at most about a quarter of the total.
  - P_2lp rows are heavier (at least 5 S-entries against at least 1 for B, plus dense anchors), so the LA could grow and cancel the gain.
  - The net effect on the rho ratio is unknown, and could be anywhere from about 0.15x to worse than now.

## Single most valuable next step

Run one frozen end-to-end 2LP measurement at g=2 and g=3 (p = 16411 and 32771), for P_2lp and for the lab's 2LP arm, each against a matched rho in the same counted unit across the §5a band. Each arm must be charged with:
- the descent of G and T into S, with up to 2 large primes;
- the y-table setup;
- the counted cycle-matrix elimination with SGE/Markowitz on the actual P_2lp cycle rows.

That measurement decides whether the 7x to 40x relation-phase saving survives descent and linear algebra, which is the only thing that bears on the gap to rho.
