# AUDIT MP-ml_empirical-3 (2026-10-31)
audit: partially_valid. corrected_outcome: inconclusive (leaning refutes for the annealing arm, supports only the pre-existing poly-basis lead).
reran: true (n=17 l=5 B=100 re-run, 233 s; every number identical: arms A/B/C/D fresh means, null p5/med/p95 = 2.5/2.95/3.0, perm p=0.0209). n=19 not re-run (524 s > budget); its numbers were checked against raw_n19_l5.json and match RESULTS.json/README.

## Code vs protocol
- Matches: train 6 targets seed 11, fresh pool seed 999 (first 20 UNSAT per V), arm seeds 101/202/303/404/505/7, Metropolis T 0.3->0.02, B/2 random then B/2 moves, equal B evals per arm, replace-one-basis-vector moves, D family <1,t..t^4> 5 draws, null 40 random V (p5=null[1], p95=null[37], fine).
- Arm C is a real control: score replaced by r.random(), top-5 chosen by random score, so winners are random V. Arm A is a real best-of-B control.
- No off-by-one / wrong-field found. Field F_{2^n} moduli irreducible-by-construction (not independently checked here); x-coordinate test Tr(z+a6/z^2)=0 correct for y^2+xy=x^3+a6. dstar uses N=2l=10 Boolean vars, Dmax=7 (sentinel 8 never hit in data; all fresh D in {2,3}).
## Issues
1. Permutation test (n=17) invalid: the 5 B winners share 3-4 of 5 basis vectors (0x1b364, 0xa3ca, 0x13ba ...), ~1-2 independent subspaces; train ties (all 2.333) were broken by sort order. README admits this; correct treatment: n=17 B result is one lucky V, compatible with null (2/40 null V are <=2.5).
2. Fresh sets differ slightly per V (first 20 SAT-filtered of the same 60), minor. Train/fresh overlap not checked (negligible probability).
3. Power: D* is integer {2,3} on generic V; null sd 0.12 vs pure target-sampling sd 0.08-0.10, so V-to-V variation among random V is almost nil. A 6-target train mean (se ~0.2) cannot rank V. Experiment shows the annealer cannot find signal in a landscape that is ~flat; it is not evidence that no V beats random. Only structured family D (dimVV 9 vs 14-15, D*~2.0-2.2 < null p5) is clearly separated, and the annealer did not reach it (no path from random V to low dimVV with 1-vector moves, objective not seeing dimVV).
4. Protocol prediction_if_true needs n=23 trend; unrun. Main cell n=19 alone fails prediction_if_true (B 2.78 vs null median 2.775, p=0.83), so the pre-registered hypothesis is not supported.
5. D* of UNSAT targets at n=17,19, l=5 is a heavily-overdetermined linear-algebra regime (n equations vs 10 vars); transfer to d_last or large n unsupported (README caveat OK).
## README overclaims
- "refutes" is too strong: it is "not supported at tested power". "The gap shrinks with n" rests on 2 cells with n=17 being one near-duplicate V; better: "no gap at n=19, n=17 gap not distinguishable from one lucky draw". "matches exhausted_at_scale" is a stretch at n<=19.
- Poly-basis claim (D beats null) is valid but is arm D from existing lead, 5 draws, not annealing.
- Numbers, runtimes, dimVV, caveats otherwise accurate. Classification engineering/no n=131 relevance fine.
