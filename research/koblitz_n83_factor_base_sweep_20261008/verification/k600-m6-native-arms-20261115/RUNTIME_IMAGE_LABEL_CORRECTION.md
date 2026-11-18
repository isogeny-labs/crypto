# K600 runtime image-label correction — 2026-11-15

The [two-revision source amendment](TWO_REVISION_SOURCE_AMENDMENT.md)
introduced a build commit `C` and a direct-child evidence commit `H`.
Its initial patch checked the image's `org.opencontainers.image.revision`
label against `C` during arm freeze, but four later runtime checks still
compared that label with `H`: export before and after its Docker child,
solver before launch, and replay before launch. A real source-matched image
would therefore have been rejected after freeze. This was found by static
source audit before any K600 arm execution.

The native arm example now computes one `frozen_image_identity` from the
lock's exact image ID and `build_source_commit=C`. Export, solver and
replayer use it at all image inspections; their separate clean-checkout
admission continues to require `source_commit=H`. Missing or malformed
build/image identities fail closed.

Rust 1.99.1 macOS arm64 release arm tests passed **15/15**, including a
regression that distinguishes `C` from `H` and rejects a missing build
revision. The production release example built. The root library gate
remains the prior 6,306 passed / 59 failed / 138 ignored on unchanged
library sources. These are local code checks; no Linux image, Docker arm,
natural query or total IC runtime was measured. No performance graph or
leaderboard row changes.
