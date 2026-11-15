# Reproduction

Restore .lean.txt sources as .lean in a fresh directory and adapt source-host paths in check.sh and dependencies.txt to the pinned toolchain/helper locations. replay.sh checks the six modules named in check.rs and the false control. inventory.txt lists nine declarations. Each before.sha256 binds source, helper caches, toolchain and native source. Dependency caches and checker binaries are omitted. This is not a self-contained toolchain distribution. Independent fresh kernel replay remains pending.
