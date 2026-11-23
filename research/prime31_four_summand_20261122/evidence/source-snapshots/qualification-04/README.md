# Qualification 04 native source recovery

This directory is an administrative, post-call recovery of the study-owned native source and configuration bytes listed in the frozen [`qualification-04/pre-call-sources.sha256`](../../qualification-04/pre-call-sources.sha256) receipt. It does not replace or modify that receipt, repeat the qualification run, or establish a new pre-call freeze.

All **12 study/native files** match their original recorded SHA-256 hashes exactly. `SHA256SUMS.native` is the unchanged subset of original receipt lines whose paths begin `research/prime31_four_summand_20261122/native/`. The restored files retain those repository-relative paths beneath this directory. Verify them from this directory with:

```sh
sha256sum -c SHA256SUMS.native
```

The successful output is preserved in [`RESTORATION_CHECK.txt`](RESTORATION_CHECK.txt). This was a file-integrity check only; no compilation, mathematical test, benchmark, or replay was performed during recovery.

## Recovery method

Nine files were copied from the surviving current or earlier sibling study tree after matching the original digest. Three copies were reconstructed by reversing known later edits, then checked against the frozen digest:

| File beneath `native/` | Recovery |
|---|---|
| `extractor.rs` | Restored the historical diagnostic name `forbidden_branches_executed` and its original comment. |
| `f4-source/build.rs` | Restored the qualification-04 provenance inventory, including the independent test source, original arithmetic library, and its two tower-module dependencies. |
| `independent_tests.rs` | Removed the two later B8 gates and their helpers, restored the original B4 oracle helper and fixed evaluation count, and restored formatting. |

The recovered independent-test source has SHA-256 `ac95f7c286c3af8280aa3c3c47743bf7712aa23959ae35ebcd6f97e2ddb3b2d8`, matching the original 16-test qualification receipt. The restored extractor and build script likewise match their original receipt entries; no semantic-equivalence substitution was accepted.

The earlier sibling source was located at `/workspace/scratch/22854d1d6689/crypto-current/research/prime31_four_summand_20261122/native`. The current study tree supplied surviving unchanged bytes and the later versions from which the documented edits were reversed. These locations describe the recovery sources, not additional dependencies needed to inspect this archive.

## Scope

This archive contains only the study-owned native files present in qualification 04. In particular, it excludes build artifacts, the later B8 source changes, and the later identity-certificate module. Production dependencies remain covered by the full original source receipt and their recorded Git revision; this directory is not a full repository snapshot or a full-root build certificate.

Only this archival directory was written during recovery. The canonical native sources and frozen qualification receipts were not edited.
