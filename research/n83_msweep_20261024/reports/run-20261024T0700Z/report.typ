#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= N83 summand sweep, m = 2…9: factor bases × decomposition engines

Run `n83_msweep_20261024`, 2026-10-24 (UTC start ≈ 07:00). Protocol: `research/n83_msweep_20261024/PROTOCOL.md`, frozen at commit `b654e95b6` before any measured run. Cell driver `examples/n83_msweep_cell.rs`; analysis `examples/n83_msweep_analyze.rs` → `ANALYSIS.json`, `table.tsv` (native); graphs drawn by `plot.py` from `ANALYSIS.json` only. Host: one 4-vCPU x86-64 cloud VM, L0 (no isolation), four single-threaded shards at a time; wall time only.

*Curve.* `icv1-f2m83-t6151469093347-cdcc5432` (`KoblitzCurve::known_n83_k1`): $y^2 + x y = x^3 + x^2 + 1$ over $F_2[z]/(z^83 + z^45 + z^2 + z + 1)$, $r = 8569786107849059$ (53 bits), cofactor 1128547018. The rho reference and the m = 4 rootfinder run the same curve in another polynomial basis (`icv1-f2m83-t6151469093347-9eba6cd5`; same trace and order). Companion rows: `icv1-f2m61-tm158598901-405ca675` ($K_1$ over $F_(2^61)$, exact one-word field).

== What ran

#table(columns: 5, stroke: 0.4pt,
  [*family*], [*engines*], [*factor bases*], [*m*], [*cells*],
  [chained (n=83)], [chained-S3 full-width SAT: explicit domain, signed-Frobenius orbit domain, orbit + ordered pair], [Frobenius unions $S = 2,3,4$], [2–9], [80],
  [algebraic (n=83)], [matrix-F4, matrix-F5, inherited-F4, F6-IC, F6-IC pair index, crossbred, Weil-descent SAT (native XOR / CNF / base-restricted), MQ-FES], [subspace $D=8$], [2–9], [80 (planted)],
  [companion (n=61)], [the same ten], [subspace $D = 8,12,16,20$], [2–9], [340],
  [reference (n=83)], [m = 4 root-table rootfinder, root11 and root22 layouts, $K = 1107$, complete DLP], [compact signed-orbit base], [4], [2],
  [rho (n=83)], [strong signed-Frobenius rho], [—], [—], [3 seeds],
)

500 cells recorded: 422 completed, 78 killed at the 180 s per-cell cap (`process_cap`). Each cell is a fresh process; each query is a public probe $[a]G$ (or a planted sum of $m$ base points for the positive control) and every witness is re-added in the curve group.

== Findings (measured unless marked derived)

+ *Soundness defect found and fixed.* At n = 83 every Weil-descent engine (F4, F5, inherited F4, F6-IC, F6-IC pair, crossbred, the three SAT encodings, MQ-FES) answered *"proved UNSAT"* on planted targets that do decompose: `FieldStructure` keeps $z^(i+j) mod f$ in one 64-bit word, so above n = 64 the descended system is a different system. The builders now refuse such a field (`FieldStructure::is_exact`); a regression test pins it. After the fix all ten engines refuse at n = 83 for every m (MQ-FES reports incomplete), shown grey/orange in Figure 1. There is no exact n = 83 F4/F5/F6 measurement in the repository to report.
+ *Chained-S3 SAT is the only full-width decomposer, and only m = 2 decides within 180 s.* m = 2, union base $S = 2$ (167 points): 4/4 public queries proved UNSAT at 17.3 s each; planted 2/2 recovered (8.0 s), $S = 3$ planted 2/2 (12.7 s). Every m = 3…9 cell and every orbit-domain cell hit the 180 s cap. Derived projection for the one decided public cell: $log_2$ collection seconds $= 75.9$ with heuristic $p = |B|^m/(m! \#E)$, i.e. $2^(75.3)$ times the rho mean — the distance remaining, not a gate.
+ *m = 4 rootfinder baseline.* Complete DLP in 188.5 s (root11) and 175.2 s (root22) process wall; root22: setup 85.95 s, rank stage 88.35 s for 1107 relations (mean 508,030 probes), target 0.14 s, peak RSS 2.50 GB. Both recover scalar 7611230323517904, group-verified.
+ *Matched rho.* 1.236, 2.696, 0.596 s online (mean 1.509 s, 3 seeds), each `PASS_VERIFIED_PUBLIC_TARGET`, recovering the same scalar. Rootfinder / rho ≈ 116–125× (derived). §5a band: every arm here is priced in wall seconds of its complete native work, so the 0 / 0.1 / 1 GAE band does not change any order.
+ *Companion n = 61, m = 2* (16 public probes per cell, all proved UNSAT; planted 4/4 recovered by every engine). Mean ms per query at $D$ = 8 / 12 / 16 / 20: native-XOR SAT 0.40 / 0.79 / 1.94 / 2.41; inherited F4 0.30 / 6.2 / 18.9 / 33.4; matrix F5 0.30 / 7.6 / 20.2 / 45.6; matrix F4 0.29 / 9.6 / 19.3 / 52.5; F6-IC 0.49 / 11.4 / 79.8 / 1770; crossbred 1.7 / 21.9 / 426 / 8780; MQ-FES 0.75 / 16.3 / 3680 / cap. m ≥ 3 is refused by the 64-variable descent mask ($m D + (m-2) n > 64$), so the companion carries no m-trend.

#figure(image("n83_status_map.svg", width: 100%), caption: [N83: verdict class per engine × factor base × m (cell text: mean seconds per query; planted: verified/planted). Run n83_msweep_20261024.])

#figure(image("n83_projected_vs_m.svg", width: 90%), caption: [N83: derived projected collection time of the only decided public cell, the m = 4 rootfinder's complete-DLP process wall, and the rho mean (log₂ seconds).])

#figure(image("companion_f2m61_engines_m2.svg", width: 100%), caption: [Companion $K_1/F_(2^61)$, m = 2: mean seconds per decided query by engine and subspace dimension (16 probes each).])

== Deviations, limits, missing data

- *Relay.* The `experiment-director` (Sonnet) compiled five lint-clean frozen-command cards; `executor-mechanical` (Haiku) started them, but each executor's background job ended with its agent, so no card wrote a receipt (assessments in `cards/`). The same frozen commands were resumed from the core session (`resume/`: HEAD, binary and script hashes, stdout). Partial records from the relay are kept; the incomplete reference run was discarded and rerun.
- *m = 7–9 chained cells* were first refused by an encoder bound defined only through m = 6; the bound was extended to m = 9 (same formula) and the 30 cells rerun. The refusal records are in `raw_superseded/`.
- Four shards ran concurrently on four vCPUs (L0): per-query times are contended wall time. No hardware counters. One exposed public point for the reference arms. Relation collection is not a DLP: the projections are derived from a heuristic $p$ wherever no relation was observed.
- No row is added to the IC leaderboard or `benchmarks/README.md`: nothing improves on the IC baseline and the validation gate's three artifacts were not produced for this stage diagnostic.
