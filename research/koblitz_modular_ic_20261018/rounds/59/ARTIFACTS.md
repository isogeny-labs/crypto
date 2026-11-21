# R59 immutable raw custody

SHA256 `a8b3aae3a6b3f30352f212a1c4f5639cdd889eb936f53907d3efc8218401b071`; 3751060 bytes, 91 regular files.
Scientific source `1eac55400d827252dce85c5babaff105b878b0ab`; 46 source pins.
All terminal workflows,including the rejected native attempt,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; five authored claims give five audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/59
cd research/koblitz_modular_ic_20261018/rounds/59
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R50/R52/R56/R57 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
