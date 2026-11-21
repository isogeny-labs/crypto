Complete software source, binaries, Rust-produced synthetic wire fixtures and full manifests are in the immutable S3 archive named in S3_RECEIPT.json. Git review subset omits assembled/ and fixture/ trees, standalone binaries and redundant full solver donors; manifests retain original full archive paths. Fetch the archive, verify byte count/SHA256, extract into a fresh directory and run sha256sum -c SHA256SUMS. The Rust fixture is92745bytes SHA4603c87286dc7b60d7323cc9138b7429b76949ba17114096da40eec02509b718; CROSSCHECK.json records independent inspection of it and361 chunks. This archive contains software fixtures, not a new scientific dataset.

- owned-search-versioned-dispatch-20261107/request-tests
- owned-search-capture-publication-20261107/bridge-tests
- owned-search-capture-encoding-20261107/checks/tests
