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

## Results

Filled after the run.
