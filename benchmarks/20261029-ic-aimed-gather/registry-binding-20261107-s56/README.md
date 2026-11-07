# S56 registry source binding, 2026-11-07

This additive successor of [the 2026-11-05 binding](../registry-binding-20261105/README.md)
accounts for S56 adding one model to the canonical registry (22,268 to 22,269).
The live `../VALIDATION.json` pointer names this successor; every previous
report, replay certificate, raw pin and measurement remains unchanged.
The native check changes only the current-registry raw binding and records its
source-equivalence receipt. Producer observation; signed execution and
mathematical admission remain unverified. [pcc:unverified]

The checker uses the preceding native check logic with the `checked_on` date
and predecessor text updated. The new copy is rustfmt-formatted; a formatting-
normalized comparison matches the predecessor with exactly those two literal
updates. The preceding source is untouched. Its pinned original registry is commit
`ddaba2819f4ee96139c4145fbed905db1c1db46e`, SHA-256
`53fe929b9b971fa2c74ee29fcd5cea7fca969d411ae28e8cbf413c9550f3032e`;
the original validator report is still SHA-256
`505327cf69bf6b6554244ee1214c71c87ee1331ba73dc74e1c36ebaa6388ee57`.
The complete unique target record for `icv1-f2m53-tm56619371-dac20a85` matches,
and restoring the old registry hash and removing the new equivalence note
recovers the original report's complete scientific fields. [pcc:unverified]

The [receipt](source-equivalence.json) records 211 unchanged other raw pins:
210 were checked from file bytes; the isolation JSONL pin was checked through
its existing artifact-store manifest. The latter is not a download/hash check
of the remote object. CI's existing validation job hydrates that raw input;
this session's direct evidence-directory check reaches only the remaining
missing isolation JSONL, with no registry-hash mismatch. No check, input,
scalar replay, timing or mathematical workload is waived or changed.
[pcc:unverified]

Reproduce on a checkout with the existing raw inputs hydrated:

```sh
git show ddaba2819f4ee96139c4145fbed905db1c1db46e:docs/curves/registry.json > /tmp/s56-original-registry.json
cargo build --manifest-path benchmarks/20261029-ic-aimed-gather/registry-binding-20261107-s56/Cargo.toml
```

Run the resulting `verify-registry-binding` with six positional arguments:
the original registry, current `docs/curves/registry.json`, original
`benchmarks/20261029-ic-aimed-gather/validator-report.json`, that evidence
directory, and two **new** output paths for the successor report and receipt.
Build dependencies are pinned to `serde_json 1.0.151` (arbitrary precision and
raw values) and `sha2 0.10.9`. The metadata-only reproduction used Rust 1.99.1.
Preserve committed outputs; a later run uses another directory. [pcc:unverified]

The [four negative controls](negative-controls.json) are rejected child
processes on temporary metadata copies: a changed target record, a duplicate
target, an extra newline in the pinned original registry, and an extra newline
in the pinned original report. No invalid successor output was emitted.
The original inputs were never edited. [pcc:unverified]

The unrelated append-only PCC ledger fork remains a merge blocker, tracked in
[the existing ledger obstruction](../../../problems/pkm-pcc-ledger-divergence.md).
This source binding neither reconciles that fork nor grants a new scientific
or performance claim. [pcc:unverified]
