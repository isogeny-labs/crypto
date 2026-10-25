# Twist follow-up artifact store

The user-selected destination is `s3://setec-astronomy-artifacts/store`.
This study has its own [config](config.json) and native [index](index.json)
so that unrelated artifacts using the repository's shared default retain
their original destination. No existing object or index name was replaced.

From the repository root, with AWS credentials available to the AWS CLI:

```sh
CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
CRYPTO_STORE_INDEX=research/twist_routes_1_3_20261021/store/index.json \
ecbench store pull \
  store:evidence/twist_routes_1_3_20261021/execution-v2-20261023/runs.tar.xz \
  --out research/twist_routes_1_3_20261021/execution-v2/runs.tar.xz
cd research/twist_routes_1_3_20261021/execution-v2
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

The archive restores raw files under `runs/`; the 104 separately tracked
`SUMMARY.json`, `ADJUDICATION.json` and `INVENTORY.json` files already in the
checkout complete that tree. Use an empty staging directory and
[verify-published-archive.mjs](../verify-published-archive.mjs) to compare every
archive member and restore/check those loose metadata files without changing
the original run tree. The archive and restored tree are ignored by Git.

The publication receipt, downloaded-member check and frozen CAS replay are
linked from the [corrected report](../reports/publication-20261025-setec/REPORT.md).
The earlier proposed policy targeted the wrong bucket and is superseded.
