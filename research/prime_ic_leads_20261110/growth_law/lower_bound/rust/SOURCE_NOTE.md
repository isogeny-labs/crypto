# Source note: native Rust re-run of the lower-bound scripts (2026-11-15)

- Source: `examples/growth_lower_bound.rs` (stand-alone; no crate APIs used; auto-discovered example,
  no `Cargo.toml` entry needed). sha256 of the source and of the release binary built with
  `cargo build --release --example growth_lower_bound` are in `binary.sha256`; toolchain and host in
  `commands.log`. Base commit `d2a16c723d`; the example is uncommitted at run time (the main session
  commits it), so the source hash is the identity.
- Checks run: `cargo clippy --release --example growth_lower_bound -- -D warnings` (clean for this
  example; the crate's lint configuration prints one unrelated unknown-lint warning),
  `rustfmt --edition 2021`, `cargo test --release --example growth_lower_bound` (11 tests).
- Arithmetic: exact over `F_786433`, `u64` accumulators (at most 12616 terms below `p^2 < 2^40`,
  far from overflow). No floating point is used for any printed value, except in the unit test that
  pins the CPython `random.random()` value.
- Outputs: `results.jsonl` (128 structured records, one per computation); `run1.txt`, `run2.txt`,
  `run3.txt`, `hsupp2_t9_c4.txt`, `s0pat_t9_c4.txt`, `mult2_d2_s6.txt` in the Python scripts' text
  format (shell-trace lines omitted); `gridcheck.txt` (no Python output was archived for it);
  `extension_t8_c5.txt`, `extension_t9_c5.txt` (two runs beyond the Python parameters);
  `compare_vs_python.txt` (the diff against the archived Python outputs); `SHA256SUMS`.
- Not a benchmark, not an experiment run on the lab infrastructure: a local exact computation of
  the same kind as the Python it replaces (local development check). No `bench.record`, no pcc claim
  is registered `[pcc:unverified]`.
