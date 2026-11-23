# Native evidence report renderer

Rust reads SHA256-bound JSON sources and emits `report.json`, `pipeline.svg`, editable `report.tex`, and `report.pdf`. PDF rendering invokes installed native `pdflatex` directly with `-no-shell-escape`; there is no Python subprocess/library or SVG-to-PDF conversion. The same source-derived observations populate SVG and the embedded TikZ diagram. Output uses a fresh directory and refuses overwrites. `SHA256SUMS` binds generated artifacts and raw rendering logs.

This renderer is deliberately limited to the current preparation-report contract: the bound plan-only input must contain zero observed executions and 36 explicit null cost-ratio cells. It refuses nonempty empirical inputs instead of presenting them as zero observations. It makes no statistical or mathematical judgments. Counts from separate software sources are shown individually with mandatory scopes; no test count is treated as an empirical target count or pooled into a total. The former Python report generator is retained as a historical, superseded toolchain and is never called.

Request schema `pencil-native-review-report-request/v1` has `title`, `snapshot`, `plan_summary: {path, sha256}`, and `observations`. Each observation has unique `id`, nonempty `scope`, `source: {path, sha256}`, and `fields` mapping displayed names to JSON pointers. Missing pointers fail; explicit null remains null. Every source path and hash is retained in report.json/PDF. `observations` can bind the narrow gate-approval record, native preparation receipts, and native canonical-packager software receipts as those become available. A receipt describes its own validation scope; it does not grant experimental admission.

```
cargo +1.99.1 test --release --locked --manifest-path research/pencil_large_prime_20261029/native_review_report/Cargo.toml
cargo +1.99.1 run --release --locked --manifest-path research/pencil_large_prime_20261029/native_review_report/Cargo.toml -- request.json NEW_REPORT_DIRECTORY
```

CI needs native `pdflatex`, provided by `texlive-latex-base`, `texlive-latex-recommended` and `texlive-pictures`. All tests are self-contained temporary fixtures, including a real PDF-render and write-once test. `tests/fixtures/historical-validation.json` is an explicitly historical subset of the old 19:06 report, extracted with jq; the original report digest is in `historical-source.sha256`. The source report is not modified. This fixture verifies that historical producer and root-suite counts retain their separate scopes; these counts are not relabeled as current results.

The final actual evidence report is rendered separately after its bound input receipts exist. It must be visually inspected before delivery; a PDF passing a software header check alone is not a visual review.
