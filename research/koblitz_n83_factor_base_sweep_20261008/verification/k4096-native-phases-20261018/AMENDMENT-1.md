# Frozen input-path amendment after infrastructure failure

The first paid ephemeral attempt, job
`20261019-015400-n83-k4096-phases-5438e8`, stopped before building or
searching. Its instance role was denied `s3:GetObject` on the originally staged
`n83-frozen-inputs/...` key. This is an IAM input-path failure, not an IC
algorithm outcome. The runner's documented role grants `s3:GetObject` on
`jobs/*` and `cache/*` in its bucket (`taskq/deploy/ephemeral/main.tf`).

For one bounded retry, the same 20,621,466 compressed bytes and SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
were stored with `If-None-Match: *` and AES-256 under
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261019-015400-n83-k4096-phases-5438e8/input/ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad.jsonl.gz`.
`launch/allowed-input-head.json` records size, digest metadata, encryption,
and ETag. The retry changes only the S3 input path in `run_ephemeral_native.sh`.
The public fixture, factor-base bytes, m, resource envelope, wall caps,
conflict cap, source instrumentation, and audit rules remain frozen as in
`PROTOCOL.md`. A later result must retain the failed attempt's receipt and
cannot interpret its exit as a zero-yield search.
