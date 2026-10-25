# Evidence, validation and reproduction

The new executions are independent of the unlocated historical runs. The frozen original audit remains in `../three_leads_audit_20261018/`; its source receipts provide the binary trace and PKM references. [TRANSFER.md](TRANSFER.md) supplies the explicit genus-2 construction and published geometric hypotheses. [CLAIMS.md](CLAIMS.md) gives the disposition of every original assertion.

## Retained files

- `results/cost-summary.json`: every cohort, curve, candidate/reference and complete declared price interval. `dashboard.json` contains the native-selected presentation rows; those best-curve selections are descriptive and post hoc.
- `results/genus-fixture-sizes.csv`: exact field and subgroup bits, orders and cofactors for all 42 genus models. Specs retain all 60 targets per model and their public generation seeds.
- `results/coverage-all.jsonl`, `coverage-summary.json`, `coverage-cells.csv`: finite primary and supplementary coverage, actual-point matched controls and curve-resampling intervals. `random-x-controls.jsonl` and `random-x-null/` retain the supplementary uniform-x null separately.
- `results/replay-summary.json`: native validation of every audit receipt, 42,862 successful measured replays, all 134 exhausted diagnostic re-executions and the ten A/A measured replays. All 945 study sessions are sealed and complete.
- `results/retired-instruction-summary.json`: same-binary, valid unmultiplexed user-space PMU pairs, with paired curve/target bootstrap intervals (4,096 resamples). These instrumented instruction counts include native and accounting overhead. They do not turn L0 walls into isolated measurements. The five-pair instruction A/A control is separate.
- `certificates/`: exact polynomial cover identity and independent small zeta/order checks. The integer witness also passes the repository's identity checker; this is not a new formal proof of the isogeny theorem.
- `correctness/`: complete finite oracle census, map checks, source/binary hashes, build/test attempts, rejected L2 preflights and control receipts. Failed development attempts remain identifiable.
- `coverage-fixtures-v3/`, `decomposition-fixtures-v2/`, `genus-fixtures-v1/`, `genus-fixtures-v2/`, `genus-order-fixtures-v1/`, `trace-filter-fixtures-v1/`: exact models, public subgroup/order witnesses, seeds, cap policies, exclusions and method configurations. Field-indexed and subgroup-indexed cohorts are separate experiments.
- `runs.tar.xz`: deterministic archive of all 945 sealed raw session trees, logs, replay receipts, individual report PDFs/SVG/150-dpi PNGs, the 945-page report bundle, all 134 exhaustion replays (plus the failed seed-parsing attempt), both superseded coverage generations, all interim analyses, the original v1 diagnostics and the extra A/A session. Archive members retain their original study-relative paths. Raw trees are ignored after archiving; no committed historical tree was moved.
- `evidence/source-snapshot-v4.tar.zst`: measured-engine snapshot, including the initial failed packaging attempt and its explanation. The final source is committed in this PR; the later input-validation guards, trace-filter configuration and reporting do not change the old method counters, as all old replays establish. Frozen binary hashes remain in `correctness/`; executables themselves are build products.

## Extract and verify

The immutable archive is stored through the adjacent `.artifacts.json`
manifest. From the repository root, hydrate the exact bytes first:

```sh
python3 tools/artifacts/artifacts.py pull research/three_leads_e2e_20261018
```

Then, from this study directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

The extracted `reports/INDEX.md` indexes every individual report; `session-reports.pdf` contains one page per study session. `instruction-AA-report/` contains the separate one-page A/A report and plot. All main plots and report PDFs have SVG/150-dpi PNG counterparts or rendered inspection pages. The main report and plots are mirrored under `../../benchmarks/20261018-three-leads-e2e/`.

From the repository root:

```sh
cargo build --release --offline --bin ecbench
cargo build --release --offline --manifest-path research/three_leads_e2e_20261018/native/Cargo.toml
research/three_leads_e2e_20261018/run-batch.sh replay
```

The replay script's default frozen-binary path is a local build artifact. Set `STUDY_BINARY=target/release/ecbench` to use a new source build. For a single session:

```sh
target/release/ecbench verify --dir research/three_leads_e2e_20261018/sessions/order-genus-40 --replay-all
```

The ecbench standard replay skips exhausted records. The native `replay-exhaustions` command separately re-executes all 134 diagnostic exhausted records using decimal algorithm seeds; its exact command and receipts are retained. Audit verification does not re-run PMU counts or wall times: it checks deterministic scalar, primitive, phase and native counters. No claims depend on reproducing incidental timing noise.

Native `analyze-sessions`, `analyze-coverage`, `instruction-AA` and `report` commands consume the extracted inputs. `run-batch.sh` creates new sessions; choose fresh output paths rather than overwriting sealed directories. `pack-runs.sh` recreates `runs.tar.xz` byte-for-byte using sorted paths, fixed 2026-10-18 timestamps, normalized ownership/mode, GNU tar and single-thread xz -6. The archive hash and member listing are committed.

## Validation and limitations

Full post-integration release library tests pass: 4,414 passed, 118 intentionally ignored, zero failed. All 16 ecbench integration tests pass. The standalone engine and identity-checker tests pass, and strict clippy passes for the new standalone package. Repository-wide strict clippy stops on the unchanged PMAC expression at `src/symmetric/modes/pmac.rs:112`; the historical and working hashes are retained. It was not suppressed or changed as part of this research.

Both L2 timing preflights rejected host contention. The instruction counter checks use same-binary pairs with positive counts, no multiplexing and no counter error; the A/A control confirms low instruction variation, not isolated wall latency. No fastest-external-implementation, optimized-Jacobian lower bound, deployed-curve, or universal-genus-2 ranking claim follows.

All primary cold comparisons charge destination construction and transfer, base construction/projection, initialization, relation roots/lifts, scalar/graph elimination, consumed unsuccessful work, restarts and source verification. Explicit fixture generation, group-order certification and diagnostic exhaustive log tables are separate; diagnostic tables never enter measured instances. Rho collision-table and IC graph resources remain recorded. Logical native counters omit some real allocation/hash costs, hence the arithmetic tables remain outside fully priced speed rankings. Complete instrumented retired instructions provide a separate implementation measure.

Publication checks also pass all 49 site tests, all 11 native cover-checker tests, the five identity tests, the leaderboard rebuild/check, both 218-input workflow path checks and the generated panel index. The refreshed native cover catalog has 2,771 verified models and zero invalid/unsupported inputs. An independent older transfer-assessment script still asserts a 2,100-model catalog even though the pre-study main branch already had 2,572; that pre-existing mismatch is preserved in `correctness/baseline-cover-*`. Repository-wide `cargo fmt --all --check` encounters missing sparse inputs and unrelated existing format differences; all touched Rust files and the new standalone package pass their format checks.

The refreshed alias index preserves every existing alias. Full name lint then reports five pre-existing retired spellings in two unchanged 2026-09-21 reports; `correctness/baseline-name-lint-files.diff` is empty against main and the exact lint output is retained. The final focused root engine/adapter check passes nine tests. None of these upstream failures is hidden by weakening a gate.

Final native `curve_traits check` reproduces the refreshed file byte-for-byte (2,947 records). Every one of the 2,748 previous trait records is unchanged; 199 are added. The final leaderboard roster also names the field degree for all 1,371 extension models, accepting both historical `k` and new `degree` metadata. Artifact checksums in `SHA256SUMS` are checked from the repository root; the raw archive has its own study-relative checksum file.
