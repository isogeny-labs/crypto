#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Chained-S3 on N83, m = 8, 9: indexed lookup and rebuild by peeling

Run `n83_chain_sat_indexed_20261101`, 2026-11-01 17:53:15Z – 18:33:36Z. Protocol `PROTOCOL.md` frozen at `7304b82d81` before the run (user: "add m=8 and m=9 after this run"). Driver `examples/n83_chain_sat_bench.rs`; native analysis `examples/n83_chain_sat_analyze.rs` → `ANALYSIS.json`, `table.tsv`; graph by `plot.py` from `ANALYSIS.json` only. Host: one 4-vCPU x86-64 cloud VM, L0, one core per m. Follow-up to #3191 (lookup + rebuild decided m = 5…8; m = 8 at 72–92 s per query, m = 9 out of reach).

*Curve.* `icv1-f2m83-t6151469093347-cdcc5432`; base: Frobenius union of span(1, z), 167 points, closed under negation. Seed 20261031: 3 public $[a]G$ and 5 planted sums of m base points per m. Per query 300,000 conflicts, 120 s.

== What changed (`ChainS3Tables::indexed`)

- *Indexed membership.* Each depth's sorted fingerprint array gets a bucket index on its top ⌊log₂ n⌋ bits (`ChainS3FrontSet`): a lookup reads one index entry and scans about one fingerprint, where the binary search took about log₂ n dependent cache misses (28 at depth 5).
- *Rebuild by peeling.* A hit $T = R + s$ is a signed $d$-sum; some summand $p$ leaves $T - p$ in the cached depth-$(d-1)$ set, and so on down to one base point, so $T$ splits into its $d$ summands with at most $d dot 167$ point additions, and $R = T - s$ needs no sign recovery. The merged rebuild re-enumerated every signed $d$-sum instead (≈ 3.4·10^7 at d = 4, ≈ 1.15·10^9 at d = 5). A peel that dead-ends falls back to that enumeration.
- Depth sets 2…d are built before the first query (warmup: depth 2 0.01 s, 3 0.55 s, 4 26 s, 5 999 s with 372,406,144 fingerprints).

== Results (measured)

#table(columns: 5, stroke: 0.4pt,
  [*m*], [*d*], [*A5 control (merged): public / planted*], [*A6 indexed: public / planted*], [*median s, A5 → A6 (public; planted)*],
  [8], [4], [3/3 refuted, 5/5 verified], [3/3 refuted, 5/5 verified], [78.4 → 39.7; 83.1 → 40.9],
  [9], [5], [0/3, 0/5 (120 s cap)], [*3/3 refuted, 5/5 verified*], [cap → 54.1; cap → 54.7],
)

+ *Soundness.* 32 records, 0 soundness flags; every witness re-adds to its target; no planted target refuted; at m = 8 both arms give the same verdict on every target.
+ *m = 9 decided.* With indexing, all 3 public targets are refuted and all 5 planted witnesses rebuilt, with no solver call, at ≈ 54 s per query; the control decides none. Improvement under the frozen rule (more decided queries).
+ *m = 8 about twice as fast.* Same verdicts; median query time 78.4 → 39.7 s (public) and 83.1 → 40.9 s (planted). Improvement under the frozen rule (lower median on paired targets where both decide every query).
+ With #3191, the chained-S3 path now decides every public and planted test target on this fixture for m = 2…9.

#figure(image("n83_chain_sat_indexed_m8_9.svg", width: 100%), caption: [Decided queries (of 8) and median seconds per query, per arm.])

== What this is and is not

- For m ≥ 5 the path is a meet-in-the-middle decomposition over the factor base (lookup decides, peel rebuilds); the SAT solver is not called on these targets. Per query ≈ $|B|^(m-d)$ point additions and lookups after a one-off $|B|^d$ build; exponential, the expected trade-off. The remaining per-query time at m = 8, 9 is the $C(170, 4) ≈ 3.4·10^7$-step back enumeration.
- Decomposition stage only: no relation collection, no DLP, no leaderboard or `benchmarks/` row (validation gate). Matched rho on this curve: 1.51 s mean online (`research/n83_msweep_20261024`).

== Limits

L0 host, 8 targets per m, one base, one curve; the depth-5 build takes 999 s once per process and, by its reserved capacity (1.15·10^9 u64 plus the shrunk copy), an estimated ≈ 12 GB at its peak (not measured). EKS (AGENTS.md §9a) not used: not an ecbench method, no aws CLI.
