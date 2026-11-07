Full immutable software packet, including binaries and full source donors, is available at the URI in S3_RECEIPT.json. Retrieve it with aws s3 cp, check the archive SHA256 and byte count against that receipt, extract into a fresh directory, and run sha256sum -c SHA256SUMS there. Original manifests retain their original local source paths; resolve them relative to each packet after extraction. The Git review subset omits:

- continuation-original-budget-joint-20261107/joint-tests
- retained-exclusion-verifier-proposal-20261107/tests
- original-owned-search-continuation-proposal-20261107/solvers.proposed.rs
- original-owned-search-continuation-proposal-20261107/component-checks/tests

The full SHA256SUMS describes the S3 payload, not this reduced Git subset. No scientific dataset is contained here.
