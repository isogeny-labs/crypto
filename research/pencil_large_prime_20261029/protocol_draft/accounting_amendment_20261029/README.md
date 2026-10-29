# Unfrozen pre-data finite-accounting amendment

This directory is a reviewable draft, not a protocol freeze, approved evidence review, resource authorization or experiment result. The historical clock-only draft and its instrument artifacts are preserved. No solver run, cloud submission or instrument build was started by this drafting lane.

`protocol.json` is version 2 of the existing draft. It binds instrument source `543a54b4b1c3be7052872af3ffe393a44c69a3c3`, records the in-progress static build 40938439022 under workflow commit `7d7f5495c5b85ae426efdbcb721b6faf72c1be14`, and leaves binary/image/artifact/immutable build-receipt bindings null. The parent owns that build and must reconcile its actual outputs; historical binary `adb19f3e…` is not reused. Protocol integrity SHA256 is recorded in validation.json; this is not a frozen protocol hash.

The amendment adds the finite `pencil-algorithm-cost/atomic-scalar-disjoint-v1` model, 36 cells, exact paired prices and complete-denominator ratio-of-sums aggregation, separate calibration components and calibration-inclusive costs. It documents deterministic DefaultHasher reference instrumentation, including possible table-layout/runtime differences from the historical RandomState baseline. Scalar constructor modular M/S are scalar native events; diagnostic counters are excluded from duplicate charging. Reporting, allocator/RSS, clocks, harness and infrastructure remain separately retained resources, not invented GAE conversions.

Source and lockfile hashes are in `source_bindings.json`; every bound file was checked byte-for-byte against source 543a54b4b. The analyzer source SHA is the mapping digest used by its executable contract. `field_changes.json` provides before/after JSON-pointer changes relative to the historical protocol-with-binary.json, bound by its SHA256. This supports review without editing the predecessor.

The complete design remains 1,260 targets, five measured streams, one warmup, three arms, 6,300 measured candidate/reference pairs, 22,680 planned execution slots and 3,780 warmups. Existing prepared spec bytes, sample-size/randomization/seed/control/AA settings and stopping rules are unchanged. The same archive and all 9 prepared specs remain the source inputs; these counts are design arithmetic, not completed observations. All-five per-target A/A admission remains separate for cold and online clocks. No aggregate timing average rescues a failed control.

## Unresolved resource and admission bindings

- Cap provenance: existing unfrozen signed-symbolic specs have 25m coefficients throughout, but the cited historical Markowitz p739 gate has 10m and upper gate 25m. Parent archaeology found T789 preserves historical caps and no explicit signed-symbolic 25m small gate authorization. The existing draft numbers are retained for review, not promoted to authorization or silently changed.
- Time envelope: the original small gate's 3,600 seconds was one external gate-command limit. The existing expanded draft says 3,600 seconds per arm/execution. These are materially different cumulative envelopes. Preserve this as an unresolved amendment issue until the parent binds actual authorization.
- Cumulative authorized resource budget and original absolute deadline remain null. Neither is inferred from a per-execution timeout or newly reset by migration/build activity.
- New binary/image identities, immutable build receipt, new-binary plan parity, final independent nine-category source coverage review, analyzer binary and launch authorization are pending.

The retained `execution` and `stopping_rules` blocks are exactly equal to the predecessor for change review; that preservation is not a finding that their unresolved envelope is authorized. Explicit blockers are under `extensions.pending_bindings`. No `reviewed_complete` evidence, approval record or frozen status is written here.

## Validation performed

Applied the experiment-protocol skill and read its canonical repository rules/schema/checker, the prior draft, current finite cost boundary/primitive registry/mappers, prepared specs, and historical Markowitz protocol. Canonical resources are in crypto-autoresearcher-fall-degree-submission-20261029; they are not present in the crypto repository root.

Canonical check:

```
uv run --no-project --with 'jsonschema>=4' python /home/ec2-user/crypto-autoresearcher-fall-degree-submission-20261029/tools/experiment_protocol.py validate research/pencil_large_prime_20261029/protocol_draft/accounting_amendment_20261029/protocol.json
```

Result: exit 0, 0 findings. `validation.json` records checker/schema hashes, exact command, protocol integrity hash and ten passing preservation/pending-status checks. `validation.log` retains raw final output. The initial semantic check found a missing formula for the derived cost metric; it was corrected and the raw first finding is retained in `validation-initial.log`. Pending identities/resources are explicit schema-permitted nulls and admission blockers, not unresolved schema defects. Schema validity does not authorize a freeze or execution.
