# Verified immutable R43 artifact

SHA256 `2414c61e6c4ccb7cc5e38dfc1a10a1f005e85f5732279bb68b08db517f80852d`; 30777112 bytes; 113 regular
files in setec-astronomy-artifacts/store. Fresh GetObject, all member hashes
and byte-identical deterministic repack passed. All 46 source
pins were checked against historical Git or the sealed R34 input.
Scientific source revision `cd6df158b453ef3cac077d6f293b1cc296d884ea`.

Three accepted native computations, one accepted Lean workflow and two
rejected attempts are retained. The archive includes615-term coordinates,
units/inverses and sections; full product bases and function lineage; every
attempted product's binary coordinates; all pivot certificates; and proof
sources/logs. Stage coordinate file sizes and selected-basis row counts were
checked against native counters. This is byte/receipt validation, not an
independent scientific replay or semantic review.

Restore:

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/43
cd research/koblitz_modular_ic_20261018/rounds/43
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read FORMAT.md for coefficient/coordinate encoding. Lean replay requires
pinned4.34.1/mathlib dependencies and dependency-path remapping. Never reuse
an existing attempt directory. Native programs rebuild without external crates.
