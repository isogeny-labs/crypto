# Source qualification

Local native development, Rust1.100.0 x86_64-unknown-linux-gnu; no external
crate downloads and no research or timing panel. The only dependency is the
unchanged prior trace-kernel crate named in Cargo.lock.

```sh
cargo test --release --locked --manifest-path research/trace_linearized_fibers_20261124/native/Cargo.toml --lib
cargo clippy --locked --manifest-path research/trace_linearized_fibers_20261124/native/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path research/trace_linearized_fibers_20261124/native/Cargo.toml -- --check
```

Status and exact stdout/stderr are separate files. `SHA256SUMS` pins the
new native subjects, the dependency subjects and these execution records.
These are producer records, not an independently signed execution receipt.

The final source qualification uses `tests-v2.*`, `clippy-v2.*` and
`status-v2.json`, after the null-direction-first chart was added. Earlier
unversioned logs are retained as development history; they are not attributed
to the final source. `fmt-v2.*` records the final formatting check.

The six tests cover all A/B/C in GF(8), a bounded GF(128) half-trace control,
trace-kernel ranks over GF(8)/GF(16), all GF(8) four-coordinate chain inputs,
all determinant routes, and ratio normalization over GF(8)/GF(16). The
same-trace property has nonvacuous controls in BOTH trace cosets.
[pcc:unverified]

The first strict-Clippy attempt flagged a parity-expression style issue;
the native method was used before final qualification. No mathematical
control was dropped, no sampled failure was hidden and no research panel
was rerouted as a development test. This library does not search unknown
leaves or verify signed group decompositions.
