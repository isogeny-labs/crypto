# Verified immutable R39 artifact

SHA256 \`6dc29746d591d55aa623d6e67efc32d61d4eee0567140f7cf3eef8c2d05a24cb\`; 17284516 bytes,
169 regular files in setec-astronomy-artifacts/store.
Fresh GetObject, every member hash and byte-identical deterministic repack
passed. All 51 source pins were checked against the scientific
commit or the explicitly identified sealed R34 archived input.

Scientific source revision \`cdd825d32fad4761e2127ad0fbd63e51eca2bdbf\`.
Five accepted native calculations, one accepted Lean workflow and two
rejected Lean compiles are retained. Eight declarations pass the axiom audit.
Byte integrity is separate from independent scientific replay.

Restore with:

\`\`\`bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/39
cd research/koblitz_modular_ic_20261018/rounds/39
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
\`\`\`

Native per-run inputs are complete. Lean replay needs equivalent pinned
4.34.1/mathlib dependencies and dependency path remapping on another host.
The chosen Koblitz model map, full saturation, proper polarized embedding
and new collector divisor classes remain open. No IC speedup or unknown
scalar recovery follows from this artifact. Source proof boundaries are
documented in REPORT.md and FACTOR_CONSTRUCTION.md.
