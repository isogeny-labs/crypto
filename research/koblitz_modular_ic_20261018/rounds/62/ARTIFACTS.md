# R62 immutable raw custody

SHA256 `a8bc9c4f5294fa158dce85cc8e5fe60893e7d4edcba9424e2964601ead51f1cc`; 1808976 bytes, 168 regular files.
Scientific source `6429a8a9d37700e0d096fc8f8d21d33bfe691c2b`; 52 source pins.
All terminal workflows,including the rejected native and Lean attempts,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; twelve authored claims give twelve audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/62
cd research/koblitz_modular_ic_20261018/rounds/62
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R61 archive and its transitive donor archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
