# Reproduce the Rust diagnostics

Run mathematical work in Rust. Python files here only restore/audit source metadata or render recorded values. Use one CPU/thread and fresh output/receipt paths. Frozen scripts refuse replacement of prior calls; never overwrite an existing pre-call record, worker or mathematical output directory.

## Source and build configuration

Check out the PR source and verify the relevant source roster from the repository root. Native integration uses `CANONICAL_SOURCE.sha256`; default-feature root units use `ROOT_UNIT_SOURCE.sha256`. Restore exact tracked literal/manifest-root inputs and require the content audit to pass. The helpers are not full Rust parsers; compiler dependencies remain authoritative. In a sparse checkout, never treat a file left by a failed restore as hydrated merely because it exists.

Recorded correctness builds used Rust1.100.0, release opt-level0, LTO/debug/incremental off,16 codegen units, CPU0, one Cargo job and one test thread. The root-unit retry changes only root-package codegen units to256 and its address-space cap. These are correctness profiles, not performance controls. [pcc:unverified]

```sh
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0
export CARGO_PROFILE_RELEASE_INCREMENTAL=false CARGO_BUILD_JOBS=1
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
cargo test --release --locked --manifest-path crates/certified-relations/Cargo.toml --lib
cargo test --release --locked --manifest-path crates/certified-relations/Cargo.toml --lib --features count-field-ops
cargo test --release --locked --manifest-path crates/relation-replay-checker/Cargo.toml --lib
cargo test --release --locked --manifest-path research/relation_root_runtime_20261127/harness/Cargo.toml --tests --no-run
```

The native harness depends on the actual full crypto library with default-features=false, matching the historical relation integration. The whole default-feature unit build is separately declared and never substituted by the harness.

Copy each compiled qualifier to a new path, remove write permission, hash it before calls, list its tests, and retain complete stdout/stderr/exit. Set `RELATION_ROOT_INTEGRATION_OUT` to a fresh directory for the original matrix and `RELATION_PAIRGRAPH_OUT` for the matched checker matrix. Their test functions enforce fixed prime/n/seed/graph rosters and all mutation assertions. Build/run scripts and JSON plans contain the exact recorded caps, flags and paths.

## Replay stored native proofs without rebuilding the root

Verify archive SHA sidecars from the repository root, then extract each archive only into an empty directory. The qualifier archive contains native executables: inspect the supplied source/hash records before executing them. Hashes bind bytes; they are not trusted execution signatures.

`native-canonical.tar.gz` contains `root-native-canonical-original-20261127/` and `root-native-pairgraph-canonical-20261127/`. `qualifiers-final.tar.gz` supplies `baseline-audit` and `pairgraph-audit`; their corresponding source plans identify all mathematical checks.

```sh
baseline-audit ORIGINAL_MATRIX reference-inputs REFERENCE_PINS.json NEW_ORIGINAL_AUDIT.json
pairgraph-audit PAIRGRAPH_MATRIX reference-inputs REFERENCE_PINS.json NEW_PAIRGRAPH_AUDIT.json
```

Use absolute paths or run from the study directory. Output reports must be new. The second auditor replays complete original-ideal/containment/reduced-basis/strict-S-pair proofs and binds native basis and coverage metadata. It does not independently execute native operation counters. Source arithmetic is the maintained separate checker; the new frontend consumes the new 108-pair artifact schema. Its five sabotage views are in memory and leave original files unchanged. [pcc:unverified]

## Pruning qualifier

Build `pruning-v2/Cargo.toml`, then freeze/hash the normal driver and libtest worker separately. The qualifier's fixed manifest is derived from `FIXED_INPUTS_412.json` by changing paths only; keep every original digest, prime, n and case ID. Relocate paths in a NEW manifest, leaving `PRUNING_INPUTS.json` frozen. The driver requires all six cases and rejects changed input hashes or an incomplete roster.

```sh
joint-pruning-stream scan NEW_RELOCATED_MANIFEST.json NEW_OUTPUT_DIRECTORY
joint-pruning-stream replay NEW_RELOCATED_MANIFEST.json NEW_OUTPUT_DIRECTORY
```

Replaying the supplied complete archive uses `replay` only. The driver independently replays all original memberships and bounded derivations through `degree_space::replay`, reconstructs sequential deletions, checks every accepted cap and optional augmented label rank, recomputes the entire summary and rejects summary/decision canaries. Reports are gzip JSON with 512MiB plaintext/128MiB compressed per-case limits. Censors and skipped rejected-trial caps are never called exact gains.

The root API module `certified_relations::prune` is the promoted byte-identical first-obstruction implementation; the standalone qualifier retains its own frozen source path. Full-cap v1 remains an explicit control and interrupted historical experiment, not a complete fixed-six admission.

## Reports and gates

`render-report.py` lays out existing Rust JSON values only. It creates standalone scientific figures and a two-page XeLaTeX report; inspect every rendered page. No renderer output supplies a missing measurement or proof.

Run actual repository parity with `python3 tools/check_lab_parity.py --repo crypto` and the research gate with `--repo-name crypto`; a RAM checkout's directory basename is not the repository name. Do not change gates, keys or frozen records. The current branch has no new PCC claim IDs.
