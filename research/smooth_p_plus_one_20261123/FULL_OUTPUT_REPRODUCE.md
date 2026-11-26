# Reproduce full-output policy qualification

[Protocol](FULL_OUTPUT.md) · [result](FULL_OUTPUT_RESULT.md).
Local software/count evidence, without an admitted timing comparison.

Frozen source: `77c465a6d1e7ee675252c729a7c302febe282d7e`.
Identity-only commit: `da258b6de6`. Rust 1.99.1, the existing optional production
source closure and msolve 0.7.5 are required. [Cold accounting reproduction](COLD_ACCOUNTING_REPRODUCE.md)
specifies the closure and runtime library path; the [source layout](native/SOURCE_LAYOUT.md)
distinguishes historical pinned production.rs from the compiled current module.

```sh
cargo +1.99.1 test --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
```

The [exact command](evidence/full_output_v1/command.sh) contains all four fixed
suites, actual paths, memory/deadline limits and stream capture. It refuses
existing evidence. Use a new directory and record the actual executable hash
for any reproduction; never overwrite the frozen controls. Invocation shape:

```sh
/absolute/driver 127 4 /absolute/msolve account-suite=781815 draws=30 \
  base-lookup full-output direct refined cofactor-msolve
```

Omit `base-lookup`, `full-output`, or both for the other policies, preserving
the exact script's worker, runtime-library and timeout wrappers. A failed suite
does not silently omit later suites. No automatic retry is part of the protocol.

Format 3's `returned` contains the complete oracle set and `consumed` its
successfully attempted ordered prefix. Stop at the first consumer scalar,
retaining unused suffixes and charging their generation. Singleton results have
their existing distinct receipt shape. `selected` in a summary counts all
submissions; `full_output` separates oracle rows from singleton hits.

Recorded-data checks perform no new solve:

```sh
bash research/smooth_p_plus_one_20261123/evidence/full_output_v1/history_check.sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/FULL_OUTPUT_SHA256SUMS
jq 'select(.kind=="account_summary" or .kind=="account_process_receipt")' \
  research/smooth_p_plus_one_20261123/evidence/full_output_v1/full_lookup.jsonl
```

The initial failed test source/log, corrected tests and all four suites remain
in the manifest. Both historical first-row histories match; this check excludes
timings and solver-specific systems. The separate Direct/cofactor comparison
includes their returned and consumed rows but excludes method/chart metadata.
All claims remain unsigned local evidence. [pcc:unverified]
