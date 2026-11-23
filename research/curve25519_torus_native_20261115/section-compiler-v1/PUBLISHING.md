# Publication state

Source PR5040 merged17:43:08Z before evidence packaging. Exact54-input inventory and21,360-byte new source
delta are tracked. Native/result statement7f4e38 and written statement70247b
are unsigned; neither is an upload or signed execution receipt.

The complete one native archive is local:

- /workspace/curve25519-section-compiler-v1/native-run-v1.tar.xz
- 117571164bytes
- SHA256697a904ccfdafc58a503396ab3858055185243f2abab8e8a273c024f25525337

It contains the full218010-byte JSONL, all24 full program/assignment exports,
build/pre-call/post-call custody and actual transport logs. All43 file members
round-trip to the inspected local bytes. The archive exceeds10MiB and is not
committed. Native-v1/ARCHIVE.json lists every member; SUMMARY.json is a labelled
producer projection, not a replacement for the raw bytes. [pcc:unverified]

Trusted-store access is a real external dependency. Earlier trusted migrate
failed HeadObject403; no new unchanged S3 retry or duplicate staging was made.
Do not infer a missing key, invent an S3 location/receipt, change policy, split
or recompress around selection, export credentials, or delete remote objects.
After access repair, use maintained trusted upload plus full remote replay,
then maintained manifest/gitignore helpers for the same object and original
canonical artifact path while keeping the inspected local bytes.

HTTPS Git authentication also became unavailable during source authoring.
The connected GitHub blob/tree APIs supplied the exact local tree through a
normal merge commit and atomic absent-branch creation. Every uploaded blob SHA
and tree008d04f6 matched; no force push, rebase or gate bypass occurred.
[pcc:unverified]

PR5022 merged while compiler authoring was in progress; its terminal tracking
merge is not publication of this compiler. PR5040 was created draft and merged
by an external action while the authoring work continued; this session did not
ready it, enable auto-merge or issue its merge. Publish this evidence/report as
a separately tracked continuation and keep its actual artifact requirements
explicit. No signed standing,
successful applicable gate or merge authorization is inferred from this note.
