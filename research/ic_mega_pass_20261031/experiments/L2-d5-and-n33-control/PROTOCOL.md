# L2-d5-and-n33-control -- PROTOCOL (written 2026-11-01 before any run)

Class: measurement. No security claim. Instances: N1 generator (`N1-gf2-d4-profile/src/lib.py::system`, unchanged;
MP-comm_alg-1 arms, seed Random(1000n+100k+s)), k=ceil(n/2), N=2k Boolean variables, n equations.

## Engine
`src/gf2close5.c` = N1 `gf2close.c` with two changes only: (i) pivot rows stored triangularly (row p holds words
0..p>>6; memory ~ncols^2/16 B, ~3.7 GB at N=32,d=5 instead of 7.4 GB); (ii) CPU cap: env `CPUCAP` seconds; on hitting
it the closure prints a partial JSON (`capped:1`, npiv, nlowpiv, queue position) and exits 0. Closure algorithm
(degree-<=d Macaulay closure under x_v multiplication of low pivots, x^2=x) unchanged.
Validation BEFORE use: (V1) d=5 vs Python reference `MP-comm_alg-1/tensor_closure.py::closure` on n=9,11,13, all arms
of system(), draws s=0..2: [dim, low, one] must match exactly. (V2) d=4 regression vs N1 raw records (n=19 all
records with closure; n=31 C1 s=1). Any mismatch -> stop, report blocker.

## (a) d=5 on stalled n=31
Task names C1 s=1 and D1 s=0. N1 raw shows D1 s=0 at n=31 has nsol=5 (SAT): 1 is never in any W_d for it, so it is
not a stalled UNSAT instance. Pre-registered substitution: D1 s=25 (first UNSAT D1 draw at n=31, stalled low_4=2835).
D1 s=0 is run only if CPU budget remains, as a SAT sanity check (must give one=0).
Cap: 40 CPU-min per closure (2400 s, clock()). 2 threads max (the two closures may run concurrently).
Predictions (P-a): from treatise d_F=4..5 data at n<=35 and the stall pattern, I predict 1 in W_5 for C1 s=1 and
D1 s=25 (d_F=5), probability ~0.6 each; ~0.4 that the cap is hit before termination (cost estimate ~x50-200 of the
19 s d=4 closure under current load ~400).

## (b) n=33 (k=17, N=34) d=4, arms A, C1, D1
Draws s=0,1,2,... ; per arm close (d=4) every UNSAT draw (bsols nsol=0) until 8 UNSAT draws per arm or s=60.
Report per arm: #UNSAT closed, #with 1 in W_4, low_4 values, CPU per closure.
Predictions (P-b): A: 1 in W_4 on >=7/8 (treatise reports d_F=4 on most random-subspace instances at n<=33 but
d_F>=5 on T33a/b which are ker-Tr subspaces, not this generator; so A failures possible but I predict 0-1).
C1, D1: stall (1 not in W_4) on >=7/8 each, continuing the n=31 pattern.
This is NOT the treatise's named-instance control (T33a/b are not reproducible by this generator); stated as such.

## Rules
No tuning; negative/capped results reported as-is. Outputs <200 MB; logs gzipped.
