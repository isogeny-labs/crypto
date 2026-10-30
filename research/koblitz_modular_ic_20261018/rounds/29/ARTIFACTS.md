# R29 artifacts and replay

The raw bundle is in the content-addressed store in bucket
`setec-astronomy-artifacts`, indexed by `.artifacts.json`. Hydrate it with
`python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/29`.
The archive contains 66 regular files. Its SHA-256 is
`1e1369713e5bcc275931715175329e68eccd47e9dbc46c6cbb6f5b20194f00d2`. `artifact-verification.json` records a fresh GetObject,
every member hash and an identical deterministic repack. This is artifact
transport verification, not independent mathematical replay.

`source.sha256` pins 28 source/bridge inputs. The aggregate
scientific source revision is `fdc49967d8527f2229bf40d1f12340147db91360`; individual runs retain
their own source revisions and immutable input copies. `runs.sha256` and
`science.files` enumerate the exact raw contents. Unpack with ordinary GNU tar
in a fresh directory and check the manifest there. The deterministic pack
flags are `--format=gnu --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner`.

The host replay wrappers reuse pinned local tool caches and retain the original
4 GiB virtual/1200-second native caps. Portable Lean replay requires Lean4.34.1
and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`; compile the round's
modules with `-j1 -M4096` and run `tools/lean/AuditAxioms.lean`. The original
cache paths are execution provenance, not floating dependency versions.

All failures, caps and missing exits remain in the archive. Current proof and
claim boundaries are in `REPORT.md`, `VALIDATION.md` and the proof/bridge map.
No old coupled saturation, colored reverse transfer, actual CM-sign image or
IC efficiency/rank conclusion is inferred from transporting these bytes.
