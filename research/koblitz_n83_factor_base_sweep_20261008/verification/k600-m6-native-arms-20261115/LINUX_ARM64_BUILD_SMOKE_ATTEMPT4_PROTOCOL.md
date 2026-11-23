# Linux ARM64 build smoke: attempt 4 protocol

Frozen before execution. This retry tests the exact five-example build after
the three tracked `include_bytes!` files missing in attempt 3 were
materialized. It reuses the partial release target and is an engineering
feasibility check, not a cold build-cost or IC runtime sample.

Use clean committed HEAD containing this protocol; record its full SHA-1.
Pin the Linux ARM64 Rust 1.99.1 image ID
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
Mount source/Git read-only, retain the same `/build-target` cache, 12 GiB
container RAM, four CPUs, 512 PIDs, one Cargo job, 1,200 s internal timeout,
and the five example names/flags of attempt 3. Docker Desktop was restored
to its original 7,424 MiB after attempt 3; temporarily raise it to
14,336 MiB and verify engine memory before this run, then restore 7,424 MiB
even on failure. No other running container is allowed at the setting
change. No new code, fixture, or Cargo dependency is introduced.

The materialized tracked inputs and frozen SHA-256 values are:

| Input | SHA-256 |
|---|---|
| `verification/matched-k1182-k2048-900s-20261011/evidence-01/panel-k1182/manifest.json` | `2735f6960233749ac887f68705fd514dfc003e2212d722a8e50e5e9738aadcfa` |
| `verification/remaining-budget-k2048-20261010/manifest.json` | `68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8` |
| `verification/mixed-orbit-known-witness-source-20261012/fixture.json` | `fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538` |

The checkout was clean after materialization. `scripts/sparse-checkout.sh add`
declined because this worktree's sparse rules were not created by that
helper; `git sparse-checkout add` for the three small parent directories
materialized the tracked files without changing Git content.

Success requires Docker exit 0, `OOMKilled=false`, and all five executable
Linux ARM64 ELF outputs, each with byte count and SHA-256. Stop and retain
logs/state for OOM, timeout, other nonzero exit, or scratch space below
1 GiB; free space before launch was about 1.9 GiB. A passing warm build
only unblocks the separately frozen source-bound charged build, runtime
image, preflight, K600 arms and full-cost comparisons. Its elapsed wall
time must not enter a cold total-runtime row.
