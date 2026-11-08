# DRAFT Amendment R2-A (proposed number 10): stage B, `m = 6`, `t = 6` (`lambda = 1.52`)

**Status: draft, NOT applied, NOT frozen. Requires amendment freeze.** Written 2026-11-08 (round 2, slug `r2pkm`)
for decision 6 of `research/open_leads_execution_20261108/DECISIONS.md` ("written as a dated protocol amendment
before its cell runs; a cell with no frozen amendment does not run"). Target file when frozen:
`research/prime_fall_degree_20261027/PREREGISTRATION.md` (append as "Amendment 10", with the freeze date). Renumbered from 9 on 2026-11-08, because R2-C was frozen first as Amendment 9.
Every number is `[pcc:unverified]`. Evidence classes are marked: derived (arithmetic on the registered formula),
cited (a repository document says so).

## 1. Timing

Freeze **before `pfd-b` is submitted**. `pfd-b` is the only request that contains the cell: `run.sh hky-B` runs
`B-m6-tower` with `--max-t 6` (cited: `run.sh`). The rule below depends only on `(m, t, p)`; it uses no recorded
row, no new data and no outcome of any other cell, so it can be frozen now.

## 2. The problem (cited, derived)

PREREGISTRATION §5 flags `lambda = |V|^m * 2^(m-1) / (m! * p)` and Amendment 1 guards only the `P0` arm with
`lambda < 0.1`. Amendment 7 C certificates decomposability only for Kummer `m = 2` (`grid_zeros`). For `m >= 3` a
closure that never contains `1` is read "`d_F > cmax`", although with `lambda >= 1` most random targets are
decomposable, and a decomposable target has a consistent system for which `1` never enters (relaunch README, item 9).
Derived `lambda` at `P1 = 2013265921` for every stage B cell that `run.sh hky-B` runs at `P1`
(`--max-t-m3 8 --max-t-m4 6 --max-t 6`):

| m | t = max | lambda (derived) |
|--:|--:|--:|
| 3 | 8 | 0.0056 |
| 4 | 6 | 0.0028 |
| 5 | 6 | 0.0711 |
| **6** | **6** | **1.517** |

`m = 6`, `t = 5` is 0.0237 and every other cell of stages A, B, C at `P1`, outside `m = 2`, is below 0.1
(derived with `python3`, `lambda(m, t)` as above). `m = 2` `t = 14` (0.133) is already governed by Amendment 7 C.
So the one cell with `lambda >= 0.1` that has no certificate is `(Kummer, m = 6, t = 6, P1)`.

## 3. Rule (the text to freeze)

> **Amendment 10 (R2-A), dated at freeze, before `pfd-b` is submitted.** For `m >= 3`, a cell whose
> `lambda = |V|^m * 2^(m-1) / (m! * p) >= 0.1` at the prime it runs at is **high-lambda**. It is run as registered
> (no flag or command changes), and its rows are written. They are reported in a separate table labelled
> "possibly decomposable (lambda >= 0.1, no certificate)". A high-lambda cell is **excluded from every §6
> median and rule (H-LFD, H-FFD, H-gap, H-null, H-m) and from the Amendment 6, 7 and 8 readings**. It is not counted as
> censored and not as "`d_F > cmax`", and it is not redrawn. For H-LFD, "the largest completed `N`" of the
> `(Kummer, m = 6)` ladder is therefore the largest completed `N` among the cells that are not high-lambda
> (`t <= 5`). Cells with `0.01 <= lambda < 0.1` keep the Amendment 7 C label "possibly decomposable" and stay in the
> rules. `lambda` is computed from `(m, t, p)` and recorded per row; it is not estimated from outcomes.
> At `P1`, the rule changes exactly one registered cell: Kummer `m = 6`, `t = 6`. No hypothesis, instrument,
> budget, seed, request or command changes.

## 4. Why this option (and what was not chosen)

- *Drop the cell* (no `--max-t 6` for `m = 6`): equally defensible, but it edits `run.sh`, hence the image, and
  discards a row that costs nothing extra (the relaunch README expects it to censor on `max_cols` anyway: 2,210,380
  columns at `c = 7` already at `t = 4`; derived). Running and excluding keeps the image and the `pfd-b` request unchanged.
- *Condition on inconsistency* needs a decomposability certificate for `m = 6`. None exists (Amendment 8 says the
  `m = 4` one is not adopted either), so it cannot be frozen as a runnable rule.
- The `0.1` threshold is Amendment 1's. It is not tuned to any observed row.

## 5. Consequence stated before data

If `(Kummer, m = 6, t = 6)` would have completed and been the largest `N` of its ladder, the `m = 6` H-LFD reading is
taken at `t = 5` instead. That is a loss of range, accepted here in return for never mixing "decomposable" and
"closure did not reach `1`". The row is still published (separate table), so a reader can see it.

## 6. Interaction

`pfd-b` request, `entry.sh`, `entry_r2.sh`, `run.sh` and the image are unchanged by this amendment. The scorer has to
implement the exclusion and the separate table; that is scorer work (relaunch README §5 item 7), done before scoring,
not before the run.
