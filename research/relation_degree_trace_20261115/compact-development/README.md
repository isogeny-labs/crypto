# Compact-chain qualification receipts

Source 96f0c814d928f07e96e15361cc43a35fe7306377 implements the compact
chain, actual-root adapter and third CLI arm. The hypothesis and standing
errata were committed before implementation; the runtime protocol is a
separate pre-timing contract. Neither old frozen protocols nor receipts were
overwritten.

Each core, counter and native build has a separate build/pre-call/post-call
manifest. Actual calls use copied executables outside Cargo's build cache,
stripped and made read-only before pre-call hashing. Native CLI tests set
RELATION_ROOT_WORKER_BINARY to the frozen worker, preserving both the parent
test and child-worker identities. Every post-call hash matches its pre-call
record `[pcc:unverified]`.

core-tests-01.log retains the 30-test release result and the exact 442 versus
20852 cofactor-term observations. counter-tests-01.log retains 31 tests;
native-api-tests-01.log retains six actual-root API checks and
native-cli-tests-01.log retains three CLI checks. These are separate
qualifications, not runtime samples. The core input-order budget failure
remains a passing negative regression. Strict lint logs and the initial test
mutation style warning are retained `[pcc:unverified]`.

The separately frozen optimized worker and CLI qualifier pass the same three
CLI contract checks. release-cli-{build,pre-call,post-call}-01.json and
release-cli-tests-01.log retain that source/profile-specific qualification;
claim 3f31f7 remains unverified. Release and debug elapsed times are not
compared `[pcc:unverified]`.

Post-call records are pinned by producer claims 39655c, e806a2 and a014f5,
all unverified. Source paths in earlier envelopes are historical snapshots;
new public functions do not rewrite those snapshots. Generated debug build
cache was reclaimed with Cargo clean during optimized-worker preparation;
the immutable executable copies and recorded artifacts remain preserved.

Archives contain the frozen executables and raw compiler records. A sibling
archive envelope records exact hashes and sizes. Trusted artifact-autofix
may migrate those files to the content-addressed store; its .artifacts.json
then remains authoritative. Do not assert remote replay before that workflow
finishes.

Requalification requires a new source/build record, new copied executables,
new pre-call manifests and fresh output names. Build without running first;
then freeze/hash copies and execute those copies under the recorded caps.
Scientific timing remains subject to the checkpoint/execution gates and
COMPACT_SPEED_PROTOCOL.md. Do not use unit-test clocks as measurements.
