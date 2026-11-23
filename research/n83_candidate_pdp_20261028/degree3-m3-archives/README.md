# Retained m3 closure data

`runs.tar.xz` contains the complete raw12-case run, including all admitted dense
rows, every row-operation trace, original input snapshots, prior complete V2
traces, capped queue states,12 immutable cumulative summaries, controls, native
executable, source identity and terminal statuses. `runs.tar.xz.sha256` checks
the archive. `.artifacts.json` maps it to immutable content-addressed S3 bytes.
No earlier run is replaced.

Read `../DEGREE3_M3_FORMAT.json` for binary offsets, monomial indexing, JSON
fields and claim boundaries. The result is LFD>=3 on each of12 retained m3
Koblitz/binary presentations. All queues hit32768 rows and remain incomplete;
exact LFD/global upper bound is unknown. Regularity is a separate invariant.

Restore from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/degree3-m3-archives
cd research/n83_candidate_pdp_20261028/degree3-m3-archives
sha256sum -c runs.tar.xz.sha256
mkdir restored
xz -dc runs.tar.xz | tar -xf - -C restored
cd restored
sha256sum -c manifest.sha256
```

For each row of `protocol-degree3-m3.json` under `extensions.cases`, replay with
its exact `arm`, `id` and `key` values, using the executable from the archive:

```sh
./instrument replay "inputs/ARM" "ID" "v2/ARM/ID" "cases/KEY"
```

Replay validates complete V2, every admitted V3 source and row operation, row
bytes, the remaining queue, and lower/upper evidence. It writes only stdout.
Recheck `manifest.sha256` afterward. Allow8GiB virtual memory and600seconds per
case. The record is a producer replay certificate; independent scientific
review remains pending.

Repack identically from the extracted root's parent:

```sh
tar --sort=name --mtime='2026-10-29 00:00:00 UTC' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - -C restored . \
  | xz -0 -T1 > repacked.tar.xz
```

The raw S3 run prefix is recorded in `../evidence/degree3-m3-storage.json`, which
also gives the content hash, byte count and source archive identity. Publication
checks are in `../evidence/degree3-m3-publication.json`; per-case values and the
unified export are in `../evidence/degree3-m3/` and
`../experiment-degree3-m3.export.json`.
