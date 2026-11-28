# Post-call evidence labels

The public SCALAR_PROTOCOL.md and PILOT_PROTOCOL.md each gained one explicit
`[pcc:unverified]` label after execution to satisfy prose lint. No acceptance
criterion, curve, target, scalar, seed, budget, algorithm or timing changed.

Original exact pre-call documents are preserved together in
`frozen/protocols.tar.xz`. `post-call-protocol-annotations.json` maps historical
paths/digests to archive members and records the annotated current digests.
Original pre-call manifests themselves are unchanged. When checking those two
historical paths, extract the exact original archive member and compare its
digest; do not relabel the newly annotated bytes as having been frozen before
execution. All other recorded pre-call inputs retain their original bytes.
