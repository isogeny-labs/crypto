# N1-gf2-d4-profile -- protocol (written 2026-11-01 BEFORE the main run)

Class expected: theory/measurement (REPORT §6 step 1). No security claim. No B*D^2 change claimed. novelty_unverified.

## Object
Binary Weil-descended S_3 (m=2) systems of MP-comm_alg-1 (generator `MP-comm_alg-1/run.py::arms`, imported unchanged,
seed `random.Random(1000*n+100*k+s)`, always called with the full ARMS list so rng streams are identical).
k=ceil(n/2), N=2k=n+1 Boolean variables, n equations. Arms closed: A (mu_V, true Semaev), C1 (mu_W), C2 (Lr o mu(W1xW2)),
D1 (random F_2-rank n), B (dense random bilinear, same affine part = bilinear-matched null),
RM = `ffd_semaev_core.random_matched(A)` (same per-equation degree profile, monomials drawn over all N vars, so NOT bilinear:
it contains a_i a_j and b_i b_j terms; this is the degree-profile null, B is the bilinear-matched null), own seed
`7_000_000+1000*n+100*k+s`.

## Instrument
`src/gf2close.c` (C, -O3 -march=native): bit-packed GF(2) Boolean closure W_d (same definition as MP-comm_alg-1
tensor_closure.closure: span of F, closed under x_v * g for g in W with deg <= d-1, to fixed point), chunked
M4RI-style elimination (8-row Gray tables). Validation (done before this protocol, raw/validation.jsonl): all 29
Python cells of MP-comm_alg-1 raw/ (d=3 n=9..45, d=4 n=9..17), all 8 arms, all draws: 4240/4240 [dim,low,one]
identical. Solution sets: `bsols` (fix a, linear in b) for bilinear arms, `sols` fast exhaustive search (Gray code)
for RM; FES validated vs brute force (30 random systems) and bsols vs FES (150 instances, n=9..21), all equal.
Static d_NS: smallest D in {2,3,4} with 1 in span{m*f_i : deg m <= D-deg f_i}; ">4" (-1) otherwise.

## Observables per instance (d=4)
low_4 = dim(W_4 ∩ B_{<=3}); one = [1 in W_4]; nsol; r_s = rank of the degree<=3 monomial evaluation matrix on V(F);
excess = C(N,<=3) - r_s - low_4 >= 0 (asserted: W_4 ∩ B_{<=3} lies in the degree<=3 vanishing space).
excess = 0 means the degree-4 closure already gives the full degree<=3 part of the vanishing ideal; on UNSAT
instances excess=0 <=> 1 in W_4. Assert d_NS<=4 => one=1.

## Design and cost rule (fixed before the run; outcome-independent)
n in {19,21,23,25,27,29,31} ascending. Per n: R=10 random draws s=0..9, all six arms (low_4 profile).
UNSAT-filtered sample: continue s=10..149 and close each arm on its UNSAT draws until that arm has U=8 UNSAT draws in
total (including those among s<10); seeds beyond 149 not used (shortfall reported).
Threads: 3. Budget 3 CPU-h; if cumulative CPU after finishing an n projects (x4 per +2 in n) past 2.5 h, stop and
report the n reached. Pilot already seen (n=21,25,29, draw 0, arms A,B,RM; raw/timing_pilot.txt): disclosed;
predictions below were written after seeing those 9 instances.

## Predictions
P1 (A, C1, C2): excess = 0 in >= 90% of draws at every n; 1 in W_4 on 100% of UNSAT draws. I.e. degree-4 closure
refutes every UNSAT instance: no evidence of d_F > 4 at n <= 31 in this (m=2, k=ceil(n/2), N=n+1) regime.
P2 (D1): same as A (rank-n mechanism, MP-comm_alg-1 audit).
P3 (B, RM): low_4 = n(N+1) exactly (no degree<=3 falls at d=4), 1 not in W_4 on every UNSAT draw (d_F >= 5),
excess ~ C(N,<=3) - n(N+1), growing ~N^3/6.
P4: static d_NS > 4 for all arms at n >= 19 (dynamic closure beats static where P1 holds).
Falsification of "d_F <= 4 persists" for A-like arms: any UNSAT A/C/D1 draw with 1 not in W_4; a rate of such draws
increasing with n would be evidence that d_F > 4 appears/grows. Scope limit: 1 not in W_4 on an UNSAT instance shows
only d_F >= 5; growth of d_F can only be read from the trend of that rate / of excess vs n, against B and RM.
Nothing here measures d_F beyond 4, nothing transfers to l << n (k << n) cells, m>=3, or n=131.
