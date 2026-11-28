# Exact WS6 commands

Run from the repository root. These are static/correctness commands, not
performance benchmarks.

```sh
cargo build --release --bin ecbench
sha256sum target/release/ecbench
target/release/ecbench optimize inventory --json \
  > research/ghs_frontier_3_7_20261119/ws6_solver_coverage/receipts/inventory.json
```

The fail-closed audit of the previously frozen 92 development shards was:

```sh
mapfile -t specs < <(find \
  research/ecbench_e2e_frontier_20261031/specs/shards \
  -maxdepth 1 -type f -name '*.json' | sort)
target/release/ecbench optimize audit \
  --development "${specs[@]}" --require-all \
  > research/ghs_frontier_3_7_20261119/ws6_solver_coverage/receipts/legacy-92-require-all.json
```

Exit 1 is the required disposition for that command: the JSON report is
well-formed, all 92 specs have zero structural errors, but the set covers only
20/111 live methods and 17/37 live nested algorithms.

The retained checker and adversarial test are:

```sh
research/ghs_frontier_3_7_20261119/ws6_solver_coverage/validate.sh
research/ghs_frontier_3_7_20261119/ws6_solver_coverage/selftest.sh
```

The targeted inventory unit test was rerun after the executable-level checks.
The first release-profile attempt compiled past the former duplicate-name
error but exhausted the overlay while writing its final test artifact. The
successful run used a clean RAM-backed target and a low-storage development
profile:

```sh
CARGO_TARGET_DIR=/dev/shm/ghs-ws6-unit \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_DEV_INCREMENTAL=false \
cargo test --lib \
  cryptanalysis::ecbench::optimization::tests::every_native_solver_and_nested_engine_is_in_the_inventory \
  -- --exact --nocapture
cargo clean --target-dir /dev/shm/ghs-ws6-unit
```

Result: 1 passed, 0 failed, 7,717 filtered. The weaker of two duplicate tests
named `the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` was removed
before this run. The retained test contains every removed assertion plus the
explicit `Tr(a)=1` refusal check. No production path changed in that cleanup.

