# R77 immutable raw custody

SHA256 `1dcbb697b9fb36aa991e2be12abd3f287ce6b8ac70d4bcdfa3f5e253fb01bd00`; 58744 bytes, 31 regular files.
Scientific source `6167ba406e532844f1720bd70139ba0a76a01f7b`; 27 source pins.
Two terminal workflows are retained: one native and one Lean, both accepted.
All eleven source/protocol/driver/audit pins match historical Git. The native bundle checks both quadratic infinity norms and an incorrect-norm rejection; the Lean bundle audits three theorems with zero failing and zero trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks, not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/77
cd research/koblitz_modular_ic_20261018/rounds/77
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, results.json and PROTOCOL.md. The donor manifest binds the R76 source equation, erratum and verified receipts, and the retained R53 geometric source statement. Do not repeat an
accepted producer merely to publish or verify custody.
