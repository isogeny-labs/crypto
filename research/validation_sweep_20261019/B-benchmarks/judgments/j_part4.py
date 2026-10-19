from j_part3 import *
row(65,
 cl("Cyclotomic gamma-weight bases are yield-neutral on Koblitz n=23: all effects -0.49%..+0.65%, min p=0.363; positive control p=0.001 on all cells", False, "falsifier: A2 yield clause survives", "null result", "L0 (4 vCPU shared container)", "yes: 1,999 random Frobenius-stable null bases; planted lambda-progression positive control", C,
    "Positive control is p=0.001 on 12 of 12 non-degenerate cells but p=1.0 on the 3 w=1 K=1 cells (identical base), so 'all cells' is wrong by wording only. Null result is well controlled."),
 cl("Cost (S, lower bounds): cyclotomic/compact 1.022 [1.021, 1.023] (K1,30 cols), 1.122 (K0,17 cols); IC arms 2.7-30x rho-frob", "partial", "falsifier", "null/negative", "L0, measurement quality 0/100 in ecbench compare", "A/A bonus unavailable (spec permits wall runs below L2)", C,
    "Counted S lower bounds; ecbench itself reports 'A/A bonus unavailable' and quality 0/100 for wall; counted ratios are fine. 480/480 verified, 320/320 replays identical. Rho reference: rho-frob, the matched strong reference."))
row(66,
 cl("Gated fall-row augmentation: I/G 1.83x [1.74, 1.94] n=9, 4.42x n=11, 11.9x n=13 on overdetermined barrel systems; all 9 pipeline cells MATCH (no firing)", "partial", "ADVANCES (constant), production no-op", "engineering (stage diagnostic); no effect on the production pipeline", "thread CPU, 4-core shared, load 5-7", "yes: A/A covers 1 at all 14 cells", C,
    "Recomputed: 7 of 9 pipeline cells MATCHES, 2 ADVANCES ((17,6) 1.0155, (23,8) 1.0443) where the gate does not fire; README footnote explains these as measurement artefacts (same code), so 'all 9 MATCH' is the correct reading. The headline gains are on barrel systems that the production pipeline never produces (opt-in switch), hence 'production no-op'."))
row(67,
 cl("Free relations in production compact-orbit IC: tuned vs tuned counted 0.960x [0.946, 0.976] pooled; wall 0.941x [0.922, 0.961]; K retune alone at n=41 0.46x", "partial", "ADVANCES (constant)", "engineering", "counts + 1-thread wall on a shared host (load 3-10)", "yes: opt-out A/A 24/24 byte-identical", C,
    "6/6 point ratios recompute (CIs partial). 4% gain, tuned arm vs tuned arm (K re-tuned per arm on training seeds: fair). The 0.46x from retuning K for the incumbent alone is larger than the free-relation gain, showing the 'incumbent' K was mis-tuned. Wall 0.941x at shared load is not validated; counted 0.960x stands."))
row(68,
 cl("First-dependency stop (d4-ed): whole 0.979x [0.973, 0.986] over 27 held-out curves, never worse on any curve", "partial", "ADVANCES (constant)", "engineering (accounting-adjacent: stops sooner)", "counts, 4 vCPU load 8-13", "yes: null A/A arm exactly 1 on 27/27", C,
    "7/7 point values recompute. A 2% constant from stopping at the exact first dependency; the study notes this is the exact bound. Ecbench ECBS1h08cd110d706f 0.990 [0.977, 0.998]. Item-01 2LP re-measured 1.026x REGRESSES is reported."))
row(69,
 cl("PKM prime-isogeny tower base with aux search charged: isogeny/plain 1.002..1.331 (search share 0.8-25% of S); IC 17-137x rho", "partial", "accounting", "accounting (negative: charging the search removes the advantage)", "L0 counts only", "yes: rho A/A 1.000 on every curve", C,
    "Counts, lower bounds (prime natives unpriced). 224-build sweep and the 2^74 P-192 extrapolation have no raw records (cannot_define x2). Rho = rho.negation: for a prime curve with negation only, the strongest in-repo generic."))
row(70,
 cl("G2 Frobenius-equivariant Macaulay pass as oracle: I/C 5.1x [4.85, 5.34] (n=11), 15.5x (n=13) but 0.25x/0.17x at n=5/7; not applicable to production systems", "partial", "TRADES (constant)", "engineering (stage diagnostic on the barrel system, not on the pipeline)", "thread CPU, 4-core shared", "yes: I' A/A; at n=17 and 23 the step A/A CI excludes 1", C,
    "6/10 recompute. 'One linear consequence short, 37/37' recomputes 36/37. Gains hold only on a system class that the production pipeline does not generate."))
row(71,
 cl("Free relations on compact-orbit IC: complete solve 0.957x [0.944, 0.970] counted, wall 0.948x [0.908, 0.989]; rank stage 0.36x/0.94x/0.94x", "partial", "ADVANCES (constant)", "engineering", "counts + wall (3 other items shared the host)", "yes: A/A exact on rank attempts; n=61 A/A rank stage 1.098 [1.002, 1.256]", C,
    "6/6 point values recompute. n=61 A/A rank stage reads 1.098 (CI excludes 1), the size of the rank-stage effect at n=53/61 (0.94x), so those two stage ratios are at noise; the counted 0.957x stands. Trace-zero filter vacuous (0/29,624)."))
row(72,
 cl("D4 + double large primes with tuned alpha: whole 1.025x [1.016, 1.037] REGRESSES; LA 0.827x advances", "partial", "REGRESSES (relation phase)", "negative result", "counts", "yes: null A/A (s = all columns) identical", V, "Honest regression reported; point values recompute (3), replay audit file absent (no_raw x1)."))
row(73,
 cl("Genus-3 IC with double large primes: q^1.385 vs incumbent q^1.910 over 2^10-2^18; ratio 0.620 at 2^10 -> 0.036 at 2^18", "partial", "ADVANCES (exponent and constant)", "advance over the in-repo full-factor-base incumbent; against rho the gap is not closed (local exponent 1.46 at 2^20 vs 4/3)", "counts, 4-core shared, 3 GB cap", "yes: A/A exponent difference 0.0002 [-0.0034, 0.0041]", C,
    "Point exponents recompute; CIs not (4 partial). The big exponent gain is relative to a weak incumbent (a full-factor-base IC with q^1.91); the strong comparison is to rho q^1.5 (see row 75). Port reproduces item 7 Python on 42/42 solves."))
row(74,
 cl("Audit: Bernstein-Lange preprocessing rho with equal or smaller advice is 53-317x faster online than IC at n=41/53/61, so the online IC rows are not IC improvements", True, "equal-advice audit (corrects earlier online claims)", "accounting (fairer rho reference)", "L0, 4 vCPU shared by 10 jobs", "yes: BL mismatch-map null control", V,
    "7/7 recompute. This is the reference-matching correction that downgrades the 13.9x n=61 and 12.29x n=83 online ratios quoted in rows 87, 104, 108, 110."),
 cl("49 independent free relations at K=600 = 8.2% of the 599 needed (x1.09)", "partial", "improvement over collecting every relation", "engineering (small)", "counts", "no", C, "49/599 = 8.2% reproduces; the x1.09 factor and the 599 denominator come from README text only (cannot_define)."))
row(75,
 cl("Genus-3 IC after cubic GHS: exponent q^1.650 -> q^1.300 (q^1.377 over 2^11-2^15), for reference rho model q^1.53", "partial", "exponent improvement", "advance over the in-repo incumbent; vs rho: fit exponent below rho's q^1.5 at small q but local exponent trends up", "counts (F_q-mult equivalents), Python, 4 vCPU shared by 10 jobs", "rho fit exponent as null instrument", C,
    "Recomputes; sizes q<=2^15 only. The q^1.53 'rho model' is a fitted model, not a matched measured rho at each size: a comparison to rho by model, not by measurement (see rows 46, 73 for the measured rho at one size)."))
row(76,
 cl("Equivariant Macaulay pass bit-identical to full F2 RREF at n=7-17: 44x/58x/73x CPU at n=11/13/17 (25-45x with assembly)", False, "stage engineering", "engineering (stage diagnostic)", "L0 CPU time, 3 GB cap", "no", U,
    "Recomputed with-assembly range is 17.7-45.2x vs claimed 25-45x. CPU-time ratios at L0 unvalidated; bit-identical claim recomputes 4/4. msolve F4 reaches degree 4 where XL reads 5..>=7: a degree claim, counted. '+21/+112 bits' price not found in repo (cannot_define)."))
row(77,
 cl("msolve runs m=4 N=20 to three degree-8 rounds within 5 GB where f4_fp_tower needs >14 GB; matched f4_fp_tower on all 16 finished systems", True, "instrument comparison", "engineering/negative result for the lead", "L0 (load 22-25 on 4 cores)", "positive control passed (msolve matches committed D)", C, "6/7 recompute; '>14 GB' has no value in the repo. Degree result is negative and the row says so."))
row(78,
 cl("D4/plain S 0.0548 [0.048, 0.061] (18.2x constant, same exponent); double large primes: whole solve 2.31x costlier at measured sizes", "partial", "constant improvement; 2LP negative", "engineering (D4: 18x constant on a size where IC is still ~1e6 x rho)", "counts, L0, 4 vCPU shared by 10 jobs", "yes: dlp-null equals plain on 21/21", C,
    "5/7; aggregator of the ecbench compare is not stated so the ratios are read from compare.txt. 84/84 IC and 672/672 rho logs verified; 12/12 replays identical. Rho in the same session (rho.negation); IC costs are lower bounds (oracle, C4 charged). Distance to rho stays enormous (row 97: 1.39e6x)."))
row(79,
 cl("Koblitz compact-orbit fast path whole-process median 60.9 s -> 37.5 s (1.63x), targets stage 1.95x; bit-identical scalars/relations (48/48)", True, "engineering", "engineering", "L0 (Apple M4 Pro, shared, no isolation receipt; README admits isolated_bench not used)", "no", U, "Process and targets medians recompute from s2 timing (60.93 -> 37.46, 1.63x). Wall at L0 with 3 reps, no A/A: unvalidated. Identical-output claim validated (48/48)."),
 cl("j=0 relation stage 13.99 s -> 11.90 s (1.17x)", None, "engineering", "engineering", "L0", "no", U, "No raw in repo for the j=0 stage (README table only)."))
row(80,
 cl("Joux-Vitse n=5: trial 5.9-9.2e9 F_p mult. flat; complete DLPs IC/rho 4.5e7 / 1.8e8 (p=11) and 6.8e7 (p=31); extrapolated crossover q ~ 2^58.2", True, "accounting correction to scoreboard k=5 constant", "accounting", "counts (isolation none)", "every trial checked against MITM", C,
    "12/14 recompute. C_trial exponent uncertainty +-0.05 vs recomputed 0.045 (std error); the 1.8e8 vs 1.75e8 printed. The scoreboard constant 3.0e10 is not in the repo. Crossover is an extrapolation from p<=31 to 2^58 (flagged as extrapolated). Comparison with rho at two sizes (p=11, 31): n=2 data points."))
