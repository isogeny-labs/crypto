# Rust integration regression repairs

Follow-up to PR #2101 at parent
`c78a36dbefa761b9cdb9ae122a837d15c564629b`. The requested scope now includes
repairing the four library failures and two maintained-target lint findings
previously recorded as baseline blockers. The earlier
[validation record](../rust-extensions-20261017/README.md) and its failed logs
remain historical evidence.

These are local correctness, metadata-rebuild and integration checks. No new
research experiment, timing campaign, factor-hint search or paid worker is
part of this repair. The registry, factor hints, source manifest, source
receipts and calibrated cost-model parameters are unchanged.

## Cause and repair

| Requirement | Repair and evidence |
| --- | --- |
| Strict traits deserialization and queries | The committed catalog was `curve-traits/1`, while the existing generator and types require `/2`, including input digests, source provenance and volcano levels. Rebuild with the native generator and the same committed inputs and factor budget. Add a regression check binding the schema and all six input hashes, with explicit compile-time dependencies for sparse CI. No field defaults or proof certificates are invented. |
| Justified Jacobian classification | The genus-8 route for c2pnb304w1 over GF(2^76) costs 160.259 log2 bit operations under the existing calibrated model, against source rho at 160.226. Its ratio is -0.033, hence `Boundary`. Replaying the superseded 640-multiplication price for genus 8 alone restores the old +2.001 ratio and `FormulaMarginal`; the calibrated 2620.9 price adds log2(2620.9/640) = 2.034 bits. A regression test checks geometry, solver, both prices and the classification change. These are formula outputs, not measurements. |
| Current Jacobian consumer data | The old JSON also lacks required cost-model fields, preventing CLI queries and its CI rebuild check. Rebuild the current derived catalog with the existing model, preserving its 2,375 curve identities and 5,974 routes. The table renderer retains the documentation's conditional-carriage wording. |
| Maintained-target Clippy | Remove the non-Drop closure's unnecessary explicit `drop` in the live volcano driver; Rust's borrow analysis ends the borrow at its last use. Use `is_multiple_of(64)` in the maintained N83 example and apply CI's pinned formatting. Existing sealed campaigns, receipts, captured source copies and measurements are untouched. |

The Jacobian catalog rebuild produces 598 `formula_advantage`, 57
`formula_marginal` and 1,720 `boundary` curve verdicts. These are outputs of
the already implemented formula model. Existence, subgroup-map and solver
obligations remain as stated in the catalog and documentation. No measured
speedup or cryptanalytic result follows from refreshing this metadata.

The old/new projections of every curve's `slug`, `field`, `trace`,
`subgroup`, and every route's `kind`, `destination`, `transport` compare
byte-for-byte after `jq -S`. Both projection SHA-256 values are
`a58189b1f48fe201c5c3476d97b98ff82ec2d1a236287249bdbae20f777c18c6`.
Cost-derived annotations, conditions, prices and verdicts are regenerated,
not included in that unchanged-geometry assertion.

## Validation environment and commands

Linux x86_64, locked Cargo dependencies, Rust 1.98.0 and 1.99.1,
`CARGO_BUILD_JOBS=2`; the full library run uses `RUST_TEST_THREADS=2` and
`RAYON_NUM_THREADS=2`. Native catalog rebuilds use `RAYON_NUM_THREADS=2`, the
existing per-composite budgets (traits 262,144; transfers 200,000), and a
600-second outer limit for builds/positive consistency checks. The existing
CI altered-file rejection controls have a 1,500-second outer limit, matching
their 25-minute PR job budget. Outputs are first written to scratch;
only successful builder output is copied to its current catalog path.
CI requests the floating `1.98` series in several jobs; the captured
Conductor CI log resolves it to 1.98.1. Local toolchain results here name
the exact versions tested, and do not substitute for final-head CI.

```sh
cargo +1.98.0 build --locked --release --bin curve_traits --bin jacobian_transfers
RAYON_NUM_THREADS=2 target/release/curve_traits build --out /tmp/regression-repair-traits-v2.json
RAYON_NUM_THREADS=2 target/release/jacobian_transfers build --out /tmp/regression-repair-jacobian-current.json
RAYON_NUM_THREADS=2 target/release/curve_traits check
RAYON_NUM_THREADS=2 target/release/jacobian_transfers check
target/release/jacobian_transfers table --write-doc docs/curves/JACOBIAN_TRANSFERS.md
cargo +1.98.0 test --locked --release --lib
cargo +1.98.0 test --locked --release --bin ic --bin volcano-ic-driver --bin curve_traits --bin jacobian_transfers --test ic_framework --test ic_progress --test rust_extension_security --example ic_custom_pipeline --example n83_precomputed_rho
RUSTUP_TOOLCHAIN=1.98.0 bash scripts/clippy-maintained.sh
RUSTUP_TOOLCHAIN=stable bash scripts/clippy-maintained.sh
cargo +stable check --locked --workspace --all-targets
cargo +1.98.0 test --offline --manifest-path crates/crypto-analysis-core/Cargo.toml --lib --target-dir /workspace/crypto-analysis-core-validation
```

Full library tests need network permission for their synthetic loopback peers.
No checks are disabled, schemas weakened, tests ignored or assertions removed.
The Jacobian expectation is corrected using the documented model and the
counterfactual regression above; no cost constant changes to obtain green.
Conductor scope checks were attempted before editing and scope expansion;
the executable is unavailable (exit 127) and no workflow is tracked here.

## Results

| Check | Observed result |
| --- | --- |
| Full release library, Rust 1.98.0 | **4,393 passed, 0 failed, 117 ignored**, 0 filtered out; exit 0. The 117 existing opt-in tests remain unchanged. |
| Selected release consumers | **53 passed, 0 failed**: 10 IC binary, 25 IC CLI, 5 progress, 7 extension/security, 2 embedding-example and 4 N83 example tests. The other three selected binaries contain no unit tests; their compilation is not counted as functional tests. |
| Full maintained-target Clippy | Rust 1.98.0 and 1.99.1 both pass with `-D warnings`, exit 0. |
| All workspace targets, Rust 1.99.1 | `cargo +stable check --locked --workspace --all-targets` passes, exit 0. |
| Existing lightweight analysis crate | 10 tests pass, exit 0; its source is unchanged. |
| Pinned formatting | All five changed Rust files pass Rust 1.98.0 rustfmt, exit 0. |
| Catalog consistency | Both native `check` commands return 0; the generated Jacobian table matches the committed Markdown. |
| Altered-file controls | Both existing CI mutations are rejected with validator exit 1, as expected; neither command times out. |
| Broader naming CI | The unchanged official text linter reports **4,042 findings on clean main c6d64436 as well as the published PR head**. This is not a passing gate. |

The managed workspace restarted during validation. Code, generated catalogs,
staged changes and the evidence already copied here survived; `/tmp` logs and
running jobs did not. The first full library run was incomplete, even though
the four formerly failing tests had passed. The release suites, both final
Clippy runs and all-target checks were rerun with logs in `/workspace` and
are the completed results above. See `pre-restart-tool-receipts.json` for
the observed catalog-check and tamper-rejection results whose temporary logs
were lost; that file is explicitly a tool-result receipt, not a raw log.

Raw completed logs are retained as `*.log.gz`; `ci-*.json.gz` retain the
decoded GitHub job logs and their exact head/job identifiers. To extract a
CI log, use `gzip -cd FILE.json.gz | jq -j .decoded_log`. `SHA256SUMS`
binds retained evidence, `source-files.sha256` binds the code and catalogs,
and `source-patch.sha256` binds their diff against parent c78a36db.

## Remaining shared CI gates

At the checked published head c78a36db, CI reproduces the stale Jacobian
expectation repaired above. Other failed jobs have distinct causes:

- Four historical replay jobs copy frozen dependency locks over the current
  manifest and fail `--locked`. Those frozen locks and hashes are preserved.
- Conductor-gap controls omit `tools/volcano-cli/Cargo.toml` from their sparse
  checkout. Their later steps include fresh measured controls; none were
  launched locally in this maintenance task.
- The naming gate's 4,042 findings (including alias-map drift) also occur in
  an isolated, unchanged main worktree. The registry, alias map and linter
  are byte-identical to main.
- The leaderboard's three generated files are stale. Backlog validation
  rejects `XP-20261017-quintic-support-frobenius` because `{artifact}` has
  no value; the lab job's three failures include that invalid card.

The affected manifests, frozen locks, workflow definitions, research inputs,
alias map and leaderboard builder are unchanged by this PR; their CI logs
and the independent main naming control are retained. These gates are
reported, not disabled or treated as passing. See the
[shared regression record](../../../problems/current-main-research-regressions.md)
for exit conditions. The PR stays open until its actual final head's
applicable checks and review gates pass.

Whole-crate side-channel resistance remains unestablished; see
[the timing coverage problem](../../../problems/rust-secret-api-timing-coverage.md).
