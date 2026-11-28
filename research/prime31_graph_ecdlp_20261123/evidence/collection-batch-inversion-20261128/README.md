# Batch inversion, Mersenne reduction and a fast hash in the M31 graph-control solve (2026-11-28)

[pcc:unverified]

**Change.** A new candidate binary `prime31-graph-mitm-fast` (`../../native/mitm_four_fast.rs`).
`mitm_four.rs` is unchanged and is the baseline. The candidate runs the same
fixture, pair table, queries, graph rank and stopping rule. It changes only
how each query's 915,304 probe points `r - left.sum` are computed, and how
the public pair table is hashed. Its levers, set by the second argument:

1. **batch inversion.** Montgomery's trick over one query's probes gives one
   field inversion per query instead of one per probe. The special cases
   (identity left, cancelling points, doubling) follow `Arithmetic::add`
   exactly.
2. **Mersenne reduction** for p = 2^31 - 1 inside that path, plus branch
   subtraction instead of `% p`.
3. **a multiply-rotate hasher** (FxHash constant) for the pair table, whose
   keys are public curve points.
4. **batch inversion for the pair-table build.** The four sign combinations
   `±P_i ± P_j` of a pair share the slope denominator `x_j - x_i`. That gives
   one prefix-product slot per pair and one inversion for the whole table.
   The sums come out in the baseline's loop order, so `pairs` and the index
   are laid out identically; a pair with equal x would use `Arithmetic::add`.

Charged work counts what the path does: one group addition per probe, one
inversion per batch, and every multiplication.

**Build and provenance.** On `main`, the producer harness does not build.
Its `f4-source` dependency pins SHA-256 hashes of production files, and
`src/cryptanalysis/glv_invariant_base.rs` has changed since the pin. The
binaries never use that crate, but Cargo builds it.

All runs here therefore use the `577d6c2c` export of the producer and its
dependencies (`../../../lp_graph_algorithms_20261123/evidence/m31-mst-cold-share-20261128/`).
In that tree I placed `main`'s `mitm_four.rs`, which equals the frozen file
plus two output fields, together with `mitm_four_fast.rs` and `main`'s
`rho_control.rs`. The build was `cargo build --locked --release`, rustc 1.97.0,
on a shared 4-vCPU x86_64 container.

## Exactness

The baseline and sixteen lever combinations (0, 1, 2, 3, 12, 13, 23, 123,
4, 14, 24, 34, 124, 134, 234, 1234) were run to the solver's natural stop (40-query cap). Each gives the same
707 relation rows, per-query classes and ranks, probe count, and recovered
scalar `987654321` after 11 queries. The normalised outputs have one
SHA-256 (`EXACTNESS.txt`).

## Cost

| arm | cold wall, median of 7 | collection | pair table | instructions per query | per probe |
|---|---:|---:|---:|---:|---:|
| baseline | 5,101 ms | 4,528 ms | 508 ms | 1.898 G | 2,073 |
| 3 (hash only) | 5,000 ms | 4,465 ms | 496 ms | — | — |
| 1 | 1,780 ms | 1,181 ms | 529 ms | 0.582 G | 636 |
| 1+2 | 1,638 ms | 1,026 ms | 539 ms | 0.500 G | 545 |
| 1+3 | 1,389 ms | 808 ms | 537 ms | 0.335 G | 366 |
| 1+2+3 | 1,283 ms | 687 ms | 527 ms | 0.253 G | 276 |

**With the table batched** (`timing-runs-table.json`: a second set of 7
interleaved rounds with the 4-lever build; the fixed part is instructions
for fixture, table and setup, `Ir[q=1]` minus per query):

| arm | cold wall, median of 7 | collection | pair table | fixed part | per query |
|---|---:|---:|---:|---:|---:|
| baseline | 5,123 ms | 4,510 ms | 538 ms | 2.326 G | 1.898 G |
| 1+2+3 | 1,272 ms | 693 ms | 499 ms | 2.067 G | 0.253 G |
| 1+3+4 | 1,126 ms | 829 ms | 237 ms | — | — |
| **1+2+3+4** | **948 ms** | **683 ms** | **206 ms** | **0.620 G** | **0.253 G** |

The table comes from `timing-runs.json` and `CALLGRIND.txt`. There were 7
rounds, with the arms interleaved within each round and every run a
separate cold process.

**Charged work over the whole solve:**
- Field inversions fall from 10,986,799 to 918,466 with levers 1+2+3, and to 3,163 with all four. What remains is the scalar multiplications and replay, which use the baseline addition.
- Field multiplications fall from 703.2 M to 119.2 M with levers 1+2+3, and to 64.0 M with all four.
- Group additions are unchanged at 10,987,529.

**Overall, with all four levers:** about 5.4x on the cold solve, 6.6x on
collection, 7.5x fewer instructions per query, and 3.8x fewer in the fixed part. Lever 2 does nothing by itself, because it applies
only inside the batched path. Lever 3 helps more once batching has removed
the inversions.

## Memory

Peak RSS (`peak-rss.json`, `VmHWM` polled every 5 ms, three runs each, 40-query cap):

| arm | peak RSS |
|---|---:|
| baseline | 129.7–130.0 MB |
| 1+2+3 | 172.0 MB (+42 MB) |
| 1+3+4 | 193.5–193.6 MB |
| 1+2+3+4 | 193.4–193.5 MB (+64 MB) |

The batch buffers are a real cost and are not free. The per-query probe
buffers (`kind`, `num`, `den`, `prefix` and 24-byte output points for
915,304 probes) account for most of the first increase. The table-build
buffers and 915,304 table sums account for the second.

## Timing records

Both timing files carry a `round` index (v2). The arms ran in a fixed order
within each round, not counterbalanced, and there is no A/A arm. The cold
clock is the producer's in-process `Instant`; it excludes process start-up
and final JSON printing. For levers 1234, cold wall minus collection minus
table is 54–68 ms per run: fixture, setup, graph/rank/replay and other work
outside those two stage clocks, not separately instrumented here.

## What dominates now, and context

- **Collection again.** With the table batched (206 ms, now mostly hashing
  and inserting), collection is 72 % of the 948 ms solve. Its 276
  instructions per probe include about 6 field multiplications. How the
  rest splits between lookup and loop was not profiled for this arm.
- **Matched-target rho.** `prime31-graph-rho-control` is an independent
  unfolded Brent rho on the same target with the baseline arithmetic. It
  recovers `987654321` in 98,366 steps and 43–45 ms (`rho-control-runs.json`;
  one seed, one target, so the step count is highly variable).
- **Not a route past rho.** Even with all four levers the graph-control
  solve (948 ms) is about 21x rho's time. These levers speed up this solver; they do
  not change the conclusion that this graph-control collection sits far
  above rho.
- **Single target.** These are timing repeats of one fixed target and seed,
  with randomized std hashing in the baseline arms. Isolated timing and a
  target panel were not run.
