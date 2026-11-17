# Amendment 1 (2026-10-11, before any decision was read): the tower engine's count depended on the thread count

## What happened

The first measured attempt ran from commit `280c5ad52` (binary sha256
`7501c98d7f8b30c2…`).

- **P5 completed.** 40 of 40 runs verified, and its replay audit reproduced
  all 40.
- **P6 completed, but its replay audit failed.** 40 of 40 runs verified,
  but 16 records, every `f4` and every `f6` run, replayed with different
  phase counts. The `sig-f5`, `enum` and rho records reproduced.
- **P7 was stopped** while running, when the P6 failure was found.
- **P8 never started.**

These sessions and audits are kept, unedited, in `attempt-1-voided/`. None
of their numbers enters any question of `PROTOCOL.md`.

Before the failure was found, P5's records had been run once through the
analyser, to check the analyser's plumbing. No decision has been read from
any of them.

## Cause

`f4_fp_tower` reduces its S-rows in chunks of
`per_task · 2 · rayon::current_num_threads()` rows. A row in a chunk is
reduced only by pivots from earlier chunks, so the number of
multiply-adds counted depends on the chunk size, and the chunk size depends
on how many threads rayon sees:

- the measured run is pinned to one CPU, so rayon sees one thread;
- the replay ran under a different CPU set.

Shown on P6 seq 1, with the same input and the attempt-1 binary:

| CPUs | multiply-adds | nodes | refuted |
| --- | --: | --: | --: |
| `taskset -c 3` (1) | 2,854,145,694 | 181 | 49 |
| 4 | 2,858,623,695 | 181 | 49 |

The search tree and every decision are the same; only the count differs.
P5's systems are small enough to fit in one chunk at any thread count,
which is why P5 reproduced.

## Fix

The `pkm-tower` oracle now runs both tower engines inside its own
one-thread rayon pool, built in `prepare`. With the fixed binary:

- the same input gives 2,854,145,694 multiply-adds under both CPU sets,
  the one-CPU value;
- a unit test (`counted_work_is_independent_of_the_callers_threads`) runs
  six whole decompositions from 1-thread and 4-thread callers and requires
  equal counts. It fails on the attempt-1 code (427,678,003 against
  428,421,214) and passes with the fix.

## What changes

- **Unchanged:** the specs, curves, seeds, arms, decision rules and every
  other part of `PROTOCOL.md`.
- **Re-run:** all four cells, from the commit that carries this
  amendment, into `sessions/` and `audits/`.
- **Library:** `f4_fp_tower` itself is not changed. Its other callers
  (`examples/pkm_tower_pilot.rs` and friends) ran unpinned. Their counts are
  therefore specific to their hosts' thread counts, which the pilot
  records did not state. That is noted here and not fixed in this round.
