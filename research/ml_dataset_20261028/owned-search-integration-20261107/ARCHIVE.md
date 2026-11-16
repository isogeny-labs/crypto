Full immutable software archive location and hash are in S3_RECEIPT.json. It includes the1453-file assembled source tree, binaries, complete source donors and all manifests. The Git subset omits assembled/ and full solvers.proposed.rs plus binaries listed below. Retrieve the archive, verify its byte count and SHA256, extract into a fresh directory, then run sha256sum -c SHA256SUMS. Original packet manifests refer to the full archive; resolve absolute source paths by their packet-relative suffix when replaying elsewhere. No scientific dataset was generated.

- original-owned-search-exclusion-integration-20261107/projection-checks/tests
- original-owned-search-policy-entry-v2-20261107/policy-tests
- original-owned-search-capture-projection-20261107/page-tests
