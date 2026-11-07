# L8 n=33 control: PROTOCOL (written before any L8 run, 2026-11-02 ~09:05 PST)

No security claims. This is a measurement/replication on a toy family (m=2, k=ceil(n/2), N=2k, GF(2) Boolean closure W_4).

## Data reused (not rerun as a new measurement)
- L2 raw/n33_k17_d4.jsonl (gf2close5 triangular engine, d=4): A, C1, D1 already have 8 UNSAT draws each (s<=40).
  A 8/8 one=1; C1 8/8 one=0 (low4 3201); D1 8/8 one=0 (low4 3201,3212,3235). These are known BEFORE this protocol,
  so predictions for A/C1/D1 at n=33 are NOT independent; they are re-checked here with the N1 engine gf2close.
- N1 raw/n27_k14_d4.jsonl already has n=27 A 9/9, C1 8/8, D1 8/8 one=1 (low4 3683) with gf2close. Also known beforehand.

## Runs
R1 (n=33): all 6 arms A,C1,C2,D1,B,RM, seeds s=0.. with system(n,k,s) from N1 lib (rng Random(1000n+100k+s)),
   bsols (RM: sols) for nsol; degree-4 closure with N1 gf2close only on nsol==0 draws; stop an arm at 8 UNSAT; s<=150.
   4 threads max.
R2 (n=27): A,C1,D1 on the same UNSAT seeds N1 used (first 8 per arm), closure with the independent L1 engine macclose.

## Pre-registered predictions
P1 A n=33: every UNSAT draw has 1 in W_4, low4 = C(34,<=3) = 6580. (Known from L2 for 8 draws; treatise Thm lfdexact.)
P2 C1, C2 n=33: 0 UNSAT draws reach 1; low4 = n(3N-5) = 3201 on every draw (C2 is NEW: not run before).
P3 D1 n=33: 0 reach 1; low4 in [3190, 3260], not always 3201.
P4 B, RM n=33: 0 reach 1; low4 = n(N+1) = 1155 exactly (counting identity, calibration only; NEW at n=33).
P5 gf2close reproduces L2's (dim4, low4, one) on every A/C1/D1 seed common to both.
P6 n=27 macclose: A, C1, D1 8/8 reach 1 with low4 = 3683, matching N1 record-for-record.
Failure of P1 on any draw = an A draw with 1 not in W_4 (d_F>=5 on this generator at n=33).
