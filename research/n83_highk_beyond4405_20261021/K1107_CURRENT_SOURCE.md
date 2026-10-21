# Current-source K1107 lower-K finalist base

The fixed31 comparison source rebuilds the K1107 signed-Frobenius factor base
with **183,762 distinct subgroup-usable points in 1,107 folded columns**. Its
ordered dump SHA-256 is
`08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc`,
identical to the earlier K1107 finalist manifest. The source's base BLAKE3
hash is `b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e`.
Independent generic N83 arithmetic replay checked every representative,
subgroup point, signed Frobenius label, distinct point and set hash. It also
verified that all ordered K1107 representatives, labels and point coordinates
are the exact prefix of the current-source K4405 archive.

The base-only process used the same macOS ARM64 release executable as the
high-K base stage (SHA-256
`07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a`)
and the current IC source SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`.
It read one earlier public N83 fixture, set `KIC_BASE_BUILD_ONLY=1`, and had a
300-second cap. Its recorded base construction interval was 4,916.1255 ms;
the process began at `2026-10-21T18:56:39.198865+00:00` and stopped at
`2026-10-21T18:56:45.133383+00:00`. This local interval is a construction
diagnostic under shared-host load.

The [raw receipts](k1107_current_source_base/receipt.json),
[independent replay](k1107_current_source_base/independent_replay.json),
[compressed ordered dump](k1107_current_source_base/base_dump.jsonl.gz), and
[artifact manifest](k1107_current_source_base/artifact_manifest.json) are
versioned. The verifier bound the K4405 archive to SHA-256
`5b13e49152b0f120400930fa181a5e8cd3e8e45640bc0134389b10945b3ce333`.
The next cross-family run is K1107 full index, rank and independently checked
one-target recovery on the same source and public point as the selected
fixed31 high-K arm, followed by the K5500 split-index finalist.
