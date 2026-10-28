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
