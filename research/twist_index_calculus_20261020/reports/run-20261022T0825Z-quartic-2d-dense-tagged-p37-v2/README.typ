#set page(paper: "a4", margin: 16mm)
#set text(font: "Noto Sans", size: 9pt)
= Combined tagged/dense p=37 exact-oracle IC · final-source rerun

*Run:* `run-20261022T0825Z-quartic-2d-dense-tagged-p37-v2` · 22 October 2026 · shared Intel Xeon 6975P-C · L0 · one public target `[271]G`. The registered 624-point, 52-column first-residual workload completed before its 64-query, 300-second and 8-GiB caps. This rerun follows a lint-only annotation and two redundant-borrow fixes; its complete non-timing record is identical to the earlier committed source revision. Exact commands, hashes and links are in `README.md`.

The full and tagged pair tables returned identical complete sets of 53,551 signed decompositions. The first 115 candidate rows supplied anchored rank 52/52 and answer 271; all factor logs were checked. The independent point-keyed oracle enumerated all 1,873,633 subgroup logs, replayed 52 selected rows and recovered 271. Both the A/A and old/new-source non-timing diffs are empty.

#table(columns: (1.8fr,1fr,1fr), inset: 4pt, stroke: .4pt,
  [One-target counted path], [Full-table dense], [Tagged dense],
  [Pair setup Fp M / group ops], [37,708,944 / 388,752], [7,796,605 / 32,396],
  [Total Fp M], [119,740,945], [89,828,606],
  [Total group ops], [1,204,356], [848,000],
)

#image("dense_tagged_cost.png", width: 100%)

The exact total deltas equal pair-table setup deltas: 29,912,339 Fp multiplications and 356,356 group operations. The tagged table retained 47,721,464 requested bytes; peak above pre-table allocation was 78,834,528 requested bytes. There were 777,503 hash lookups and 324,839 candidate postings. The shared-host L0 observation was 639.113 ms and includes an excluded unanchored-rank diagnostic. Polynomial S5, hash CPU, isolated timing and a matched rho GAE band remain unpriced.
