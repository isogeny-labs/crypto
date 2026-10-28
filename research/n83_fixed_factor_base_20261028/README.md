# Shared n83 factor-base points for m=2..9

Fixed ell=27: enumerate all 134217728 candidate x-values on each of the original
Koblitz, general binary and prime curves. One full factor base per curve is
shared across m=2,3,4,5,6,7,8,9. Actual point counts are determined by lifting,
not assumed equal to candidate counts. See [DESIGN.md](DESIGN.md).

## Format and reader

Each family has 128 Zstandard-compressed shards. Decompressed header: 8 bytes
`N83FB01\n`; one-byte family (0 Koblitz, 1 binary, 2 prime); one-byte ell;
six zero bytes; little-endian u64 start and exclusive end x. Header length32.
Every following record is little-endian u32 x and u128 y (20 bytes), in ascending
x then y order. Both distinct signs are stored. Field elements use polynomial
coefficient bits for binary fields and canonical integers for prime fields.

```sh
cargo build --locked --release --manifest-path research/n83_fixed_factor_base_20261028/Cargo.toml
# Print 20 points from a hydrated shard, or omit 20 to export the whole shard.
research/n83_fixed_factor_base_20261028/target/release/n83-factor-base csv DATA/koblitz/part-0000.bin.zst 20
# Recheck all stored points and every absent candidate x in that shard.
research/n83_fixed_factor_base_20261028/target/release/n83-factor-base verify DATA/koblitz/part-0000.bin.zst
```

These are full-curve affine factor bases. No subgroup filtering is applied.
The previous first-fall results used varying ell; they are not measurements
for this new fixed base. Data generation is separate from any new degree sweep.

## Completed data

All384 shards are complete and validated. Each curve has128 shards covering
exactly the same fixed x-domain; m does not select or resize the dataset.

| Curve | Liftable x-values | Points, both signs | Compressed bytes |
|---|---:|---:|---:|
| Koblitz | 67,110,393 | 134,220,785 | 1,329,389,614 |
| Binary | 67,113,567 | 134,227,133 | 1,329,453,079 |
| Prime | 67,106,828 | 134,213,656 | 1,849,491,803 |

Start with [data/index.json](data/index.json). Its three manifest hashes identify
the exact fixed bases for every m=2..9. The per-family manifests list compressed
and uncompressed hashes, x-ranges, point counts and validation certificates.
The actual points are `data/{koblitz,binary,prime}/part-NNNN.bin.zst`.
Small readable point previews are in [samples/](samples/).

Large point files live in the shared content-addressed artifact store. Hydrate
them in this checkout with:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_fixed_factor_base_20261028/data
```

This Python command only transfers files; generation, arithmetic and validation
are native Rust. To export a whole family as decimal CSV after building the
reader, run `bash research/n83_fixed_factor_base_20261028/export-csv.sh BINARY
koblitz OUTPUT.csv`. The uncompressed CSV is much larger than the binary shards.

R1 was cancelled without scheduling (Spot quota). R2 failed admission (odd CPU
count). R3 started but failed shared-bucket authorization before enumeration.
The completed run is native R4 on the existing m8i.8xlarge Spot workspace,
six worker threads,8Gi virtual-memory limit,3300s producer timeout. No IAM
permissions or account quotas were changed. All frozen attempts are retained.
