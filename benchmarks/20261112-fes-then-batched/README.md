# `fes-then-batched`: the two measured gains combined, 2026-11-12

Stage diagnostic (the algebraic index-calculus oracle end to end), not a
leaderboard row: no table row in `benchmarks/README.md`, no VALIDATION.json.
Study and pre-stated expectation:
[`research/fes_then_batched_20261112/`](../../research/fes_then_batched_20261112/).
Record `benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json`
(measured). `ic bench --sweep` on the six e2e-ladder instances, 3 repeats,
one thread, local container `[pcc:unverified]`:

- `fes-then-batched` (`fes-f2` up to 14 unknowns, `inherited-f4-batched` above)
  is the lowest algebraic row on every instance, 0.74 to 0.89× of
  `inherited-f4`, and the lowest index-calculus row on five of six (the pair
  table stays below it on random 21).
- Koblitz rows reproduce the earlier runs exactly (pinned pricing); random
  rows move in absolute S with the host-measured word-XOR ratio while their
  ratio to `inherited-f4` is unchanged.
- All configurations and all matched rho runs recovered their logarithms.

Files: `REPORT.pdf`, `results.svg`, `results.png`.
