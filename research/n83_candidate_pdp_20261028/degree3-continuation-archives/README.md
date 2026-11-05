# Retained m3 continuation data

`runs.tar.xz` contains all twelve suffixes, canonical operations, global row
offsets, parent hash bindings, input snapshots, complete V2 traces, runtime
controls, executable, statuses and immutable progress summaries. The parent
is preserved separately in `../degree3-m3-archives/`; both archives are required.
No previous row, operation or report has been overwritten.

Read `../DEGREE3_CONTINUATION_FORMAT.json` for byte offsets, monomial indices,
JSON fields, parent references and unknown-value conventions. Complete V3 and
an LFD lower bound are different from a global LFD upper bound or regularity.

Hydrate both archives from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/degree3-continuation-archives
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/degree3-m3-archives
```

Check each `runs.tar.xz.sha256`. Extract into separate fresh `suffix` and
`parent` directories. Run `sha256sum -c manifest.sha256` in both directories.
For each successful producer, use the key, arm and id from the suffix protocol:

```sh
suffix/instrument replay "suffix/inputs/ARM" "ID" "suffix/v2/ARM/ID" "parent/cases/KEY" "suffix/cases/KEY"
```

Allow 8 GiB virtual memory and 1200 seconds per replay. Verify both manifests
again afterward. Replay reconstructs the complete parent and verifies its
hashes, every new operation and row byte, the queue and exact stop boundary.
This is producer verification; independent scientific review remains pending.

Repack identically from the extracted suffix root:

```sh
tar --sort=name --mtime='2026-10-29 00:00:00 UTC' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - -C suffix . \
  | xz -0 -T1 > repacked.tar.xz
```

`../evidence/degree3-continuation-storage.json` gives the object identities and
restore instructions. The native summary, unified export and publication
record preserve per-case outcomes and exact verification coverage.
