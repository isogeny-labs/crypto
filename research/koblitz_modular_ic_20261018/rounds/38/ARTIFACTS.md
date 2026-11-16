# Verified immutable R38 raw artifact

SHA256 `ffa977d110a89a09636a51eeb9848aa9d5ba45e0b5bb9db626f1e352ea2a9f69`; 745012 bytes,
22 regular files in `setec-astronomy-artifacts/store`.
Fresh GetObject, all member hashes and a byte-identical deterministic repack
passed. Byte integrity is separate from independent mathematical replay/review.

Scientific source commit `2d5b85c9f1ca62645c67550a74eaeb448f1660d3`.
All 26 source pins verified: committed source plus the separately
archived R34 proof-cache path record. Original accepted and rejected inputs,
logs and certificates are retained. No accepted producer was rerun for restore.

Restore:

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/38
cd research/koblitz_modular_ic_20261018/rounds/38
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Native per-run inputs are complete. Lean replay needs equivalent pinned
4.34.1/mathlib dependencies and path remapping on another host. Native arrays
and proper geometry are not formally verified. No IC relation-rank, speed,
unknown-scalar or intended CM-sign image claim follows from these artifacts.
Global metric integration remains pending exclusive path ownership; the
scoped patch is documented in R37/METRICS_PENDING.md.
