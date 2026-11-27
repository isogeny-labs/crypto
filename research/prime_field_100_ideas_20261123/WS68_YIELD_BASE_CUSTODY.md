# Exact custody for yield-triggered base policy raw output

The frozen #96 run emitted `WS68_YIELD_BASE.stdout` as 331,949 bytes, SHA-256
`9a56dd485f4c84456673f68bed2930522eb642b12f6ae77e796ff0de8ed93e31`.
The original [post-call receipt](WS68_YIELD_BASE.post-call.json) is unchanged.
That monolithic file remains local and is excluded from Git by an exact entry
in the repository's local `.git/info/exclude`. It must not be staged. The eight
shards below preserve its bytes in target order, including the initial run
header in the first shard and each target's three arm summaries after its
512 candidate lines.

| Target | File | Bytes | SHA-256 |
|---:|---|---:|---|
| 123 | [target123](WS68_YIELD_BASE.target123.stdout) | 41,526 | `604bd9abd4e9e6148f950544ece0422c017752587114ccb08ebfff62d51a2c0e` |
| 277 | [target277](WS68_YIELD_BASE.target277.stdout) | 41,386 | `05d9f4d3337108f9229e348a6baaa7182cc2274c3f7cddc0a497de2affba90c9` |
| 411 | [target411](WS68_YIELD_BASE.target411.stdout) | 41,413 | `860c6172bc48ca52d9023407fe801b10225030df3306b255390ca995b8561564` |
| 509 | [target509](WS68_YIELD_BASE.target509.stdout) | 41,412 | `bb210bc4512529c18802a33e0a0f5f6846d2b24be2881097e58d168aa2d43285` |
| 701 | [target701](WS68_YIELD_BASE.target701.stdout) | 41,418 | `b77cd2f67732eab3f938cea5d4ed33b465b9124f6196ac868a5b8c8d1df15e98` |
| 827 | [target827](WS68_YIELD_BASE.target827.stdout) | 41,420 | `0f148764eb047d037ef7084196db7bae960eca29d40783565477922af7fde7c8` |
| 941 | [target941](WS68_YIELD_BASE.target941.stdout) | 41,456 | `9515cc0662a20d70e069bf7d361ac72494e8fbeae0ef3ff765287daeb81f2d1d` |
| 1003 | [target1003](WS68_YIELD_BASE.target1003.stdout) | 41,918 | `ed4003f89a8c54c0debdb822260c847e992c8929e5f6887c6978aea74da94c04` |

All shards are under 64 KiB. The sum is exactly 331,949 bytes. Reconstruct
the original byte stream from this directory with:

```sh
cat WS68_YIELD_BASE.target123.stdout WS68_YIELD_BASE.target277.stdout WS68_YIELD_BASE.target411.stdout WS68_YIELD_BASE.target509.stdout WS68_YIELD_BASE.target701.stdout WS68_YIELD_BASE.target827.stdout WS68_YIELD_BASE.target941.stdout WS68_YIELD_BASE.target1003.stdout | shasum -a 256
```

The observed concatenation SHA-256 is
`9a56dd485f4c84456673f68bed2930522eb642b12f6ae77e796ff0de8ed93e31`,
equal to the post-call receipt and local original. Each shard was split at the
first `candidate target=<next target>` line; no run byte was edited.
