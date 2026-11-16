# LEADS3 report (ic_mega_pass_20261031, leads L7-L9)

All numbers are audited values (AUDIT.md in each lead directory). Toy scale, one generator (N1 `lib.system`), one closure definition (round-based W_d). No security claims.

## Bottom line
1. **L7 (valid):** the n=31 stall is a degree-4 effect. Stalled UNSAT C1 s=1 has 1 in W_5, not in W_4 (CPU 2818 s, full run with no cap). D1 s=25 had already reached W_5 in L2. L2's earlier "stall" was a cap hit about 2% short in queue progress.
2. **L8 (valid, 3 README fixes):** at n=33, arm A (Semaev on a random subspace) reaches 1 in W_4 on 8/8 UNSAT draws. C1, C2 and D1 stall on 0/8 (low4 3201 = n(3N-5), D1 also 3212 and 3235). B and RM stall at 1155. The n=27 replay with macclose matches N1 exactly.
3. **L9 (partially valid):** A's extra round-3 falls number exactly 4n (124, 132, 140 at n=31, 33, 35). They are trace components of 4 K-valued bit-degree-3 relations that need at least 4 Frobenius twists and shared K-structure in the bilinear and affine parts. Random or unstructured stand-ins (C1, A_Lrand, A_randaff) get 0. The mechanism is measured, not derived.
4. **What is known about the stall:** C1, C2 and D1 lack the K-semilinear (Frobenius) structure. Their degree-4 rank saturates at the n(3N-5) plateau from n=31 on, so they need degree 5. The Semaev arm escapes at degree 4 through the 4n Frobenius-window falls.
5. **Bearing on true Semaev last-fall-degree growth:** none shown. The stall belongs to the non-Semaev stand-ins. On this generator the Semaev arm keeps d_F <= 4 through n=33. The treatise's certified d_F >= 5 onset (T33a/b, a subset of ker Tr) was not reproduced, because arm A is a different instance family.
6. **Open:** the 1488 = 31*48 degree-4 deficit, the symbolic form of the 4 relations, the 82 non-window falls at n=29, d=5 at n=33, and the A onset at n >= 35.

---

## L7 - d=5 on stalled n=31 instances
- **Question:** does a stalled n=31 UNSAT instance reach 1 at degree 5?
- **What ran:** L2's `gf2close5` unchanged (sha256 recorded), re-validated 18/18 against the Python reference (n=9,11). Single-thread jobs: n=31 C1 s=1 d=5 (CPUCAP 12600) and n=29 C1 s=0 d=5 as the SAT calibration.
- **Audited numbers** (`experiments/L7-d5-stalled/RESULTS.json`, `raw/*.jsonl.gz`):
  - n=31 C1 s1 d5: dim = ncols = 242825, low 41449, one=1, nprod 693216 (= 32*21663, early exit), CPU 2818 s, wall 5011 s.
  - n=29 C1 s0 d5 (SAT, nsol=2): dim 174435 = ncols - 2, low 31929, one=0, CPU 2677 s.
  - Imported from L2: D1 s25 d5 one=1. L2's capped C1 s1 run stopped at qh 21215 of 37979.
  - Auditor: 30 extra fresh-seed cells (n=9,11,13), 0 mismatches. d=4 regeneration gives (37293, 2821, 0) on both engines.
- **Audited outcome:** valid. 1 is in W_5 and not in W_4 for this closure, so "d_F=5" means exactly that and is not the treatise's fall degree. P1 held. The P2 CPU ratio failed (0.95 against the predicted 0.3-0.6) and is reported as failed. Weaknesses: the PROTOCOL header time is false (the file order is correct), no certificate for 1 was saved, the 3.7 GB RAM figure is inherited, and the d=5 engine is cross-checked independently only up to n=19/21. L7 claimed Conductor T-5752 although BRIEF §0.6 says not to.
- **Next step:** run d=5 on more n=31/n=33 stalled C1 and D1 UNSAT draws, and explain the 1488 deficit.

## L8 - n=33 control
- **Question:** at n=33, does A still reach 1 in W_4 while the stand-ins stall, and does the independent engine confirm n=27?
- **What ran:** `src/run33.py` (N1 gf2close, d=4, 4 threads, seeds 0..89, closures only on UNSAT draws). `src/run27.py` (L1 macclose, n=27, first 8 UNSAT seeds per arm for A, C1 and D1). PROTOCOL.md was written before the runs.
- **Audited numbers** (`experiments/L8-n33-control/RESULTS.json`):
  - n=33 (k=17, N=34), 8 UNSAT draws per arm: A 8/8 (low4 6580, dim4 52956); C1 0/8 and C2 0/8 (low4 3201 = 33*97); D1 0/8 (3201 x6, 3212, 3235); B 0/8 and RM 0/8 (1155 = n(N+1)). The Clopper-Pearson 95% upper bound on the opposite outcome is 0.312 per arm.
  - Against L2's gf2close5: 24/24 match. The auditor ran macclose at n=33 on C1 s0 (45507, 3201, 0) and A s1 (52956, 6580, 1), both matching. Exhaustive 2^34 counts were checked on 4 draws.
  - n=27 macclose: A, C1 and D1 each 8/8 reach 1 (low4 3683), 0 mismatches against N1.
- **Audited outcome:** valid, with these README corrections. Wall time was about 1.5 h elapsed, not 2.3 h. The R1 and R2 jobs overlapped, so the 4-thread cap was probably exceeded (peak up to about 6). D1 stalled on 1 of 8 at n=29 (low4 2536), while C1 does not stall at n <= 29. Predictions for A, C1 and D1 were already known from L2 and N1, so they are not independent. Eight equal low4 values are eight outcomes, not eight confirmations of a mechanism. The data rule out both n=2k-1 and k parity as the separator. The stand-in stall begins at n=31 (D1 is not sharp at n=29).
- **Next step:** run d=5 on one stalled n=33 C1 (about 300k columns, about 5 GB). Run the n=35 A arm to look for the onset. Explain why n(3N-5) saturates from n=31 on.

## L9 - structure of A's 124 round-3 falls
- **Question:** what are the 124 round-3 falls that let A (but not C1) reach 1 at n=31?
- **What ran:** `src/macdump.c` (macclose plus a round limit and dumps of the low pivot rows). Closures stopped at round 3 for n=27..35, plus 7 structural variants at n=31 s2. K-window analysis (`src/krel.py`, `kker.py`, `newrel.py`, `chk.py`). A mid-run non-linearity bug in the reduction was fixed, and all reported numbers come from the corrected code.
- **Audited numbers** (`experiments/L9-124-falls-structure/RESULTS.json`, `analysis/*.json`, `AUDIT.md`):
  - A round-3 falls explained by a 4-twist window: n=31 124 (seeds s0, s2, s3, s6), n=33 132, n=35 140 (= 4n, one seed each). n=29: 116 of 198 (82 unclassified). n=27: 431 of 1233 (swamped by count-forced falls).
  - Any 3-twist window explains 0. Gapped windows {0,1,2,4} and {0,1,3,4} explain all 124.
  - Variants: A_V1V2 and A_lin keep 124. A_Lrand and A_randaff give 0, with a rank profile identical to C1. C1 gives 0 at n=31 and n=33.
  - Against C1 at n=31 and 33: A's degree-4 rank is lower by 8n and its total rank by 4n.
  - Auditor: 36 small-cell dump checks, an independent reimplementation of the window analysis (exact match), and the 4 relations verified in W3, outside W2, spanning 124 dimensions.
- **Audited outcome:** partially valid; the core holds. Corrections: "consecutive twists" becomes "at least 4 distinct twists". "None pure a^3/b^3" depends on the pivot order (only "degree 3" is invariant). P3 "held" is generous at n=27 and n=29, and n=33 and n=35 have one seed each. The field-equation interplay was not tested. The dumps are 101 MB in /tmp, not 10 MB, and `chk.py` hard-codes that path. Classification: toy-scale relabelling/accounting. Novelty is unverified.
- **Next step:** derive the 4 relations symbolically in a generic Frobenius model (a_i^2 = a_{i+1}) to prove the 4n count. Test whether the same mechanism drives A's rounds 4-5 and the 82 non-window falls at n=29.

---

## The n=31 stall and true-Semaev fall-degree growth
- **Now known:**
  - The stall is confined to stand-in arms with no coherent K-structure (C1, C2, D1, and the nulls B and RM at a lower plateau).
  - It sets in at n=31, persists at n=33, and is resolved at degree 5 (C1 s1, D1 s25).
  - Arm A avoids it through 4n Frobenius-window falls in round 3.
- **Still unexplained:** why the plateau saturates from n=31 on (the 1488 = 31*48 deficit).
- **Bearing on the true Semaev system:**
  - None is established. The stall is a property of the stand-ins, not of the Semaev arm, which stays at d <= 4 through n=33 on this generator.
  - The data neither confirm nor refute last-fall-degree growth for the treatise's instances (random subsets of ker Tr). Settling that needs a run on that instance family, or A at n >= 35.
  - All of this holds for one generator and one closure definition, at toy scale. No security claims.
