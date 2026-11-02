# R70 immutable raw custody

SHA256 `9af2407b1c9b4ecd4f66f0100907cea6ab352cb622053127cdde9928a63fa8e7`; 217880 bytes, 82 regular files.
Scientific source `ce1dbf7ef9b618b0fc7b3504747e192cd12ab8e3`; 33 source pins.
Five terminal workflows are retained: two rejected, three accepted.
All 27 executed source/protocol/driver/audit pins match historical Git.
The accepted native bundle has the full two-class rank-two map,
zero kernel and old-order identity; final Lean compile/audit passes.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/70
cd research/koblitz_modular_ic_20261018/rounds/70
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, SESSION.md, results.json and PROTOCOL.md. The
donor manifest also requires retained R51/R52/R53/R61 archives and their
transitive donors for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
