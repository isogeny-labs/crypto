# R85 immutable raw custody

SHA256 `56b7eabcac3237179f26a0b405151ff4490248805199e800006e27e8c3bc18e9`; 37572 bytes, 31 regular files.
Scientific source `aaf9d4606c2496332f50127c5b2a9fa2b6a58ec9`; 29 source pins.
One native Singular and one pinned Lean compile/audit workflow are retained and accepted.
All 11 executed source/protocol/driver/audit pins match historical Git. The bundle checks the quadratic-base diamond overlap products and three audited Lean theorems.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks, not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/85
cd research/koblitz_modular_ic_20261018/rounds/85
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, results.json and PROTOCOL.md. The donor manifest binds the accepted R50 source geometry and R27 diamond formulas, plus the pinned audit dependency. Do not repeat an
accepted producer merely to publish or verify custody.
