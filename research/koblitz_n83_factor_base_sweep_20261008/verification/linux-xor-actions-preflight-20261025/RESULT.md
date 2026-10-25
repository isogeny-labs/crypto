# Linux ARM XOR preflight: all 11 S3 inputs and verifier gate passed

The frozen [hosted run 40177842203, attempt 2](https://github.com/aburan28/crypto/actions/runs/40177842203)
at protocol head `9b6d5f4bbc4bb9c80cf22a1c58bbfb288804f723`
finished with provider conclusion `success`. Its Linux ARM host passed the
CryptoMiniSat wide-XOR smoke, read the exact first XCNF object with the GitHub
AWS identity, built the pinned external-model worker, freshly fetched and
replayed all 11 S3 inputs, rebuilt the `hash_4_2` verifier input, and rejected
an incomplete synthetic SAT model. The complete input replay and negative
verifier gate qualify this host path for a separately frozen relation-search
protocol; they do not measure natural relation yield or full IC runtime.
`selected_best_total_runtime` remains `null`.

The provider [artifact 12593635829](https://github.com/aburan28/crypto/actions/runs/40177842203/artifacts/12593635829)
is an exact 941,393-byte ZIP with SHA-256
`00713618c7c09188a9f75fef6ce6323af376428cc48d537491394c91429d1237`
and reported expiry `2027-01-23T09:01:23Z`. The independent
[replay](run-40177842203-attempt-2/replay.json) checked the provider terminal
job, source and protocol commits, exact ZIP digest and members, solver binary
SHA-256 `0936420373bbdf22c2a3d37291e3a4ab4e041d7f41ae3ceed003373fb596ddb2`,
worker binary SHA-256 `ee471eb40a313b9b9be4e538780d2468548c323134c4d789e0db29fe7d5d18be`,
AWS caller, first-object S3 checksum, all 11 manifest tuples, and the
verifier-input gate. A ZIP with an appended forged SAT line was rejected at
the pinned digest; the [negative check](run-40177842203-attempt-2/tamper-check.json)
is retained. The complete ZIP remains available from the provider artifact;
the 11 factor-base inputs are retained in S3 at the frozen manifest keys.

Attempt 1 of the same run failed at the first S3 probe under caller
`arn:aws:iam::590183823895:user/rho-gh-snapshot-readonly-20261024`.
Its [CLI stderr](run-40177842203-attempt-1/s3-probe.stderr) identifies the
missing identity-based `s3:GetObject` grant. The exact 11-object
[read-only policy](iam-input-read-policy.json) was attached to that IAM user;
the [attachment receipt](iam-attachment.json) records live policy replay and
individual IAM simulations for all 11 allowed input ARNs plus three denied
out-of-scope probes. Attempt 2's successful first-object fetch is the hosted
check of that policy. GitHub removed attempt 1's artifact on rerun; its
[retained diagnostic files](run-40177842203-attempt-1/README.md) are not an
exact provider-ZIP replay.

Replay the successful artifact with:

```sh
gh api repos/aburan28/crypto/actions/artifacts/12593635829/zip > /tmp/n83-preflight-2.zip
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/linux-xor-actions-preflight-20261025/audit_run2.py \
  --artifact-zip /tmp/n83-preflight-2.zip
```

## Earlier first-run failure

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

The successor diagnostic moved the S3 access probe ahead of the costly worker
build, recorded the AWS caller and full CLI stderr, and retained the solver
executable. After the exact-input IAM grant, attempt 2 passed the 11-object
fetch and verifier-input gate. The original primary factor-base objects remain
in S3; the first-run failure does not change their independently replayed
custody or the earlier hosted access to the canonical `a0/objects/` prefix.

Recheck the first artifact from an exact ZIP download with:

```sh
gh api repos/aburan28/crypto/actions/artifacts/12592487092/zip > /tmp/n83-preflight-1.zip
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/linux-xor-actions-preflight-20261025/audit_attempt1.py \
  --artifact-zip /tmp/n83-preflight-1.zip
```
