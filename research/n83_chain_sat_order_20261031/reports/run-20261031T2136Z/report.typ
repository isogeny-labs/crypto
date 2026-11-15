#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Chained-S3 SAT on N83, m = 2…9: summand ordering and root-finder tables

Run `n83_chain_sat_order_20261031`, 2026-10-31 21:36:33Z – 22:56:35Z. Protocol `research/n83_chain_sat_order_20261031/PROTOCOL.md`, frozen at commit `8951a4a1cf` before the measured run. Driver `examples/n83_chain_sat_bench.rs`; native analysis `examples/n83_chain_sat_analyze.rs` → `ANALYSIS.json`, `table.tsv`; graphs drawn by `plot.py` from `ANALYSIS.json` only. Host: one 4-vCPU x86-64 cloud VM, L0 (no isolation), four single-threaded shards. Counted unit: solver conflicts; wall seconds are an L0 diagnostic.

*Curve.* `icv1-f2m83-t6151469093347-cdcc5432` (`KoblitzCurve::known_n83_k1`), $r$ = 8569786107849059 (53 bits). Base: Frobenius union of span(1, z), 167 points. Per m: 3 public targets $[a]G$, 2 planted sums of m base points (seed 20261031); per query 300,000 conflicts and 120 s.

== What changed in the solver

- *All-summand ordering* (`order_all_summands`): lex constraints $x_1 <= dots.c <= x_m$ remove the $m!$ relabelling symmetry; every identity pattern is still searched, so a refutation still means "no decomposition".
- *Root-finder tables* (`constrain_intermediate`, `ChainS3Tables`): each intermediate $y_k$ of the chain is confined to the exact set $x("signed" (k+1)"-sums")$ from the front and $x(R - "signed" (m-k-1)"-sums")$ from the back, intersected — a meet-in-the-middle inside the SAT instance. Sound because the base is closed under negation and every link may take either sign. Depth 2, ≤ 300,000 points per table.
- *Affine-only pattern* (opt-in, A3): searches one identity pattern; complete only for decompositions without a zero-sum sub-multiset. Reported separately.

== Results (measured)

#table(columns: 9, stroke: 0.4pt,
  [*m*], [*A0 unordered*], [], [*A1 ordered*], [], [*A2 ordered + tables*], [], [*A3 A2 affine*], [],
  [], [pub /3], [plant /2], [pub /3], [plant /2], [pub /3], [plant /2], [pub /3], [plant /2],
  [2], [3], [2], [3], [2], [3], [2], [3], [2],
  [3], [0], [0], [0], [0], [*3*], [*2*], [3], [2],
  [4], [0], [0], [0], [0], [*3*], [0], [3], [0],
  [5–9], [0], [0], [0], [0], [0], [0], [0], [0],
)

+ *Soundness.* 160 records, 0 soundness flags: no planted target refuted, every witness re-adds to its target in the group, no invalid model.
+ *m = 2, ordering.* All arms decide all 5 targets. Paired conflict ratio A1/A0 on the 5 targets: 0.544, 0.597, 0.501 (public), 0.065, 0.197 (planted) — fewer conflicts on every paired target. Median public: 13,461 → 7,195 conflicts, 12.8 → 6.2 s. Tables add nothing at m = 2 (no intermediate).
+ *m = 3, tables.* A2 refutes all 3 public targets with *0 conflicts* (0.38 s median, table build included) and finds both planted witnesses (median 8,735 conflicts, 27.5 s); A0 and A1 decide none within 120 s. Decided-by-arm-only: 5 of 5.
+ *m = 4, tables.* A2 refutes all 3 public targets (median 156 conflicts, 0.98 s); A3 in 0 conflicts. Planted witnesses are not found within 120 s by any arm (calibration probe outside the frozen run: 44,327 conflicts, 336 s).
+ *m ≥ 5.* Every arm is wall-capped on the first identity pattern (A1 public: ≈ 13k conflicts at m = 5 falling to ≈ 4k at m = 9 in 120 s; 85k–170k variables). Depth-2 tables reach only the outer intermediates: at m = 5 each of $y_1, y_2, y_3$ gets one table, and for m ≥ 6 $y_2 dots y_(m-4)$ get none, and depth-3 tables (≈ $6 dot 10^6$ points) exceed the cap.

#figure(image("n83_chain_sat_m2_9.svg", width: 100%), caption: [Decided public queries, verified planted witnesses and median public wall seconds per arm and m. Run n83_chain_sat_order_20261031.])

#figure(image("n83_chain_sat_conflicts.svg", width: 75%), caption: [Median solver conflicts on decided queries (0 drawn at 0.5).])

== Reading against the frozen decision rule

- Improvement claim holds for A1 over A0 at m = 2 (fewer conflicts on every paired target, same verdicts) and for A2 over A0 at m = 3 and m = 4 (more decided public queries, no planted target lost). No pooling across m; m = 5…9 shows no difference within the caps.
- This is a decomposition-stage measurement, not a complete DLP: no index-calculus-versus-rho row, no leaderboard or `benchmarks/` row (no replay certificate or matched-rho `VALIDATION.json`). For scale, the matched rho of the previous study solved this curve in a mean 1.51 s online (`research/n83_msweep_20261024`); a refuted public query costs 0.38 s (m = 3) and only about `|B|^m / (m! · #E)` of public queries decompose, so the distance to rho remains astronomically large (derived, not measured).

== Limits and deviations

- L0 host, 5 targets per m, one base size; wall caps shape m ≥ 4. EKS (AGENTS.md §9a) was not used: the workload is not an ecbench method and the session has no aws CLI; conflicts are the primary unit for that reason.
- Next lever (not measured): constrain the middle intermediates by lookup ($b - p in F_2$ for $b in B_2$) instead of materialising 3-sum tables, and reuse learnt clauses across patterns and queries (solver assumptions).
