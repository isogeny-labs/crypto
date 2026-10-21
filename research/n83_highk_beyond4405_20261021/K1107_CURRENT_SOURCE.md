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

## Full-process source bridge on an earlier public point

The same current-source executable completed a K1107 root11 full-process
control on fixture point 0 from the earlier cold-factor-base study. The exact
one-root layout, two target workers, cancellation, source and binary hashes,
public point and 900-second cap are in
`k1107_current_source_full_control/launch.json`. This run built 101,708,823
root-table entries with 1,342,177,280 allocated table bytes, reached final
rank 1,107, and recovered scalar `3640978018804724` after 940,621 target
probes. The raw child wall interval was 128.9140485 seconds with
1,933,492,224 bytes peak child RSS; the producer's exclusive target interval
was 334.80775 ms. These are local shared-host diagnostics.

The independent N83 group replay checked every factor-base point and label,
the exact ordered base and set digests, all four relation points, the target
subgroup, recovered scalar, final rank, root11 flags, and the five exclusive
target phases. All 15 checks passed with status
`PASS_INDEPENDENT_FULL_REPLAY`. The full-process base dump is byte-identical
to the compressed ordered dump above, so the versioned artifact directory
retains the smaller launch, process, target and replay receipts while the raw
full-process dump remains at
`/Volumes/data/cryptanalysis/n83_k1107_full_current_source_20261021_1930/base_dump.jsonl`.

Reproduce the control and replay from the repository root with:

```sh
python3 -m research.n83_highk_beyond4405_20261021.run_k1107_full_control \
  --binary /absolute/path/to/current-source/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-output
python3 -m research.n83_highk_beyond4405_20261021.verify_k1107_full_control \
  --output /absolute/path/to/new-output
```

The versioned receipts can also be replayed with
`--output research/n83_highk_beyond4405_20261021/k1107_current_source_full_control`
and `--base-dump research/n83_highk_beyond4405_20261021/k1107_current_source_base/base_dump.jsonl.gz`.

The next cross-family measurement pairs K1107 and the selected fixed31 high-K
arm on the sealed previously unseen holdout point under one Tier 1 allocation,
including the same-point rho reference. K5500's full split-index run is a
separate finalist gate.
