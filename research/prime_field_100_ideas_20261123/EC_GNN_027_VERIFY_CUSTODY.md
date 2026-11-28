# Selected-ID stdout custody: eight target-boundary shards

Date: 2026-11-25 (America/Los_Angeles), draft crypto PR #5165. This note follows the [#27 independent replay protocol](EC_GNN_027_VERIFY_PROTOCOL.md). The output-only emitter's original stdout is 438,641 bytes, above the 64 KiB artifact-migration threshold. S3 artifact migration has returned HTTP 403 for missing objects. Keep the [original local monolithic stdout](EC_GNN_027_VERIFY_EMIT.stdout.txt) byte-for-byte on this host, SHA-256 `02e7e8517aeaf3da60e2b23c0b4d70225828f21488d4c099d90a194425eab7f8`, **unstaged**. It is not required in the PR because the eight tracked shards reconstruct it exactly. This is a custody change only; the frozen emitter source, original raw, and original GP qualifier result are unchanged.

The stdout has three preamble lines, eight consecutive `model_ids_uN=[...]` lines, then arm metrics. Shard 01 contains lines 1–4 (preamble plus target 1); shards 02–07 each contain exactly one next target-ID line; shard 08 contains line 11 through EOF (target 8 plus all arm metrics). Each target ID vector stays whole. `head`, `sed`, and `tail` copied the original LF-delimited byte lines without editing content. A bytewise `cmp` of the original against concatenation returned 0, and `cat shard01..08 | shasum -a 256` returned the original SHA-256.

| Shard | Bytes | SHA-256 |
|---|---:|---|
| [01](EC_GNN_027_VERIFY_EMIT.shard01.txt) | 54,839 | `bda1f04846091bedfc8d50957cdcfd3f00878406d5d697ca13e5931908850de1` |
| [02](EC_GNN_027_VERIFY_EMIT.shard02.txt) | 54,401 | `1f97292851570c6ec66c7168329069f16ecbfe74cb89ffc35318905a6ef85f16` |
| [03](EC_GNN_027_VERIFY_EMIT.shard03.txt) | 54,401 | `b8c5b8918350d52711da12e05eb2736371ab1e8dfd116a45cd851b15a2604c6f` |
| [04](EC_GNN_027_VERIFY_EMIT.shard04.txt) | 54,401 | `899b05926abf8c2557c563a0c1758029fea7b1cf29ab962d9968667d4cf87e0f` |
| [05](EC_GNN_027_VERIFY_EMIT.shard05.txt) | 54,401 | `d4b8f23e0d948f444ce5833bfc01592bc64f4d2fc3111c6a9c5db1221d3c2505` |
| [06](EC_GNN_027_VERIFY_EMIT.shard06.txt) | 54,401 | `18e16a3619bff8cf94b5adb5f8af867989a0df735a89e17252838037c26b6f8e` |
| [07](EC_GNN_027_VERIFY_EMIT.shard07.txt) | 54,401 | `6d3b1625eae2bb92e4dc0b472391c89fb7813213c531eabd4db58819a2630501` |
| [08](EC_GNN_027_VERIFY_EMIT.shard08.txt) | 57,396 | `3d2b3a6823ce6e48a197948e0a072f3115ac05264a73cb0f273f4c` |
| **Concatenation** | **438,641** | `02e7e8517aeaf3da60e2b23c0b4d70225828f21488d4c099d90a194425eab7f8` |

All eight shards are below 65,536 bytes. From a fresh checkout, reconstruct only in an ignored local temp path and verify the digest before replay:

```sh
cat research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_EMIT.shard{01,02,03,04,05,06,07,08}.txt > /tmp/ec_gnn_027_verify_emit.reconstructed.stdout.txt
shasum -a 256 /tmp/ec_gnn_027_verify_emit.reconstructed.stdout.txt
gp -q < research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP_SHARDS.gp > research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP_SHARDS.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GNN_027_VERIFY_GP_SHARDS.stderr.txt
```

The [shard-aware qualifier source](EC_GNN_027_VERIFY_GP_SHARDS.gp) has SHA-256 `9e0dc014b8e670da0853a066d3db838b739136380b9c7487a1de619677183b42`. Its diff against the original GP qualifier changes only the `readstr` input path to `/tmp/ec_gnn_027_verify_emit.reconstructed.stdout.txt`; all point anchors, ID parsing, scalar relation labels, full-grid scan, and rank checks remain identical. Invoke it once after the reconstructed SHA matches, preserve stderr and shell status, and compare its exact per-target/summary lines to the original qualifier. The local `/tmp` reconstruction is disposable; the shards plus their hashes are the portable custody record. `[pcc:unverified]`

## Shard reconstruction and replay receipt

The eight shards were concatenated to the stated `/tmp` path. Its SHA-256 matched the original and bytewise `cmp` returned 0. The shard-aware GP qualifier then ran once with shell exit code `0` and empty stderr. Its stdout was byte-identical to the original GP qualifier stdout (`cmp` exit code `0`). The exact summary line was `verify_summary selected_ids=65536 selected_hits=51 selected_coverage=4 selected_rank=9 full_hits=223 full_coverage=7 full_rank=22 false_negative_hits=172 rank_loss=13 point_logs_anchored=24`.

| Receipt | SHA-256 |
|---|---|
| [Qualifier stdout](EC_GNN_027_VERIFY_GP_SHARDS.stdout.txt) | `0cfe33e065ed77355c10e0bec9f5f00924cfa2c8fbba1148af32e4d1d7cf9b8b` |
| [Qualifier stderr](EC_GNN_027_VERIFY_GP_SHARDS.stderr.txt) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Qualifier exit code](EC_GNN_027_VERIFY_GP_SHARDS.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |

This receipt verifies custody and reproduces the original independent scalar replay from portable shards. It does not add a new model run or change the frozen model result. `[pcc:unverified]`
