# Protocol amendment 1: the F4 budget bug in the harness, and a re-run of every card

Frozen 2026-10-27, after cards L1–L4 and before any re-run. It changes nothing
in `PROTOCOL.md` except what is stated here.

## What happened

`examples/postgb_la_bench.rs` built every system's `F4Options` together with
the corpus, before the first measurement. `F4Options::with_budget` stores an
absolute deadline, so any system reached more than its budget after the card
started ran F4 past that deadline. F4 timed out at once, and the bench skipped
the system as "F4 not usable".

| card | skipped by the bug | budget |
|:--|:--|--:|
| L3 | `n3-d10-p32-1`, `n3-d10-p32-2` | 600 s |
| L4 | all nine `n6-d3` systems | 600 s |

L1 and L2 skipped nothing. The arms (`dense`, `sparse`, `fast`) and their
timings are not affected. What the bug changed is which systems were measured.

## Change

- **The fix.** Each system's options are now built just before its F4 call,
  so its budget starts then (`opts: fn() -> F4Options`). A skipped system is
  written to the run JSON (`skipped`, with the reason) as well as to stderr.
  No arm's code changed.
- **The new binary.** `postgb_la_bench` is rebuilt and the cards are re-pinned
  to it. `jv_cover` and the library are unchanged, so the binary hash of
  `jv_cover` is the same.
- **The re-run.** All ten cards are re-run under new ids,
  `CARD-postgb-la-20261026-R2-<NAME>`, writing `runs/R2-<NAME>.json`. Every
  reported number comes from the R2 runs, all on one binary.
- **The first runs.** The first L1–L4 runs are kept as committed and are
  marked superseded. They are not pooled with R2.
- **The comparison.** The README compares L1–L4 with R2-L1–R2-L4 on the
  systems both measured, as a check of run-to-run variation. That comparison is
  not a result.

## Unchanged

Arms, corpus generators and seeds, rounds, measures, predictions P1–P5, the
decision rule, and the dense-arm cap (δ ≤ 1024).
