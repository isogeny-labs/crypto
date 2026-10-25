# Linux ARM XOR preflight: solver and worker passed; S3 input gate failed

The first branch-push [hosted run 40176531834](https://github.com/aburan28/crypto/actions/runs/40176531834)
used dispatch commit `642e49b23ce625c55b2dc5ef2feaa894fd4ebc49` and the
external-model worker source at `acfcd8adfad8d45ddf9c5f799f9385120dce6cf4`.
The job ended with provider conclusion `failure`. Its first seven setup stages
included a successful Linux ARM solver smoke and a successful pinned worker
build; the S3 fetch stage failed on the first `hash_5_1.xcnf` `GetObject`
before any of the 11 panel inputs could be replayed. The AWS CLI returned
code 254, but the original fetch wrapper suppressed its stderr. The provider
log therefore does not establish the underlying S3 error. No N83 relation
search ran, and `selected_best_total_runtime` remains `null`.

The provider uploaded [artifact 12592487092](https://github.com/aburan28/crypto/actions/runs/40176531834/artifacts/12592487092),
an 862,165-byte ZIP with SHA-256
`a7be830c192ac994c112a002f432f3583120b23af380d3d8b762a77ab564c560`.
Its reported expiry is `2027-01-23T08:25:08Z`. The committed
[provider jobs](attempt-1/provider_jobs.json), [artifact metadata](attempt-1/artifact_metadata.json)
and [independent replay](attempt-1/replay.json) bind all 17 ZIP members,
terminal stage statuses, source/protocol commits, exact smoke streams and
worker binary bytes. The replay returned `PASS_PRE_S3_FAILURE_REPLAY`.
A scratch artifact with an appended forged SAT line was rejected at the ZIP
digest; the [negative check](attempt-1/tamper-check.json) is retained.
The host receipt reports Ubuntu package `cryptominisat=5.11.15+dfsg1-1.1build1`
and solver SHA-256 `0936420373bbdf22c2a3d37291e3a4ab4e041d7f41ae3ceed003373fb596ddb2`;
the exact two-variable wide-XOR smoke returned SAT with exit code 10. The
artifact contains the worker binary, whose SHA-256 independently matched
`3d069ccd4b4adfb1784ed1364edc5bbdd2b656c9c7da416ebc8f22526c65d29c`.
The solver binary itself was not in this artifact, so its reported digest is
an observation awaiting byte-level replay in a successor artifact.
The exact ZIP was also stored at the SHA-named
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/outputs/preflight-40176531834/a7be830c192ac994c112a002f432f3583120b23af380d3d8b762a77ab564c560.zip`
with `If-None-Match: *`, an explicit SHA-256 checksum and AES256 encryption.
The [S3 custody receipt](attempt-1/s3-custody.json) binds the PUT, fresh HEAD
and fresh GET responses; downloaded bytes matched the GitHub ZIP exactly.
The worker role's output-prefix policy grants write without read, so this
independent read used the publishing identity.

The next branch-push diagnostic moves the S3 access probe ahead of the costly
worker build, records the AWS caller and full CLI stderr, and retains the
solver executable. It must finish the 11-object fetch and the exact
verifier-input gate before any paired Linux solver timing is authorized.
The original primary factor-base objects remain in S3; this failure does not
change their locally replayed custody or the earlier hosted access to the
canonical `a0/objects/` prefix.

Recheck the first artifact from an exact ZIP download with:

```sh
gh api repos/aburan28/crypto/actions/artifacts/12592487092/zip > /tmp/n83-preflight-1.zip
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/linux-xor-actions-preflight-20261025/audit_attempt1.py \
  --artifact-zip /tmp/n83-preflight-1.zip
```
