Source-publication CI diagnosis — observed head `20e82118494e438afedb83e1dfdfb3b2b27a3dcd` (outcomes do not transfer to later metadata heads).

The backend [current job133570169265](https://github.com/isogeny-labs/crypto/actions/runs/43944432135/job/133570169265) failed at19:56:10Z and [recompute job133570169492](https://github.com/isogeny-labs/crypto/actions/runs/43944432056/job/133570169492) at19:56:20Z. Both complete decoded logs stop during maintained S3 artifact hydration with `HeadObject403 Forbidden`, before backend validation. [pcc:unverified]

The backend workflow, artifact helper/policy, backend manifests/catalog/reproduction lock and original replay-fixture manifest are identical Git blobs across feature-parent21f5, prior lane50a5 and source20e. Hosted checkout746b3df has exactly source20e's tree3d77e113. This diagnoses an artifact-read access failure on unchanged service inputs; it does not assert a source-algorithm failure or that main's latest CI was inspected. [pcc:unverified]

Diagnostic PCC-20261116-ffe108 pins projected facts and the lossless58092-byte decoded-log archive under indexed-circuit-v1/publication-ci-20e821. Original decoded logs are423416 and428623 UTF-8 bytes with hashes in RETENTION.json; these are not raw server-byte or signed execution receipts. [pcc:unverified]

Canonical maintainers need to restore CI read access to the referenced artifact objects. No credentials, policy, verifier, old subject/pointer or remote object was changed, and no rerun or bypass was requested. The earlier missing806be3 diagnosis remains a separate historical PCC issue; this adds no duplicate request for it. Pending/cancelled source-head gates and fresh-head merge checks are not inferred passes. [pcc:unverified]
