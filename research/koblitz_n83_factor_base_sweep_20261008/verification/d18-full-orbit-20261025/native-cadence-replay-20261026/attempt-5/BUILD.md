# Clean-source native build for attempt 5

- Frozen source commit: `45ce91d503ee0d0acc57e3d8d5cb49108c90ded6`.
- Clean sparse checkout: `/private/tmp/n83-source-45ce-20261026`; `GIT_NO_LAZY_FETCH=1 git status --porcelain` was empty before and after the build.
- First clean-target build exposed four omitted `include_str!` inputs under `docs/curves`, `docs/ecbench`, and `docs/ic`. `git sparse-checkout add` materialized the tracked blobs at the frozen commit; no source files were edited. The first build's nonzero exit is a sparse-materialization failure, not an algorithm run.
- Successful command from that checkout: `cargo build --release --offline --jobs 2 --target-dir /private/tmp/n83-source-bound-cargo-target-20261026 --example koblitz_n83_factor_base_export --example n83_d18_native_relation_replay`.
- Build result: exit 0, `Finished release profile [optimized] target(s) in 3m 43s`; Homebrew Cargo and Rust `1.93.1`, Darwin arm64.
- Worker SHA-256: `d15ecfefb86baaf351535a54f5bbcdcd10a4ded39fe3c75c765c67bd4dfe040e`.
- Separate verifier SHA-256: `e89578703888568d67f79e5b2645e47b911a30a9d7a8a3d57e795be997296b71`.
- `strings` on the worker contained `/private/tmp/n83-source-45ce-20261026`, the path read by its compile-time `CARGO_MANIFEST_DIR` guard. The worker's started phase must independently report the new binary hash and frozen source commit before its result is interpreted.

The absolute checkout and target paths identify this local build; the source commit, command, toolchain, hashes, and worker phase receipt are the portable identity checks for replay. The producer object and fresh GET receipts are recorded separately with the run.
