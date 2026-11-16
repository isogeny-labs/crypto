#set page(paper: "a4", margin: 16mm)
#set text(font: "Noto Sans", size: 9pt)
#set heading(numbering: "1.")
= Combined tagged-index and dense-row exact-oracle IC · p=37

*Run:* `run-20261022T0758Z-quartic-2d-dense-tagged-p37` · 22 October 2026 · shared Intel Xeon 6975P-C · L0 · one public target `[271]G`. The registered protocol, raw JSON, independent point-keyed audit, A/A diff and commands are linked from `README.md`. The run completed 1/1 residual before its 64-query, 300-second and 8-GiB caps.

The 624-point signed full two-dimensional base folded to 52 columns. The tagged direct-posting table used 32,396 pair-orbit representatives and 388,752 postings. On the first residual it returned exactly the same complete 53,551 signed four-distinct-factor decompositions as a separately counted full-table reference. The first 115 candidates yielded 52 independent anchored rows, rank 52/52 and recovered 271. Every factor log was verified. The independent oracle enumerated all 1,873,633 subgroup logs and replayed the complete hit count, selected rows, rank and answer.

#table(
  columns: (1.45fr, 1fr, 1fr, 1fr, 1fr),
  stroke: .4pt,
  inset: 4pt,
  [Phase], [Full Fp M], [Tagged Fp M], [Full group ops], [Tagged group ops],
  [Common model/base/input/map], [4,466,941], [4,466,941], [17,726], [17,726],
  [Pair table], [37,708,944], [7,796,605], [388,752], [32,396],
  [One residual], [75,560,753], [75,560,753], [779,024], [779,024],
  [Answer checks], [2,004,307], [2,004,307], [18,854], [18,854],
  [*Total*], [*119,740,945*], [*89,828,606*], [*1,204,356*], [*848,000*],
)

#image("dense_tagged_cost.png", width: 100%)

The tagged producer's counted one-target totals are 1.333-fold and 1.420-fold below the earlier full-table dense arm in separate field and group units. It retained 47,721,464 requested bytes and peaked 78,834,528 requested bytes above its pre-table allocation. It recorded 777,503 hash lookups and 324,839 candidate postings. Anchored matrix selection and solve used 20,969 and 11,484 modular operations; a further 17,251,447 operations were an excluded unanchored-rank diagnostic but remained in its 632.080 ms shared-host L0 wall observation.

This prices an exact group-oracle path, not polynomial S5 relation finding, external Gröbner comparison, isolated wall speed or calibrated rho. The tagged table changes setup cost and storage, not the complete relation set. Full reproducibility pointers and the status boundary are in `README.md`.
