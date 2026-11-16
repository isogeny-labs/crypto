# Four approved artifacts published and verified

The exact four files in the [frozen approved proposal](../s3-yield-20261024/ARTIFACTS_UPLOAD_PENDING.json)
are now published under their SHA-256 object keys in
`s3://setec-astronomy-artifacts/store`: **684,341 bytes, four uploads**.
The unmodified pinned repository tool exited0 and created two genuine
`.artifacts.json` hydration manifests. Its remote verification reports four
entries and zero problems. Fresh downloads of all four objects independently
match their approved byte counts and SHA-256 digests; S3 also returns the
matching SHA-256 checksums. No report, figure or archive was regenerated.

[Status and exact objects](status.json) · [upload log](upload.log) ·
[tool verification](verify.log) · [fresh-download hashes](remote-verification.json) ·
[approved hashes](payloads.sha256) · [execution source](credential-session.py.txt).
Individual `get-0.log` through `get-3.log` retain successful S3 responses in
the proposal's payload order. Their exits and both main command exits are0.

The first command lacked command-level network permission and could not
connect to the inherited proxy; [that attempt](attempt-01-network-sandbox.log)
is retained with exit2. A normal proxy-path HEAD with network permission
returned the bucket region `us-east-2`. The same approved upload then ran
with network permission, that region and the inherited proxy/CA. This was
not an S3 credential rejection and no alternate destination or identity was
used. Both the upload and verification commands had45-second caps; neither
cap was reached. The four GET commands each had the same cap.

Credentials were supplied only to a short-lived process and its AWS CLI
children. No credential value, identifier, session token, credential file
or chat transcript is included in this record. No persistent AWS connection
was configured and no further credential reuse is scheduled. The older
[missing-credential attempt](../artifact-upload-20261024/README.md) and the
original proposal remain historical records rather than current blockers.

The tool added managed ignore blocks duplicating the exact filenames already
ignored by the two original `.gitignore` files. Those files are pinned by the
frozen OUTPUTS manifest. The [generated patch](tool-generated-ignore.patch.txt)
is retained here and the original ignore bytes restored, preserving every
frozen checksum and the same ignore behavior. Both new artifact manifests
remain byte-for-byte as emitted by the tool. This does not alter the tool,
its policy, the remote objects or its verification.

To hydrate from a checkout with AWS access, use the committed pinned tool:

```sh
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store \
  python3 research/quadratic_twist_evidence_20261021/validation/s3-yield-20261024/artifacts.py.txt pull \
  research/quadratic_twist_evidence_20261021/reports/s3-yield-20261024 \
  research/quadratic_twist_evidence_20261021/validation/s3-yield-20261024
```

Set `ARTIFACTS_AWS` and a writable `ARTIFACTS_CACHE` when required by the
local environment; supply credentials through its normal AWS setup. The
manifests specify the exact bytes rather than mutable object names.

This is artifact publication, not another research experiment or speed
result. All earlier study data, proofs, certificates, Cargo inputs and
frozen INPUTS/OUTPUTS bytes remain unchanged. The report and diagrams are
unchanged because no mathematical finding changed. The prior185-declaration,
4,479-pass library,22-backend and four-YAML receipts remain their original
source-validation records, not fresh resulting-head CI. Current publication
checks cover the four local/remote hashes, manifests, frozen checksums,
artifact policy, names and the final diff.

The actual shared Conductor check still exits127; no reservation or local
replacement was claimed. The PR remains open for its unchanged research,
Conductor, exact-head CI/access, review and merge/integration gates.
