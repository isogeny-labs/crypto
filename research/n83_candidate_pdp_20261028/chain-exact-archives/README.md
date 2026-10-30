# Exact overlapping chain certificates

`runs.tar.xz` preserves all84 unchanged input pairs, native executable, protocol,
full coefficient/window certificates, complete tiny controls, per-case exact
original-top certificates, all84 immutable progress summaries, logs and statuses.
The proof and every JSON/binary convention are in `../CHAIN_EXACT_DESIGN.md` and
`../CHAIN_EXACT_FORMAT.json`; partial Lean scope is in the proof map.

Hydrate via `python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/chain-exact-archives`.
Check `runs.tar.xz.sha256`, extract to a fresh directory, verify
`sha256sum -c manifest.sha256`, run `instrument replay ROOT` with8GiB and1800s,
and verify the manifest again. The executable is the same producer instrument;
this is a full fresh-process replay, not independent scientific review.

Exact original-top regularity does not establish affine solving degree,
FFD/LFD, full n83 Hilbert functions or an elite factor-base improvement.
All historical sources, bounds, targets, points and failed attempts remain.

`support.tar.xz` preserves66 files: all helper sources, diagnostics, initial
failed publication checks, both report drafts, inspected-page images, source
roundtrip/checkpoint records and metadata comparison receipts. `CONTENTS.json`
inside describes every payload's format, byte count and hash; `manifest.sha256`
covers65 payloads, and the archive hash binds that manifest too. Restore with
`tar -xJf support.tar.xz -C FRESH_ROOT`, then check the manifest. This support
archive is not needed for mathematical replay of `runs.tar.xz`; both are kept.

Both archives use sorted GNU tar, fixed2026-10-29 UTC mtime, uid/gid0,
mode u+rwX,go+rX,go-w and single-thread xz level0. The exact commands are in
the retained archive helpers. No earlier archive was repacked or removed.
