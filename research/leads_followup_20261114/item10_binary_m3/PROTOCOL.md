# Item 10 PROTOCOL: binary m = 3 (S_4 descended) last fall degree at k = 6 (and k = 7 if feasible)

Frozen 2026-11-14 before any LFD run on the k = 6 cells (instances were built first to read |Z| only;
the builder run printed n, |Z|, equation count; no closure was run on them).

Question: does d_lf stay at 7 (= Kousidis-Wiemers m^2 - m + 1 for m = 3) as k grows past 5?

## Instances (builder: tools/build_m3.py = csd.ic_dreg.binary_descent / binary_solutions, same calls as
csd.ic_bounds_binary.m3_direct_system; three copies of V; variables 3k; n Boolean equations of degree 6)
Instance rule: field F_(2^n) with csd.ic_research_campaign.field(n); curve y^2+xy=x^3+a x^2+b with (a,b) = (1,1)
(Koblitz; the curve of every existing k = 5 row); V = seeded random non-Frobenius-invariant k-subspace,
seed = sha256("20261114:<name>"); target = seeded random rational point (x != 0). Seed 20261114.
Cells (spec files spec_control.json, spec_k6.json):
* CONTROL ctl_k5_n7: existing results-m3-direct.jsonl row d5137c4cd40e97581a46 (n=7, k=5, basis [64,18,8,6,1],
  target (21,90)); must rebuild with repo system_hash fb84b4078c0a9817... and |Z| = 457, and the engine
  must return LFD 7, certificate degree 7, quotient ranks [0,0,0,0,0,0,7,15927].
* A  k6_n9_koblitz1_rr: k = 6, n = 9 (r - n = 9; same under-determination regime as the n=9, k=5 rows, |Z| order 2^9).
* B  k6_n18_koblitz1_rr: k = 6, n = 18 = 3k (balanced; |Z| small, expected 0 so a unit ideal).
* C  (only if A and B finish with > 2 h to spare, which is not expected): k = 7 (21 variables).

## Engine and certification
scripts/ic_bounds_bin_big_run.py ladder (csd/ic_bounds_bin_big.py, bit-packed GF(2), cpu, 1-2 threads, ulimit -v 6 GB),
D = 0..7. Unit ideal: LFD = least D with 1 in V_D. Proper ideal: exact LFD = last strict fall at the first D* where
evaluation rank of M_<D* on Z = |Z| = codim V_D* (the engine's local certificate). Every closure is certified closed by
the engine's full sweep (closed = true, repairs counted).
If A is not certified by D = 7 (codim V_7 != |Z|), degree 8 (106,762 columns) runs only if its projected time
(from the measured D = 7 time, scaling by the engine's flop model, 4.9x update flops) fits the 2.5 h rule.

## Predictions (frozen)
* CONTROL: LFD 7, certificate degree 7 (reproduction of the stored value; falsified by anything else, in which
  case the engine or builder is not comparable and all k = 6 numbers are void).
* A: proper ideal, first strict fall at D = 7, certified at D* = 7 with codim V_7 = |Z| (688); LFD = 7.
* B: unit ideal, 1 in V_D first at D = 7 (LFD 7). (Existing unit-ideal rows reach 7 at k = 3..5; B has |Z| = 0.)
Falsifiers of "d_lf stays at 7": any cell with exact LFD >= 8 or <= 6 at k = 6. A single cell at 7 supports but does
not prove the pattern (two cells, one curve, one basis each: a pattern, not a plateau claim).
Not measured: any ECDLP running time or speedup; no leaderboard or benchmarks/README row.
