# p739 timing-boundary audit

This is an observation-only source and record audit for the public synthetic
known-answer p739 gate. It does not qualify a clock, compute a speed or cost
ratio, amend a protocol, approve a repetition factor, or execute a solver.

The machine-readable record is [`audit.json`](audit.json), the quantitative
graph is [`timing-ranges.svg`](timing-ranges.svg), and the inspected two-page
report is [`report.pdf`](report.pdf). The graph shows the minimum, exact
median, and maximum of all 360 retained calls per arm, with measured and
warmup counts stated separately. `timing_qualification` and
`qualified_ratios` remain `null`.

## Bound evidence

- Instrument source commit:
  `543a54b4b1c3be7052872af3ffe393a44c69a3c3`.
- Canonical exporter source commit:
  `77765ba8b51d782aa0f32767b3e217e245a81cbd`.
- Raw 1,080-record SHA-256:
  `9ccc07c9b55b9180af36c3b910891803dde7f4249742881a7322f748bfdb73f7`.
- Frozen diagnostic-protocol-v6 SHA-256:
  `d2ca570ce39f2effa9564576bdb6b6e16148a0a6d511eeb05c3a01565e48d948`.
- The complete reviewed Rust and protocol blobs, Git object IDs, SHA-256
  digests, byte counts, and source line anchors are retained in
  [`source-blobs/`](source-blobs/) and `audit.json`.

The primary sources checked were the exact instrument blobs `record.rs`,
`three_leads.rs`, and `methods.rs`; the exact canonical-exporter `lib.rs`; the
frozen v6 protocol; all 1,080 raw records; the immutable-ingestion receipt;
and the neutral gate-observation report. These sources establish the timer
boundaries and retained values directly; no literature claim or novelty claim
is made by this audit.

## Observed facts

- Grading version 3 tests `time.solve_wall_ns` against the unchanged
  1,000,000 ns floor. The frozen protocol selects
  `time.method_call_wall_ns` for its primary cold metric.
- The protocol prose says that metric stops at the immediate method return
  before the profiling dump. The selected source field actually stops after
  nested instrumentation closes and `methods::complete` returns.
- Every rho and rho_AA solve interval is below 1 ms. Every one of their 720
  broader method-call intervals is also below 1 ms. Changing only the field
  tested by the classifier would therefore admit none of those rows.
- The instrument source contract permits `rho_solve` and `recovery_check` for
  a generic method. The reviewed canonical exporter requires the five
  index-calculus target phase names for every arm.
- All 1,080 outcomes are verified and match their public targets. The packet
  preserves the count and mathematical observations while leaving timing
  qualification and qualified ratios null.

The K=8 text in the packet is explicitly an **unapproved prospective design
question**. It records only that eight times the smallest observed control
online interval is 1,282,504 ns. Any repetition count, boundary, phase schema,
budget, or new run needs a new pre-data protocol and review.

## Native reproduction

The extractor is a standalone Rust crate and invokes no solver or Python. It
accepts only the exact pinned source objects, protocol, and records. Run it
into a new directory; it refuses to overwrite an existing report:

```sh
export CARGO_TARGET_DIR=/tmp/pencil-timing-boundary-audit-target
cargo +1.99.1 test --release --locked \
  --manifest-path native_extract/Cargo.toml
cargo +1.99.1 run --release --locked \
  --manifest-path native_extract/Cargo.toml -- build \
  /home/ec2-user/crypto-pencil-signed-graph-20261029 \
  /home/ec2-user/pencil-actual-results-20261030-a1/fresh/session/records.jsonl \
  /home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_large_prime_20261029/validation/analysis_provenance_erratum_20261030/diagnostic-protocol-v6.json \
  /tmp/pencil-timing-boundary-audit-reproduction
```

Two independent fresh-output invocations were byte-identical for the JSON,
SVG, TeX, PDF, source blobs, and renderer logs. PDF metadata uses the fixed
evidence date through `SOURCE_DATE_EPOCH=1793318400`.

Validation retained here:

- `cargo +1.99.1 fmt -- --check`: passed.
- `cargo +1.99.1 clippy --release --locked -- -D warnings`: passed.
- `cargo +1.99.1 test --release --locked`: 3 passed, 0 failed.
- `xmllint --noout timing-ranges.svg`: passed.
- `pdfinfo report.pdf`: two letter-size pages, unencrypted, no JavaScript.
- `pdftotext -layout report.pdf`: all sections and source hashes present.
- Both pages were rendered at 120 DPI and visually inspected; the table,
  logarithmic graph, 1 ms line, labels, and limits are legible with no
  clipping or overlap.

[`validation.json`](validation.json) records the validation scope and exact
output hashes. [`SHA256SUMS`](SHA256SUMS) binds the complete packet.

