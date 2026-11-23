#set page(paper:"a4", margin:18mm)
#set text(size:10pt)
= n83 spike40 factor-base dataset
Observations only. This report records complete point enumeration and verification. It establishes no degree reduction, elite-base status or ECDLP result.

One fixed base per original curve is retained for all m=2..9. The domain contains exactly 134,217,728 candidates, encoded by selector bits 0..26 mapped to coordinate positions 0..25,40. Both distinct affine lifts are saved. Infinity is omitted and no subgroup filter is applied.

#image("counts.svg", width:100%)
Exact counts from n83-candidate-spike40-e27-20261028-r1. Each family has 128 shards of 1,048,576 candidates. No sampling uncertainty applies to these totals.

#table(columns:(1fr,1fr,1fr),inset:5pt,
 [Family],[Liftable x],[Compressed bytes],
 [Koblitz],[67,113,938],[1,394,855,010],
 [General binary],[67,103,971],[1,394,620,808],
 [Prime],[67,105,985],[1,849,489,651])

== What ran and what was checked
All 384 requested shards completed; no producer failure or timeout. The native generator reread each written shard and checked every candidate for complete valid root pairs or no lift. Distributed samples use slow reference arithmetic. Five native generator tests passed; the earlier failed inherited test log remains preserved. Nine additional same-instrument replays cover the first, middle and last shard of each family. Raw manifest checks and the unified export check pass. This is producer verification, not independent review.

Run receipt: 28 October 2026, 05:54:15–05:55:51 UTC, existing Spot workspace, six native workers, 8 GiB virtual-memory cap, 5400-second producer cap. CPU time and peak RSS were not recorded; elapsed timestamps are diagnostic, not performance evidence. Periodic uploads logged transient rename warnings; the final sync succeeded and all raw files remain retained.

== Exact curves
Binary field: GF(2^83), modulus z^83+z^45+z^2+z+1. Equation y²+xy=x³+ax²+b. Koblitz a=0,b=1; general binary a=1,b=2^82+0x51b. Prime p=4835703278761981908090881; y²=x³+2x+3.

#pagebreak()
== File structure and access
Start with ../../data/index.json and ../../FORMAT.json. Each family manifest binds 128 shard receipts and hashes. Point shards are zstd compressed N83FB02: 32-byte header, then records of 20 bytes. Bytes 0..3 of each record contain a little-endian selector index; bytes 4..19 contain y as a little-endian u128. The selector is not x. Decode with:
#raw("x=(index & (2^26-1)) | ((index>>26)<<40)",block:true)

Header: magic N83FB02 plus newline at bytes0..7; family tag at8, selector dimension at9, moved coordinate bit40 at10, reserved zeros at11..15, start/end selector indices as u64 LE at16..23 and24..31. JSON receipts record interval, counts, raw/compressed hashes, coverage hash and samples. The README gives native CSV and verification commands.

== Durable evidence
Raw S3 prefix:
#raw("s3://setec-astronomy-artifacts/runs/n83-candidate-spike40-e27-20261028-r1/local/",block:true)
The metadata archive contains the executable, controls, environment, logs, terminal receipt and full manifest. Per-family artifact manifests reference the point payloads by SHA-256. Reconstruct data/ alongside metadata and check the original manifest. All source, failed validation logs and uploader diagnostics are retained.

#text(size:8pt)[
Instrument source: 7046a36c42a05e003e107100b3b9759068fdba39\
Frozen protocol: add3481a2d31e9a2d9e82a47b657cd4abb27f67353990568b92ce43957e87f69\
Binary: bd9cbb2c6edb948d62c8cf7d62b069263951e335cfa09e80b7f9330aa9225c7a\
Frozen source archive: 71188aa2faa4db605022e77cdcf1e95a6c579d33f59402e37cf676a8a5e7e1ab
]

Unified export: ../../experiment-base.export.json. Full storage/reconstruction metadata: ../../evidence/storage.json. No A/A timing interval applies; this is deterministic enumeration with positive and altered/deleted-point negative controls.

== Remaining experiment
PDP_PLAN.md specifies paired natural targets and identical planted targets from the common low26 domain. The changed selector weight must be tested with unchanged preprocessing. FFD, LFD, highest-form quotient regularity and intermediate solving degree remain distinct quantities. Enumeration alone measures none of them.

Prior method: #link("https://arxiv.org/html/1906.05594")[Kousidis and Wiemers, On the first fall degree of summation polynomials]. Its subspace setting informs the candidate construction; its FFD results do not establish a degree reduction for this dataset. Existing polynomial/random/affine subspace studies are recorded in DESIGN.md. No novelty claim.
