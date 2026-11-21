# Storage amendment for three frozen evidence streams

The three compressed JSONL streams first committed in `4407852121e`, `6b6e67a2c5c`, and `9ce119a9234` are stored as deterministic, line-aligned gzip shards. Their **decompressed JSONL bytes and SHA-256 digests are unchanged**. This amendment changes storage only; it does not change the frozen experiment protocols, runner sources, mathematical results, or row identities. The original whole-file packed digests remain recorded in [stream_storage.py](stream_storage.py), and the [storage result](storage-result.json) records every shard's digest and size.

The repository's trusted artifact policy moves a data-suffixed file above 64 KiB to S3. CI's artifact migration received `403 HeadObject` for these new objects; CI also requires the candidate policy to match its trusted copy byte for byte. These three streams are therefore split into shards below 64 KiB while retaining the raw logical stream:

| Stream | Original packed bytes | Shards | Largest shard | Decompressed SHA-256 |
| --- | ---: | ---: | ---: | --- |
| Local selector candidate rows | 91,340 | 4 | 28,412 | `6d7506f8bb57ca02ab4a5b509d7a00a58d45de373fc8f88e6b439f2271c445c0` |
| Wide-period selector candidate rows | 1,946,204 | 44 | 46,128 | `951b870c9c5ac12c2bb12e5f8639e3a1d3caa118f77cf2db64c01d7dbf099a1a` |
| Wide-period rational-check rows | 342,359 | 7 | 52,280 | `b2475a108e69586d99c24e1f101ffee7256111d0b0ca015078ada001677c3e87` |

On a fresh checkout, hydrate the three original filenames before running the frozen scripts, and remove the hydrated copies afterward:

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_selector_20261120/stream_storage.py verify
python3 research/exotic_ecdlp_20261021/xedni_local_selector_20261120/stream_storage.py hydrate
# Run the frozen experiment or audit commands from the relevant README.
python3 research/exotic_ecdlp_20261021/xedni_local_selector_20261120/stream_storage.py dehydrate
```

`verify` checks every shard against the committed storage manifest, checks each decompressed stream against its original raw digest, and enforces the 64 KiB per-file limit. `hydrate` reconstructs the original filenames for unchanged runners and audits. `dehydrate` verifies equality with the shards before deleting only those temporary whole-file copies. The raw evidence stream is directly recoverable by concatenating the decompressed shards in filename order.
