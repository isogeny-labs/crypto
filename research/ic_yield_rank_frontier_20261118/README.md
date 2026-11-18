# Fixed-population yield/rank contract

Date: 2026-11-18

This directory implements the schema/checker portion of
`RT-IC-PUBLIC-POPULATION-CONTRACT`.  It uses synthetic fixtures only.  It does
not produce, admit or authorize a natural scientific population.

The contract binds:

- the curve/subgroup, generator, factor base, symmetry quotient and arity;
- solver/checker versions, scalar field, budgets and registered scale ladder;
- the complete target manifest and a digest of that frozen specification;
- every terminal target outcome, including zero, rejection, failure and
  timeout;
- every relation row and rejection reason; and
- the complete prefix rank/support trace over accepted rows.

`validate_population.py` is the authoritative fail-closed checker.  The JSON
Schema is a portable structural description; cross-record invariants such as
manifest completeness, scalar ranges and rank monotonicity are enforced by the
checker.

```sh
python3 research/ic_yield_rank_frontier_20261118/test_contract.py
python3 research/ic_yield_rank_frontier_20261118/validate_population.py \
  research/ic_yield_rank_frontier_20261118/fixtures/valid.json
```

The formal countermodels and their instance-gate analysis are recorded in
[`PROOF_MAP.md`](PROOF_MAP.md).  No finite fixture in this directory is evidence
for an asymptotic yield or terminal-rank law.
