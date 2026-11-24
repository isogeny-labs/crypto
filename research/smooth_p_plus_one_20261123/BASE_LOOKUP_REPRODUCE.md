# Reproduce the common base-lookup control

[Protocol](BASE_LOOKUP.md) · [result](BASE_LOOKUP_RESULT.md).
This is local software qualification, not an admitted timing comparison.

Frozen source: `6a0c732e8e033b2839857c481b425c96364e4403`.
The later identity-only commit is `8f58c60786`. Rust 1.99.1, the same optional
production closure, msolve 0.7.5 and its runtime library path are required.
See [cold accounting reproduction](COLD_ACCOUNTING_REPRODUCE.md) for source
closure/build details and [source layout](native/SOURCE_LAYOUT.md) for the frozen
historical production artifact versus the compiled current module.

```sh
cargo +1.99.1 test --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
cargo +1.99.1 build --release --locked --features production-controls \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml
```

The [exact command](evidence/base_lookup_v1/command.sh) runs original then lookup
suites with the actual workspace paths, limits and stream capture. It refuses
existing evidence. Reproductions must use a new evidence directory and record
their actual executable identity. The two native interfaces are:

```sh
/absolute/driver 127 4 /absolute/msolve \
  account-suite=781815 draws=30 direct refined cofactor-msolve
/absolute/driver 127 4 /absolute/msolve \
  account-suite=781815 draws=30 base-lookup direct refined cofactor-msolve
```

Use the frozen script's one-worker, memory, runtime-library and timeout wrapper
around both invocations. A failed first suite does not silently omit the second;
its exit is retained. A fatal child lists unrun arms. Do not overwrite the frozen
original/lookup records or reinterpret a subset as a full four-leaf answer.

Format 2 distinguishes `base_relation` from `complete_query`; `calls` counts
four-leaf queries only. The summary records lookup checks/hits and the unchanged
oracle scope. The separate `base_lookup` interval and dictionary setup are
included in accounting; the consumer accepts and replays the one-term rows.
Unflagged format 1 remains available with the original first-row policy.

Recorded-history and hash checks perform no new solve:

```sh
bash research/smooth_p_plus_one_20261123/evidence/base_lookup_v1/history_check.sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/BASE_LOOKUP_SHA256SUMS
jq 'select(.kind=="account_summary" or .kind=="account_process_receipt")' \
  research/smooth_p_plus_one_20261123/evidence/base_lookup_v1/lookup.jsonl
```

The original-policy histories match all 31 earlier query records. Both suites
recover the fixture in all arms, but their query/row policies differ explicitly.
All observations remain unsigned local evidence. No speed comparison can be
inferred from the diagnostic nanoseconds in these files. [pcc:unverified]
