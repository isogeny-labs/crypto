# AUDIT L2-d5-and-n33-control (snapshot 2026-11-01 ~19:35 PST)

audit: partially_valid | reran: true | corrected_outcome: provisional. The experiment is NOT finished. There is no README.md, no RESULTS.json and no analysis script, so "numbers in README match RESULTS" cannot be checked yet. Part (a) has produced no result, part (b) is partial.

## State of the directory
- Present: PROTOCOL.md, src/{gf2close5.c, gf2close5, l2lib.py, validate5.py, regress4.py, run_a.py, run_b.py}, raw/{validate_d5.jsonl, regress_d4.jsonl, n33_k17_d4.jsonl (growing), prog_d5_C1_s1.log (growing), d5_n31.jsonl (0 bytes), run_a.err, run_b.err, run_b.out (0 bytes)}. Size 112 KB, well under 200 MB.
- Process listing (ps/pgrep) is denied in this sandbox. "Still running" is inferred from mtimes: n33 jsonl advanced 19:04 -> 19:18 and the d=5 progress log advanced 19:02 -> 19:34 (qh=12000 of 15841 queued low pivots, cpu 733 s of the 2400 s cap, npiv=120514). Machine load average was 175-275 during the audit, so wall time is 6-9x CPU time.
- Ordering: PROTOCOL.md 18:40, gf2close5.c 18:40, validate_d5 18:49, regress_d4 18:54:16, main runs 18:54:26. Protocol precedes every result file. (PROTOCOL and the binary share a minute, so "before any run" is shown only at that resolution.)
- Conductor: BRIEF section 0 says T-5736 holds the directory and agents must not run `conductor check`. I did not run it either. CLAUDE.md asks for one before code changes. I made none, only this file under the reserved directory.

## Verified (reran, all well under 10 CPU-min)
1. Validation artifacts are real and the arithmetic holds. validate_d5.jsonl has 81 cells (n=9,11,13 x s=0..2 x 9 arms A,Ap,B,C1,C2,D1,D2,D4,RM), 0 mismatches C vs Python tensor_closure at d=5, 20 cells with one=1. regress_d4.jsonl has 101 cells (all n=19 closure records plus n=31 C1 s=1 and D1 s=25), 0 mismatches vs N1 raw. Both match the PROTOCOL V1/V2 statements.
2. Weakness of V1: the Python reference only reaches n=13, so the triangular-storage change is checked against the reference only at ncols <= ~8000. V2 is d=4 only. I closed this gap with new checks (3, 4).
3. Triangular d=5 engine vs the independent N1 non-triangular engine (gf2close, d=5), n=19 s=0,1 arms A,C1,D1,B,RM (10 cells) plus n=21 s=0 arm A (1 cell): 11/11 equal in [dim, low, one] (e.g. C1 s=1 gives [21700, 6196, 1]; n=21 A gives [35441, 9107, 0]). The n=21 C1 and D1 cells were still running when I stopped waiting.
4. n=33 d=4, independent N1 engine on C1 s=0 and D1 s=0: [45507, 3201, 0] for both, equal to the L2 records (dim4 45507, low4 3201, one 0).
5. Solution counts at n=33 s=0, by a different method: the fast-exhaustive-search mode ("sols", full 2^34 enumeration) gives nsol A=2, C1=0, D1=0. The bsols values in n33_k17_d4.jsonl are the same. So the UNSAT/SAT split used to select closures is confirmed on those three instances. bsols was not otherwise checked at N=34.
6. Protocol substitution is correct and was disclosed. N1 raw n31_k16_d4.jsonl: D1 s=0 has nsol=5 (SAT, so 1 can never be in W_d), and s=25 is the first D1 draw with nsol=0 (s=0..24 are all SAT). C1 s=1 has nsol=0 and one=0, low4=2821. D1 s=25 low4=2835, one=0. So the target instances are genuine stalled UNSAT cases.
7. Code reading of gf2close5.c. The closure queue logic (only pivots below nlow are multiplied; mono|bv gives x^2=x; early exit on pivot 0), the triangular rows (row p holds (p>>6)+1 words; loc and table sizes use nw = (c1>>6)+1) and M4RI-style block reduction are consistent. I found no bug that the checks above would not expose. Memory is 460,777,515 words (3.69 GB) for N=32, d=5, matching ncols=242,825.
8. Control reality at n=33. Arm A (Semaev on a random subspace, the treatise-like arm) on its first UNSAT draw (s=1) has one=1, dim4=52956=ncols, low4=6580 (full closure). C1 and D1 UNSAT draws so far (s=0,1,2) give one=0, low4=3201. So the positive control (A reaches 1) and the stall arms behave as predicted, on the draws completed so far.
9. Observation, not a defect: C1 and D1 give identical dim4/low4 on different draws (45507/3201 on s=0 for both arms, and on C1 s=2; D1 s=1 gives 45482/3201). N1 shows the same at n=31 (low4=2821 for all C1 draws). The closure is saturated by the degree profile, so low4 carries almost no per-instance information. The README must not present such counts as independent evidence.

## Problems / findings
A. Prior-art statement is inaccurate. PROTOCOL P-b says the treatise reports d_F=4 on "most" random-subspace instances at n<=33. falldegree.tex Thm lfdexact says d_F=4 on ALL 29 random-subspace instances with 15<=n<=33, and Thm lfd5 certifies T33a/b (random inside ker Tr) at >=5. Arm A at n=33 is therefore a replication of known data, not a new measurement. The README must cite this.
B. The n=33 control is not the treatise's named instance. T33 is "random subset of K_0 = ker Tr"; the generator draws a random subspace of GF(2^n) (rand_subspace). PROTOCOL says so. The README must keep this caveat and must not describe the A arm as "the T33 control".
C. Selection: closures are run only on UNSAT draws. For A, nsol is about Poisson(2), so only about 13% of draws are UNSAT (observed so far: s=0,2,3 SAT, s=1 UNSAT). Reaching 8 UNSAT draws by s=60 is borderline. Report the number actually reached per arm, not "8". With n=8 the one-sided 95% Clopper-Pearson bound on a failure rate after 0 failures is about 31%. Prediction thresholds ">=7/8" are weak.
D. Capped runs are inconclusive, not a stall. On hitting CPUCAP the program prints before flushing the pending chunk of up to S=1024 rows, so npiv, nlowpiv and `one` are undercounts, and the queue is not exhausted. A capped d=5 record must be reported as "cap hit at X CPU s, queue qh/qt, 1 not yet in span", never as "1 not in W_5". The cap path itself has no correctness test (my attempt is in the addendum).
E. CPU cap is clock() of the child (CPU time), while wall time is 6-9x larger under the current load. The README should report both. The two d=5 closures plus n33 run use about 3 processes at times; the 2-thread limit applies per experiment and I could not verify it, because ps is denied.
F. Process: PROTOCOL gives the d=5 CPU estimate "x50-200 of the 19 s d=4 closure". The progress log supports the C1 s=1 run being on a path to hit or approach the 2400 s cap (733 s CPU at 12000/15841, superlinear growth). Whatever happens, it must be reported as measured. A d=5 result with 1 in W_5 would then rest on this unverified-at-scale triangular engine, so the n=19/21 cross-checks above matter.

## Overclaim check
No README exists, so nothing is overclaimed. Guard rails for the README:
- no security claim, no B*D^2 statement beyond the BRIEF's certificate wording; this is a measurement on toy sizes
- do not call capped runs "stalled at d=5"
- do not generalise from n=31/33 to larger n; the treatise's own d_F>=5 data at n=35,37 is not reproduced here
- state that the substitution D1 s=25 for D1 s=0 happened, and why
- state V1 coverage limit (reference only to n=13)

## must_fix before the final README
1. Write README.md and RESULTS.json from a saved analysis script reading raw/*.jsonl, then re-audit number by number (I will not trust hand-copied numbers).
2. Cite treatise falldegree.tex Thm lfdexact/lfd5 and correct "most" to "all 29" (A above).
3. Report per arm: UNSAT draws reached, one=1 counts with exact binomial bounds, both CPU and wall.
4. Treat capped records as inconclusive (D above).
5. Save the audit-style cross-checks (triangular vs non-triangular d=5 at n=19/21) if the d=5 result is used for a claim.

corrected_outcome (provisional): validation of the engine is genuine and passes all independent re-checks I ran; selection of instances and the SAT/UNSAT split are confirmed; arm A reaches 1 in W_4 at n=33 s=1 and C1/D1 stall at W_4 on the draws so far. Part (a) has no completed result. No headline can be audited until README/RESULTS exist.

## Addendum (19:40 PST): checks that did not finish
Under load ~200 these were too slow to complete in the time I allowed: (i) fresh-seed re-validation of gf2close5 vs the Python reference (n=9 s=3..5, n=11 s=3..4, all 9 arms), (ii) the CPUCAP code-path test (n=25, C1, d=5, cap 1 s), (iii) n=21 s=0 C1/D1 vs the N1 engine. They are NOT claimed as verified. The scripts are small and sit in my scratch directory, not in this experiment. The 2-thread limit was exceeded briefly by my audit (3 short processes).
Re-audit needed once README.md/RESULTS.json exist and the d=5 closures finish or hit the cap.
