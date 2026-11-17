# Amendments (the frozen rule in PROTOCOL.md, sha256 cd7ea967…, is unchanged)

## A1 — 2026-11-13 ~00:35Z: RunPod pod for memory-bound cells
Reason: the coordinator relayed the user's request to run on RunPod (pod bdr-chase-big, L1 share
24 threads / 200 GB, msolve 0.6.5-1build2 from Ubuntu 24.04). The local 3 GB cap censors the
decisive cells. Added cells/instruments (no change to the decision rule, which is applied to
the certified closure D exactly as frozen; msolve's F4 degree is reported alongside as an upper
bound on the PC degree and for comparability with yesterday's preset):
- Instrument 2: msolve 0.6.5 `-t T -v 2 -g 2` (T ≤ 24 total over concurrent jobs) on the raw
  presentation (tower equations y_{i,j+1} − y_{i,j}², 1 − y_{i,t−1}², Semaev S3 chain in the
  y_{i,0} with x_i = g·y_{i,0}), the same layout as the committed repo systems.
  D_msolve = max degree of a reduction round in the -v 2 trace up to the round producing 1.
- Instrument 1 (closure) on the pod with a larger entry cap (per-process `prlimit --as` set so the
  sum over concurrent jobs ≤ 200 GB).
- Cells added on the pod: m=4 t=4,5 (N=16,20); m=5 t=3; m=6 t=3; m=7 t=2; m=8 t=1,2,3;
  m=2 t=13,14 at P1 = 2013265921 with the committed repo curve (gen/m2_t13.jsonl curve; random
  x_R; consistency of m=2 targets at P1 is NOT enumerated — read off the output: inconsistent
  iff the reduced basis is {1}).
- Pod caps: 3 h wall per command; whole pod ~6 h.
Results from the pod are reported both under the frozen rule and as amendment rows.

## A2 — written 2026-11-13 01:37Z: added cells and an upper-bound certificate. Disclosure: written AFTER the m8 t2 msolve result (D_F4 = 8) and the m5 t3, m7 t2, m8 t2 closure lower bounds had been seen; the p and diagnostic cells were chosen at ~00:40–01:10Z, before their results
- Upper bound from msolve: a completed msolve F4 run whose maximal round degree is D_F4 and whose
  basis is {1} is itself a PC derivation of degree ≤ D_F4 (every row of a degree-d round is a
  multiple of an input/basis element of degree ≤ d, reduced within degree d; raw-PC = NF-PC for
  D ≥ 4). So D_PC ≤ D_F4. A cell whose closure certifies 1 ∉ W_{D−1} and whose msolve run finished
  at D_F4 = D is reported as exact ("closure-lb + msolve-ub"). This only tightens bounds; the
  frozen rule is applied unchanged to the resulting values.
- p = 16801793 (2^13 | p−1, prime-order curve) for m4 t5 and m8 t3: at p = 40961 random targets
  for these cells are almost all decomposable (≈ multisets·signs > p), so an inconsistent random
  target needs a larger p (D was observed p-independent; not re-tested at this p).
- Diagnostic cells for the theory part (not under the decision rule): restricted systems with
  m−1 blocks fixed (restr_*) and with m−2 blocks fixed (restr2_*), random seeds, p = 40961.
- Independent dense re-implementation (scripts/indep_closure.py) on 6 tiny cells.
- Pod memory caps rebalanced live (prlimit --pid; total ≤ 200 GB); a duplicate m4 t4 msolve at
  30 GB because the original 8 GB cap could not be raised.

## A3 — 2026-11-13T02:34:38Z, before any run of these cells: the decisive cell D(8,3)
Requested by the coordinator. D(8,3) (m = 8, t = 3, p = 16801793, seed 1; current certified
bound D ≥ 9 from a complete level-8 closure) is named the decisive cell for m = 8. Reading fixed
in advance:
- D(8,3) = 9 (closure finds 1 ∈ W_9, or msolve finishes at D_F4 = 9 with basis {1}): m = 8
  SURVIVES at toy scale (σ = 1/8 over t = 1→2→3; E ≈ 0.43 under w = 1; E ≥ 1 under w = 2).
- D(8,3) ≥ 10 (level-9 closure complete without 1): KILL for m = 8 (σ ≥ 1/4 > α*(8,1) = 0.156).
- Neither completes: INCONCLUSIVE, reported with the bound D ≥ 9.
Runs: exact closure at degree 9 on m8t3 seed 1, plus a second random target (seed 2) if memory
allows, otherwise a planted (consistent) m8t3 control, which must never give 1. msolve to
completion on m8t3 seed 1. Then, if time remains, D(4,4). The frozen PROTOCOL.md rule is unchanged;
this reading is an m = 8 specialization of it (with only t = 2, 3 exact at t ≥ 2 the frozen
rule's interval reading is reported alongside).
- 2026-11-13T02:42:28Z: memory does not allow two full level-9 closures (≈ 100 GB each expected); per A3 the second slot runs the planted m8t3 control (33 GB cap, may be censored). The seed-2 random target is being generated locally and is NOT run unless memory frees. D(4,4) closure runs alongside (10 GB).
