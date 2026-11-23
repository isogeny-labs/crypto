# Root flat recovery replay

Archive337 preserves the actual-root322 native36 matrix, complete cache33098-call cohort, failed32996-call cohort, all new build/audit logs, primary source contexts, all1452 runtime source pins and frozen correctness tools. Original component archive301 stays unchanged. Report334 records scope, failures and pending gates. These are unsigned correctness records; no comparative timing or 15× admission. [pcc:unverified]

From this directory, validate the SHA sidecar and extract into a new directory:

```sh
sha256sum -c root-flat-recovery-337.tar.xz.sha256
mkdir /tmp/root-flat-recovery-337
tar -xJf root-flat-recovery-337.tar.xz -C /tmp/root-flat-recovery-337
cd /tmp/root-flat-recovery-337
```

Index337 lists every payload path, size and SHA-256. Verify all entries and the exact file roster; index.json is the sole unindexed file. Source322, source330 and auditor331 records bind every runtime/harness/checker source. Complete earlier failed contexts are under contexts/; they are never reconstructed by replacing current source in place.

Replay all36 saved proofs with the same separate arithmetic checker and semantic canaries, on Linux x86-64 with CPU0 available:

```sh
taskset -c 0 bin/flat-root-audit native-322 references-85 \
  study/MATCHED_PINS_BEFORE_85.json /tmp/flat-root-replayed-337.json
cmp /tmp/flat-root-replayed-337.json study/FLAT_ROOT_AUDIT_331.json
```

Reexecute the post-run-frozen actual-root cache correctness test, retaining all96 varied-input calls plus two declared same-last-input controls and every exhaustive4096-assignment oracle. Use a new output directory:

```sh
ROOT_CACHE_QUALIFICATION_OUT=/tmp/root-cache-replayed-337 \
  RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 \
  RUST_TEST_THREADS=1 taskset -c 0 bin/cache-controls-330 --test-threads=1
```

The binary is frozen after330, explicitly a correctness reproduction rather than a prospective performance pin. All input identities, independent models, actual solution sets, algorithm statistics and per-call obligation counts must equal cache-330. Raw wall clocks are diagnostic and vary. Failed329 remains failed (no successful summary); replay330 does not relabel it. Native counter reexecution and independent-host/signed-execution attestation are absent. Full root unit332 fails compilation on18 diagnostics and executes zero tests. [pcc:unverified]

For source rebuilds, use source/ with the root dependencies/submodules at the PR's pinned parent, ordinary Cargo locked dependency resolution, and explicit release opt-level0. Root native tests use root-integration/Cargo.toml with default-features=false. Whole-root332 uses default features with opt0/no-LTO/codegen16. These correctness profiles and Boolean cache fixtures do not establish original Fp solving degree, optimized runtime or a pipeline speedup. [pcc:unverified]
