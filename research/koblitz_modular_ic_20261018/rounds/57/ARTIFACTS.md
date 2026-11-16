# R57 immutable raw custody

SHA256 `06e9e3863f65932cd6eadc9bdbd92e8b3f8e0b9cb80c923168fddb97b5f1d845`; 463828 bytes, 152 regular files.
Scientific source `a4b82b7b26bb7c9a0704bf37ab37b537541ba103`; 52 source pins.
All ten terminal workflows,including all four rejected attempts,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; four authored claims give four audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/57
cd research/koblitz_modular_ic_20261018/rounds/57
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R50/R52/R56 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
