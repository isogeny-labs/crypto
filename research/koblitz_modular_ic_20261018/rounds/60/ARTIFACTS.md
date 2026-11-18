# R60 immutable raw custody

SHA256 `3c93bd755acdc1776b0c2b6c1c56af686cf8a297dff8975b469005ed7ef36978`; 853604 bytes, 112 regular files.
Scientific source `da60d349b540fd32f00b9b54dbafd737c320238e`; 44 source pins.
All terminal workflows,including the rejected native attempt,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; three authored claims give three audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/60
cd research/koblitz_modular_ic_20261018/rounds/60
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R50/R52/R56/R57 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
