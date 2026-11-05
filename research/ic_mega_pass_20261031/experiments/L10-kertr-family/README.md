# L10-kertr-family -- README

Toy-scale measurement. No security claims. One closure definition: N1 gf2close W_d (gf2close5 for d=5).
Generator: src/lib10.py, which follows treatise gen2.py (basis=tr0, zmode=curve, with the Tr(a6/z^2)=1 rejection) and is
built with the N1 tensor_closure helpers. Numbers come from RESULTS.json, which analysis/analyze.py builds from raw/*.jsonl.
PROTOCOL.md was written before any closure ran.

## Answer
Yes, but only at n=33. On this closure the ker-Tr family T fails degree-4 refutation one step earlier in n than arm A:

| n | T (V in ker Tr): fails d=4 / UNSAT, CP95 | A (random V, uniform z): fails / UNSAT, CP95 |
|---|---|---|
| 29 | 0/8, [0, 0.369] (low4 4526) | 0/10, [0, 0.309] (N1 raw) |
| 31 | 0/8, [0, 0.369] (low4 5489) | 0/9, [0, 0.336] (N1 raw) |
| 33 | **16/16, [0.794, 1]** (low4 2968, dim4 48068 on all 16) | **0/8, [0, 0.369]** (L8, low4 6580) |
| 35 | 8/8, [0.631, 1] (low4 3323, dim4 57503 on all 8) | 8/8, [0.631, 1] (new run; low4 3885, dim4 61425 on all 8) |

- The onset is n=33 for T and n=35 for A. At n=35 both fail, at different plateaus (T 3323 < A 3885).
- Every failing draw sits on one deterministic plateau per (family, n), equal to the treatise's certified values: T33a/b 2968/48068, R35a 3885/61425.
- Population answer for open problem 13 on this closure and toy range: within each (family, n) cell, the failure rate is 0 or 1. The sampled named instances are typical of their family, not cherry-picked.

## Validation
- V1: lib10.build equals the treatise s3_descent equation-for-equation, and modulus(n) equals the published modulus on 7/7 named instances.
- V2: gf2close reproduces the published (low, dim, one) on 7/7 named instances:
  - T29a d4 (4526, 31931, 1)
  - T31a d4 (5489, 41449, 1)
  - T33a d3 (64, 2208, 0) and d4 (2968, 48068, 0)
  - T33b d4 (2968, 48068, 0)
  - R33a d4 (6580, 52956, 1), the same tuple as L8's arm A
  - R35a d4 (3885, 61425, 0)
- A35 reproduces R35a's tuple on 8/8 draws. This is consistent with the note in PROTOCOL: for odd n, curve-z with uniform a2 is uniform z, so A is distributionally the treatise R/U family.
- C21a/b (the t=3 chain) was not run. It is a different system (N=42, cubic) with no bearing on the m=2 question.

## Predictions
- P1 held on all 237 T draws: span rank n-1 and no degree<=1 part. All A35 draws have rank n with low part 1.
- P2 held.
- P3 held.
- P4 held (16/16 at 2968/48068).
- P5 held.
- P6: see the d=5 section.
- All of these were informed by the treatise and are not independent.

## d=5
- Planned: gf2close5 on T33 s=3, CPUCAP 10800 s. The result is in the "d5" section of RESULTS.json and in the addendum below.
- Blocker: n=35 d=5 needs about 444k columns and about 12 GB of triangular storage, at many CPU-hours per draw. It was not run for T or A.

## Caveats
- T-family UNSAT draws are about 7-20% of all draws (nsol is mostly 2-8, even). The rates above are conditioned on UNSAT, as in the treatise.
- Reject counts are logged per draw.
- There are 8-16 draws per cell. A minority rate below roughly 20-37% would not be seen.
- One closure definition and one generator convention were used. "Fails d=4" means 1 is not in W_4 for this closure, with no independent certificate (the treatise certifies T33a/b and R35a/b only).
- The thread cap was exceeded briefly: three d=4 processes ran together with validation runs early on (up to 4 single-thread processes for about 2 min). A pkill of the T33 extension was blocked by the sandbox, so T33 ran on to 16 UNSAT draws before d=5 started.
- No conductor check was run, per BRIEF 0.6.
- Classification: replication/measurement. Novelty: none on onset values (the treatise's T and R tables). New here: population rates per cell and the A35 onset (LEADS3 open item) closed as fail.

## Addendum: d=5 (blocker)
- T33 s=3 with gf2close5 (cap 10800 CPU-s) is **inconclusive and blocked**.
  - The first attempt died with an empty log. The cause is unknown.
  - The rerun reached peak RSS 6.7 GB, then thrashed in swap on the shared host (about 30% CPU, about 90 MB free RAM).
  - It was killed after about 4500 CPU-s and 9100 s wall. It never reached a W_5 dimension or the `one` flag, and it logged no progress.
- P6: not tested (blocked).
- The T-only d=4 failure at n=33 therefore has no d=5 follow-up from this run.
- n=35 d=5 was declared infeasible in advance for both families.
- The d=4 answer does not depend on any of this.
