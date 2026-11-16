# Next-4 run (REPORT.md §6 steps 1-4): audited report

Date: 2026-11-01. Scope: repair items 1-4 of each first audit (the user asked to "do 1-4"); later audit items are still open everywhere. No security claims.

## Bottom line (6 lines)
1. N1 (GF(2) d=4 profile): replication only. Through n=25, the degree-4 closure refutes all UNSAT A/C1/C2/D1 draws (34/34, 34/34, 34/34, 35/35; CP95 upper bound on the failure rate about 0.08 per arm) and none of the B/RM draws. The run was still going (n=27 partial), so the result is provisional, and the range stops below the known d_F>=5 onset at n=33-35.
2. N2 (D4-symmetrised S5 lattice): engineering, no lever. The l=1 closure is 0.00568 and still rising, with no ceiling established. The Y-box comparison is retracted. The 2^6144 vanishing passes a non-root false-positive probe (0 false vanishings; rank 3). The threshold bracket is log2 X in (3.58, 7). This is not root recovery.
3. N3 (capacity/LP converse): the m=4 ceiling 1/32 (5.4x below the needed 0.169) holds only conditionally on premise P. P is unproved, assumed at m=4, and fails undiagnosed at m=3, p=13, seed 0. The (p,f)^l part re-derives W10/MP-lattices-1.
4. N4 (char-2 x-filter): no growth seen and inconclusive. Same-mask z for the 42 non-Koblitz cells runs from -2.62 to +2.58, with none above 3.5. E is mostly the marginal floor (median E_marg/E_real 0.982). The oracle margins are only 1.05-1.07x, and Koblitz cells have an unmatched null.
5. All four re-audits: partially_valid. The numbers in scope reproduce. Open items: N1 items 5-6 plus the final n>=27 re-analysis; N2 items 5-8; N3 items 5-8; N4 item 5 plus wording.
6. No lever is demonstrated. The two lever-relevant open questions are d_F growth at n>=33 (N1) and the untested rank-59 non-full-rank structure (N2).

## N1: GF(2) degree-4 profile (lever 1)
- **Question:** Does d_F stay at 4 for the descended S3 arms (A/C/D1) as n grows, against no-structure nulls (B, RM)?
- **What was run:** A compiled closure at n=19..31. The run was not finished when audited: n=19, 21, 23 and 25 are complete, n=27 is partial and excluded, and n=29/31 had not started. The analysis script is analysis/analyze.py, which reads raw/*.jsonl. README.md and RESULTS.json were written for the first time in this pass.
- **Audited numbers** (`experiments/N1-gf2-d4-profile/RESULTS.json`):
  - 1 in W_4 on UNSAT draws: A 34/34, C1 34/34, C2 34/34, D1 35/35.
  - Exact one-sided CP95 upper bound on the failure rate: 0.084 per arm (0.082 for D1), and 0.26-0.31 per (arm, n) with U = 8-10.
  - B: 0/32 (bound 0.089). RM: 0/34 (bound 0.084).
  - B/RM low_4 = n(N+1) = 399, 483, 575 and 675.
  - Static d_NS > 4 everywhere. Zero P1-P4 violations. r_s == nsol everywhere.
  - Child CPU 296 s.
  - The independent recompute reproduced every number. The C closure matched the Python closure at n=19.
- **Outcome (audited):** partially_valid. This is a replication consistent with the treatise (falldegree.tex Thm lfdexact/lfd5) and says nothing about d_F growth.
- **Caveats:**
  - The run is unfinished, so the last n is not final.
  - The x4 cost rule is ambiguous: it does not fire under the next-n reading (0.29 CPU-h) but does under the all-remaining-n reading (4.35 CPU-h).
  - P3 is a counting-identity null calibration, and P1/P4 were informed by the pilot.
  - A/C/D1 are not independent replicates, and A's nsol is always even.
  - The reasons for omitting n=33/35 and the R35a/T33a positive control are asserted, not measured.
  - Items 5 (validation artifacts) and 6 (CPU accounting in the README) are open.
  - The conductor check was blocked by the sandbox.
- **Next step:**
  1. When run_main ends, rerun analyze.py and record the final n and both readings of the cost rule.
  2. Re-audit n>=27.
  3. Do items 5-6.
  4. Time one n=33 closure, or write "not attempted; cost unmeasured".

## N2: D4-symmetrised S5 lattice (lever 3)
- **Question:** Does the D4-invariant l=1 lattice give a valid reach or dimension saving, validated at finite p?
- **What was run:** Repair items 1-4, plus a non-root false-positive probe that was pre-registered as PROTOCOL Amendment C (fp_probe.py, about 12 CPU-min). It ran 3 LLL rebuilds at 2^6144 and evaluated them at all small D4-invariant non-root points.
- **Audited numbers** (`experiments/N2-d4-s5-lattice/RESULTS.json`, `raw/fp_probe_L6144_*.json`):
  - 0 false vanishings over 139, 139 and 2001 points.
  - h1-h3 have rank 3 over Q and gcd 1.
  - The rebuilt norms equal the stored ones (sha256 verified 3/3).
  - Bracket: log2 X in (3.58, 7), i.e. delta in (5.8e-4, 1.1e-3). Point extrapolations of 5.0-6.6 are not adopted.
  - The l=1 closure is 0.00568 at bbox 14 and still rising.
- **Outcome (audited):** partially_valid. Engineering only, no lever. The test checks the Howgrave-Graham vanishing criterion, not root recovery.
- **Caveats:**
  - The Y-box comparison was retracted rather than computed, and the matched G(Y) closure was not run.
  - The predicted 8x dimension cut came out at 62.5x.
  - Residual text in the RESULTS closure_crosscheck note and the README "105 vs 6561" phrasing still implies a comparison.
  - "Independent" should read "linearly independent, pairwise coprime".
  - The null cannot vanish by construction.
  - Items 5-8 are open: the rank-59 lever, relabelling the arm-X falsifier, the rule 4/5 statements, raw p/A/c/xR for the 8-seed runs, this run's validation values, 37 not 35 bits, and the ~150 CPU-min overrun.
- **Next step:**
  1. Do items 5-8 and the residual wording fixes.
  2. Test the rank-59 non-full-rank structure as a lever.
  3. Optionally, compute the matched G(Y) closure.

## N3: capacity / LP converse (lever 3)
- **Question:** Does the 1/(m d) ceiling extend from (p,f)^l to all auxiliary polynomials vanishing on U?
- **What was run:**
  - Repair items 1-4.
  - Corrected the excess-cell list.
  - Re-ran A0 verbatim with armA0_v2.gp (parisizemax) and kept the old output as SUPERSEDED.
  - Cited prior art and stated the conditionality on P.
  - Claimed Conductor T-5749.
- **Audited numbers** (`experiments/N3-capacity-lp-converse/RESULTS.json`, `raw/armA0.out`):
  - Ceiling 1/32 = 0.03125 at m=4, against 0.169 needed (5.4x short).
  - 8 box8 excess cells at l=3, p>=29. The added cell is seed 1, p=31, npts 30, excess +2 (gain 85 vs ksum 83).
  - A0: 9/125/6561 monomials, top degrees 4/12/32, each a single monomial with coefficient 1. The re-audit re-ran gp and it matches line for line.
  - m2_threshold_rule_transfers_to_m3 = false.
- **Outcome (audited):** partially_valid. The ceiling is supported conditionally on P. The only new content is the extension to all polynomials vanishing on U, and that extension is exactly P.
- **Caveats:**
  - P is certified only at m=2 (l<=3, small p) and in the trivial m=3, l=1 case.
  - The m=3, p=13, seed-0 failure is undiagnosed.
  - Small-p certificates are not measurements at p ~ 2^256.
  - Items 5-8 are open: B2 l=8 was not run, and the SA started at the optimum, so the 0.0 gap holds by construction.
  - The CPU figures disagree (4192 s summed vs about 60 min claimed), and n3lib.py still uses 3 threads.
  - The e-variable 1/32 claim is unproven.
  - Provenance items are open.
  - The headline still begins "SUPPORTS".
  - T-5749 is still open.
- **Next step:**
  1. Do items 5-8 and close T-5749.
  2. Diagnose the m=3, p=13, seed-0 failure of P.
  3. Find a proof route for P at m=4, or treat 1/32 as conditional in all decisions.

## N4: char-2 x-filter spectral test (lever 8)
- **Question:** Do degree-d x-functional filters on binary curves show gain growth above a symmetric null?
- **What was run:**
  - PROTOCOL Amendment 4 retired the vacuous E* rule and pre-registered a same-mask z-test: the REAL best tuple against 10 NULL-1sym permutations (seeds 9000-9009), with z > 3.5 counting as a flag and Koblitz cells excluded.
  - src/repair_samemask.py ran on all 19 Arm B <P> files, 3 processes at a time, and src/repair_analyze.py did the analysis.
  - The conductor check ran clear.
- **Audited numbers** (`experiments/N4-char2-xfilter/RESULTS.json` `repair` block, `raw/repair_samemask_*.json`):
  - Non-Koblitz z runs from -2.62 to +2.58. The top two cells are both h=4, j=2.
  - Koblitz z runs from -2.74 to +0.74, with 9 of 11 negative.
  - Median E_marg/E_real is 0.982 (range 0.61-0.999).
  - The Arm A non-Koblitz REAL/null-max ratio is 0.957-1.000.
  - K3b margins are 1.07x (n=13) and 1.05x (n=17).
  - The re-audit recomputed 53/53 z values with 0 mismatches.
- **Outcome (audited):** partially_valid, inconclusive. No growth was seen. The test has power for j=1 and for the visited j>=2 tuples only.
- **Caveats:**
  - With K=10, z > 3.5 is roughly a t9 per-cell p of about 0.003, not Bonferroni.
  - The two z of about +2.5 are borderline.
  - Koblitz cells have no Frobenius-orbit-preserving null.
  - The RESULTS summary ratio ranges still include Koblitz cells.
  - README lines 57/67 overreach, by calling the result "consistent with" Weil and by saying a bound would make it "a theorem".
  - Item 5 is open: the README number fixes 0.010871 vs 0.010886, 0.103-0.187, the 0.0715/0.1364 values, and the theta_eff drift.
- **Next step:**
  1. Fix item 5 and the wording.
  2. Build a Frobenius-orbit-preserving null.
  3. Run an exhaustive 2-dim subspace search at n=13 in C, for power at j>=2.

## Addendum (main session, after the workflow)
N1 finished through n=31 after its audit. At n=31 the rank-n surrogate arms C1, C2, D1 stop being refuted at degree 4
(0/8 each, all stalled at low_4=2821) while the true Semaev arm A is refuted 9/9. Variable-permutation invariance
passed; msolve did not finish; no independent N=32 closure exists yet. See experiments/N1-gf2-d4-profile/README.md
"Addendum". This is the one result of the pass that bears on lever 1, and it is unverified.
