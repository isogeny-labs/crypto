# Independent provider replay for the fresh-trial panel

Run [audit_artifact.py](audit_artifact.py) separately for each hosted stratum.
It needs the unmodified GitHub Actions ZIP for that stratum and the exact
worker-build ZIP, SHA-256 digests of both ZIP byte streams, provider artifact
metadata for both IDs, and the provider jobs JSON for the same run and
attempt. For run `40185426122`, obtain those through the GitHub Actions API;
do not substitute a re-zipped directory or a later workflow rerun. Preserve
the fetched JSON and ZIP digests beside the replay result.

```sh
python3 audit_artifact.py \
  --stratum t1_m1 \
  --artifact-zip /path/to/exact-stratum.zip \
  --expected-zip-sha256 STRATUM_ZIP_SHA256 \
  --metadata /path/to/stratum-artifact-metadata.json \
  --build-artifact-zip /path/to/exact-build.zip \
  --expected-build-zip-sha256 BUILD_ZIP_SHA256 \
  --build-metadata /path/to/build-artifact-metadata.json \
  --jobs /path/to/provider-jobs.json
```

The auditor binds both artifacts and provider jobs to the frozen workflow
head, checks successful setup/fetch/search stages, replays the eleven S3
input identities, checks the exact source-built worker, recomputes all four
formula SHA-256 digests from the retained XCNF bytes, verifies export and
process stream hashes, and independently parses status and conflict counts
from each solver stdout. A malformed stream, changed arm order, missing
verifier receipt, or changed provider ZIP fails closed. A watchdog, OOM, or
bounded `INDETERMINATE` remains censored.

If a SAT arm contains a group-checked partial, the auditor returns
`PENDING_INDEPENDENT_SAT_GROUP_REPLAY`. That candidate still needs a second
build of the pinned worker and fresh independent archive fetch, followed by
exact model replay and curve-group verification, before it is counted in a
relation-yield or rank measurement. A passing censored stratum is evidence
only for its bounded pattern search. Neither status selects a best total
index-calculus runtime.
