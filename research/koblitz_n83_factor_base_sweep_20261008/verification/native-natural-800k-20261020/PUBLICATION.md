# Native N83 800K raw evidence: sealed locally, S3 write denied

Status: native sweep and local archive verification complete; requested S3
publication blocked by the migrated AWS role's missing write permission.

- Archive: runs.tar.xz,50,310,788bytes,294payloadfiles.
- SHA256: `d15b8935a470707e572e4aa5a2bf6ab9da04270b266859fafe3f693b04b254d6`.
- Payload-manifest SHA256: `824f1e55d78a44782d3bc7d3ecd89722356a0ff2c3942244bf382bbcf1d4de8f`.
- Local path: `/home/ec2-user/scratch/n83-natural-800k-20261020/publication/runs.tar.xz`.
- Intended bucket/region: setec-astronomy-artifacts/us-east-2.
- Intended content key: `store/sha256/d1/d15b8935a470707e572e4aa5a2bf6ab9da04270b266859fafe3f693b04b254d6`.
- Intended logical name: `evidence/n83/natural-800k-20261020/runs.tar.xz`.

The native store attempt exited2 atHeadObject403 beforePUT. The actual full-
archive conditionalPutObject exited254 withAccessDenied, explicitly naming
missing identity-based permission for s3:PutObject on that exact resource.
Role: `arn:aws:sts::104186811093:assumed-role/spot-experiments-WorkerRole-G1FZ84yk3gS6/i-001adbf39a5a83f8a`.
See raw-store-push.* and raw-direct-put.*. Current store index has no successful
new entry. No successful upload, remote absence proof or fresh S3 GET is claimed.

[Local verification](LOCAL-VERIFICATION.md) passes every payload hash, both
admitted full native replays, original OOM-prefix/admission-refusal replays,
the immediate declaration refusal, exact ELF/panel comparisons and deterministic
repack. Local verification is not a substitute for the requested fresh S3 readback.
Restore intended AWS identity/bucket permissions, upload this exact immutable
archive, then verify HEAD/GET/archive and payload hashes/native replay/repack.
All five actual attempts, including failures and costs, are retained; no
missing terminal counters are replaced with zeros. Earlier publications remain
at their historical content addresses, linked in the raw UPSTREAM.md.

[Protocol](PROTOCOL.md), [8GiB addendum](MEMORY-V13.md),
[declaration correction](MEMORY-V14.md), [native summary](SUMMARY.json),
[report](../../reports/run-20261020T0600Z/REPORT.md),
[inspected PDF](../../reports/run-20261020T0600Z/report.pdf).

A local verification sidecar is also sealed at
`/home/ec2-user/scratch/n83-natural-800k-20261020/publication/verification.tar.xz`.
Its SHA256/size receipts are local-proof-sidecar.sha256 and local-proof-sidecar.bytes.
It contains the then-current compact validation/transport records and excludes
raw exec/. Its own transport receipts and later source-Git preservation metadata
stay outside that archive to avoid recursive hashes. It has not been uploaded.
