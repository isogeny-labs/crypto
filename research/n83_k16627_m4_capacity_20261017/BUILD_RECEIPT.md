# Native m=4 adapter worker build and input staging

The frozen [protocol](PROTOCOL.md) was committed as `791bcf70ed` and its
[control clarification](CONTROL_ADDENDUM.md) as `f74f6bc43a`, both before
the first capacity worker. The five guarded m=4 adapter edits were committed
as `8adde7c049` before this build. No cryptographic worker has yet run from
this receipt.

The Linux arm64 release build used the pinned offline
`rust:1.93.1-bookworm` image digest
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
The source, Cargo registry and Git dependency cache were mounted read-only;
the target was separately writable. Network was disabled. The container had
four CPUs, 8 GiB hard memory, zero swap, and a 600-second `timeout` wrapper.
The exact inner command was:

```sh
cargo build --release --offline --locked --no-default-features \
  --example koblitz_n83_factor_base_export
```

The [stderr log](linux-build.stderr.log) records a completed release build
in 3m37s. The [container state](linux-build.container-state.json) records
exit 0 and no OOM. The resulting arm64 ELF binary SHA-256 is
`2f05a22a6365768e138c64f98275e1fe9a3d1dbe1ec1d780603c107260b3fd0a`.
The macOS release example tests passed 19/19 against the same committed
adapter source; see [test log](example-test.log). `rustfmt --check` passed on
the touched Rust file, and repository-wide `cargo fmt --all -- --check`
passed. These are build and correctness gates, not model-capacity results.

The worker input directory was staged outside Git from the K=16,627 object
and committed receipts in PR #2124, plus the pinned public corpus. The
staged SHA-256 values, measured before any worker, are:

| Staged input | SHA-256 |
| --- | --- |
| `manifest.json` | `0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275` |
| `replay.json` | `75b3bc17d8366e258e1b8f3f8de5f42911eb893c3d029a2e40c65b9e59cffe60` |
| `upload-receipt.json` | `70e82dfef4ad735a6974b46a4d6bca726dba4cc68be5f35ca354fab3262b445b` |
| `probe-corpus.json` | `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd` |
| `objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz` | `2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369` |

The source and input digests remain the identity checks for the capped
worker. The local path is a staging location; the content-addressed object
and custody receipt in PR #2124 are the durable reference.
