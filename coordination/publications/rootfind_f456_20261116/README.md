# K13 combined panel source publication

The immutable `source-publication-fef1b537.json` was emitted by
`taskq/deploy/eks-karpenter/check-experiment-git` against pushed commit
`fef1b537dd6204272384ec37e249229756ff033e` of
`isogeny-labs/crypto`. It binds 37 exact files: the 36 inputs in the
previous combined-panel publication plus that previous publication receipt.
Its SHA-256 is
`1554ff4bcc2c3a7eb94c8002776b10decb70021ffa6f3563cc2e7a97b02ecc6a`.
Local recomputation confirmed all 37 byte counts and SHA-256 digests.

Relative to the previous source publication at `4d6980dd748bd4ff164147c71d14676d36f64eb0`,
three listed source files changed: `src/bin/ecbench/pdp3_koblitz.rs`,
`src/cryptanalysis/ic_boundary.rs`, and
`src/cryptanalysis/koblitz_index_calculus.rs`. The first two add a separate
collision-harvest engine and refactor the batch table through a shared builder;
the last change adds a cached-lift test. The frozen panel still names
`rootfind-negfold-batch` as its candidate. This receipt binds the exact new
source; it is not evidence that the refactor has identical timing.

This is **source and protocol publication only**. It does not bind an OCI
image, release binary, EKS launch request or host, and it contains no panel
measurement. The frozen ten-arm spec and analyzer remain in
`research/rootfind_f456_batch_combined_20261115/`. The latest committed
correct-account budget readback there, from November 15, was above its
monthly limit. This session's read-only `aws sts get-caller-identity` resolved
account `590183823895`, while the panel requires `104186811093`; no live
correct-account budget or pool admission was possible and no build/job was
submitted. When both gates clear, bind the image and binary to this exact
source revision and follow the frozen protocol. A different source revision
requires its own publication receipt.
