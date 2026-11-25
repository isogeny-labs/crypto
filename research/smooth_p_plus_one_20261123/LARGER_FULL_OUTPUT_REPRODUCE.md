# Reproduce larger controls and separate replay

[Frozen protocol](LARGER_FULL_OUTPUT.md) · [results](LARGER_FULL_OUTPUT_RESULT.md).
This procedure distinguishes recovery from post-run verification and preserves
incomplete outcomes. It supplies local unsigned evidence only.

The recovery executable is from source `77c465a6d1e7ee675252c729a7c302febe282d7e`.
The auditor is from source `15ff500915` and has its own executable hash. Read
the [before-call identity](evidence/larger_full_output_v1/source_identity.json)
for full source revisions, executable/protocol/command hashes and historical
input hashes. Build each source separately with Rust 1.99.1 and the existing
optional production closure; [cold reproduction](COLD_ACCOUNTING_REPRODUCE.md)
records that closure and the msolve 0.7.5 runtime requirements.

```sh
cargo +1.99.1 test --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
```

Retain separate recovery/auditor binaries and record their actual hashes; do not
overwrite either frozen executable or any evidence. The [exact command](evidence/larger_full_output_v1/command.sh)
contains the six planned processes, their actual paths, worker/memory/deadline
bounds, historical-audit gate, per-process exit/stderr and local stream capture.
It refuses existing destinations. Use a new evidence directory for a replay.

The two interfaces, with the command's wrappers preserved, are:

```sh
/absolute/recovery 8191 5 /absolute/msolve account-suite=781815 draws=30 \
  base-lookup full-output direct refined cofactor-msolve
/absolute/auditor 8191 5 /unused replay-account /absolute/f8191_d5.jsonl
```

Use depth 4 for the other suite. The auditor's solver argument is unused; no
solver is invoked by replay. Audit both retained F127 full-output files first,
using prime 127/depth 4; failed historical audits block new collection and list
all unrun processes. After both new suites, audit both new files once, preserving
rejections as well as passes. No retry or 30-draw extension is part of this plan.

Recorded-data checks:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/LARGER_FULL_OUTPUT_SHA256SUMS
jq 'select(.kind=="account_summary" or .kind=="account_process_receipt")' \
  research/smooth_p_plus_one_20261123/evidence/larger_full_output_v1/f8191_d5.jsonl
cat research/smooth_p_plus_one_20261123/evidence/larger_full_output_v1/audit_totals.json
```

Full-set comparison includes unconsumed suffixes; consumer replay includes
every submitted row. Incomplete oracle calls remain missing outputs even when
their records pass replay. The existing fixture/reference/consumer code is
shared, so this is not independently implemented or signed validation.
