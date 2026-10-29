# Source, validation and controller S3 readback

Runtime and test source: `9e1d8e3817bb69285462957de72c7fac2f6983c6`.
Native tree: `93207490ffee3cb66a382e3fb7b0dad85af36546`.
CLI SHA256: `04ea30dacf6f2755239874b7e1b66107348d67c349aae6845694c1a7895ba591`.
The delivery documentation leaves that native tree unchanged.

The complete private source/evidence archive is persisted at:

```
s3://setec-astronomy-artifacts/ml-dataset/20261029/software/independent-partial-consumer/9e511cc223708e80f04a8a42351e8a3702dc58b93ced1ae4af76e593e165cc02/consumer.tar.gz
```

Its SHA256 is `9e511cc223708e80f04a8a42351e8a3702dc58b93ced1ae4af76e593e165cc02`
and its length is 29,000,657 bytes. The conditional PUT, HEAD, ETag-bound fresh
GET, length checks, archive byte/hash comparison and contained regular-file
hash checks completed at 2026-10-29 06:45:33 UTC. The packet includes source,
design, review findings and corrections, both release freezes' twelve retained
executables, all raw finite fixtures/outputs, and the original NFS CLI failure.

The freshly extracted archive then passed an independent filesystem manifest
comparison over 15,431 entries, including five regular-file hardlink groups and
two literal symlink target strings. Shared-inode aliasing between hardlinked
symlinks is not preserved by the finite test copier or claimed by this manifest.
The rejected symlink fixture's target text and NOFOLLOW error remain retained.
The manifest SHA256 is
`90bba7741347906ceff33c8733a2eef23863f98b810aa3e4b8ee49fa57aaca09`.

The downloaded test executable invoked the downloaded CLI on all eight
downloaded fixture cases, each with a new local restore and separate fresh
audit. All sixteen stdout byte sequences match the pre-upload results exactly.
The readback/replay attestation was completed at 2026-10-29 06:48:19 UTC. Local
receipts are `consumer-partial-delivery/receipt.json` and
`consumer-partial-delivery/readback-replay.json` under the retained controller
evidence root. A separate delivery archive preserves these post-readback
receipts, command logs/outputs and final documentation.

The final standalone release suite passed 239/0/9; strict release all-target
Clippy passed. The earlier e430 freeze's 42 sealed byte comparisons and mocked
legacy-fetch byte comparison remain e430 evidence for unchanged sealed/legacy
paths. They are not relabeled as executions of the final binary. See
[the acceptance ledger](ACCEPTANCE_EXECUTED.md) for exact coverage and limits.

This is controller archival and local software replay. No generator, PMU,
scientific corpus, solver resume, actual remote consumer invocation, cloud job,
IAM change or worker publication occurred. Errors return failure without
automatic adoption; marker presence is only claimed at tested pre-removal
boundaries. NFS server crash/power-loss durability is not established.

Delivery ancestry was reconciled after PR2824 merged. The target branch
`codex/ml-consumer-generic-filter-20261028` was at
`0a281dfa7aa9d456f472b7af7f9d86e20ce86a19`; its merge-base with this branch is
the original `ee936f2f498bba8a0a4e5df69c218034d4e8a009`. The target's native
consumer tree equals that original base. Hosted CI and merge acceptance remain
separate current-head gates; no previous parent CI status is inherited.
