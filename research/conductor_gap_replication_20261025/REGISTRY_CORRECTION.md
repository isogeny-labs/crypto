# Post-measurement catalogue correction

2026-10-25. This corrects the registration implementation, not the frozen
experiment design or observations. No session, audit, fixture, plan, task
card, initial registration receipt or preflight was rewritten.

The first `register.mjs`, committed before measurement, parsed and encoded
the existing registry with JavaScript `Number`. Legacy integer tokens beyond
2^53 in traces, cofactors and related metadata were rounded. Its preservation
check compared two already-rounded parsed values; the assertion
`prior_models_and_representations_preserved: true` in `registration.json`
therefore did **not** establish preservation of the original integers. That
assertion is retracted. The native cover checker caught a legacy EC1 tuple
mismatch during final catalogue regeneration.

`examples/conductor_gap_registry.rs` replaces that merger. It uses borrowed
`RawValue` objects and arbitrary-precision JSON numbers, preserving every
prior object's original bytes. The prior registry is read from commit
`cf08d6d5`, before initial registration; its SHA-256 is
`595b71600e1f471a2792d89666a8378ddbbeddc7a1425a808d16dfcbced76bb7`.
The repair restores 362 legacy models among 15,791 prior models and appends
the same 2,976 new experiment models. New model objects are checked equal
to both their measured registration and frozen native identity exports.
All new field primes are below 2^34; none of their tuple integers rounded.
The exact changed legacy slugs and before/after hashes are in
[registration-native.json](registration-native.json).

The timeline metadata renderer had the same numeric parsing issue. Its
legacy content is restored from the original timeline at frozen commit
`68e6c8b12d9185824bc071d7bc29ce2c2b5a2372`, retaining only the intended new
non-numeric result and date. Its parser now preserves raw numeric tokens.
The embedded scoreboard timeline is regenerated from that restored source.

Reproduce the lossless registry check from the repository root:

```sh
git show cf08d6d5:docs/curves/registry.json > /tmp/conductor-gap-prior-registry.json
cargo run --release --locked --example conductor_gap_registry -- \
  /tmp/conductor-gap-prior-registry.json --check
```

The archive remains the original measured evidence. A fresh native-binary
replay after repair verifies all 43,520 main and 128 calibration records;
its [certificate](validation/post-repair-replay.json) and complete native
receipts in `validation/post-repair-replays.tar.xz` are separate. It never
overwrites the measurement-time audit receipts. The cover, cover-link,
Jacobian-transfer, traits and browser catalogues are regenerated with their
native or existing maintained tools. This metadata repair supplies no new
performance observation or scientific claim.

The complete traits catalogue contains 18,943 records. Its pretty JSON is
156,045,766 bytes, above GitHub's blob limit. The native CLI now uses
compact encoding for large reports: 87,793,168 bytes, with every parsed
value checked equal before writing. No trait, source, curve or factoring
budget is removed or reduced. Build and freshness-check use the same
canonical encoding; the negative-mutation workflow accepts either whitespace
form. The independent native freshness computation passed before encoding.
