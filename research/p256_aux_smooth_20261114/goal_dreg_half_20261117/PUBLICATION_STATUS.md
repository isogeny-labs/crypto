# Publication and admission status, 2026-11-17

Scientific package commit4071a9ac52ac4fa103ae897aa5d67cb682ecd240 is
published in open draft PR4590. It includes the exact local unplanted m2
198to14 certificate, complete reference and report. All93files in
PACKAGE_HASHES.json match locally. No independent signed/Lean admission.

The exact-head check-run API snapshot returned2records: review skipped,
migrate failed. This is not a census of workflows that have not produced
check runs. gate/size/artifacts were not returned and no success is inferred.
No submitted reviews or review threads were returned. Snapshot and log
excerpt are in evidence/publication-turn3/head4071a9-checks.json and
migrate4071a9-excerpt.txt. These are observations, not a green-CI statement.

Migration failed exit2 on S3 HeadObject HTTP403 for
`store/sha256/58/58c651d8cfe08e049db1605d8bb1f89a79de5388d5c879e81cb213715c4ab248`,
the retained-depth8/certificate.json (1685993bytes). It is selected by the
repository's >1MiB non-code artifact policy. Automatic verified migration
could not complete; the raw immutable file remains reviewable in this PR.
Do not infer absence or a specific IAM cause from403. The artifact guide
records an earlier disabled-store incident, but this current log establishes
only the HeadObject denial. No storage policy, object hash, gate, remote,
manifest digest or proof bytes were changed to bypass it. A successful
approved upload plus verified download remains a publication dependency.

A read-only merge-tree preview against main833a35d21e also conflicts in
ACTIVE and frozen pcc/ledger.jsonl. Copilot was requested in comment
6956684367; no response observed. No actual merge/resolution or frozen-row
rewrite was attempted. The append-only conflict needs an approved
reconciliation; do not select away either chain. Do not silently mark the
PR merged or enable a bypass route.

This continuation leaves the user goal ACTIVE. ANN_M2_PROTOCOL.md is the
next unexecuted concrete qualification, targeting exact planted m2 with
annihilator interpolation. Higher-m full degree admission, independent
standing and affine F4/runtime obligations remain. Native constituent units
and fresh byte replays passed; the combined wrapper was syntax-checked,
not rerun to evade closed session caps. Root Rust/Lean suites were not run
because those executables are absent. No new experiment launched here.
