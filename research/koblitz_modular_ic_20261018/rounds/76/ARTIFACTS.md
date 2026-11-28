# R76 immutable raw custody

SHA256 `822464cd6df3a46e53e21ebf2c552569303b0ead7d52607e2240a7bd6c5a30d2`; 42224 bytes, 36 regular files.
Scientific source `9d391ddb41328c509d96aa9caca789ae9836f3f6`; 29 source pins.
Three workflows are retained: one native accepted, one Lean accepted, one Lean launcher rejected before compile.
All six native and five Lean source/protocol/driver/audit pins match historical Git. The accepted native bundle classifies six rational branches; the Lean bundle audits three theorems with zero failing and zero trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks, not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/76
cd research/koblitz_modular_ic_20261018/rounds/76
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md, results.json and PROTOCOL.md. The donor manifest requires the retained R52 table and R72 local factor data, with their transitive donors for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
