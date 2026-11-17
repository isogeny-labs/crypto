# RESULTS: d_NS at m = 3, Kummer towers (2026-11-15) [pcc:unverified]

PROTOCOL.md sha256 (frozen before data): b790f8bde4fd1991a2051ac42b0a5b71e534b1bac9db3381a6394dd53c47857f

All numbers are unverified assertions from `ns_degree.py` / `run_all.py` / `extra_*.py` (no replay certificate, no pcc claim id).
Measured deg F1 = 4, deg F2 = 3 on every instance (popcount degree from inverse DFT; agrees with the symbolic expansion at t=2).

## Table (per (p,t): 3 instances each, all instances gave identical values)
| t | 3t+5 | d_NS(E=1) exact | d_NS(E>=2) | m=2 control d_NS (= 2+popdeg f^-1) | 2t+2 |
|---|------|-----------------|------------|-------------------------------------|------|
| 2 | 11 | 11 (6/6) | 10 (6/6; E=2..8, full u-range) | 6 (6/6) | 6 |
| 3 | 14 | 14 (6/6) | 13 (6/6 at E=2; E=3,4 on 1 inst/prime: 13) | 8 (6/6) | 8 |
| 4 | 17 | 17 (6/6) | not computed (too large) | 10 (6/6) | 10 |
| 5 | 20 | 20 (6/6) | not computed | 12 (6/6) | 12 |
E = cap on u-degree of the multiplier mu. E=1 is exact by the unique pointwise Bezout solve (t=2,3 also by rank method, equal).

## H0 outcome
H0 (d_NS = 3t+5 for all instances) did NOT hold: at t=2 (E>=2, which is the full definition since E up to 8 >= c - deg F) d_NS = 10 = 3t+4, and at t=3 with E = 2,3,4 d_NS = 13 = 3t+4 (cap-restricted upper bound on the true d_NS; true d_NS <= 13). With u-degree <= 1 multipliers (the D9 certificate class) d_NS = 3t+5 exactly at t = 2..5. At t = 4, 5 only E=1 was computed.

## Controls
- C1 (m=2): rank-method d_NS = 2 + popdeg(f^-1) = 2t+2 on all 24 instances (t=2..5, both primes). Passed.
- C2 (planted common root, both primes): gate flags every planted instance (1..24 singular grid points); for t=2 (E=8, c=13 and E=2, c=11) and t=3 (E=2, c=14) 1 is NOT in the span; t=4,5 the E=1 solver reports a singular point. Passed.
- C3: explicit group-algebra product (t=2, monomial z1 z2^2 z3^3, F1 and F2) equals the pointwise column: True, True. Passed.

## Commands
```
export OMP_NUM_THREADS=1
python3 run_all.py controls; python3 run_all.py m3_small; python3 run_all.py m3_t3; python3 run_all.py m3_big
python3 extra_t2.py   # E sweep 1..8 at t=2 and planted E=8
N1_RAW=<scratch>/raw python3 extra_t3.py   # E=3,4 at t=3, instance 0 of each prime
```
Runtime: controls ~92 s; m3_small ~5 s; t=2..5 E=1 ~10 s; m3_t3 (E=1,2) ~9 min; extra_t3 (E=3,4) ~7 min; 1 core each, 2 processes at a time, < 1 GB.

## Deviations from protocol
- E cap as preregistered (E=1 at t=4,5; E<=4 at t=3). t=4 with E=2 was not attempted (about 20k x 24k dense system).
- Descending search from the known E=1 degree (monotone in c) instead of ascending from c=4.
- extra_t3 and the sweep outputs were written under the scratchpad during the run, then copied here; run_all.py reads N1_RAW for the output dir.
- Optional Rust d_F run: not attempted. No matplotlib; figures/ns_degree.svg is hand-written SVG.
- Session-log/Conductor steps of CLAUDE.md not done (lab restricted to this directory).

## What is and is not shown
Shown (computationally, small instances): at t=2 and t=3 there are Nullstellensatz certificates of degree 3t+4 using multipliers of u-degree 2, so the u-degree <= 1 certificate of degree 3t+5 is not minimal there; with u-degree <= 1 it is exactly 3t+5 for t=2..5.
Not shown: d_NS(full) at t >= 3 (only upper bound 13 at t=3); d_NS for E>=2 at t >= 4; any pattern in t beyond two points (3t+4 vs another form); true d_F; anything about other primes or non-Kummer towers.
