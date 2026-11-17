# Verified immutable R41 artifact

SHA256 \`193b464974ead810f4460dfdf778c4fe526d21766760b1b764084ddf3527057e\`; 122740524 bytes,
285 regular files in setec-astronomy-artifacts/store.
Fresh GetObject, every member hash and a byte-identical deterministic repack
passed. All 55 source pins were checked against the
historical scientific commit or the explicitly identified sealed R34 input.
Scientific source revision \`ebb2c0598c6195eaba4edf8fc624e7858c9ca375\`.

Eight native computations and two Lean workflows are retained, including
1604 full-generator quotients, both independent signed/cofactor expressions,
full split/operator/old-line comparisons and complete output class bases.
Nine Lean declarations pass transitive audits. Native outcome-file flags
were also compared with the report metadata. Byte integrity and flag
consistency are separate from independent mathematical replay.

Restore:

\`\`\`bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/41
cd research/koblitz_modular_ic_20261018/rounds/41
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
\`\`\`

Native per-run inputs are complete. Lean replay needs the pinned
4.34.1/mathlib dependencies and dependency-path remapping. Both crossed
SOURCE operator images are evaluated and nonzero on an independent
collector input. The explicit chosen Koblitz model isogeny, saturation,
polarized embedding and larger requested geometry remain open.
No IC relation rank, speedup or cryptographic scalar recovery is claimed.
