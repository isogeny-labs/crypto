# AUDIT: L6-frobenius-stable-nonsubspace-bases

Auditor pass, 2026-11-01. Read PROTOCOL.md, README.md, RESULTS.json, raw/, src/ (all of it). Re-ran about 10 CPU-min.
Nothing in the experiment files was modified; audit scratch lives outside the experiment directory and was not kept.
No security claims. Conductor: not run (BRIEF 0.6, task T-5736 holds the directory).

## Verdict: PARTIALLY VALID
The numbers reproduce, the controls are real, and the negative conclusion (no advance, A2 rule FAIL, N1 dead) is supported.
The README overstates its pre-registration check (P1 and P4 are reported as "holds" although their literal pre-registered
criteria were not met), one pre-registered check (CRT) was not run, and a few statements are unmeasured or too strong.
None of the issues changes the classification (relabelling/engineering at best).

## What was re-run, and result
| item | method | result |
|---|---|---|
| yields n=11 CYC2/HAM2/SUB8_s0, n=23 CYC2/HAM2 | recompiled src/l6.c (-O3), same sets | identical N2, N3, subgroup counts, decomps/target to README/RESULTS |
| yields n=11 and n=23 CYC2, independent | PARI/GP (`ellinit`, `elladd`, `ellmul`, Map), other irreducible factor of Phi_p, own subgroup test | N2 = 2640 / 276, N3 raw = 617870 / 1933242, N3 nontrivial = 547184 / 101016, m = 154 / 782: all match |
| group orders | `ellcard` | 1982 = 2*991, 8393806 = 2*4196903: match |
| N1, 256 and 4096 targets | `run_n1.py` unchanged | raw/n1.json reproduced exactly (64 / 894 S3 solutions, L1 0, L2 2, L3 0) |
| N1 ring model | random-element CRT check of the convolution mod z^47-1 against field multiplication, 2000 pairs | 0 failures (see issue 4: the protocol item was not done by the experimenter) |
| N1 lemma | by hand | aug(lifted S3) = 1 for L1 (weight-2 X1, X2 have aug 0, bb = e0), so 0 lifts is forced; for L2 any solution with X1, X2, X3 all symmetric lifts exactly (gamma and gamma^-1 components coincide, aug 0). Every weight-2 symmetric element is an ONB weight-1 element (+-2^i cover Z/47*). The 2 L2 hits are exactly this case and double as a positive control for the lift test |
| solver instances | `solve.py` functions, targets from raw/yields.json | CMS and msolve verdicts, msolve max degrees (3 / 4) and times match raw (HAM2 n=11 target 0: SAT, msolve 5.8 s degree 4; target 11: UNSAT both, gt 0; SUB6 degree 3; SUB8 degree 4; SUB10 n=23 UNSAT both; HAM2 n=23 CMS UNSAT 3 s, msolve TIMEOUT 120 s) |
| msolve CYC2 n=11 | target 0 with timeout raised to 300 s | TIMEOUT at 300 s (no finish; confirms the README blocker at 2.5x its limit; one instance only) |
| trace lemma | Tr(beta_i)=1 for all i, Tr(1)=1 | holds. Over CYC2: Tr(x)=1 iff exponent classes (0, QR, NQR) differ; n=11: 132 candidates, 77 usable; n=23: 552 candidates, 391 usable |
| table arithmetic | RESULTS.json vs README table, every cell | all match (N2, ratios, per column, N3', B*D^2/N, polynomial counts 3601/32549/363/3611/17779, which I also re-derived from the construction) |
| N2 clumping | divisibility | N2 = 276 and 1380 are 3 and 15 multiples of 92 (n * swap * negation); n=11: 2640 = 60 * 44 |

## Code vs PROTOCOL: checked and correct
- S3 expansion, quadratic in x2, half-trace root solving, lift_x, group law, B built from subgroup points via [r]P = O, N2/N3 definitions, trivial-tuple count 3m^2-3m (inclusion-exclusion checked by hand), same 16 targets for every base (rng reset), SUB matched sizes k = round(log2 size), 5 seeds in RESULTS.
- CNF: XOR sign convention, Tseitin ANDs, exactly-2 encoding for CYC, at-most-r and at-least-1 for HAM; msolve system sizes re-derived.
- Controls: S3 self-test on real point triples (1995 / 2000 good, 0 bad); solver verdict versus brute-force S3 ground truth has both signs at n=11 (HAM2: 1 UNSAT, gt 0; SUB6: 2 UNSAT, gt 0) and agrees everywhere gt exists (no `bad` rows when I recomputed it from raw/solve.json.gz).
- Timeout skip rule implemented as written (HAM3 CMS sequence S T U S S S T T then skipped).
- CPU: solver driver uses 2 single-thread processes; yields one process. Within the 2-thread limit. Output size 164 KB.

## Issues (ordered by importance)
1. **P4 reported "holds" but the literal prediction failed.** PROTOCOL P4: "0 exact lifts under L1 (forced), L2, L3". Observed L2 = 2. README's own text explains them (both X1, X2 symmetric, lemma above) and reports 0/892 non-trivial, which is a correct and reasonable reading, but it is a post-hoc reclassification. It should read "P4 literally failed on L2 (2 trivial symmetric hits); holds after excluding the symmetric case, which the lemma forces." The bound 3/892 is a rule-of-three bound over the 892 remaining ordered solutions; ordered pairs (x1,x2),(x2,x1) are not independent, so the effective sample is smaller and the bound is up to about a factor 2 weaker than stated. It is a toy-scale statement about this lift convention only.
2. **P1 reported "holds per point" but the pre-registered criterion was "within factor 2 of the null".** CYC2 n=23 N2 ratio is 2.42 (outside); HAM2 n=11/23 N3' ratios are 2.2 and 1035 (structural, flagged in README). The README substitutes a Poisson-with-clumping criterion. That substitute is fair (276 is only 3 independent units of 92, expected 1.2, P(>=3) about 0.12; HAM3 1380 is 15 units, expected 9, P about 0.05) and N3' is within 0.94 to 1.13 everywhere, but the check should be stated as "N3': yes; N2: outside factor 2 for CYC2 n=23, consistent with clumping noise". The second half of P1 ("per column about n times SUB") is loose: N2 per column is 11 to 18x SUB8 at n=11 but 25x (SUB11) to about 100x (SUB10) at n=23; the ratio is n times (usable-size ratio)^2, so it is confounded by CYC2 having 391 usable x versus SUB10's 264. README's "about n" is only true up to that size mismatch.
3. **"Enumeration ... under 1 ms" is not in any raw data and is not true for the shipped code.** Measured with src/l6.c: 3.2 ms per target at n=23 CYC2 (m = 782 points; bit-serial multiplication and Fermat inversion); larger for HAM3 (m = 1518). An optimized implementation would be under 1 ms, but that was not run. Conclusion (enumeration is orders of magnitude faster than 6 s msolve or a timeout) is unaffected; the number should say "about 3 ms with this code".
4. **PROTOCOL says "CRT check on random elements"; no code does it** (run_n1.py contains the comment only) and the README does not list it as a deviation. I ran it (0 failures in 2000 pairs), so the N1 numbers stand, but it is an undisclosed omission at the time of writing.
5. **RESULTS.json `solve` fields are misleading where ground truth was not computed.** `gt_targets_with_s3_solution` is 0 and `solver_agrees_with_ground_truth` is true for n=23 CYC2, HAM3, SUB10, SUB11 because gt is None (bases > 300 x are skipped) and the aggregation treats None as 0 and an empty comparison as true. README states the gt scope correctly in prose; the JSON should have null/false there. Real gt exists only for n=11 (all four) and n=23 HAM2, SUB8.
6. **The solver instances are a relaxation of the yield accounting.** The yield tables count subgroup points (2E) only; the CNF/msolve systems ask for any S3 solution with x1, x2 in the base (curve or twist, either coset, since P1+P2 in 2E only forces equal cosets). Example: n=11 HAM2 15 of 16 targets SAT, but only 5 of 16 have a subgroup decomposition. This is the same for every base so the comparison stays fair, but README's heading "Decomposition systems (2-sum)" and "SAT" counts are not decomposition into B. State it.
7. **A2 FAIL rests on timeouts and a mixed yield prong.** The degree prong: cyclotomic degree is unmeasured (timeout) so "not lower" is not demonstrated, only "not measured and slower"; the README words this correctly ("unmeasurable or higher"). The yield prong is mixed: at n=11 CYC2 N2/col (377) beats HAM2 (0); at n=23 CYC2 is below HAM3 (16 vs 42) but above HAM2 on N3'/col (5942 vs 1104). README picks the unfavourable HAM3 comparison and the sizes differ (1081 vs 2047). The FAIL verdict is defensible but it is the degree prong plus encoding choice that carry it. Also, CYC2 has 47 variables per point against 8 to 11 for a subspace of the same size: part of the gap is encoding redundancy, not the set. README scope line "these encodings" covers this; the KN-OPEN answer "no" should keep that scope wherever it is quoted.
8. **Wording too strong or imprecise.**
   - "msolve did not finish any CYC2 instance..." means the first two timed out and 14 were skipped by the rule.
   - "B*D^2/N >= 1 for every base with nonzero yield": HAM2 n=23 has nonzero N3' (1104) and ratio 0.023 (README calls that count "structural", which is fine, but the sentence is then false as written). Also CYC1/HAM1 at n=11 are 10.7 only because D is tiny; fine.
   - "CYC_w usability is set by the QR/non-QR split": necessary (Tr(x)=1), not sufficient (also need x on curve with Tr(1/x)=0): 132 candidates give 77 usable at n=11.
   - Timing is wall clock on a machine shared by 6 experiments; my unloaded-ish re-runs gave the same statuses and similar times, so no effect seen, but it is not stated.
   - n=11 "SUB11" (the CYC3-matched control) is the whole field (k = n); degenerate, as is CYC3. Not used in the README claims.
9. Deviation disclosure is otherwise honest: CYC3 at n=23 and SUB14 dropped for the CPU rule; n=131 / mu_263 not executed; novelty_unverified; no security claim.

## Corrections to carry forward
- Replace "P1: holds per point" by "P1: holds for N3' (0.94 to 1.13); N2 outside factor 2 only for CYC2 n=23 (2.42, consistent with 3 independent units); per-column factor n only up to usable-size mismatch".
- Replace "P4: holds" by "P4: literally failed on L2 (2 hits, both forced by the symmetric-lift lemma); 0 non-trivial lifts".
- Replace "under 1 ms" by "about 3 ms with this code".
- Add to README: CRT check was not part of the original run (done by auditor, 0 failures); solver instances relax to any S3 solution in the base.
- RESULTS.json: set gt-dependent fields to null where gt is None.

## Classification check
README: relabelling/engineering at best, no advance, no non-generic signal. Agreed: B*D^2/N >= 1 (apart from the structural HAM2 case) and all per-point yields sit at the random null.
