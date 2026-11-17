# Frozen v1 bindings

The nine new source/fixture inputs in `SOURCES.sha256`, the final executable and
the core qualifier are immutable for this qualification. A source change needs
a new version and qualification; never edit these receipts to match a later
working tree.

The 64-input context is composite: unchanged 43-input v2 archive (55,368 bytes),
unchanged six-input indexed delta (34,384), unchanged six-input worklist delta
(12,952), and nine-input affine delta (53,152). `BUILD_MANIFEST.json` and
`SOURCE_QUALIFICATION.json` bind all inputs and archive round-trips. There is no
monolithic 64-input archive. [pcc:unverified]

Retained executable: `/workspace/curve25519-native-binaries/affine-closure-control-v1`,
2,710,656 bytes, SHA-256
`54ad26145290b1228e4f45c6669176ae4b79ca17c7f187441bc9b6c2b7d1de7d`.
Core qualifier SHA-256:
`daaad933364f4675d9aa61a51e6c73dcf7ab6c72bc3ef2aef3111ba3cd2c4f34`.
[pcc:unverified]

`native-v1.tar.xz` is the complete 283-file raw tree: 16,263,777 raw bytes,
146,256 archive bytes, SHA-256
`ef5b561a184672f36ac9c2e12963f23d14d56aa731ea6e3d79f68968b3e2a875`.
Member hashes and original sizes are in `NATIVE_ARCHIVE.json`. The historical
Python-launched interface receipts remain unchanged inside it and are excluded
from final interface acceptance. Core native computation was Rust.
[pcc:unverified]

The corrected shell-only round has separate immutable plan/pre-call/qualification
and full stdout/stderr/status archive, 1,576 bytes, SHA-256
`78b380884607cba91d7fccb14c713f0cddcdd06fe4d1406f50c2bf7bd08841b9`.
This is a distinct verification round on the same input and executable bytes,
not a split of the core archive to avoid artifact policy. [pcc:unverified]

PCC-20261116-313284 and PCC-20261116-503900 are unsigned/unverified. The separate
checker does not validate producer counters, primality, execution identity or
signed standing. Original criteria, old source contexts and claims retain their
bindings.
