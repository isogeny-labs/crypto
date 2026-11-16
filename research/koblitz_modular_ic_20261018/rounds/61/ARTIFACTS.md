# R61 immutable raw custody

SHA256 `c6146d43cfe9ebf45f8b3bb588f3b6ebc84bceb6463f74fc5b3374604214eaea`; 3429968 bytes, 200 regular files.
Scientific source `bc4f2a41f45a0f9052eb74b1ead1194b04621589`; 68 source pins.
All terminal workflows,including the rejected native attempt and interrupted pre-native launch,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; six authored claims give six audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/61
cd research/koblitz_modular_ic_20261018/rounds/61
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R43/R50/R52/R57/R58/R60 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
