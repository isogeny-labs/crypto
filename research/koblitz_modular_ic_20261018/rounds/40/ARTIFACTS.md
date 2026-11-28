# Verified immutable R40 artifact

SHA256 \`2d928f5db353dbc740ee0a963aaaa47e49f1f35ad171d5b44cafa119210695fd\`; 6908148 bytes,
324 regular files in setec-astronomy-artifacts/store.
Fresh GetObject, every member hash and byte-identical deterministic repack
passed. All 48 source pins were checked against the scientific
commit or the explicitly identified sealed R34 archived input.
Scientific source revision \`b9df19a5563b4695b90ee1556df19b9b591a79d2\`.

Four accepted native runs and one accepted Lean workflow are retained.
All five Lean declarations pass the transitive audit. Full source jets,
59 complete small/large bases, condition matrices, kernels, actual collector
class difference/inverse/cancellation bases and product certificates remain.
Byte integrity is separate from independent scientific replay.

Restore:

\`\`\`bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/40
cd research/koblitz_modular_ic_20261018/rounds/40
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
\`\`\`

Native per-run inputs are complete. Lean replay needs equivalent pinned
4.34.1/mathlib dependencies and path remapping. Actual modular-factor images,
chosen Koblitz model identification, larger requested supports and section
spaces, full saturation and polarization remain open. No IC relation rank,
speedup or unknown-scalar recovery follows from this artifact.
