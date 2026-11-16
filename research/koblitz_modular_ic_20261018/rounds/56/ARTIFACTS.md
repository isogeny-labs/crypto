# R56 immutable raw custody

SHA256 `714a5f816b34a084bb32b236196d952feab565eeac1a531a3b5b13c2084912ad`; 529548 bytes, 109 regular files.
Scientific source `4ccff22bccf07d83c9ad65901128011ade04495b`; 44 source pins.
All six terminal workflows,including the rejected first infinity attempt,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; four authored claims give four audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/56
cd research/koblitz_modular_ic_20261018/rounds/56
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R52/R55 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
