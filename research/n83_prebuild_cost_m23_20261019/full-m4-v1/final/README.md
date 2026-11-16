# Completed full-size U/D card

Status: `COMPLETE_BOUNDED`, 2026-10-20 **02:06:17–06:48:53 UTC**.
All 18 scheduled public searches finished under the original frozen card.
The independent native receipt audit passes **329 files**, including four
constructed controls and their four separate general-field replays.

- Archive SHA-256: `b2921b2e1c2cbb7c8b2030b1dc4e24acfee781546a3691bbe76d24ef9f960c78`.
- Manifest SHA-256: `570b6a102741eae862a638dcd9b0bfd08e0f8d8e6fb28a3a2904f53e2e3218a9`.
- Frozen worker revision: `d9ef42301cc3d22f437e1f89304e673836afb6a3`.
- Collector SHA-256: `acd9c3f252d9853a707cbbb9d8adbe49a086d18c6692e80e5c6e4284965357ef`.

The original source, inputs, card, binaries, controls, host observations,
container limits, exits, phases and durable progress are retained in
`runs.tar.xz`. The extracted `host/` and `runs/` are ignored convenience
copies. [audit.json](audit.json) binds the complete inventory and all
outcomes; [the report](../../reports/run-full-m4-natural-20261020/RUN_REPORT.md)
explains the censoring and accounting boundaries.

Reproduce in a fresh output directory:

```sh
cargo build --release --example n83_full_m4_audit
study=research/n83_prebuild_cost_m23_20261019/full-m4-v1/final
out=$(mktemp -d)
cp "$study/runs.tar.xz" "$study/files.sha256" "$study/archive.sha256" "$out/"
(cd "$out" && sha256sum -c archive.sha256 &&
 tar -xJf runs.tar.xz && sha256sum -c files.sha256)
target/release/examples/n83_full_m4_audit "$out" \
  b2921b2e1c2cbb7c8b2030b1dc4e24acfee781546a3691bbe76d24ef9f960c78 \
  570b6a102741eae862a638dcd9b0bfd08e0f8d8e6fb28a3a2904f53e2e3218a9 \
  "$out/audit.json"
cmp "$out/audit.json" "$study/audit.json"
```

U has five wall caps and four conflict caps; D has nine conflict caps.
No accepted natural row was returned. Rank zero describes the retained
empty matrices; natural yield and the complete useful-rank panel remain
null. The original wall-cap receipts lack final cgroup peak readings;
the latest durable CPU/RSS samples are explicitly lower bounds. The later
watchdog resource change does not alter this frozen evidence.

The closing host snapshot records a separate workload on CPU 14 while
the frozen card used CPU 31. This does not certify isolation beyond L0.
