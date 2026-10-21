# Native N83 ownership raw publication

S3 access works in us-east-2; bucket `setec-astronomy-artifacts`, native remote
`s3://setec-astronomy-artifacts/store`. Objects use content-addressed keys
`store/sha256/<first-two>/<sha256>`; no ACL/public exposure change. Historical
IAM refusals remain history, not a current storage blocker.

Final raw: **36,130,808 bytes**, **484 payload files plus SHA256SUMS**,
SHA256 `03754a1f073698cc4f786f03a2d3acd6eda253712370f266d11553b85982ea98`.
Manifest SHA256 `9af134a70b884ba0208c5cb3c07d4b900dabb7b8ada7cbb3ee5e3a6a004663e3`.
Key `store/sha256/03/03754a1f073698cc4f786f03a2d3acd6eda253712370f266d11553b85982ea98`.
Native name `evidence/n83/allocation-profile-20261021/runs.tar.xz`.

Fresh PUT/HEAD/GET, original byte comparison, all payload hashes, source bundle,
both exact per-arm native audits, memory profiles, storage invariants and451
retained counter snapshots per base, supplemental streams, original public
inputs/frozen ELFs and byte-identical deterministic repack all pass. An old
positive1M trace missing storage is rejected; a changed lock-wait record in a
constructed copy is rejected by custody audit. Originals are unchanged; negative
controls are not actual search attempts or additional SAT charges.

[Compact receipts and exact scripts](publication-proof/) retain store/provider
responses, payload log, replay JSON, negative stderr/exit codes, empty input
differences, bundle/repack status. Full raw includes ELFs/public inputs/cards/
prompts/receipts/assessments/full SAT traces/cgroup samples, all setup/build failures
and unused preliminary ELFs, exact source ancestry and inspected four-page
report/graphs/benchmark copies. First failed development build's complete source
was not snapshotted; original log is retained. K2048-p21 shell failure lacks a
measured duration/native receipt; K4096-p21 retains its actual10.985894322-second
refusal receipt. Missing records remain missing. No reclamation candidate, higher
cap, speedup, UNSAT, selected base or recovery result.

## Interim recovery checkpoints

Each has fresh GET/payload verification. Pre-worker checkpoint also checks
source/ELFs/original1M audits/missing-storage rejection/repack. Completed K2048
replays audit/memory/storage-prefix/repack. Queued checkpoint checks source/ELF/
old arm audit/repack. Full proof is in raw checkpoint-proofs/; compact receipts
are in publication-proof/checkpoints/. Names share prefix
`evidence/n83/allocation-profile-20261021/`:

| Name | Bytes | SHA256 |
|---|---:|---|
| `pre-profile-checkpoint.tar.xz` | 34149480 | `4822a25cf24f4e43c1493404f3496c18f6abbedbb106ddec2309db021cc28d8f` |
| `p22-launch.tar.xz` | 8040 | `924c49350a8bcf51d309ffc3132fbb16c835ea9a4e391a4221d61c770fa19be3` |
| `completed-k2048.tar.xz` | 287300 | `684432eeb4c133ddf537908a8e327b2aed00d3321469c5975521fe6cfc719060` |
| `queued-checkpoint.tar.xz` | 853144 | `f8fa473bccd5bfc77daa9ef74277a0c51e6da6ca4062fbad3e50be5c7c080740` |

## Restore and verify

Download the exact key with aws s3api get-object, verify SHA256, extract fresh,
and run sha256sum -c SHA256SUMS under runs/. GNUtar sorted,mtime2026-10-21UTC,
uid/gid0,normalized public modes,xz-6-T1 is deterministic. The exact tested
native replay/negative/repack recipe is publication-proof/publish-final.sh.
Absolute public-input/worker binds must be restored on another host; archive
contains those bytes and fresh verification compares them separately. Bundle
requires published37679b35; see SOURCE-COMMITS.md. Native audit replays original
query/group and terminal/custody/resource checks, not a timed SAT solve. No
retained reference is silently admitted as a timing control.

Original1M raw98c4396b189493eba773c7d3d5e10f05f974d77cf9b811af8437c1150417c5ea
and sidecar3a7bb909928f164075ac695f58959968eaf4a692463c38b37b00d0b9bf4a0f6b remain
separately verified immutable dependencies. Old800K failures/raw and historical
checkpoints retain their own publication records; none is retrospectively
admitted or overwritten. This raw cannot contain its own upload receipts.
Verification sidecar is sealed after this proof commit; final sidecar receipts
are appended outside that sidecar. Broader PR2036 base reconciliation/CI/review
remains HOLD independently of native correctness/publication checks.

## Final verification sidecar

Native name `evidence/n83/allocation-profile-20261021/verification.tar.xz`:
**633,232 bytes**,153payload files plus manifest, SHA256
`fdf07b855f097ecb3afb5a331ae3440b43746ff2a4e53ff588d06cdec135f76e`.
Key `store/sha256/fd/fdf07b855f097ecb3afb5a331ae3440b43746ff2a4e53ff588d06cdec135f76e`.
Manifest SHA256 `a08412086fa8bbc8494346d1011b2e7cd7594701a3d0d57424f4ac74c71b90db`.
Fresh PUT/HEAD/GET, every payload, both source bundles and byte-identical repack
pass; [external receipts](sidecar-proof/). Sidecar SOURCE-PUBLICATION.bundle
retains the post-raw-publication commit, in addition to both experimental
freezes; prerequisite remains published37679b35. Final sidecar receipts and store
index update are outside that archive, avoiding recursive hashes. Shared
store/index.json includes all six new names, preserving all prior entries.

Both native arms and all requested raw records are stored and verified within
the exact declared diagnostic scope. Full-grid search, reclamation optimization,
higher conflict caps, minimum memory and complete historical cold accounting
remain open; no task or timed worker continues after publication of this result.

## Concurrent PR integration

A late independent main merge at05ec85b5287ebf137f1b1e16c8e91705d48cf77f was
preserved after the leased publication guard stopped. Original source/archives
stay immutable. See INTEGRATION.md and the dated report CLARIFICATION.md for
current canonical-graph/benchmark-admission status and separate integrated checks.

## Integrated-code supplement, after concurrent main merge

A seventh native name is now recorded:
`evidence/n83/allocation-profile-20261021/integration-verification.tar.xz`,
**608,144 bytes**,21payloads plus manifest, SHA256
`99aeb0c37f3d585368be87a66787102ccc154aa1b2e818e573ebe8600bb65c53`.
Key `store/sha256/99/99aeb0c37f3d585368be87a66787102ccc154aa1b2e818e573ebe8600bb65c53`.
Manifest SHA256 `81f0762d91a4d33617e48b89e2553ef112052d8b23135dba135975d3827159a1`.
Fresh PUT/HEAD/GET, all payloads, exact integrated source bundle, validation logs
and byte-identical repack pass. See integration-proof/ and INTEGRATION.md.
Combined code source d5aac8e9 passes4481library/119ignored,19exporter,2model-checker,
38guard unit+7card checks. Its SOURCE-INTEGRATED.bundle requires preserved
remote05ec85b5; original experiment bundles retain their original prerequisites.
No new SAT worker or cap. Prior six objects/indices and all remote store entries
remain preserved. Final publication metadata/index additions are external to
their own archives. Native validation does not resolve exact-head CI/review.
