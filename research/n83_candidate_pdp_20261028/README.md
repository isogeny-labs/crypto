# n83 spike40 factor bases and matched PDP work

The complete **spike40** point dataset is at `data/index.json`. Every curve has
134,217,728 candidate x-values and 128 shards. The coordinate map is
`x=(index & (2^26-1)) | ((index>>26)<<40)`. The same base serves m=2..9.

| Family | Liftable x | Affine points | Compressed point bytes |
|---|---:|---:|---:|
| Koblitz | 67,113,938 | 134,227,875 | 1,394,855,010 |
| General binary | 67,103,971 | 134,207,941 | 1,394,620,808 |
| Prime | 67,105,985 | 134,211,970 | 1,849,489,651 |

All distinct affine lifts are stored; no subgroup filter or infinity record.
These counts do not establish lower FFD, LFD, regularity or solving cost.

## Access and encoding

`FORMAT.json` specifies every byte. N83FB02 has a 32-byte header and 20-byte
records: selector index u32 LE, y u128 LE. **The first field is not x.** Apply the
coordinate map, or use the CSV decoder below. Binary integers encode polynomial
coefficients; prime coordinates are canonical integers. Shards are sorted by
selector and then y. Each JSON receipt describes its selector interval, exact
counts, compressed/raw hashes, coverage hash and sample actual coordinates.

```sh
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/data/koblitz research/n83_candidate_pdp_20261028/data/binary research/n83_candidate_pdp_20261028/data/prime
cargo build --locked --release --manifest-path research/n83_candidate_pdp_20261028/Cargo.toml --bin factor_base
research/n83_candidate_pdp_20261028/target/release/factor_base csv research/n83_candidate_pdp_20261028/data/koblitz/part-0064.bin.zst 8
research/n83_candidate_pdp_20261028/target/release/factor_base verify research/n83_candidate_pdp_20261028/data/koblitz/part-0064.bin.zst
```

`experiment-base.export.json` binds all 384 shard receipts to the frozen
`protocol-base.json`. `evidence/storage.json` supplies the raw S3 prefix, immutable
source archive and reconstruction instructions. `base-run-metadata.tar.xz`
contains the frozen executable, controls, environment, logs and full manifest;
point payloads are stored separately with per-family `.artifacts.json` pointers.
The producer verifies every candidate, including omitted x-values. Nine additional
same-instrument shard replays are in `validation/base-readback/`; this is not an
independent research review.

## Next measurement

`PDP_PLAN.md` defines the paired 96-system panel. `src/bin/pdp.rs` generates
matched targets and asserts unchanged natural baseline source hashes;
`src/bin/fall.rs` verifies explicit falls or prime degree-four minors with the
correct selector weights. Six generator tests and eight fall tests pass.
No candidate degree measurement is claimed by the enumeration report.

The shared progress-metrics exporter was reserved by Conductor task T-705 when
publication was prepared. Its integration remains a separate pending task;
actual committed counts are available directly from `data/index.json`.
