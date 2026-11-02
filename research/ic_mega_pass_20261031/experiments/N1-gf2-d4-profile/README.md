# N1-gf2-d4-profile -- README (written 2026-11-01 ~12:50 PST during repair; run NOT finished)

Class: measurement / replication. No security claim. No change to B*D^2. Novelty: none on the main question
(prior art below). All numbers come from `analysis/analyze.py` over `raw/*.jsonl` + `raw/run_main.log` -> `RESULTS.json`.

## Status
- Planned n: 19,21,...,31. Completed at writing: n = 19, 21, 23, 25 (`raw/run_main.log`). n = 27 was in progress
  (partial file, excluded from all totals); n = 29, 31 not started. **Last completed n = 25.**
- The main run was still going when this README was written. It must be regenerated (`python3 analysis/analyze.py`)
  and re-audited number by number once the run ends; the numbers below cover n <= 25 only.
- Cost rule: not implemented in code (manual). After n = 25 cumulative child CPU = 296 s. Projecting x4 per +2n for the
  next n only gives ~0.29 CPU-h (no stop); projecting all remaining n (27, 29, 31) gives ~4.35 CPU-h (> 2.5 h, stop).
  The protocol does not say which; the run was not stopped, i.e. the next-n reading was applied in practice.
  Measured growth per +2n is x2.4-2.8, not x4.

## Prior art (omitted from PROTOCOL.md)
The treatise (`brief/treatise/falldegree.tex`, Thm lfdexact and Thm lfd5) already measured this diagonal (m=2,
k=ceil(n/2)): d_F = 4 on all 29 random-subspace instances with 15 <= n <= 33; d_F >= 5 certified (1 not in W_4) at
n = 33 (T33a/b), 35 (R35a/b, U35a/b, Q35a/b), 37 (R37a/b) and 45 (P45a); exactly 5 at R35a (n=35, k=18).
PROTOCOL.md did not cite this; that is an omission. Consequences:
- The A/C1/C2/D1 results below are a **replication consistent with prior d_F = 4 data**, not a new result.
- The planned range (n <= 31) ends **below the known d_F >= 5 onset (n = 33-35)**, so this design cannot see the
  transition the treatise reports, and its trend clause cannot say anything about growth of d_F.
- No n = 33/35 run and no known-answer R35a/T33a-type positive control was done. Why: the T33a/R35a instance
  definitions (subspace inside ker Tr, specific seeds) are not reproduced by this generator, and an n = 33/35 closure
  (~67k columns, ~0.5 GB per process, CPU extrapolated from x2.4-2.8 per +2n at several CPU-h) would exceed the
  3-thread share while the main run still occupies those 3 threads. It is therefore NOT done; the experiment has no
  positive control for "1 not in W_4" on an A-like arm (B/RM are nulls without structure, not such a control).

## Predictions -- how to read them
- P1 (A,C1,C2) and P4 (static d_NS > 4) were written **after** a 9-instance pilot that showed A with 1 in W_4 at n=29;
  they are pilot-informed, and given the prior art P1 was close to foregone. Not independent confirmed predictions.
- P3 (B, RM: low_4 = n(N+1), 1 not in W_4) is a **counting-identity null calibration** (n equations plus N*n
  products x_v f_i with no cancellation), also seen in the pilot. Not an empirical prediction.
- P2 (D1) shares A's low-rank mechanism.

## Results, n = 19..25 (complete n only)
UNSAT draws per arm per n are U = 8-10: the target was 8, but batches of 3 seeds are processed before counts update,
so some arms overshoot to 9 or 10. A always has even nsol (a<->b swap), so its UNSAT-conditioned sample is conditioned
differently from B/RM.

Fraction of UNSAT draws with 1 in W_4 (degree-4 closure refutes), with exact one-sided 95% Clopper-Pearson upper bound
on the rate of the opposite outcome. Per arm, not pooled across arms (A, C1, C2, D1 are not independent replicates).

| n | A | C1 | C2 | D1 | B (1 in W_4) | RM (1 in W_4) |
|---|---|---|---|---|---|---|
| 19 | 9/9, fail<=0.283 | 10/10, <=0.259 | 9/9, <=0.283 | 8/8, <=0.312 | 0/8, <=0.312 | 0/9, <=0.283 |
| 21 | 8/8, <=0.312 | 8/8, <=0.312 | 8/8, <=0.312 | 9/9, <=0.283 | 0/8, <=0.312 | 0/8, <=0.312 |
| 23 | 8/8, <=0.312 | 8/8, <=0.312 | 9/9, <=0.283 | 9/9, <=0.283 | 0/8, <=0.312 | 0/9, <=0.283 |
| 25 | 9/9, <=0.283 | 8/8, <=0.312 | 8/8, <=0.312 | 9/9, <=0.283 | 0/8, <=0.312 | 0/8, <=0.312 |
| per arm, n=19..25 | 34/34, <=0.084 | 34/34, <=0.084 | 34/34, <=0.084 | 35/35, <=0.082 | 0/32, <=0.089 | 0/34, <=0.084 |

("fail" = upper bound on the rate of 1 not in W_4 for A/C/D1, of 1 in W_4 for B/RM.) A rising failure rate below
~26-31% per (arm, n) would be undetectable at these U.

Other observables (complete n):
- Random draws s=0..9: excess = 0 on 10/10 for each of A, C1, C2, D1 at each n. However r_s == nsol on every closure
  record, so excess = C(N,<=3) - nsol - low_4 carries no information beyond low_4 and nsol; it is not an independent
  observable.
- B and RM: low_4 = n(N+1) exactly (399, 483, 575, 675) on every closure record (the counting identity).
- Static d_NS: > 4 (-1) on every closure record, all arms.
- Zero violations of P1-P4 at n <= 25.

Supported statement: through n = 25, the degree-4 closure refutes every sampled UNSAT A/C1/C2/D1 draw and none of the
B/RM draws, consistent with (and not extending) treatise Thm lfdexact. Not supported: any statement about growth or
non-growth of d_F; anything for k << n, m >= 3, or n = 131.

## Not covered by this repair (items 5-6 of AUDIT.md, not requested in this pass)
Missing validation artifacts (bsols-vs-FES script/output, festest output, n=19 Python-closure cross-check) and the full
CPU-accounting statement remain open; see AUDIT.md.

## Repair
Requested: AUDIT.md must_fix items 1-4 (user: "please do 1-4 now"). Items 5-6 not done in this pass.
1. README/RESULTS from saved script: added `analysis/analyze.py`; `RESULTS.json` generated from raw. Last completed n
   (25), partial n (27), cost-rule status (manual, not fired; two readings) recorded. Not yet done: regeneration and
   number-by-number re-audit after the run ends -- the run was still in progress.
2. Prior art: cited Thm lfdexact/lfd5, noted PROTOCOL omission, labelled results replication, stated range ends below
   the d_F >= 5 onset; n=33/35 and positive control not done, with reasons. No "persists"/"no growth" wording.
3. P3 labelled counting-identity null calibration; P1/P4 labelled pilot-informed.
4. Exact Clopper-Pearson bounds per (arm, n) and per arm, not pooled; U overshoot, A's even-nsol conditioning, and
   excess's non-independence stated.
No original numbers were superseded (there was no prior README/RESULTS). Conductor check could not be run
(connection to the local conductor server blocked by the sandbox).

## Addendum 2026-11-01 (main session): run completed through n=31

The main run finished after the audit snapshot (raw/run_main.log: n=19..31 complete, 3,795 s child CPU).
`analysis/analyze.py` was re-run; RESULTS.json now covers n=19..31 and supersedes the n<=25 numbers above.

| n | A | C1 | C2 | D1 | B | RM |
|---|---|---|---|---|---|---|
| 19-27 | all UNSAT draws 1 in W_4 | all | all | all | none | none |
| 29 | 10/10 | 9/9 | 8/8 | **7/8** | 0/8 | 0/8 |
| 31 | **9/9** | **0/8** | **0/8** | **0/8** | 0/8 | 0/9 |

(counts are 1 in W_4 / UNSAT draws; random-draw excess=0 is 10/10 for A,C1,C2,D1 at every n<=29 and 0/10 for C1,C2,D1 at n=31.)

At n=31 (N=32) every C1, C2 and D1 draw stalls at low_4 = 2821 (of C(32,<=3) = 5489), while the true Semaev arm A
reaches the full 5489 and refutes all 9 UNSAT draws. P1/P2 are falsified for C1, C2, D1 at n=31; P1 holds for A.

Checks on the instrument:
- Variable-permutation invariance at n=31 (crosscheck/perm_test.py, crosscheck/perm_test.json): C1 s=1, D1 s=0 and A s=2 give identical (low_4, dim_4, one)
  under two random relabellings, so a fixed-index bug at variable 31 is excluded.
- Independent msolve F4 over GF(2) on n31_C1_s1 (crosscheck/c1s1.log) did not finish degree 4 in 600 s: no verdict.
- No independent closure implementation was run at N=32. The C closure was validated against Python only to n=17 (d=4)
  and n=19 (5 instances, audit).

Reading (interpretation, not established): if the stall is real, A's degree-4 refutation at n=31 is NOT explained by
its F_2-rank-n tensor structure alone, which contradicts the mega pass's "relabelling" reading for d=4 at this n.
Before believing it: (1) an independent N=32 closure (d=4) on one C1 and one A instance; (2) n=33 for A, where the
treatise certifies d_F>=5 on named instances, as the positive control; (3) d=5 closure on a stalled C1 instance.
Nothing here concerns d_F growth for A, n=131, or any deployed curve.
