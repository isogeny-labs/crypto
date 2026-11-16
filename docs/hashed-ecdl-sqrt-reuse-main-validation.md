# Hashed-ECDL reuse: latest-main integration receipt

2026-10-30. Merge source `339282eb018b93f76a7adafe16601c60ebb8dcc7`
combines publication head `9ff02d52a3fcaef47b5d5574f4af06a8c724afce`
with main `7923f05fcb96bdf653545b09cba2a5d05cd3c744`.
The shared October30 log conflicted after main advanced during validation.
The Copilot reasonable-wait fallback was already explained on PR2906 and
reaffirmed in comment6514448315. Removed only conflict markers and retained
both complete parent line sequences, including newlines, in order. No frozen
file was selected or edited. Main's new panel runner and unrelated work remain.

## Source identity and validation

The [earlier correctness receipt](hashed-ecdl-sqrt-reuse.md) and its raw-log
receipt/manifests are preserved byte for byte. All18 old source entries still
match. Comparing `c84df306c` with this merge, no changed tracked path appears
in the root release dependency files. Root and native Cargo manifests/locks
are unchanged. Thus the full-library binary tested at c84df306c still has
identical library inputs:5430passed,0failed,123existingignored, complete
parent and terminal exit0. The [raw receipt](hashed-ecdl-sqrt-reuse-tests.txt)
records487100bytes, SHA256
`1cccbc7bec2c043d4ca86f16a773447309497a8faf47c8c937169bd20748514d`,
NOT_UPLOADED with no artifact-store identity. It is not a full-root pass of
a different library source, nor a count assembled from child summaries.

The only added native source is `src/bin/hashed-ecdl-panel.rs`. Its three
unit tests pass independently; native strict all-target Clippy, Rust1.78
all-target compilation and formatting pass with this binary included.
The new [19-file source manifest](hashed-ecdl-sqrt-reuse-main-sources.sha256)
has SHA256 `ed03771069fe437d7586d3a88f509c92cf73ae879f3d3ff38b7c88815a5225e5`. Check it from the native crate with
`sha256sum -c ../../docs/hashed-ecdl-sqrt-reuse-main-sources.sha256`.
No additional broad panel was needed because the library inputs did not change.
These are development correctness checks; no experiment, challenge solve,
benchmark, cloud resource or performance claim was added.

## Whitespace and unchanged base evidence

The first `git diff --cached --check` against the old head reports whitespace
in25 incoming research paths. Every flagged staged blob is identical to
main7923f05fc; these are existing generated TeX/fixture logs, outside this PR.
They remain unchanged. Raw diagnostic:17216bytes, SHA256
`a05c0aaade38e69d8cbdd41f7eb1c9714bb028e7ab189be1e82a9b0294890dfb`,
local `/tmp/hashed-ecdl-sqrt-root-main-incoming-whitespace.txt`, NOT_UPLOADED.
It is reproducible with `git diff --check 9ff02d52a 339282eb0`.
The complete staged PR tree against actual main passes whitespace and the
unchanged artifact checker. No gate, waiver, dependency or base receipt changed.

Conductor scope check returned127 (executable absent); documented
warn-and-allow fallback, no shared reservation. Current-head gate,size,artifacts,
actual required reviews/protection and blocking feedback remain merge conditions.

## Compact raw validation output

Hashes and lengths below describe the original unmodified raw bytes. One
separator newline follows each section; elapsed times are not paired speed data.

### cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --bin hashed-ecdl-panel; exit0

Raw bytes:628; SHA256:2460a915d04ac22ed2c9783a781ca3de98a6d6a6dd37f9e8aa60929cc06524db

```text
   Compiling hashed-ecdl-tensor v0.1.0 (/workspace/crypto/crates/hashed-ecdl-tensor)
    Finished `release` profile [optimized] target(s) in 0.83s
     Running unittests src/bin/hashed-ecdl-panel.rs (crates/hashed-ecdl-tensor/target/release/deps/hashed_ecdl_panel-b9850214598f0c14)

running 3 tests
test tests::oracle_checks_full_cardinality_and_all_native_multiples ... ok
test tests::panel_preserves_every_frozen_case_and_separates_answers ... ok
test tests::preparation_is_complete_and_existing_evidence_is_never_replaced ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

### cargo clippy --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets -- -D warnings; exit0

Raw bytes:157; SHA256:6bdfe26054507a7aefdac28dc33fe14a627f5c676d29b336655de63e62a98214

```text
    Checking hashed-ecdl-tensor v0.1.0 (/workspace/crypto/crates/hashed-ecdl-tensor)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

### cargo +1.78.0 check --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets; exit0

Raw bytes:224; SHA256:cef7198d8852b53ef3106737d0f2b07f16d7fa5e7296d15cd38ec8d2681b27a5

```text
warning: unused manifest key: lints.rust.unexpected_cfgs.check-cfg
    Checking hashed-ecdl-tensor v0.1.0 (/workspace/crypto/crates/hashed-ecdl-tensor)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

### cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all -- --check; exit0

Raw bytes:0; SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

```text
```
