# Final report and metadata verification

Commands run from the repository root, after the frozen measurements:

```sh
cargo +1.98.0 clippy --locked --lib --example known_log_targets --test ecbench -j2 -- -D warnings
cargo test --release --locked --example known_log_targets -j2
python3 tests/test_curve_identity.py
timeout 900 env RAYON_NUM_THREADS=4 target/release/curve_traits build --out /tmp/crypto-traits-full.json
cmp docs/curves/traits.json /tmp/crypto-traits-full.json
python3 scripts/build_ic_leaderboard.py
```

The full native traits rebuild completed and matched the guarded incremental
output byte for byte: SHA-256
`4823e60bfd83cfed7139a56a12702ebff643499c06258f7d4e9694593e192757`.
Its log is `curve-traits-parallel-build.log`, with empty successful comparison
output in `curve-traits-full-compare.log`. The first single-thread attempt hit
its 900-second wall limit (including the benchmark pause); it is preserved in
`curve-traits-build.log` and was not a cryptanalytic execution.

The receipt cross-check was compiled with native Rust against the locked build's
serde_json rlib (same pattern as `refresh_traits.rs`), then invoked as:

```sh
rustc --edition=2021 -O research/ecbench_known_log_targets_20261021/audit_totals.rs \
  -L dependency=target/release/deps --extern serde_json=target/release/deps/libserde_json-<build-hash>.rlib \
  -o /tmp/known-log-audit-totals
/tmp/known-log-audit-totals research/ecbench_known_log_targets_20261021 > /tmp/study-audits.json
cmp research/ecbench_known_log_targets_20261021/verification/study-audits.json /tmp/study-audits.json
```

The actual local helper path was `/workspace/crypto-tools/known-log-audit-totals`.
The helper asserts receipt checks, identical measured replays, counted A/A
equality and transported point-set equality. It is a same-author, same-host
cross-check, not an independent validator certificate.

Layout uses matplotlib 3.10.8, Typst Python wheel 0.14.8 and PyMuPDF 1.26.6.
`render.py reports/run-*` writes eight run PDFs and plots from native summaries.
Typst compiles `RESULTS.typ` to `RESULTS.pdf`. All ten PDF pages (eight single-page
run reports and two results pages) were text-checked and visually inspected.
The copied benchmark files are byte-identical to their study originals.

Sparse generator inputs were materialized from the recorded Git revision;
the first generator attempts' sparse-path failures are retained in the logs.
Conductor's local service was unavailable; the scope check receipts are retained
under `conductor-*.txt`. This does not substitute for independent validation.
