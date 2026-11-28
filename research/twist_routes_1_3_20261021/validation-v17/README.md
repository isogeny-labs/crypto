# Validation without changing the frozen worker

This minimal workspace compiles the exact existing native source paths used by
the cold worker, harness, analysis and model/registry checks. It has no root
library dependency. It is a validation adapter, not a replacement instrument:
all recorded costs and the main analysis continue to use their original pinned
binaries and immutable source/lock hashes.

The incoming PR-head merge adds path crates to the root package. The original
standalone runner has an optional dependency on that package, so even a metadata
build now resolves the new root dependency graph. Updating its frozen Cargo.lock
would invalidate the filed instrument snapshot. Keep that file unchanged and
run these targeted source tests with this separate, locked workspace instead.
The earlier full standalone package test passed before integration. This does
not claim that the root library suite has passed.
