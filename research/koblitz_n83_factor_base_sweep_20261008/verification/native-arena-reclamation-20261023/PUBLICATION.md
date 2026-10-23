# Native arena validation: immutable S3 custody

All objects are in `setec-astronomy-artifacts`, regionus-east-2, under
`store/sha256/<first2>/<digest>`. Configure
`CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store` when resolving the
logical `store:` names below. The tracked store/index.json retains all prior
entries and adds these content-addressed objects; no remote object was deleted
or name repointed.

| Logical name after `store:` | Bytes | SHA256 |
|---|---:|---|
| evidence/n83/arena-reclamation-20261023/readiness.tar.xz |3390892|585d71ad1ce557a3354062a732834c5a2f61cc135a2b5038e7f65ba7fe971e16|
| evidence/n83/arena-reclamation-20261023/integration-readiness.tar.xz |197506204|3f798c707ffc2d8640746b15422a97671cc089ce2478ff115b4f516f2e0d1fe1|
| evidence/n83/arena-reclamation-20261023/validation-k2048-r1.tar.xz |34144472|4176a287c6d4cbcb18f27dd1df67a8a069788d434fcc4a9d90993f99a68a52c9|
| evidence/n83/arena-reclamation-20261023/validation-k4096-r1.tar.xz |34143964|97eba74aaabf9ea5c6e3edc31a91bf985b8b2d840bb80f7f219cfd036044ae93|

Readiness includes original failed CI job logs, initial failed SAT development
source/log, corrected checks, Docker restoration, no-argument loader refusal,
source bundle and initial ELFs. Integration retains actual main reconciliation,
failed integrated GPU private executable/receipt/streams, bounded pre-exec fix,
complete4497-pass native validation and naming lint correction. Its Git bundle
contains the experimental/published history through37631c5c; prerequisite is
public05ec85b5287ebf137f1b1e16c8e91705d48cf77f.

Each1M archive includes exact run/streams/cards/specs/binds/inputs/ELFs/reference
stderr, source snapshots, native audit/memory/reclamation summaries and director
compilation/readiness/assessment records. K2048 includes both packaging failures;
K4096 includes its postcompile pressure refusal and independent later readiness.
Privileged reads and permission normalization apply only to staged copies.
No worker was retried and original files/permissions were unchanged.

All four pass PUT, HEAD, freshGET, byte comparison, every payload hash and
byte-identical deterministic repack. Both1M fresh extractions additionally pass
native audit, memory summary and full1M reclamation/prefix verification. The
readiness bundles' source-bundle verification also passes. Exact receipts,
manifest hashes and native fresh summaries are under [publication/](publication/).
Upload/verification receipts stay outside immutable archives.

To reproduce an archive, extract its sole root directory (`readiness`,
`integration`, or `validation-k2048-r1`/`validation-k4096-r1`), verify its
SHA256SUMS, then from its parent pack exactly:

```sh
tar --sort=name --mtime='2026-10-23 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - ROOT \
  | xz -6 -T1 > repacked.tar.xz
```

Replace ROOT with that extracted directory's literal name. The fresh replays
use the included frozen guard and original1M reference archive
SHA25698c4396b189493eba773c7d3d5e10f05f974d77cf9b811af8437c1150417c5ea;
restore the frozen absolute input paths or use the byte-identical archived
bind copies. These are zero-relation diagnostic replays, not scalar-recovery
certificates or performance admission.

## Continuation preparation and pre-search refusal correction

Two additional immutable names are registered in store/index.json:

- `evidence/n83/arena-reclamation-20261023/continuation-preparation.tar.xz`,
  2984808bytes, SHA256
  `c103fb3250fe66466fffe6f90fff5ebe4bcddef49dce117c912b8ae3340c6bdc`.
  Source througha591, prerequisite37631; final report inspection, v2/v3 guards,
  metadata/format/render failures and their corrections. No higher-cap worker
  at that archive freeze. PUT/HEAD/freshGET/payload/source/repack pass.
- `evidence/n83/arena-reclamation-20261023/admission-correction.tar.xz`,
  37079684bytes, SHA256
  `d44f b92c a0bd fdec 4c96 6c0c 030d 29bf 0c1e ab3d b0e7 da52 89b5 ad29 0773 9d6f`
  (concatenate groups). Source throughb877, prerequisitea591. Includes the
  actual K2048-c1 refusal, complete raw/card/spec/binds/relay/receipt/source,
  director assessment, pressure/lock refusals, first failed packaging collision,
  corrected staging, new binaries and21/43+7 native checks, native13/8987
  accounting, and separate inspected failure report. PUT/HEAD/freshGET/all
  payload SHA/source/native accounting/byte-identical repack pass.

Receipts and manifests are under publication/continuation-preparation-publication
and publication/admission-correction-publication-v2. Prior entries were checked
structurally unchanged; objects were neither overwritten nor deleted. Copies
were made readable only in the new staging directory. No corrected continuation
has run at this publication freeze; see RESPEC-C2.md for the exact bounded plan.

## Assessed launch failure and validated C3 correction

`evidence/n83/arena-reclamation-20261023/launch-correction.tar.xz`:
37112636bytes, SHA256
`0debd4e2016dc99c12ff152e6921b111861fe97c169d135b1e70cd2c6e285018`.
Includes the complete K2048-c2 launch-failed raw tree, exact card/spec/binds,
verbatim relay/receipt/streams/source, director custody assessment, readiness
checks including the refused usage invocation, all new21/44+7 test/build/source
records, cumulative43/8957 accounting and negative released-worker control,
Docker diagnostics and native barrier/scope/swap/cleanup smoke, inspected
separate one-page failure report and actual full-PR large-files PASS (the
initial missing-base invocation error is retained separately).

Source bundle through99516b4af7f40b053c62db598e5905bedc94fbef includes the
interveningde389 publication commit, with prerequisiteb877 in the previous
admission-correction archive. All payload hashes, source bundle, native launch
accounting and byte-identical repack pass from a fresh S3 GET after PUT/HEAD.
Receipts/manifests: publication/launch-correction-publication. Prior store
entries are structurally unchanged. No object was overwritten or deleted.
No c3/K4096 continuation has run at this publication freeze. See RESPEC-C3.md.

## Completed K2048-c3 native 1.5M continuation

`evidence/n83/arena-reclamation-20261023/continuation-k2048-c3.tar.xz`,
37639824bytes, SHA256
`8930bb0a93a608148f8f4e271a34628a0d2e20ce3e4fd57e1791ed14e7e2b143`.
One completed attempt, exit0/no timeout,1500000conflicts/boundedUnknown,
zero models/relations/rank. All1001 reference signatures through1M and152storage
snapshots validate; later signatures unpaired. Four reclamations remove
1581835327words. All89 memory samples and explicit unsampled tail pass; native
cleanup and director custody pass. Complete raw/card/spec/binds/source/streams,
receipt/relay, all readiness refusals/final clearance, independent assessments,
CI failure logs/annotations/repairs and inspected one-page report are included.
Source bundle prerequisite99516 is in the prior launch-correction archive.

PUT/HEAD/freshGET/allpayload hashes/source bundle/fresh native audit/memory/
reference-prefix reclamation/byte-identical repack all PASS. Receipts and store
manifest: publication/k2048-c3-publication. Previous entries remain unchanged;
no objects were overwritten or deleted. Original private artifact permissions
are unchanged; only staging copies were normalized. K4096-c1 remains open at
this publication freeze. No speed, base winner, UNSAT, minimum-memory or rho claim.
