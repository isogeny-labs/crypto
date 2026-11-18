# Interrupted-worker registry and authentic supervisor preparation

Reviewed patches preserve the actual original-input storage receipts before returning the verified producer token, and implement an ignored native-child supervisor using production process cleanup, outcome sealing and single-owner historical replay APIs. A separate immutable worker-originals.json avoids conflict with final worker-artifacts.json. Contradictory terminal registries are rejected.

Both patches apply together to production source 3517ae554787. All six assembled Rust files passed syntax parsing. Independent source review found no remaining blocker. Patches remain unapplied to production and uncompiled; eight proposed registry tests, authentic execution and public CLI export are unverified. External hard timeout and independently checked exact-source build receipt remain prerequisites.

Apply implementation.patch first and supervisor.patch second in a disposable checkout of the bound source. COMBINED_PATCH_CHECKS.json records patch and assembled-file hashes. This is software evidence, not generated scientific data. No scientific deadline, budget or build allowance was extended.

S3_RECEIPT.json binds the complete 26-file archive, freshly downloaded with byte equality and member hashes checked. Git omits duplicate full mod.rs/solvers.rs snapshots and their bases; the packet manifests describe the full S3 versions. Download the URI, check archive SHA-256, then check payload/CONTENTS.json before replay.
