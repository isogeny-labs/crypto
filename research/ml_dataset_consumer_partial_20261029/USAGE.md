# Independent partial-lineage consumer

The standalone native consumer restores and audits caller-pinned interrupted
attempt artifacts. It checks the complete ancestry, original metadata and bytes,
authoritative context, committed JSONL/payload membership, and checked phase
prefixes. It does not resume a solver or turn partial records into training rows.

```
dataset-consumer partial-restore --anchor anchor.json --expected-sha256 HEX --objects LOCAL_CAS_MIRROR --out NEW_DIRECTORY
dataset-consumer partial-audit --restore RESTORED_DIRECTORY --expected-anchor-sha256 HEX
dataset-consumer partial-fetch --anchor anchor.json --expected-sha256 HEX --out NEW_DIRECTORY
```

Local restore follows the explicitly selected mirror. The anchor's original
absolute file URI remains an identity and is never followed. A destination must
be new. Errors return failure; invoking the command again does not adopt or
repair an existing destination. Tested failures before marker removal retain
that marker, while a late sync error can leave it absent without returning
success. A fresh audit reopens the original metadata and
recomputes the proof, ignoring any saved report claims. All reads, semantics and
inventory for one audit share the same pinned directory descriptor.

The remote adapter accepts only canonical `s3://setec-astronomy-artifacts` names,
uses us-east-2 HEAD followed by ETag-conditioned GET, and explicitly limits each
AWS CLI command to one attempt. It never lists, uploads, deletes, retries or
falls back. Its process-group, output, time and polling download guards are
bounded; the polling guard is not a filesystem quota. Actual remote invocation
is outside this increment's authorization and validation; only local fixtures
and a mock adapter were used.

Partial reports are restricted provenance. They have zero public rows and keep
cell completion, solver resumption, scientific validity, hardware truth,
platform admission, training cost, clock validity, whole-worker coverage and
worker publication false; whole-worker instruction count is null. A successful
byte/structure check is not authenticity or proof that an opaque event happened.
The sealed consumer's existing report and export contracts remain unchanged.

The cumulative modeled allocation limit is256MiB, including one reserved32MiB
phase allowance shared through every prefix and the final audit. The model counts
logical requested storage and all new collection capacities; it is not RSS or a
time quota. Logical object and byte admissions count repeated references and
fresh audit reads again. `actual_reader_bytes` counts bytes returned by artifact
readers, not AWS network traffic or command-output bytes.
`written_restore_bytes_before_report` excludes the report's own write. Other
caps, finite allowlists and transport details are frozen in IMPLEMENTATION_FREEZE
and the original design packet. Inputs can hit the cumulative model before an
individual maximum, so maximum fields do not promise that all combined maxima fit.

No Python cryptography, producer validator import, new producer invocation,
hardware collection, cloud worker, IAM change or dataset campaign is part of
this implementation. The optional software fixtures and exact-byte comparisons
run from existing saved inputs and test-only synthetic envelopes.
