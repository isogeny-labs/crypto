#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("All 43 reconciled raw runs published with exact integrity checks")

#text("The finite joint-image round now has a complete canonical raw checkpoint. Every failed, interrupted and duplicate-audit receipt in its 43 selected run directories is retained. This is artifact preservation, not a new IC result.")


#image("../../figures/complete-checkpoint-verification.png", width: 100%)

== #text("Actual verification")

#text("The archive is 1,153,392,456 bytes, SHA-256 54b31b3d73602d7758dad263939bdfdcbd8c305df2299fe7beb498029090b729. Fresh extraction passes 9,088 native regular-file comparisons and matches the inside manifest to its sidecar. Repacking the extracted tree with the same GNU tar normalization and xz level reproduces the identical archive SHA. No archived program is executed by these checks.")


#text("The canonical name is store:evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz. Native store push, cache/remote verification and S3 whole-object SHA-256 and byte-count checks all pass. The old 542,528,576-byte f4ddf5ce checkpoint remains in the index's superseded history and its immutable remote object also passes checksum and size verification. No remote object is deleted.")


== #text("Attempts and source")

#text("The earlier level-6 packing attempt was deliberately stopped at 09:52. Its partial, hash, executed script and before/after stop receipts remain on the durable workspace. Only infrastructure compression changed to single-worker level 1; no mathematical run or successful upload was repeated. Both packing and full verification use the shared busy reservation and durable-workspace staging. Compression duration is not an admitted timing.")


#text("The actual packing source label is 30d6ac05a94679c97e035ea33df118bba2b66b35. The literal selection and commands are in pack-complete-checkpoint-level1.sh and verify-complete-checkpoint-level1.sh. Exact receipts are in results/publication/complete-archive-verify-v3/, interrupted-packaging-v2/ and complete-store-publish-v3.json. Credentials entered hidden stdin only and are absent from repository files.")


#text("The checkpoint does not certify a generic S5 ideal, strongest-rho comparison, calibrated complete price or breakthrough. Later current-main integration checks live separately; the completed 43-run snapshot is fixed.")
