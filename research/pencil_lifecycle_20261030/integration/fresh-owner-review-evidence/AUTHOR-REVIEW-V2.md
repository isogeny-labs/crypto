# Fresh owner native adapter and wiring — source proposal

This supersedes the implementation-status section of AUTHOR-REVIEW.md; original proposal, logs and failures remain retained. No shared source edits, cloud interactions, image build/publication, mathematical trials or scientific admission occurred. User ceiling25,200 seconds and expiry2026-10-30 22:08:40UTC remain unchanged.

## Exact implementation set

Apply native/package patch `fresh-owner-native-packaging-v3.patch` (f1f53433860cadd9e412e7cadc30b368065c48bde0f678d3e04c6830d38f1788), then `fresh-owner-wiring-on-campaign-v2.patch` (890bf747a00d03af14508dda25bea72f5296a09c41fa50ff0f831e84738d40bf) on reviewed campaignV2 patch116d6acb4095e6b256555589898d6c643dada68502f866b5ffa94a4248dc2788. The separate, unapplied reserved-path schema proposal is `fresh-owner-campaign-schema-proposal.patch` (6d15a2cac085587c5d68354a7a7db18f436d26c1c8a0fc3164b9cc8a93932ea3). It adds only optional worker_campaign/fresh_owner properties to the diagnostic-transport schema baseline. Root must obtain the applicable reservation authorization before applying it. Native and controller enforce cross-field conditions beyond JSON schema.

`source-manifest-v2.json` contains exact incremental source bases/proposed hashes. Packaging bases are unchanged from v1; `packaging-v3-correction.json` records the one later publisher correction. Full final source is `wiring/proposed/taskq/deploy/eks-karpenter/`; publisher final text is `publish-image-proposed-v3`, other packaging text remains in the patch. Native crate tests depend on the real adjacent run-ecbench path, and therefore run from this coherent private tree, not the earlier standalone directory.

## Wiring and consumers

Optional Request.fresh_owner explicitly binds adapter binary hash and absolute stop, requires diagnostic transport and cannot extend request deadline. Absent option preserves legacy serialized request. Controller emits owner request/image/binary/stop environment and obtains Job UID through downward label batch.kubernetes.io/controller-uid. For a campaign reservation it freezes the minimum of owner admission stop and reservation payload cutoff into the intended manifest. Controller independently compares actual Job environment, volumes and mounts to intent. Receipt checker uses that exact stored cutoff rather than a later request-global value.

Wrapper verifies adapter digest against explicit controller admission and build metadata; generates separate exact run/replay config bytes; invokes the native adapter for both original commands; and records its separate observation checker output. Config includes exact argv/spec/launch and request/Job/Pod/image identities. Config hash is computed in the trusted wrapper: it is not a claim of separately signed config or root-private custody. Legacy branch remains direct ecbench when optional admission is absent.

Wrapper forwards TERM/INT to the active native owner and repeats wait when the shell trap interrupted wait. Native owner kills its separately owned child process group and records actual wait status. Wrapper hash/upload failure no longer replaces a prior nonzero original child status; it reports collection error separately and preserves original status. An otherwise successful child still fails with90/91 on hash/upload failure. No successful-publication claim is made for failed uploads.

Native --check rejects mismatched exact config, missing observed reap/identity, changed retained preparation/journal bytes and invalid replay closure. It explicitly leaves accepted_external_closure and active-worker costs null. Controller fresh_owner::check independently reconstructs expected identities/argv from request, recorded Job/Pod and immutable manifest, rejects duplicate JSON recursively, checks config/receipt/checker hashes and build metadata, and checks retained preparation/journal hashes. This is identity/custody plus trusted observation checking, not the full native resource-journal semantic validator. Existing ecbench_owner_closure schema is not silently widened. A separate full-journal closure consumer remains necessary before claiming the enclosing cost is independently validated.

## Failed and expired collection

Before interpreting failed/succeeded fresh Job status, evidence-only collection requests bounded ranges for four run/replay config/receipt objects at expected S3 keys. Exact bytes become digest-addressed top-level JSON for the existing controller EXIT uploader. Missing files, too many Pods, unknown identities and exhausted collection time are findings. No verifier or replay is called in this lane. An error writing collection findings does not replace the original failed-Job status.

The campaign-expiry branch first retains existing Job data, then reads existing Pods with that Job selector, verifies ownerReferences against actual Job UID, and calls the same collector. This happens before returning the original admission rejection. Each existing kubectl/AWS call requires185 seconds remaining (existing180-second timeout plus5-second forced-kill allowance). The S3 collector uses the campaign absolute deadline when present, retaining the scientific payload cutoff separately. No new deadline or grant is created. Raw collection metadata explicitly keeps owner_receipts_authenticated=false and accepted closure null; collecting from an expected object key alone is not semantic authentication.

No IAM/RBAC edits are proposed. The existing code's bucket and read/collection transport remain unchanged; this source work did not test external permissions or authorize writes to any new destination. Actual role capability and approved publication path still require live reconciliation before deployment.

## Packaging

Actual fresh Dockerfile.ecbench receives the native owner, built static x86-64 musl by build-bundle and hash-checked/copied as data by publish-image. Computation runtime packaging alone is insufficient and is not used as a substitute. Adapter hash is in build-info.json. V2 publisher output added a fourth image receipt field, which could violate legacy Request::Build deny_unknown_fields. V3 preserves the original three-field image receipt and emits a separate `$output.fresh-owner.json` sidecar. This discovered compatibility defect and v2 are retained; only v3 is proposed for application. Static musl/image/publisher integration has not been executed.

## Source-bound software results

- `test-v7.log`: four native unit fixtures plus two integration fixtures passed. Integration exercises child self-signal exit143, child failure7, successful replay, missing run binding, altered Pod identity, direct TERM delivered to native owner, and TERM delivered to a Bash wrapper using the literal current signal/wait sections. Both owner/wrapper TERM cases observed child reap exit137 and incoming signal15 in receipt. These are distinct cases; child-self-signal alone is not wrapper qualification.
- `clippy-v7.log`: final coherent native crate all-target strict Clippy passed.
- `controller-wiring-test-v7.log`:39controller tests passed, including inherited campaign/control tests and new admission boundary fixture. Nested subprocess test counts are subsets, not additional experiments.
- `controller-clippy-v6.log`: strict all-target controller Clippy passed on this final controller source.
- Bash syntax passed for wrapper and proposed build/publish scripts. A direct jq positional-arguments fixture originally failed because `--spec` was interpreted as an option; corrected by inserting `--` before the argv. The corrected direct fixture preserved all arguments.
- Private merge initially omitted campaign module export, yielding failed controller compile logs v3/v4. Corrected by retaining both campaign and fresh_owner modules; failed logs remain. Schema-generation formatting assertion failed before modifying its output; corrected to actual compact properties syntax.

No full wrapper AWS/upload integration, real mounted downward API, image build, controller fake-S3 range test, full native-journal closure checker or measured allocation release is certified by these tests. Those are concrete remaining validation boundaries. The primary empirical four-lead task is not complete from this source/software work.
