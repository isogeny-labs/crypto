# Preserved unpublished transport attempt — 2026-11-23

**Historical administrative evidence only [pcc:unverified].** These files preserve the local `PCC-20261123-c7e76b` attempt after HTTPS git push failed for missing username/credentials. They do not register that claim in the active ledger. The published receipt is `d532700090da7a3d35e5c1b03f081a0783dbd6ac`, and the new active registration uses `PCC-20261123-c7e76c`.

| File | Exact origin | Bytes |
| --- | --- | ---: |
| `receipt-90522ccb.commit` | `git cat-file commit 90522ccbf6c70e9c0a7a5b51c14b6a7a6c7a6a9f` | 253 |
| `registration-485a0bf.commit` | `git cat-file commit 485a0bf233954219ab735d2aee0eeaf99143b8e5` | 252 |
| `registration-90522ccb-to-485a0bf.patch` | `git diff --binary 90522ccbf6c70e9c0a7a5b51c14b6a7a6c7a6a9f 485a0bf233954219ab735d2aee0eeaf99143b8e5` | 15,119 |
| `PCC_PRIME31_ADMISSION_20261123.json` | Original committed native registration output from local `485a0bf` | 2,765 |

The raw commit records preserve author, committer, timestamp, parent, tree and message bytes. Their Git object hashes were checked with `git hash-object -t commit` and match the original commit IDs. The local receipt tree `ce74e476321857abd077244b8b90137707f379af` exactly matches the published receipt tree. The patch preserves the subsequent metadata delta to tree `ba2797ed96b8925458684308aae611497cda2939`, including the old claim and append, without placing them in the active `pcc/` paths. Together with the existing repository objects and identical published receipt tree, these records preserve the information needed to reconstruct the original local attempt in a separate historical checkout.

`SHA256SUMS` records native SHA-256 hashes of all four exact historical files. Their individual paths must resolve for that manifest, so they remain loose under the AGENTS §13a path exception. Each is below 64 KiB; no Git bundle, large archive or compressed workaround was created. The original local commits and worktree were not altered.

To check these historical file bytes:

```sh
sha256sum -c SHA256SUMS
```

Do not apply the historical patch to the active ledger or copy its claim into active `pcc/claims/`. It records the preserved unpushed attempt only. See [the transport note](../../REGISTRATION_TRANSPORT_20261123.md) for the actual published-receipt binding. No new research execution, signature, verdict or standing upgrade is represented here.
