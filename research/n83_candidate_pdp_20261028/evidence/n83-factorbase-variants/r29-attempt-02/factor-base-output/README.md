# spike29 fixed factor-base output

`data/index.json` binds the completed candidate base and three family manifests. Each family manifest points to 128 contiguous shards covering the fixed 2^27 selector domain. Every `part-NNNN.bin.zst` has a matching JSON receipt with selector span, point/lift counts, compressed and raw hashes, coverage hash, and sample coordinates. `index-summary.json` gives a compact readable inventory; the exact byte layout is in `FORMAT.json` at the study root and in the generator source.

The same generated factor base is reused for all m=2..9; no per-m base regeneration occurs. Large payload files are tracked through adjacent `.artifacts.json` manifests and the canonical artifact store after publication.
