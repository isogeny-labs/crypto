# N83 summand sweep, m = 2…9: factor bases × decomposition engines (2026-10-24)

Stage diagnostic on the pinned N83 fixture (`icv1-f2m83-t6151469093347-cdcc5432`,
`KoblitzCurve::known_n83_k1`, 53-bit subgroup), with the repository's m = 4
rootfinder and the matched strong rho as complete-DLP references, and an
exact-field companion on `icv1-f2m61-tm158598901-405ca675`. Preregistered in
[`PROTOCOL.md`](PROTOCOL.md) (frozen at `b654e95b6`); frozen grid
[`grid.tsv`](grid.tsv). Report with graphs and PDF:
[`reports/run-20261024T0700Z/report.pdf`](reports/run-20261024T0700Z/report.pdf).

## Answer

- **The F4/F5/F6 family has no valid n = 83 measurement, and until this change it
  silently gave wrong answers there.** Every Weil-descent engine (matrix F4, matrix
  F5, inherited F4, F6-IC, F6-IC pair index, crossbred, SAT descent in three
  encodings, MQ-FES) returned "proved UNSAT" on planted n = 83 relations, because
  `FieldStructure` truncates the field table to one 64-bit word. The descent
  builders now refuse such a field (`FieldStructure::is_exact`, regression test
  `n83_planted_relation_is_never_refuted_by_a_truncated_field`).
- **Chained-S3 full-width SAT is the only decomposer that runs at n = 83 for every
  m.** Within the 180 s cap it decides only m = 2: 17.3 s per public query (4/4
  proved UNSAT), and planted witnesses recovered 2/2 with the 167- and 333-point
  union bases. All m = 3…9 cells, and every orbit-domain cell, hit the cap.
  The derived projection for the decided cell is 2^75.9 s, which is 2^75.3 × rho.
- **m = 4 rootfinder (complete DLP, K = 1107):** 188.5 s (root11) and 175.2 s
  (root22), against a strong-rho mean of 1.509 s (3 seeds). That is 116–125×
  from rho, the distance remaining. All five solves recover the same group-verified
  scalar.
- **Companion n = 61, m = 2:** native-XOR SAT is the fastest decider at
  D ≥ 12 (0.79–2.41 ms per query). Inherited F4 is the fastest Gröbner engine
  (6.2–33.4 ms); matrix F5 runs at 0.78–1.05× of matrix F4. F6-IC and crossbred
  scale worst with D. m ≥ 3 is refused by the 64-variable descent mask.

No improvement over the IC baseline is measured, so no leaderboard or
`benchmarks/README.md` row is added.

## Files

| path | what |
|---|---|
| `raw/shard{1..4}/` | 500 cell records (`curve__mM__fb__engine__probes.json`), incl. `process_cap` records |
| `raw/reference/` | rho receipts, rootfinder stdout/result/base dump, wall times |
| `raw_superseded/encoder_cap_m7-9/` | the first m = 7–9 chained records, refused by the m ≤ 6 encoder bound |
| `cards/` | director task cards, executor reports, director assessments (relay stopped; see report) |
| `resume/` | core-session resume of the same frozen commands: HEAD, hashes, stdout |
| `ANALYSIS.json`, `table.tsv` | `cargo run --release --example n83_msweep_analyze -- research/n83_msweep_20261024` |
| `plot.py` | lays out `ANALYSIS.json` into the SVG/PNG graphs |
