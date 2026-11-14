# Pilot (master seed 777, not the data seed 20261114), [pcc:unverified]

Local development pilot, 4 vCPU Intel Xeon 2.30 GHz, 15 GB RAM, msolve 0.6.5 (`/usr/bin/msolve`). Excluded from the analysis.
Instrument binary `target/release/examples/pkm_m8_panel` sha256 `b83a2f19d3cd7b2874c4da3e419218bf1b2f82b3a8c34ad58894f0b84354545f`.
Raw logs: `smoke/pilot-t2r8-s777/runs/`.

## Finding 1: every prime below 2^18 crashes msolve 0.6.5 on this system
`t2r8` under the rule is p = 65557 (65537 is Fermat and excluded). The call died at once (0.00 s, rc 139, max RSS 5 MB) on the
SAT and on the UNSAT target. Probe (same pilot cell rule, one target each, `msolve -t 1 -P 1`, `prlimit --as=8G`, 20 s):

| least eligible prime >= | p | msolve |
|---|---|---|
| 2^16 | 65557 | rc 139 immediately |
| 2^17+1 | 131101 | rc 139 immediately |
| 2^18+1 | 262153 | runs (timeout 20 s, no crash) |
| 2^19+1 | 524309 | runs |
| 2^20+1 | 1048589 | runs |

By the frozen crash rule the t2r8 cell is replaced by the next eligible prime; the replacement is **p = 262153, which is the t2r9 prime
range (>= 2^18)**. The ratios 8 and 9 would then give the same prime. The protocol needs a decision (e.g. lower bound 2^18 for every cell,
t2r8 renamed, or t2r8 dropped); `cell --min-prime N` is the instrument hook.

## Pilot cell
`pkm_m8_panel cell --t 2 --ratio 8 --master-seed 777 --min-prime 262145 --out smoke/pilot-t2r8-s777/cell`:
p = 262153, a = 192130, b = 237762, r = 262051 (prime), g = 2, zeta = 174598, B = 4, B_real = 3 (cell sha256 `d4e10674...b10a`).
`targets --cell ... --n-sat 1 --n-unsat 1 --planted 1 --master-seed 777`: 471 draws, 1 real-SAT draw (rate 2.1e-3).
`export --cell ... --targets ... --out smoke/pilot-t2r8-s777/export` (22 variables, 23 equations).

| target | label | real decompositions | algebraic multisets | ordered algebraic solutions (native formula) |
|---|---|---|---|---|
| d000000 | natural UNSAT | 0 | 0 | 0 |
| d000470 | natural SAT | 10 | 20 | 84,464 |
| planted0 | planted | 6 | 10 | 39,912 |

## msolve calls (exact command shape; `run.sh` and `run2.sh` are in `runs/`)
```
/usr/bin/time -v timeout 3600 prlimit --as=<cap> msolve -t 4 -v 2 -P 1 -f export/<target>.ms -o <target>.P1.out
```
| call | cap (address space) | outcome | cpu_s (user+sys) | wall | max RSS | last round |
|---|---|---|---|---|---|---|
| d000000 UNSAT | 8 GB | complete, `[-1]:` (no solution) | 1988.3 | 950.9 s (951.1 s by time) | 6.79 GB | 165 rounds, last degree-7 round 5519 pairs, 41487 x 120909 |
| d000470 SAT | 8 GB | killed, rc 139 (SIGSEGV at the cap; not the small-prime crash: it ran 999 s) | 2172.5 | 999.9 s | 6.90 GB | degree-7 round 11116 pairs, 121672 x 238051 |
| d000470 SAT (rerun) | 14 GB | killed, rc 139 (SIGSEGV at the cap) | 5755.9 | 2024.7 s | 12.74 GB | degree-7 round 21905 pairs, 448836 x 684766 |

The SAT call at t = 2 does not complete locally in 14 GB. Both SAT stops are memory-cap stops (lower bounds), not timeouts.
`verify` on the SAT output could not be run for lack of an output. It was run on the UNSAT output (`d000000.verify.json`,
`d000000.check.json`: 0 relations, msolve solution count 0 = native ordered count 0).
Contention note: the dev-check commands of the instrument (small m = 4, 5 runs, rho, the 30 s native run) ran under `nice` while the first two calls were in progress.

## Reachability line (B4)
KILL needs min c_lo(t=3) >= 2^3.5 x max c(t=2) = 11.3 x max c(t=2). With UNSAT completed at 1,988 CPU-s and the SAT t = 2 call already above
5,756 CPU-s (censored), 11.3 x 5,756 = **>= 65,000 CPU-s** for a t = 3 stop, in the 8-thread pod run (about 2.3 h wall at perfect scaling;
the 14,400 s wall cap gives at most 115,200 CPU-s). If the SAT t = 2 call needs more, the t = 3 requirement grows with it. A completed SAT t = 2 call
needs more than 14 GB (RSS 12.7 GB when killed, still in the first degree-7 round); the pod's 256 GB is not the limit there. For scale, the committed D83 run
(t = 3, 8 threads) reached about 40k CPU-s before dying.

## Other development checks (not measurements)
- msolve -g 2 vs native f4 reduced bases, same exporter path, p = 262153: m = 4 and m = 5, one SAT and one UNSAT target each, `compare-gb` equal in all four
  (71 and 54 elements for the SAT systems, `[1]` for the UNSAT ones). m = 8 native run censors at 30 s (degree 8 reached).
- Variety gate on the same systems: msolve distinct-solution count equals the native ordered algebraic count exactly (m = 4 SAT 56 = 56, m = 5 SAT 20 = 20; UNSAT 0 = 0).
- L1 replication (`export --l1-replication`): natively reconstructed cell (a = 11698169, b = 5750081, g = 3871275, zeta = 12665968, x_R = 15707930), 30 variables, 31 equations;
  `msolve -t 1 -v 2 -g 2` rows compared with the committed A3 log: the first 37 F4 rounds (through the degree-8 rounds, up to a 25550 x 27344 matrix) are identical in degree,
  pairs, matrix size, density and new/zero counts (timing columns differ).
- rho reference on the pilot cell, 3 seeds, all `xP = Q` verified: 160, 384, 320 walk steps (prediction 454).
