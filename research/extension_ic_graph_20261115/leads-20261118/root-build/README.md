# root-build lane: whole-root build repair (2026-11-18)

All statements below are engineering observations from local `cargo check` runs. [pcc:unverified]
No research result, cost, benchmark row or rho comparison is changed by this lane. Nothing was
relabelled; the edits only remove byte-for-byte or strict-subset duplicates left by merges.

## Question
Does current `main` (origin/main `ad2a014c00`, includes PR #4689) still fail to build because of
`f4_fp.rs` duplicates (`recording_degree_falls` twice in `impl F4Options`, duplicated degree-fall
fields) and the "8 crate::metrics errors" in `crates/hashed-ecdl-tensor`
(`../../runpod-table-roots-20261117-47/CI_BLOCKERS.md`)? Is a fix already open elsewhere?

## Findings
1. **f4_fp.rs: already fixed on main.** Commit `4219f6fa28` ("Repair main: restore Hilbert
   producer API; dedup degree-fall fields and trace fns") plus `662b4c4bb9`/`ac6474dd30` removed the
   duplicates. Only one reference to the degree-fall option remains (doc comment, line ~6493).
   `cargo check -p crypto --lib` passes (log `cargo-check-crypto-lib.log`, exit 0, 2m59s).
2. **hashed-ecdl-tensor `crate::metrics`: already fixed.** `src/lib.rs:29` has `pub mod metrics;`.
   `cargo check -p hashed-ecdl-tensor --all-targets` passes (log
   `cargo-check-hashed-ecdl-tensor-all-targets.log`, exit 0).
3. **Open PRs:** `gh api -X GET search/issues -f q='repo:isogeny-labs/crypto is:pr is:open f4_fp'`
   returned `total_count 0`.
4. **Remaining breakage on main (non-lib targets), found and fixed here** (all merge duplicates):
   - `src/cryptanalysis/ic_framework/plugins.rs` (cfg(test)): test
     `the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` defined twice (E0428). Removed the
     first, undocumented copy; the kept copy is a strict superset (same assertions plus
     `trace_kernel_basis(&ko, 6).is_err()`). No test coverage lost.
   - `tests/ml_dataset_live_capture.rs:148`: `channels: None` specified twice in one `DatasetPlan`
     literal (E0062). Removed the leading copy; the one beside `factor_base_policies` remains.
   - `examples/jv_sieve_quadratic_pair_relation_controls.rs`: `fn main() {}` defined twice (E0428),
     residue of `0eceac7ae5` and `b0aba1c106` both adding one. Removed the first; kept the
     documented one at file end.
   Files edited outside `src/` (tests/, examples/) are listed here explicitly; they were the exact
   files the compiler named. The orchestrator committed them as `1f1397f525` and `257eef0391`.
   Combined diff vs origin/main: `fix.diff` (30 deletions, 0 additions).
5. **Not a source bug:** with `--lib --tests`, example `n83_d18_shared_bits` (test = true) fails on
   `include_str!` of `research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/
   selected-representatives.csv`. That directory keeps its data in the artifact store
   (`.artifacts.json`, `.gitignore`); the file is not hydrated in this checkout. Left alone.

## Commands run (CARGO_TARGET_DIR=/home/user/cargo-target-round2, rustc 1.97.0)
| command | log | exit |
|---|---|---|
| `cargo check -p crypto --lib` | cargo-check-crypto-lib.log | 0 |
| `cargo check -p hashed-ecdl-tensor --all-targets` | cargo-check-hashed-ecdl-tensor-all-targets.log | 0 |
| `cargo check -p crypto --lib --tests` (before fix) | cargo-check-crypto-lib-tests.log | 101 (plugins.rs E0428, ml_dataset E0062) |
| same, after fix | cargo-check-crypto-lib-tests-after-fix.log | 101 (only n83 artifact include) |
| `cargo check -p crypto --profile test --lib --test '*'` | cargo-check-crypto-libtest-integration-after-fix.log | 0 |
| `cargo check -p crypto --bins --benches` | cargo-check-crypto-bins-benches.log | 0 |
| `cargo check -p crypto --examples` (before fix) | cargo-check-crypto-examples.log | 101 (duplicate main) |
| `cargo check -p crypto --examples --keep-going` (after fix) | cargo-check-crypto-examples-after-fix.log | 0 |

Disk: examples check grew `debug/incremental` to 9.3 GB (free fell to 4.0 GB); with no cargo/rustc
running, the regenerable `incremental/` cache was deleted (free back to 14 GB).

## Not done / limits
- No full-workspace check, no clippy (`-D warnings`), no `cargo test` execution; other workspace
  crates were not checked beyond those pulled in as dependencies.
- CI's artifact-access (403/InvalidAccessKeyId), digest-parity and validation-gate failures in
  CI_BLOCKERS.md are credential/data issues, not compile errors; untouched.
- research-kb: `scripts/research-kb` reports "no executable found"; MCP server not available in this
  agent. No sources read.

## Next steps
- CI on the lane PR is the authority: watch the `check`/`clippy`/`sweep` jobs.
- A merge-time guard (e.g. `cargo check --all-targets` on merge commits) would catch the recurring
  merge-duplicate pattern (`fn main`, tests, struct fields) seen in four separate repairs.
