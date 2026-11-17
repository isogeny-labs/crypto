# N83 five-base replay isolates cross-runner compiler drift

Both eight-point Tier 1 strata from [GitHub Actions run 39282996293](https://github.com/aburan28/crypto/actions/runs/39282996293) passed their individual arithmetic, resource and calibration checks, and their local independent replays reproduced the hosted analyses. The frozen aggregate returned `VERIFICATION_FAILED` because the IC and rho executable hashes differed between strata. Stratum 0 used Rust 1.99.1 and stratum 1 used Rust 1.99.0, supplied by different runner images. The aggregate recorded `same_source_and_binary: false`, `SOURCE_TARGET_OR_REPLAY_GATE`, and null online and cold selected IDs. This is a preserved failed attempt, with no five-base selection.

| Stratum | Public points | Rust compiler | IC binary SHA-256 | Rho binary SHA-256 | Individual result |
| --- | --- | --- | --- | --- | --- |
| 0 | 0–7 | 1.99.1 | `c4af7e27c0c7918c41e32cff89ec33b5e0d72ac429715831a4e8aded9ed4517a` | `4af717e15faaabf0335a194a3dd5d782519d7e15655c17390d93ce460d568b38` | `STRATUM_ADMITTED` |
| 1 | 8–15 | 1.99.0 | `4be33ac0a9ce5bc1d9f0206c5ccb58e24bbac1e75d40f43a40ffbaf7e827026e` | `4ed1335f55240ff98470caf0096b9c0cbb3da0a01b157ce88bc83cf886f761e9` | `STRATUM_ADMITTED` |

The IC source SHA-256 was `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507` on both hosts. Provider runner identities were distinct and concurrent. Both strata passed Tier 1 telemetry and every 10% calibration gate. The exact-source local verifiers each exited 0, wrote no stderr and matched their archived analyses as parsed JSON. The binary mismatch is an executed-code identity failure, so the table below is diagnostic only.

| Base | Mean cold process | Geometric mean one-target online |
| --- | ---: | ---: |
| K992/root11 dynamic | 112.227 s | 37.662 ms |
| K1024/root11 dynamic | 109.130 s | 52.266 ms |
| K3300/wide3 | 424.285 s | 10.016 ms |
| K2304/root11 dynamic | 187.762 s | 11.740 ms |
| K1107/root11 dynamic | 102.923 s | 56.988 ms |

[Validated rows](validated_rows.json) preserve all 80 IC and 80 same-point rho pairs, stage costs, candidate identities, host strata and provider binding. The [selection](selection.json) preserves the null outputs, and the [custody receipt](custody.json) binds both original provider ZIP hashes and the independently checked S3 downloads. The raw ZIPs remain at the two S3 URIs in that receipt.

The separate [binary-pinned retest](../../../n83_fresh_cold_binary_retest_20261021/PROTOCOL.md) retains the same five bases and selector on 16 newly frozen points. Its workflow builds the executable pair once and supplies those exact bytes to both measurement jobs before the new comparison starts.
