# prime_fast ic_solve (summands 3) vs prime_fast rho_parallel: scaling probe

Status: [pcc:unverified] producer's own exploratory probe (not a frozen protocol, no holdout split, constants c in {0.25, 0.5, 1.0} not tuned per size, not independently replayed).

Engine: `examples/prime_fast_vs_rho.rs`, same native Montgomery arithmetic for both arms; rho = `rho_parallel` with 1 thread and negation map; IC = `ic_solve` summands 3, +6 extra relations, 1 thread. 10 planted targets per curve (a = -3 curves, p ≡ 3 and 1 mod 4); every solve verified (k = planted). Wall time per solve, median over targets, best c per row.

| bits | res | IC ms | rho ms | IC/rho | rho steps (median) |
|---|---|---|---|---|---|
| 24 | 3 | 12.4 | 2.45 | 5.1 | 3136 |
| 24 | 1 | 14.8 | 2.79 | 5.3 | 5040 |
| 28 | 3 | 100.7 | 3.43 | 29 | 12000 |
| 28 | 1 | 95.2 | 3.44 | 28 | 16425 |
| 32 | 3 | 1716 | 6.68 | 257 | 61698 |
| 32 | 1 | 1630 | 7.85 | 208 | 98988 |

Reading: the single-target gap to rho widens by about 5x to 9x per 4 bits (IC time grows about 8x then 17x per 4 bits, rho about 1.4x to 2x), the direction the Gaudry-Diem barrier predicts; no crossover is approached. Distance remaining is therefore larger than the CLI-stack figure (22x to 49x) at 28 bits and above. Limits: IC at 32 bits was not re-tuned (best c was 0.25 to 0.5, edge of grid), rho is the DP implementation with start-up costs ignored at these sizes, no §5a pricing band applied (rho hash/DP bookkeeping unpriced), toy sizes.

Next discriminating experiment: amortize relation collection across many targets on `prime_fast` (shared factor-base logs, one linear system, then one cheap extraction per target) and compare per-target cost against the same rho for T targets, under a frozen protocol.

Correction (validator, `results/pf_multi/VALIDATOR_REPORT.md`): `rho_parallel` builds a 1024-entry table per solve (1.3 to 2.9 ms), which dominates the small-size rho times above. The IC/rho ratios at 24 and 28 bits are therefore overstated; the validator measured 4.4x to 9.5x advantage corrections for the multi-target case at 23 to 26 bits. Re-measure with a right-sized table before citing.
