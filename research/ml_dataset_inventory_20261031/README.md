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

`build_inventory_v2.py` extends that immutable snapshot with the completed
48-cell ordinary binary paired-method campaign. Its independent receipt
projection checks contiguous indexes, protocol bindings, retained S3 readback
receipts, and the pinned official export receipt. The versioned total is
**148 observed cells**, **93 independently solved ECDLP cells**, **16 bounded
root certificates**, and **two retained F6 pair domains**. Four official
exports contain 84 rows: 78 completed and six censored. The new campaign
contributes 15 native verified outcomes, 22 partial, seven budget-exhausted,
and four timeouts; these native outcomes remain separate from administrative
export status. A fresh independent S3 GET/hash was also performed on its final
restricted archive; the inventory does not claim fresh GETs for every archive.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v2.py
```
# Version 3 update

`build_inventory_v3.py` appends the four-cell prime61 root/enumeration pilot to
the immutable five-campaign inventory. `inventory-v3.json` has 152 retained
receipts, 93 independent scalar solves, five checker-receipted official exports,
80 completed export rows, and eight censored rows. The prime61 pilot contributes
two verified enumeration artifacts and two root timeouts across both twists,
with zero scalar solves. The earlier inventory files remain unchanged.

# Version 4 update

`build_inventory_v4.py` appends the sixteen-cell Koblitz47 m=4
F6/enumeration campaign. `inventory-v4.json` binds its protocol, plan,
sixteen custody checkpoints, independent 512-artifact archive audit, two-object
fresh S3 readback sample, official export receipt, and phase cost projection.
It reports **168 retained receipts**, **93 independent scalar solves**, six
checker-receipted official exports with 96 completed rows and eight censored
rows, and ten retained F6 pair domains. The new campaign adds eight F6
pair-domain and eight bounded enumeration artifacts across both twists. Its
calling-thread instruction samples are retained as scoped raw observations;
none qualifies as a whole-worker instruction-to-solve training label.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v4.py
```

# Version 5 update

`build_inventory_v5.py` appends the four-cell ordinary-binary n=47, m=4
F6/enumeration pilot to the immutable version 4 inventory. It binds the frozen
protocol and plan, four custody checkpoints, independent 128-artifact audit,
official export and checker receipt, two-object fresh S3 readback sample, and
phase cost observations. `inventory-v5.json` reports **172 retained receipts**,
**93 independent scalar solves**, seven checker-receipted official exports
with 99 completed and nine censored rows, and twelve retained F6 pair domains.
The new pilot contributes two verified F6 pair domains, one verified bounded
enumeration trace, and one censored twisted enumeration timeout. Its scoped
PMU values remain ineligible as complete instruction-to-solve labels.

```sh
python3 tools/artifacts/artifacts.py pull research/ml_dataset_binary_methods_20261031
python3 research/ml_dataset_inventory_20261031/build_inventory_v5.py
```

# Version 6 update

`build_inventory_v6.py` appends the independently audited 26-cell Koblitz
`n=23`, `m=3..15` enumeration campaign to immutable version 5. It binds the
frozen plan and protocol, all 26 Git custody checkpoints, 685 rehashed native
artifacts, official export/checker receipt, 82 phase cost records, first/last
fresh S3 readback sample, and the interrupted shard 18 custody reconciliation.
`inventory-v6.json` reports **198 retained receipts**, **93 independently
solved ECDLP cells**, eight checker-receipted official exports with 103
completed and 31 censored rows, and twelve retained F6 pair domains. The new
campaign has four verified finite fixed-base searches at `m=3,4`, 22 native
timeouts at `m=5..15`, and zero complete instruction-to-solve labels.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v6.py
```

# Version 7 update

`build_inventory_v7.py` appends the independently audited 26-cell
ordinary-binary `n=23`, `m=3..15` enumeration campaign to immutable version 6.
It binds the frozen plan and protocol, all 26 Git custody checkpoints, 832
rehashed native artifacts, the official export/checker receipt, the first/last
fresh S3 archive readback sample, and the `.artifacts.json` pointer for the
85,780-byte migrated cost file. `inventory-v7.json` reports **224 retained
receipts**, **93 independently solved ECDLP cells**, nine checker-receipted
official exports with 129 completed and 31 censored rows, and 40,360,937
restricted archive bytes. This campaign contributes six verified native
enumeration outcomes at `m=3..5` and twenty `budget_exhausted` outcomes at
`m=6..15`, all with successful native verify, report, and strict export.
"Completed export" records successful finalization, not completion of a
budget-limited search. It contributes no independent scalar solve or complete
instruction-to-solve label; 104 scoped phase observations remain diagnostic.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v7.py
```
