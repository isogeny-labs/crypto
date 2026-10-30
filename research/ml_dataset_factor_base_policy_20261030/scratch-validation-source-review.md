# Isolated proposal validation review

Read-only review of root's proposed scratch source validation approach. No
compilation, tests, copy/materialization, live source edit or Git mutation was
performed by this reviewer.

No inspected repository rule forbids compiling an isolated, separately scoped
copy of a proposed patch. AGENTS.md1307–1309 expressly distinguishes local
builds/unit tests/static checks as development work. AGENTS.md122 and1777–1780
require Conductor ownership before code edits; docs/CONDUCTOR.md describes
edit/write path coordination. This is not permission to overwrite T887/T888
live source or import their changing donor implementations.

The acceptable source binding is exact committed base5138 plus recorded overlay
patches and materialization inventory04, copied to independent regular files.
Bind source path/content/mode inventory, all Cargo/path dependency and build
configuration inputs, patch SHA256, toolchain/environment, final binary hash and
pre/post source readback. Avoid symlinks or hardlinks into changing live inputs.
Keep output/target directories outside the immutable source inventory, and use
the shared Cargo target only under sequential exclusive validation ownership.

Set exact build variables CRYPTO_BUILD_GIT_COMMIT to the base commit and
CRYPTO_BUILD_GIT_DIRTY=true. The base hash alone is not the candidate source
identity; overlay/inventory hashes are mandatory. Native generation intentionally
rejects dirty build provenance (current mod.rs1472). Therefore this snapshot can
provide proposal compilation/unit-test evidence, never a clean committed binary,
production generate acceptance, delivered CLI integration or generated corpus.
Preserve failures, counts and exact test filters; no zero-test success claim.

Own/check the validation script/output scope before writing; native Rust remains
the execution and correctness path. AGENTS.md570–585 prohibits new/extended
Python execution harnesses, correctness/replay tests and orchestration; shell
may provide thin build/run orchestration. Existing historical Python references
do not waive that requirement. This restriction is distinct from whether the
source snapshot is in scratch or a live worktree.
