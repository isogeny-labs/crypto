# `genroot`: the generalized root-finder as an IC decomposition oracle

**Change class:** integration and measurement. No speed, exponent or
security claim.

## What was added

- `tools/generalized-rootfinder/src/solver.rs`: `S3Chain`, the exact
  all-roots solver for the `m`-summand Semaev `S3` chain
  `S3(w_i, x_{i+1}, w_{i+1}) = 0` (`w_1 = x_1`, `w_m = x(R)`), with sources
  `x_i` in a domain `D` and auxiliaries anywhere in the field, solved by
  meeting in the middle after any `k` of the `m` sources. The existing
  five-summand `Separator` is the case `m = 5, k = 3`; it now shares the
  `S3` evaluation and fiber code (`s3_evaluate`, `s3_fiber_coefficients`)
  with the chain. `field.rs` keeps a per-thread tally of field
  multiplications and inversions (`op_tally`).
- `src/cryptanalysis/ic_framework/genroot.rs`: the `genroot` oracle. It
  compiles the tool's `solver.rs` into the crate (`#[path]`, with
  `extern crate self as crypto_lib` in `lib.rs`), so the oracle runs the
  code the tool's tests check. One generic implementation serves prime
  curves (`PrimeCurve`) and ordinary binary and Koblitz curves
  (`BinaryGroup`), because the chain needs only abscissae, the field and
  the curve's `S3`. Parameters: `m` (2–6), `left` (default `ceil(m/2)`),
  `max_table`.
- `ic bench --oracle genroot:m=…` on prime, char-2 and Koblitz instances,
  listed in `ic bench --list`.

## Contracts

- **Fail closed.** A solver error (cap, exhausted splitting budget,
  unrestricted fiber) is counted in `genroot_unknown` and the target is
  skipped, never read as "does not decompose". Every relation is re-added
  in the group by `lift_abscissae` before it is returned.
- **Costing.** Each fiber the solver computes is charged as two group
  additions; sign resolution is charged by `lift_abscissae`. The
  root-finding inside a fiber costs far more than that: its field
  multiplications and inversions are reported (`genroot_field_muls`,
  `genroot_field_invs`) but not priced, and the pipeline marks every
  `genroot` row `partial_counted_cost`, so the framework prints no
  `S/rho` for it. The counted `S` below is therefore a **lower bound** on
  `genroot`'s cost.

## Tests

`cargo test --manifest-path tools/generalized-rootfinder/Cargo.toml`:
16 pass, including

- `s3_chain_matches_brute_force_for_every_split`: every `m = 2..4`, every
  split `k`, every target, on five curves (`p = 7, 11, 13`, `F_8`,
  `F_16`), against exhaustive search over the auxiliaries;
- `s3_chain_with_five_summands_reproduces_the_separator`: identical
  solution sets to the `Separator` (`m = 5, k = 3`);
- the existing separator brute-force test, after the refactor.

End to end, every `genroot` run below recovered the planted logarithm.

## Matched runs (`run.sh`, `summarize.py`)

Same instance, factor base, walk targets (3 repeats) and 16-run matched
rho reference per pair; macOS aarch64, release build, one core of a shared
machine, so wall times are indicative only. Means over the 3 targets;
per-target rows are in `results/summary.md`. The rho reference is the
framework's matched one: the negation-map walk on prime and char-2
curves; on the Koblitz curve the cheaper of the signed-Frobenius walk
(mean S 13.48 at `n = 13`) and the negation walk (2.70), i.e. the
negation walk. The recorded JSONs were produced by binary
`a91c01d8…` (before a formatting-only pass); the committed tree's binary
reproduces the Koblitz `m = 3` rows exactly (same `S` and fiber counts).

| family | config | oracle | S counted | S / rho | wall s | field muls / fiber |
|---|---|---|---:|---:|---:|---:|
| prime, r ≈ 2^20 | m=2, \|F_x\|=256 | rootfind-subtract | 586.8 | 587.9 | 0.010 | – |
| | | genroot | 634.1 | ≥ 635.3 | 22.2 | 4,738 |
| | m=3, \|F_x\|=48 | rootfind-subtract | 31.5 | 31.5 | 0.001 | – |
| | | genroot | 34.9 | ≥ 35.0 | 1.12 | 4,756 |
| | m=4, \|F_x\|=16 | genroot | 422.2 | ≥ 423.0 | 14.7 | 4,744 |
| binary F_{2^17}, r ≈ 2^14.4 | m=2, dim 8 | mitm | 264.5 | 163.5 | 0.005 | – |
| | | genroot | 256.4 | ≥ 158.6 | 2.37 | 455 |
| | m=3, dim 5 | mitm | 61.9 | 38.2 | 0.001 | – |
| | | genroot | 58.7 | ≥ 36.3 | 0.49 | 454 |
| | m=4, dim 4 | genroot | 592.3 | ≥ 366.2 | 5.74 | 452 |
| Koblitz F_{2^13}, r ≈ 2^11 | m=2, dim 5 | mitm | 31.0 | 11.5 | 0.000 | – |
| | | genroot | 134.9 | ≥ 50.0 | 0.21 | 359 |
| | m=3, dim 4 | mitm | 56.9 | 21.1 | 0.000 | – |
| | | genroot | 60.4 | ≥ 22.4 | 0.08 | 355 |
| | m=4, dim 3 | genroot | 169.2 | ≥ 62.7 | 0.28 | 353 |

## Reading

- On matched targets `genroot` needs the same number of relations as the
  existing oracles in every configuration but one (binary `m = 3`, first
  target), and its counted `S` is within about 10% of theirs, except
  Koblitz `m = 2` (4.4×). That is expected: the chain's meet in the
  middle is the same search as `rootfind-subtract` and the pair tables,
  done on abscissae.
- It is slower in wall time: about 470–2,200× where the baseline's time
  is measurable (prime and binary rows); the Koblitz baselines finish in
  under a millisecond, too fast to ratio. A fiber
  is a quadratic root-finding (about 350–4,750 field multiplications in
  the solver's arbitrary-precision arithmetic), where the existing oracles
  do one native group addition. Pricing the fibers' field work would raise
  `genroot`'s `S` well above the existing oracles'.
- What it adds is coverage on prime curves: `m = 4` to 6, where the prime
  oracles in `ic bench` stop at `m = 3`. (On binary and Koblitz curves
  `descent-algebraic` and `chained-x` already reach `m = 4`.) Every such
  configuration here sits far above rho.
- Every configuration, old or new, costs 21× to 635× rho. Nothing here
  changes the repository's IC-vs-rho verdict on any of the three
  families, and the Koblitz factor-base wall (`x^13 - 1` has only
  invariant subspaces of dimension 1, 12 and 13) is untouched.

Frobenius-orbit bases are not in the panel: at `n = 13` they have 2 or
4,096 abscissae, and the `n = 15` Koblitz instance (`r ≈ 2^7.7`) is too
small to verify. `genroot` itself accepts any base.
