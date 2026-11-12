# `fes-then-batched`: the two measured gains combined (2026-11-12)

Owner: "Keep working on these things." Two levers measured today each lower
the S of the algebraic index-calculus row on part of the ladder:

- `fes-f2` at 14 unknowns, through the size rule `fes-then-inherited`
  ([`../size_rule_fold_20261112/`](../size_rule_fold_20261112/): degree-13
  Koblitz S 101.9 against `inherited-f4`'s 138.0);
- `inherited-f4-batched` at every size
  ([`../inherited_f4_batched_20261112/`](../inherited_f4_batched_20261112/):
  S 0.82 to 0.89× of `inherited-f4` on all six instances; at degree 13 its
  120.2 stays above `fes-f2`'s 101.9).

The new engine `fes-then-batched` (`src/cryptanalysis/ic_framework/solvers.rs`)
is the size rule with `inherited-f4-batched` above the threshold: `fes-f2` for
quadratic systems of at most `fes_max_vars` = 14 unknowns, the batched engine
otherwise. The threshold is the one fixed before the size-rule run. Test:
`ic_framework::solvers::tests::size_rule_and_inherited_fold_match_the_reference`
(extended: both branches against the exhaustive reference, word XORs no more
than `fes-then-inherited`'s).

## Protocol

`run_e2e.sh` under `tools/benchrec.py run`, frozen binary: `ic bench --sweep`
on the six instances of the e2e ladder, the algebraic oracle with
`inherited-f4`, `fes-then-inherited` and `fes-then-batched`, the pair table as
reference row, 3 repeats, 60 s per call, 16 matched rho runs, one thread,
local container, not isolated; no leaderboard row.

## Pre-stated expectation and falsifier

Counts are deterministic for fixed seeds, so: at degree 13 `fes-then-batched`
has exactly `fes-then-inherited`'s S (both run `fes-f2`); on the other five
instances it has exactly the S `inherited-f4-batched` had in the batched
study's run 2 (same instances, seeds and repeats), 0.82 to 0.89× of
`inherited-f4`'s. So it is the lowest algebraic row on every instance; the
pair table stays below it on random 21. Falsifier: any configuration failing
to recover its logarithm; `fes-then-batched` S differing from
`fes-then-inherited` at degree 13, or not below `inherited-f4` on any other
instance.

## Results `[pcc:unverified]`

Run 2026-11-12 20:09 to 20:13 Z, record
`benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json` (measured; raw
`results.tar.gz`, `results.json`), binary built at 4900647bcd from a clean
tree (SHA-256 in `results/ic-sha256.txt`). Full table `RESULTS.md`, graph
`results.svg/png`, `REPORT.pdf`. Every configuration recovered its planted
logarithm in every repeat, and so did every matched rho run.

| instance (unknowns) | rho S | pair table | `inherited-f4` | `fes-then-inherited` | `fes-then-batched` | ftb / `inherited-f4` |
|:--|--:|--:|--:|--:|--:|--:|
| Koblitz 13 (14) | 2.89 | 197.8 | 137.9 | 101.9 | **101.9** | 0.739 |
| random 15 (16) | 2.28 | 473.7 | 326.0 | 326.0 | **277.5** | 0.851 |
| Koblitz 17 (18) | 1.73 | 493.2 | 304.5 | 304.5 | **249.7** | 0.820 |
| Koblitz 19 (20) | 1.35 | 1,063.3 | 465.5 | 465.5 | **413.6** | 0.889 |
| random 21 (22) | 1.24 | 1,975.9 | 4,411.6 | 4,411.6 | 3,726.1 | 0.845 |
| Koblitz 23 (24) | 0.88 | 3,986.3 | 2,051.2 | 2,051.2 | **1,674.3** | 0.816 |

Mean over 3 repeats; bold where `fes-then-batched` is the lowest index-calculus
row of the instance.

- **`fes-then-batched` is the lowest algebraic row on every instance** and the
  lowest index-calculus row on five of six (the pair table stays below it on
  random 21): 0.74 to 0.89× of `inherited-f4`. At degree 13 it runs `fes-f2`
  (S 101.9, as `fes-then-inherited`); above it runs the batched engine.
- **The Koblitz instances reproduce exactly**: S equals `fes-then-inherited`'s
  at degree 13 and the batched study's `inherited-f4-batched` S at degrees 17,
  19 and 23 to every digit, since those rows are priced from pinned counts.
- **The pre-stated expectation was wrong in one detail**: on the random
  instances the absolute S differs from the batched study's (277.5 against
  223.9 at random 15; `inherited-f4` moved from 262.7 to 326.0), because those
  rows are priced by a word-XOR ratio measured on the host in each run, not a
  pinned one. The ratio to `inherited-f4` in the same run is unchanged (0.851
  and 0.845 in both runs). The falsifier did not fire: no failed recovery,
  degree 13 equal to `fes-then-inherited`, and below `inherited-f4` everywhere.
- Distance remaining to the matched rho with `fes-then-batched`: 35.3× at
  degree 13, 144× at Koblitz 17, 306× at 19, 1,903× at 23 (and 122× / 3,005×
  on the random instances).

No leaderboard row: no replay certificate or validator report (validation
gate); wall time per call carries the batched engine's caveat below 22
unknowns (`../inherited_f4_batched_20261112/`, runs 1 and 3).

## Sources checked

KB (`research-kb`) unavailable in this session. `src/cryptanalysis/ic_framework/{solvers,mod}.rs`
(the `measured` versus `pinned` word-XOR pricing), `research/size_rule_fold_20261112/`,
`research/inherited_f4_batched_20261112/`.
