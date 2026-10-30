# R47raw artifact custody

SHA256 `ba4cb4a7e39bf3c62e83bfc3b8f54a66f548b1b387b717618f5a8767736a27b6`;
28525072bytes,169regular files,49historical source pins. Scientific source
revision `0cc5ef2ddbb7df0c73a36bac5c24f5f7215d5c94`.
Store: setec-astronomy-artifacts/store. Four accepted native and two accepted
Lean attempts retain all inputs, native source/binary identities, every
condition/kernel/class certificate and all proof/audit receipts. Raw quotient
certificates cover365full-generator computations. No native/Picard replay
or independent semantic review is implied by byte/receipt checks.

artifact-verification.json records fresh GetObject, all member hashes and
byte-identical deterministic repack. publication-validation.json records
historical source and per-attempt hashes plus terminal receipt consistency.

Restore with:

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/47
cd research/koblitz_modular_ic_20261018/rounds/47
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read FORMAT.md for coefficient/jet/class encoding. Lean replay requires the
pinned R34dependency cache and path mapping. Never reuse an attempt directory.
