# Item 10 RESULTS: binary m = 3 last fall degree at k = 6   [pcc:unverified] (all results below)

Date 2026-11-14. Protocol: PROTOCOL.md (frozen before the k = 6 ladder runs; not amended).
Repos: ml-cryptanalysis 27a67b9 (engine csd/ic_bounds_bin_big.py sha256 3ba6e423cbe04c8cfb70fef59d3584dd52d4fb92363cc4b94811525a19f1aa4b,
driver scripts/ic_bounds_bin_big_run.py), crypto 44d35033fa, cryptanalysis fb3f88a0 (not used).
Host: 4 vCPU shared, cpu device, OMP/OPENBLAS threads 2, `ulimit -v 6000000`, one run at a time (run_k6.sh, PID 10285, exited rc 0,
status in out/status.txt, ends with ALLDONE; no orphan left).

## Commands
    python3 tools/build_m3.py spec_control.json inst ; python3 tools/build_m3.py spec_k6.json inst
    python3 /home/user/ml-cryptanalysis/scripts/ic_bounds_bin_big_run.py ladder --instance inst/<cell>.json --out out/<cell> \
        --max-degree {8 control | 7 k=6} --work-gb {2|1} --tile-mb 256 --batch 512

## Results (measured by the engine; exact certified LFD)
| cell | n | k | vars | equations | |Z| | status | exact LFD | certificate | quotient ranks (D=0..7) | closure s | peak RSS MB |
|---|---|---|---|---|---|---|---|---|---|---|---|
| CONTROL ctl_k5_n7 (stored row d5137c4cd40e97581a46) | 7 | 5 | 15 | 7 | 457 | exact-proper-local | **7** | D*=7, codim 457 = eval rank 457 | 0,0,0,0,0,0,7,15927 | 83 | 1168 |
| A k6_n9_koblitz1_rr | 9 | 6 | 18 | 9 | 688 | exact-proper-local | **7** | D*=7, codim 688 = eval rank 688 | 0,0,0,0,0,0,9,62316 | 2215 | 1480 |
| B k6_n18_koblitz1_rr | 18 | 6 | 18 | 18 | 0 | exact-unit | **7** (1 in V_7, V_7 = all 63,004 columns) | D=7 | 0,0,0,0,0,0,18,63004 | 1527 | 1485 |

* Control reproduces the stored value: system_hash (repo kind binary-m3-direct) fb84b4078c0a9817... equals the stored row's,
  |Z| = 457, LFD 7, certificate degree 7, quotient ranks identical to the stored `quotient_ranks`
  [0,0,0,0,0,0,7,15927] and codim 457 (stored 457). Engine time 83 s vs stored 335 s with the older engine (different engine, no speed claim).
* A: the first strict fall is at D = 7 (rank of V_6 mod field equations 0, then 9 = n equations appear at D = 7 on the 31,180-column
  degree-6 layer; fall list [7]); local criterion met at D = 7. Local checks (D: eval rank / codim): 1: 1/19, 2: 18/172, 3: 154/988,
  4: 618/4048, 5: 667/12616, 6: 685/31171, 7: 688/688.
* B: balanced n = 3k, |Z| = 0, so a unit ideal; 1 first lies in V_7 (V_6 has rank 18 = n, V_7 is everything). LFD = 7.
* Every closure: closed = true after one clean full sweep, repairs 0 (closed_all true in the summary JSON).
* Predictions of PROTOCOL.md: CONTROL 7/7 confirmed; A (7, certificate 7) confirmed; B (unit, 7) confirmed. No falsifier triggered.

## Answer to the question
At k = 6 (18 Boolean variables, 63,004 columns at D = 7) the exact LFD is 7 in both cells, for a proper ideal (n = 9, |Z| = 688)
and a unit ideal (n = 18 = 3k). So d_lf stays at 7 = m^2 - m + 1 (m = 3) from k = 3 through k = 6 on these instances;
the F_2-Macaulay kernel bound r + 1 = 19 is far above. This is one curve (Koblitz (1,1)), one random basis per n, one target per
cell: an extension of the existing pattern (which had LFD 7 at k = 3, 4, 5), not a proof or a plateau claim.
Degree 8 was not needed (both cells certified at D = 7), so it was not run.

## Dropped / not done
* k = 7 (21 variables, 198,440 columns at D = 7): not built into the closure and not run. The engine's flop model (plan r = 21, D = 7:
  update 2.60e15, residual bound 5.18e15; k = 6 model 8.34e13 / 1.93e14 took 2215 s measured, actual flops 9.3e13 + 2.7e13) gives a
  projected 30x or more, roughly 18 h or more, on this host: beyond the 2.5 h rule, and the two k = 6 cells already used 1.1 h of it.
  Exact next command (needs a machine with ~20 h or the isolab-eks route, not authorized here; build first):
      python3 tools/build_m3.py spec_k7.json inst   # spec: n=21 (balanced) or n=12, k=7, a=1,b=1, seed 20261114
      python3 /home/user/ml-cryptanalysis/scripts/ic_bounds_bin_big_run.py ladder --instance inst/<k7cell>.json --out out/<k7cell> --max-degree 7
  Predicted peak RSS about 3.5 GB (plan), Z enumeration over V^3 = 2^21 points takes about a minute in Python.
* Not measured: ECDLP running time, speedup, or any comparison with rho. No leaderboard / benchmarks row.

## Files (sha256 of the summary JSONs)
inst/ctl_k5_n7.json 7164ac33...; inst/k6_n9_koblitz1_rr.json b1b2881a...; inst/k6_n18_koblitz1_rr.json 16bb03e6... (build outputs, full hashes by `sha256sum inst/*.json`).
out/ctl_k5_n7 b1715b74f16beca3...; out/k6_n9_koblitz1_rr cfe141fc3ad3f86b...; out/k6_n18_koblitz1_rr 8d13c112f632c16d... (ladder summaries; per-degree out/<cell>.D<d>.json, logs out/*.log).
Tools: tools/build_m3.py, run_k6.sh, spec_control.json, spec_k6.json.
