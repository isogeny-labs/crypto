# f4_gf2 port: same-binary comparison (2026-10-18)

Engineering result, kernel level only: the sibling suite's rank-indexed
Boolean F4 kernel (`src/cryptanalysis/f4_gf2.rs`) against this crate's
`matrix_f4_f2` family, one test binary, 20 draws of the Koblitz `m = 3`
cell n = 15, ell = 5 (30 variables), Macaulay degree 4, equal decisions on
every draw. No end-to-end `S` claim, no rho comparison.

Source of truth: [`docs/ic/perf/F4_GF2_PORT_COMPARISON_20261018.md`](../../docs/ic/perf/F4_GF2_PORT_COMPARISON_20261018.md)
(commands, per-draw table, host, caveats). Host: 4-vCPU KVM guest, Xeon
2.80 GHz, no PMU, shared (L0).

| arm | median ms (20 draws) | min | max | median speedup vs crate full | median word XORs |
|:--|--:|--:|--:|--:|--:|
| crate full (`matrix_f4_f2_counted_with`) | 199.5 | 175.0 | 253.1 | 1.00x | 154 963 841 |
| crate solver (engine's call) | 396.7 | 368.9 | 420.7 | 0.50x | 64 342 540 |
| port F5 on | 35.6 | 33.5 | 55.7 | 5.61x | 57 472 744 |
| port F5 off | 40.1 | 37.5 | 85.5 | 4.98x | 68 746 716 |

Four full runs: port F5 on is 5.6-6.0x faster than the full-readback path
and 10.4-11.2x faster than the solver path. The word-XOR counts are within
1.1x of each other: the gain is the build and readback, not the elimination.
No figures or graphs were produced for this entry (a table-only result).
