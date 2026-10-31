# Local ML dataset custody inventory

`build_inventory.py` reconstructs `inventory.json` from four named campaign
audits and their per-cell Git checkpoints. It checks contiguous shard indices,
the exact plan and protocol file hashes retained with each cell, native
verification/export flags, producer archive and pointer readback receipts, and
archive byte totals. It also checks the digest and successful official-checker
receipt for each available experiment export. Running the pinned official
checker against the three available exports on 2026-10-31 returned zero
findings for each.

The inventory contains **100 observed native cells**, including **80
independently solved ECDLP cells**, **16 bounded root-query certificates**, and
**two retained geometric F6 pair domains**. The three official exports account
for 36 attempted rows: 34 completed and two censored BSGS timeouts. The prime
64-cell campaign has native verification and custody but no checker-clean
official export because its earlier protocol declared 64 instances across two
arms, requiring 128 export rows. The script keeps that absence explicit.

This is a local evidence inventory; its producer S3 readback flags are not a
fresh S3 audit by this script. The default v1 planner enumerates 165,150,720
method cells, and this inventory does not treat planned cells as generated.
The cell identities of these four campaigns are distinct; the totals are a
sum of these named campaigns, not a statement about the whole requested matrix.

Recompute without generating cells:

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory.py
```

The script refuses to overwrite a differing `inventory.json`; review changed
source campaign evidence and create a new version instead.
