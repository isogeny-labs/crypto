# The hash-table floor of the prime IC relation trial: a null result (2026-10-29)

Lead `ICP-prime-ic-constant-engineering`, stacked on PR #2783 (the code this tests, `PairTable::touch`,
`WalkPrimeLanes` and the lane walk, is on that stack and not yet on `main`). Report: [`REPORT.pdf`](REPORT.pdf)
(source `report.html`, figure `table-floor.svg` / `table-floor.png`), raw output `micro-results.txt`.

## Question

`research/ic_wide_ext_20261028` timed the target-guard insert and the oracle probe at about 137 ns of a ~318 ns trial
(16-32 lanes) and called it the floor, with no handle for software prefetch because the pair table is a std hash map.
Can an identical-output replacement (one open-addressed region of 64-bit words for the pair table, an open-addressed set
for the guard, real prefetches for a lane round) lower it?

## Result

Timed in isolation on the real tables of the tuned bases (24/28/32-bit curves, bases 256/512/1024; median of seven
passes over one million uniformly random keys, so almost every probe misses):

| | 24-bit | 28-bit | 32-bit |
|---|---|---|---|
| pair probe, miss: std map / flat word table (ns) | 10.7 / 24.7 | 13.4 / 39.0 | 22.4 / 60.7 |
| the same with the lane round's touch / prefetch (ns) | 14.5 / 22.0 | 17.2 / 24.9 | 29.3 / 36.9 |
| guard insert, map + touch / flat + prefetch (ns) | 46.1 / 38.4 | 33.8 / 29.1 | 43.0 / 36.6 |

- The flat pair table is slower on misses by 2.3-2.9x, and still 1.3-1.5x with the prefetch.
- The open-addressed guard saves 5-8 ns of insert time, about 2% of a 318 ns trial.
- The two operations together are 51-72 ns in isolation against 137-140 ns in the diagnosis; the diagnosis times each
  stage of the loop with clock reads, and the gap is consistent with that cost, not with the tables (not separated here).

So the table-layout item of the lead closes as measured. Nothing in this branch changes a production code path.

## What was not done, and why

The paired old/new ecbench sessions (and so a replay certificate, validator report and `VALIDATION.json`) were not run:
`/tmp/crypto-bench.lock` was held for the whole session by another job's multi-hour sessions, and the sessions started
here were refused ("another benchmark holds the lock"). The lock was not bypassed. The micro-benchmark numbers are
descriptive, from a shared machine, and are not a gate-valid row: no `benchmarks/` entry and no leaderboard line are added.

## What is in the branch

`ic_boundary.rs` gains test-only code: `FlatSet`, `PackedPairTable::prefetch`, a helper that builds the batched prime table
as one packed region, tests that compare it with the map form on every stored key, random keys and the negation-folded
probe (`flat_batch_table_equals_the_map_form_in_every_answer`, `flat_set_answers_like_a_hash_map_through_growth`), and
the `#[ignore]` timing test `micro_table_floor`. The one production change is a behaviour-neutral parameter on
`PackedPairTable` construction (`build_regions`, the region size).

Reproduce the timing: `cargo test --release --lib micro_table_floor -- --ignored --nocapture`.
