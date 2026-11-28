# Exact custody of the #97 interval-policy raw output

The original one-shot native stdout was `WS68_INTERVAL_POLICY.stdout`, 173,772 bytes, SHA-256 `c4dca6d76edabc364e6c51e7d85587adc8113a28a128dc3b65dcbc686614f22d`. Its frozen source, qualifier, pre-call input and post-call result are unchanged. That output was initially committed in draft PR #5165. The PR artifact-migration job received HTTP 403 on `HeadObject` for this previously absent content-addressed object, so the store could not accept or replay it. The original output was then partitioned at the eight frozen target boundaries into the records below. Each shard is below the repository's 64 KiB data-file threshold; no bytes were changed or omitted. This is a custody repair, not a new experiment. `[pcc:unverified]`

| Order | Target | Byte count | SHA-256 | Record |
|---:|---:|---:|---|---|
| 1 | 123 | 21,834 | `e218e98e5915ecece70f1f6a0e0ec7b61b206b397e69853c8eef9e1508b77474` | [target 123](WS68_INTERVAL_POLICY_PARTS/target_0123.stdout) |
| 2 | 277 | 21,635 | `ea8faf9f3d115daf5a2d7e44b9ea004eff977793f49eb9860009b20a0c77467e` | [target 277](WS68_INTERVAL_POLICY_PARTS/target_0277.stdout) |
| 3 | 411 | 21,633 | `921e61e5e6433bf636b605948d9de6ae57aa3f35344adbcfb1cb756ada68f98c` | [target 411](WS68_INTERVAL_POLICY_PARTS/target_0411.stdout) |
| 4 | 509 | 21,619 | `e27ebceb9cf4c6137f5c4316dffdc116f71c39a4a5e86d86e80ce7c9e8b8aa9d` | [target 509](WS68_INTERVAL_POLICY_PARTS/target_0509.stdout) |
| 5 | 701 | 21,616 | `508c2f51190740b707ba1288850854f8693544f28dbe5c3441b44b0cf53c9bca` | [target 701](WS68_INTERVAL_POLICY_PARTS/target_0701.stdout) |
| 6 | 827 | 21,599 | `ca3a7e2fa0211d5b90385113c472f08a9b9cd1e4b7557a63c47e3a5172e9ad0f` | [target 827](WS68_INTERVAL_POLICY_PARTS/target_0827.stdout) |
| 7 | 941 | 21,594 | `af87b7eb985dc65958152ff3e993d7edbe297dd01574114ae45af315902c8f60` | [target 941](WS68_INTERVAL_POLICY_PARTS/target_0941.stdout) |
| 8 | 1003 | 22,242 | `910d554084cbd5eec17de80761f90068ea2e80484b5b485f1cd4dc397049e430` | [target 1003](WS68_INTERVAL_POLICY_PARTS/target_1003.stdout) |

The first shard includes the original one-line header; the last contains the original trailing claim-boundary line. Every target shard preserves 512 candidate lines and three arm lines, in original order. Reconstruct and check the exact original bytes from repository root:

```sh
cat research/prime_field_100_ideas_20261123/WS68_INTERVAL_POLICY_PARTS/target_*.stdout | shasum -a 256
```

The expected digest is `c4dca6d76edabc364e6c51e7d85587adc8113a28a128dc3b65dcbc686614f22d`. Recomputed length is 173,772 bytes, with 4,096 candidate lines and 24 arm lines. The historical unsigned claim `PCC-20261125-e6787f` pins the monolithic path and therefore does not resolve in the final Git checkout; it remains in the append-only ledger as the original producer record. The new unsigned claim `PCC-20261125-406cc8` pins all eight shards. The post-call receipt still describes the one-shot run, and this note describes its subsequent byte-preserving custody change. Neither claim is a signed verdict. `[pcc:unverified]`
