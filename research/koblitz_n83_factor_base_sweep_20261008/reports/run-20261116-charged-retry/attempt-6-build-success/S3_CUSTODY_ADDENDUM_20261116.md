# K600 build artifact S3 custody addendum

The repository artifact bot commit `3139c5e2f12b1a96ba39885993020baeb47b98d0` moved five of the six Linux ARM64 executables from the PR to content-addressed objects under `s3://setec-astronomy-artifacts/store`. The bot committed [`.artifacts.json`](binaries/.artifacts.json) with their SHA-256 and byte counts. The smaller `budget-controller` remains in Git. This addendum leaves the earlier [build report](REPORT.md) and its measured intervals unchanged.

On 2026-11-16 UTC, `ecc-challenges-new` (`arn:aws:iam::104186811093:user/coreuser`) performed a fresh `GetObject` for each exact key. A local `shasum -a 256` and `wc -c` check passed against the committed manifest for all five downloads. The [machine-readable read-back receipt](S3_READBACK_20261116.json) records the check time, source and bot revisions, exact object keys, expected digests and sizes. The AWS profile name identifies the read identity used for this custody check; it does not authorize new factor-base writes.

| Executable | Bytes | SHA-256 | Read-back |
|:--|--:|:--|:--|
| `arm-supervisor` | 1,764,136 | `e3fcc0da60417838bdbd8dcf33b28d3c35e4be495533890404e32e5f54d8d906` | PASS |
| `exporter` | 9,770,336 | `44ab0d6927ee3ab5c79afbaf62b6922cb98591c739e5af87a06a445d367a9852` | PASS |
| `preflight-supervisor` | 1,696,440 | `867f36e7e65610260f9d3413ddcccbf19f4b0df984fa655bc45892baa86b901f` | PASS |
| `solver` | 5,133,168 | `1aaec2a6ffdfd215075ce8cc7c093e2d884904a161fbf3b77db70c82fc478444` | PASS |
| `worker` | 2,655,768 | `d6aad2f3737eff25e2d3759e204b6714e83c58212017b39a650414c7b6ac269b` | PASS |

The exact key for each row is `store/sha256/<first-two-digest-characters>/<digest>` in bucket `setec-astronomy-artifacts`. Reproduce one row with `aws --profile ecc-challenges-new s3api get-object --bucket setec-astronomy-artifacts --key <key> <output>`, then check its digest and size against `.artifacts.json`. This is artifact custody verification only. A later AWS host must still establish its own source and image lock, pull the frozen K600 input, and pass the zero-trial preflight before worker execution.
