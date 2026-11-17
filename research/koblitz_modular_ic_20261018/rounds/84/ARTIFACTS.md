# R84 immutable raw custody

SHA256 `42bf2620adddc808cf1763b97146ceb185d17cab0e4fa510a6815e6f36c2c4fe`; 5128 bytes, 15 regular files.
Scientific source `14978e137b98b66999dbc9bf342b7219750e0823`; 30 source pins.
One native workflow is rejected: direct reduction returned an inconclusive nonzero remainder at the first fraction.
All 6 executed source/protocol/driver pins match historical Git. One source-fraction polynomial was tried, with zero certified positive memberships; this archive supports an inconclusive reduction only.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks, not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/84
cd research/koblitz_modular_ic_20261018/rounds/84
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, results.json and PROTOCOL.md. The donor manifest binds the rejected R50 candidate arrays, R83 and R82 failures, R81 maps and installed Singular normal.lib for provenance. Do not repeat an
accepted producer merely to publish or verify custody.
