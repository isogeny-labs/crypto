# Multiplier factor bases silently drop colliding points

- **Status:** `open`
- **Kind:** implementation gap (reporting)
- **Affected work:** [`research/multiplier_closure_scaling_20261024/`](../research/multiplier_closure_scaling_20261024/README.md), `research/ic_improve_20261023/{PRIME,BINARY}_README.md`
- **Next action owner:** unassigned
- **Last updated:** 2026-10-24

## Problem

`cm-multiplier` and `binary-multiplier` skip any generated multiple whose point (or its negative) is already in the base
(`seen` checks, `src/cryptanalysis/ic_framework/plugins.rs` ~1543, ~1581, ~1711), without counting or reporting the drops.

The reported `points` (`signed_points`) is therefore counted after collisions are dropped, so it can be below the nominal
`K x (number of multipliers) x 2`, and a run cannot tell how many requested multiples were lost. Columns and weights stay
consistent (unit tests `cm_multiplier_columns_are_scalar_multiples_of_their_seed`, `binary_multiplier_columns_...` pass), so
correctness is not affected; the cost accounting of the grid (points vs nominal) is.

## Evidence and uncertainty

- **Observed:** code reading only; the scaling study's point counts (e.g. 960 for K = 4, norm 64 on j0 = 4 x 240) match nominal on the best cells, so collisions
  were not seen to matter there. No run counted drops.
- **Derived or proved:** not established.
- **Suspected explanation:** collisions are rare at these sizes and large K / small r.
- **Missing capability:** a `multiplier_collisions` native counter in the factor-base cost.
- **What this evidence does not establish:** that collisions are negligible at small r (r near 2^18..2^21) or large norm.

## What would unblock it

- [ ] Add a counted `multiplier_collisions` native counter and report it in the bench JSON; re-run the K = 256..512 cells at r = 2^18..2^21 and confirm nominal vs actual points.
