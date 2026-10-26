# Amendment 2: the narrow engines past 64 unknowns (`wide-vars` build)

Registered 2026-10-20. Committed before any run of the grid below.

## Why

Amendment 1 answered the 64-unknown cap with a different solver
(`wide-chain`). The owner then asked to expand the narrow engines themselves.
`pq_groebner_f2::Mask` is now `u64` by default and `u128` with the Cargo
feature `wide-vars`; F4, inherited F4, F5/F6-IC, the Koblitz Gröbner driver
and the SAT encoder run on it, and `koblitz_groebner::MAX_VARS` is 128 in
that build. The default build is unchanged. This amendment reruns the three
arms that were `unsupported` at `m = 4`, n = 23, 29, 31 in `runs/m4/`.

## What is frozen

PROTOCOL.md §2–§6 and AMENDMENT_1.md apply unchanged, except:

- **Build:** `cargo build --release --features wide-vars --example
  ic_algebraic_floor`. Every record in `runs/m4-engines/` comes from it.
- **Arms:** `f4-chain`, `f6-ic`, `sat`.
- **Cells:** all seven `m = 4` cells (1:11:3, 0:13:4, 1:17:5, 1:19:5,
  1:23:6, 1:29:8, 0:31:8), so one build and one machine state cover the whole
  fit. The cells at n ≤ 19 also let the wide build be compared with the
  narrow records in `runs/m4/`.
- **Targets, floor, budgets, early stop, estimator, bootstrap, decision
  rule:** unchanged (seed 20261014, 1,024-target pool, 6 + 6 targets, 120 s
  and 4 GiB per target, stop after 4 leading censored targets, bootstrap
  2,000).
- **Output:** `runs/m4-engines/`; analysis `amendment-2/ANALYSIS.json` from
  `ic_algebraic_floor analyze --dir runs/m4-engines`.
- **Exactness:** every solved relation is verified in the group by the
  harness; a decomposable target an arm reports as "no relation" is an
  error, not a data point.
- **Counted unit:** as before, recorded per arm and never compared across
  arms.
- **Prediction (before data):** the three arms still grow faster than the
  floor, and n = 29 and 31 are censored at 120 s for all three. Widening the
  word changes what can be attempted, not the search the chain system
  forces.
- **Follow-up:** "grows more slowly than the floor" triggers PROTOCOL.md §7.
  Lower bounds from censored cells are never reported as full costs.
