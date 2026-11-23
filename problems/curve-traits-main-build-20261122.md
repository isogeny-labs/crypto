# Current main tree does not parse: truncated ecbench/ic_framework merge

- **Status:** `open`
- **Kind:** implementation gap; inherited base-tree build blocker
- **Affected work:** every Rust build from `main` after `52e986e114`, including `curve_traits` and PR #4932
- **Next action owner:** unassigned; maintainer of the `ecbench` canonical-panel integration
- **Last updated:** 2026-11-22

## Problem

`origin/main` does not parse from `52e986e114` (`Merge main into
codex/ecbench-canonical-panel-20261118`) through at least `ff1f049288`.
Three source files carry unclosed delimiters, and the same merge dropped
declarations and imports. No `cargo build`, `cargo check` or `cargo test`
of the library can run, so PR #4932 cannot regenerate
`docs/curves/traits.json` or run its `curve_traits::dissect` tests.

This is a source-integrity blocker, not a mathematical result or a failure
of any dependent branch.

## Evidence and uncertainty

- **Observed:** at `origin/main = ff1f049288`, `rustfmt --edition 2021
  --check` reports unclosed delimiters in
  `src/cryptanalysis/ecbench/spec.rs`,
  `src/cryptanalysis/ic_framework/mod.rs` and
  `src/cryptanalysis/ic_framework/plugins.rs`. `git show
  origin/main:src/cryptanalysis/ecbench/spec.rs | tail -5` ends at
  `plan_for_new_run(binary_groebner_spec(...`, `.expect("canonical profile
  plans");` and `}`: the call and its closing delimiters are missing.
- **Observed:** `git merge-tree --write-tree 58356b85 05714baf` reports
  content conflicts in `docs/ACTIVE.md`, four session logs,
  `src/bin/ecbench.rs`, `src/cryptanalysis/ecbench/challenge.rs`,
  `src/cryptanalysis/ecbench/spec.rs` and `src/cryptanalysis/ecbench/sweep.rs`.
  The committed merge is a hand resolution of those conflicts.
- **Observed:** restoring the three parse-broken files from either parent
  exposes further losses in the same area: `CurvePolicy` /
  `validate_curve_policy` / `plan_for_new_run` references, and
  `F4Skeleton`, `F4PairTrace`, `TRACE_DEFAULT_CAP_BYTES` imports in
  `src/cryptanalysis/ic_framework/solvers.rs`. The break is therefore wider
  than the three unclosed files.
- **Observed:** the `Rust all targets compile` workflow was skipped on the
  main pushes at `021cd64a`, `17a8f9cfdc`, `c6a9c98b66` and `4765efa466`
  because its scope job classified them as not touching Rust. The last
  completed `check` job in the sampled 60 runs failed (`64c98d1af1`,
  `161dc76a1b`); no sampled run had a successful `check`.
- **Derived:** the files named above are unchanged by PR #4932, which only
  edits `src/cryptanalysis/curve_traits/dissect.rs` and
  `docs/curves/TRAITS.md`. Its changed file is `rustfmt`-clean and its
  arithmetic was type-checked in a standalone `num-bigint` crate, but that
  is not the crate build.
- **Suspected explanation:** the merge's conflict resolution concatenated
  parent regions without closing them. No particular author or tool is
  attributed.
- **What this evidence does not establish:** it does not show that the
  affected features are wrong, that the mathematics fails, or that no
  toolchain can build a coherent merge of the two parents. It does not
  establish which parent side is correct for each conflicted file.

## What would unblock it

Re-resolve the `52e986e114` merge from `58356b85` (main side) and
`05714baf` (feature side), or revert the bad merge and re-land it from a
rebuilt branch, keeping the feature's new declarations and main's newer
callers together.

- [ ] `cargo check --workspace --all-targets` and
  `cargo check --lib --tests --features wide-vars` pass at the repaired
  head with `RUSTFLAGS=-Awarnings`.
- [ ] The `check` job of `Rust all targets compile` runs (not skipped) and
  succeeds on a push to `main`.
- [ ] `cargo test --release --lib curve_traits::dissect` passes, then
  `cargo run --release --bin curve_traits -- build` and `curve_traits check`
  regenerate and accept `docs/curves/traits.json` for PR #4932.

## 2026-11-23 update: the damage is wider than three files

Repair attempts on the three original files reached the next parse failure and then the next. `rustfmt` 1.10 (Rust 1.99.1, so not a toolchain-parser artifact) rejects **14 files** across at least five merge events:

| file | corrupting merge | last version that parses |
|---|---|---|
| `src/cryptanalysis/ecbench/spec.rs`, `ic_framework/{mod,plugins,solvers}.rs`, `Cargo.toml` | `52e986e114` | `58356b85` (except `solvers.rs`, which needs the missing `F4*` imports) |
| `src/bin/genus_ladder.rs`, `src/cryptanalysis/quartic_ladder.rs` | `72b9183914` | `f49709baf2`, `905ba32130` |
| `src/bin/hecx.rs`, `hec_dlp.rs`, `binary_hyperelliptic_index_calculus.rs`, `hyperelliptic_index_calculus.rs`, `efd_representations.rs`, `prime_ecc/efd/{checked,formats,tests}.rs` | `77d388738a`, `670f20b52a` | `1e85fdf181`, `4c1acb888b`, `3de7339605` |
| `src/cryptanalysis/ic_boundary.rs` | `842036b7ef` | `97b4ad17ea` |
| `src/cryptanalysis/semaev_projection.rs` | `52f3283554` | `40089016d7` |
| `src/cryptanalysis/prime_radical_root.rs` | `cfac756e0f` | `d138c5b4cb` |
| `src/cryptanalysis/prime_root_trace.rs` | `cfac756e0f` | none in its 67-commit history |

The failures are truncations, not one-line typos: e.g. `quartic_ladder.rs` loses the `if pt[i] != 0 …}` body at `mv[I2[i][j]]`, and `prime_root_trace.rs` loses delimiters in its test module. Several affected files are on live lanes: rootfind rounds 114–124 (`prime_*`), prime-ic/MITM (`hec*`), the hyperelliptic lane (`quartic_ladder`, `genus_ladder`), graph-theory IC (`ic_boundary`) and ecbench (`spec`, `ic_framework`).

**Why this cannot be repaired by a single restore commit without coordination.** Restoring each file from its last parsing version drops the feature work that the corrupting merge was landing; those features are still in open PRs (#4933 for the quartic/genus ladder, and the rootfind/hec lanes). Truncating the broken test modules would pass a build by deleting tests, which the repository rules forbid. A global restore would also collide with the conductor scopes those lanes hold.

**Recommended recovery.**
1. Each lane restores its own affected files from the last parsing commit above, re-applies its post-merge changes, and lands the result on its existing PR.
2. ecbench/`ic_framework`/`Cargo.toml` are repaired independently; the two lost `spec.rs` functions (`validate_curve_policy`, `require_current_curve_policy`) and the missing `[[bin]]`/`[[example]]` headers are the concrete first fixes.
3. Add a push-time parse gate: `rustfmt --check` on changed Rust files, or an unconditional `cargo check` that the `scope` job cannot skip on docs-only pushes. The current `Rust all targets compile` `check` job was skipped on the recent main pushes, which is why this persisted.
4. Do not mark this record resolved until `cargo check --workspace --all-targets` and `cargo check --lib --tests --features wide-vars` pass at a main head and the `check` job runs (not skipped) on that head.
