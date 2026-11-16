# First current-base development gate: resource failure

The exact source is974f2e9cd46b000021cb20e6a78c3f54063f8882. A too-low6GiB
virtual-memory limit caused rustc-LLVM allocation failures in tests/build.
Lean aborted while creating a thread under the same resource cap. The raw
test101, defaultClippy0, build101, Lean1 and dependent-check-not-run125 remain
unchanged. This is not a source pass or a scientific timeout. No research
child was interrupted and no scientific source/protocol changed.

The separate attempt-2 removes the artificial virtual-memory cap and invokes
Lean with-j1. Its success does not erase this failed attempt. Both attempts
are development correctness/static checks, with no performance claim.
