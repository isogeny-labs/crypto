# Native rebuild and replay

All paths below are relative to the repository root except the sealed dependency
cache. Record exact compiler/library hashes and use new output paths.

```sh
study=research/trace_incidence_20261116/setup_20261123
bash "$study/build.sh" NEW_BUILD
bash "$study/build-target.sh" NEW_BUILD
bash "$study/build-candidates.sh" NEW_BUILD
bash "$study/build-current-rho.sh" NEW_RHO_BUILD
# Reuse retained public fixtures for exact replay. The following creates a new
# cohort only if both output paths do not already exist.
bash "$study/build-flat.sh" NEW_BUILD ORIGINAL_MINIMAL NEW_PUBLIC NEW_PRIVATE_VALIDATION
bash "$study/qualify-flat.sh" NEW_BUILD NEW_PUBLIC NEW_QUALIFICATION
bash "$study/run-flat.sh" NEW_BUILD NEW_PUBLIC NEW_RHO_BUILD/current-rho NEW_RUN
NEW_BUILD/analyze-flat NEW_RUN/rows.jsonl NEW_PUBLIC NEW_PRIVATE_VALIDATION NEW_SUMMARY
```

`build-candidates.sh` is a consolidated rebuild command added after the native
measurements; individual stage qualifiers retain the actual executable hashes
used. The original static FFI archive supplies exactly the retained `ti_*` ABI.
Its public-check/group code was not rebuilt against the whole current Rust crate.
The separate current-rho wrapper compiles the intact current prime_fast module
and uses the sealed older compatible crate only for adapter types/primality.
Its build script identifies the exact rlib dependencies. This is not a portable,
cache-free Cargo build. Retained executables and dependency hashes allow replay
on the recorded runtime; rebuilding from source needs those intact sealed caches.

The private scalar validation file is passed only to analysis. Solvers receive
public curve/order/generator/target/seed. Given-input target construction is
excluded for every arm; all actual pool construction is inside the timer.
Never feed the private validation path to a solver or substitute planted pools.
