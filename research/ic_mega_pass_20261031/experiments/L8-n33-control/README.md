# L8: n=33 control (full) plus an n=27 check with an independent engine

Class: measurement and replication on a toy family (m=2, k=17, N=34; GF(2) degree-4 closure W_4). No security claim. Nothing here changes B*D^2. Novelty: none. Arm A replicates treatise falldegree.tex Thm lfdexact (d_F=4 on all 29 random-subspace instances with 15<=n<=33). This arm is NOT the treatise's T33a/b instance (those are random subsets of ker Tr; Thm lfd5 certifies them at d_F>=5). The generator here draws a random subspace through N1 lib.system().
All numbers come from `src/analyze.py`, which reads `raw/*.jsonl` and writes `RESULTS.json`. PROTOCOL.md was written before any L8 run, but the A/C1/D1 values at n=33 and all n=27 values were already known from L2 and N1 (see PROTOCOL).

## Runs
- R1: `src/run33.py` with the N1 engine gf2close, 4 threads. Seeds s=0..89 use `system(33,17,s)`, i.e. rng Random(1000n+100k+s). nsol comes from bsols (sols for RM). The d=4 closure runs only on draws with nsol=0, and each arm stops after 8 UNSAT draws. CPU 3843 s (closures plus solution counts); wall about 2.3 h on a shared, heavily loaded host.
- R2: `src/run27.py` reruns n=27 A/C1/D1 on the first 8 UNSAT seeds N1 used. It uses the independently written L1 engine `src/macclose` (copied from L1 and built with gcc -O3), 2 threads.

## Results, n=33 (8 UNSAT draws per arm; 95% one-sided Clopper-Pearson upper bound 0.312 on the rate of the opposite outcome)
| arm | draws scanned | UNSAT | 1 in W_4 | low4 | dim4 |
|---|---|---|---|---|---|
| A | 38 | 8 | **8/8** | 6580 = C(34,<=3) (all) | 52956 = all columns |
| C1 | 22 | 8 | 0/8 | 3201 (all) | 45507 |
| C2 | 90 | 8 | 0/8 | 3201 (all) | 45507 |
| D1 | 42 | 8 | 0/8 | 3201 x6, 3212, 3235 | 42884..45507 |
| B | 54 | 8 | 0/8 | 1155 = n(N+1) (all) | 19107 |
| RM | 76 | 8 | 0/8 | 1155 (all) | 19107 |

- No A draw failed. The certified d_F>=5 onset from the treatise therefore does NOT appear on this generator at n=33. With 8 draws, a failure rate up to about 31% cannot be excluded.
- The stand-ins C1/C2 stall at exactly 3201 = n(3N-5) = 33*97. This is the n=31 formula 2821 = 31*91 carried forward. D1 stalls too, but its low4 varies.
- P5: gf2close agrees with L2's gf2close5 on all 24 shared (arm, s) closures and on nsol, with 0 mismatches.
- Prediction check: P1-P6 all held. P2 for C2 and P4 at n=33 are the only predictions that were new.
- Caveat: low4/dim4 are effectively fixed by the degree profile (AUDIT L2 item 9). The 8 identical values per arm are 8 outcomes, not 8 independent confirmations of a mechanism.

## Results, n=27 (macclose)
A, C1 and D1 each give 8/8 with 1 in W_4 and low4 = 3683 = C(28,<=3). These match N1's gf2close records exactly (0 mismatches). So the stand-ins do not stall at n=27, at n=29 (N1: C1 9/9, D1 7/8) or below. They stall at n=31 and at n=33.

## Correction to LEADS2 wording
The claim that "n=27 was not run" is wrong: N1 raw already had n=27 (A 9/9, C1 8/8, C2 8/8, D1 8/8 reach 1). L8 adds an independent-engine replay.
"No n=2k-1 effect" needs qualifying. n=27 (k=14), 29 (k=15), 31 (k=16) and 33 (k=17) all satisfy n=2k-1 (k=ceil(n/2)), so n=2k-1 cannot be what separates stall from no-stall. Within this family, the stand-in stall appears from n=31 on and persists at n=33. Its mechanism remains unexplained. Arm A does not stall at any n up to 33.

## Blockers / not done
None blocking. The n=33 d=5 behaviour of the stalled arms and n=35 were not run (out of scope).
