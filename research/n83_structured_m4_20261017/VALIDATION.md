# Structured-base worker validation and current-head gate

The worker's `run4` mode and m=4 library path were built from native source
commit `1b00cf8a3450540aacd7778b6b2115276f46a81c`. Before the later
mainline stack merge, the touched release example tests passed **2/2** in
[example-release.log](example-release.log). The native macOS and Linux
archive verifiers both returned `PASS` in [verify.json](verify.json) and
[linux-verify.json](linux-verify.json). The m=4 core's pre-merge full release
library suite passed 4,216 tests as recorded in the linked
`research/n83_chained_s3_m4_20261017/VALIDATION.md`.

The first example test attempt stopped because four tracked compile-time
files were absent from the sparse checkout. Its
[failure log](example-release-sparse-failed.log) was retained. The repository
`scripts/sparse-checkout.sh apply` materialized those files before the
passing 2/2 rerun. An initial `verify` launch used a test-harness path
instead of the separate example binary; its
[launch error](verify-launch-failed.stderr) is retained. No query result
was collected from either failed setup attempt.

The final branch was stacked on PR #2119's merged mainline head after the
frozen worker runs. Its current-head checks are:

| Check | Outcome | Evidence |
| --- | --- | --- |
| `cargo test --release --offline --example koblitz_n83_structured_orbit_worker` | blocked at library compilation, exit 101, 25 inherited errors | [log](final-example-tests.log) |
| `cargo test --release --lib -- --test-threads=2` | blocked at library-test compilation, exit 101, 26 inherited errors | [log](final-release-lib.log) |
| `cargo fmt --all -- --check` | fails on unrelated inherited files | [log](final-fmt-repo.log) |
| `rustfmt --check examples/koblitz_n83_structured_orbit_worker.rs` | pass | local command |
| `git diff --check` on this PR's worker, study and benchmark paths | pass | local command |
| `jq` exact m=4 result and m=5 cap assertions | pass | both raw JSON receipts |
| `jq` configs after removing `summands` | byte-for-byte equal canonical JSON | [configs](run4/config.json) and [control config](run5/config.json) |

The current-head errors arise in ecbench and quartic files unchanged between
the PR base and this branch, as confirmed by a path-scoped `git diff --quiet`.
The specific inherited build obstruction is in
[the current-main problem](../../problems/current-main-library-build-20261017.md).
The earlier worker binary and its capped results remain tied to the checked
pre-merge source, not to a claimed passing build of the merged head.

Both result PDFs and the import PDF were rendered from their Typst sources
and visually inspected. The two diagram PNGs were rendered at 150 dpi from
their SVG sources through the checked Typst render files. The same files
were copied into `benchmarks/20261017-n83-structured-m4/` from this study.
The PNG metadata was set to 150 dpi with `sips`; `sips -g dpiWidth -g
dpiHeight` reports 150/150 for both study copies, and `cmp` confirms each
benchmark copy is byte-identical.
No canonical IC progress graph changed because neither capped arm supplied a
verified relation/rank increment or a complete-cost result.

The scoped Conductor CLI check returned `404 not_found` after its local
service state reset. The earlier source edit had a clear check and claim;
the reset prevented a fresh coordination lease for the result branch.
