# Native K600 m6 arm-input staging component

**Status: implemented and locally tested, not run against S3 for this arm pair.**
The Rust `examples/n83_k600_m6_native_arms.rs` `stage` command bridges the
already verified read-only astronomy-store K600 object to the original
`pilot-01` panel layout consumed by `primary-k600-m6-xcnf` and
`primary-k600-m6-replay`. It checks the five exact legacy metadata and public
fixture files by SHA-256 and length, selects the unique primary `a=0`,
K600, `public_x_hash`, seed `2026100801` row, checks the configured store
remote and content-addressed index, and requests the object through native
`Store::ensure` into an empty cache. The staged 3,025,798-byte object is
rehash-checked at the legacy object path; source metadata, index, and clean
source commit are rechecked after staging. The output is write-once and sits
outside the source checkout. An error after output creation writes a terminal
`failure.json`. The command makes no S3 PUT and does not execute a solver.

The release example tests passed **2/2** on local macOS arm64 Rust 1.99.1,
and the production release example build exited 0. The same invocation also
passed the existing exporter **27/27** and native preflight **6/6** suites.
The two staging tests confirm that committed `pilot-01` metadata match all
frozen hashes and the exact K600 row; a changed manifest byte is rejected.
The [compressed raw test log](example-test.log.gz)
retains the result. The actual S3-backed `stage` command was **not** run, so
its fresh-fetch and terminal-receipt paths remain unverified. It has not
exported a K600 XCNF, solved a constructed or natural arm, or replayed a
model. The frozen 3,600-second proposed build/preflight/arm allowance remains
unapproved and unused. A source-locked Linux image, completed native outer
arm runner, same-circuit positive control, then at most one public natural
query remain separate admission gates. No relation yield or total runtime is
measured by these tests.

The last required root release-library run on this branch's unchanged root
library and Cargo sources remains [failed](../k600-m6-native-supervisor-20261115/ROOT_VALIDATION_20261115.md):
6,305 passed, 60 failed, 138 ignored; the extra deadline test passed when
rerun alone. This component changes an example and study files only.
