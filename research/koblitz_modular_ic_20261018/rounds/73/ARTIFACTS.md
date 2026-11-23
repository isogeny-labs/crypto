# R73 immutable raw custody

SHA256 `fb267e8b42eedf1b4fe06a8430a612ace68235cad927f34b1eb7376657d5d754`; 44884 bytes, 73 regular files.
Scientific source `3f3181086a7163064f0058c1e6d0652c7097abeb`; 35 source pins.
Five terminal workflows are retained: three rejected, two accepted.
All 27 executed source/protocol/driver/audit pins match historical Git.
The accepted native bundle has the complete F32 square and relative
Frobenius matrices, stable image and all residue-degree checks;
final Lean compile/audit passes three conditional arithmetic claims.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/73
cd research/koblitz_modular_ic_20261018/rounds/73
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, SESSION.md, results.json and PROTOCOL.md. The
donor manifest also requires retained R52/R61 source files, R65/R70/R72
certificates and their transitive donors for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
