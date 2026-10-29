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
`analysis_script`, `execution_history`, mandatory `analysis_provenance` (all
protocol-pinned analysis sources, build receipt and both analyzer binaries),
optional `additional_sidecars`, and
`export_id`. Paths are resolved from the process working directory. All attached
paths in output are absolute; relocation requires preserving or repackaging the
bindings, not silently modifying the write-once export. The retained source files
must match the compiled exporter; their actual bytes and lockfile are attached.
The helper's sibling `../schemas/` must match the embedded schema bytes.

Before freezing new data, `extensions.canonical_packaging.adapter_sha256` must
bind the **native** `adapter-hash`, replacing the historical Python file hash.
This SHA-256 covers the concatenation, in order, of `src/lib.rs`, `src/main.rs`,
`src/protocol_lint.rs`, `src/output_contract.rs`, `Cargo.lock`, `Cargo.toml`, protocol schema and export
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

The CLI attaches the immutable packaging request. `check-export` revalidates all
attachments and reconstructs the entire export, including backend diagnostics,
missing slots and native analysis identity. Public packaging and checking always
recompute the native analysis using the source-linked `native_price` aggregate
implementation, requiring exact output equality. Its input manifest is bound to
the same plans, protocol, instrument binary and record corpus being exported.
All 13 linked analyzer and paired-export source files must match protocol pins.
This is deterministic software replay, not a second mathematical implementation
or proof that the attached binary originally produced the supplied output.

Tests require this crate and sibling `../native_price` and `../paired_export`:
`fixtures/protocol-draft.json` is a
retained draft donor whose only test copies are labeled software fixtures and
frozen in temporary directories. Private unit contracts use a test-only callback
to isolate admission rules. Production integration uses unconditional public
replay with 1,260 planned workloads, 22,680 missing execution slots and zero
solver calls; it rejects mutated controls and cost summaries. Mock build/binary
receipts are explicitly software fixtures. No test is an empirical target, timing observation,
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
