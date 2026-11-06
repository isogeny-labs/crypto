#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Chained-S3 SAT on N83, m = 5…9: meet-in-the-middle lookup for the middle intermediate

Run `n83_chain_sat_meet_20261101`. m = 5, 6, 7: 2026-11-01 01:50:17Z – 02:04:52Z, protocol `PROTOCOL.md` frozen at `0704c3b008`. m = 8, 9: 02:11:51Z – 02:47:51Z, amendment `PROTOCOL_m8_9.md` frozen before its data (user request). Driver `examples/n83_chain_sat_bench.rs`; native analysis `examples/n83_chain_sat_analyze.rs` → `ANALYSIS.json`, `table.tsv`; graphs by `plot.py` from `ANALYSIS.json` only. Host: one 4-vCPU x86-64 cloud VM, L0, one core per m. Follow-up to `research/n83_chain_sat_order_20261031`, where every arm was wall-capped at m ≥ 5.

*Curve.* `icv1-f2m83-t6151469093347-cdcc5432` (`KoblitzCurve::known_n83_k1`), $r$ = 8569786107849059. Base: Frobenius union of span(1, z), 167 points (closed under negation). Same seed and targets as the 10-31 run: 3 public $[a]G$ and 2 planted sums of m base points per m; per query 300,000 conflicts and 120 s.

== The lookup (`ChainS3Tables::meet = d`)

The middle intermediate $y_(d-1)$ is the x-coordinate of a signed sum of $d$ base points, and of $R$ minus a signed sum of the other $m - d$. The x-fingerprints of every signed $d$-sum are built once per base (batched inversion, query-independent, reported as warmup); per query every signed $(m-d)$-sum $s$ gives $R - s$, kept when its x is in that set. The survivors are the exact domain of $y_(d-1)$ (a fingerprint collision can only add a value). Patterns with the identity at $y_(d-1)$ need $R$ itself to be a signed $(m-d)$-sum and are refuted without a solve otherwise; all other patterns are refuted without a solve when the domain is empty. Unit tests: batched additions equal `point_add`; the depth-2 set equals the exact sum table; a planted witness is kept; a target with a zero-sum decomposition is never refuted.

== Results (measured)

#table(columns: 7, stroke: 0.4pt,
  [*m*], [*d*], [*warmup (once)*], [*A2 control: public / planted*], [*A4 lookup: public refuted*], [*A4 median public s*], [*A4 planted domain sizes*],
  [5], [3], [0.68 s, 347,688 fp], [0/3, 0/2], [*3/3* at 0 conflicts], [0.80], [10, 16],
  [6], [3], [0.68 s], [0/3, 0/2], [*3/3* at 0 conflicts], [1.39], [20, 62],
  [7], [4], [26.1 s, 12,976,303 fp], [0/3, 0/2], [*3/3* at 0 conflicts], [2.54], [35, 35],
  [8], [4], [27.2 s], [0/3, 0/2], [*3/3* at 0 conflicts], [76.7], [70, 250],
  [9], [5], [958.8 s, 372,406,144 fp], [0/3, 0/2], [0/3 (120 s cap)], [—], [—],
)

+ *Soundness.* 50 records, 0 soundness flags. No planted target refuted by either arm.
+ *Public targets, m = 5…8.* The lookup arm refutes all 12 with an empty domain and 0 solver conflicts; the control (ordering + depth-2 tables, the best arm of the 10-31 run) decides none in 120 s. Per the frozen rule this is an improvement at m = 5, 6, 7 and 8 (more decided public queries, no planted target lost).
+ *Planted targets.* Neither arm finds a witness within 120 s at any m ≥ 5. The lookup domain is non-empty on every planted target (10–250 values) and empty on every public one: the lookup separates them, but the SAT search over the rest of the chain does not finish within the cap.
+ *m = 9.* The depth-5 set (372M fingerprints, 959 s) is built, but each query's back enumeration (≈ 3.4·10^7 additions, each with a binary search in a 3 GB sorted array) exceeds 120 s: no improvement at m = 9.

#figure(image("n83_chain_sat_meet_m5_9.svg", width: 100%), caption: [Public queries refuted, planted witnesses verified, and the lookup domain size of the middle intermediate per target. m = 9 has no domain: its enumeration hit the cap.])

#figure(image("n83_chain_sat_meet_cost.svg", width: 75%), caption: [Median public query seconds per arm and the once-per-base lookup build (L0 host).])

== What this is and is not

- The refutations are a meet-in-the-middle over the factor base placed in front of the SAT instance: an empty domain proves no decomposition exists, without search. Per-query cost grows as $|B|^(m-d)$ with a $|B|^d$ precomputation, which is the usual MITM trade-off, not a sub-exponential step.
- Decomposition stage only: no relations were collected and no DLP solved. No leaderboard or `benchmarks/` row (validation gate: no replay certificate, validator report or matched-rho `VALIDATION.json`). Matched rho on this curve: 1.51 s mean online (`research/n83_msweep_20261024`), so the distance remaining is unchanged in kind.

== Deviations and limits

- `PROTOCOL.md` priced the lookup with 334 signed points; the base is already negation-closed (167), so costs are ≈ $2^d$ smaller than declared. m = 8, 9 were added by a separate frozen amendment rather than by editing the protocol.
- L0 host, 5 targets per m, one base size; m = 8 and 9 ran in parallel and the m = 9 queries overlapped the m = 8 run.
- EKS (AGENTS.md §9a) not used: not an ecbench workload, no aws CLI in this session.
- Next levers (not measured): reconstruct the planted witness directly from the lookup hit (re-enumerate the matching $d$-multiset) instead of leaving it to SAT; a direct-indexed hash in place of binary search for the m = 8, 9 lookups.
