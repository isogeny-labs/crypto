# Verified immutable R44 artifact

SHA256 `75ce4225486332e328af1a187f20165c127edb984ada31fc1773a80bc806223f`; 37724 bytes; 222 regular files
in setec-astronomy-artifacts/store. Fresh GetObject, every member hash and a
byte-identical deterministic repack passed. Source revision `510b780cc6afa953eb2d4c77268a330b09d02299`;
71 source pins checked against historical Git or the sealed R34 input.

The archive retains all20 attempts:10 accepted native,1 partial native,
4 rejected native,4 accepted Lean workflows and1 rejected Lean workflow.
The partial degree436 run retains its timeout124; the accepted completion
reuses hash-bound completed geometry and finishes only the missing twist checks.
All three full degree436 timeouts remain charged. Eleven audited Lean
declarations comprise7 named statements and4 generated equations.

This is byte and receipt validation, not independent scientific replay or
semantic review. See REPORT.md and FORMAT.md for claim boundaries and encoding.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/44
cd research/koblitz_modular_ic_20261018/rounds/44
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```
