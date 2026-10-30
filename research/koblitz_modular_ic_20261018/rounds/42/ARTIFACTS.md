# Verified immutable R42 artifact

SHA256 `d92086f6f1e3c7465d1c08e356918874a3f724da2b7162dc3e719cec3cc404f4`; 20006164 bytes; 758 regular
files in setec-astronomy-artifacts/store. Fresh GetObject, every member hash,
and byte-identical deterministic repack passed. All 64 source
pins were checked against the scientific Git revision or the sealed R34 input.
Scientific source revision `4827d76038cb35c1fbc29c75c465a4094e37cd02`.

Three native computations, four accepted Lean workflows and two unsuccessful
proof attempts are retained. This includes all four integer matrices and
primitive inverses, all six target cases and both operator signs, and54
audited declarations. Byte integrity is separate from scientific replay.

The first package was rejected for Cargo build hard links. Its original
object pointer remains as runs-hardlinks-rejected.tar.xz and its source-pin
record is in publication-attempts. The accepted package expands hard links.
No scientific run was repeated to repair packaging.

Restore:

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/42
cd research/koblitz_modular_ic_20261018/rounds/42
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Lean replay requires the pinned4.34.1/mathlib dependencies and dependency-path
remapping. Use fresh attempt names. REPORT.md states all scientific boundaries.
