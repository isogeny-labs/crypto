# Reports in S3

The [`reports-s3` workflow](../.github/workflows/reports-s3.yml) copies the
reports the `research-visuals` skill asks for to S3 once they are on `main`:

| In the repository | In S3 |
| --- | --- |
| `docs/ic/measurement/reports/<session>/` (ICMS run reports) | `…/reports/crypto/docs/ic/measurement/reports/…` |
| `research/**/reports/**` (ecbench, autolab and other run reports) | `…/reports/crypto/research/…/reports/…` |
| `research/**/*.pdf` (result and search-round reports) | `…/reports/crypto/research/…` |
| (one per push that uploaded anything) | `s3://setec-astronomy-artifacts/reports/crypto/_manifests/<commit>.json` |

Each object keeps its repository path under `reports/crypto/`. The manifest
lists every file the push uploaded with its SHA-256, size and S3 URI, the
commit, and the workflow run. Sessions never upload: the job runs on each push
to `main` that adds or changes a report, so S3 holds reports that merged.
Removing a report from the repository does not delete its S3 copy.

## Configuration

Repository variables (all optional):

- `REPORTS_S3_BUCKET`: the bucket; default `setec-astronomy-artifacts`
  (encrypted with SSE-S3, public access blocked). It is the same bucket that
  holds the content-addressed artifact store (`docs/artifacts.md`), under a
  different prefix.
- `REPORTS_AWS_REGION`: default `us-east-2` (the bucket's region).
- `REPORTS_S3_ROLE_ARN`: an IAM role the job assumes through GitHub OIDC.

Credentials, in the order the job tries them:

1. **OIDC role (preferred).** No long-lived key. Create (or reuse) the IAM
   OIDC provider for `https://token.actions.githubusercontent.com`, audience
   `sts.amazonaws.com`, and a role that only `main` of this repository may
   assume:

   ```json
   {
     "Version": "2012-10-17",
     "Statement": [{
       "Effect": "Allow",
       "Principal": {"Federated": "arn:aws:iam::<account-id>:oidc-provider/token.actions.githubusercontent.com"},
       "Action": "sts:AssumeRoleWithWebIdentity",
       "Condition": {"StringEquals": {
         "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
         "token.actions.githubusercontent.com:sub": "repo:aburan28/crypto:ref:refs/heads/main"
       }}
     }]
   }
   ```

   Set `REPORTS_S3_ROLE_ARN` to its ARN.
2. **Access keys.** Repository secrets `ARTIFACTS_AWS_ACCESS_KEY_ID` and
   `ARTIFACTS_AWS_SECRET_ACCESS_KEY` (the artifact-store keys, which already
   write this bucket), falling back to `AWS_ACCESS_KEY_ID` and
   `AWS_SECRET_ACCESS_KEY`. Whichever is used needs the permission below on the
   `reports/crypto/` prefix, or the first upload fails with AccessDenied.

Either identity needs only:

```json
{
  "Version": "2012-10-17",
  "Statement": [{
    "Effect": "Allow",
    "Action": "s3:PutObject",
    "Resource": "arn:aws:s3:::setec-astronomy-artifacts/reports/crypto/*"
  }]
}
```

With neither configured, the job uploads nothing and says so in a notice, so
`main` never turns red for lack of configuration. Credentials that cannot write
the prefix fail the job.

## Backfill and reading

Run the workflow by hand (**Actions → reports-s3 → Run workflow**) to upload
every report already on `main`. To read one:

```sh
aws s3 ls --recursive s3://setec-astronomy-artifacts/reports/crypto/
aws s3 cp s3://setec-astronomy-artifacts/reports/crypto/<path> <local path>
aws s3 presign s3://setec-astronomy-artifacts/reports/crypto/<path> --expires-in 604800
```

Use `--region us-east-2` (or `AWS_REGION=us-east-2`) if the profile's default
region differs. A presigned link opens the private object for its lifetime
(here 7 days) without making the bucket public.

## Who may write, and what a reader may do

- The **workflow** is the only writer: it runs on `main` after a report merges,
  or on a manual dispatch. It never deletes.
- An **agent session is a reader**. It may list, copy down and presign objects;
  it must not upload, overwrite or delete under `reports/crypto/`. If a report
  is wrong, fix it in the repository and let the next merge upload the new copy;
  the old object stays as history, like the artifact store (`docs/artifacts.md`).
- Reading needs `s3:GetObject` and `s3:ListBucket` on
  `arn:aws:s3:::setec-astronomy-artifacts/reports/crypto/*`; credentials come
  from the environment as usual (the `ARTIFACTS_AWS_*` keys have it).
- The `_manifests/<commit>.json` object is the authority for what a merge
  uploaded: path, SHA-256, size and S3 URI. Verify a downloaded file against
  the manifest's SHA-256 before treating it as evidence.
