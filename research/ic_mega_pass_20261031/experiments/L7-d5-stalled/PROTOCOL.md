# L7-d5-stalled -- PROTOCOL (written 2026-11-02 09:05 PST, before any run)

Class: measurement. No security claim. Toy sizes only.

## Context (from L2 raw, read before writing this)
- L2 `raw/d5_n31.jsonl`: D1 s=25 (n=31) reached 1 in W_5 (dim=ncols=242825, cpu 2393 s, just under the 2400 s cap).
  C1 s=1 (n=31) hit the 2400 s cap at queue qh=21215/qt=37979, one=0 -> INCONCLUSIVE (AUDIT L2 item D).
- D1 s=0 at n=31 is SAT (nsol=5, N1 raw): 1 is in no W_d. Not a stalled UNSAT instance; L2's substitution D1 s=25
  stands and is not re-run here.

## Engine
Reuse L2 `src/gf2close5` (copied, unchanged; sha256 recorded) = N1 gf2close.c + triangular pivot storage + CPUCAP.
Single-threaded. Memory 3.69 GB at N=32,d=5 (fits the ~8 GB plan). No checkpointing is added: the engine is not
re-entrant and adding a resume path would itself need validation; instead a single run with a larger cap is used.
Generator: N1 `src/lib.py::system` unchanged.
Validation BEFORE main runs: (V) d=5 vs Python reference `MP-comm_alg-1/tensor_closure.py` at n=9,11 (s=0..2, arms
C1,D1,A) and brute force n<=13 is already covered by L2 V1 (81/81) + audit cross-checks (11/11 at n=19/21); I rerun
the n=9,11 subset on the copied binary. Any mismatch -> stop.

## Runs (<= 4 threads; actually 2 concurrent single-thread processes)
R1: n=31 k=16 C1 s=1, d=5, CPUCAP=12600 s (3.5 CPU-h).
R2: n=29 k=15 C1 s=0, d=5, calibration (SAT per L1, so must give one=0; gives time/size scaling), CPUCAP=3600 s.

## Predictions
P1: R1 terminates within cap with 1 in W_5 (d_F = 5 exactly for C1 s=1), prob 0.65. Basis: D1 s=25 reached 1
    at d=5; treatise d_F=5 data. Alternative: terminates with one=0 (d_F>=6), prob 0.15; cap hit, prob 0.2.
P2: R2 terminates with one=0 (SAT sanity), prob 0.99; CPU 0.3-0.6x of R1.
Capped records are reported as "inconclusive: cap hit at X s, queue qh/qt", never as "1 not in W_5".
