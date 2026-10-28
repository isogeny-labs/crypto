# Open pencil workload inventory

This directory freezes the exact metadata-only inventory left open by the
point-driven pencil evidence. It contains 717 workload identities: 177 at 28
bits and 180 each at 32, 36, and 40 bits. The sole workload with prior capped
attempts is `(32, 186379, 0)`, `Wa0fc1e72d7a0`; the other 716 have no indexed
pencil record.

Each row includes the archive's algorithm seed, target seed, and planted
known-answer scalar as well as its workload identity and provenance.

`open-workloads.jsonl` is stored outside Git under the repository artifact
policy. Its tracked `.artifacts.json` entry fixes 434,290 bytes with SHA-256
`efd504b64437309408c3f199fcf3ed85e94322ef01d7d1545d87f6bf1409ae3e`.
Hydrate and verify it with:

```sh
python3 tools/artifacts/artifacts.py pull \
  research/four_followups_review_20261022/open-pencil-inventory
python3 tools/artifacts/artifacts.py verify \
  research/four_followups_review_20261022/open-pencil-inventory --remote
python3 tools/freeze_pencil_open_inventory.py --check
```

Regenerate the canonical bytes from pinned repository evidence with:

```sh
python3 tools/freeze_pencil_open_inventory.py
```

The generator reads the frozen ecbench archive and standardized pencil
indexes, and cross-checks their complete status partition against the frozen
1,260-row coverage index. It enforces the archive, schema, identity, seed,
count, verification, cap, serialization, byte-count, and digest assertions
recorded in `RECEIPT.json`.
It does not import or invoke collector, replay, matrix, nullspace, or recovery
code. This inventory does not execute the 717 open workloads or close their
experimental status.
