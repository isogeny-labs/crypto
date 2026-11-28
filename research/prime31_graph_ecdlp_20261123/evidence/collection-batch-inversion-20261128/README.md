# Batch inversion, Mersenne reduction and a fast hash in M31 relation collection (2026-11-28)

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

The baseline and all eight lever combinations (0, 1, 2, 3, 12, 13, 23, 123)
were run to the solver's natural stop (40-query cap). Each gives the same
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
| **1+2+3** | **1,283 ms** | **687 ms** | 527 ms | **0.253 G** | **276** |

The table comes from `timing-runs.json` and `CALLGRIND.txt`. There were 7
rounds, with the arms interleaved within each round and every run a
separate cold process.

**Charged work over the whole solve:**
- Field inversions fall from 10,986,799 to 918,466. What remains is the pair-table build and the scalar multiplications, which still use the baseline addition.
- Field multiplications fall from 703.2 M to 119.2 M.
- Group additions are unchanged at 10,987,529.

**Overall:** about 4.0x on the cold solve, 6.6x on collection and 7.5x fewer
instructions per query. Lever 2 does nothing by itself, because it applies
only inside the batched path. Lever 3 helps more once batching has removed
the inversions.

## What dominates now, and context

- **The pair table.** Its build (915,304 additions, each still with a Fermat
  inversion) is 527 ms, 41 % of the new cold wall. The same batching applies
  to it; that is the next lever, not done here.
- **Matched-target rho.** `prime31-graph-rho-control` is an independent
  unfolded Brent rho on the same target with the baseline arithmetic. It
  recovers `987654321` in 98,366 steps and 43–45 ms (`rho-control-runs.json`;
  one seed, one target, so the step count is highly variable).
- **Not a route past rho.** Even with all three levers the graph-control
  solve is about 29x rho's time. These levers speed up this solver; they do
  not change the conclusion that this graph-control collection sits far
  above rho.
- **Single target.** These are timing repeats of one fixed target and seed,
  with randomized std hashing in the baseline arms. Isolated timing and a
  target panel were not run.
