# What the 32× is, and what it is not — 2026-11-25

The lane has been quoting "32× fewer F4 multiply-adds" since the first
degree-32 screen. This note separates that operation-count result from the
user-visible speed, because they are not the same number and the PR body should
not conflate them.

## Measured per arm, all three 13-bit equal-18 instances

| Instance | Call | Ops (G) | GAE (B) | Wall (s) | ns per counted MA | reported ns/op |
|:--|:--|---:|---:|---:|---:|---:|
| 20261185 | baseline:1 | 209.54 | 2.810 | 173.4 | 0.83 | 0.82 |
| 20261185 | candidate:1 | 6.40 | 1.944 | 100.5 | 15.69 | 11.30 |
| 20261185 | candidate:2 | 6.40 | 0.856 | 60.1 | 9.38 | 8.29 |
| 20261185 | baseline:2 | 209.54 | 2.680 | 124.5 | 0.59 | 0.49 |
| 20261221 | baseline:1 | 278.37 | 3.317 | 179.7 | 0.65 | 0.65 |
| 20261221 | candidate:1 | 8.35 | 0.309 | 28.7 | 3.44 | 3.25 |
| 20261221 | candidate:2 | 8.35 | 0.739 | 43.2 | 5.18 | 3.74 |
| 20261221 | baseline:2 | 278.37 | 2.843 | 156.6 | 0.56 | 0.43 |
| 20261276 | baseline:1 | 316.04 | 2.219 | 140.6 | 0.44 | 0.45 |
| 20261276 | candidate:1 | 11.64 | 0.622 | 30.1 | 2.58 | 2.46 |
| 20261276 | candidate:2 | 11.64 | 0.599 | 28.7 | 2.46 | 2.36 |
| 20261276 | baseline:2 | 316.04 | 2.782 | 137.8 | 0.44 | 0.42 |

Operation counts are **deterministic**: the two calls of each arm produce
identical totals (6.40 G twice, 8.35 G twice, 11.64 G twice). Wall time, GAE
and the derived price are not — the same baseline ops cost 0.59–0.83 ns each in
one instance and 0.44–0.65 in another, on one host and one binary.

## The three numbers

1. **Operation count: 32× for the candidate.** Robust across bits 13/14/15 and
   across all seven curves measured, with matched auxiliary-variable counts.
   This is a property of the systems, not of the machine.
2. **Wall clock: 1.7×, 6.3× and 4.7× on the three instances** (pair 1), i.e.
   never 32×. Charged GAE tracks wall, not ops: 1.45×, 10.7× and 3.6×.
3. **The gap is the per-operation price.** One counted candidate multiply-add
   costs 2.5–15.7 ns of wall time against 0.44–0.83 ns for the interval arm.

## The single-threaded basis: 7.35×, with a 4.5× price gap

The multi-threaded numbers above mix the algorithm with the scheduler. Re-running
the same two arms with `RAYON_NUM_THREADS=1` (4 repeats each, same instance,
quiet host) separates them:

| Arm | Ops | Solver wall | ns per counted MA | Charged GAE |
|:--|---:|---:|---:|---:|
| interval | 43.19G | 16.16 s | 0.374 | 0.338G |
| candidate | 1.303G | 2.20 s | 1.689 | 0.046G |
| ratio | **33.1×** | **0.136** | 4.51× | **0.136** |

Single-threaded, the degree-32 map solves the same 4 target repeats for
**0.136 of the charged cost — a 7.35× per-system speedup**. The 33× operation
reduction is diluted by a 4.5× higher price per counted multiply-add, not by
3–30× as the loaded runs suggested. Most of the earlier spread was thread-pool
scheduling: the candidate's price fell from 2.5–15.7 ns/MA under load to 1.69
ns/MA alone, while the interval's moved only 0.44–0.83 → 0.37.

So the correct scaling statement is: 33× fewer multiply-adds, **7.4× less
work**, and a 4.5× residual per-operation gap that is the engineering target.

## Why the price differs: the counter is not the runtime

`echelon` increments `muladds` only from `Acc::sub_tail`, i.e. once per
non-zero entry of a pivot row's tail. Everything else in the elimination loop is
uncounted: the word-scan of the accumulator, `Acc::take` and its reductions,
`drain_from`, `Pivot::new` (which copies and re-indexes a row), the sort, and
the bitset maintenance (`runs`, `bits`, `hi`).

The interval arm's rows are dense (190–465 nnz/row), so its runtime is
concentrated in the counted tail arithmetic and the price per counted op is
near a true F_p multiply-add. The candidate's rows are 3.5–3.8× sparser
(39–116 nnz/row), so a larger fraction of its time sits in the per-row
bookkeeping that the counter does not see. The count is honest about
multiply-adds; it is simply not a proxy for wall time in the sparse regime.

A `sample` profile of a candidate run confirms the elimination phase dominates
(`f4_tower_cached` → `eliminate` → rayon chunk reduction), so this is not time
lost elsewhere in the oracle. The profile also shows what the uncounted time
looks like at the bottom: on a host that was running other work, 72 % of samples
sat in `swtch_pri` thread yielding inside the rayon pool and 20 % in
`__psynch_cvwait`. Small sparse row tasks plus a busy host means scheduler
overhead, not arithmetic, becomes a large share — which the muladd counter
cannot see by construction.

That splits the price gap into two uncounted terms which this note does **not**
yet separate: per-row bookkeeping inside `echelon` (scan, drain, pivot copies,
bitset work) and thread-pool scheduling. Both are engineering targets; only a
quiet-host A/B with the per-step `ms_rows / ms_reduce / ms_echelon / ms_update`
breakdown surfaced in the store will apportion them.

## Consequence

- The correct headline for a user is the closed-pipeline charged ratio,
  0.284–0.314, and the deterministic 32× op reduction **described as an
  operation count**. On a quiet single thread the same comparison is 0.136
  (7.35×), so the pipeline figures are conservative by roughly the factor the
  host's scheduler takes from the sparse arm.
- The remaining headroom is 4.5× per system, and it is an engineering target,
  not a mathematical one: cut the uncounted work — the sparse-echelon per-row
  bookkeeping (`Pivot::new` copies, accumulator drain, bitset maintenance).
  Closing the price gap would turn 7.35× into ~33× on identical hardware; the
  op count would not change at all.
- Any future claim should quote wall or GAE per instance and treat the 32× as
  an operation-count invariant that is *not* load-dependent and *not* a
  speedup.
