# No sub-base-restricted decomposition test in the cover attack (large primes have nothing to trade)

- **Status:** `open`
- **Kind:** proof gap (the structural claim is argued, not proved); empirical bottleneck (relation collection is 99.99 % of S)
- **Affected work:** lead L6 of #2216 (`research/ic_cover_leads_20261018/`); study `research/jv_cover_large_prime_20261018/`
- **Next action owner:** unassigned
- **Last updated:** 2026-10-18

## Problem

We cannot lower `S` of `examples/jv_cover.rs` by a large-prime variation, because the cost of a decomposition test (`C_cov` ≈ 1.22M counted multiplications, about 791 tests per decomposition) does not fall when the factor base is restricted to `F' ⊂ F`, and the linear algebra, the only term that does fall, is 2.1e-5 of `S` at p = 101 and 2.5e-3 at p = 12000.

The instance is the cover attack on `E(F_{p^6})` (genus-3 GHS cover, six-point Nagao decompositions, `|F| ≈ p/2`). The acceptance criterion was a modelled saving of at least 10 % for some p ≤ 12k.

## Evidence and uncertainty

- **Observed:** 322 real decompositions at p = 101 (6 seeds) and 254 at p = 251 (2 seeds) have uniform abscissae and 1/2/≥3-outside shares matching the hypergeometric model (largest |z| 2.6 over 72 cells); `research/jv_cover_large_prime_20261018/raw/yields.json`.
- **Derived:** with generic coefficients, any elimination needs at least as many solutions as the plain method (`X ≥ touched + 1`, so `c* = 0.9975`); the best saving is the LA share plus 0.25 %. Single and double large-prime graphs need more solutions (Monte Carlo). At p ≤ 12k the best modelled saving is 0.02 % (1 large prime), 0.27 % (2), and at most 0.56 % for the information bound.
- **Suspected explanation:** none needed.
- **Missing capability:** a decomposition test whose cost depends on `|F'|`, for example a relation search that walks only `F'` (the sieve path of `src/cryptanalysis/jv_sieve.rs`).
- **What this evidence does not establish:** that no F'-dependent test exists; large-prime behaviour of the sieve path; any p above 12k beyond the model (extrapolated to p ≥ 1e6).

## What would unblock it

- [ ] A test (or the sieve path with a counter at `m-1`) whose measured cost per relation over `F'` falls with `|F'|`, with the discrete logarithm verified at p = 1009 and 1823 over at least two seeds.

## Next discriminating step

- **Action or proof obligation:** model and measure a single-large-prime sieve (`jv_sieve.rs`): the LA share there is already 8.5–9.9 % at p = 1823 (`experiments/38_jv_cover_sieve_1823.json`).
- **Inputs and controls:** the existing sieve records at p = 1009, 1511, 1777, 1823; plain sieve as the control.
- **Outcomes and next actions:** a saving of at least 10 % of S at p ≤ 12k lands a large-prime sieve; none closes this record for the sieve too.
- **Recheck trigger:** a new decomposition test, or a sieve S/rho whose LA share exceeds 10 % at p ≤ 12k.

## Resolution and history

2026-10-18: opened with the study above.
