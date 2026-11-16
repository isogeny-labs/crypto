# L11 PROTOCOL (written 2026-11-02 before any L11 run)

Toy GF(2) measurement. One generator (N1 `lib.system`, rng Random(1000n+100k+s)), one closure (N1 `gf2close`, round-based W_d, d=4). No security claims. Arm A = Semaev on a random subspace; this is NOT the treatise's T33a/b family.

## Runs
- R1 `src/run.py n`: for n=35 (k=18, N=36) and n=37 (k=19, N=38), scan seeds s=0,1,..; nsol via `bsols`; on UNSAT draws run d=4 closure. Stop per arm at 8 UNSAT draws (A and C1). C1 is the stand-in control. If a single n=37 closure exceeds 45 CPU-min, stop at what is reached.
- R2 `src/rnd3.py`: L9 `macdump` with maxround=3 (no dumps) on 2 UNSAT A draws per n, plus 1 C1 draw per n. Round-3 falls := low rank after round 3 minus low rank after round 2.
- At most 3 concurrent single-threaded processes.

## Memory projection (dense pivot matrix, ncols x ncols bits)
- n=35: ncols = C(36,<=4) = 66712 -> ~531 MB/process.
- n=37: ncols = C(38,<=4) = 82993 -> ~821 MB/process; 3 procs ~2.5 GB. Feasible.

## Pre-registered predictions
- P1: A reaches 1 in W_4 on >= 7/8 UNSAT draws at n=35, low4 = C(36,<=3) = 7807, dim4 = 66712.
- P2: same at n=37: low4 = C(38,<=3) = 9178, dim4 = 82993. (Weaker confidence; this is the real question.)
- P3: C1 stalls (1 not in W_4) on 8/8 at both n, low4 = n(3N-5): 3605 (n=35), 4033 (n=37).
- P4: A round-3 falls = 4n: 140 (n=35, both seeds), 148 (n=37, both seeds); round-2 low rank = n(3N-5). C1 round-3 falls = 0.
- P5: CPU per n=37 closure < 45 min (extrapolating n=33 ~44 s by ncols^3: ~3-4 CPU-min).
- Falsification: any A UNSAT draw with one=0 at n=35/37 is an onset of d_F>=5 for arm A on this generator (in this closure's sense).
