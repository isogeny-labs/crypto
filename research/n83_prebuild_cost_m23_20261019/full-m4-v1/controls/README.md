# Full-base controls: archived prerequisite

All four U/D cache-off/on constructed controls and their four separate
general-field replays passed. The two native control-pair checks passed
on the runner and were repeated successfully on macOS from these exact
archived receipts (`local-pair-U.json`, `local-pair-D.json`). The local
pair check checks receipt/model/work equality and replay bindings; it does
not substitute for the general-field arithmetic replay.

The snapshot contains **107 files** under `host/` and `runs/`, including
source/input/card identities, eight completed container records, the two
native pair receipts and a timestamped observation. The eighteen-attempt
natural panel is outside this control snapshot and was running at capture.

- `controls.tar.xz` SHA-256:
  `3f017d34fd4a9a71c6af7a44a425c085f0be6df8dc34a405d9962d010d8b5d94`.
- `files.sha256` SHA-256:
  `ff96c5b4271d1bd1efcee13dd67ad8de66413b29f6647322eee7bb34a779d6d4`.

Verify `shasum -a 256 -c archive.sha256`, extract with
`tar -xJf controls.tar.xz`, then run `shasum -a 256 -c files.sha256`.
The independent local hash check is retained in `local-hash-check.log`.
The thin `collect-full-m4-controls.sh` uses sorted GNU tar, fixed mtime,
uid/gid zero and normalized modes, then xz -6 -T1. Raw extracted trees
are ignored locally and remain inside the archive.

[Control report](../../reports/run-full-m4-controls-20261020/RUN_REPORT.md),
[live panel status](../STATUS.md), [host card](../../FULL_M4_HOST_CARD.md).
