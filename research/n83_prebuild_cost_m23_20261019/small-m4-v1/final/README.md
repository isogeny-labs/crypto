# Completed small-m4-v1 public-query card

The card ended at **2026-10-19 20:48:34 UTC**, with `COMPLETE_BOUNDED`.
All ten containers (two preflights, two constructed controls, two separate
control replays, four public queries) exited zero without OOM. Each public
query returned `UNKNOWN_search_cap` at exactly 100,000 conflicts. Natural
yield and useful rank remain censored. See the [run report](../../reports/run-small-m4-natural-20261019/RUN_REPORT.md).

`runs.tar.xz` is a new immutable archive of the complete card, separate from
the earlier control archive. SHA-256:
`b319353bf163c91b3180d6bf863b5129c580d3b7852751f345e4fd257b5abee9`.
It includes 132 files under `host/` and `runs/`, plus their `files.sha256`
manifest (SHA-256 `0eef8c9bffc75f0dd239ea5d89de4074965ea9bb48b906415795dc9da7f0533b`).
No input point object or private credential is embedded. Input hashes and
read-only mount identities are retained.

## Reproduce the receipt audit

From this directory:

```sh
shasum -a 256 -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
shasum -a 256 -c files.sha256
```

From the repository root, use a new output path:

```sh
cargo run --release --example n83_small_m4_audit -- \
  research/n83_prebuild_cost_m23_20261019/small-m4-v1/final /tmp/n83-m4-audit-new.json
```

The native audit checks the pinned archive and manifest, every listed file,
absence of extra raw files/symlinks, identities, ten exit/resource/mount
receipts, control/replay binding, serial order, all natural phase/progress
receipts, monotone counters and equality of each base's repeated search
counters. Its output is [audit.json](audit.json). It is a receipt audit;
the prior general-field control replay remains the mathematical check.
There is no natural relation object to replay in this card. Rank and yield
are explicitly null in this audit, rather than inferred as zero.

The archive was made with GNU tar sorted paths, fixed mtime 2026-10-19 UTC,
uid/gid zero, normalized `u+rwX,go+rX,go-w` modes, GNU format, and xz -6 -T1.
The earlier control archive and 19:05 live observation are unchanged.

Validation logs: `../../small-m4-audit-run-2.log`,
`../../small-m4-final-example-tests.log`, and
`../../small-m4-final-release-lib.log`. The first audit execution rejected
an incorrectly specified expected replay mount (`worker.json` versus the
actual pinned `receipt.json`); `../../small-m4-audit-run.log` preserves
that failure. The corrected audit uses the frozen wrapper's exact mount.
