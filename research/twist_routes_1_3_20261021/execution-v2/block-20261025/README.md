# E26 raw evidence, 2026-10-25

[Report and scope](../../reports/block-20261025/RESULTS.md),
[PDF](../../reports/block-20261025/RESULTS.pdf),
[raw manifest](RAW_ARTIFACTS.json), [full member hashes](RAW_MEMBERS.json),
[loose record hashes](LOOSE_METADATA.json) and
[next proposed experiments](NEXT_EXPERIMENTS.md).

This is a new archive of newly produced E26 run records only. No original
execution-v1 file or earlier 2026-10-23 archive was repacked, moved or rewritten.
The study store uses the user-confirmed `setec-astronomy-artifacts` bucket.
The immutable upload, fresh download, remote verification and every downloaded
member hash pass. The [publication receipt](../block-publication-20261025-setec-v1/RECEIPT.json)
records the operations. The native store index entry accompanies these
records. After earlier 403/404 errors, CLI access now succeeds; the
[delivery receipt](../block-checks-20261025/DELIVERY.json) records the actual
pushed commit separately from remote artifact availability.

From the repository root, with the AWS CLI authenticated:

```sh
CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
CRYPTO_STORE_INDEX=research/twist_routes_1_3_20261021/store/index.json \
ecbench store pull \
  store:evidence/twist_routes_1_3_20261021/block-20261025/runs.tar.xz \
  --out research/twist_routes_1_3_20261021/execution-v2/block-20261025/runs.tar.xz
cd research/twist_routes_1_3_20261021/execution-v2/block-20261025
sha256sum -c runs.tar.xz.sha256
cd ../../../..
node research/twist_routes_1_3_20261021/verify-block-archive.mjs \
  research/twist_routes_1_3_20261021/execution-v2/block-20261025/runs.tar.xz \
  /tmp/FRESH_E26_ARCHIVE_CHECK
```

The verifier rejects an incorrect archive hash or member list, extracts into a
fresh directory, restores the separately tracked loose records and checks every
member's bytes and SHA-256. It does not overwrite the original run tree or
recompute scientific results. To restore raw files for historical Rust replay,
extract the archive from `execution-v2`; names begin with `runs/block-...`.

Individually linked summaries, workload lists, exact matched-control proofs,
panel/assessment JSON, instruction COST/RECORDS and adjudications remain loose
in Git. Raw jobs, witnesses, basis inputs/outputs, certificates, profiler traces
and other per-branch files are archived. The large job catalogs are never
added as Git blobs. Every loose file and archive member is listed explicitly.

[Packing source](../../pack-block-evidence.mjs) inventories hashes, then uses
GNU tar with the fixed date 2026-10-25, numeric owner/group 0, normalized modes,
an explicit sorted NUL file list and single-threaded xz -6. Extraction and
repacking must yield the identical archive digest. The first synchronous
transport attempt and its extraction error are retained under
`../block-checks-20261025/packing-attempt-v1/` and `packing.stderr`; the corrected
asynchronous packer is a separate attempt with its own receipt and output.

Source revisions and pinned binary hashes distinguish the original panel,
separate-worker probe, batch worker and independent auditors. The batch
post-run receipt explicitly says its launch UTC timestamps were not captured;
the actual saved child exit, exact command, source and output hashes are retained.
See [checks](../block-checks-20261025/) and the two independent
[first-probe](../block-audit-20261025-v1/AUDIT.json) and
[batch-probe](../block-audit-20261025-v2/AUDIT.json) audits.

The original full E26 requirements, E27 complete cold accounting, E28 final
selection and independent-host replication remain open. These raw records do
not establish a calibrated IC speedup or novelty.
