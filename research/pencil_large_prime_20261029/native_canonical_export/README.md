# Native canonical observation exporter

Supported replacement for the retained historical `../canonical_export/package.py`.
All packaging, hashing, exact plan checks, schema validation, semantic checks and
write-once output are Rust. It executes no experiments and invokes no Python.
The Python helper is retained as reference text in `tools/`; its two canonical
schemas are vendored byte-identically. `jsonschema` 0.33 validates the actual
Draft 2020-12 schemas in process, with remote retrieval disabled.

```
cargo +1.99.1 test --release --locked --manifest-path Cargo.toml
cargo +1.99.1 run --release --locked --manifest-path Cargo.toml -- adapter-hash
cargo +1.99.1 run --release --locked --manifest-path Cargo.toml -- validate-protocol PROTOCOL.json
cargo +1.99.1 run --release --locked --manifest-path Cargo.toml -- REQUEST.json --out NEW-export.json
cargo +1.99.1 run --release --locked --manifest-path Cargo.toml -- check-export NEW-export.json --protocol PROTOCOL.json
```

The request retains `pencil-canonical-package-input/v1`: hash-bound `protocol`,
`canonical_tool` (reference text, never executed), `native_analysis`,
`analysis_script`, `execution_history`, optional `additional_sidecars`, and
`export_id`. Paths are resolved from the process working directory. All attached
paths in output are absolute; relocation requires preserving or repackaging the
bindings, not silently modifying the write-once export. The retained source files
must match the compiled exporter; their actual bytes and lockfile are attached.
The helper's sibling `../schemas/` must match the embedded schema bytes.

Before freezing new data, `extensions.canonical_packaging.adapter_sha256` must
bind the **native** `adapter-hash`, replacing the historical Python file hash.
This SHA-256 covers the concatenation, in order, of `src/lib.rs`, `src/main.rs`,
`src/protocol_lint.rs`, `Cargo.lock`, `Cargo.toml`, protocol schema and export
schema. The separate `canonical_tool_sha256` binds the reference helper.
This change is an explicit implementation identity amendment, not approval of a
protocol or an inference that any recorded experiment is qualified.

The cold clock is `time.method_call_wall_ns`, never `solve_wall_ns`.
All five online phase names, exact integer sums and explicit error-free clock
diagnostics are checked and retained as observations. Every exported row keeps
`wall_valid:false`. Integer counters above 2^53 are decimal strings, including
values wider than u64; no floating-point conversion touches counted work.
Clock and index values outside the supported u64 domain fail closed. Seconds
are formed as exact decimal nanoseconds; RSS multiplication is checked.

Every observed record needs explicit known attempt/restore counts and hash-bound
evidence. Restores need a checkpoint generation. Missing slots remain a backend
sidecar and are never fabricated as attempted rows. Resource sidecars and raw
outcomes remain attached. Incomplete cohorts produce a valid observation export
with semantic shortfall findings and a nonzero CLI exit. Output creation is
exclusive. A schema/identity/history failure emits no export.

Tests are self-contained within this crate: `fixtures/protocol-draft.json` is a
retained draft donor whose only test copies are labeled software fixtures and
frozen in temporary directories. The analysis source fixture is reference text;
it is never executed. No test is an empirical target, timing observation,
protocol approval or measured candidate/rho result.

See `CONTRACT_COVERAGE.md` for canonical semantic obligations and explicit
unsupported cases. Historical Python files and failed validation logs remain
unchanged. Final source/protocol/binary qualification and actual experiment
execution remain separate obligations.

`validate-protocol` checks the actual canonical protocol schema and applicable
semantic rules without changing or freezing the input. A null preflight estimate
is allowed on a draft, matching the canonical helper; it is refused on a frozen
protocol. Existing analysis script bytes are checked against their declared hash.
The success receipt explicitly retains `launch_authorized:false`.
