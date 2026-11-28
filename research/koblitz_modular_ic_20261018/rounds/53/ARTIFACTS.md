# R53 immutable raw custody

SHA256 `216560dcfbe58f0d00f3a072408e5e93879010727e99b8e9d08c48a6a76c9f59`;892964bytes,157regular files.
Scientific source `aebb0a5967f5b9b925cedf3b3c506f72df079c85`;42scientific source pins.
All accepted and rejected attempts,and the compiled Lean artifacts are retained.
The conditional PUT targets setec-astronomy-artifacts/store. A fresh direct
GetObject,all member hashes and byte-identical deterministic repack pass.
Valid completed workflows have historical Git input pins. Packet53-01 has
invalid intermediate custody after an interrupted live-runner edit;it is
retained as rejected evidence and excluded from valid executed-byte pins.
See results.json and PROOF_MAP.md for the complete exception.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/53
cd research/koblitz_modular_ic_20261018/rounds/53
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read README.md,CM.md,PROOF_MAP.md and NEXT.md. Do not regenerate accepted
producers for publication. These are byte-custody checks,not a substitute for
native mathematical replay or independent semantic review.
