# Reconstruct the exact complete source

This incremental software packet retains new source overlays, focused-test binaries, fixtures, logs and manifests. The prior complete source is already durable in the immutable object in BASE_S3_RECEIPT.json. Fetch that base archive and verify its exact bytes and SHA256 before extracting into a separate fresh directory. Its owned-search-capture-composed-20261107/assembled directory is the input base. Verify both archives with sha256sum -c SHA256SUMS.

Run python3 owned-search-continuation-delivery-20261107/assemble.py --base BASE_EXTRACT/owned-search-capture-composed-20261107/assembled --output FRESH_OUTPUT. The assembler verifies every base and overlay hash and refuses an existing output directory. Compare generated ASSEMBLED_HASHES.json with the archived expected manifest:1458files. No build or network operation is performed by the assembler. Full source compilation/runtime checks remain outstanding.

No scientific datasets were generated or copied into this packet. Existing native prefix readbacks contain metadata only. No scientific allowance, deadline or pending rebuild authorization was changed.
