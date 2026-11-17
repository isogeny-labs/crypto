# Actual-root worker development receipts

The separately frozen source is 6ff21fd8711d33d84fa8c3eb2856e2d71899ac07.
The executable calls the real root F4/certified APIs; it does not reuse the
study's reference solver as a control. ROOT_WORKER_CONTRACT.md was committed
first. The original five-check API qualifier and its source envelope remain
historical snapshots at 0eb68e7d; they are not overwritten by this follow-up.

Three CLI contract tests pass: complete 31-bit seed-7 basis agreement,
incomplete/null output after a degree cap, and explicit unsupported wide-field
F4 control. Both the worker and its test executable were separately copied,
stripped, made read-only and hashed before invocation. The test executable's
RELATION_ROOT_WORKER_BINARY override pointed at the frozen worker, not a
mutable build path. Both hashes agree afterward. Strict all-target clippy
passes. These are producer development checks, recorded in claim
PCC-20261115-461551, without an independent execution signature
`[pcc:unverified]`.

Read build-01.json, pre-call-01.json and post-call-01.json in order. The archive
qualification-01.tar.xz contains both executable copies and the raw compiler
JSON/stderr. The post-call record pins its hash and byte count. Initial type
inference compile diagnostics are preserved in initial-check.log; the fix was
an explicit Poly annotation before the source was frozen. No test assertion
or resource threshold was weakened.

The worker takes one System JSON on stdin (4 MiB maximum):

```text
native-root-worker f4|certified [--degree-cap 12] [--deadline-ms 10000]
```

The degree cap and cooperative deadline apply to F4. Certified construction
retains its own structural/proof caps and requires an external process limit.
Every non-complete status exits nonzero and emits a null basis. The worker
reports F4's highest productive degree only for a complete F4 run; it reports
no such observation for certified construction. Raw F4 completion flags remain
available when an attempt is capped.

Common admission, conversion and canonical-output clocks are retained along
with construction/internal checking. They exclude launch and printing and
are not a speed comparison. No test compares clock values. A future matched
protocol must charge launch/output consumption and identical external basis
checking, retain every incomplete/unsupported case, and use supported mature
CAS controls. Checkpoint/isolation admission remains mandatory before any
scientific timing.

To build without running a campaign:

```sh
cargo build --release --locked --bin native-root-worker --manifest-path research/relation_degree_trace_20261115/rust-integration/Cargo.toml
cargo test --locked --no-run --test root_worker --manifest-path research/relation_degree_trace_20261115/rust-integration/Cargo.toml
```

For recorded qualification, freeze and hash both built executable copies
before running the test binary with RELATION_ROOT_WORKER_BINARY set to the
frozen worker. Use distinct new manifests; do not replace round 01. Trusted
artifact-autofix may migrate the archive to S3, retaining its exact digest in
the directory's .artifacts.json. Remote replay is not asserted before that job
finishes.
