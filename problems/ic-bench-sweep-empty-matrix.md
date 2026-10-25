# `ic bench --sweep` rejects a sweep that lists only `configurations`

- **Status:** `open`
- **Kind:** implementation gap
- **Affected work:** [`research/multiplier_closure_scaling_20261024/`](../research/multiplier_closure_scaling_20261024/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-24

## Problem

A sweep file with an explicit `configurations` list and no `matrix` cannot run, because `expand_matrix` returns one empty
configuration for an empty matrix and `src/bin/ic/bench.rs` appends it to the explicit list.

`expand_matrix` (bench.rs:770-799) starts from `vec![BTreeMap::new()]` and only multiplies when a key has choices, so an
empty matrix yields `[{}]`. The loader (bench.rs ~929) does `configs.extend(expand_matrix(&file.matrix))`, so the empty
map is run as a configuration and fails with "a configuration needs a `factor_base`". The "no configurations and no matrix"
check at the same place never fires, because `configs` is never empty.

## Evidence and uncertainty

- **Observed:** `ic bench --sweep` on `{"instance":{...},"configurations":[{"factor_base":"glv-orbit:size=50","oracle":"mitm:negation_folded=1,m=3"}]}`
  (prime j0 24 bit) returned `message: a configuration needs a factor_base`; the same configurations as a one-key-per-field `matrix` ran.
  Binary from origin/main 5cb553810e.
- **Derived or proved:** by reading the code above; not otherwise tested.
- **Suspected explanation:** the matrix helper was written for the matrix-only case.
- **Missing capability:** none; a one-line guard (skip the matrix when empty).
- **What this evidence does not establish:** whether any checked-in sweep file relies on the current behaviour.

## What would unblock it

- [ ] `expand_matrix` returns no rows for an empty matrix (or the loader skips it), with a test that an explicit-`configurations`-only sweep runs.
