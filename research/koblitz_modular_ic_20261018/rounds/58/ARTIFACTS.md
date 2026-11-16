# R58 immutable raw custody

SHA256 `4abe8523e940f4ae37a076b650a280c8bde623f389959f42b2b92422438fb8e8`; 754284 bytes, 123 regular files.
Scientific source `20049b1e75da833b5fcf79cb0fb5a4b021076df9`; 51 source pins.
All terminal workflows,including all rejected Lean attempts,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; seven authored claims give seven audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/58
cd research/koblitz_modular_ic_20261018/rounds/58
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R50/R52/R56/R57 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
