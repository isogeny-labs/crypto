# Setup-reuse lifecycle follow-up — 2026-10-30

Only crypto PR2906 and cryptanalysis PR1309 were checked. No research,
benchmark, challenge solve, new improvement or cloud resource was started.

## Crypto conflict resolution

Previous published head3520b4cb5f1142e3f1be0fbc107f893f3fcef1c7 has current-head
gate,size,artifacts SUCCESS, no submitted reviews or inline threads. Main
advanced to cfedbfe26 and created two conflict regions in the unsealed
October30 session log. The original Copilot request remains unanswered;
reasonable-wait fallback was already explained, then reaffirmed in PR comment
6515356009. No active second resolver was observed.

Merge5fb9e4da7cfae849688d7b04f5354f02beed69b2 preserves every line, including
newlines, from both parents' October29 and October30 logs in original order.
Only conflict markers were removed. No frozen evidence conflict was selected.
Both resulting logs were checked as ordered supersets of complete parent
sequences before committing.

All19 entries of docs/hashed-ecdl-sqrt-reuse-main-sources.sha256 still match.
Manifest SHA256 remains
ed03771069fe437d7586d3a88f509c92cf73ae879f3d3ff38b7c88815a5225e5.
Root/native Cargo manifests and locks are unchanged from c84df306c, and no
changed tracked path from that tested commit appears in current root release
dependency files. Native code and all tested library inputs therefore remain
unchanged. No new broad panel was run. The uncapped library result remains
5430passed,0failed,123pre-existing ignored, actual parent/terminal0; its
487100-byte raw log remains NOT_UPLOADED with the earlier committed digest.
The existing47 native tests and3 panel tests, lint, MSRV and API evidence
retain their original source scope. Earlier receipts/manifests were not edited.

Fresh required checks must pass on the new published head. Previous-head
success is not a new-head gate pass. Actual protection, required reviews and
blocking feedback still apply; normal MERGE uses an expected-head guard.

## Cryptanalysis merged outcome

PR1309 merged2026-10-30T04:31:20Z, commit
82eb1eacf4130ef35ccc9cda4792a6017ab0fa08, parents
5985261c503158fa5e89592d9b88facdeb38a02e and final head
1a4870668442a542c356ef1538135c96767d537b. The actual tree
3d3c7511992c358c7d8f9218fd82410b156918fc equals the deterministic merge
of those parents. The final seven-file PR diff matches the reviewed local
patch;48986bytes, SHA256
5ab06c6c5a5749d61346ed4ce97d35eeb910ff87eaed5e5de58e4cf96168bfec.
No submitted reviews or inline threads. Final-head checks succeeded; opcounts
is legitimately excluded by its pull_request condition, and the CodeQL
aggregate is neutral while its individual analysis jobs succeeded.

The merged task source blobs equal the final PR head:

- arithmetic.rs:1ce4b20ac752f2a31cec59feac7ff3f96d7fb98158f9c2bdc00927c95b8f5bc4
- curve.rs:c73dd7383207e6ff83450b01c30aa4002334261930928b525292631ff657187f
- reduction.rs:86050430ae0245f10e70a11827d80020e74ea1514df52cb8dbc7d1a9c1e5da76

The shared square-root fragment remains SHA256
f0ea5dfb64c9dd96e9e024cefcd939815bbc58819fe8e2702fa61cf88db6dd5c;
the shared decoder remains SHA256
86f04424807f17ad947588c26a4105e70e89fd40e1aeb02c0ac4fc65f0577642.
All three original reuse receipt files remain byte-identical, including the
17-file source snapshot07014e0ba3f11e3624ec42d630f23b90d7e83159d477cfbb88a95ed80b6b4fdc.
Main brought additional independent native features, so neither that historical
manifest nor the earlier106-test preview is claimed as current whole-tree
validation. The native CI job succeeded on the actual final PR head.
The merged outcome was recorded on PR1309; tracking stops there.

## Errors and scope

The merged PR no longer has refs/pull/1309/merge; fetch was repeated with main
and its retained head, then actual merge objects were verified directly.
The first local resolver expected one conflict block, found two, and stopped
before writing. The resolver then handled both blocks and checked all four
parent-log sequences before commit. No marker-containing file was committed.
Conductor returned127; documented warn-and-allow fallback, no reservation.
Full independent theorem/complexity audit remains open. No timed performance,
whole-tree cross-repository parity, sub-rho or deployed-curve claim is added.
