# R55 immutable raw custody

SHA256 `5c38192a3fbd068d731f1085d83ad1a15a525bfce175b154b92a7a27d6d8e1c1`; 402536 bytes, 94 regular files.
Scientific source `783ada9b48ca44533028678e10cbe6d6f475faee`; 39 source pins.
All six terminal workflows,including the rejected first quotient attempt,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; four authored claims give four audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/55
cd research/koblitz_modular_ic_20261018/rounds/55
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R52/R54 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
