#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "New Computer Modern", size: 9pt)
#align(center)[= N83 retained-base exact rank-query census]
#align(center)[Run N83-RQC-20261115-v1, 2026-11-15 UTC]

*Analytic necessary-bound screen.* Inputs: October 77-object census, SHA-256 `1117ead0dfcc9164a73d83d5ca709ec538528f7042e070ca8d72a5e188487ade`; full direct-subgroup D18 manifest, SHA-256 `1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`. Protocol and exact 390 rows are in `native_rank_query_census/PROTOCOL.md` and `results.json`.

Primary curve: `icv1-f2m83-tm6151469093347-debefd74`; subgroup order `2417851639230796216685689`. The a1 subgroup is a separate diagnostic. For B declared points and K orbit columns, M = binomial(B+m-1,m). Under uniform whole-subgroup queries and at most one independent full-smooth row per query, p <= min(M,r)/r. The necessary count before the rank-K first-moment ceiling can reach one half is max(K,ceil(Kr/(2 min(M,r)))). Correlation among queries is allowed. Declared B and K are manifest claims.

#figure(image("rank-query-plot.svg", width: 100%), caption: [Derived necessary query counts for four primary size classes. Logarithmic query axis; no runtime measurement.])

#table(columns: (auto,auto,auto,auto,auto,auto), inset: 3pt,
  [Base], [K], [B], [m4], [m5], [m6],
  [Orbit], [64], [10,624], [145,677,800,576], [68,534,909], [38,688],
  [Orbit], [256], [42,496], [2,277,179,841], [267,904], [256],
  [Orbit], [600], [99,600], [176,888,106], [8,880], [600],
  [Full D18], [32,643], [5,418,738], [32,643], [32,643], [32,643],
)

Local host: Darwin 27.2.0 arm64, macOS 27.2; nested native crate: Rust 1.93.1 release. The native release tests passed 3/3, including a changed-byte source rejection, and all 30 frozen historical cases reproduced. This screen covers 78 objects, 390 rows, and no timed phases. Solver cost, large-prime partials, rank attainment, construction, descent, verification, and matched rho remain unmeasured here. The full-runtime factor-base winner remains unset. The K600 m6 hosted run remains subject to its frozen host, input, and budget gates. No canonical performance graph changed because there is no new timing.
