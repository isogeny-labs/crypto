# Two-workspace lock gate v4 UTC-parser successor

This source-only successor preserves the reviewed v3 offline lock-generation gate and changes only the operational defect observed before v3 could create its workspace or systemd unit:

- both the caller and worker require the frozen deadline to end in `Z`, replace that terminal `Z` with `+00:00`, and then call `datetime.fromisoformat`;
- the workspace is `/var/tmp/pencil-kummer-lock-v4-20261111`;
- the transient unit is `pencil-kummer-lock-v4-20261111.service`;
- command paths point only to this v4 packet.

The v3 packet and its failed-caller observation remain unchanged. The retained terminal campaign ledger charges the full v3 reservation: 3,660 seconds charged, zero reserved, 125,940 unreserved under the unchanged 129,600-second ceiling and `2026-11-11T12:00:00Z` deadline.

This packet does not release v4. It performs no resolver, compiler, test, lock generation, systemd, or scientific payload execution. Independent source review, a fresh current-state readback, publication, and a separate immutable release are required before one invocation.

## Sources checked

- Python `datetime.fromisoformat` documentation: an explicit `+00:00` offset produces an aware UTC datetime; Python 3.11 broadened accepted forms, while the deployed v3 caller supplied direct evidence that its host interpreter rejects terminal `Z`.
- The frozen v3 source manifest and independent source review.
- The immutable v3 failed-caller observation and terminal campaign ledger.
- KB unavailable was already recorded in the predecessor session; this is an operational parser correction, not a novelty claim.
