# Published receipt packaging

The exact Cargo stdout is preserved in [cargo-test-release.log.gz](cargo-test-release.log.gz),
compressed without a timestamp. Decompression reproduces the original bytes,
including the final blank line. The retrospective receipt and its original
execution-files.sha256 refer to the original uncompressed filename; restore
that filename before checking that original full execution manifest. Some
original execution hashes also bind toolchain/binary files kept in the local
audit workspace, which are not committed here.

The outer source-manifest.sha256 binds every published receipt byte. No test,
source, lock, command or reported result was changed for packaging. The report's
absolute audit paths describe the actual execution location; the corresponding
published logs, metadata and source hashes are in this directory.
