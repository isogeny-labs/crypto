# PR #2924: final combined-source qualification

Main advanced after the [first integration](../refresh-20261030T0629Z/README.md).
This additive receipt qualifies the actual source after preserving its new
diagnostic counters. The earlier receipt and original frozen report are intact.

- First refresh: `2d94e666671e1c9aa7a24b6c04373b392077d1f3`.
- New merged main: `954c0bb023f33657fe3cb6b95720f572655d0b12`.
- Resolved index tree before this receipt and final journal entry:
  `0e13a632fb6426b60e2a565e161f1616d87d5031`.
- [Tested source manifest](native-sources.sha256): all 40 tracked native package
  files. The final uploaded commit is recorded on
  [PR #2924](https://github.com/isogeny-labs/crypto/pull/2924).

Between the two main pins, the native changes are limited to `arithmetic.rs`
and `lib.rs`: square-root requests and curve-addition diagnostic counters.
They are preserved. Only the append-only journal conflicted; its resolution
keeps all 2,504 first-parent and 2,605 incoming-parent lines in order, checked
in [log-preservation.txt](log-preservation.txt). The original hash-prime feature
and the two existing complete journal histories are retained. No rebase, force
push, source-side selection or frozen evidence replacement was used.

All four development commands exited 0 on this source:

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo clippy --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --locked --all-targets -- -D warnings
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
# Rust 1.78.0
cargo check --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --locked --all-targets
```

The full suite passes **82 tests**: 51 library, 5 runner, 3 source-panel,
6 checked-hash-prime, 1 factoring CLI, 9 finite-extension, 3 strict-JSON and
4 width-extension tests. Raw logs are the four lossless `*.log.gz` files;
[artifact hashes](ARTIFACTS.sha256) and [uncompressed hashes](uncompressed-logs.sha256)
pin their bytes. The entire source manifest was checked again after testing.
Elapsed test diagnostics establish no runtime gain.

Compared with this main parent, native changes remain only the four original
feature source files and the original fixture, preparer and integration test.
Main's `lib.rs`, strict JSON, finite/width/partial/binary, factoring, weighted,
primality and contraction code are byte-identical. Original frozen study
artifacts, source snapshots, protocols and previous validation receipts remain
unchanged. The PR diff passes `git diff --cached --check MERGE_HEAD`.

The first receipt records the Copilot fallback, protection inspection and
external review-service limitation. Fresh `gate`, `size`, `artifacts`, every
applicable check and blocking review still govern guarded MERGE. Verify the
actual merge parents/tree and main reachability before retiring tracking.

No scientific run or benchmark was launched, and no runtime speedup or challenge
scalar is claimed. Profiling, the original frozen matched A/B panel, binary
challenge pipeline, both scalars and the independent general audit remain open.
Fresh current-source S3 checkpoint/readback and authenticated native image/
execution admission remain prerequisites for new scientific work. This
tracking task does not edit or launch the separate ECBench integration study.
