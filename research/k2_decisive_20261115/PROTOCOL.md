# K2 decisive cells: frozen protocol (2026-11-18)

Extension of `research/k2_flatness_20261115/PROTOCOL.md` (same definitions, same "solved at D", same controls and the
same flat/breaks logic), registered BEFORE any decisive-cell data. All numbers are `[pcc:unverified]`.
Lead: `LEAD-KOB-flat-degree-crossover-bar` (K2). The original study found every chain system solved at D = 4 for
n <= 18, k <= 6 (61/61) and named these cells as the ones where the degree would have to break.

## Instrument

`k2m4r.rs` (standalone, no `src/` edits). System builders, symbolic descent (checked against the word formula on random
points per system), solution enumeration and the dense reference closure `closure_ref` come unchanged from `k2close.rs`.
New: `closure_m4`, the degree-<= D closure by a packed-u64, tile-major, Four-Russians (M4RI-style) GF(2) echelon that
adds products x_i f in batches and keeps only the echelon basis (no dense matrix of the whole closure). Registered
definition kept: **solved at D** iff rank W_D = M(N, <= D) - rank(evaluation vectors of the enumerated solutions)
(1 in W_D for an inconsistent system). `--engine ref` runs the dense original for comparison.

Acceptance of the instrument (before data): `m4` and `ref` must agree on (solved, rank, target rank, cols) for every
(cell, target, D) row of the small original cells (`run_validate.sh`, `compare.py`); the original study's D_sol values
must be reproduced (D_sol = 4 on A, B, C cells, 3 or 4 on P). Any mismatch stops the study.

## Cells (seed 1; curve/subspace/target derived from the seed as in the original)

| cell | m | n | k | N | cols M(N,<=4) | role | targets |
|:--|--:|--:|--:|--:|--:|:--|--:|
| P | 2 | 16, 20 | 8, 10 | 16, 20 | - | positive control (seeds 1, 2, 2 targets) | 2 |
| N | 3 / 4 | 12 / 8 | 4 / 2 | 24 | 12951 | negative control, original shape | 1 |
| N-D1 | 3 | 21 | 7 | 42 | 124314 | negative control of the decisive shape (random, planted solution) | 1 |
| L2 | 3 | 19 | 6 | 37 | 74519 | ladder | 1 |
| L1 | 4 | 13 | 3 | 38 | 82993 | ladder (n = 13 chosen so k = 3 sits one step above the original C range) | 1 |
| **D1** | 3 | 21 | 7 | 42 | 124314 | decisive, m = 3, k = 7 (n = 3k, the smallest n with 3k >= n at k = 7) | 1 (+ second if budget) |
| **D2** | 3 | 24 | 8 | 48 | 213053 | decisive, m = 3, k = 8 | 1 |
| **D3** | 4 | 16 | 4 | 56 | 396607 | decisive, m = 4, k = 4 | 1 |

(Column counts are sums of binomials C(N, i), i <= 4, computed by hand here; the instrument reports its own count and it
governs on any difference.)

D scanned: 3 then 4 for the chain cells (D = 3 fails, as in 0/62 before; stopping at D = 4 solved or stall).
D = 5 is not scanned (not needed for the verdict below; a D = 4 stall is reported as "stall at 4", not as a D_sol).

## Resources, budget, order

Local CPU only if no EKS access (see README "Route"): 4 threads, `ulimit -v 9000000` (9 GB), shared CPU lock.
Budget: 3 CPU-hours = 2700 s wall at 4 threads for the whole grid. Order (run_grid.sh): controls P, N; N-D1; L2; D1;
L1; then D3 and D2 only if the gate holds: projected time T(D1) * (cols/124314)^3 must fit the remaining budget and
the basis (rank x cols / 8 bytes, rank <= cols) must fit 7 GB. By that bound D3 (396607 columns, up to 19.7 GB) can
never run under the 9 GB cap and is registered in advance as **not run (memory)** locally; D2 (up to 5.7 GB) is gated by time. A cell that fails the gate is reported as **not run** with its
projection; a cell stopped by its time cap is **censored** (reported "not reached"). No cell is rerun, resized or
re-seeded after seeing its result. Second targets (D1, L1, L2) only with leftover budget >= 1.2 x the first target time.

## Verdict rule (registered, per decisive cell, over completed targets)

- **flat (at this cell)**: every completed target solves at D <= 4;
- **breaks**: at most half of the completed targets solve at D <= 4 (a D = 4 stall with deficit > 100 and the
  negative control stalling alike is not a "break" if it is censored: censored is "not reached");
- **inconclusive**: anything else;
- **not reached**: not run, gated out, or censored by the time cap.

Overall: "breaks" if any decisive cell breaks; "flat" only if D1, D2 and D3 are all flat; "flat where reached" if
some are flat and the others not reached. Controls must pass first: P solves at D <= 4 on every target; every N cell
stalls at D = 4 with deficit > 100 (a closure that "solves" a structure-free system invalidates the run).

## What the verdict does not mean

A one-target cell is a single draw; "flat at D1" is evidence on one system, not a rate. D_sol = 4 on the chain says
nothing about the direct presentation (degree 6), about cost against rho (per-relation yield 2^(mk-n)/m! unchanged),
or about larger n. Closure degree is a proxy (WEAKNESSES E-GB-064).
