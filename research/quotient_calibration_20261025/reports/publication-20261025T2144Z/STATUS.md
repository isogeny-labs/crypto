# Approved merge and archive-publication follow-up

The user explicitly approved both enabling auto-merge for PR #2569 and
uploading the exact 67,540-byte validation archive to
`s3://setec-astronomy-artifacts/store`. The earlier approval blocker is
resolved. This does not waive current-head review, CI, archive integrity or
remote-readback requirements.

The archive still has SHA-256
`ae5e8b6973db769db3ccef01fb81ba5837702a26221206134091a8bdff3bad2a`.
No archive contents, source input, scientific result or frozen receipt changed.

Execution remains blocked by access as of 2026-10-25 21:44 UTC:

- GitHub connector and CLI repository/PR reads return HTTP 404. Git's remote
  read returns HTTP 403, `Write access to repository not granted`.
  The current PR head, reviews and checks cannot be verified, so auto-merge
  was not enabled from a stale snapshot. Its last verified state was disabled.
- The AWS CLI was installed in the workspace. A normal SDK S3 head request
  exits 255 with `Unable to locate credentials`. The repository's artifact
  push, using the approved exact remote and archive, exits 2 with the same
  credential error before uploading or generating a manifest. The cloud
  environment reports no configured AWS identity or identity manifest.

[STATUS.json](STATUS.json) records the exact scope, hash and observed errors.
The [earlier raw-log publication record](../run-20261025T1845Z/ARCHIVE_PUBLICATION.json)
is preserved as historical evidence; it describes the state before approval.
The archive remains local and unpublished. No merge, upload or remote
readback success is claimed.

Required next input is restored GitHub repository access and supported AWS
access to the approved bucket, supplied through the connections/environment
configuration. No further permission is needed for the approved actions.
Once access is restored, verify the live PR, enable auto-merge, upload the
unchanged archive, verify a fresh download by SHA-256, then publish the
generated artifact manifest and this status update on the existing branch.
