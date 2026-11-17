# Strict-input integration with approved main changes

2026-10-30. The input-fix snapshot `6eb7175e38a3488f305da3faee30703e3c325961`
matches merged cryptanalysis PR1303, including its 17-file manifest
`f3ab5c7f5e2aa83b47c40b5cb43d150d9c9fd5b9ed24fef66939965167df24b4`.
That snapshot's full crypto release library passed 5389 tests with zero
failures and 123 pre-existing ignored tests; its compact receipt records the
interrupted earlier attempt and the successful uncapped retry separately.

Meanwhile, crypto main `986365365ac2fcb587ef421aadfdf6ed4bb546a0` includes
approved primality-certificate and canonical-witness changes (`c866ee97f`),
plus a bounded-reference base-construction hook. Preserve these additions.
They are not part of the input-handling improvement and were not ported into
cryptanalysis by this task. Whole-crate byte parity with that older sibling
snapshot therefore is not asserted after main integration.

## Scoped integration checks

A read-only merge preview of main with input-fix/docs head `0fb9c825f`
produced native crate tree `972d7fabe201d4749d224d9f9bb9e348d02306ea`.
Native code merged cleanly. Only the unsealed October30 session log had an
add/add conflict; its resolution is separate from this code validation.
The clean native subtree was exported to an isolated scratch directory;
the actual branch and the log conflict were not edited by this preview.

| Check against the combined native code | Actual result |
| --- | --- |
| `cargo test --locked --release` | exit0: 34 core, 5 runner and 3 strict-JSON tests passed; [transcript](hashed-ecdl-tensor-input-integration-tests.txt) |
| `cargo clippy --locked --all-targets -- -D warnings` | exit0 |
| `cargo +1.78.0 check --locked --all-targets` | exit0 |
| Current-main bounded compatibility module, standalone `rustc --test -O -D warnings -D missing-docs`, linked to that native crate | 10 passed, including the new invalid accelerated-base rejection test; no GPU execution |
| Rebuilt runner, replay existing stored receipt | exit0; independently verified scalar4 |
| Tiny synthetic input, duplicate proof `witness` | exit1; duplicate field rejected before curve preparation |
| Tiny synthetic input, duplicate nullable `prime_certificates` | exit1; duplicate field rejected |

The two directly shared input implementation files, `src/serialization.rs`
and `tests/strict_json.rs`, remain byte-identical to cryptanalysis PR1303.
Curve and runner files also retain the approved crypto-only additions; the
same typed visitors cover their new optional proof records. Verifier equations,
dependencies, frozen evidence, validation gates and challenge inputs are
unchanged by this improvement. No challenge solve, experiment, cloud resource
or performance claim was made.

The [18-file integration manifest](hashed-ecdl-tensor-input-integration-sources.sha256)
has SHA256 `bbeb4d5ae4b8f3fe549ce5ea47d1e89ca66504e68670a2b3a5bfc374222ca869`.
It pins the combined native subtree above and can be checked from the crate
directory once that main integration is present. The original 17-file input
manifest and earlier manifests remain unchanged and pin their named snapshots.
These are native integration checks, not a claimed full-root rerun of the
later main additions. The full-root result remains explicitly tied to its
tested input-fix snapshot.

An initial check started before a lazy archive export finished and twice
exited101 at manifest lookup, before any tests ran. The startup diagnostic
is retained locally at `/tmp/hashed-ecdl-input-integration-startup.txt`:
`error: manifest path .../strict-input-integration/crates/hashed-ecdl-tensor/Cargo.toml does not exist`.
The export subsequently completed successfully; exporting just the native
subtree and awaiting completion preceded all passing checks listed above.
