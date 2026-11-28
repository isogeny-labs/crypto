# Complete raw containers and canonical publication

All fifteen declared containers remain in full at immutable public checkpoint
[`265b97cabcc41ad60ba86d2d769b4cc24836c4ea`](https://github.com/isogeny-labs/crypto/tree/265b97cabcc41ad60ba86d2d769b4cc24836c4ea/research/chained_richelot_map_20261117).
`IMMUTABLE_RAW_ARTIFACTS.json` records every original path, complete byte length,
SHA-256, Git blob OID and direct immutable URL. The complete bytes were read and
checked while preparing the canonical import. No container is replaced by a
hash-only scientific record or discarded. The successful run retains its entire
source, all three rlibs, both compiled/copied ELF executables and every trace.
All failed runs and the full old PCC/journal/tool/index fork are retained too.
[pcc:unverified]

The current main artifact policy is unchanged. This PR adds source, uncompressed
scope/custody metadata, complete native stdout and inspected PDFs. It references
the already published complete containers instead of introducing new inline
compressed datasets. S3 migration remains open: parent4514's complete log records
`InvalidAccessKeyId` on GetObject and nine archive files belonging in the store.
No successful S3 upload or independent signed execution is asserted.
[pcc:unverified]

For a fresh clone, fetch the immutable checkpoint through the existing origin,
then restore only the declared containers needed for the review:

```sh
git fetch --filter=blob:none --no-tags origin 265b97cabcc41ad60ba86d2d769b4cc24836c4ea
python3 research/chained_richelot_map_20261117/materialize_checkpoint_archives.py --only complete-first-map14-F-source-full-rlibs-both-ELFs-and-all-traces.tar.xz --fetch
```

The materializer performs Git I/O and exact length/SHA-256/Git-object hash checks;
it refuses to replace existing different bytes and invokes no research code.
The original declared archive paths must be materialized before direct local
proof-file verification. `--all` restores the entire fixed list when space permits.
These local hydration caches are ignored so a later add cannot inline them by
accident. The frozen scientific Rust/caller/lock inputs remain byte-identical to
the pre-run source checkpoint `f35867831245560527428e3562ab55d74eefb5a0`.
[pcc:unverified]
