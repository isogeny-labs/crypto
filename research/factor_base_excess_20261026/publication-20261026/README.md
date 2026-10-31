# Verified storage publication — 2026-10-26

The owner approved S3 checkpoints and uploading the prepared validation archive. The exact original 73,608-byte software-check archive is stored under its SHA256:

`s3://setec-astronomy-artifacts/store/sha256/fb/fb2b6e1f4c4283f82f2ef7e33cddbb3b9774d6003e6c1892a9f7f02afd7b4982`

The canonical artifact tool generated `../validation/.artifacts.json` and its managed `.gitignore`. Remote checksum/length verification passed. The local hydrated copy was moved aside and a completely empty cache used for a fresh S3 download; SHA256 and size match the original, and all65 extracted member hashes pass. This establishes custody, not new scientific validity.

From the checkout root, use `python3 tools/artifacts/artifacts.py pull research/factor_base_excess_20261026/validation/checks.tar.xz`. Extract that archive into a fresh directory and run `sha256sum -c <checkout>/research/factor_base_excess_20261026/validation/CHECKS_SHA256SUMS` there. The preexisting `LOCAL_CHECKS.json` and earlier reports are historical snapshots of the then-blocked state; this versioned receipt supersedes that storage status without changing those bytes.

The canonical checkpoint gate also passed. Its uploaded checkpoint was freshly downloaded. Every logical payload was checked: RESUME.md equals the local handoff, worktree.patch equals the exact Git binary diff, and meta.json names the actual b828 source head with0unpushed commits. The encoded gzip hash before upload was not captured; the readback hash is recorded without claiming before/after gzip-byte identity. This checkpoint resumes the storage handoff; future research still requires refreshed final source/protocol/accounting and its own current gate.

`RECEIPT.json` records scope, object and checkpoint identity, hashes and checks. `checks.tar.xz` holds sanitized publication command receipts plus the initial failed authentication attempts. Check its sidecar here, extract it, and verify `CHECKS_SHA256SUMS` in the extraction directory. Repacking the extracted tree with deterministic GNU tar/xz must produce identical bytes. This separate storage phase contains no research measurements.

Credentials were supplied only to a short-lived child process through echo-disabled stdin and environment variables. No credential values were written to files, AWS configuration, receipts or Git. No paid host, native research panel, old-card dispatch, old F4 rerun or benchmark admission occurred. The source-specific native results remain those in ../VALIDATION.md; this storage follow-up changes no native inputs, assertions, limits, registry, aliases or existing store entries. Canonical scientific visuals are unchanged because no measured figure changed.

## Local follow-up checks

The canonical artifact tool suite passes18tests. The current validation-gate suite runs62tests:61pass and1errors on an existing committed-input schema mismatch (`B-benchmarks/verdicts.jsonl:1` has no `row`/`row_id`). The production gate stops on the same error. Benchmark rows, gate, freezes and verdict data are byte-exact pinned main; they are not patched here. Initial sparse helper/fixture failures and the unchanged retry are in the phase archive. Artifact policy, checksum/member/repack and focused whitespace checks cover this custody change; no fresh native-library pass is claimed for later main.
