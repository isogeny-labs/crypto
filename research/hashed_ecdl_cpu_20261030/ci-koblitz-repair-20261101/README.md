# Import upstream Koblitz source repair

The root check and Clippy jobs on CPU PR head972d0d2fed4d failed to parse
examples/koblitz_orbit_dlp_fast.rs. The compiler reported mismatched and unclosed
delimiters in RootTable128 constructors and merged tests.

Sources:
- https://github.com/isogeny-labs/crypto/actions/runs/41340691213
- https://github.com/isogeny-labs/crypto/actions/runs/41340691156
- https://github.com/isogeny-labs/crypto/commit/2b1d79ecfb2ef33364db324076eeb60436d5b9df

T-1279 applies only that upstream commit's patch to the example. It restores
the coherent RootTable128 API by removing the union-merged duplicate bodies,
side-fingerprint fragments and interleaved tests. The larger upstream revert
also deletes research files; those deletions are deliberately not imported.
Existing evidence and the CPU pipeline remain intact.

The resulting example is byte-for-byte equal to the file at upstream
2b1d79ecfb2ef33364db324076eeb60436d5b9df. source-check.sha256 records both hashes.
rustfmt --edition2021 --check passes, establishing parser and formatting success.
This is not a compile or execution result. The root/CUDA workspace is not built
locally under the owner's disk constraint. Exact-head hosted check, Clippy and
harness checks remain required.

No round21 evidence or round22 frozen input is changed; no cloud run is
submitted. Round22 admission remains pending. This is an integration repair,
not a CPU speedup claim. The previous failed patch-import attempt for the policy
module is separate and did not modify this file.
