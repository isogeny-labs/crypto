# 2026-11-18 — repair main: duplicate test (PR #4691)

## 07:30Z — lib-test target compiles

### What was done
- Removed the merge-inserted duplicate `the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e`
  in `src/cryptanalysis/ic_framework/plugins.rs` (kept the complete copy with its doc comment and
  the `Tr(a) = 1` refusal assertion).
- Verified: `cargo test --release --locked --lib --no-run` → `Finished`, unit-test executable built.
- `cargo build --release --locked --lib --tests` now stops only at example `n83_d18_shared_bits`,
  which `include_str!`s `research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv`,
  a store-managed artifact (`.artifacts.json`) not pulled in this checkout — environmental, not a code fault.

### Decisions made
- Kept this PR to the one duplicate; registry/names and artifact-store CI failures are inherited from `main`
  and were explained on the PR instead of being fixed here.

### Open threads
- CI `names` (unregistered `icv1-*` names, `curve_aliases.json` out of sync) and store-pull jobs (`gate` etc.)
  fail on `main`; separate repair.

### Errors
- First test-build log filtered away the error text; a container restart killed the rerun; re-ran and isolated
  the lib-test target.
