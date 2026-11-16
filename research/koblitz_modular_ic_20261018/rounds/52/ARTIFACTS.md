# R52 immutable raw custody

SHA256 `eac39f184b9f72d5576f2e206f22163d6a71bf0d34794cdbdc3b1136028c2bc3`;2268956bytes,192regular files.
Scientific source `ed606e275da390f0c9ad97715251d6825552862a`;79scientific source pins.
All accepted and rejected attempts,and the compiled Lean artifacts are retained.
The conditional PUT targets setec-astronomy-artifacts/store. A fresh direct
GetObject,all member hashes and byte-identical deterministic repack pass.
Historical Git source bytes and terminal receipts are verified separately.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/52
cd research/koblitz_modular_ic_20261018/rounds/52
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read README.md,CM.md,PROOF_MAP.md and NEXT.md. Do not regenerate accepted
producers for publication. These are byte-custody checks,not a substitute for
native mathematical replay or independent semantic review.
