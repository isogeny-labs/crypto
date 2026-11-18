# K600 artifact-store custody addendum — 2026-11-15 UTC

This is a post-run storage event. It does not change the precommitted
[protocol](PROTOCOL.md), the local-store [result](RESULT.md), or any timing
interval. The full primary K600 base was constructed under `file://` at source
`498efdc53e821edb24fcebf0826ff94549ce266b`. Commit `77759a467fc1` put
the compressed object and report graph in the PR. The repository's artifact
automation then added commit `83703966a1e462b47207d91709dddf0b24584e91`,
moving those bytes to its configured S3 artifact store and retaining
content-addressed [object](attempt-1/objects/.artifacts.json),
[run graph](../../reports/run-20261115T1209Z/.artifacts.json), and
[benchmark graph](../../../../benchmarks/20261115-n83-k600-v3-local-custody/.artifacts.json)
manifests in Git.

| Check | Observed |
|---|---|
| Artifact-store remote | `s3://setec-astronomy-artifacts/store` from `.artifacts/config.json` at PR head `83703966a1e4` |
| Object key | `s3://setec-astronomy-artifacts/store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` |
| Tracked pointer | `attempt-1/objects/.artifacts.json`: 3,025,798 bytes, SHA-256 `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` |
| Independent remote HEAD | `ContentLength=3025798`; HEAD did not return a SHA-256 checksum or user metadata, so its ETag is not treated as the content digest |
| Fresh S3 GET | `ecc-challenges-new` (`arn:aws:iam::104186811093:user/coreuser`) downloaded 3,025,798 bytes to a new temporary path; direct SHA-256 was `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`; `gzip -t` exited 0 |
| Browsing PNG | Both Git pointers name the same 124,420-byte object; a separate fresh GET of `store/sha256/0e/0e85bf50c64bca559c283cf71c299255ace381ad8c663973a1c9a7fa97c6896e` had the manifest SHA-256 and decoded as a 1,952 × 623 PNG |
| Relation and IC result | No relation search or cold IC timing was performed by the post-run storage event |

Reproduction from an authorized read identity:

```sh
aws s3api head-object --profile ecc-challenges-new \
  --bucket setec-astronomy-artifacts \
  --key store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
aws s3api get-object --profile ecc-challenges-new \
  --bucket setec-astronomy-artifacts \
  --key store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf \
  K600-v3.jsonl.gz
shasum -a 256 K600-v3.jsonl.gz
gzip -t K600-v3.jsonl.gz
```

The exact new S3 object is now durably accessible from this read identity and
identical to the locally constructed and historical K600 compressed object.
This archive upload was the repository bot's PR artifact migration, not a
native v3 `Store::push` to the selected factor-base logical name. It does not
establish the authorized writer/destination for future N83 publications, a
fresh S3-backed v3 importer receipt, useful rank, target descent, or a
minimum-runtime factor base. Those obligations remain open.
