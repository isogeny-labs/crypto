# R51 immutable raw custody

SHA256 `181804a06718ee102f3725523085f058cdb3aa65175a6b5f2e9c2b11a48e98f6`;2746684bytes,237regular files.
Scientific source `bab78d26d9d7dc74476dd0e7fc9ef50895c52f5c`;98scientific source pins.
All20attempts,including4rejections,and both compiled Lean artifacts are retained.
The conditional PUT targets setec-astronomy-artifacts/store. A fresh direct
GetObject,all member hashes and byte-identical deterministic repack pass.
Historical Git source bytes and terminal receipts are verified separately.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/51
cd research/koblitz_modular_ic_20261018/rounds/51
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read README.md,CM.md,PROOF_MAP.md and NEXT.md. Do not regenerate accepted
producers for publication. These are byte-custody checks,not a substitute for
native mathematical replay or independent semantic review.
