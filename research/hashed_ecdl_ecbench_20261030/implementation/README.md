# Development validation and source custody

This directory contains software checks and preflight metadata, not scientific
measurements. Test elapsed times are not compared or used as benchmark results.

All native package checks pass: **90 tests in crypto, 96 in cryptanalysis**,
formatting, strict all-target clippy and Rust 1.78 all-target checks. The
new observer/default-policy control preserves the entire partial certificate,
RNG state and metered work. Crypto's selected ECBench CLI target passes
**12 tests** and the full ECBench integration file passes **26 tests** after
restoring its original tracked bitop-vector fixture. Native success and
exhaustion records are independently checked and exactly replayed.

Before the final witness-phase review correction, the broad root run executed
all **5,660 registered tests**: initially 5,511
passed, 25 failed for absent tracked runtime inputs, and 124 existing tests
were ignored. Restoring the original inputs and rerunning **exactly those
25 tests on the same binary** gives 25/25 passes. Thus all 5,536 active tests
were checked, but there was no second single all-green broad invocation.
The final correction changes only phase attribution and adds regression
assertions to an existing selector control. The affected native suites,
root adapter and entire ECBench CLI/integration file are rechecked afterward;
the broad invocation is not represented as a run of that later source.
The initial failure log, exact failed-name list, original input identities
and successful targeted rerun are retained. No fixture was rewritten, no
assertion disabled and no ignore added. The device-source controls use the
existing CPU emulator; they do not qualify GPU execution or a benchmark.

Representative exact commands:

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo test --locked --release --bin ecbench --test ecbench
cargo test --locked --release --lib
cargo clippy --locked --lib --bin ecbench --test ecbench -- -D warnings
cargo clippy --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
# Rust 1.78.0, all native targets
cargo check --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets
bash taskq/deploy/eks-karpenter/test-checkpoint-source.sh
```

The final full integration pass was also invoked through the exact compiled
Rust test binary after immutable fixture restoration, with two test threads.
Source and binary identity did not change between the failing and passing
fixture checks. Initial compile attempts needed tracked bin/include inputs
and 32 compiled frozen fixtures materialized in the sparse checkout. A broad
metadata-size walk and an overbroad restore were stopped; the final restoration
uses only the explicit original input list. These development attempts are
not substitutions for the planned native measurement.

The prerequisite methods-file repair preserves both original test modules
byte-for-byte from merge `12d82e2d99d52f6391898583c1856ec6a25572b6`'s parents
`ccb75292fa3ce16e84085b3d454eb93f4829904c` (strong negation) and
`1e11a12d66a14a41f50ae83dd779839346243a47` (tensor). Both complete modules and
all their assertions are restored; their seven new/restored/native adapter
controls pass in the broad run. The parse failure is retained. Source hash
manifests qualify the actual final native package and runner/control sources.

Checkpoint infrastructure has seven mocked controls: valid readback passes;
wrong head, parent path, dirty source, untracked source, wrong account and
tampered readback are rejected. The source-selected checkpoint executable is
never run. `actionlint` validates both workflow definitions, and the helper
passes Bash syntax checks. This does not prove real S3 authorization. Actual
local STS returns NoCredentials; repository variable introspection returned
403, which does not show that the existing configured OIDC role is absent.

The canonical protocol checker accepts the protocol with zero findings; it is
frozen in a separate metadata commit before affected data. Its first-run bootstrap limitation
is explicit: no matched native runtime preflight exists yet; 1,800 seconds is
the original job-budget ceiling, not a measured prediction or power estimate.
The plan stays fixed and no local test timing is used as measurement evidence.

The connected GitHub API supplied the older actual image/launcher log after a
CLI redirect denial. The older source's build/publish succeeded, then launcher
exit 23 had a job UID but no pod UID/results URI. That is retained as historical
infrastructure evidence, not current native admission or a scheduler diagnosis.

The one-page engineering PDF and vector diagram were rendered and visually
inspected. The paired mirror's first store check rejected its 88,055-byte
render; PDF optimization produces 42,226 bytes with the text/figures visually
rechecked. The build recipe reproduces the optimization. Raw source/test
receipts are unchanged. Original frozen experiments, reports and leaderboard
graphs are unchanged; this is an additive engineering/pre-data handoff.

Review finding [1325#discussion_r4425961315](https://github.com/isogeny-labs/cryptanalysis/pull/1325#discussion_r4425961315)
identified independent selector-witness replay charged to search. Both copies
now switch to `witness_check` inside the oracle wrapper before membership and
group-sum replay, and charge row assembly to `matrix_build`. Existing selector
controls assert replay scalar calls occur in the checking phase and that row
assembly has no group calls. Cold phases remain distinct; both map to the
canonical online relation-check interval. Totals and solver choices are unchanged.

`pre-review-*.sha256` bind the earlier checks; current `native-source.sha256`
and `runner-control-source.sha256` bind the final review correction.
The full broad log is losslessly line-split and compressed as
`root-lib-tests2.part-*.txt.gz`: concatenating their decompressed bytes in name
order reconstructs the original hash in `broad-root-log-original.sha256`.
No lines are omitted. `raw-receipts.sha256` covers the retained raw receipts.
The development test binary hashes refer to the pre-correction broad run;
final affected-check binaries are recorded separately when available.

Concurrent mirror commit017a1d1 added a stronger certificate/RNG/work phase
control. The ordinary push rejection preserved it; final mirror merge91116f8
keeps both parent histories, complete ordered journal snapshots and both
controls. The final shared finite-extension file now has11 passing cases in
each repository, followed by fmt, strict clippy and MSRV all-target checks.
The earlier full90/96-suite receipts precede this one additional test; no
second full91/97 invocation is claimed. Final native/runner source manifests
bind the current code, while all earlier identities remain separate.

The protocol build argv matches the actual maintained static-musl producer:
Rust1.99.1, release, locked, no default features, x86_64-unknown-linux-musl,
RUSTFLAGS=-C target-cpu=x86-64. Local GNU development binaries do not replace
this later authenticated scientific binary/image qualification. ECBench's
canonical spec identity in plan-summary.json hashes identity fields, while
S3 receipts hash the exact input file bytes; those distinct hashes are retained.
