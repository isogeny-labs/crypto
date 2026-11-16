#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Chained-S3 on N83, m = 5…8: planted witnesses rebuilt from the lookup hit

Run `n83_chain_sat_rebuild_20261101`, 2026-11-01 06:42:02Z – 07:06:28Z. Protocol `PROTOCOL.md` frozen at `e4662c05f1` before the run. Driver `examples/n83_chain_sat_bench.rs`; native analysis `examples/n83_chain_sat_analyze.rs` → `ANALYSIS.json`, `table.tsv`; graph by `plot.py` from `ANALYSIS.json` only. Host: one 4-vCPU x86-64 cloud VM, L0, one core per m. Follow-up to `research/n83_chain_sat_meet_20261101` (#3149): the lookup refuted every public target for m = 5…8, but no arm found a planted witness, though every planted target had a non-empty lookup domain.

*Curve.* `icv1-f2m83-t6151469093347-cdcc5432` (`KoblitzCurve::known_n83_k1`). Base: Frobenius union of span(1, z), 167 points, closed under negation. Seed 20261031: 3 public $[a]G$ and 5 planted sums of m base points per m (public and first two planted as in the earlier runs). Per query 300,000 conflicts, 120 s.

== The rebuild (`ChainS3Tables::rebuild`)

The per-query back enumeration now records the signed $(m-d)$-multiset $s$ behind each lookup hit $T = R + s$. The signed $d$-sums are re-enumerated until one, $f$, has $x(f) = x(T)$; then $f = plus.minus T$, so $R = (plus.minus f) - s$: $m$ base points, mapped to base indices and re-added in the group before being returned. No SAT instance is built. If no hit lifts (a fingerprint collision, or a summand outside the base), the query falls through to the solver unchanged.

== Results (measured)

#table(columns: 6, stroke: 0.4pt,
  [*m*], [*d*], [*A4 control: planted verified*], [*A5 rebuild: planted verified*], [*A5 planted median s (range)*], [*public refuted (A4 / A5)*],
  [5], [3], [0/5], [*5/5*, 0 solver calls], [0.91 (0.79–1.00)], [3/3 / 3/3],
  [6], [3], [0/5], [*5/5*, 0 solver calls], [1.65 (1.63–1.83)], [3/3 / 3/3],
  [7], [4], [0/5], [*5/5*, 0 solver calls], [10.8 (2.9–17.0)], [3/3 / 3/3],
  [8], [4], [0/5], [*5/5*, 0 solver calls], [81.5 (71.5–92.1)], [3/3 / 3/3],
)

+ *Soundness.* 64 records, 0 soundness flags; every witness re-adds to its target.
+ *Planted.* A5 rebuilds 20/20 planted witnesses with no solver call; the control (lookup + SAT, merged in #3149) finds 0/20 within 120 s. Per the frozen rule this is an improvement at every m = 5…8, with every public verdict unchanged.
+ *Cost.* A rebuild adds one partial re-enumeration of the signed $d$-sums (it stops at the first lifting match): ≈ 0.1–0.4 s at d = 3 and up to ≈ 20 s at d = 4 (planted minus public medians and ranges), on top of the per-query lookup (≈ 0.7 s at m = 5, ≈ 72 s at m = 8, dominated by the binary search in the depth-4 set).

#figure(image("n83_chain_sat_rebuild_m5_8.svg", width: 100%), caption: [Verified planted witnesses per arm (of 5) and median seconds per query. Public verdicts are identical in both arms.])

== What this is and is not

- With rebuild, the chained-S3 "SAT decomposer" at m ≥ 5 is a meet-in-the-middle decomposition over the factor base: a lookup decides existence, the rebuild produces the witness, and the SAT solver is never called on these targets. Cost per query ~ $|B|^(m-d)$ after a $|B|^d$ build (plus at most one $|B|^d$ re-enumeration per witness): exponential, the expected trade-off.
- Decomposition stage only: no relation collection, no DLP, no leaderboard or `benchmarks/` row (validation gate). Matched rho on this curve: 1.51 s mean online (`research/n83_msweep_20261024`).

== Limits

L0 host, 8 targets per m, one base, one curve; m = 9 excluded (lookup alone exceeds the cap). EKS (AGENTS.md §9a) not used: not an ecbench method, no aws CLI. Next lever (not measured): a direct-indexed fingerprint table in place of the binary search, which dominates m = 8 and blocks m = 9.
