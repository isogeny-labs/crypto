# `fes-then-batched`: the two measured gains combined, 2026-11-12

Stage diagnostic (the algebraic index-calculus oracle end to end), not a
leaderboard row: no table row in `benchmarks/README.md`, no VALIDATION.json.
Study and pre-stated expectation:
[`research/fes_then_batched_20261112/`](../../research/fes_then_batched_20261112/).
Record `benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json`
(measured). `ic bench --sweep` on the six e2e-ladder instances, 3 repeats,
one thread, local container `[pcc:unverified]`:

- At the pinned word-XOR ratio, `fes-then-batched` (`fes-f2` up to 14
  unknowns, `inherited-f4-batched` above) has the lowest S of the algebraic
  rows on every instance, 0.74 to 0.89× of `inherited-f4`, and the lowest
  index-calculus S on five of six. Under the AGENTS.md §5a band the ranking
  is unresolved: a tie with `inherited-f4` at w = 0 on five instances, above
  it at w = 0 at degree 13 (independent validator, `VALIDATION.json` in the
  study; gate verdict `valid` for the measurement).
- Koblitz rows reproduce the earlier runs exactly (pinned pricing); random
  rows move in absolute S with the host-measured word-XOR ratio while their
  ratio to `inherited-f4` is unchanged.
- All configurations and all matched rho runs recovered their logarithms.

Files: `REPORT.pdf`, `results.svg`, `results.png`.
