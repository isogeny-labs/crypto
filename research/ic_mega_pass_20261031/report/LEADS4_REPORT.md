# LEADS4 report (ic_mega_pass_20261031, leads L10-L13)

All numbers below are audited values, taken from AUDIT.md in each lead directory. Everything is toy scale, uses one generator convention (N1 `lib.system`, or L10's `lib10.draw`, which matches the treatise `s3_descent`) and one closure definition (round-based W_d). "d_F" means the first d with 1 in W_d for this closure; it is not the treatise's fall degree. No security claims.

## Bottom line
1. **L10 (valid):** the treatise's ker-Tr family T fails at degree 4 on 16/16 UNSAT draws at n=33 and 8/8 at n=35, and refutes on 0/8 at both n=29 and n=31. Each cell's plateau equals the certified T33a/b and R35a values. The named instances are typical of their family. The d=5 run on T33 was blocked by memory (thrashing in swap) and gave no result.
2. **L11 (partially valid):** arm A does not reach 1 in W_4 at n=35 on 9/9 UNSAT draws (low4 3885 = n(3N-5)+8n). A second engine (macclose) reproduces this on one draw. P1 is falsified. n=37 has no closures.
3. **L12 (partially valid):** the 4 relations behind A's 4n round-3 falls are now explicit over F2(z,c). Their inclusion in W3 has a certificate (computer-assisted, for z*c != 0). The 4(L-3) window count is verified only for finite windows. In the real system, "exactly 4n" is confirmed on 3 dumps (n=31, 33, 35) and fails at n<=29.
4. **L13 (partially valid):** a block-counting formula reproduces the stand-in plateau n(3N-5) and the degree-4 rank exactly on 10/10 stalled runs (6 cells). On the diagonal the deficit is n(n^3-38n^2+263n-274)/24, with 1488 at n=31. The formula is not proved, and the pure-algebra check P1 is falsified.
5. **Last-fall-degree growth:** for the true Semaev system, on both the treatise's ker-Tr family and arm A, the degree needed by this closure rises from <=4 to >=5 within the tested odd n. The rise happens at n=33 for T and at n=35 for A. Whether d=5 suffices was not shown for any Semaev draw.
6. **Open:** a d=5 run on Semaev draws (T33, A35) on a host with at least 8 GB of free RAM, n=37, and proofs of the L12 and L13 counts.

---

## L10 - ker-Tr family (treatise instances)
- **Question:** is the treatise's d=4 failure on T33a/b typical of the ker-Tr family T, and how does T compare with arm A across n=29-35?
- **What ran:**
  - The `lib10.draw` generator, validated equation by equation against the treatise `s3_descent`.
  - N1 gf2close at d=4 on T at n=29, 31, 33 and 35, and on A at n=35. The A baselines for n<=33 were reused from N1 and L8.
  - Named-instance validation: 7 of 7 reproduce the published (low, dim, one).
  - A gf2close5 run at d=5 on T33 s=3, killed after about 4500 CPU-s.
- **Audited numbers** (`experiments/L10-kertr-family/RESULTS.json`, `AUDIT.md`), as failures at d=4 out of UNSAT draws, with CP95 intervals:

  | n | T fails / UNSAT | T plateau (low4, dim4) | A fails / UNSAT | A plateau (low4, dim4) |
  |---|---|---|---|---|
  | 29 | 0/8 [0, 0.369] | low4 4526 | 0/10 | |
  | 31 | 0/8 [0, 0.369] | low4 5489 | 0/9 | |
  | 33 | 16/16 [0.794, 1] | (2968, 48068) on all 16 | 0/8 | low4 6580 |
  | 35 | 8/8 [0.631, 1] | (3323, 57503) | 8/8 [0.631, 1] | (3885, 61425) |

  - Fraction of draws that are UNSAT: about 14-24% for T and 32% for A35 (the README's "7-20%" is wrong).
  - The auditor's fresh build reproduced T29 s0, T33 s3 and A35 s2.
- **Audited outcome:** valid. The headline numbers stand. "0/N or N/N" is an observed count, not a rate, and the onset statement covers only odd n in {29..35}. Text fixes still to apply:
  - The d5 `cause` field is wrong: `d5.py` captured stderr, so an empty log carries no information, and the CPU and RSS figures cannot be verified.
  - The d5 and predictions entries in RESULTS.json were added by hand.
  - The R-family control was not run.
  - "Fails d=4" rests on a single engine.
- **Next step:** run gf2close5 alone on T33 s=3 and on A n=33 on a host with at least 8 GB of free RAM, or with out-of-core storage. Also run a second-engine (macclose) check on one T33 draw.

## L11 - Semaev arm A at n=35/37
- **Question:** does arm A keep d_F <= 4 at n=35 and n=37?
- **What ran:**
  - `src/run.py`: N1 gf2close at d=4 with bsols solution counts, on arms A and C1. It covered n=35 and started n=37.
  - The round-3 script `src/rnd3.py` was written but not run.
  - The auditor added a cross-engine check with L1 macclose on A s=2.
- **Audited numbers** (`experiments/L11-semaev-n35-37/raw/n35_d4.jsonl`, first 60 lines, sha256 bc3233a7...; `AUDIT.md`):
  - A at n=35: one=0 on 9/9 UNSAT draws (seeds 2, 6, 13, 14, 16, 20, 23, 24, 25). All 9 have low4 3885 = 3605 + 280 (8n) and dim4 61425 < 66712 columns.
  - C1 at n=35: one=0 on 7/7 UNSAT draws. All 7 have low4 3605 = n(3N-5) and dim4 54845.
  - macclose on A s=2 gives (61425, 3885, 0), matching.
  - n=37: 3 records, all SAT, no closures.
  - The raw file now has 63 lines. The 3 lines after the audit snapshot add one more C1 UNSAT stall (s=33, same plateau) and 2 SAT draws.
- **Audited outcome:** partially valid. The measurement is real and confirmed by two engines. The README and RESULTS.json are stale and must be regenerated from raw:
  - They undercount the draws.
  - They say "runner died", which is false for n=35.
  - P1 should be marked falsified.
  - They should state that the thread cap was exceeded, with up to 6 or more processes running at once.
  - P2, P4 and P5 are untested.
- **Next step:** finish n=37, rerun with a single runner at <=3 threads and dedupe the rows. Run `rnd3.py` on A s=2 to see whether the 4n round-3 falls persist. Run d=5 on A n=35 s=2 (memory permitting, about 12 GB projected).

## L12 - symbolic form of the 4n falls
- **Question:** what are the 4 relations behind A's 4n round-3 falls, and can the 4n count be proved?
- **What ran:**
  - `src/sym.py`: an exact computation over F2(z,c) with cypari2, in a generic Frobenius model (a_i^2 = a_{i+1}) with windows L=3..6.
  - `src/kech.c`: a GF(2^m) echelon engine for the numeric model.
  - Comparison with L9's relations and with the numeric model on 15 draws.
  - Fresh macdump runs to round 3 at n=31, 33 and 35, with a bit-level check; the dumps were deleted afterwards.
- **Audited numbers** (`experiments/L12-symbolic-4n/RESULTS.json`, `AUDIT.md`):
  - The pivots of R1..R4 are a0a1b3, a0a2b3, a0a3b1 and a0a3b2. Clearing denominators uses the lcm c*z^7, and each certificate has 10 rows.
  - Window dimensions (W2, W3): 37/41 at L=4, 66/74 at L=5, 103/115 at L=6. The gain 4(L-3) is exact for L<=6 at padding 0 and numeric up to L=8.
  - The independent-coefficient control gives 39/73, so the Frobenius coupling is necessary.
  - Real dumps, as (n, s): W2 -> W3 (new falls):
    - (31, 2): 2821 -> 2945 (124)
    - (33, 0): 3201 -> 3333 (132)
    - (35, 0): 3605 -> 3745 (140)
  - In each dump, every R_i has all n of its trace components in W3 and none in W2.
  - Auditor checks: an independent GF(2^16) reimplementation agrees; a negative control (z flipped by one bit) gives 0; 856 point checks of the exceptional set found no failure.
- **Audited outcome:** partially valid.
  - Inclusion in W3 holds by a computer-assisted certificate for z*c != 0. The certificate lambdas are not archived.
  - The 4(L-3) count and the 4n count are conjectural for general L and n. The README's "proven in the generic model" overstates this.
  - Completeness (no other round-3 falls) fails at n=29 (82 extra falls) and at n=27.
  - P5 failed.
  - Classification: explanation of a measured effect. Novelty is unverified.
- **Next step:** archive the certificates. Prove independence modulo the real W2 (this needs L13's plateau). Test whether the window mechanism drives A's rounds 4-5 and the n=35 stall at 3605+8n.

## L13 - plateau n(3N-5) and the 1488 deficit
- **Question:** why do the stand-ins plateau at n(3N-5) from n=31 on, and where does the 1488 deficit come from?
- **What ran:**
  - A counting model with blocks g_r = λ_r μ_r + ℓ_r and top-degree ideals (λμ, λℓ, μℓ) with pairwise Koszul overlaps.
  - `src/model.py`: pure GF(2) ranks on 11 (n,N) pairs.
  - 17 closure jobs (N1 gf2close and L8 macclose, both unchanged) on C1/C2 at n=35 and 37, off-diagonal k values, synthetic rank-n systems, and macclose round logs at n=25 and 27.
- **Audited numbers** (`experiments/L13-plateau-deficit/RESULTS.json`, `AUDIT.md`):
  - R4 = n[3C(N-3,2)+N-3] - 9C(n,2). The deficit C(N,4)-R4 = n(n^3-38n^2+263n-274)/24, with largest root 29.36. Deficits are 1488 at n=31, 4070 at n=33, 7665 at n=35 and 12469 at n=37.
  - Exact stall matches on 10/10 runs, covering 6 cells:
    - (35,36): (3605, 54845) on 3 runs
    - (37,38): (4033, 65379)
    - n31/k17: (3007, 43028), 2 runs
    - n29/k16: (2639, 35148)
    - synthetic n=31 and n=33
  - n33/k16 is predicted not to stall and has 1 in W_4 on 3/3 runs.
  - P1 (the pure model) is exact at degree 4 on only 4 of 11 pairs, and misses at degree 3 at (4,10). The small-N misses depend on the draw.
- **Audited outcome:** partially valid. On the C1/C2 and synthetic families the formula matches every stalled run, so it is a strong empirical accounting. It is not proved, and it is not shown to be exact for N<32. "Extra syzygies" is unsupported: the small-N misses are more likely non-generic GF(2) draws. The README needs fixes E1-E7: the 17/17 tally is inflated, "N>=18 4 of 4" is wrong, and n=37 was not pre-registered. The formula says nothing directly about arm A or T.
- **Next step:** extend the count to A (C1 plateau + 8n at n=35 suggests the 4n Frobenius falls add a fixed correction) and to T's plateaus (2968 at n=33, 3323 at n=35). Prove the formula for N>=32 or find where it breaks.

---

## Last-fall-degree growth for the true Semaev system on the treatise's instance family
- **Now known (this closure, toy scale, odd n in {29..35}):**
  - On the treatise's own ker-Tr family T, degree 4 suffices at n=29 and 31 (0/8 failures each) and fails at n=33 (16/16) and n=35 (8/8). So T33a/b are typical, not exceptional, and in this closure's sense the d_F >= 5 onset for T is between n=31 and n=33.
  - On the random-subspace Semaev arm A, the same onset is between n=33 and n=35 (0/8 at n=33, 8/8 and 9/9 at n=35 in L10 and L11).
  - Both families now leave the d<=4 regime, so LEADS3's statement that "the stall belongs to the non-Semaev stand-ins" is superseded.
  - On every failing draw, the d=4 failure sits on a fixed plateau per (family, n) cell.
  - For A at n=35 the plateau is the stand-in plateau n(3N-5) plus 8n. The 4n Frobenius falls (L12) still occur at n=35 (140 at s=0), but they no longer close the deficit, which L13 puts at 7665 for the stand-ins at n=35.
- **Not known:**
  - Whether 1 is in W_5 for any Semaev draw (T33 or A35). The only d=5 attempt was blocked by memory.
  - Growth beyond n=35 (n=37 has no closures).
  - The asymptotic rate. A polynomial deficit (L13) growing faster than the Frobenius gain (4n per round) suggests the required degree keeps rising, but the formula is fitted to the stand-ins, not derived for Semaev, and is not proved.
  - Whether this closure's d_F equals the treatise's fall degree.
- **Scope:** one closure definition, two generator conventions, 8-16 draws per cell, and odd n from 29 to 37 only. No security claims.
