# R54 immutable raw custody

SHA256 `1464d0b3667f4af6c5391d23e895559dcb7306b6f40e9155cf968a8aa1dfa45a`; 435688 bytes, 92 regular files.
Scientific source `daf1a262223918b98d3768e0f05bd5389689db19`; 43 source pins.
All five terminal workflows,including the rejected first primitive attempt,
are retained. All 29 executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; five authored claims
plus one generated equation give six audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/54
cd research/koblitz_modular_ic_20261018/rounds/54
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R52/R53 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
