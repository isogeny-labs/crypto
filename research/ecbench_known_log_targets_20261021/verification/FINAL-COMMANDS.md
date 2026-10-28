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
The two additional versioned catalog pages were inspected afterward;
`pdf-inspection-final.json` covers all twelve pages.
The copied benchmark files are byte-identical to their study originals.

Sparse generator inputs were materialized from the recorded Git revision;
the first generator attempts' sparse-path failures are retained in the logs.
Conductor's local service was unavailable; the scope check receipts are retained
under `conductor-*.txt`. This does not substitute for independent validation.

The final packaged-session audits (`package-audit-*.json`) all pass with zero
problems. An inspection of empty execution directories initially raised a
missing-log concern; every recorded stderr digest is null, the pre-restoration
audit already passed, and the hash-guarded check restored zero files. No session
file changed. `empty-stream-restoration.json` records that no-op explicitly.

Registry-dependent covers and graph were regenerated natively. All 11 native
cover tests passed. A versioned `catalog-update/` report preserves the older
report and diagrams. Its live assessment retains the status matrix and checks,
with population pins and semantic digests updated for the three new models and
five representations. The standalone constructor's catalog population assertion
is updated from 2,572 to 2,575 before rerunning its exact all-model control.

The validation gate without `--base` passes: every displayed row is validated
or frozen pre-gate, and this study adds no row. With `--base origin/main`, five
existing archive entries fail the check's demand that every archive entry also
appear in the base's admitted table. Those entries are already archived on the
base. The archive, index, freeze and checker are unchanged, proven by the empty
`validation-gate-findings-unchanged.diff`. This baseline failure is not patched
or bypassed here. The global curve-name lint's seven unchanged historical
findings are likewise retained.

Touched Rust files pass `rustfmt --check --edition 2021 --config skip_children=true`.
The optional `cargo fmt --all -- --check` reports 14 formatting blocks in two
untouched historical examples. `format-findings-unchanged.diff` is empty against
the base. The 13 standalone-constructor tests and its CI-toolchain clippy both
pass after the explicit catalog-count update. The final cover assessment verifier
reports `problem_count: 0`; its earlier failing population/binding diagnostics
remain as development evidence, not successful validation.
