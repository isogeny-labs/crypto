# LEADS2 report (ic_mega_pass_20261031, leads L1-L6)

All numbers below are the audited values (AUDIT.md in each lead directory). No security claims are made or supported.
Note: the relayed user request was "try npow"; no tool, file or reference named npow was found, so the computed lead tasks were executed.

## Bottom line
1. **The n=31 anomaly survived the independent closure (L1).** A separately written degree-4 closure (macclose.c) reproduces N1's gf2close exactly: C1 s0/s1 (2821, 37293, 0), D1 s0 (2821, 37278, 0), A s2 reaches 1. The stall is real for this W_4 definition (shared generator); its mechanism, and the 1488 = 31*48 degree-4 rank deficit, are unexplained.
2. Key contrast: C1 s1 and A s2 are both UNSAT; A reaches 1 in W_4, C1 stalls. The 2821 = n(3N-5) formula holds for C1 only (D1 fails at n=29).
3. L2 (d=5 at n=31, n=33 control): INCOMPLETE. No d=5 result yet. At n=33, early UNSAT draws repeat the pattern (A reaches 1; C1/D1 stall at low4=3201).
4. L3: N1/N2/N4 audit items closed as stated. The N3 p=13 "point-set artifact" diagnosis is withdrawn: f is irreducible (C1b falsified), the cause is undiagnosed, and the excess also occurs at p=19.
5. L4: msolve D and closure d_F agree and reproduce d_F = 3,4,4,4,4,5,5,5 for t=1..8. C-step is undecided (t=12..14 unreachable). Run still incomplete.
6. L5: negative. Non-adaptive S4 membership needs S*D = Omega(n_F^3); the 2-torsion quotients are a relabelling/trade. L6: negative. Lever 6 fails (A2 FAIL, a timeout-based verdict); N1 has 0/892 non-trivial lifts.
7. Every lead is audited "partially_valid": the code and data hold, but the README wording overstates in places (corrections listed per lead).

---

## L1 - n=31 independent closure
- **Question:** is N1's n=31 stall (C1/D1 do not reach 1 in W_4) an artefact of gf2close?
- **What ran:** new engine `src/macclose.c`, validated on 297 instances (0 mismatches against gf2close; n=5,7,9 also against a pure-Python reference). n=31 C1 s0/s1, D1 s0, A s2 and n=29 C1 s0, D1 s0 used N1's generator. Auditor re-ran 189 validation cells (0 mismatches) and the D1 n=29/n=31 round logs (identical).
- **Audited numbers** (`experiments/L1-n31-independent-closure/RESULTS.json`):
  - n=31: C1 s0, s1 = (low 2821, dim 37293, one 0); D1 s0 = (2821, 37278, 0); A s2 = (5489, 41449, 1).
  - C1 holds 2821 = 31*91 after round 2 and gets +0 in round 3. Its degree-4 rank stops at 34472 of C(32,4)=35960 (deficit 1488 = 31*48). D1 has 2760 after round 2, gets +61 in round 3, and its final degree-4 rank is 34457. A gets +124 falls in round 3 with degree-4 rank 34224 < 35960, then reaches 1.
  - Round 1 counts: 92/930 at n=31 and 86/812 at n=29. The 3n count of degree-<=2 elements (93, 87) holds after round 2.
  - n=29: C1 low 2465 = 29*85 after round 2 (prediction held), final (4524, 31929), SAT. D1 2432 after round 2 (prediction failed), then reaches 1.
- **Audited outcome:** **the anomaly survives.** It is not a gf2close fault. The "count threshold" reading is a description, not an explanation: A gets falls below full rank, and there is about 2.5x oversampling. "No n=2k-1 effect" rests on one instance each at n=29 and n=31, and n=27 was not run. P2 literally mis-predicted D1 dim (37293 vs 37278). Caveat: the shared generator means the result holds only for this definition of W_4. Validation goes only up to n=21.
- **Next step:** identify A's 124 round-3 falls (leading monomials or the a/b blocks). Explain the 1488 = 31*48 deficit. Run more seeds and n=27. Run degree 5 on a stalled C1 instance (that is L2).

## L2 - d=5 on n=31 and n=33 control
- **Question:** does a stalled n=31 UNSAT instance reach 1 at degree 5, and does arm A still refute (and C1/D1 still stall) at n=33?
- **What ran:** `src/gf2close5.c` (N1 engine with triangular pivot storage and a CPU cap). Validation: 81/81 d=5 cells against the Python reference (n<=13), 101/101 d=4 regression cells against N1 raw. Auditor: 11/11 cross-checks against N1's engine and a 2^34 enumeration of n=33 s=0 solution counts. Jobs: `src/run_a.py` (n=31 C1 s1, D1 s25 at d=5), `src/run_b.py` (n=33 d=4).
- **Audited numbers:** no RESULTS.json yet. Raw files are `experiments/L2-d5-and-n33-control/raw/{validate_d5,regress_d4,n33_k17_d4,d5_n31}.jsonl`. d5_n31 is empty; C1 s1 d=5 was at 733 of 2400 CPU-s (queue 12000 of 15841) at audit. d=5 N=32 uses 242825 columns and about 3.7 GB RAM. n=33: C1/D1 UNSAT draws s=0,1,2 give (dim4 45507, low4 3201, one 0). A s1 (UNSAT) reaches 1 in W_4.
- **Audited outcome:** provisional and incomplete. D1 s0 at n=31 is SAT, so the substitution of D1 s25 is correct. A at n=33 replicates the treatise (Thm lfdexact: all 29 instances, not "most"). The 8-draw thresholds are weak (95% bound about 31%). A capped d=5 record must be reported as inconclusive.
- **Next step:** let the jobs finish. Generate RESULTS/README with a saved script, with per-arm UNSAT counts and Clopper-Pearson bounds. Report a capped d=5 run as inconclusive. Re-audit.

## L3 - open audit items N1-N4
- **Question:** close the open N1-N4 audit items.
- **What ran:** `diag_n3_p13.py`, `diag_n3_p13_fp2.gp` (N3), N1 crosschecks (festest, n=19 brute force, Python closure, n=27-31 recount), and README/RESULTS edits in N1-N4. Auditor: irreducibility tests and a multi-curve scan in `experiments/L3-audit-items/audit_scripts/`.
- **Audited numbers** (`experiments/L3-audit-items/RESULTS.json`): N3 kernel dims 149/1, 181/3, 277/1 (p=11/13/17). On V(F_169) only f survives. N1 brute force 36/36. n=31 recount: A 9/9, C1/C2/D1 0/8 (low_4 2821). n=29 D1 7/8. N1 CPU 3795 s; N3 CPU 4192 s. The auditor found f irreducible over F_13, F_169 and F_13^3, and found kernel excess on 2 p=19 curves (3 of 17 cells with npts >= 125).
- **Audited outcome:** N1, N2 and N4 are closed with the partials listed. N3: C1a is confirmed. C1b is falsified (the "no factoring tool" blocker was false), and the cause is undiagnosed and not isolated to one curve. The pre-registered C2 prediction failed in one cell (n=29 D1 7/8). The n=31 stall recount is from raw, not independent; L1 has now supplied the independent check.
- **Next step:** diagnose the N3 excess across p=13/19 curves (save the curve-generation script). Run the N2 rank-59 lattice test (needs pre-registration). Rerun the N2 8-seed raw p/A/c/xR.

## L4 - PKM plateau (C-step prediction)
- **Question:** does m=2 Kummer d_F follow the registered C-step (blind cells t=12..14)?
- **What ran:** `gen.py`, exact closure `closure.c`, msolve via `run_msolve.py`, in two nohup chains. Auditor: rebuilt closure (24/24 cells match), dense numpy reimplementation, S3 encoding test 1000/1000, full-ring planted runs for t=2..5.
- **Audited numbers:** no RESULTS.json yet. Raw files and the auditor's scripts are under `experiments/L4-pkm-plateau/raw/` and `audit/`. d_F (closure) for t=1..8: 3,4,4,4,4,5,5,5. msolve D is identical at t=1..8 (t=9 has one seed, D=5). ffd=4 for t>=2. t=10 at c=6 needs 60460 columns.
- **Audited outcome:** valid reproduction only. C-step is undecided. The planted control is conclusive only for t=2..5 (t=6,7: "no unit up to c=6"). The 4 targets per t are the same 4 systems at every t. The "56-112 GiB" figure is unsourced. If msolve gives D<=5 at t=10, that is a reproduction discrepancy against the registered 6.
- **Next step:** wait for chainA/chainB to finish. Score the null and P2 arms and t=10. Fix the README. Run t=12..14 only on a large-memory host.

## L5 - S4 subquadratic membership
- **Question:** can any structure decide S_4 membership below n_F^2?
- **What ran:** `memb.c` (RAND/TCL/E2CL bases, p up to 2^28, n_F 32-512), `arm_c_pari.py` (gcd oracle, n_F 8-64), `analyze.py`. Auditor: PARI recomputation of X3 (0 mismatches), mutation tests, saturation check.
- **Audited numbers** (`experiments/L5-s4-subquadratic-membership/RESULTS.json`, `RESULTS_grid.json`): X3 ratio TCL 0.251-0.267, E2CL 0.063-0.068. Online cost per relation TCL 4.1x, E2CL 16.6-17.1x. S*D/|X3| = 3.00-3.26 (D=2n by construction). Arm C per-target CPU 0.00066 s to 0.0435 s at n_F 8 to 64.
- **Audited outcome:** negative, supported at toy scale. In the non-adaptive equality-lookup model S*D = Omega(n_F^3), and the counting is near-tautological. The pair table already gives O(n) online per target. P2 is a partial pass (12 of 16 cells; the 4 saturated cells are explained). The arm C exponent is "about n^2 over n<=64", with a naive product. At equal yield the quotients give table x0.63 and online x1.59 with S*D unchanged: a trade. The 3SUM-indexing citation is unverified.
- **Next step:** close Lever 2 in the non-adaptive model, or measure adaptive (Hellman/Fiat-Naor) S-T curves.

## L6 - Frobenius-stable non-subspace bases
- **Question:** do cyclotomic or Hamming-ball bases beat the random null or give lower solving degree (Lever 6, KN-OPEN-5e3dd0)? Is N1 alive?
- **What ran:** `src/l6.c` plus Python drivers on the Koblitz curve, n=11 and n=23. Solvers: cryptominisat5 and msolve (120 s timeout). Auditor: independent PARI yields, rerun of N1, and the CRT check (2000/2000).
- **Audited numbers** (`experiments/L6-frobenius-stable-nonsubspace-bases/RESULTS.json`): N3' ratios to null 0.94-1.13. N2 ratio CYC2 n=23 2.42 (about 3 independent units). N2 per column at n=23: CYC2 16.2, HAM3 41.8. msolve on CYC2 timed out (also at 300 s, auditor). N1: L1=0, L2=2 (both forced), L3=0, so 0/892 non-trivial lifts. Enumeration takes about 3 ms per target.
- **Audited outcome:** negative stands. A2 FAIL rests on degree timeouts (encoding-dependent), and the yield prong is mixed. P4 literally failed (2 L2 hits, both trivial). The RESULTS.json ground-truth fields should be null where ground truth was not computed. Classification: relabelling/engineering, novelty unverified.
- **Next step:** optionally test the Lucas/QFC quadratic encodings (KN-LIT-316c94). Otherwise close Lever 6 and N1 in this scope.

---
Process notes: conductor check was not run, per BRIEF §0.6 (T-5736); it was attempted unsuccessfully in L1/L3. Thread limits were briefly exceeded in L1, L2 and audits (disclosed). L2 and L4 jobs may still be running under nohup.
