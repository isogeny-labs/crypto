# H116 blinded target-manifest gate

No H116 candidate/direct moment, predicate score or timing measurement existed
when this input was generated. Producer source SHA-256 was
`e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8` and
optimized binary SHA-256 was
`d46f3627db7b75142324b7c926e6840e90b25e9e0890ccb34e938161d6132263`.

The producer applied the five protocol seeds to scalar indices in a separate
`--prepare-targets` process. It emitted two artifacts:

| Artifact | Rows | Bytes | SHA-256 | Consumer |
| --- | ---: | ---: | --- | --- |
| `inputs/blinded-targets.tsv` | 4560 + header | 460709 | `585a07cddf4bc0b9f44523f2982e6a140fddc6f79c70f48d17d8944a1a935b56` | crypto moment producer |
| scorer-only label map | 4560 + header | 444266 | `368aec98f2c3143f16a838572dbf3aacbe1e3c56b119370685efd1082850554b` | independent cryptanalysis scorer only |

The tracked manifest contains curve ID, public generator label, opaque target
SHA-256 digest and affine coordinates. It contains no scalar index. The label
map is not tracked in this repository and is not passed to `--run`.

Each `(curve,generator_label)` block has exactly `r-1` rows: 126, 196, 256,
396 and 546 for the five curves, repeated for labels 1, 2 and 3. A second
preparation produced byte-identical target and label files. The independent
cryptanalysis verifier must recompute the shuffle, coordinates, digests and
one-to-one label join before this manifest is admitted.

An earlier target-preparation attempt used extension-field point arithmetic,
was stopped before completion for avoidable cost, and its partial files were
deleted. It was never hashed, committed or used by the moment producer.
